mod db;
mod modules;

use modules::dashboard::DashboardService;
use modules::dining::DiningService;
use modules::finance::FinanceService;
use modules::grocery::GroceryService;
use modules::maintenance::MaintenanceService;
use modules::registry::ModuleRegistry;
use modules::travel::TravelService;
use modules::gifts::GiftsService;
use modules::household::HouseholdService;
use modules::settings::SettingsService;
use slint::VecModel;
use std::rc::Rc;

slint::include_modules!();

fn refresh_modules(ui: &AppWindow, db_path: &str) {
    let database = db::Db::new(db_path).expect("Failed to open DB");
    let registry = ModuleRegistry::new(&database);
    let all_modules = registry.get_all_modules().expect("Failed to get modules");

    let modules_model = Rc::new(VecModel::default());
    for m in all_modules {
        modules_model.push(ModuleData {
            id: m.id.into(),
            name: m.name.into(),
            description: m.description.into(),
            is_enabled: m.is_enabled,
        });
    }
    ui.set_runtime_modules(modules_model.into());
}

fn refresh_dashboard_summary(ui: &AppWindow, db_path: &str) {
    let database = db::Db::new(db_path).expect("Failed to open DB");
    let dashboard_service = DashboardService::new(&database);
    let settings_service = SettingsService::new(&database);
    let settings = settings_service.get_settings().unwrap_or(modules::settings::AppSettings {
        currency_code: "INR".to_string(),
        currency_symbol: "₹".to_string(),
        user_name: "Lokesh".to_string(),
        theme: "Dark".to_string(),
    });

    ui.set_welcome_message(format!("Welcome home, {}!", settings.user_name).into());

    if let Ok(summary) = dashboard_service.get_summary() {
        ui.set_dashboard_balance(format!("{:.2}", summary.net_balance).into());
        ui.set_dashboard_trip_count(summary.active_trips as i32);
        ui.set_dashboard_grocery_count(summary.grocery_items as i32);
    }
}
fn refresh_maintenance(ui: &AppWindow, db_path: &str) {
    let database = db::Db::new(db_path).expect("Failed to open DB");
    let maintenance_service = MaintenanceService::new(&database);

    if let Ok(appliances) = maintenance_service.get_appliances() {
        let app_model = Rc::new(VecModel::default());
        for app in appliances {
            app_model.push(MaintenanceApplianceData {
                id: app.id.into(),
                name: app.name.into(),
                brand: app.brand.unwrap_or_else(|| "-".to_string()).into(),
                purchase_date: app
                    .purchase_date
                    .unwrap_or_else(|| "Unknown".to_string())
                    .into(),
                warranty_expiry: app
                    .warranty_expiry
                    .unwrap_or_else(|| "None".to_string())
                    .into(),
            });
        }
        ui.set_maintenance_appliances(app_model.into());
    }
}

fn refresh_dining(ui: &AppWindow, db_path: &str) {
    let database = db::Db::new(db_path).expect("Failed to open DB");
    let dining_service = DiningService::new(&database);

    if let Ok(restaurants) = dining_service.get_restaurants() {
        let rest_model = Rc::new(VecModel::default());
        for rest in restaurants {
            rest_model.push(DiningRestaurantData {
                id: rest.id.into(),
                name: rest.name.into(),
                cuisine_type: rest
                    .cuisine_type
                    .unwrap_or_else(|| "General".to_string())
                    .into(),
                location: rest
                    .location
                    .unwrap_or_else(|| "Unknown".to_string())
                    .into(),
                visited: rest.visited_flag,
                rating: rest
                    .rating
                    .map(|r| r.to_string())
                    .unwrap_or_else(|| "-".to_string())
                    .into(),
            });
        }
        ui.set_dining_restaurants(rest_model.into());
    }
}

