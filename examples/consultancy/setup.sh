#!/bin/bash
# Consultancy Ranch Setup
# Usage: ./setup.sh

echo "🐄 Setting up Consultancy Ranch..."

# Copy breed files
mkdir -p pasture/cattle/consultancy-cow-v1
cp breed.md pasture/cattle/consultancy-cow-v1/

# Create gene traits if they don't exist
mkdir -p genetics/traits/research_synthesis
cat > genetics/traits/research_synthesis/meta.json << 'EOF'
{
  "id": "research_synthesis",
  "name": "Research Synthesis",
  "description": "Synthesizes multiple sources into coherent summaries",
  "size_bytes": 50000000,
  "compatible_species": ["Cattle"],
  "tags": ["domain", "output"]
}
EOF

echo "✅ Consultancy Ranch ready!"
echo "   Run: make run"
