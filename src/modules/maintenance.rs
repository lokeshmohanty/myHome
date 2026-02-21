use crate::db::Db;
use chrono::Utc;
use rusqlite::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Appliance {
    pub id: String,
    pub name: String,
    pub brand: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub purchase_date: Option<String>,
    pub warranty_expiry: Option<String>,
}

pub struct MaintenanceService<'a> {
    db: &'a Db,
}

impl<'a> MaintenanceService<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    pub fn get_appliances(&self) -> Result<Vec<Appliance>> {
        let mut stmt = self.db.conn.prepare(
            "SELECT id, name, brand, model, serial_number, purchase_date, warranty_expiry FROM appliances WHERE deleted_at IS NULL ORDER BY name ASC"
        )?;

        let appliances = stmt
            .query_map([], |row| {
                Ok(Appliance {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    brand: row.get(2)?,
                    model: row.get(3)?,
                    serial_number: row.get(4)?,
                    purchase_date: row.get(5)?,
                    warranty_expiry: row.get(6)?,
                })
            })?
            .filter_map(Result::ok)
            .collect();

        Ok(appliances)
    }

    pub fn add_appliance(
        &self,
        name: &str,
        brand: Option<&str>,
        purchase_date: Option<&str>,
        warranty_expiry: Option<&str>,
    ) -> Result<()> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();

        self.db.conn.execute(
            "INSERT INTO appliances (id, name, brand, purchase_date, warranty_expiry, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (id, name, brand, purchase_date, warranty_expiry, &now, &now),
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maintenance_service() {
        let db = Db::new(":memory:").unwrap();
        db.init().unwrap();

        let service = MaintenanceService::new(&db);

        service
            .add_appliance(
                "HVAC System",
                Some("Carrier"),
                Some("2020-05-01"),
                Some("2030-05-01"),
            )
            .unwrap();
        service
            .add_appliance(
                "Refrigerator",
                Some("Samsung"),
                Some("2022-11-15"),
                Some("2025-11-15"),
            )
            .unwrap();

        let appliances = service.get_appliances().unwrap();
        assert_eq!(appliances.len(), 2);

        // Ordered by name
        assert_eq!(appliances[0].name, "HVAC System");
        assert_eq!(appliances[1].name, "Refrigerator");
    }
}