fn refresh_travel(ui: &AppWindow, db_path: &str) {
    let database = db::Db::new(db_path).expect("Failed to open DB");
    let travel_service = TravelService::new(&database);

    if let Ok(trips) = travel_service.get_trips() {
        let trips_model = Rc::new(VecModel::default());
        for trip in trips {
            let start = trip
                .start_date
                .split('T')
                .next()
                .unwrap_or(&trip.start_date);
            let end = trip.end_date.split('T').next().unwrap_or(&trip.end_date);

            trips_model.push(TravelTripData {
                id: trip.id.into(),
                name: trip.name.into(),
                destination: trip.destination.into(),
                date_range: format!("{} to {}", start, end).into(),
                trip_type: trip.trip_type.unwrap_or_else(|| "Trip".to_string()).into(),
                status: trip.status.into(),
            });
        }
        ui.set_travel_trips(trips_model.into());
    }
}

fn refresh_grocery(ui: &AppWindow, db_path: &str) {
    let database = db::Db::new(db_path).expect("Failed to open DB");
    let grocery_service = GroceryService::new(&database);

    if let Ok(groceries) = grocery_service.get_grocery_list() {
        let grocery_model = Rc::new(VecModel::default());
        for item in groceries {
            grocery_model.push(GroceryItemData {
                id: item.id.into(),
                name: item.name.into(),
                quantity: item.quantity.unwrap_or(1.0) as f32,
                unit: item.unit.unwrap_or_else(|| "unit".to_string()).into(),
                category: item
                    .category
                    .unwrap_or_else(|| "Uncategorized".to_string())
                    .into(),
            });
        }
        ui.set_grocery_items(grocery_model.into());
    }

    if let Ok(inventory) = grocery_service.get_inventory() {
        let inventory_model = Rc::new(VecModel::default());
        for item in inventory {
            inventory_model.push(InventoryItemData {
                id: item.id.into(),
                name: item.name.into(),
                quantity: item.quantity.unwrap_or(1.0) as f32,
                unit: item.unit.unwrap_or_else(|| "unit".to_string()).into(),
                location: item
                    .location_id
                    .unwrap_or_else(|| "Pantry".to_string())
                    .into(),
            });
        }
        ui.set_inventory_items(inventory_model.into());
    }
}

fn refresh_finance(ui: &AppWindow, db_path: &str) {
    let database = db::Db::new(db_path).expect("Failed to open DB");
    let finance_service = FinanceService::new(&database);
    let settings_service = SettingsService::new(&database);
    let _settings = settings_service.get_settings().unwrap_or(modules::settings::AppSettings {
        currency_code: "INR".to_string(),
        currency_symbol: "₹".to_string(),
        user_name: "Lokesh".to_string(),
        theme: "Dark".to_string(),
    });

    ui.set_finance_balance(format!("{:.2}", finance_service.get_total_balance()).into());

    let mut first_account_id = None;
    if let Ok(accounts) = finance_service.get_accounts() {
        let accounts_model = Rc::new(VecModel::default());
        for acc in accounts {
            if first_account_id.is_none() {
                first_account_id = Some(acc.id.clone());
            }
            accounts_model.push(FinanceAccountData {
                id: acc.id.into(),
                name: acc.name.into(),
                account_type: acc.account_type.into(),
                currency_code: acc.currency_code.into(),
                balance: format!("{:.2}", (acc.current_balance_cents as f64) / 100.0).into(),
            });
        }
        ui.set_finance_accounts(accounts_model.into());
    }

    if let Ok(transactions) = finance_service.get_transactions(50) {
        let tx_model = Rc::new(VecModel::default());
        for tx in transactions {
            let mut formatted_amount = format!("{:.2}", (tx.amount_cents.abs() as f64) / 100.0);
            if tx.amount_cents < 0 {
                formatted_amount = format!("-{}", formatted_amount);
            }

            // Just extracting the YYYY-MM-DD from the RFC3339 timestamp
            let short_date = tx.date.split('T').next().unwrap_or(&tx.date);

            tx_model.push(FinanceTransactionData {
                id: tx.id.into(),
                amount: formatted_amount.into(),
                is_income: tx.amount_cents > 0,
                date: short_date.into(),
                merchant: tx.merchant.into(),
                category: tx
                    .category_name
                    .unwrap_or_else(|| "Uncategorized".to_string())
                    .into(),
            });
        }
        ui.set_finance_transactions(tx_model.into());
    }

    // Attach the inferred first_account_id to the UI so we can use it to create transactions blindly.
    // However, since Slint doesn't have an internal place to store secret context easily without property editing,
    // we'll handle the "auto_first_account" tag directly in the create_transaction callback.
}

