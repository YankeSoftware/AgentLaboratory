# Docker Support for Agent Laboratory

This directory contains Docker configurations for running Agent Laboratory in different environments.

## Quick Start 🚀

### Windows Users
```cmd
run_windows.bat [standard|minimal|gpu]
```

### Linux Users
```bash
./run_linux.sh [standard|minimal|gpu]
```

## Available Configurations

### 1. Standard Setup (`/standard`)
- Full feature set with LaTeX support
- GPU acceleration enabled
- All ML libraries included
- Build time: ~20-30 minutes

### 2. Minimal Setup (`/minimal`)
- Basic features without LaTeX
- Faster build time (~5-10 minutes)
- Smaller image size
- Perfect for quick experiments

### 3. GPU-Optimized (`/gpu`)
- CUDA optimization
- cuDNN and cuBLAS properly configured
- Best for machine learning tasks
- Full GPU acceleration support

## Testing Your Setup 🧪

We provide a comprehensive testing framework to validate your deployment:

```bash
# Install test dependencies
pip install pytest pytest-cov docker

# Run platform-specific tests
python -m pytest tests/deployment/windows/  # For Windows
python -m pytest tests/deployment/linux/    # For Linux

# Test Docker builds
python -m pytest tests/deployment/test_docker_builds.py
```

## Troubleshooting 🔧

### Common Issues

#### Windows
- WSL2 not enabled: Run `wsl --set-default-version 2`
- Docker Desktop not running: Check Docker Desktop status
- Path issues: Use proper path format in docker-compose.win.yml

#### Linux
- Permission denied: Add user to docker group
- GPU not detected: Install NVIDIA Container Toolkit
- Directory access: Check mount point permissions

## Contributing 🤝

Found a bug or want to improve something? Feel free to:
1. Test on your platform
2. Add platform-specific fixes
3. Update documentation
4. Submit test cases

## Notes 📝

- All configurations use the same entrypoint script
- Volume mounting is handled automatically
- GPU support requires proper drivers and toolkit
- See individual README files in each configuration directory for specific details