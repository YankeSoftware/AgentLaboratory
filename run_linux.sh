#!/bin/bash

# Ensure required directories exist
mkdir -p output research_dir state_saves

# Run the container with GPU support
docker-compose -f docker/docker-compose.yml run --rm agent-laboratory "$@"