// --- Gifts Refresh ---
fn refresh_gifts(ui: &AppWindow, db_path: &str) {
    let database = db::Db::new(db_path).expect("Failed to open DB");
    let gifts_service = GiftsService::new(&database);
    
    if let Ok(people) = gifts_service.get_people() {
        let mut ui_people = Vec::new();
        for person in &people {
            ui_people.push(PersonData {
                id: person.id.clone().into(),
                name: person.name.clone().into(),
                relationship: person.relationship.clone().unwrap_or_default().into(),
                date_of_birth: person.date_of_birth.clone().unwrap_or_default().into(),
            });
        }
        let rc_people = std::rc::Rc::new(slint::VecModel::from(ui_people));
        ui.set_gifts_people(rc_people.into());
        
        // Fetch all gift ideas for all people for the global list
        let mut all_ideas = Vec::new();
        for person in people {
            if let Ok(ideas) = gifts_service.get_gift_ideas() {
                for idea in ideas {
                    if idea.person_id == person.id {
                        all_ideas.push(GiftIdeaData {
                            id: idea.id.into(),
                            person_id: idea.person_id.into(),
                            description: idea.description.into(),
                            estimated_price: idea.estimated_price_cents.unwrap_or(0).to_string().into(),
                            status: idea.status.into(),
                        });
                    }
                }
            }
        }
        let rc_ideas = std::rc::Rc::new(slint::VecModel::from(all_ideas));
        ui.set_gifts_ideas(rc_ideas.into());
    }
}

// --- Household Refresh ---
fn refresh_household(ui: &AppWindow, db_path: &str) {
    let database = db::Db::new(db_path).expect("Failed to open DB");
    let household_service = HouseholdService::new(&database);
    
    if let Ok(members) = household_service.get_members() {
        let mut ui_members = Vec::new();
        for member in members {
            ui_members.push(MemberData {
                id: member.id.into(),
                name: member.name.into(),
                relationship: member.relationship.into(),
                date_of_birth: member.date_of_birth.unwrap_or_default().into(),
                is_primary: member.is_primary,
            });
        }
        let rc_members = std::rc::Rc::new(slint::VecModel::from(ui_members));
        ui.set_household_members(rc_members.into());
    }

    if let Ok(documents) = household_service.get_documents() {
        let mut ui_documents = Vec::new();
        for doc in documents {
            ui_documents.push(DocumentData {
                id: doc.id.into(),
                member_id: doc.member_id.unwrap_or_default().into(),
                name: doc.name.into(),
                document_type: doc.document_type.into(),
                expiry_date: doc.expiry_date.unwrap_or_default().into(),
            });
        }
        let rc_documents = std::rc::Rc::new(slint::VecModel::from(ui_documents));
        ui.set_household_documents(rc_documents.into());
    }
}

// --- Settings Refresh ---
fn refresh_settings(ui: &AppWindow, db_path: &str) {
    let database = db::Db::new(db_path).expect("Failed to open DB");
    let settings_service = SettingsService::new(&database);
    
    if let Ok(settings) = settings_service.get_settings() {
        ui.set_currency_symbol(settings.currency_symbol.into());
        ui.set_selected_currency(settings.currency_code.into());
        ui.set_user_name(settings.user_name.into());
        ui.set_current_theme(settings.theme.into());
    }

    let codes: Vec<slint::SharedString> = settings_service.get_currency_list().into_iter().map(|c| c.code.into()).collect();
    ui.set_currency_codes(Rc::new(slint::VecModel::from(codes)).into());
}

fn refresh_all_modules(ui: &AppWindow, db_path: &str) {
    refresh_settings(ui, db_path);
    refresh_dashboard_summary(ui, db_path);
    refresh_maintenance(ui, db_path);
    refresh_dining(ui, db_path);
    refresh_travel(ui, db_path);
    refresh_grocery(ui, db_path);
    refresh_finance(ui, db_path);
    refresh_gifts(ui, db_path);
    refresh_household(ui, db_path);
}

