use agent_laboratory::agents::{Agent, AgentConfig};
use agent_laboratory::agents::research::{ResearchAgent, ResearchStyle};
use agent_laboratory::agents::paper::{PaperAgent, AnalysisDepth};

#[tokio::test]
async fn test_research_workflow() {
    let config = AgentConfig {
        model: "gpt-4".to_string(),
        temperature: 0.7,
        max_tokens: 1000,
        api_key: "test_key".to_string(),
    };

    // Create research agent
    let research_agent = ResearchAgent::new(config.clone())
        .with_style(ResearchStyle::Deep)
        .with_max_papers(5);

    // Process research query
    let result = research_agent.process("quantum computing applications").await;
    assert!(result.is_ok());

    // Create paper agent
    let paper_agent = PaperAgent::new(config)
        .with_depth(AnalysisDepth::Deep)
        .extract_code(true)
        .extract_math(true);

    // Process paper
    let result = paper_agent.process("Sample paper content...").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_agent_configuration() {
    let config = AgentConfig {
        model: "gpt-3.5-turbo".to_string(),
        temperature: 0.5,
        max_tokens: 500,
        api_key: "test_key".to_string(),
    };

    let mut agent = ResearchAgent::new(config.clone());
    assert_eq!(agent.config().model, "gpt-3.5-turbo");
    
    let new_config = AgentConfig {
        model: "gpt-4".to_string(),
        ..config
    };
    
    agent.update_config(new_config);
    assert_eq!(agent.config().model, "gpt-4");
}