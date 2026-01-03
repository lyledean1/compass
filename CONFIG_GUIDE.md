# Configuration Guide

Treescan uses language-specific configuration files to define coding preferences.

## Default Configurations

Built-in configurations are located in `config/`:

```
config/
├── rust.toml
├── go.toml
├── java.toml
├── javascript.toml
└── zig.toml
```

Each file contains rules for that specific language. The language is automatically detected from the file extension.

## Usage

### Use built-in defaults:
```bash
compass src/main.rs                    # Uses config/rust.toml
compass Example.java                    # Uses config/java.toml
compass app.js                          # Uses config/javascript.toml
```

### Override with custom preferences:
```bash
compass src/main.rs my-rust-prefs.toml
compass Example.java my-java-prefs.toml
```

## Creating Custom Configs

Create a `.toml` file with your preferred rules:

```toml
# my-java-prefs.toml

[[rules]]
name = "system_out_println"
query = """
(method_invocation
  object: (field_access
    object: (identifier) @sys (#eq? @sys "System")
    field: (identifier) @out (#eq? @out "out")
  )
  name: (identifier) @method (#eq? @method "println")
) @call
"""
severity = "warning"
message = "System.out.println found"
suggestion = "I prefer using a proper logging framework (SLF4J, Logback)"
enabled = true
weight = 2.0
```

### Rule Fields:

- **name**: Unique identifier for the rule
- **query**: Tree-sitter query pattern
- **severity**: `error`, `warning`, `info`, or `style`
- **message**: Brief description of the issue
- **suggestion**: Your preferred solution (in your voice!)
- **enabled**: `true` or `false`
- **weight**: Impact multiplier (default: 1.0)

## Customizing Per Language

You can create different configs for different languages:

```bash
~/.config/compass/
├── rust.toml          # Your Rust preferences
├── java.toml          # Your Java preferences
└── javascript.toml    # Your JavaScript preferences
```

Then use them:
```bash
compass src/main.rs ~/.config/compass/rust.toml
compass Example.java ~/.config/compass/java.toml
```

## Benefits

✅ **Language-specific**: Each language has its own file
✅ **Clean configs**: No `language` field needed anymore
✅ **Easy override**: Users can provide their own configs
✅ **LLM-ready**: Suggestions written in personal voice

## Example Workflow

1. **Start with defaults**: `compass MyCode.java`
2. **See what rules triggered**: Review the JSON output
3. **Customize**: Copy `config/languages/java.toml` to `~/.config/compass/java.toml`
4. **Adjust weights/messages**: Edit to match your preferences
5. **Use custom config**: `compass MyCode.java ~/.config/compass/java.toml`
6. **Share with team**: Check your custom config into version control
