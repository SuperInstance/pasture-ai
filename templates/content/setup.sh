#!/bin/bash
# Content Creator Template Setup Script

set -e

echo "═══════════════════════════════════════════════════════════"
echo "  ✍️ Content Creator Agent Setup"
echo "═══════════════════════════════════════════════════════════"

if [ ! -f "Makefile" ]; then
    echo "❌ Please run this script from the SuperInstance root directory"
    exit 1
fi

echo "📁 Creating pasture structure..."
mkdir -p pasture/cattle/content-cow-v1

echo "🧬 Installing breed configuration..."
cp templates/content/breed.md pasture/cattle/content-cow-v1/
cp templates/content/config.json pasture/cattle/content-cow-v1/

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
echo "2. Request a YouTube script:"
echo "   'Create a YouTube script about [topic]'"
echo ""
echo "3. Plan your content:"
echo "   'Create a monthly content calendar for [niche]'"
echo ""
