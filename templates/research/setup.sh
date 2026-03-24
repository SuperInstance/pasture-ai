#!/bin/bash
# Research Assistant Template Setup Script

set -e

echo "═══════════════════════════════════════════════════════════"
echo "  🔬 Research Assistant Agent Setup"
echo "═══════════════════════════════════════════════════════════"

if [ ! -f "Makefile" ]; then
    echo "❌ Please run this script from the SuperInstance root directory"
    exit 1
fi

echo "📁 Creating pasture structure..."
mkdir -p pasture/cattle/research-cow-v1

echo "🧬 Installing breed configuration..."
cp templates/research/breed.md pasture/cattle/research-cow-v1/

echo "💾 Setting up research database..."
mkdir -p ~/.superinstance
sqlite3 ~/.superinstance/research.db <<'SQL'
CREATE TABLE IF NOT EXISTS projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_name TEXT,
    field TEXT,
    status TEXT DEFAULT 'active',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS sources (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER,
    citation_key TEXT,
    title TEXT,
    authors TEXT,
    year INTEGER,
    source_type TEXT,
    notes TEXT,
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

CREATE TABLE IF NOT EXISTS drafts (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER,
    document_type TEXT,
    content_path TEXT,
    version INTEGER DEFAULT 1,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

CREATE INDEX idx_sources_project ON sources(project_id);
SQL

cp templates/research/config.json pasture/cattle/research-cow-v1/

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
echo "2. Request a literature review:"
echo "   'Create a literature review on [topic] from [years]'"
echo ""
echo "⚠️  IMPORTANT: Always verify citations against original sources."
echo ""
