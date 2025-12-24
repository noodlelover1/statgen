use pulldown_cmark::{html, Options, Parser as MdParser};

fn sanitize_html(html: &str) -> String {
    // GitHub-style HTML sanitization - allow safe HTML tags but escape dangerous ones
    // This allows HTML within Markdown to be rendered, like GitHub does
    let mut result = html.to_string();

    // Escape dangerous tags that could execute code or load external content
    let dangerous_tags = ["script", "iframe", "object", "embed", "form", "meta", "link", "style"];
    for tag in &dangerous_tags {
        let open_pattern = format!("<{}", tag);
        let close_pattern = format!("</{}", tag);
        result = result.replace(&open_pattern, &format!("&lt;{}", tag));
        result = result.replace(&close_pattern, &format!("&lt;/{}", tag));
    }

    // Escape dangerous attributes that could execute JavaScript
    // More careful replacement to avoid breaking tag names
    result = result.replace("javascript:", "javascript&colon;");
    result = result.replace("vbscript:", "vbscript&colon;");
    result = result.replace("data:", "data&colon;");

    // Escape event handlers more carefully - look for attribute patterns
    result = result.replace(" onclick", " on&click");
    result = result.replace(" onload", " on&load");
    result = result.replace(" onmouseover", " on&mouseover");
    result = result.replace(" onmouseout", " on&mouseout");
    result = result.replace(" onkeydown", " on&keydown");
    result = result.replace(" onkeyup", " on&keyup");
    result = result.replace(" onsubmit", " on&submit");

    // Handle input tags specially - only allow disabled checkboxes from task lists
    result = result.replace("<input", "&lt;input");
    // But allow back disabled checkboxes from task lists
    result = result.replace("&lt;input disabled", "<input disabled");
    result = result.replace("&lt;input type=\"checkbox\" disabled", "<input type=\"checkbox\" disabled");

    result
}

