# Agent Laboratory (Rust Implementation)

A high-performance research paper analysis and ML experimentation framework implemented in Rust.

## Features

- **Research Agents**: Automated paper discovery and analysis
- **Paper Analysis**: Deep analysis of research papers with code and math extraction
- **Safe File Operations**: Atomic file operations with automatic backups
- **Async Support**: Built with async/await for optimal performance
- **Strong Type Safety**: Leveraging Rust's type system for reliability

## Quick Start

```rust
use agent_laboratory::agents::{Agent, AgentConfig};
use agent_laboratory::agents::research::ResearchAgent;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = AgentConfig {
        model: "gpt-4".to_string(),
        temperature: 0.7,
        max_tokens: 1000,
        api_key: std::env::var("OPENAI_API_KEY")?,
    };

    let agent = ResearchAgent::new(config);
    let result = agent.process("quantum computing advances").await?;
    println!("{}", result);
    Ok(())
}
```

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
agent_laboratory = "0.1.0"
```

## Running Tests

```bash
# Run all tests
cargo test

# Run specific test suite
cargo test --test integration

# Run with logging
RUST_LOG=debug cargo test
```

## Project Structure

```
src/
  ├── agents/       # Agent implementations
  │   ├── research.rs
  │   └── paper.rs
  ├── utils/        # Utility functions
  │   ├── error.rs
  │   └── file_ops.rs
  └── tools/        # Analysis tools
```

## License

MIT License - See LICENSE file for details