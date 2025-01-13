use agent_laboratory::agents::{Agent, AgentConfig};
use agent_laboratory::agents::research::{ResearchAgent, ResearchStyle};
use agent_laboratory::agents::paper::{PaperAgent, AnalysisDepth};
use agent_laboratory::utils::AgentResult;

#[tokio::test]
async fn test_research_workflow() -> AgentResult<()> {
    let config = AgentConfig {
        model: "gpt-4".to_string(),
        temperature: 0.7,
        max_tokens: 1000,
        api_key: "test_key".to_string(),
    };

    // Initialize research agent
    let research_agent = ResearchAgent::new(config.clone())
        .with_style(ResearchStyle::Deep)
        .with_max_papers(5);

    // Process research query
    let result = research_agent.process("quantum computing applications").await?;
    assert!(!result.is_empty());

    Ok(())
}

#[tokio::test]
async fn test_paper_analysis() -> AgentResult<()> {
    let config = AgentConfig {
        model: "gpt-4".to_string(),
        temperature: 0.7,
        max_tokens: 1000,
        api_key: "test_key".to_string(),
    };

    // Initialize paper agent
    let paper_agent = PaperAgent::new(config)
        .with_depth(AnalysisDepth::Deep)
        .extract_code(true)
        .extract_math(true);

    // Process paper content
    let result = paper_agent.process("Sample paper content for testing...").await?;
    assert!(!result.is_empty());

    Ok(())
}