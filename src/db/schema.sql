-- Module state
CREATE TABLE IF NOT EXISTS module_state (
    module_id TEXT PRIMARY KEY,
    is_enabled INTEGER NOT NULL DEFAULT 0,
    schema_version INTEGER NOT NULL DEFAULT 0,
    enabled_at TEXT,
    settings_json TEXT
);

-- Household members
CREATE TABLE IF NOT EXISTS members (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    relationship TEXT NOT NULL,
    date_of_birth TEXT,
    profile_photo_path TEXT,
    is_primary INTEGER NOT NULL DEFAULT 0,
    permission_json TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted_at TEXT
);

-- Finance: accounts
CREATE TABLE IF NOT EXISTS accounts (
    id TEXT PRIMARY KEY,
    member_id TEXT REFERENCES members(id),
    name TEXT NOT NULL,
    account_type TEXT NOT NULL, -- checking|savings|credit|cash|investment|loan|crypto
    institution_name TEXT,
    currency_code TEXT NOT NULL DEFAULT 'USD',
    current_balance_cents INTEGER NOT NULL DEFAULT 0,
    credit_limit_cents INTEGER,
    statement_close_day INTEGER,
    payment_due_day INTEGER,
    is_linked INTEGER NOT NULL DEFAULT 0, -- bank-connected
    open_banking_provider TEXT,
    access_token_encrypted BLOB,
    color TEXT,
    icon TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted_at TEXT
);

-- Finance: categories (tree structure via parent_id)
CREATE TABLE IF NOT EXISTS categories (
    id TEXT PRIMARY KEY,
    parent_id TEXT REFERENCES categories(id),
    name TEXT NOT NULL,
    icon TEXT,
    color TEXT,
    type TEXT NOT NULL, -- income|expense
    is_system INTEGER NOT NULL DEFAULT 0,
    sort_order INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted_at TEXT
);

-- Finance: transactions
CREATE TABLE IF NOT EXISTS transactions (
    id TEXT PRIMARY KEY,
    account_id TEXT NOT NULL REFERENCES accounts(id),
    amount_cents INTEGER NOT NULL, -- negative = expense, positive = income
    currency_code TEXT NOT NULL,
    date TEXT NOT NULL,
    merchant TEXT,
    payee TEXT,
    category_id TEXT REFERENCES categories(id),
    notes TEXT,
    receipt_path TEXT,
    tags TEXT, -- JSON array
    is_pending INTEGER NOT NULL DEFAULT 0,
    import_hash TEXT UNIQUE, -- for dedup on import
    source TEXT NOT NULL DEFAULT 'manual', -- manual|import|bank_sync
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted_at TEXT
);

-- Finance: budgets
CREATE TABLE IF NOT EXISTS budgets (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    category_id TEXT REFERENCES categories(id),
    account_id TEXT REFERENCES accounts(id),
    amount_cents INTEGER NOT NULL,
    period_type TEXT NOT NULL, -- monthly|quarterly|annual|custom
    period_start TEXT,
    period_end TEXT,
    rollover INTEGER NOT NULL DEFAULT 0,
    alert_threshold_pct INTEGER NOT NULL DEFAULT 80,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted_at TEXT
);

-- Finance: bills
CREATE TABLE IF NOT EXISTS bills (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    amount_cents INTEGER NOT NULL,
    is_estimated INTEGER NOT NULL DEFAULT 0,
    currency_code TEXT NOT NULL,
    recurrence_type TEXT NOT NULL, -- weekly|biweekly|monthly|quarterly|annual
    recurrence_day INTEGER, -- day of month for monthly
    next_due TEXT NOT NULL,
    account_id TEXT REFERENCES accounts(id),
    category_id TEXT REFERENCES categories(id),
    is_autopay INTEGER NOT NULL DEFAULT 0,
    alert_days_before INTEGER NOT NULL DEFAULT 3,
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted_at TEXT
);

-- Household: documents
CREATE TABLE IF NOT EXISTS documents (
    id TEXT PRIMARY KEY,
    member_id TEXT REFERENCES members(id),
    name TEXT NOT NULL,
    document_type TEXT NOT NULL,
    document_number TEXT,
    issue_date TEXT,
    expiry_date TEXT,
    issuing_authority TEXT,
    file_path TEXT, -- path to encrypted file on device
    file_encrypted_key BLOB, -- per-file encryption key, encrypted with master key
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted_at TEXT
);

