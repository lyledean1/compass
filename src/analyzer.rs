use serde_json::{json, Value};
use tree_sitter::{Language, Parser, Query, QueryCursor, StreamingIterator};

#[derive(Debug, Clone)]
pub struct AnalysisResult {
    pub rule_name: String,
    pub severity: Severity,
    pub message: String,
    pub line: usize,
    pub column: usize,
    pub text: String,
    pub suggestion: Option<String>,
    pub score_impact: f64,
}

#[derive(Debug, Clone)]
pub enum Severity {
    Error,
    Warning,
    Info,
    Style,
}

impl Severity {
    pub fn base_score_impact(&self) -> f64 {
        match self {
            Severity::Error => -3.0,
            Severity::Warning => -1.5,
            Severity::Info => -0.4,
            Severity::Style => -0.2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AnalysisRule {
    pub name: String,
    pub query: String,
    pub severity: Severity,
    pub message_template: String,
    pub suggestion: Option<String>,
    pub weight_multiplier: f64,
}

impl AnalysisRule {
    pub fn new(
        name: String,
        query: String,
        severity: Severity,
        message: String,
        suggestion: Option<String>,
    ) -> Self {
        Self {
            name,
            query,
            severity,
            message_template: message,
            suggestion,
            weight_multiplier: 1.0,
        }
    }

    pub fn with_weight(mut self, weight: f64) -> Self {
        self.weight_multiplier = weight;
        self
    }
}

#[derive(Debug, Clone)]
pub struct CodeScore {
    pub overall_score: f64,
    pub max_score: f64,
    pub total_issues: usize,
    pub breakdown: ScoreBreakdown,
    pub rating: String,
    pub summary: String,
}

#[derive(Debug, Clone)]
pub struct ScoreBreakdown {
    pub errors: usize,
    pub warnings: usize,
    pub info_issues: usize,
    pub style_issues: usize,
    pub error_deduction: f64,
    pub warning_deduction: f64,
    pub info_deduction: f64,
    pub style_deduction: f64,
    pub size_bonus: f64,
}

pub struct CodeAnalyzer {
    rules: Vec<AnalysisRule>,
}

impl CodeAnalyzer {
    pub fn new() -> Self {
        CodeAnalyzer { rules: Vec::new() }
    }

    pub fn add_rule(&mut self, rule: AnalysisRule) {
        self.rules.push(rule);
    }

    pub fn has_rules(&self) -> bool {
        !self.rules.is_empty()
    }

    pub fn analyze(
        &self,
        source_code: &str,
        language: &Language,
    ) -> Result<Vec<AnalysisResult>, Box<dyn std::error::Error>> {
        let mut parser = Parser::new();
        parser.set_language(language)?;

        let tree = parser.parse(source_code, None).unwrap();
        let mut results = Vec::new();

        for rule in &self.rules {
            let query = Query::new(language, &rule.query)?;
            let mut cursor = QueryCursor::new();

            let mut matches = cursor.matches(&query, tree.root_node(), source_code.as_bytes());
            while let Some(match_) = matches.next() {
                for capture in match_.captures {
                    let node = capture.node;
                    let start = node.start_position();
                    let text = node.utf8_text(source_code.as_bytes()).unwrap_or("");

                    let score_impact = rule.severity.base_score_impact() * rule.weight_multiplier;

                    results.push(AnalysisResult {
                        rule_name: rule.name.clone(),
                        severity: rule.severity.clone(),
                        message: rule.message_template.clone(),
                        line: start.row + 1,
                        column: start.column + 1,
                        text: text.to_string(),
                        suggestion: rule.suggestion.clone(),
                        score_impact,
                    });
                }
            }
        }

        Ok(results)
    }

    pub fn analyze_with_score(
        &self,
        source_code: &str,
        language: &Language,
    ) -> Result<(Vec<AnalysisResult>, CodeScore), Box<dyn std::error::Error>> {
        let results = self.analyze(source_code, language)?;
        let score = self.calculate_score(&results, source_code);
        Ok((results, score))
    }

    fn calculate_score(&self, results: &[AnalysisResult], source_code: &str) -> CodeScore {
        let base_score = 10.0;
        let line_count = source_code.lines().count();

        let mut breakdown = ScoreBreakdown {
            errors: 0,
            warnings: 0,
            info_issues: 0,
            style_issues: 0,
            error_deduction: 0.0,
            warning_deduction: 0.0,
            info_deduction: 0.0,
            style_deduction: 0.0,
            size_bonus: 0.0,
        };

        for result in results {
            match result.severity {
                Severity::Error => {
                    breakdown.errors += 1;
                    breakdown.error_deduction += result.score_impact.abs();
                }
                Severity::Warning => {
                    breakdown.warnings += 1;
                    breakdown.warning_deduction += result.score_impact.abs();
                }
                Severity::Info => {
                    breakdown.info_issues += 1;
                    breakdown.info_deduction += result.score_impact.abs();
                }
                Severity::Style => {
                    breakdown.style_issues += 1;
                    breakdown.style_deduction += result.score_impact.abs();
                }
            }
        }

        let total_deduction = breakdown.error_deduction
            + breakdown.warning_deduction
            + breakdown.info_deduction
            + breakdown.style_deduction;

        let size_factor = if line_count > 200 {
            let leniency = ((line_count as f64 - 200.0) / 1000.0).min(0.3);
            breakdown.size_bonus =
                leniency * (breakdown.info_deduction + breakdown.style_deduction);
            1.0 + leniency
        } else if line_count < 50 {
            0.9
        } else {
            1.0
        };

        let adjusted_deduction = total_deduction / size_factor;
        let overall_score = (base_score - adjusted_deduction).max(0.0);
        let rounded_score = (overall_score * 10.0).round() / 10.0;

        let (rating, summary) = self.get_rating_and_summary(rounded_score, &breakdown);

        CodeScore {
            overall_score: rounded_score,
            max_score: base_score,
            total_issues: results.len(),
            breakdown,
            rating,
            summary,
        }
    }

    fn get_rating_and_summary(&self, score: f64, breakdown: &ScoreBreakdown) -> (String, String) {
        let rating = match score {
            9.0..=10.0 => "Excellent",
            7.5..=8.9 => "Good",
            6.0..=7.4 => "Fair",
            4.0..=5.9 => "Poor",
            _ => "Critical",
        }
        .to_string();

        let summary = if breakdown.errors > 0 {
            format!(
                "Code has {} critical errors that need immediate attention",
                breakdown.errors
            )
        } else if breakdown.warnings > 5 {
            "Multiple warnings detected - consider addressing them".to_string()
        } else if breakdown.info_issues > 10 {
            "Many minor issues found - good opportunity for cleanup".to_string()
        } else if score >= 9.0 {
            "Excellent code quality with minimal issues".to_string()
        } else if score >= 7.5 {
            "Good code quality with room for minor improvements".to_string()
        } else {
            "Code needs improvement in several areas".to_string()
        };

        (rating, summary)
    }

    pub fn format_score_as_json(&self, results: &[AnalysisResult], score: &CodeScore) -> Value {
        json!({
            "score": score.overall_score,
            "max_score": score.max_score,
            "rating": score.rating,
            "summary": score.summary,
            "total_issues": score.total_issues,
            "breakdown": {
                "errors": score.breakdown.errors,
                "warnings": score.breakdown.warnings,
                "info_issues": score.breakdown.info_issues,
                "style_issues": score.breakdown.style_issues,
                "deductions": {
                    "from_errors": score.breakdown.error_deduction,
                    "from_warnings": score.breakdown.warning_deduction,
                    "from_info": score.breakdown.info_deduction,
                    "from_style": score.breakdown.style_deduction
                },
                "size_bonus": score.breakdown.size_bonus
            },
            "issues": results.iter().map(|r| json!({
                "rule": r.rule_name,
                "severity": format!("{:?}", r.severity),
                "message": r.message,
                "line": r.line,
                "column": r.column,
                "text": r.text,
                "suggestion": r.suggestion,
                "score_impact": r.score_impact
            })).collect::<Vec<_>>()
        })
    }
}
