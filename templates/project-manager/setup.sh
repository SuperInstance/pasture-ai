#!/bin/bash
# Project Manager Template Setup Script

set -e

echo "═══════════════════════════════════════════════════════════"
echo "  📊 Project Manager Agent Setup"
echo "═══════════════════════════════════════════════════════════"

if [ ! -f "Makefile" ]; then
    echo "❌ Please run this script from the SuperInstance root directory"
    exit 1
fi

echo "📁 Creating pasture structure..."
mkdir -p pasture/cattle/pm-cow-v1

echo "🧬 Installing breed configuration..."
cp templates/project-manager/breed.md pasture/cattle/pm-cow-v1/
cp templates/project-manager/config.json pasture/cattle/pm-cow-v1/

echo "💾 Setting up project database..."
mkdir -p ~/.superinstance
sqlite3 ~/.superinstance/projects.db <<'SQL'
CREATE TABLE IF NOT EXISTS projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_name TEXT,
    status TEXT DEFAULT 'active',
    start_date DATE,
    target_end_date DATE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS sprints (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER,
    sprint_number INTEGER,
    start_date DATE,
    end_date DATE,
    goal TEXT,
    velocity INTEGER,
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

CREATE TABLE IF NOT EXISTS tasks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    sprint_id INTEGER,
    title TEXT,
    description TEXT,
    assignee TEXT,
    estimate INTEGER,
    status TEXT DEFAULT 'todo',
    completed_at DATETIME,
    FOREIGN KEY (sprint_id) REFERENCES sprints(id)
);

CREATE TABLE IF NOT EXISTS risks (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER,
    description TEXT,
    probability TEXT,
    impact TEXT,
    mitigation TEXT,
    owner TEXT,
    status TEXT DEFAULT 'open',
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

CREATE INDEX idx_tasks_sprint ON tasks(sprint_id);
CREATE INDEX idx_risks_project ON risks(project_id);
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
echo "2. Create a status report:"
echo "   'Generate a weekly status report for [project name]'"
echo ""
echo "3. Plan a meeting:"
echo "   'Create an agenda for a [type] meeting'"
echo ""
