# Compass

Compass is a tree-sitter powered preference engine for LLM-assisted coding. Point it at a source file and it evaluates the code against a set of fully configurable rules that encode how you want code to look and behave. Use it during code generation loops to give your model immediate feedback, or wire it into CI so team‑wide style expectations stay consistent.

Because the config format is simple TOML plus Tree-sitter queries, you can even ask an LLM to author new rules for you (“write a rule that flags Go functions with nested loops,” etc.) and drop them straight into the config file.

## Why

Large language models are great at producing code, but nudging them toward your personal conventions takes work. Compass lets you describe those conventions declaratively:

- Write Tree-sitter queries that capture the structures you like or dislike.
- Attach severities, suggestions, and scores so you (or the LLM) know what to fix first.
- Scope rules per language while sharing a single config file.

The default config (`config/config.toml`) includes opinionated rules for Rust, Go, and JavaScript (e.g., no inline logic in `match` arms, warn on unchecked Go errors, catch `console.log`). You can tweak or replace it entirely.

## Installation

### Quick Install (Recommended)

```bash
curl -fsSL https://raw.githubusercontent.com/lyledean1/compass/main/install.sh | bash
```

This will automatically detect your platform and install the latest release.

### Manual Install

<details>
<summary>Platform-specific instructions</summary>

```bash
# macOS ARM64 (M1/M2/M3)
curl -L https://github.com/lyledean1/compass/releases/latest/download/compass-macos-arm64.tar.gz | tar xz
sudo mv compass /usr/local/bin/

# macOS Intel
curl -L https://github.com/lyledean1/compass/releases/latest/download/compass-macos-amd64.tar.gz | tar xz
sudo mv compass /usr/local/bin/

# Linux AMD64
curl -L https://github.com/lyledean1/compass/releases/latest/download/compass-linux-amd64.tar.gz | tar xz
sudo mv compass /usr/local/bin/

# Linux ARM64
curl -L https://github.com/lyledean1/compass/releases/latest/download/compass-linux-arm64.tar.gz | tar xz
sudo mv compass /usr/local/bin/
```

</details>

### Build from Source

```bash
cargo install --git https://github.com/lyledean1/compass
```

## Usage

```bash
# Analyze with the built-in rules
compass path/to/file.rs

# Override with your own config
touch my-style.toml
# ... define [[rules]] ...
compass path/to/file.rs my-style.toml
```

**Supported languages:** Rust, Go, JavaScript, Java, C++, Swift, Zig

`compass` auto-detects file extensions: `.rs`, `.go`, `.js`, `.jsx`, `.java`, `.cpp`, `.cc`, `.cxx`, `.h`, `.hpp`, `.swift`, `.zig`

## Configuration Model

Each rule lives in a TOML `[[rules]]` entry:

```toml
[[rules]]
name = "large_match_prefer_functions"
language = "rust"
query = '''
(match_expression
  body: (match_block
    (match_arm value: (block) @body)
  )
) @match
'''
severity = "info"
message = "Match arm contains inline logic"
suggestion = "Extract helper functions per arm."
weight = 1.5
enabled = true
```

Key fields:

- `language` (optional) – scope to `rust`, `go`, or `javascript`. Omit to apply everywhere.
- `query` – Tree-sitter query run against the file’s syntax tree.
- `severity`, `message`, `suggestion` – what gets emitted when a match is found.
- `weight` – multiplies the severity’s base score impact.
- `enabled` – toggle rules without deleting them.

## Output

Compass prints JSON so tools or LLMs can parse it easily:

```json
{
  "score": 7.3,
  "max_score": 10.0,
  "rating": "Fair",
  "summary": "Code needs improvement in several areas",
  "issues": [
    {
      "rule": "go_missing_error_check",
      "severity": "Warning",
      "line": 42,
      "message": "Potential unchecked error",
      "suggestion": "Follow this assignment with `if err != nil`."
    }
  ]
}
```

Use that feedback loop to steer your LLM: reject generations until the score clears a threshold, or surface the suggestions directly in a conversation.

## Development

```bash
# Build
cargo build --release

# Format & check
cargo fmt
cargo check
```

## License

Compass is MIT licensed. Tree-sitter grammars retain their respective licenses (see `THIRDPARTY.yml`).
