# Minimal Docker Configuration

A lightweight setup without LaTeX support, perfect for quick experiments and development:
- Basic Python environment
- Essential machine learning packages
- No LaTeX compilation tools
- Smaller image size

## Build Time
Approximately 5-10 minutes

## Features
- Core Python packages only
- Basic machine learning support
- Reduced image size (~2GB smaller than standard)
- Faster build and startup times

## Usage

### Windows
```cmd
run_windows.bat minimal
```

### Linux
```bash
./run_linux.sh minimal
```

## Build Manually
```bash
cd docker/minimal
docker-compose build
```

## Environment Variables
- `OPENAI_API_KEY`: Your OpenAI API key
- `DEEPSEEK_API_KEY`: Your DeepSeek API key
- `CUDA_VISIBLE_DEVICES`: GPU device selection
- `TF_FORCE_GPU_ALLOW_GROWTH`: Prevents TensorFlow from allocating all GPU memory

## Notes
- Best for development and testing
- No LaTeX document generation
- Faster iteration cycles
- Good for CI/CD pipelines