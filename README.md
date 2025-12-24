# StatGen

A modern static site generator that converts Markdown to beautiful HTML websites.

## Features

- **Full Markdown support** with GitHub-style HTML rendering
- **Emoji favicon support** - Use any emoji as your website favicon
- **Batch processing** for entire directories
- **Configuration files** (JSON/YAML)
- **Responsive design** with clean, airy layout
- **Theme support** (light/dark/auto)
- **Security** with XSS protection

## Installation

### Using Cargo
```bash
cargo install statgen
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
- `--accent <COLOR>`: Accent color for website (accepts color names or hex codes) - Default: #3498db

#### Font Examples

```bash
# Serif fonts (decorative, easier to read):
statgen -f content.md --font Georgia
statgen -f content.md --font "Times New Roman"

# Sans-serif fonts (clean, modern):
statgen -f content.md --font Arial
statgen -f content.md --font Helvetica

# Monospace fonts (code-like):
statgen -f content.md --font "Courier New"
statgen -f content.md --font monospace
```

**Note**: The requested font must be installed on your system to display properly. If the font is not available, browsers will fall back to system default fonts.

### Configuration File
Create `statgen.json` or `statgen.yaml`:
```json
{
  "font": "Georgia",
  "theme": "dark",
  "accent": "#ff6b35",
  "output": "build",
  "favicon": "🚀"
}
```

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

## License

MIT
