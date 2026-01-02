use crate::analyzer::{AnalysisRule, CodeAnalyzer, Severity};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct RuleConfig {
    pub name: String,
    pub query: String,
    pub severity: String,
    pub message: String,
    pub suggestion: Option<String>,
    #[serde(default = "default_weight")]
    pub weight: f64,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub language: Option<String>,
}

fn default_weight() -> f64 {
    1.0
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AnalyzerConfig {
    #[serde(default)]
    pub rules: Vec<RuleConfig>,
}

impl AnalyzerConfig {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: AnalyzerConfig = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn from_str(content: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config: AnalyzerConfig = toml::from_str(content)?;
        Ok(config)
    }

    pub fn to_analyzer_for_language(&self, language: &str) -> CodeAnalyzer {
        let mut analyzer = CodeAnalyzer::new();
        let target_language = language.to_lowercase();

        for rule_config in &self.rules {
            if !rule_config.enabled {
                continue;
            }

            if let Some(rule_language) = &rule_config.language {
                if rule_language.to_lowercase() != target_language {
                    continue;
                }
            }

            let severity = match rule_config.severity.to_lowercase().as_str() {
                "error" => Severity::Error,
                "warning" => Severity::Warning,
                "info" => Severity::Info,
                "style" => Severity::Style,
                _ => Severity::Info,
            };

            let rule = AnalysisRule::new(
                rule_config.name.clone(),
                rule_config.query.clone(),
                severity,
                rule_config.message.clone(),
                rule_config.suggestion.clone(),
            )
            .with_weight(rule_config.weight);

            analyzer.add_rule(rule);
        }

        analyzer
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_parsing() {
        let toml_str = r#"
[[rules]]
name = "test_rule"
query = "(ERROR) @error"
severity = "error"
message = "Test error"
enabled = true
weight = 2.0
        "#;

        let config: AnalyzerConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.rules.len(), 1);
        assert_eq!(config.rules[0].name, "test_rule");
        assert_eq!(config.rules[0].weight, 2.0);
        assert!(config.rules[0].language.is_none());
    }
}
