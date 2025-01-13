import os
import pytest
import subprocess
import platform
import docker
import shutil
from pathlib import Path

class DockerTestEnvironment:
    def __init__(self):
        self.client = docker.from_env()
        self.test_dir = Path(__file__).parent
        self.project_root = self.test_dir.parent.parent
        
    def cleanup(self):
        """Remove all test containers and images"""
        for container in self.client.containers.list(all=True):
            if container.name.startswith('test_'):
                container.remove(force=True)
        
        for image in self.client.images.list():
            if any(tag.startswith('test_') for tag in image.tags):
                self.client.images.remove(image.id, force=True)

@pytest.fixture(scope="module")
def docker_env():
    env = DockerTestEnvironment()
    yield env
    env.cleanup()

@pytest.fixture(scope="module")
def test_directories(tmp_path_factory):
    base_dir = tmp_path_factory.mktemp("docker_test")
    dirs = {
        'output': base_dir / 'output',
        'research_dir': base_dir / 'research_dir',
        'state_saves': base_dir / 'state_saves'
    }
    for dir_path in dirs.values():
        dir_path.mkdir(exist_ok=True)
    return dirs

def test_docker_standard_build(docker_env):
    """Test standard Docker build"""
    dockerfile_path = docker_env.project_root / 'docker/standard/Dockerfile'
    assert dockerfile_path.exists(), "Standard Dockerfile not found"
    
    client = docker.from_env()
    image, logs = client.images.build(
        path=str(docker_env.project_root),
        dockerfile=str(dockerfile_path),
        tag='test_agent_lab_standard'
    )
    assert image is not None, "Standard build failed"

def test_docker_minimal_build(docker_env):
    """Test minimal Docker build"""
    dockerfile_path = docker_env.project_root / 'docker/minimal/Dockerfile'
    assert dockerfile_path.exists(), "Minimal Dockerfile not found"
    
    client = docker.from_env()
    image, logs = client.images.build(
        path=str(docker_env.project_root),
        dockerfile=str(dockerfile_path),
        tag='test_agent_lab_minimal'
    )
    assert image is not None, "Minimal build failed"

def test_docker_gpu_build(docker_env):
    """Test GPU-enabled Docker build"""
    dockerfile_path = docker_env.project_root / 'docker/gpu/Dockerfile'
    assert dockerfile_path.exists(), "GPU Dockerfile not found"
    
    client = docker.from_env()
    image, logs = client.images.build(
        path=str(docker_env.project_root),
        dockerfile=str(dockerfile_path),
        tag='test_agent_lab_gpu'
    )
    assert image is not None, "GPU build failed"

@pytest.mark.parametrize("config", ["standard", "minimal", "gpu"])
def test_docker_run(docker_env, test_directories, config):
    """Test Docker container running with different configurations"""
    image_tag = f'test_agent_lab_{config}'
    
    # Prepare environment variables
    env = {
        'OPENAI_API_KEY': 'test_key',
        'DEEPSEEK_API_KEY': 'test_key',
        'TF_CPP_MIN_LOG_LEVEL': '2'
    }
    
    # Prepare volume mounts
    volumes = {
        str(test_directories['output']): {'bind': '/workspace/output', 'mode': 'rw'},
        str(test_directories['research_dir']): {'bind': '/workspace/research_dir', 'mode': 'rw'},
        str(test_directories['state_saves']): {'bind': '/workspace/state_saves', 'mode': 'rw'}
    }
    
    # Run container
    container = docker_env.client.containers.run(
        image_tag,
        command=['python', '-c', 'import sys; print(sys.version)'],
        environment=env,
        volumes=volumes,
        detach=True
    )
    
    # Wait for container to finish and check exit code
    result = container.wait()
    assert result['StatusCode'] == 0, f"Container exited with non-zero status: {result['StatusCode']}"
    
    # Check logs
    logs = container.logs().decode('utf-8')
    assert 'Python' in logs, "Python environment not properly set up"

@pytest.mark.skipif(platform.system() != 'Linux', reason="GPU tests only on Linux")
def test_gpu_support(docker_env):
    """Test GPU support in Docker container"""
    # Check if nvidia-smi is available
    try:
        subprocess.run(['nvidia-smi'], check=True, capture_output=True)
    except (subprocess.CalledProcessError, FileNotFoundError):
        pytest.skip("No GPU available")
    
    image_tag = 'test_agent_lab_gpu'
    
    # Run container with GPU support
    container = docker_env.client.containers.run(
        image_tag,
        command=['nvidia-smi'],
        runtime='nvidia',
        detach=True
    )
    
    result = container.wait()
    assert result['StatusCode'] == 0, "GPU test failed"

def test_directory_permissions(docker_env, test_directories):
    """Test directory permissions and ownership"""
    image_tag = 'test_agent_lab_standard'
    
    container = docker_env.client.containers.run(
        image_tag,
        command=['ls', '-la', '/workspace/output'],
        volumes={
            str(test_directories['output']): {'bind': '/workspace/output', 'mode': 'rw'}
        },
        user='1000:1000',
        detach=True
    )
    
    result = container.wait()
    assert result['StatusCode'] == 0, "Permission test failed"

def test_environment_variables(docker_env):
    """Test environment variable handling"""
    image_tag = 'test_agent_lab_standard'
    test_vars = {
        'OPENAI_API_KEY': 'test_key',
        'DEEPSEEK_API_KEY': 'test_key',
        'CUSTOM_VAR': 'test_value'
    }
    
    container = docker_env.client.containers.run(
        image_tag,
        command=['env'],
        environment=test_vars,
        detach=True
    )
    
    result = container.wait()
    assert result['StatusCode'] == 0, "Environment variable test failed"
    
    logs = container.logs().decode('utf-8')
    for var, value in test_vars.items():
        assert f"{var}={value}" in logs, f"Environment variable {var} not properly set"