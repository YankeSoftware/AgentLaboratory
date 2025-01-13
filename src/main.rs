use std::path::PathBuf;
use clap::{Parser, Subcommand};
use agent_laboratory::agents::{Agent, AgentConfig, AgentFactory};
use agent_laboratory::agents::research::ResearchStyle;
use agent_laboratory::agents::paper::AnalysisDepth;
use agent_laboratory::utils::InitManager;

#[derive(Parser)]
#[command(name = "agentlab")]
#[command(about = "Research paper analysis and ML experimentation tool", long_about = None)]
struct Cli {
    /// Optional config file path
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// API key (overrides config file)
    #[arg(short, long, env = "OPENAI_API_KEY")]
    api_key: Option<String>,

    /// Model to use (defaults to gpt-4)
    #[arg(short, long, default_value = "gpt-4")]
    model: String,

    /// Temperature for model responses
    #[arg(short, long, default_value_t = 0.7)]
    temperature: f32,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Research a topic
    Research {
        /// Research query
        query: String,

        /// Research style (broad/deep/hybrid)
        #[arg(short = 's', long, default_value = "hybrid")]
        style: String,

        /// Maximum number of papers to analyze
        #[arg(short = 'p', long, default_value_t = 5)]
        max_papers: usize,

        /// Minimum citation count for papers
        #[arg(short = 'c', long, default_value_t = 10)]
        min_citations: usize,
    },
    /// Analyze a paper
    Analyze {
        /// Path to paper file
        #[arg(short, long)]
        file: PathBuf,

        /// Analysis depth (quick/normal/deep)
        #[arg(short, long, default_value = "normal")]
        depth: String,

        /// Extract code snippets
        #[arg(short, long, default_value_t = true)]
        extract_code: bool,

        /// Extract mathematical formulas
        #[arg(short, long, default_value_t = true)]
        extract_math: bool,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging with timestamps and file info
    tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .init();

    // Initialize directories and verify credentials
    let init = InitManager::new(".");
    init.ensure_directories()?;
    init.cleanup_temp()?;

    if !init.verify_llm_credentials() {
        eprintln!("Error: No valid LLM API keys found. Please set either DEEPSEEK_API_KEY or ANTHROPIC_API_KEY");
        std::process::exit(1);
    }

    // Parse command line
    let cli = Cli::parse();

    // Create base configuration
    let config = AgentConfig {
        model: "deepseek-chat".to_string(),
        temperature: cli.temperature,
        max_tokens: 1000,
        api_key: "".to_string(), // Not used directly anymore
    };

    // Get API key from environment
    let api_key = std::env::var("DEEPSEEK_API_KEY")
        .map_err(|_| "DEEPSEEK_API_KEY environment variable not found")?;
    
    // Create agent factory
    let factory = AgentFactory::new(api_key, config.clone())?;

    match cli.command {
        Commands::Research { 
            query, 
            style, 
            max_papers, 
            min_citations 
        } => {
            let style = match style.to_lowercase().as_str() {
                "broad" => ResearchStyle::Broad,
                "deep" => ResearchStyle::Deep,
                _ => ResearchStyle::Hybrid,
            };

            let mut research_agent = factory.create_research_agent();
            research_agent = research_agent
                .with_style(style)
                .with_max_papers(max_papers)
                .with_min_citations(min_citations);

            let result = research_agent.process(&query).await?;
            println!("Research Results:\n{}", result);
        },
        Commands::Analyze { 
            file, 
            depth, 
            extract_code, 
            extract_math 
        } => {
            let depth = match depth.to_lowercase().as_str() {
                "quick" => AnalysisDepth::Quick,
                "deep" => AnalysisDepth::Deep,
                _ => AnalysisDepth::Normal,
            };

            let mut paper_agent = factory.create_paper_agent();
            paper_agent = paper_agent
                .with_depth(depth)
                .extract_code(extract_code)
                .extract_math(extract_math);

            let content = std::fs::read_to_string(file)?;
            let result = paper_agent.process(&content).await?;
            println!("Paper Analysis:\n{}", result);
        }
    }

    Ok(())
}
