# Standard Docker Configuration

This is the standard setup with full features including:
- LaTeX support for document generation
- GPU acceleration for machine learning tasks
- All required Python packages and dependencies
- Proper CUDA configuration

## Build Time
Approximately 20-30 minutes depending on your system

## Features
- Full LaTeX suite for document generation
- GPU support with CUDA and cuDNN
- All machine learning libraries (TensorFlow, PyTorch)
- Complete Python environment

## Usage

### Windows
```cmd
run_windows.bat standard
```

### Linux
```bash
./run_linux.sh standard
```

## Build Manually
```bash
cd docker/standard
docker-compose build
```

## Environment Variables
- `OPENAI_API_KEY`: Your OpenAI API key
- `DEEPSEEK_API_KEY`: Your DeepSeek API key
- `CUDA_VISIBLE_DEVICES`: GPU device selection
- `TF_FORCE_GPU_ALLOW_GROWTH`: Prevents TensorFlow from allocating all GPU memory

## Notes
- This is the recommended setup for most users
- Requires approximately 10GB of disk space
- Needs a GPU with CUDA support for full functionality