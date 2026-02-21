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
