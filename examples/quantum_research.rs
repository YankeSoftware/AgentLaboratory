use agent_laboratory::{
    ResearchState,
    ResearchContext,
    ExperimentContext,
    SystemNote,
    ArtifactStore,
};
use anyhow::Result;
use chrono::{DateTime, Utc, Duration};
use serde_json::json;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize research state
    let research_root = PathBuf::from(r"C:\Users\YourUser\Documents\QuantumResearch");
    std::fs::create_dir_all(&research_root)?;

    let target_completion = Utc::now() + Duration::days(30);
    let mut state = ResearchState::new(
        "quantum-ml-research-2025",
        "Quantum Computing Applications in Machine Learning",
        target_completion,
        research_root.clone(),
    )?;

    // Add Windows + RTX 2070 specific task notes
    let task_notes = vec![
        json!({
            "phases": ["setup", "environment preparation"],
            "note": "Running on Windows 10 with NVIDIA RTX 2070. CUDA 12.1 is available for GPU acceleration."
        }),
        json!({
            "phases": ["environment preparation"],
            "note": "Using PyTorch with CUDA support. Verify with: torch.cuda.is_available() and torch.cuda.get_device_name(0)"
        }),
        json!({
            "phases": ["data preparation", "running experiments"],
            "note": "With RTX 2070 8GB VRAM, use batch sizes of 32 for most experiments. Monitor VRAM usage with nvidia-smi."
        }),
        json!({
            "phases": ["running experiments"],
            "note": "For quantum circuit simulations, utilize cuQuantum for GPU acceleration when available."
        }),
        json!({
            "phases": ["setup"],
            "note": r#"Required setup:
                1. Install CUDA 12.1 from NVIDIA website
                2. Install Python 3.11 from python.org
                3. Install PyTorch with CUDA: pip3 install torch torchvision torchaudio --index-url https://download.pytorch.org/whl/cu121
                4. Install additional requirements: pip install qiskit qiskit-aer-gpu pennylane cupy-cuda12x
                5. Verify GPU: python -c "import torch; print(torch.cuda.is_available())"
            "#
        })
    ];

    state.add_task_notes(task_notes)?;

    // Set up experiment tracking
    let experiment = Experiment {
        id: "qml-bench-001".to_string(),
        title: "Quantum-Classical Model Comparison".to_string(),
        description: "Comparing classical CNN vs Quantum CNN for image classification".to_string(),
        hypothesis: "Quantum CNN will show advantage for specific feature detection tasks".to_string(),
        methodology: r#"
        1. Dataset: MNIST subset (10000 images)
        2. Models:
           - Classical: CNN with 3 conv layers
           - Quantum: 2 conv layers + 1 quantum circuit layer
        3. Hardware:
           - GPU: RTX 2070 (CUDA 12.1)
           - Batch size: 32
           - Memory allocation: 6GB max for training
        "#.to_string(),
        variables: [
            ("learning_rate", "0.001"),
            ("batch_size", "32"),
            ("n_qubits", "4"),
            ("n_layers", "3"),
        ].iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
        results: None,
        status: ExperimentStatus::Planned,
        related_papers: Vec::new(),
        notes: Vec::new(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    state.experiments.add_experiment(experiment)?;

    // Set up artifact storage for experiment results
    let code_artifact = r#"
    import torch
    import pennylane as qml
    from torch import nn
    import torch.nn.functional as F

    # Ensure CUDA is available
    device = torch.device("cuda:0" if torch.cuda.is_available() else "cpu")
    print(f"Using device: {device}")
    if device.type == "cuda":
        print(f"GPU: {torch.cuda.get_device_name(0)}")
        print(f"Memory: {torch.cuda.get_device_properties(0).total_memory / 1024**2:.0f}MB")

    class QuantumCircuit(nn.Module):
        def __init__(self, n_qubits=4, n_layers=3):
            super().__init__()
            self.n_qubits = n_qubits
            self.n_layers = n_layers
            self.dev = qml.device("default.qubit", wires=n_qubits)
            self.qlayer = qml.QNode(self.circuit, self.dev)
            self.weights = nn.Parameter(torch.randn(n_layers, n_qubits, 3))

        def circuit(self, inputs, weights):
            # Encode classical data into quantum state
            for i in range(self.n_qubits):
                qml.RY(inputs[i], wires=i)
            
            # Variational quantum circuit
            for l in range(self.n_layers):
                for i in range(self.n_qubits):
                    qml.Rot(*weights[l, i], wires=i)
                for i in range(self.n_qubits - 1):
                    qml.CNOT(wires=[i, i + 1])
            
            return [qml.expval(qml.PauliZ(i)) for i in range(self.n_qubits)]

        def forward(self, x):
            batch_size = x.shape[0]
            x = x.view(batch_size, -1)
            
            # Process each sample in the batch
            q_out = torch.empty(batch_size, self.n_qubits, device=device)
            for i in range(batch_size):
                q_out[i] = torch.tensor(self.qlayer(x[i], self.weights), device=device)
            
            return q_out
    "#;

    let artifact_id = state.artifacts.store_artifact(
        ArtifactType::Code,
        "quantum_cnn.py",
        code_artifact.as_bytes(),
    )?;

    // Add a note about the code artifact
    let note = SystemNote {
        id: format!("code-note-{}", Utc::now().timestamp()),
        phases: vec!["implementation".to_string()],
        note: format!("Quantum CNN implementation stored in artifact: {}", artifact_id),
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