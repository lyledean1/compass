use std::env;
use std::fs;
use std::path::Path;
use std::process;

use crate::config::AnalyzerConfig;
use serde_json::to_string_pretty;
use tree_sitter::Language;

const RUST_CONFIG: &str = include_str!("../config/languages/rust.toml");
const GO_CONFIG: &str = include_str!("../config/languages/go.toml");
const JAVASCRIPT_CONFIG: &str = include_str!("../config/languages/javascript.toml");
const JAVA_CONFIG: &str = include_str!("../config/languages/java.toml");
const ZIG_CONFIG: &str = include_str!("../config/languages/zig.toml");

pub fn run() {
    let mut args = env::args();
    let program = args.next().unwrap_or_else(|| "compass".to_string());
    let remaining: Vec<String> = args.collect();

    if remaining.is_empty() || remaining.len() > 2 {
        usage(&program);
    }

    let source_path = remaining[0].clone();
    let config_override = remaining.get(1).cloned();

    if !Path::new(&source_path).exists() {
        eprintln!("Error: file '{}' does not exist", source_path);
        process::exit(1);
    }

    let language = SupportedLanguage::from_path(&source_path).unwrap_or_else(|| {
        eprintln!(
            "Error: unsupported file extension for '{}'. Supported extensions: .rs, .go, .js, .jsx, .zig, .java",
            source_path
        );
        process::exit(1);
    });

    let config_label;
    let config = match config_override.as_deref() {
        Some(path) => {
            config_label = path.to_string();
            AnalyzerConfig::from_file(path).unwrap_or_else(|e| {
                eprintln!("Error: failed to load config '{}': {}", path, e);
                process::exit(1);
            })
        }
        None => {
            config_label = format!("built-in {}", language.config_key());
            let default_config = language.default_config();
            AnalyzerConfig::from_str(default_config).expect("embedded config should parse")
        }
    };

    let analyzer = config.to_analyzer();
    if !analyzer.has_rules() {
        eprintln!(
            "Error: config '{}' contains no enabled rules for language '{}'",
            config_label,
            language.config_key()
        );
        process::exit(1);
    }

    let source_code = fs::read_to_string(&source_path).unwrap_or_else(|e| {
        eprintln!("Error: failed to read '{}': {}", source_path, e);
        process::exit(1);
    });

    println!(
        "Analyzing {} file with custom preferences: {}",
        language.display_name(),
        source_path
    );
    println!("Config: {}", config_label);
    println!("----------------------------------------");

    let tree_sitter_language = language.tree_sitter_language();
    let (results, score) = analyzer
        .analyze_with_score(&source_code, &tree_sitter_language)
        .unwrap_or_else(|e| {
            eprintln!("Error: analysis failed: {}", e);
            process::exit(1);
        });

    let output = analyzer.format_score_as_json(&results, &score);
    match to_string_pretty(&output) {
        Ok(json) => println!("{}", json),
        Err(e) => {
            eprintln!("Error: failed to format analysis result: {}", e);
            process::exit(1);
        }
    }
}

fn usage(program: &str) -> ! {
    eprintln!("Usage: {} <source-file> [config-file]", program);
    eprintln!("Example: {} src/main.rs", program);
    eprintln!("         {} src/main.rs my-preferences.toml", program);
    eprintln!("\nSupported extensions: .rs, .go, .js, .jsx, .zig, .java");
    process::exit(1);
}

#[derive(Clone, Copy)]
enum SupportedLanguage {
    Rust,
    Go,
    JavaScript,
    Zig,
    Java,
}

impl SupportedLanguage {
    fn from_path(file_path: &str) -> Option<Self> {
        let extension = Path::new(file_path)
            .extension()
            .and_then(|ext| ext.to_str())?
            .to_ascii_lowercase();

        match extension.as_str() {
            "rs" => Some(SupportedLanguage::Rust),
            "go" => Some(SupportedLanguage::Go),
            "js" | "jsx" => Some(SupportedLanguage::JavaScript),
            "zig" => Some(SupportedLanguage::Zig),
            "java" => Some(SupportedLanguage::Java),
            _ => None,
        }
    }

    fn tree_sitter_language(&self) -> Language {
        match self {
            SupportedLanguage::Rust => tree_sitter_rust::LANGUAGE.into(),
            SupportedLanguage::Go => tree_sitter_go::LANGUAGE.into(),
            SupportedLanguage::JavaScript => tree_sitter_javascript::LANGUAGE.into(),
            SupportedLanguage::Zig => tree_sitter_zig::LANGUAGE.into(),
            SupportedLanguage::Java => tree_sitter_java::LANGUAGE.into(),
        }
    }

    fn config_key(&self) -> &'static str {
        match self {
            SupportedLanguage::Rust => "rust",
            SupportedLanguage::Go => "go",
            SupportedLanguage::JavaScript => "javascript",
            SupportedLanguage::Zig => "zig",
            SupportedLanguage::Java => "java",
        }
    }

    fn display_name(&self) -> &'static str {
        match self {
            SupportedLanguage::Rust => "Rust",
            SupportedLanguage::Go => "Go",
            SupportedLanguage::JavaScript => "JavaScript",
            SupportedLanguage::Zig => "Zig",
            SupportedLanguage::Java => "Java",
        }
    }

    fn default_config(&self) -> &'static str {
        match self {
            SupportedLanguage::Rust => RUST_CONFIG,
            SupportedLanguage::Go => GO_CONFIG,
            SupportedLanguage::JavaScript => JAVASCRIPT_CONFIG,
            SupportedLanguage::Zig => ZIG_CONFIG,
            SupportedLanguage::Java => JAVA_CONFIG,
        }
    }
}
