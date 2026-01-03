use compass::config::AnalyzerConfig;
use std::fs;

const RUST_CONFIG: &str = include_str!("../config/rust.toml");
const JAVA_CONFIG: &str = include_str!("../config/java.toml");
const GO_CONFIG: &str = include_str!("../config/go.toml");
const JAVASCRIPT_CONFIG: &str = include_str!("../config/javascript.toml");
const CPP_CONFIG: &str = include_str!("../config/cpp.toml");

#[test]
fn test_rust_analyzer_end_to_end() {
    let config = AnalyzerConfig::from_str(RUST_CONFIG).expect("Failed to parse Rust config");
    let analyzer = config.to_analyzer();

    assert!(analyzer.has_rules(), "Rust analyzer should have rules");

    let source = fs::read_to_string("tests/fixtures/test.rs").expect("Failed to read test.rs");
    let language = tree_sitter_rust::LANGUAGE.into();

    let (results, score) = analyzer
        .analyze_with_score(&source, &language)
        .expect("Analysis failed");

    // Should detect unwrap usage
    let has_unwrap_issue = results.iter().any(|r| r.rule_name.contains("unwrap"));
    assert!(has_unwrap_issue, "Should detect .unwrap() usage");

    // Should detect deep nesting
    let has_nesting_issue = results.iter().any(|r| r.rule_name.contains("nesting"));
    assert!(has_nesting_issue, "Should detect deep nesting");

    // Should have a score less than perfect
    assert!(score.overall_score < 10.0, "Code with issues should have score < 10");

    println!("Rust test: Found {} issues, score: {}/10", results.len(), score.overall_score);
}

#[test]
fn test_java_analyzer_end_to_end() {
    let config = AnalyzerConfig::from_str(JAVA_CONFIG).expect("Failed to parse Java config");
    let analyzer = config.to_analyzer();

    assert!(analyzer.has_rules(), "Java analyzer should have rules");

    let source = fs::read_to_string("tests/fixtures/Test.java").expect("Failed to read Test.java");
    let language = tree_sitter_java::LANGUAGE.into();

    let (results, score) = analyzer
        .analyze_with_score(&source, &language)
        .expect("Analysis failed");

    // Should detect System.out.println
    let has_sysout_issue = results.iter().any(|r| r.rule_name.contains("system_out"));
    assert!(has_sysout_issue, "Should detect System.out.println");

    // Should detect magic numbers
    let has_magic_number = results.iter().any(|r| r.rule_name.contains("magic"));
    assert!(has_magic_number, "Should detect magic numbers");

    // Should detect null return
    let has_null_return = results.iter().any(|r| r.rule_name.contains("null"));
    assert!(has_null_return, "Should detect null return");

    assert!(score.overall_score < 10.0, "Code with issues should have score < 10");

    println!("Java test: Found {} issues, score: {}/10", results.len(), score.overall_score);
}

#[test]
fn test_go_analyzer_end_to_end() {
    let config = AnalyzerConfig::from_str(GO_CONFIG).expect("Failed to parse Go config");
    let analyzer = config.to_analyzer();

    assert!(analyzer.has_rules(), "Go analyzer should have rules");

    let source = fs::read_to_string("tests/fixtures/test.go").expect("Failed to read test.go");
    let language = tree_sitter_go::LANGUAGE.into();

    let (results, score) = analyzer
        .analyze_with_score(&source, &language)
        .expect("Analysis failed");

    // Should detect unchecked error
    let has_error_check = results.iter().any(|r| r.rule_name.contains("error"));
    assert!(has_error_check, "Should detect unchecked error");

    // Should detect panic usage
    let has_panic = results.iter().any(|r| r.rule_name.contains("panic"));
    assert!(has_panic, "Should detect panic() usage");

    assert!(score.overall_score < 10.0, "Code with issues should have score < 10");

    println!("Go test: Found {} issues, score: {}/10", results.len(), score.overall_score);
}

