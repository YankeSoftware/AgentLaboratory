# GPU-Optimized Docker Configuration

Optimized setup for GPU-accelerated machine learning:
- Full CUDA support
- Optimized cuDNN and cuBLAS
- Performance-tuned for ML workloads
- Advanced GPU memory management

## Build Time
Approximately 25-35 minutes

## Features
- CUDA 11.8 with cuDNN 8
- Optimized GPU memory handling
- Full ML framework support
- Advanced CUDA configurations

## Usage

### Windows
```cmd
run_windows.bat gpu
```

### Linux
```bash
./run_linux.sh gpu
```

## Build Manually
```bash
cd docker/gpu
docker-compose build
```

## Environment Variables
- `OPENAI_API_KEY`: Your OpenAI API key
- `DEEPSEEK_API_KEY`: Your DeepSeek API key
- `CUDA_VISIBLE_DEVICES`: GPU device selection
- `TF_FORCE_GPU_ALLOW_GROWTH`: Prevents TensorFlow from allocating all GPU memory
- `NVIDIA_VISIBLE_DEVICES`: GPU visibility for container
- `NVIDIA_DRIVER_CAPABILITIES`: Required NVIDIA capabilities

## Hardware Requirements
- NVIDIA GPU with CUDA support
- Minimum 8GB GPU memory recommended
- Compatible NVIDIA drivers installed

## Notes
- Best for machine learning tasks
- Includes advanced GPU memory management
- Optimized CUDA configurations
- Proper library version matching