fn main() -> Result<(), slint::PlatformError> {
    // DB setup
    let db_path = "myhome_dev.db";
    let database = db::Db::new(db_path).expect("Failed to open DB");
    database.init().expect("Failed to init DB");

    let registry = ModuleRegistry::new(&database);
    registry
        .setup_default_modules()
        .expect("Failed to setup default modules");

    let ui = AppWindow::new()?;

    ui.set_welcome_message(DashboardService::get_welcome_message().into());

    // --- Onboarding / First Launch Seeding ---
    // If the database is completely empty (no accounts), seed a starting state
    let finance_service = FinanceService::new(&database);
    if let Ok(accounts) = finance_service.get_accounts() {
        if accounts.is_empty() {
            println!("First launch detected: Seeding default mock data...");
            // Seed base accounts
            let _ = finance_service.create_account("My Checking", "checking", 325000); // $3250.00
            let _ = finance_service.create_account("Emergency Fund", "savings", 1000000); // $10000.00

            if let Ok(new_accounts) = finance_service.get_accounts() {
                if !new_accounts.is_empty() {
                    let now = chrono::Utc::now().to_rfc3339();
                    // Seed initial transactions
                    let _ = finance_service.create_transaction(
                        &new_accounts[0].id,
                        -1250,
                        "Coffee Shop",
                        &now,
                        None,
                    );
                    let _ = finance_service.create_transaction(
                        &new_accounts[0].id,
                        -8500,
                        "Grocery Store",
                        &now,
                        None,
                    );
                }
            }

            // Seed a few groceries
            let grocery_service = crate::modules::grocery::GroceryService::new(&database);
            let _ = grocery_service.add_grocery_item("Organic Milk", Some("Dairy"));
            let _ = grocery_service.add_grocery_item("Sourdough Bread", Some("Bakery"));

            // Seed a default upcoming trip
            let travel_service = crate::modules::travel::TravelService::new(&database);
            let _ = travel_service.add_trip(
                "Summer Vacation",
                "Paris, France",
                "2024-07-01",
                "2024-07-14",
                Some("vacation"),
            );
        }
    }
    // ------------------------------------------

    refresh_modules(&ui, db_path);
    refresh_finance(&ui, db_path);

    let ui_handle = ui.as_weak();
    let db_path_clone = db_path.to_string();
    ui.on_toggle_module(move |id, is_enabled| {
        let database = db::Db::new(&db_path_clone).expect("Failed to open DB");
        let registry = ModuleRegistry::new(&database);
        registry
            .toggle_module(id.as_str(), is_enabled)
            .expect("Failed to toggle module");

        if let Some(ui) = ui_handle.upgrade() {
            refresh_modules(&ui, &db_path_clone);
        }
    });

    let ui_handle2 = ui.as_weak();
    let db_path_clone2 = db_path.to_string();
    ui.on_create_account(move |name, acc_type, starting_balance| {
        let database = db::Db::new(&db_path_clone2).expect("Failed to open DB");
        let finance_service = FinanceService::new(&database);

        // Ensure starting_balance is converted from generic float to cents
        finance_service
            .create_account(
                name.as_str(),
                acc_type.as_str(),
                (starting_balance * 100.0) as i64,
            )
            .expect("Failed to create test account");

        if let Some(ui) = ui_handle2.upgrade() {
            refresh_finance(&ui, &db_path_clone2);
            refresh_dashboard_summary(&ui, &db_path_clone2);
        }
    });

    let ui_handle3 = ui.as_weak();
    let db_path_clone3 = db_path.to_string();
    ui.on_create_transaction(move |account_id, amount, merchant, category_name| {
        let database = db::Db::new(&db_path_clone3).expect("Failed to open DB");
        let finance_service = FinanceService::new(&database);

        // Find an account id to use if "auto_first_account" is passed
        let mut target_account_id = account_id.to_string();
        if target_account_id == "auto_first_account" {
            if let Ok(accounts) = finance_service.get_accounts() {
                if !accounts.is_empty() {
                    target_account_id = accounts[0].id.clone();
                } else {
                    println!("Cannot create transaction: No accounts available.");
                    return;
                }
            }
        }

        // We'll create the category dynamically if it doesn't exist, just for testing
        let mut cat_id = None;
        if let Ok(cats) = finance_service.get_categories() {
            if let Some(c) = cats.iter().find(|c| c.name == category_name.as_str()) {
                cat_id = Some(c.id.clone());
            } else {
                let _ =
                    finance_service.create_category(category_name.as_str(), "expense", "#555555");
                if let Ok(cats2) = finance_service.get_categories() {
                    if let Some(c2) = cats2.iter().find(|c| c.name == category_name.as_str()) {
                        cat_id = Some(c2.id.clone());
                    }
                }
            }
        }

        let now = chrono::Utc::now().to_rfc3339();

        finance_service
            .create_transaction(
                &target_account_id,
                (amount as f64 * 100.0) as i64,
                merchant.as_str(),
                &now,
                cat_id.as_deref(),
            )
            .expect("Failed to create transaction");

        if let Some(ui) = ui_handle3.upgrade() {
            refresh_finance(&ui, &db_path_clone3);
            refresh_dashboard_summary(&ui, &db_path_clone3);
        }
    });

    let ui_handle4 = ui.as_weak();
    let db_path_clone4 = db_path.to_string();
    ui.on_add_grocery_item(move |name, category| {
        let database = db::Db::new(&db_path_clone4).expect("Failed to open DB");
        let grocery_service = GroceryService::new(&database);

        grocery_service
            .add_grocery_item(name.as_str(), Some(category.as_str()))
            .expect("Failed to add grocery item");

        if let Some(ui) = ui_handle4.upgrade() {
            refresh_grocery(&ui, &db_path_clone4);
            refresh_dashboard_summary(&ui, &db_path_clone4);
        }
    });

    let ui_handle5 = ui.as_weak();
    let db_path_clone5 = db_path.to_string();
    ui.on_add_inventory_item(move |name, quantity| {
        let database = db::Db::new(&db_path_clone5).expect("Failed to open DB");
        let grocery_service = GroceryService::new(&database);

        grocery_service
            .add_inventory_item(name.as_str(), quantity as f64)
            .expect("Failed to add inventory item");

        if let Some(ui) = ui_handle5.upgrade() {
            refresh_grocery(&ui, &db_path_clone5);
        }
    });

    let ui_handle6 = ui.as_weak();
    let db_path_clone6 = db_path.to_string();
    ui.on_add_travel_trip(move |name, destination, start_date, end_date, trip_type| {
        let database = db::Db::new(&db_path_clone6).expect("Failed to open DB");
        let travel_service = TravelService::new(&database);

        travel_service
            .add_trip(
                name.as_str(),
                destination.as_str(),
                start_date.as_str(),
                end_date.as_str(),
                Some(trip_type.as_str()),
            )
            .expect("Failed to add trip");

        if let Some(ui) = ui_handle6.upgrade() {
            refresh_travel(&ui, &db_path_clone6);
            refresh_dashboard_summary(&ui, &db_path_clone6);
        }
    });

    let ui_handle7 = ui.as_weak();
    let db_path_clone7 = db_path.to_string();
    ui.on_add_restaurant(move |name, cuisine, location, visited| {
        let database = db::Db::new(&db_path_clone7).expect("Failed to open DB");
        let dining_service = DiningService::new(&database);

        dining_service
            .add_restaurant(
                name.as_str(),
                Some(cuisine.as_str()),
                Some(location.as_str()),
                visited,
            )
            .expect("Failed to add restaurant");

        if let Some(ui) = ui_handle7.upgrade() {
            refresh_dining(&ui, &db_path_clone7);
        }
    });

    let ui_handle8 = ui.as_weak();
    let db_path_clone8 = db_path.to_string();
    ui.on_add_appliance(move |name, brand, purchase_date, warranty_expiry| {
        let database = db::Db::new(&db_path_clone8).expect("Failed to open DB");
        let maintenance_service = MaintenanceService::new(&database);

        maintenance_service
            .add_appliance(
                name.as_str(),
                Some(brand.as_str()),
                Some(purchase_date.as_str()),
                Some(warranty_expiry.as_str()),
            )
            .expect("Failed to add appliance");

        if let Some(ui) = ui_handle8.upgrade() {
            refresh_maintenance(&ui, &db_path_clone8);
        }
    });

    let ui_handle9 = ui.as_weak();
    let db_path_clone9 = db_path.to_string();
    ui.on_add_person(move |name, relationship, date_of_birth| {
        let database = db::Db::new(&db_path_clone9).expect("Failed to open DB");
        let gifts_service = GiftsService::new(&database);
        
        gifts_service.add_person(name.as_str(), Some(relationship.as_str()), Some(date_of_birth.as_str())).expect("Failed to add person");

        if let Some(ui) = ui_handle9.upgrade() {
            refresh_gifts(&ui, &db_path_clone9);
        }
    });

    let ui_handle10 = ui.as_weak();
    let db_path_clone10 = db_path.to_string();
    ui.on_add_gift_idea(move |person_id, description, estimated_price| {
        let database = db::Db::new(&db_path_clone10).expect("Failed to open DB");
        let gifts_service = GiftsService::new(&database);
        
        let price_val = estimated_price as i64;

        gifts_service.add_gift_idea(person_id.as_str(), description.as_str(), Some(price_val)).expect("Failed to add gift idea");

        if let Some(ui) = ui_handle10.upgrade() {
            refresh_gifts(&ui, &db_path_clone10);
        }
    });

    let ui_handle11 = ui.as_weak();
    let db_path_clone11 = db_path.to_string();
    ui.on_add_member(move |name, relationship, date_of_birth, is_primary| {
        let database = db::Db::new(&db_path_clone11).expect("Failed to open DB");
        let household_service = HouseholdService::new(&database);
        
        household_service.add_member(name.as_str(), relationship.as_str(), Some(date_of_birth.as_str()), is_primary).expect("Failed to add member");

        if let Some(ui) = ui_handle11.upgrade() {
            refresh_household(&ui, &db_path_clone11);
        }
    });

    let ui_handle12 = ui.as_weak();
    let db_path_clone12 = db_path.to_string();
    ui.on_add_document(move |member_id, name, doc_type, number, expiry_date| {
        let database = db::Db::new(&db_path_clone12).expect("Failed to open DB");
        let household_service = HouseholdService::new(&database);
        
        household_service.add_document(Some(member_id.as_str()), name.as_str(), doc_type.as_str(), Some(number.as_str()), Some(expiry_date.as_str())).expect("Failed to add document");

        if let Some(ui) = ui_handle12.upgrade() {
            refresh_household(&ui, &db_path_clone12);
        }
    });

    let ui_handle13 = ui.as_weak();
    let db_path_clone13 = db_path.to_string();
    ui.on_update_currency(move |code| {
        let database = db::Db::new(&db_path_clone13).expect("Failed to open DB");
        let settings_service = SettingsService::new(&database);
        
        // Find the symbol for the code
        let symbol = settings_service.get_currency_list()
            .into_iter()
            .find(|c| c.code == code.as_str())
            .map(|c| c.symbol)
            .unwrap_or_else(|| "$".to_string());

        settings_service.update_currency(code.as_str(), &symbol).expect("Failed to update currency");

        if let Some(ui) = ui_handle13.upgrade() {
            refresh_all_modules(&ui, &db_path_clone13);
        }
    });

    let ui_handle14 = ui.as_weak();
    let db_path_clone14 = db_path.to_string();
    ui.on_update_profile(move |name, theme| {
        let database = db::Db::new(&db_path_clone14).expect("Failed to open DB");
        let settings_service = SettingsService::new(&database);
        
        settings_service.update_profile(name.as_str(), theme.as_str()).expect("Failed to update profile");

        if let Some(ui) = ui_handle14.upgrade() {
            refresh_all_modules(&ui, &db_path_clone14);
        }
    });

    refresh_all_modules(&ui, db_path);

    ui.run()
}