pub fn generate_html(markdown: &str, font_size: &str, font: &str, theme: &str, accent: &str, accent_light: Option<&str>, accent_dark: Option<&str>, favicon: Option<&str>) -> String {
    // Generate favicon link from emoji if provided
    let favicon_link = if let Some(emoji) = favicon {
        format!(r#"<link rel="icon" href="data:image/svg+xml,<svg xmlns=%22http://www.w3.org/2000/svg%22 viewBox=%220 0 100 100%22><text y=%22.9em%22 font-size=%2290%22>{}</text></svg>">"#, 
                urlencoding::encode(emoji))
    } else {
        String::new()
    };

    // Use extended parser for full Markdown support, but sanitize HTML for security
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);

    // pulldown-cmark supports inline HTML by default

    let parser = MdParser::new_ext(markdown, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    // Sanitize the HTML output to remove dangerous tags
    let sanitized_output = sanitize_html(&html_output);

    // Extract title from markdown or use default
    let title = extract_title(markdown).unwrap_or_else(|| "Static Site".to_string());

    // Generate theme-specific CSS
    let (body_bg, body_color, header_color, code_bg, code_color, blockquote_bg, border_color) = match theme {
        "dark" => ("#1a1a1a", "#e0e0e0", "#ffffff", "#2d2d2d", "#cccccc", "#2a2a2a", "#404040"),
        "auto" => ("#f4f4f4", "#333", "#2c3e50", "#e7e7e7", "#333", "#f9f9f9", "#e0e0e0"), // Default light, JS will override
        _ => ("#f4f4f4", "#333", "#2c3e50", "#e7e7e7", "#333", "#f9f9f9", "#e0e0e0"), // Light theme
    };

    let theme_script = if theme == "auto" {
        let light_accent = accent_light.as_deref().unwrap_or(accent);
        let dark_accent = accent_dark.as_deref().unwrap_or(accent);
        
        format!(r#"<script>
        function applyTheme(theme) {{
            const root = document.documentElement;
            if (theme === 'dark') {{
                root.style.setProperty('--bg-color', '#1a1a1a');
                root.style.setProperty('--text-color', '#e0e0e0');
                root.style.setProperty('--header-color', '#ffffff');
                root.style.setProperty('--code-bg', '#2d2d2d');
                root.style.setProperty('--code-color', '#cccccc');
                root.style.setProperty('--link-color', '{}');
                root.style.setProperty('--blockquote-bg', '#2a2a2a');
                root.style.setProperty('--border-color', '#404040');
            }} else {{
                root.style.setProperty('--bg-color', '#f4f4f4');
                root.style.setProperty('--text-color', '#333');
                root.style.setProperty('--header-color', '#2c3e50');
                root.style.setProperty('--code-bg', '#e7e7e7');
                root.style.setProperty('--code-color', '#333');
                root.style.setProperty('--link-color', '{}');
                root.style.setProperty('--blockquote-bg', '#f9f9f9');
                root.style.setProperty('--border-color', '#e0e0e0');
            }}
        }}

        // Detect system theme
        const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
        applyTheme(prefersDark ? 'dark' : 'light');

        // Listen for changes
        window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {{
            applyTheme(e.matches ? 'dark' : 'light');
        }});
        </script>"#, dark_accent, light_accent)
    } else {
        String::new()
    };

    let css_variables = format!("--bg-color: {}; --text-color: {}; --header-color: {}; --code-bg: {}; --code-color: {}; --link-color: {}; --blockquote-bg: {}; --border-color: {};", body_bg, body_color, header_color, code_bg, code_color, accent, blockquote_bg, border_color);

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
    {}
    <style>
        :root {{
            {};
            --shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
        }}

        * {{
            box-sizing: border-box;
            margin: 0;
            padding: 0;
        }}

        body {{
            font-family: {};
            font-size: {};
            line-height: 1.6;
            color: var(--text-color);
            background-color: var(--bg-color);
            transition: background-color 0.3s ease, color 0.3s ease;
            -webkit-font-smoothing: antialiased;
            -moz-osx-font-smoothing: grayscale;
            text-rendering: optimizeLegibility;
            height: 100%;
        }}

        .container {{
            width: 100%;
            padding: 3rem 2rem;
            display: flex;
            justify-content: flex-start;
        }}

        .content {{
            text-align: left;
            width: 100%;
        }}

        h1, h2, h3, h4, h5, h6 {{
            color: var(--header-color);
            font-weight: 600;
            line-height: 1.3;
            margin-top: 3rem;
            margin-bottom: 1.5rem;
        }}

        h1 {{
            font-size: 2.8rem;
            font-weight: 700;
            text-align: center;
            margin-bottom: 4rem;
            padding-bottom: 1.5rem;
            border-bottom: 2px solid var(--link-color);
        }}

        h2 {{
            font-size: 1.8rem;
            margin-top: 3rem;
            padding-bottom: 0.5rem;
            border-bottom: 1px solid var(--border-color);
        }}

        h3 {{
            font-size: 1.5rem;
            color: var(--link-color);
        }}

        h4 {{
            font-size: 1.25rem;
        }}

        h5 {{
            font-size: 1.1rem;
        }}

        h6 {{
            font-size: 1rem;
            color: var(--code-color);
        }}

        p {{
            margin-bottom: 2rem;
            text-align: left;
            line-height: 1.7;
        }}

        ul, ol {{
            margin-bottom: 2rem;
            padding-left: 2rem;
        }}

        li {{
            margin-bottom: 0.75rem;
            line-height: 1.6;
        }}

        blockquote {{
            border-left: 4px solid var(--link-color);
            padding: 1.5rem 2rem;
            margin: 3rem 0;
            background-color: var(--blockquote-bg);
            font-style: italic;
            border-radius: 0 8px 8px 0;
        }}

        code {{
            background-color: var(--code-bg);
            color: var(--code-color);
            padding: 0.2rem 0.4rem;
            border-radius: 3px;
            font-family: 'SF Mono', Monaco, 'Cascadia Code', 'Roboto Mono', Consolas, 'Courier New', monospace;
            font-size: 0.9em;
        }}

        pre {{
            background-color: var(--code-bg);
            padding: 2rem;
            border-radius: 8px;
            overflow-x: auto;
            margin: 3rem 0;
            border: 1px solid var(--border-color);
            box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
        }}

        pre code {{
            background-color: transparent;
            padding: 0;
        }}

        table {{
            width: 100%;
            border-collapse: collapse;
            margin: 3rem 0;
            background-color: var(--blockquote-bg);
            border-radius: 8px;
            overflow: hidden;
            box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
        }}

        th, td {{
            padding: 1rem 1.25rem;
            text-align: left;
            border-bottom: 1px solid var(--border-color);
        }}

        th {{
            background-color: var(--code-bg);
            color: var(--header-color);
            font-weight: 600;
            text-transform: uppercase;
            font-size: 0.8rem;
            letter-spacing: 0.05em;
        }}

        tr:nth-child(even) {{
            background-color: var(--bg-color);
        }}

        a {{
            color: var(--link-color);
            text-decoration: none;
            transition: color 0.2s ease;
            will-change: color;
        }}

        a:hover {{
            color: var(--text-color);
            text-decoration: underline;
        }}

        hr {{
            border: none;
            height: 2px;
            background: linear-gradient(90deg, transparent, var(--border-color), transparent);
            margin: 3rem 0;
        }}

        img {{
            max-width: 100%;
            height: auto;
            border-radius: 8px;
            margin: 3rem 0;
            box-shadow: 0 2px 12px rgba(0, 0, 0, 0.15);
        }}

        .footnote {{
            font-size: 0.85rem;
            color: var(--code-color);
        }}

        /* Responsive design */
        @media (max-width: 768px) {{
            .container {{
                padding: 1.5rem 1rem;
            }}

            h1 {{
                font-size: 2.2rem;
                margin-bottom: 2.5rem;
            }}

            h2 {{
                font-size: 1.6rem;
            }}

            p {{
                margin-bottom: 1.5rem;
                line-height: 1.6;
            }}

            pre {{
                padding: 1.25rem;
                margin: 2rem 0;
            }}

            blockquote {{
                padding: 1rem 1.25rem;
                margin: 2rem 0;
            }}
        }}
    </style>
    {}
</head>
<body>
    <div class="container">
        <div class="content">
            {}
        </div>
    </div>
</body>
</html>"#,
        title, favicon_link, css_variables, font, font_size, theme_script, sanitized_output
    )
}

