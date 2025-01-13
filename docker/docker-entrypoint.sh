#!/bin/bash
set -e

# Function to safely create/clean directories
setup_directory() {
    local dir_path="$1"
    if [ -d "$dir_path" ]; then
        # Clean directory contents but preserve directory
        rm -rf "${dir_path:?}"/* 2>/dev/null || true
    else
        # Create directory if it doesn't exist
        mkdir -p "$dir_path"
    fi
    # Ensure proper permissions
    chmod 777 "$dir_path"
}

# Setup required directories
setup_directory "/workspace/research_dir"
setup_directory "/workspace/state_saves"
setup_directory "/workspace/output"

# Verify CUDA setup
if [ -x "$(command -v nvidia-smi)" ]; then
    echo "CUDA devices found:"
    nvidia-smi
else
    echo "Warning: nvidia-smi not found. GPU may not be available."
fi

# Execute the main command
exec "$@"