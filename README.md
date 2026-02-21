# My Home (Home Management App)

A modern, desktop-first home management application built with **Rust**, **Slint**, and **SQLite**.

## Features
My Home is a modular application designed to track everything from finances to household inventory and travel plans. It features a clean, reactive UI and a local-first, encrypted data store.

### Current Modules
- **Dashboard**: Cross-module summary and welcome page.
- **Finance**: Multi-account tracking, transactions, and categories.
- **Grocery & Inventory**: Shopping lists and pantry management.
- **Travel Manager**: Trip planning and status tracking.
- **Dining**: Restaurant wishlist and visited logs.
- **Maintenance**: Appliance inventory and warranty tracking.
- **Gifts & Occasions**: People registry and gift ideas.
- **Household**: Member directory and encrypted document vault.
- **Settings**: Module management, theme toggles, and currency customization.

## Architecture
The application follows a clean separation between the UI and Business Logic:

- **Frontend (Slint)**: The `ui/main.slint` file defines the reactive interface. It uses properties to receive data from Rust and callbacks to trigger actions.
- **Backend (Rust)**: The `src/modules/` directory contains independent services (e.g., `FinanceService`, `TravelService`) that handle business logic.
- **Data Layer (SQLite)**: A local SQLite database (`myhome_dev.db`) persists all records. The schema is defined in `src/db/schema.sql`.

## Getting Started

### Prerequisites
- **Nix**: The project uses a Nix flake to manage dependencies (Slint, Rust, SQLite).
- **Cargo**: Rust package manager.

### Development
1. Enter the Nix shell:
   ```bash
   nix develop
   ```
2. Run the application:
   ```bash
   just dev
   ```
3. Run tests:
   ```bash
   just test
   ```

## Extending the App
To add a new module:
1. **Schema**: Append your table definitions to `src/db/schema.sql`.
2. **Service**: Create a new file in `src/modules/[module_name].rs` with your CRUD logic.
3. **Rust Export**: Export the module in `src/modules/mod.rs`.
4. **UI**: Add a new tab and properties to `ui/main.slint`.
5. **Main**: Wire the service logic into Slint callbacks in `src/main.rs`.

## Personalization
The app supports global settings including:
- **Currency**: Customizable base currency (default: INR / ₹).
- **User Profile**: Personalize your display name.
- **Theme**: Toggle between Dark and Light modes.

---
Built with ❤️ for better home organization.
