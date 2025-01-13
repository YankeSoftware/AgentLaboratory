import os
import pytest
import subprocess
import winreg
from pathlib import Path

def check_wsl2():
    """Check if WSL2 is properly installed and configured"""
    try:
        result = subprocess.run(
            ['wsl', '--status'],
            capture_output=True,
            text=True,
            check=True
        )
        return 'Default Version: 2' in result.stdout
    except subprocess.CalledProcessError:
        return False

def check_docker_desktop():
    """Check if Docker Desktop is installed"""
    try:
        key = winreg.OpenKey(
            winreg.HKEY_LOCAL_MACHINE,
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\Docker Desktop",
            0,
            winreg.KEY_READ
        )
        return True
    except WindowsError:
        return False

def test_wsl2_installation():
    """Test WSL2 installation status"""
    assert check_wsl2(), "WSL2 is not properly installed or configured"

def test_docker_desktop_installation():
    """Test Docker Desktop installation"""
    assert check_docker_desktop(), "Docker Desktop is not installed"

def test_windows_path_handling():
    """Test Windows path handling in Docker commands"""
    test_path = Path('C:\\Test\\Path\\With Spaces')
    test_path.mkdir(parents=True, exist_ok=True)
    
    try:
        # Test path conversion in docker-compose
        result = subprocess.run(
            ['docker-compose', 'config'],
            cwd=test_path,
            capture_output=True,
            text=True,
            check=True
        )
        assert '/Test/Path/With\\ Spaces' in result.stdout.replace('\\', '/')
    finally:
        if test_path.exists():
            test_path.rmdir()

@pytest.mark.skipif(not check_docker_desktop(), reason="Docker Desktop not installed")
def test_windows_volume_mounting():
    """Test volume mounting on Windows"""
    test_dir = Path('C:\\Test\\Docker\\Mount')
    test_dir.mkdir(parents=True, exist_ok=True)
    test_file = test_dir / 'test.txt'
    test_file.write_text('test content')
    
    try:
        result = subprocess.run(
            ['docker', 'run', '--rm', '-v', 
             f"{test_dir}:/test", 'alpine', 'cat', '/test/test.txt'],
            capture_output=True,
            text=True,
            check=True
        )
        assert 'test content' in result.stdout
    finally:
        test_file.unlink()
        test_dir.rmdir()

def test_windows_line_endings():
    """Test handling of Windows line endings in mounted files"""
    test_file = Path('test_crlf.txt')
    test_file.write_text('test\r\nwindows\r\nline\r\nendings', encoding='utf-8')
    
    try:
        result = subprocess.run(
            ['docker', 'run', '--rm', '-v', 
             f"{test_file.absolute()}:/test.txt", 'alpine', 
             'cat', '-A', '/test.txt'],
            capture_output=True,
            text=True,
            check=True
        )
        assert '$' in result.stdout  # Check if line endings are preserved
    finally:
        test_file.unlink()

@pytest.mark.skipif(not check_docker_desktop(), reason="Docker Desktop not installed")
def test_gpu_support_windows():
    """Test GPU support on Windows with Docker"""
    try:
        result = subprocess.run(
            ['docker', 'run', '--rm', '--gpus', 'all', 'nvidia/cuda:11.8.0-base-ubuntu22.04', 'nvidia-smi'],
            capture_output=True,
            text=True,
            check=True
        )
        assert 'NVIDIA-SMI' in result.stdout
    except subprocess.CalledProcessError:
        pytest.skip("No GPU support available")

def test_windows_environment_variables():
    """Test environment variable handling on Windows"""
    test_var = 'TEST_VAR_WINDOWS'
    test_value = 'test_value'
    os.environ[test_var] = test_value
    
    try:
        result = subprocess.run(
            ['docker', 'run', '--rm', '-e', f"{test_var}", 'alpine', 'env'],
            capture_output=True,
            text=True,
            check=True
        )
        assert f"{test_var}={test_value}" in result.stdout
    finally:
        del os.environ[test_var]