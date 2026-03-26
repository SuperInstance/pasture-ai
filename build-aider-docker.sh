#!/bin/bash
# Build the Aider Docker image

set -e

IMAGE_NAME="aider-sandbox"
DOCKERFILE="Dockerfile.aider"

echo "Building Aider Docker image..."
docker build -t $IMAGE_NAME -f $DOCKERFILE .

echo "✅ Image built: $IMAGE_NAME"
echo ""
echo "To run:"
echo "  docker run -it --rm -v \$(pwd):/home/aider/code $IMAGE_NAME aider"
echo ""
echo "Or with your DeepSeek config:"
echo "  docker run -it --rm \\"
echo "    -v \$(pwd):/home/aider/code \\"
echo "    -v ~/.aider.conf.yml:/home/aider/.aider.conf.yml \\"
echo "    $IMAGE_NAME aider"