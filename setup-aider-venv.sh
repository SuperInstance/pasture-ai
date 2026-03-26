#!/bin/bash
# Setup Aider in a virtual environment for sandboxing

set -e

VENV_DIR="$HOME/.aider-venv"
REQUIREMENTS="$VENV_DIR/requirements.txt"

echo "Setting up Aider virtual environment in $VENV_DIR..."

# Create virtual environment
python3 -m venv "$VENV_DIR"

# Create requirements file
cat > "$REQUIREMENTS" << 'EOF'
aider-chat>=0.86.0
gitpython>=3.1.0
EOF

# Install packages
source "$VENV_DIR/bin/activate"
pip install -r "$REQUIREMENTS"

echo "✅ Virtual environment setup complete!"
echo ""
echo "To activate:"
echo "  source $VENV_DIR/bin/activate"
echo ""
echo "To run aider:"
echo "  $VENV_DIR/bin/aider"
echo ""
echo "For OpenClaw integration, use:"
echo "  $VENV_DIR/bin/python $HOME/.openclaw/workspace/aider-bridge.py 'your message'"