use errors::StorageError;
use rusqlite::Connection;

pub struct Migration {
    pub version: u32,
    pub sql: &'static str,
}

pub fn run_migrations(conn: &Connection, migrations: &[Migration]) -> Result<(), StorageError> {
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS _migrations (
            version INTEGER PRIMARY KEY,
            applied_at TEXT NOT NULL DEFAULT (datetime('now'))
        );",
    )
    .map_err(|e| StorageError::Migration {
        version: "init".into(),
        reason: e.to_string(),
    })?;

    let current = current_version(conn)?;

    for m in migrations {
        if m.version <= current {
            continue;
        }
        tracing::info!(version = m.version, "applying migration");
        conn.execute_batch(m.sql)
            .map_err(|e| StorageError::Migration {
                version: m.version.to_string(),
                reason: e.to_string(),
            })?;
        conn.execute("INSERT INTO _migrations (version) VALUES (?1)", [m.version])
            .map_err(|e| StorageError::Migration {
                version: m.version.to_string(),
                reason: e.to_string(),
            })?;
    }

    Ok(())
}

pub fn current_version(conn: &Connection) -> Result<u32, StorageError> {
    let has_table: bool = conn
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='_migrations')",
            [],
            |row| row.get(0),
        )
        .map_err(|e| StorageError::Query(e.to_string()))?;

    if !has_table {
        return Ok(0);
    }

    conn.query_row(
        "SELECT COALESCE(MAX(version), 0) FROM _migrations",
        [],
        |row| row.get(0),
    )
    .map_err(|e| StorageError::Query(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mem_conn() -> Result<Connection, StorageError> {
        Connection::open_in_memory().map_err(|e| StorageError::Connection(e.to_string()))
    }

    #[test]
    fn run_migrations_creates_table() -> Result<(), StorageError> {
        let conn = mem_conn()?;
        run_migrations(&conn, &[])?;
        let exists: bool = conn
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='_migrations')",
                [],
                |row| row.get(0),
            )
            .map_err(|e| StorageError::Query(e.to_string()))?;
        assert!(exists);
        Ok(())
    }

    #[test]
    fn run_migrations_applies_in_order() -> Result<(), StorageError> {
        let conn = mem_conn()?;
        let ms = [
            Migration {
                version: 1,
                sql: "CREATE TABLE t1 (id INTEGER PRIMARY KEY);",
            },
            Migration {
                version: 2,
                sql: "CREATE TABLE t2 (id INTEGER PRIMARY KEY);",
            },
        ];
        run_migrations(&conn, &ms)?;

        let t1: bool = conn
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='t1')",
                [],
                |row| row.get(0),
            )
            .map_err(|e| StorageError::Query(e.to_string()))?;
        let t2: bool = conn
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='t2')",
                [],
                |row| row.get(0),
            )
            .map_err(|e| StorageError::Query(e.to_string()))?;
        assert!(t1);
        assert!(t2);
        assert_eq!(current_version(&conn)?, 2);
        Ok(())
    }

    #[test]
    fn run_migrations_skips_applied() -> Result<(), StorageError> {
        let conn = mem_conn()?;
        let m1 = [Migration {
            version: 1,
            sql: "CREATE TABLE t1 (id INTEGER PRIMARY KEY);",
        }];
        run_migrations(&conn, &m1)?;

        let both = [
            Migration {
                version: 1,
                sql: "CREATE TABLE t1 (id INTEGER PRIMARY KEY);",
            },
            Migration {
                version: 2,
                sql: "CREATE TABLE t2 (id INTEGER PRIMARY KEY);",
            },
        ];
        run_migrations(&conn, &both)?;

        assert_eq!(current_version(&conn)?, 2);
        let t2: bool = conn
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name='t2')",
                [],
                |row| row.get(0),
            )
            .map_err(|e| StorageError::Query(e.to_string()))?;
        assert!(t2);
        Ok(())
    }

    #[test]
    fn current_version_no_migrations() -> Result<(), StorageError> {
        let conn = mem_conn()?;
        let v = current_version(&conn)?;
        assert_eq!(v, 0);
        Ok(())
    }

    #[test]
    fn current_version_after_empty_run() -> Result<(), StorageError> {
        let conn = mem_conn()?;
        run_migrations(&conn, &[])?;
        assert_eq!(current_version(&conn)?, 0);
        Ok(())
    }

    #[test]
    fn current_version_returns_latest() -> Result<(), StorageError> {
        let conn = mem_conn()?;
        let ms = [
            Migration {
                version: 1,
                sql: "CREATE TABLE a (id INTEGER);",
            },
            Migration {
                version: 5,
                sql: "CREATE TABLE b (id INTEGER);",
            },
            Migration {
                version: 10,
                sql: "CREATE TABLE c (id INTEGER);",
            },
        ];
        run_migrations(&conn, &ms)?;
        assert_eq!(current_version(&conn)?, 10);
        Ok(())
    }
}
