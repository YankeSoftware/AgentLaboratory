#!/bin/bash

# Ensure the script exits on any error
set -e

# Create required directories if they don't exist
mkdir -p output research_dir state_saves

# Set proper permissions
chmod 777 output research_dir state_saves

# Run the container with proper GPU support
docker run --gpus all \
  -it --rm \
  --runtime=nvidia \
  -e NVIDIA_VISIBLE_DEVICES=0 \
  -e NVIDIA_DRIVER_CAPABILITIES=compute,utility \
  -e TF_FORCE_GPU_ALLOW_GROWTH=true \
  -e TF_CPP_MIN_LOG_LEVEL=2 \
  -v "$(pwd):/workspace" \
  -v "$(pwd)/output:/output" \
  -e OPENAI_API_KEY="${OPENAI_API_KEY}" \
  -e DEEPSEEK_API_KEY="${DEEPSEEK_API_KEY}" \
  -w /workspace \
  agent-laboratory \
  "$@"