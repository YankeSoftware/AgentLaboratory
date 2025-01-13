import os
import pytest
import subprocess
import pwd
import grp
from pathlib import Path

def get_docker_group():
    """Get docker group information"""
    try:
        return grp.getgrnam('docker')
    except KeyError:
        return None

def test_docker_installation():
    """Test Docker installation on Linux"""
    assert subprocess.run(['docker', '--version'], 
                        capture_output=True).returncode == 0, \
           "Docker is not properly installed"

def test_docker_service():
    """Test Docker service status"""
    service_status = subprocess.run(['systemctl', 'is-active', 'docker'], 
                                  capture_output=True, text=True)
    assert service_status.stdout.strip() == 'active', \
           "Docker service is not running"

def test_docker_group_exists():
    """Test if docker group exists"""
    docker_group = get_docker_group()
    assert docker_group is not None, "Docker group does not exist"

def test_user_in_docker_group():
    """Test if current user is in docker group"""
    docker_group = get_docker_group()
    if docker_group is None:
        pytest.skip("Docker group does not exist")
    
    username = pwd.getpwuid(os.getuid())[0]
    assert username in docker_group.gr_mem, \
           f"User {username} is not in docker group"

def test_nvidia_docker():
    """Test NVIDIA Docker support"""
    try:
        subprocess.run(['nvidia-docker', 'version'], 
                      capture_output=True, check=True)
    except (subprocess.CalledProcessError, FileNotFoundError):
        pytest.skip("NVIDIA Docker not installed")

@pytest.mark.skipif(not os.path.exists('/usr/bin/nvidia-smi'),
                    reason="NVIDIA driver not installed")
def test_gpu_access():
    """Test GPU access from Docker container"""
    try:
        result = subprocess.run(
            ['docker', 'run', '--rm', '--gpus', 'all', 
             'nvidia/cuda:11.8.0-base-ubuntu22.04', 'nvidia-smi'],
            capture_output=True,
            text=True,
            check=True
        )
        assert 'NVIDIA-SMI' in result.stdout
    except subprocess.CalledProcessError:
        pytest.fail("Failed to access GPU from Docker container")

def test_directory_permissions():
    """Test directory permissions and ownership"""
    test_dir = Path('/tmp/docker_test')
    test_dir.mkdir(exist_ok=True)
    
    try:
        # Set proper permissions
        test_dir.chmod(0o777)
        
        # Test mounting with current user
        result = subprocess.run(
            ['docker', 'run', '--rm', '-v', 
             f"{test_dir}:/test", '--user', f"{os.getuid()}:{os.getgid()}", 
             'alpine', 'touch', '/test/testfile'],
            capture_output=True,
            check=True
        )
        
        test_file = test_dir / 'testfile'
        assert test_file.exists(), "Failed to create file in mounted volume"
        assert test_file.owner() == pwd.getpwuid(os.getuid())[0], \
               "File ownership incorrect"
    finally:
        subprocess.run(['rm', '-rf', str(test_dir)])

def test_selinux_context():
    """Test SELinux context handling if applicable"""
    try:
        subprocess.run(['getenforce'], capture_output=True, check=True)
    except (subprocess.CalledProcessError, FileNotFoundError):
        pytest.skip("SELinux not available")
    
    test_dir = Path('/tmp/selinux_test')
    test_dir.mkdir(exist_ok=True)
    
    try:
        result = subprocess.run(
            ['docker', 'run', '--rm', '-v', 
             f"{test_dir}:/test:Z", 'alpine', 'touch', '/test/testfile'],
            capture_output=True,
            check=True
        )
        assert (test_dir / 'testfile').exists(), \
               "Failed to create file with SELinux context"
    finally:
        subprocess.run(['rm', '-rf', str(test_dir)])

def test_cgroup_configuration():
    """Test cgroup configuration"""
    cgroup_path = Path('/sys/fs/cgroup/memory/docker')
    if not cgroup_path.exists():
        cgroup_path = Path('/sys/fs/cgroup/docker')  # cgroups v2
    
    assert any(cgroup_path.exists() for cgroup_path in 
              [Path('/sys/fs/cgroup/memory/docker'), 
               Path('/sys/fs/cgroup/docker')]), \
           "Docker cgroup configuration not found"

def test_network_isolation():
    """Test network isolation in Docker containers"""
    # Create two containers and test network isolation
    network_name = 'test_network'
    
    try:
        # Create network
        subprocess.run(['docker', 'network', 'create', network_name], 
                      check=True)
        
        # Run first container
        subprocess.run(
            ['docker', 'run', '--rm', '--network', network_name, 
             '--name', 'container1', '-d', 'alpine', 'sleep', '30'],
            check=True
        )
        
        # Test network connectivity from second container
        result = subprocess.run(
            ['docker', 'run', '--rm', '--network', network_name, 
             'alpine', 'ping', '-c', '1', 'container1'],
            capture_output=True,
            text=True
        )
        assert result.returncode == 0, "Container networking failed"
    finally:
        # Cleanup
        subprocess.run(['docker', 'container', 'rm', '-f', 'container1'], 
                      capture_output=True)
        subprocess.run(['docker', 'network', 'rm', network_name], 
                      capture_output=True)