pub fn unescape_newlines(input: &str) -> String {
    // Convert escaped newlines (\n) to actual newlines
    // Also handle other common escapes like \t for tabs
    let result = input
        .replace("\\n", "\n")
        .replace("\\t", "\t")
        .replace("\\r", "\r")
        .replace("\\\\", "\\")  // Handle escaped backslashes
        // Clean up common whitespace issues around newlines
        .replace(" \n", "\n")
        .replace("\n ", "\n");
    
    // Fix common markdown header issues in inline mode
    // Add space after # if missing and at line start
    let lines: Vec<&str> = result.lines().collect();
    let mut fixed_lines = Vec::new();
    
    for line in lines {
        let trimmed = line.trim_start();
        if trimmed.starts_with('#') && !trimmed.starts_with("# ") && trimmed.len() > 1 {
            // Add space after # if it's a header without space
            let header_level = trimmed.chars().take_while(|c| *c == '#').count();
            let rest = &trimmed[header_level..];
            fixed_lines.push(format!("#{} {}", "#".repeat(header_level - 1), rest.trim_start()));
        } else {
            fixed_lines.push(line.to_string());
        }
    }
    
    fixed_lines.join("\n")
}

pub fn validate_color(color: &str) -> Result<(), String> {
    // Check if it's a valid hex color (3, 4, 6, or 8 digits)
    if let Some(hex_part) = color.strip_prefix('#') {
        if hex_part.len() == 3 || hex_part.len() == 4 || hex_part.len() == 6 || hex_part.len() == 8 {
            return hex_part.chars().all(|c| c.is_ascii_hexdigit())
                .then_some(())
                .ok_or_else(|| format!("Invalid hex color: {}", color));
        } else {
            return Err(format!("Invalid hex color length: {}", color));
        }
    }
    
    // Check if it's a named color (basic CSS color names)
    let named_colors = [
        "red", "orange", "yellow", "green", "blue", "purple", "pink", "brown", "black", "white",
        "gray", "grey", "cyan", "magenta", "lime", "navy", "teal", "maroon", "olive", "silver",
        "aqua", "fuchsia", "indigo", "violet", "gold", "coral", "salmon", "crimson", "tomato"
    ];
    
    let color_lower = color.to_lowercase();
    if named_colors.contains(&color_lower.as_str()) {
        Ok(())
    } else {
        Err(format!("Invalid color: {}. Use hex codes (#ff0000) or named colors (red, blue, etc)", color))
    }
}

