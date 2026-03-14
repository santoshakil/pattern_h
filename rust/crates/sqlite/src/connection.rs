use errors::StorageError;
use parking_lot::Mutex;
use rusqlite::Connection;
use std::path::Path;

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn open(path: &Path) -> Result<Self, StorageError> {
        let conn = Connection::open(path).map_err(|e| StorageError::Connection(e.to_string()))?;
        Self::configure(conn)
    }

    pub fn in_memory() -> Result<Self, StorageError> {
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
