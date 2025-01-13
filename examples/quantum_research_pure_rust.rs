use agent_laboratory::{
    ResearchState,
    state::{
        ResearchContext,
        ExperimentContext,
        SystemNote,
        ArtifactStore,
        Experiment,
        ExperimentStatus,
        ArtifactType,
        NoteImportance,
    },
};
use anyhow::Result;
use chrono::{Utc, Duration};
use serde_json::json;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize research state
    let research_root = PathBuf::from(r"C:\Users\YourUser\Documents\QuantumResearch");
    std::fs::create_dir_all(&research_root)?;

    let target_completion = Utc::now() + Duration::days(30);
    let mut state = ResearchState::new(
        "quantum-ml-research-2025".to_string(),
        "Quantum Computing Applications in Machine Learning".to_string(),
        target_completion,
        research_root.clone(),
    )?;

    // Add Windows + RTX 2070 specific task notes
    let task_notes = vec![
        json!({
            "phases": ["setup", "environment preparation"],
            "note": "Running on Windows 10 with RTX 2070. Using pure Rust implementation."
        }),
        json!({
            "phases": ["environment preparation"],
            "note": "Using Rust native GPU acceleration through wgpu-rs."
        }),
        json!({
            "phases": ["setup"],
            "note": "Quantum circuit implementation with RTX 2070 optimizations."
        })
    ];

    state.add_task_notes(task_notes)?;

    // Set up experiment tracking
    let experiment = Experiment {
        id: "qml-bench-001".to_string(),
        title: "Quantum Circuit Optimization".to_string(),
        description: "Quantum circuit simulation with GPU acceleration".to_string(),
        hypothesis: "GPU-accelerated quantum circuit simulation will show significant speedup".to_string(),
        methodology: r#"
        1. Implementation:
           - Pure Rust quantum circuit simulator
           - GPU acceleration via wgpu
           - Parallel CPU execution with rayon
        2. Hardware:
           - GPU: RTX 2070
           - Explicit memory management
           - Batch processing for large circuits
        "#.to_string(),
        variables: [
            ("n_qubits", "16"),
            ("circuit_depth", "10"),
            ("batch_size", "1000"),
            ("gpu_memory_limit_mb", "7000"),
        ].iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
        results: None,
        status: ExperimentStatus::Planned,
        related_papers: Vec::new(),
        notes: Vec::new(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    state.experiments.add_experiment(experiment)?;

    // Store the Rust quantum circuit implementation as an artifact
    let code_artifact = r#"
use std::sync::Arc;
use rayon::prelude::*;
use ndarray::{Array2, ArrayView2};
use num_complex::Complex64;

#[cfg(feature = "gpu")]
use wgpu;

pub struct QuantumCircuit {
    n_qubits: usize,
    state_vector: Vec<Complex64>,
    #[cfg(feature = "gpu")]
    device: wgpu::Device,
    #[cfg(feature = "gpu")]
    queue: wgpu::Queue,
    #[cfg(feature = "gpu")]
    state_buffer: wgpu::Buffer,
}

#[derive(Debug, Clone, Copy)]
pub enum GateType {
    Hadamard,
    PauliX,
    PauliY,
    PauliZ,
    CNOT,
}

impl QuantumCircuit {
    #[cfg(feature = "gpu")]
    pub async fn new(n_qubits: usize) -> Self {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });
        
        // Request high-performance GPU (RTX 2070)
        let adapter = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        }).await.unwrap();

        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("Quantum Circuit Device"),
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            },
            None,
        ).await.unwrap();

        // Initialize quantum state vector
        let state_size = 1 << n_qubits;
        let mut state_vector = vec![Complex64::new(0.0, 0.0); state_size];
        state_vector[0] = Complex64::new(1.0, 0.0);

        // Create GPU buffer for state vector
        let state_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Quantum State Buffer"),
            size: (state_size * std::mem::size_of::<Complex64>()) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: true,
        });

        Self {
            n_qubits,
            state_vector,
            device,
            queue,
            state_buffer,
        }
    }

    #[cfg(not(feature = "gpu"))]
    pub fn new(n_qubits: usize) -> Self {
        let state_size = 1 << n_qubits;
        let mut state_vector = vec![Complex64::new(0.0, 0.0); state_size];
        state_vector[0] = Complex64::new(1.0, 0.0);

        Self {
            n_qubits,
            state_vector,
        }
    }

    #[cfg(not(feature = "gpu"))]
    pub fn apply_gate(&mut self, gate_type: GateType, target: usize) {
        match gate_type {
            GateType::Hadamard => self.apply_hadamard(target),
            GateType::PauliX => self.apply_paulix(target),
            GateType::PauliY => self.apply_pauliy(target),
            GateType::PauliZ => self.apply_pauliz(target),
            GateType::CNOT => (), // TODO: Implement CNOT
        }
    }

    #[cfg(not(feature = "gpu"))]
    fn apply_hadamard(&mut self, target: usize) {
        let n = 1 << self.n_qubits;
        let factor = Complex64::new(1.0 / 2.0_f64.sqrt(), 0.0);
        
        let mut new_state = vec![Complex64::new(0.0, 0.0); n];
        new_state.par_iter_mut().enumerate().for_each(|(i, amp)| {
            let i_bit = (i >> target) & 1;
            let paired_i = i ^ (1 << target);
            *amp = factor * (self.state_vector[i] + 
                if i_bit == 0 { self.state_vector[paired_i] } 
                else { -self.state_vector[paired_i] });
        });
        
        self.state_vector = new_state;
    }

    #[cfg(not(feature = "gpu"))]
    fn apply_paulix(&mut self, target: usize) {
        let n = 1 << self.n_qubits;
        (0..n).into_par_iter()
            .filter(|&i| (i >> target) & 1 == 0)
            .for_each(|i| {
                let paired_i = i ^ (1 << target);
                let temp = self.state_vector[i];
                self.state_vector[i] = self.state_vector[paired_i];
                self.state_vector[paired_i] = temp;
            });
    }

    #[cfg(not(feature = "gpu"))]
    pub fn measure(&self) -> Vec<f64> {
        self.state_vector.par_iter()
            .map(|amp| amp.norm_sqr())
            .collect()
    }
}"#;

    let artifact_id = state.artifacts.store_artifact(
        ArtifactType::Code,
        "quantum_circuit.rs",
        code_artifact.as_bytes(),
    )?;

    // Add a note about the code artifact
    let note = SystemNote {
        id: format!("code-note-{}", Utc::now().timestamp()),
        phases: vec!["implementation".to_string()],
        note: format!("Pure Rust quantum circuit implementation stored in artifact: {}", artifact_id),
        importance: NoteImportance::Info,
        created_at: Utc::now(),
        active: true,
    };
    state.notes.add_system_note(note);

    // Save the research state
    let save_path = research_root.join("research_state.json");
    state.save_to_file(save_path.to_str().unwrap())?;

    println!("Research environment initialized successfully!");
    println!("State saved to: {}", save_path.display());
    
    // Print active notes for current phase
    println!("\nSetup Notes:");
    for note in state.get_active_notes_for_phase("setup") {
        println!("- {}", note);
    }

    Ok(())
}
