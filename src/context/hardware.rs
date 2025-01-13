use serde::{Deserialize, Serialize};
use std::fmt;
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardwareContext {
    pub gpu_info: Option<GpuInfo>,
    pub cpu_info: CpuInfo,
    pub memory_info: MemoryInfo,
    pub os_info: OsInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuInfo {
    pub name: String,
    pub vram_mb: usize,
    pub cuda_available: bool,
    pub cuda_version: Option<String>,
    pub target_device: String,  // "cuda:0", "cpu", etc.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuInfo {
    pub cores: usize,
    pub threads: usize,
    pub architecture: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryInfo {
    pub total_mb: usize,
    pub available_mb: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OsInfo {
    pub os_type: OsType,
    pub version: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum OsType {
    Windows,
    Linux,
    MacOS,
}

impl HardwareContext {
    pub fn detect() -> Result<Self> {
        let gpu_info = detect_gpu()?;
        let cpu_info = detect_cpu()?;
        let memory_info = detect_memory()?;
        let os_info = detect_os()?;

        Ok(Self {
            gpu_info,
            cpu_info,
            memory_info,
            os_info,
        })
    }

    pub fn get_optimal_device(&self) -> String {
        if let Some(gpu) = &self.gpu_info {
            if gpu.cuda_available {
                return "cuda:0".to_string();
            }
        }
        "cpu".to_string()
    }

    pub fn get_batch_size_recommendation(&self) -> usize {
        // RTX 2070 has 8GB VRAM, lets be conservative
        if let Some(gpu) = &self.gpu_info {
            if gpu.cuda_available && gpu.vram_mb >= 8000 {
                32  // Good batch size for RTX 2070
            } else {
                16  // Smaller batch size for limited VRAM
            }
        } else {
            8   // CPU batch size
        }
    }
}

fn detect_gpu() -> Result<Option<GpuInfo>> {
    // For Windows + CUDA
    if cfg!(target_os = "windows") {
        // Check for NVIDIA GPU using nvml or system info
        #[cfg(target_os = "windows")]
        {
            use windows::Win32::System::SystemInformation;
            // This is a simplified check - in real code we\d
