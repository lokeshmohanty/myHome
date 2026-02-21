use crate::db::Db;
use rusqlite::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleManifest {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub is_enabled: bool,
}

pub struct ModuleRegistry<'a> {
    db: &'a Db,
}

impl<'a> ModuleRegistry<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    pub fn setup_default_modules(&self) -> Result<()> {
        let defaults = Self::get_hardcoded_modules();

        for m in defaults {
            self.db.conn.execute(
                "INSERT OR IGNORE INTO module_state (module_id, is_enabled) VALUES (?1, ?2)",
                (&m.id, if m.is_enabled { 1 } else { 0 }),
            )?;
        }
        Ok(())
    }

    pub fn get_all_modules(&self) -> Result<Vec<ModuleManifest>> {
        let mut defaults = Self::get_hardcoded_modules();

        let mut stmt = self
            .db
            .conn
            .prepare("SELECT module_id, is_enabled FROM module_state")?;
        let state_map: HashMap<String, bool> = stmt
            .query_map([], |row| {
                let id: String = row.get(0)?;
                let is_enabled: i32 = row.get(1)?;
                Ok((id, is_enabled == 1))
            })?
            .filter_map(Result::ok)
            .collect();

        for module in &mut defaults {
            if let Some(&is_enabled) = state_map.get(&module.id) {
                module.is_enabled = is_enabled;
            }
        }

        Ok(defaults)
    }

    pub fn toggle_module(&self, module_id: &str, is_enabled: bool) -> Result<()> {
        self.db.conn.execute(
            "UPDATE module_state SET is_enabled = ?1 WHERE module_id = ?2",
            (if is_enabled { 1 } else { 0 }, module_id),
        )?;
        Ok(())
    }

    fn get_hardcoded_modules() -> Vec<ModuleManifest> {
        vec![
            ModuleManifest {
                id: "finance".to_string(),
                name: "Finance".to_string(),
                description: "Manage accounts, budgets, and transactions".to_string(),
                icon: "".to_string(),
                is_enabled: true, // Enabled by default
            },
            ModuleManifest {
                id: "grocery".to_string(),
                name: "Grocery".to_string(),
                description: "Shopping lists and inventory manager".to_string(),
                icon: "".to_string(),
                is_enabled: false,
            },
            ModuleManifest {
                id: "travel".to_string(),
                name: "Travel".to_string(),
                description: "Trip planning and itineraries".to_string(),
                icon: "".to_string(),
                is_enabled: false,
            },
            ModuleManifest {
                id: "dining".to_string(),
                name: "Dining".to_string(),
                description: "Restaurant lists and visits".to_string(),
                icon: "".to_string(),
                is_enabled: false,
            },
        ]
    }
}
