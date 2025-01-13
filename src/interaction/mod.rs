use serde::{Deserialize, Serialize};
use anyhow::Result;
use crate::state::{ResearchState, notes::{SystemNote, UserNote}};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub command_type: CommandType,
    pub args: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommandType {
    // Research commands
    AddPaper,
    AnalyzePaper,
    SearchPapers,
    
    // Experiment commands
    CreateExperiment,
    RunExperiment,
    RecordResults,
    
    // Note commands
    AddSystemNote,
    AddUserNote,
    UpdateNote,
    
    // Progress commands
    AddTask,
    CompleteTask,
    AddMilestone,
    UpdateProgress,
    
    // State commands
    SaveState,
    LoadState,
    ExportResults,
}

pub struct InteractionManager {
    research_state: Arc<Mutex<ResearchState>>,
}

impl InteractionManager {
    pub fn new(state: ResearchState) -> Self {
        Self {
            research_state: Arc::new(Mutex::new(state)),
        }
    }

    pub async fn execute_command(&self, command: Command) -> Result<String> {
        let mut state = self.research_state.lock().await;
        
        match command.command_type {
            CommandType::AddPaper => {
                // Parse paper details from args and add to state
                let paper = serde_json::from_value(command.args)?;
                state.research.add_paper(paper)?;
                Ok("Paper added successfully".to_string())
            },
            
            CommandType::AddSystemNote => {
                let note: SystemNote = serde_json::from_value(command.args)?;
                state.notes.add_system_note(note);
                Ok("System note added".to_string())
            },
            
            CommandType::AddUserNote => {
                let note: UserNote = serde_json::from_value(command.args)?;
                state.notes.add_user_note(note);
                Ok("User note added".to_string())
            },
            
            CommandType::SaveState => {
                let path = command.args.get("path")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow::anyhow!("Save path not provided"))?;
                    
                state.save_to_file(path)?;
                Ok("State saved successfully".to_string())
            },
            
            // Implement other command handlers...
            _ => Err(anyhow::anyhow!("Command not implemented yet")),
        }
    }

    pub async fn get_notes_for_current_phase(&self) -> Result<Vec<String>> {
        let state = self.research_state.lock().await;
        let current_phase = state.progress.get_active_phase()
            .ok_or_else(|| anyhow::anyhow!("No active phase"))?;
            
        Ok(state.notes.get_notes_for_phase(&current_phase.name)
            .into_iter()
            .map(|s| s.to_string())
            .collect())
    }
}
