use myhome::db::Db;
use myhome::modules::dashboard::DashboardService;
use myhome::modules::finance::FinanceService;
use myhome::modules::grocery::GroceryService;
use myhome::modules::registry::ModuleRegistry;
use myhome::modules::travel::TravelService;

#[test]
fn test_cross_module_dashboard_aggregation() {
    // 1. Initialize DB
    let db = Db::new(":memory:").expect("Failed to create in-memory DB");
    db.init().expect("Failed to initialize schema");

    // 2. Setup Modules
    let registry = ModuleRegistry::new(&db);
    registry
        .setup_default_modules()
        .expect("Failed to setup default modules");

    // Validate nothing is seeded initially by accident
    let dashboard = DashboardService::new(&db);
    let summary = dashboard.get_summary().expect("Failed to get summary");
    assert_eq!(summary.net_balance, 0.0);
    assert_eq!(summary.active_trips, 0);
    assert_eq!(summary.grocery_items, 0);

    // 3. Inject Finance Data
    let finance = FinanceService::new(&db);
    finance
        .create_account("Main Checking", "checking", 500000)
        .expect("Failed to create checking account"); // $5000.00
    finance
        .create_account("Emergency Savings", "savings", 1000000)
        .expect("Failed to create savings account"); // $10000.00

    let accounts = finance.get_accounts().unwrap();
    assert_eq!(accounts.len(), 2);

    // Make a transaction just to be doubly sure transactions apply to the dashboard
    finance
        .create_transaction(
            &accounts[0].id,
            -12550,
            "Groceries Store",
            "2024-01-01T12:00:00Z",
            None,
        )
        .expect("failed tx"); // -125.50
    finance
        .create_transaction(
            &accounts[1].id,
            50000,
            "Interest",
            "2024-01-02T12:00:00Z",
            None,
        )
        .expect("failed tx"); // +500.00

    // 15000.00 - 125.50 + 500.00 = 15374.50
    let summary = dashboard.get_summary().unwrap();
    assert_eq!(summary.net_balance, 15374.50);

    // 4. Inject Travel Data
    let travel = TravelService::new(&db);
    travel
        .add_trip(
            "Tokyo 2024",
            "Tokyo, Japan",
            "2024-04-01",
            "2024-04-14",
            Some("international"),
        )
        .expect("Failed to create trip");
    travel
        .add_trip(
            "NYC Business",
            "New York",
            "2024-06-01",
            "2024-06-03",
            Some("business"),
        )
        .expect("Failed to create trip");

    let summary = dashboard.get_summary().unwrap();
    assert_eq!(summary.active_trips, 2);

    // 5. Inject Grocery Data
    let grocery = GroceryService::new(&db);
    grocery.add_grocery_item("Apples", Some("Produce")).unwrap();
    grocery.add_grocery_item("Milk", Some("Dairy")).unwrap();
    grocery.add_grocery_item("Bread", Some("Bakery")).unwrap();

    let summary = dashboard.get_summary().unwrap();
    assert_eq!(summary.grocery_items, 3);

    // 6. Final comprehensive validation to ensure nothing polluted
    let summary = dashboard.get_summary().unwrap();
    assert_eq!(summary.net_balance, 15374.50);
    assert_eq!(summary.active_trips, 2);
    assert_eq!(summary.grocery_items, 3);
}
