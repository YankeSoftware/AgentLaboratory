"""Prompt management utilities for Agent Laboratory."""

import re
import json
import logging
from pathlib import Path
from typing import Optional, Dict, Any, List

class PromptManager:
    """Manages prompts with validation and formatting."""
    
    def __init__(self, prompt_dir: Optional[str] = None):
        """Initialize prompt manager.
        
        Args:
            prompt_dir: Directory containing prompt templates
        """
        self.logger = logging.getLogger(__name__)
        self.prompt_dir = Path(prompt_dir) if prompt_dir else None
        self._load_templates()

    def _load_templates(self) -> None:
        """Load prompt templates from files."""
        self.templates = {}
        if self.prompt_dir and self.prompt_dir.exists():
            for template_file in self.prompt_dir.glob("*.json"):
                try:
                    with open(template_file) as f:
                        self.templates[template_file.stem] = json.load(f)
                except Exception as e:
                    self.logger.error(f"Failed to load template {template_file}: {str(e)}")

    def format_prompt(self, template_name: str, **kwargs) -> str:
        """Format prompt template with variables.
        
        Args:
            template_name: Name of template to use
            **kwargs: Variables to format template with
            
        Returns:
            Formatted prompt string
        """
        template = self.templates.get(template_name, "")
        if not template:
            self.logger.warning(f"Template {template_name} not found")
            return ""
            
        try:
            return template.format(**kwargs)
        except KeyError as e:
            self.logger.error(f"Missing required variable in template {template_name}: {str(e)}")
            return ""
        except Exception as e:
            self.logger.error(f"Failed to format template {template_name}: {str(e)}")
            return ""

    def clean_search_query(self, query: str) -> str:
        """Clean search query string.
        
        Args:
            query: Search query string
            
        Returns:
            Cleaned query string
        """
        # Remove special characters
        query = re.sub(r'[^\w\s-]', '', query)
        # Normalize whitespace
        query = " ".join(query.split())
        return query

    def extract_code_block(self, text: str, language: str = None) -> Optional[str]:
        """Extract code block from text.
        
        Args:
            text: Text containing code block
            language: Optional programming language identifier
            
        Returns:
            Extracted code block or None
        """
        pattern = r'```(?:{})?\n(.*?)\n```'.format(language if language else r'\w*')
        match = re.search(pattern, text, re.DOTALL)
        return match.group(1) if match else None

    def extract_command(self, text: str, command: str) -> Optional[str]:
        """Extract command block from text.
        
        Args:
            text: Text containing command block
            command: Command identifier
            
        Returns:
            Extracted command content or None
        """
        pattern = r'```{}\n(.*?)\n```'.format(command)
        match = re.search(pattern, text, re.DOTALL)
        return match.group(1) if match else None

    def validate_prompt(self, prompt: str) -> List[str]:
        """Validate prompt for common issues.
        
        Args:
            prompt: Prompt string to validate
            
        Returns:
            List of validation issues found
        """
        issues = []
        
        # Check for missing closing quotes
        if prompt.count('"') % 2 != 0:
            issues.append("Mismatched quotes")
            
        # Check for inconsistent line endings
        if '\r\n' in prompt and '\n' in prompt:
            issues.append("Mixed line endings")
            
        # Check for very long lines
        if any(len(line) > 120 for line in prompt.splitlines()):
            issues.append("Lines exceed recommended length")
            
        # Check for common placeholder patterns
        if re.search(r'<[^>]+>', prompt):
            issues.append("Unreplaced placeholders found")
            
        return issues

    def format_error_message(self, error: str) -> str:
        """Format error message for consistency.
        
        Args:
            error: Raw error message
            
        Returns:
            Formatted error message
        """
        # Remove unnecessary details
        error = re.sub(r'File ".*?", line \d+,', '', error)
        # Normalize whitespace
        error = " ".join(error.split())
        return error.strip()

    def format_feedback(self, feedback: str, max_length: int = 1000) -> str:
        """Format feedback message.
        
        Args:
            feedback: Feedback message
            max_length: Maximum length
            
        Returns:
            Formatted feedback
        """
        if len(feedback) > max_length:
            feedback = feedback[:max_length] + "..."
        return feedback.strip()

    def build_system_prompt(self, role: str, task: str, notes: List[str] = None) -> str:
        """Build system prompt with consistent formatting.
        
        Args:
            role: Agent role
            task: Task description
            notes: Optional list of notes
            
        Returns:
            Formatted system prompt
        """
        prompt_parts = [
            f"You are {role}",
            f"Task instructions: {task}"
        ]
        
        if notes:
            prompt_parts.append("Notes:")
            prompt_parts.extend(f"- {note}" for note in notes)
            
        return "\n\n".join(prompt_parts)

    def validate_response(self, response: str, expected_pattern: str) -> bool:
        """Validate model response format.
        
        Args:
            response: Model response string
            expected_pattern: Expected regex pattern
            
        Returns:
            True if valid, False otherwise
        """
        try:
            return bool(re.match(expected_pattern, response, re.DOTALL))
        except Exception as e:
            self.logger.error(f"Failed to validate response: {str(e)}")
            return False