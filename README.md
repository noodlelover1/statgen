<div align='center'>

<img src=zap.png alt="logo" width=100 height=100 />
<br>


<h1>StatGen</h1>
<p>Super fast CLI website generator</p>
<a href="./LICENSE"><img src="https://img.shields.io/badge/License-CC_BY--NC--ND-blue?style=for-the-badge"/></a>
<a href="https://noodlelover1.github.io/statgen/"><img alt="Website" src="https://img.shields.io/website?url=https%3A%2F%2Fnoodlelover1.github.io%2Fstatgen&style=for-the-badge"></a>


</div>

## Features

- **Full Markdown support** - Tables, text format ...
- **Emoji favicon support** - Use any emoji as your website favicon
- **Batch processing** for entire directories
- **Basic HTML** : Got a piece of HTML in your Markdown ? No problem !
- **Responsive design** with clean, airy layout
- **Theme support** (light/dark/auto)
- **Security** with XSS protection

## Installation

Download startgen.zip from [the latest release](https://github.com/noodlelover1/statgen/releases) and extract it, then run :
```bash
cd /path/to/exctracted/statgen
cargo install --path .
```

## Usage

### Basic Examples
```bash
# Generate from inline markdown
statgen -i "# Hello World"
statgen -i $'# Header\n\nParagraph text'  # bash with newlines
statgen -i "# Header`n`nParagraph text"   # PowerShell with newlines

# Generate from file
statgen -f README.md

# Process entire directory
statgen -d docs -o site

# With emoji favicon
statgen -f content.md --favicon "🚀" --theme dark
statgen -i "# Hello 🌟" --favicon "🌟"

# Custom styling (use quotes for fonts with spaces)
statgen -f content.md --theme dark --font "Georgia"
statgen -f content.md --theme dark --font "Times New Roman"

# Custom accent color
statgen -f content.md --accent "#ff6b35"
statgen -f content.md --theme dark --accent purple

# Custom accent color for light and dark themes in auto theme
statgen -f content.md --theme auto --accent-light purple --accent-dark yellow
```

### Options
- `-f, --file <FILE>`: Markdown file path
- `-d, --directory <DIR>`: Process all .md files in directory
- `-i, --inline <TEXT>`: Inline markdown content
- `-o, --output <DIR>`: Output directory (default: dist)
- `-f, --favicon <EMOJI>`: Emoji to use as favicon (e.g., 🚀, 📚, 🌟)
- `--font <FONT>`: Font family (use quotes for fonts with spaces) - Default: system fonts
- `--font-size <SIZE>`: Font size (default: 16px)
- `--theme <THEME>`: Theme (light/dark/auto, default: auto)
- `--accent <COLOR>`: Accent color for website (accepts color names or hex codes) - Default: #3498d
- `--accent-light / --accent-dark <COLOR>`: Choose different accent color for two themes when in "auto" mode

#### Font Examples
Fonts must be installed on your system to display. The default uses your system's font stack.

```bash
# Serif fonts (decorative, easier to read):
statgen -f content.md --font Georgia
statgen -f content.md --font "Times New Roman"

# Sans-serif fonts (clean, modern):
statgen -f content.md --font "Noto Sans"
statgen -f content.md --font "DejaVu Sans"

# Monospace fonts (code-like):
statgen -f content.md --font "Liberation Mono"
```

**Note**: If a font doesn't display, it's not installed on your system. Browser falls back to system fonts (usually Cantarell/DejaVu on Arch). Try Noto Sans, DejaVu Sans, or Liberation Sans.

## Output

Generates responsive HTML with:
- Modern CSS with generous spacing
- Full-width layout optimized for readability
- Syntax highlighting for code blocks
- Automatic theme switching
- XSS-safe HTML rendering

## Development

```bash
cargo build    # Build
cargo test     # Run tests
cargo fmt      # Format code
cargo clippy   # Lint
```
