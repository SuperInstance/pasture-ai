#!/bin/bash
# Developer Copilot Template Setup Script

set -e

echo "═══════════════════════════════════════════════════════════"
echo "  💻 Developer Copilot Agent Setup"
echo "═══════════════════════════════════════════════════════════"

if [ ! -f "Makefile" ]; then
    echo "❌ Please run this script from the SuperInstance root directory"
    exit 1
fi

echo "📁 Creating pasture structure..."
mkdir -p pasture/cattle/developer-cow-v1

echo "🧬 Installing breed configuration..."
cp templates/developer/breed.md pasture/cattle/developer-cow-v1/
cp templates/developer/config.json pasture/cattle/developer-cow-v1/

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
echo "2. Request a code review:"
echo "   'Review this function for security issues: [code]'"
echo ""
echo "3. Debug an error:"
echo "   'Debug this Rust error: [error message]'"
echo ""
