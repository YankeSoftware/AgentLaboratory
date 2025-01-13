@echo off
REM Ensure output directory exists
if not exist output mkdir output
if not exist research_dir mkdir research_dir
if not exist state_saves mkdir state_saves

REM Run the container with GPU support
docker-compose -f docker\docker-compose.win.yml run --rm agent-laboratory %*