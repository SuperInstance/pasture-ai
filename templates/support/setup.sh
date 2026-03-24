#!/bin/bash
# Customer Support Template Setup Script

set -e

echo "═══════════════════════════════════════════════════════════"
echo "  🎧 Customer Support Agent Setup"
echo "═══════════════════════════════════════════════════════════"

if [ ! -f "Makefile" ]; then
    echo "❌ Please run this script from the SuperInstance root directory"
    exit 1
fi

echo "📁 Creating pasture structure..."
mkdir -p pasture/sheep/support-sheep-v1

echo "🧬 Installing breed configuration..."
cp templates/support/breed.md pasture/sheep/support-sheep-v1/
cp templates/support/config.json pasture/sheep/support-sheep-v1/

echo "💾 Setting up ticket database..."
mkdir -p ~/.superinstance
sqlite3 ~/.superinstance/support.db <<'SQL'
CREATE TABLE IF NOT EXISTS tickets (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ticket_id TEXT UNIQUE NOT NULL,
    customer_id TEXT,
    subject TEXT,
    priority TEXT,
    category TEXT,
    status TEXT DEFAULT 'open',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    resolved_at DATETIME,
    satisfaction_score INTEGER
);

CREATE TABLE IF NOT EXISTS responses (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ticket_id INTEGER,
    response_text TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (ticket_id) REFERENCES tickets(id)
);

CREATE TABLE IF NOT EXISTS knowledge_base (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT,
    content TEXT,
    category TEXT,
    helpful_count INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_tickets_status ON tickets(status);
CREATE INDEX idx_tickets_priority ON tickets(priority);
SQL

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
echo "2. Triage a ticket:"
echo "   'Triage this support ticket: [ticket details]'"
echo ""
echo "3. Create a knowledge base article:"
echo "   'Write a KB article about [topic]'"
echo ""
