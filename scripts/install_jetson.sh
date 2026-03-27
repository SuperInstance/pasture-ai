#!/bin/bash
# SuperInstance Jetson Install
# Delegates to the full install script bundled with the superinstance crate.

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FULL_INSTALL="$SCRIPT_DIR/../superinstance/scripts/install_jetson.sh"

if [ ! -f "$FULL_INSTALL" ]; then
    echo "Error: Full install script not found at $FULL_INSTALL" >&2
    echo "Make sure you have cloned the full repository." >&2
    exit 1
fi

exec bash "$FULL_INSTALL" "$@"
