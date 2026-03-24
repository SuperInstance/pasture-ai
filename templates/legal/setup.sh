#!/bin/bash
# Legal Review Template Setup Script

set -e

echo "═══════════════════════════════════════════════════════════"
echo "  ⚖️ Legal Review Agent Setup"
echo "═══════════════════════════════════════════════════════════"

if [ ! -f "Makefile" ]; then
    echo "❌ Please run this script from the SuperInstance root directory"
    exit 1
fi

echo "📁 Creating pasture structure..."
mkdir -p pasture/cattle/legal-cow-v1

echo "🧬 Installing breed configuration..."
cp templates/legal/breed.md pasture/cattle/legal-cow-v1/

echo "💾 Setting up clause library..."
mkdir -p ~/.superinstance
sqlite3 ~/.superinstance/legal.db <<'SQL'
CREATE TABLE IF NOT EXISTS matters (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    matter_number TEXT UNIQUE NOT NULL,
    client_name TEXT,
    matter_type TEXT,
    status TEXT DEFAULT 'active',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS documents (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    matter_id INTEGER,
    document_type TEXT,
    file_path TEXT,
    reviewed_at DATETIME,
    risk_level TEXT,
    notes TEXT,
    FOREIGN KEY (matter_id) REFERENCES matters(id)
);

CREATE TABLE IF NOT EXISTS clause_library (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    category TEXT,
    clause_name TEXT,
    clause_text TEXT,
    jurisdiction TEXT,
    notes TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_matters_number ON matters(matter_number);
CREATE INDEX idx_documents_matter ON documents(matter_id);
SQL

cp templates/legal/config.json pasture/cattle/legal-cow-v1/

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
echo "2. Review a contract:"
echo "   'Review this NDA for risks: [paste contract text]'"
echo ""
echo "⚠️  IMPORTANT: This AI assists but does not replace legal counsel."
echo ""