-- Grocery: shopping lists
CREATE TABLE IF NOT EXISTS shopping_lists (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    is_template INTEGER NOT NULL DEFAULT 0,
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted_at TEXT
);

-- Grocery: shopping list items
CREATE TABLE IF NOT EXISTS shopping_list_items (
    id TEXT PRIMARY KEY,
    list_id TEXT NOT NULL REFERENCES shopping_lists(id),
    name TEXT NOT NULL,
    quantity REAL NOT NULL DEFAULT 1,
    unit TEXT,
    category TEXT,
    estimated_price_cents INTEGER,
    preferred_brand TEXT,
    store_hint TEXT,
    is_checked INTEGER NOT NULL DEFAULT 0,
    notes TEXT,
    barcode TEXT,
    sort_order INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Grocery: inventory
CREATE TABLE IF NOT EXISTS inventory_items (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    category TEXT,
    quantity REAL NOT NULL DEFAULT 0,
    unit TEXT,
    location TEXT,
    minimum_threshold REAL,
    barcode TEXT,
    expiry_date TEXT,
    purchase_date TEXT,
    cost_per_unit_cents INTEGER,
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted_at TEXT
);

-- Travel: trips
CREATE TABLE IF NOT EXISTS trips (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    destination TEXT NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    trip_type TEXT, -- vacation|business|family|road_trip|camping
    status TEXT NOT NULL DEFAULT 'planning', -- planning|upcoming|active|completed
    total_budget_cents INTEGER,
    currency_code TEXT NOT NULL,
    notes TEXT,
    cover_photo_path TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted_at TEXT
);

-- Maintenance: appliances
CREATE TABLE IF NOT EXISTS appliances (
    id TEXT PRIMARY KEY,
    home_area TEXT,
    name TEXT NOT NULL,
    brand TEXT,
    model TEXT,
    serial_number TEXT,
    purchase_date TEXT,
    warranty_expiry TEXT,
    purchase_price_cents INTEGER,
    vendor_contact TEXT,
    manual_path TEXT,
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted_at TEXT
);

-- Notifications schedule
CREATE TABLE IF NOT EXISTS scheduled_notifications (
    id TEXT PRIMARY KEY,
    module_id TEXT NOT NULL,
    entity_id TEXT NOT NULL,
    notification_type TEXT NOT NULL,
    title TEXT NOT NULL,
    body TEXT NOT NULL,
    scheduled_for TEXT NOT NULL,
    is_fired INTEGER NOT NULL DEFAULT 0,
    is_dismissed INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL
);

-- Sync: vector clocks for CRDT
CREATE TABLE IF NOT EXISTS sync_vector_clocks (
    table_name TEXT NOT NULL,
    row_id TEXT NOT NULL,
    device_id TEXT NOT NULL,
    clock INTEGER NOT NULL,
    PRIMARY KEY (table_name, row_id, device_id)
);-- Restaurants wishlist & visited log
CREATE TABLE IF NOT EXISTS restaurants (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    cuisine_type TEXT,
    location TEXT,
    price_range TEXT,
    visited_flag INTEGER NOT NULL DEFAULT 0,
    rating INTEGER,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted_at TEXT
);

-- Gifts: people
CREATE TABLE IF NOT EXISTS people (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    relationship TEXT,
    date_of_birth TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted_at TEXT
);

-- Gifts: gift_ideas
CREATE TABLE IF NOT EXISTS gift_ideas (
    id TEXT PRIMARY KEY,
    person_id TEXT NOT NULL REFERENCES people(id),
    description TEXT NOT NULL,
    estimated_price_cents INTEGER,
    status TEXT NOT NULL DEFAULT 'idea', -- idea|purchased|gifted
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted_at TEXT
);
-- App preferences/settings
CREATE TABLE IF NOT EXISTS app_preferences (
    id INTEGER PRIMARY KEY CHECK (id = 1),
    currency_code TEXT NOT NULL DEFAULT 'INR',
    currency_symbol TEXT NOT NULL DEFAULT '₹',
    user_name TEXT NOT NULL DEFAULT 'Lokesh',
    theme TEXT NOT NULL DEFAULT 'Dark'
);