#[test]
fn test_javascript_analyzer_end_to_end() {
    let config = AnalyzerConfig::from_str(JAVASCRIPT_CONFIG).expect("Failed to parse JavaScript config");
    let analyzer = config.to_analyzer();

    assert!(analyzer.has_rules(), "JavaScript analyzer should have rules");

    let source = fs::read_to_string("tests/fixtures/test.js").expect("Failed to read test.js");
    let language = tree_sitter_javascript::LANGUAGE.into();

    let (results, score) = analyzer
        .analyze_with_score(&source, &language)
        .expect("Analysis failed");

    // Should detect var keyword
    let has_var_issue = results.iter().any(|r| r.rule_name.contains("var"));
    assert!(has_var_issue, "Should detect 'var' keyword usage");

    // Should detect console.log
    let has_console_log = results.iter().any(|r| r.rule_name.contains("console"));
    assert!(has_console_log, "Should detect console.log");

    assert!(score.overall_score < 10.0, "Code with issues should have score < 10");

    println!("JavaScript test: Found {} issues, score: {}/10", results.len(), score.overall_score);
}

#[test]
fn test_cpp_analyzer_end_to_end() {
    let config = AnalyzerConfig::from_str(CPP_CONFIG).expect("Failed to parse C++ config");
    let analyzer = config.to_analyzer();

    assert!(analyzer.has_rules(), "C++ analyzer should have rules");

    let source = fs::read_to_string("tests/fixtures/test.cpp").expect("Failed to read test.cpp");
    let language = tree_sitter_cpp::LANGUAGE.into();

    let (results, score) = analyzer
        .analyze_with_score(&source, &language)
        .expect("Analysis failed");

    // Should detect raw new
    let has_new_issue = results.iter().any(|r| r.rule_name.contains("smart_pointer"));
    assert!(has_new_issue, "Should detect raw 'new' operator usage");

    // Should detect manual delete
    let has_delete_issue = results.iter().any(|r| r.rule_name.contains("delete"));
    assert!(has_delete_issue, "Should detect manual 'delete' operator usage");

    // Should detect cout/cerr
    let has_cout_issue = results.iter().any(|r| r.rule_name.contains("cout_cerr"));
    assert!(has_cout_issue, "Should detect std::cout or std::cerr usage");

    // Should detect c-style cast
    let has_cast_issue = results.iter().any(|r| r.rule_name.contains("cast"));
    assert!(has_cast_issue, "Should detect C-style cast");

    // Should detect throw
    let has_throw_issue = results.iter().any(|r| r.rule_name.contains("throw"));
    assert!(has_throw_issue, "Should detect throw statement");

    assert!(score.overall_score < 10.0, "Code with issues should have score < 10");

    println!("C++ test: Found {} issues, score: {}/10", results.len(), score.overall_score);
}

#[test]
fn test_all_configs_parse() {
    // Ensure all embedded configs are valid TOML
    AnalyzerConfig::from_str(RUST_CONFIG).expect("Rust config should parse");
    AnalyzerConfig::from_str(JAVA_CONFIG).expect("Java config should parse");
    AnalyzerConfig::from_str(GO_CONFIG).expect("Go config should parse");
    AnalyzerConfig::from_str(JAVASCRIPT_CONFIG).expect("JavaScript config should parse");
    AnalyzerConfig::from_str(CPP_CONFIG).expect("C++ config should parse");
}

#[test]
fn test_all_analyzers_have_rules() {
    // Ensure each language has at least one enabled rule
    let rust_analyzer = AnalyzerConfig::from_str(RUST_CONFIG).unwrap().to_analyzer();
    assert!(rust_analyzer.has_rules(), "Rust analyzer must have rules");

    let java_analyzer = AnalyzerConfig::from_str(JAVA_CONFIG).unwrap().to_analyzer();
    assert!(java_analyzer.has_rules(), "Java analyzer must have rules");

    let go_analyzer = AnalyzerConfig::from_str(GO_CONFIG).unwrap().to_analyzer();
    assert!(go_analyzer.has_rules(), "Go analyzer must have rules");

    let js_analyzer = AnalyzerConfig::from_str(JAVASCRIPT_CONFIG).unwrap().to_analyzer();
    assert!(js_analyzer.has_rules(), "JavaScript analyzer must have rules");

    let cpp_analyzer = AnalyzerConfig::from_str(CPP_CONFIG).unwrap().to_analyzer();
    assert!(cpp_analyzer.has_rules(), "C++ analyzer must have rules");
}
