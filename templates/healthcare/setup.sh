#!/bin/bash
# Healthcare Triage Template Setup Script
# Sets up a complete healthcare triage agent for remote workers

set -e

echo "═══════════════════════════════════════════════════════════"
echo "  🏥 Healthcare Triage Agent Setup"
echo "═══════════════════════════════════════════════════════════"

# Check if running in Ranch directory
if [ ! -f "Makefile" ]; then
    echo "❌ Please run this script from the SuperInstance root directory"
    exit 1
fi

# Create pasture directory
echo "📁 Creating pasture structure..."
mkdir -p pasture/cattle/healthcare-cow-v1

# Copy breed file
echo "🧬 Installing breed configuration..."
cp templates/healthcare/breed.md pasture/cattle/healthcare-cow-v1/

# Create local database
echo "💾 Setting up local patient database..."
mkdir -p ~/.superinstance
sqlite3 ~/.superinstance/patients.db <<'SQL'
CREATE TABLE IF NOT EXISTS patients (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    mrn TEXT UNIQUE NOT NULL,
    first_name TEXT,
    last_name TEXT,
    date_of_birth DATE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS encounters (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    patient_id INTEGER,
    encounter_date DATETIME DEFAULT CURRENT_TIMESTAMP,
    chief_complaint TEXT,
    triage_level TEXT,
    provider TEXT,
    notes TEXT,
    FOREIGN KEY (patient_id) REFERENCES patients(id)
);

CREATE TABLE IF NOT EXISTS vitals (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    encounter_id INTEGER,
    recorded_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    bp_systolic INTEGER,
    bp_diastolic INTEGER,
    heart_rate INTEGER,
    temperature REAL,
    spo2 INTEGER,
    respiratory_rate INTEGER,
    pain_level INTEGER,
    FOREIGN KEY (encounter_id) REFERENCES encounters(id)
);

CREATE INDEX idx_patients_mrn ON patients(mrn);
CREATE INDEX idx_encounters_patient ON encounters(patient_id);
SQL

# Copy config
echo "⚙️  Installing configuration..."
cp templates/healthcare/config.json pasture/cattle/healthcare-cow-v1/

# Create examples directory
mkdir -p pasture/cattle/healthcare-cow-v1/examples

# Success message
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
echo "2. Test the triage agent:"
echo "   Send a message like: 'Patient with chest pain, BP 150/90'"
echo ""
echo "3. Customize the breed:"
echo "   Edit pasture/cattle/healthcare-cow-v1/breed.md"
echo ""
echo "⚠️  IMPORTANT: This AI assists but does not replace clinical judgment."
echo "   All outputs should be reviewed by licensed healthcare professionals."
echo ""
