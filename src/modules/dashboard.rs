use crate::db::Db;
use crate::modules::finance::FinanceService;
use crate::modules::grocery::GroceryService;
use crate::modules::travel::TravelService;

pub struct DashboardSummary {
    pub net_balance: f64,
    pub active_trips: usize,
    pub grocery_items: usize,
}

pub struct DashboardService<'a> {
    db: &'a Db,
}

impl<'a> DashboardService<'a> {
    pub fn new(db: &'a Db) -> Self {
        Self { db }
    }

    pub fn get_welcome_message() -> String {
        "Welcome home, Lokesh!".to_string()
    }

    pub fn get_summary(&self) -> Result<DashboardSummary, rusqlite::Error> {
        let finance_service = FinanceService::new(self.db);
        let travel_service = TravelService::new(self.db);
        let grocery_service = GroceryService::new(self.db);

        Ok(DashboardSummary {
            net_balance: finance_service.get_total_balance(),
            active_trips: travel_service.get_trips().map(|t| t.len()).unwrap_or(0),
            grocery_items: grocery_service
                .get_grocery_list()
                .map(|i| i.len())
                .unwrap_or(0),
        })
    }

    pub fn get_expenditure_by_category(&self) -> Result<Vec<(String, f64)>, rusqlite::Error> {
        let mut stmt = self.db.conn.prepare(
            "SELECT c.name, SUM(ABS(t.amount_cents)) as total_cents 
             FROM transactions t
             JOIN categories c ON t.category_id = c.id
             WHERE t.amount_cents < 0
             GROUP BY c.id",
        )?;
        let rows = stmt.query_map([], |row| {
            let name: String = row.get(0)?;
            let total_cents: i64 = row.get(1)?;
            Ok((name, (total_cents as f64) / 100.0))
        })?;

        let mut results = Vec::new();
        for res in rows {
            results.push(res?);
        }
        Ok(results)
    }
}
