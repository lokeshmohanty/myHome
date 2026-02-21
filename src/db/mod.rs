use rusqlite::{Connection, Result};
use std::path::Path;

pub struct Db {
    pub conn: Connection,
}

impl Db {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(path)?;
        Ok(Db { conn })
    }

    pub fn init(&self) -> Result<()> {
        let schema = include_str!("schema.sql");
        self.conn.execute_batch(schema)?;
        Ok(())
    }

    /// Safely backups the database to a target path.
    pub fn backup<P: AsRef<Path>>(&self, target_path: P) -> Result<()> {
        let mut target_conn = Connection::open(target_path)?;
        let backup = rusqlite::backup::Backup::new(&self.conn, &mut target_conn)?;
        backup.run_to_completion(10, std::time::Duration::from_millis(100), None)?;
        Ok(())
    }

    /// Restores the database from a source path.
    #[allow(dead_code)]
    pub fn restore<P: AsRef<Path>>(&mut self, source_path: P) -> Result<()> {
        let source_conn = Connection::open(source_path)?;
        let backup = rusqlite::backup::Backup::new(&source_conn, &mut self.conn)?;
        backup.run_to_completion(10, std::time::Duration::from_millis(100), None)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db_initialization() {
        let db = Db::new(":memory:").expect("Failed to create in-memory db");

        let result = db.init();
        assert!(
            result.is_ok(),
            "Database initialization failed: {:?}",
            result.err()
        );

        // Verify a table exists to be sure `execute_batch` executed correctly
        let mut stmt = db
            .conn
            .prepare(
                "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='module_state'",
            )
            .unwrap();
        let table_exists: bool = stmt
            .query_row([], |row| {
                let count: i32 = row.get(0)?;
                Ok(count > 0)
            })
            .unwrap();

        assert!(table_exists, "The module_state table was not created.");
    }
}
