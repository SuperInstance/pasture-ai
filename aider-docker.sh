#!/bin/bash
# Dockerized Aider wrapper
# Usage: ./aider-docker.sh [command] [files...]

set -e

IMAGE_NAME="aider-sandbox"
WORKDIR="${1:-/home/aider/code}"
shift

# Create config file for Docker (copy from host)
CONFIG_TEMP=$(mktemp)
if [ -f ~/.aider.conf.yml ]; then
    cp ~/.aider.conf.yml "$CONFIG_TEMP"
else
    echo "# Aider config" > "$CONFIG_TEMP"
    echo "model: deepseek-chat" >> "$CONFIG_TEMP"
fi

# Run aider in Docker
docker run -it --rm \
    -v "$(pwd):/home/aider/code" \
    -v "$CONFIG_TEMP:/home/aider/.aider.conf.yml" \
    -w "/home/aider/code" \
    "$IMAGE_NAME" \
    aider "$@"

# Cleanup
rm -f "$CONFIG_TEMP"