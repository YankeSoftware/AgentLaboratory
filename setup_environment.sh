#!/bin/bash

# Detect OS
if [[ "$OSTYPE" == "linux-gnu"* ]]; then
    echo "Detected Linux OS"
    
    # Check for sudo access
    if command -v sudo >/dev/null 2>&1; then
        SUDO="sudo"
    else
        SUDO=""
    fi
    
    # Install Docker if not present
    if ! command -v docker >/dev/null 2>&1; then
        echo "Installing Docker..."
        $SUDO apt-get update
        $SUDO apt-get install -y \
            apt-transport-https \
            ca-certificates \
            curl \
            gnupg \
            lsb-release
        curl -fsSL https://download.docker.com/linux/ubuntu/gpg | $SUDO gpg --dearmor -o /usr/share/keyrings/docker-archive-keyring.gpg
        echo \
          "deb [arch=amd64 signed-by=/usr/share/keyrings/docker-archive-keyring.gpg] https://download.docker.com/linux/ubuntu \
          $(lsb_release -cs) stable" | $SUDO tee /etc/apt/sources.list.d/docker.list > /dev/null
        $SUDO apt-get update
        $SUDO apt-get install -y docker-ce docker-ce-cli containerd.io
    fi
    
    # Install NVIDIA Container Toolkit if GPU present
    if command -v nvidia-smi >/dev/null 2>&1; then
        echo "Installing NVIDIA Container Toolkit..."
        distribution=$(. /etc/os-release;echo $ID$VERSION_ID)
        curl -s -L https://nvidia.github.io/libnvidia-container/gpgkey | \
            $SUDO apt-key add -
        curl -s -L https://nvidia.github.io/libnvidia-container/$distribution/libnvidia-container.list | \
            $SUDO tee /etc/apt/sources.list.d/nvidia-container-toolkit.list
        $SUDO apt-get update
        $SUDO apt-get install -y nvidia-container-toolkit
    fi
    
    # Setup permissions
    $SUDO usermod -aG docker $USER
    $SUDO systemctl restart docker
    
elif [[ "$OSTYPE" == "msys" ]] || [[ "$OSTYPE" == "win32" ]]; then
    echo "Detected Windows OS"
    echo "Please install:"
    echo "1. Docker Desktop for Windows from: https://www.docker.com/products/docker-desktop"
    echo "2. WSL2 from: https://docs.microsoft.com/en-us/windows/wsl/install"
    echo "3. NVIDIA Container Toolkit (if using GPU) from: https://docs.nvidia.com/datacenter/cloud-native/container-toolkit/install-guide.html#windows"
else
    echo "Unsupported OS: $OSTYPE"
    exit 1
fi

# Create required directories
mkdir -p output research_dir state_saves
chmod 777 output research_dir state_saves

echo "Environment setup complete!"
echo "Please logout and login again for group changes to take effect (Linux only)"