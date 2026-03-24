#!/bin/bash
# Journalist Assistant Template Setup Script

set -e

echo "═══════════════════════════════════════════════════════════"
echo "  📰 Journalist Assistant Agent Setup"
echo "═══════════════════════════════════════════════════════════"

if [ ! -f "Makefile" ]; then
    echo "❌ Please run this script from the SuperInstance root directory"
    exit 1
fi

echo "📁 Creating pasture structure..."
mkdir -p pasture/cattle/journalist-cow-v1

echo "🧬 Installing breed configuration..."
cp templates/journalist/breed.md pasture/cattle/journalist-cow-v1/
cp templates/journalist/config.json pasture/cattle/journalist-cow-v1/

echo "💾 Setting up secure notes database..."
mkdir -p ~/.superinstance
sqlite3 ~/.superinstance/journalism.db <<'SQL'
CREATE TABLE IF NOT EXISTS stories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    working_title TEXT,
    status TEXT DEFAULT 'research',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS sources (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    story_id INTEGER,
    name_encrypted BLOB,
    contact_method TEXT,
    notes_encrypted BLOB,
    trust_level TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (story_id) REFERENCES stories(id)
);

CREATE TABLE IF NOT EXISTS documents (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    story_id INTEGER,
    title TEXT,
    file_path TEXT,
    source TEXT,
    verified INTEGER DEFAULT 0,
    notes TEXT,
    FOREIGN KEY (story_id) REFERENCES stories(id)
);

CREATE TABLE IF NOT EXISTS fact_checks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    story_id INTEGER,
    claim TEXT,
    verification_status TEXT,
    source TEXT,
    confidence TEXT,
    FOREIGN KEY (story_id) REFERENCES stories(id)
);

CREATE INDEX idx_sources_story ON sources(story_id);
CREATE INDEX idx_documents_story ON documents(story_id);
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
echo "2. Request a research brief:"
echo "   'Create a research brief about [investigative topic]'"
echo ""
echo "3. Prepare for an interview:"
echo "   'Prepare interview questions for [subject] about [topic]'"
echo ""
echo "🔒 SOURCE PROTECTION: All processing is LOCAL ONLY."
echo ""
