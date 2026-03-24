#!/bin/bash
# Education Assistant Template Setup Script

set -e

echo "═══════════════════════════════════════════════════════════"
echo "  📚 Education Assistant Agent Setup"
echo "═══════════════════════════════════════════════════════════"

if [ ! -f "Makefile" ]; then
    echo "❌ Please run this script from the SuperInstance root directory"
    exit 1
fi

echo "📁 Creating pasture structure..."
mkdir -p pasture/cattle/education-cow-v1

echo "🧬 Installing breed configuration..."
cp templates/education/breed.md pasture/cattle/education-cow-v1/

echo "💾 Setting up local student database..."
mkdir -p ~/.superinstance
sqlite3 ~/.superinstance/students.db <<'SQL'
CREATE TABLE IF NOT EXISTS students (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    student_id TEXT UNIQUE NOT NULL,
    first_name TEXT,
    last_name TEXT,
    grade_level INTEGER,
    homeroom TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS assignments (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    student_id INTEGER,
    assignment_name TEXT,
    subject TEXT,
    score REAL,
    max_score REAL,
    feedback TEXT,
    graded_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (student_id) REFERENCES students(id)
);

CREATE TABLE IF NOT EXISTS lesson_plans (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    subject TEXT,
    grade_level INTEGER,
    topic TEXT,
    objectives TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    last_used DATETIME
);
SQL

cp templates/education/config.json pasture/cattle/education-cow-v1/

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
echo "2. Request a lesson plan:"
echo "   'Create a 3rd grade science lesson about plant life cycles'"
echo ""
echo "3. Request student feedback:"
echo "   'Write feedback for student Emma who scored 8/10 on fractions'"
echo ""