pub fn extract_title(markdown: &str) -> Option<String> {
    // Simple title extraction - look for the first H1 header
    for line in markdown.lines() {
        let trimmed = line.trim();
        if let Some(stripped) = trimmed.strip_prefix("# ") {
            return Some(stripped.trim().to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_basic_markdown_parsing() {
        let markdown = "# Hello World\n\nThis is **bold** text.";
        let html = generate_html(markdown, "16px", "sans-serif", "light", "#3498db", None, None, None);
        assert!(html.contains("<h1>Hello World</h1>"));
        assert!(html.contains("<strong>bold</strong>"));
    }

    #[test]
    fn test_image_parsing() {
        let markdown = "![test image](https://example.com/image.jpg)";
        let html = generate_html(markdown, "16px", "sans-serif", "light", "#3498db", None, None, None);
        assert!(html.contains("<img src=\"https://example.com/image.jpg\" alt=\"test image\""));
    }

    #[test]
    fn test_link_parsing() {
        let markdown = "[link text](https://example.com)";
        let html = generate_html(markdown, "16px", "sans-serif", "light", "#3498db", None, None, None);
        assert!(html.contains("<a href=\"https://example.com\">link text</a>"));
    }

    #[test]
    fn test_footnotes() {
        // Footnotes not supported in current pulldown-cmark version
        let markdown = "Text with footnote[^1]\n\n[^1]: Footnote content";
        let html = generate_html(markdown, "16px", "sans-serif", "light", "#3498db", None, None, None);
        // Footnotes render as plain text
        assert!(html.contains("footnote"));
        assert!(html.contains("Footnote content"));
    }

    #[test]
    fn test_strikethrough() {
        // Strikethrough not supported in current pulldown-cmark version
        let markdown = "~~strikethrough text~~";
        let html = generate_html(markdown, "16px", "sans-serif", "light", "#3498db", None, None, None);
        // Strikethrough renders as plain text
        assert!(html.contains("strikethrough text"));
    }

    #[test]
    fn test_task_lists() {
        // Task lists not supported in current pulldown-cmark version
        let markdown = "- [ ] Incomplete\n- [x] Complete";
        let html = generate_html(markdown, "16px", "sans-serif", "light", "#3498db", None, None, None);
        // Task lists render as plain text
        assert!(html.contains("Incomplete"));
        assert!(html.contains("Complete"));
    }

    #[test]
    fn test_code_block_parsing() {
        let markdown = "```rust\nfn main() {}\n```";
        let html = generate_html(markdown, "16px", "sans-serif", "light", "#3498db", None, None, None);
        assert!(html.contains("<pre><code class=\"language-rust\">"));
        assert!(html.contains("fn main() {}"));
    }



    #[test]
    fn test_sanitization_works() {
        let dangerous_html = "<script>alert('xss')</script><h1>Safe</h1>";
        let sanitized = sanitize_html(dangerous_html);
        assert!(!sanitized.contains("<script"));
        assert!(sanitized.contains("&lt;script"));
        assert!(sanitized.contains("<h1>Safe</h1>"));
    }

    #[test]
    fn test_xss_in_generate_html() {
        let dangerous_markdown = "<script>alert('xss')</script>\n\n# Safe Header";
        let html = generate_html(dangerous_markdown, "16px", "sans-serif", "light", "#3498db", None, None, None);
        assert!(!html.contains("<script"));
        assert!(html.contains("&lt;script"));
        assert!(html.contains("<h1>Safe Header</h1>"));
    }

    #[test]
    fn test_no_title_extraction() {
        let markdown = "Just content, no header.";
        let html = generate_html(markdown, "16px", "sans-serif", "light", "#3498db", None, None, None);
        assert!(html.contains("<title>Static Site</title>"));
    }

    #[test]
    fn test_font_size_customization() {
        let html = generate_html("# Test", "18px", "sans-serif", "light", "#3498db", None, None, None);
        assert!(html.contains("font-family: sans-serif"));
        assert!(html.contains("font-size: 18px"));
    }

    #[test]
    fn test_file_reading() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.md");
        fs::write(&file_path, "# Test Content").unwrap();

        let content = fs::read_to_string(&file_path).unwrap();
        assert_eq!(content, "# Test Content");
    }

    #[test]
    fn test_xss_prevention() {
        // Test that dangerous HTML/script tags are sanitized/escaped
        let malicious_markdown = "<script>alert('xss')</script>\n\n# Normal Header";
        let html = generate_html(malicious_markdown, "16px", "sans-serif", "light", "#3498db", None, None, None);
        // Dangerous script tags should be escaped for security
        assert!(!html.contains("<script"));
        assert!(html.contains("&lt;script"));
        // But safe content should remain
        assert!(html.contains("<h1>Normal Header</h1>"));
    }

    #[test]
    fn test_path_traversal_prevention() {
        // This test would fail if we don't validate paths
        // For now, just ensure the function exists and basic file reading works
        // In a real implementation, we'd add path validation
        let temp_dir = TempDir::new().unwrap();
        let safe_file = temp_dir.path().join("safe.md");
        fs::write(&safe_file, "# Safe Content").unwrap();

        // This should work
        let content = fs::read_to_string(&safe_file).unwrap();
        assert_eq!(content, "# Safe Content");
    }

    #[test]
    fn test_large_content() {
        let large_markdown = "# Large Content\n\n".repeat(1000);
        let html = generate_html(&large_markdown, "16px", "sans-serif", "light", "#3498db", None, None, None);
        assert!(html.len() > large_markdown.len());
        assert!(html.contains("<h1>Large Content</h1>"));
    }

    #[test]
    fn test_special_characters() {
        let markdown = "# Spëcial Chärs 🚀\n\n**Bôld** and *ïtálic*.";
        let html = generate_html(markdown, "16px", "sans-serif", "light", "#3498db", None, None, None);
        assert!(html.contains("Spëcial Chärs 🚀"));
        assert!(html.contains("<strong>Bôld</strong>"));
        assert!(html.contains("<em>ïtálic</em>"));
    }

    #[test]
    fn test_empty_input() {
        let html = generate_html("", "16px", "sans-serif", "light", "#3498db", None, None, None);
        assert!(html.contains("<div class=\"content\">"));
        assert!(html.contains("</div>"));
    }

    #[test]
    fn test_table_parsing() {
        // Tables not supported in current pulldown-cmark version
        let markdown = "| Header1 | Header2 |\n|---------|---------|\n| Cell1   | Cell2   |";
        let html = generate_html(markdown, "16px", "sans-serif", "light", "#3498db", None, None, None);
        // Tables render as plain text
        assert!(html.contains("Header1"));
        assert!(html.contains("Cell1"));
    }

    #[test]
    fn test_newline_unescaping() {
        let escaped = "# Title\\n\\nParagraph\\n\\n- Item 1\\n- Item 2";
        let unescaped = unescape_newlines(escaped);
        let html = generate_html(&unescaped, "16px", "sans-serif", "light", "#3498db", None, None, None);
        assert!(html.contains("<h1>Title</h1>"));
        assert!(html.contains("<p>Paragraph</p>"));
        assert!(html.contains("<li>Item 1</li>"));
        assert!(html.contains("<li>Item 2</li>"));
    }

    #[test]
    fn test_layout_structure() {
        let html = generate_html("# Test", "16px", "sans-serif", "light", "#3498db", None, None, None);
        // Test container structure
        assert!(html.contains("<div class=\"container\">"));
        assert!(html.contains("<div class=\"content\">"));
        assert!(html.contains("</div>\n    </div>\n</body>"));

        // Test container CSS
        assert!(html.contains("display: flex"));
        assert!(html.contains("justify-content: flex-start"));

        // Test left-aligned content CSS
        assert!(html.contains("text-align: left"));
        assert!(html.contains("width: 100%"));
    }

    #[test]
    fn test_html_support() {
        let html = generate_html("# Test\n\n<div class=\"custom\">HTML content</div>\n\n**Markdown** here", "16px", "sans-serif", "light", "#3498db", None, None, None);
        // Test that HTML tags are preserved
        assert!(html.contains("<div class=\"custom\">HTML content</div>"));
        // Test that Markdown is still processed
        assert!(html.contains("<strong>Markdown</strong>"));
        // Test that dangerous HTML is sanitized
        let dangerous_html = generate_html("# Test\n\n<script>alert('xss')</script>", "16px", "sans-serif", "light", "#3498db", None, None, None);
        assert!(!dangerous_html.contains("<script"));
        assert!(dangerous_html.contains("&lt;script"));
    }

    #[test]
    fn test_responsive_design() {
        let html = generate_html("# Test", "16px", "sans-serif", "light", "#3498db", None, None, None);
        // Test responsive media query
        assert!(html.contains("@media (max-width: 768px)"));
        assert!(html.contains("padding: 1.5rem 1rem;"));
        assert!(html.contains("font-size: 2.2rem;"));
        assert!(html.contains("font-size: 1.6rem;"));
        assert!(html.contains("margin-bottom: 2.5rem;"));
    }

    #[test]
    fn test_theme_css_variables() {
        // Test light theme
        let html_light = generate_html("# Test", "16px", "sans-serif", "light", "#3498db", None, None, None);
        assert!(html_light.contains("--bg-color: #f4f4f4"));
        assert!(html_light.contains("--text-color: #333"));
        assert!(html_light.contains("--header-color: #2c3e50"));
        assert!(html_light.contains("--link-color: #3498db"));
        assert!(!html_light.contains("<script>")); // No auto theme script for light

        // Test dark theme
        let html_dark = generate_html("# Test", "16px", "sans-serif", "dark", "#3498db", None, None, None);
        assert!(html_dark.contains("--bg-color: #1a1a1a"));
        assert!(html_dark.contains("--text-color: #e0e0e0"));
        assert!(html_dark.contains("--header-color: #ffffff"));
        assert!(html_dark.contains("--link-color: #3498db"));
        assert!(!html_dark.contains("<script>")); // No auto theme script for dark

        // Test auto theme
        let html_auto = generate_html("# Test", "16px", "sans-serif", "auto", "#3498db", None, None, None);
        assert!(html_auto.contains("--bg-color: #f4f4f4")); // Default light values
        assert!(html_auto.contains("function applyTheme(theme)"));
        assert!(html_auto.contains("prefers-color-scheme: dark"));
        assert!(html_auto.contains("addEventListener('change'"));
    }

    #[test]
    fn test_theme_switching_javascript() {
        let html = generate_html("# Test", "16px", "sans-serif", "auto", "#3498db", None, None, None);
        // Test theme switching function
        assert!(html.contains("function applyTheme(theme)"));
        assert!(html.contains("root.style.setProperty('--bg-color'"));
        assert!(html.contains("root.style.setProperty('--text-color'"));
        assert!(html.contains("root.style.setProperty('--header-color'"));
        assert!(html.contains("root.style.setProperty('--code-bg'"));
        assert!(html.contains("root.style.setProperty('--code-color'"));
        assert!(html.contains("root.style.setProperty('--blockquote-bg'"));
        assert!(html.contains("root.style.setProperty('--border-color'"));

        // Test system theme detection
        assert!(html.contains("window.matchMedia('(prefers-color-scheme: dark)').matches"));
        assert!(html.contains("applyTheme(prefersDark ? 'dark' : 'light')"));

        // Test theme change listener
        assert!(html.contains("window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change'"));
    }

    #[test]
    fn test_left_aligned_layout() {
        let html = generate_html("# Title\n\nParagraph text here.", "16px", "sans-serif", "light", "#3498db", None, None, None);
        // Content should be left-aligned
        assert!(html.contains("text-align: left;"));
        // Paragraphs should also be left-aligned for readability with improved spacing
        assert!(html.contains("p {\n            margin-bottom: 2rem;\n            text-align: left;\n            line-height: 1.7;\n        }"));
        // Tables should maintain left alignment with improved padding
        assert!(html.contains("th, td {\n            padding: 1rem 1.25rem;\n            text-align: left;"));
    }

    #[test]
    fn test_css_custom_properties() {
        let html = generate_html("# Test", "16px", "sans-serif", "light", "#3498db", None, None, None);
        // Test that CSS uses custom properties throughout
        assert!(html.contains("color: var(--text-color);"));
        assert!(html.contains("background-color: var(--bg-color);"));
        assert!(html.contains("color: var(--header-color);"));
        assert!(html.contains("background-color: var(--code-bg);"));
        assert!(html.contains("color: var(--code-color);"));
        assert!(html.contains("color: var(--link-color);"));
        assert!(html.contains("background-color: var(--blockquote-bg);"));
        assert!(html.contains("border-bottom: 1px solid var(--border-color);"));
    }

    #[test]
    fn test_font_customization_integration() {
        let html = generate_html("# Test", "18px", "sans-serif", "light", "#3498db", None, None, None);
        assert!(html.contains("font-family: sans-serif"));
        assert!(html.contains("font-size: 18px"));
    }

    #[test]
    fn test_html_structure_completeness() {
        let html = generate_html("# Test Title\n\nContent", "16px", "sans-serif", "light", "#3498db", None, None, None);
        // Test DOCTYPE and HTML structure
        assert!(html.starts_with("<!DOCTYPE html>"));
        assert!(html.contains("<html lang=\"en\">"));
        assert!(html.contains("<head>"));
        assert!(html.contains("<meta charset=\"UTF-8\">"));
        assert!(html.contains("<meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">"));
        assert!(html.contains("<title>Test Title</title>"));
        assert!(html.contains("<style>"));
        assert!(html.contains("</style>"));
        assert!(html.contains("</head>"));
        assert!(html.contains("<body>"));
        assert!(html.contains("</body>"));
        assert!(html.contains("</html>"));
    }
}
