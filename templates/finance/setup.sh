#!/bin/bash
# Financial Advisor Template Setup Script

set -e

echo "═══════════════════════════════════════════════════════════"
echo "  💰 Financial Advisor Agent Setup"
echo "═══════════════════════════════════════════════════════════"

if [ ! -f "Makefile" ]; then
    echo "❌ Please run this script from the SuperInstance root directory"
    exit 1
fi

echo "📁 Creating pasture structure..."
mkdir -p pasture/cattle/finance-cow-v1

echo "🧬 Installing breed configuration..."
cp templates/finance/breed.md pasture/cattle/finance-cow-v1/

echo "💾 Setting up client database..."
mkdir -p ~/.superinstance
sqlite3 ~/.superinstance/finance.db <<'SQL'
CREATE TABLE IF NOT EXISTS clients (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    client_id TEXT UNIQUE NOT NULL,
    first_name TEXT,
    last_name TEXT,
    date_of_birth DATE,
    risk_tolerance TEXT DEFAULT 'moderate',
    retirement_age INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS holdings (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    client_id INTEGER,
    ticker TEXT,
    shares REAL,
    cost_basis REAL,
    current_value REAL,
    asset_class TEXT,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (client_id) REFERENCES clients(id)
);

CREATE TABLE IF NOT EXISTS projections (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    client_id INTEGER,
    projection_date DATE,
    target_value REAL,
    probability REAL,
    notes TEXT,
    FOREIGN KEY (client_id) REFERENCES clients(id)
);

CREATE INDEX idx_clients_id ON clients(client_id);
CREATE INDEX idx_holdings_client ON holdings(client_id);
SQL

cp templates/finance/config.json pasture/cattle/finance-cow-v1/

echo ""
echo "✅ Setup complete!"
echo ""
echo "═══════════════════════════════════════════════════════════"
echo "  NEXT STEPS"
echo "═══════════════════════════════════════════════════════════"
echo ""
echo "1. Start the Ranch:"
echo "   $ make run"
echo ""
echo "2. Analyze a portfolio:"
echo "   'Analyze portfolio: [holdings data]'"
echo ""
echo "⚠️  IMPORTANT: All outputs require compliance review."
echo ""
