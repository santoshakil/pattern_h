use errors::StorageError;
use parking_lot::Mutex;
use rusqlite::Connection;
use std::path::Path;

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn open(path: &Path) -> Result<Self, StorageError> {
        tracing::info!(path = %path.display(), "opening database");
        let conn = Connection::open(path).map_err(|e| StorageError::Connection(e.to_string()))?;
        Self::configure(conn)
    }

    pub fn in_memory() -> Result<Self, StorageError> {
        tracing::info!("opening in-memory database");
        let conn =
            Connection::open_in_memory().map_err(|e| StorageError::Connection(e.to_string()))?;
        Self::configure(conn)
    }

    fn configure(conn: Connection) -> Result<Self, StorageError> {
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA foreign_keys = ON;
             PRAGMA busy_timeout = 5000;",
        )
        .map_err(|e| StorageError::Connection(e.to_string()))?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn execute(
        &self,
        sql: &str,
        params: &[&dyn rusqlite::types::ToSql],
    ) -> Result<usize, StorageError> {
        self.conn
            .lock()
            .execute(sql, params)
            .map_err(|e| StorageError::Query(e.to_string()))
    }

    pub fn with_connection<T, F>(&self, f: F) -> Result<T, StorageError>
    where
        F: FnOnce(&Connection) -> Result<T, StorageError>,
    {
        let conn = self.conn.lock();
        f(&conn)
    }

    pub fn with_transaction<T, F>(&self, f: F) -> Result<T, StorageError>
    where
        F: FnOnce(&rusqlite::Transaction<'_>) -> Result<T, StorageError>,
    {
        let mut conn = self.conn.lock();
        let tx = conn
            .transaction()
            .map_err(|e| StorageError::Transaction(e.to_string()))?;
        let result = f(&tx)?;
        tx.commit()
            .map_err(|e| StorageError::Transaction(e.to_string()))?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_memory_succeeds() {
        let db = Database::in_memory();
        assert!(db.is_ok());
    }

    #[test]
    fn execute_runs_sql() -> Result<(), StorageError> {
        let db = Database::in_memory()?;
        db.with_connection(|conn| {
            conn.execute_batch("CREATE TABLE t (id INTEGER PRIMARY KEY, val TEXT);")
                .map_err(|e| StorageError::Query(e.to_string()))
        })?;
        let rows = db.execute("INSERT INTO t (val) VALUES (?1)", &[&"hello"])?;
        assert_eq!(rows, 1);
        Ok(())
    }

    #[test]
    fn with_connection_provides_access() -> Result<(), StorageError> {
        let db = Database::in_memory()?;
        db.with_connection(|conn| {
            conn.execute_batch("CREATE TABLE c (x INTEGER);")
                .map_err(|e| StorageError::Query(e.to_string()))
        })?;
        let count: i64 = db.with_connection(|conn| {
            conn.query_row("SELECT COUNT(*) FROM c", [], |row| row.get(0))
                .map_err(|e| StorageError::Query(e.to_string()))
        })?;
        assert_eq!(count, 0);
        Ok(())
    }

    #[test]
    fn with_transaction_commits_on_success() -> Result<(), StorageError> {
        let db = Database::in_memory()?;
        db.with_connection(|conn| {
            conn.execute_batch("CREATE TABLE tx_test (v INTEGER);")
                .map_err(|e| StorageError::Query(e.to_string()))
        })?;
        db.with_transaction(|tx| {
            tx.execute("INSERT INTO tx_test (v) VALUES (?1)", [1])
                .map_err(|e| StorageError::Query(e.to_string()))?;
            tx.execute("INSERT INTO tx_test (v) VALUES (?1)", [2])
                .map_err(|e| StorageError::Query(e.to_string()))?;
            Ok(())
        })?;
        let count: i64 = db.with_connection(|conn| {
            conn.query_row("SELECT COUNT(*) FROM tx_test", [], |row| row.get(0))
                .map_err(|e| StorageError::Query(e.to_string()))
        })?;
        assert_eq!(count, 2);
        Ok(())
    }

    #[test]
    fn with_transaction_rolls_back_on_error() -> Result<(), StorageError> {
        let db = Database::in_memory()?;
        db.with_connection(|conn| {
            conn.execute_batch("CREATE TABLE rb_test (v INTEGER);")
                .map_err(|e| StorageError::Query(e.to_string()))
        })?;
        let res: Result<(), StorageError> = db.with_transaction(|tx| {
            tx.execute("INSERT INTO rb_test (v) VALUES (?1)", [1])
                .map_err(|e| StorageError::Query(e.to_string()))?;
            Err(StorageError::Query("forced fail".into()))
        });
        assert!(res.is_err());
        let count: i64 = db.with_connection(|conn| {
            conn.query_row("SELECT COUNT(*) FROM rb_test", [], |row| row.get(0))
                .map_err(|e| StorageError::Query(e.to_string()))
        })?;
        assert_eq!(count, 0);
        Ok(())
    }

    #[test]
    fn execute_returns_affected_rows() -> Result<(), StorageError> {
        let db = Database::in_memory()?;
        db.with_connection(|conn| {
            conn.execute_batch("CREATE TABLE ar (id INTEGER PRIMARY KEY, v TEXT);")
                .map_err(|e| StorageError::Query(e.to_string()))
        })?;
        db.execute("INSERT INTO ar (v) VALUES (?1)", &[&"a"])?;
        db.execute("INSERT INTO ar (v) VALUES (?1)", &[&"b"])?;
        db.execute("INSERT INTO ar (v) VALUES (?1)", &[&"c"])?;
        let affected = db.execute("UPDATE ar SET v = ?1", &[&"z"])?;
        assert_eq!(affected, 3);
        Ok(())
    }
}
