use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use statgen::{generate_html, unescape_newlines, validate_color};

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    font_size: Option<String>,
    font: Option<String>,
    theme: Option<String>,
    accent: Option<String>,
    accent_light: Option<String>,
    accent_dark: Option<String>,
    output: Option<String>,
    favicon: Option<String>,
}

fn load_config() -> Option<Config> {
    let config_files = ["statgen.json", "statgen.yaml", "statgen.yml"];

    for filename in &config_files {
        if Path::new(filename).exists() {
            match fs::read_to_string(filename) {
                Ok(content) => {
                    if filename.ends_with(".json") {
                        match serde_json::from_str(&content) {
                            Ok(config) => return Some(config),
                            Err(e) => eprintln!("Warning: Failed to parse {}: {}", filename, e),
                        }
                    } else {
                        match serde_yaml::from_str(&content) {
                            Ok(config) => return Some(config),
                            Err(e) => eprintln!("Warning: Failed to parse {}: {}", filename, e),
                        }
                    }
                }
                Err(e) => eprintln!("Warning: Failed to read {}: {}", filename, e),
            }
        }
    }
    None
}



#[derive(Parser)]
#[command(name = "statgen")]
#[command(about = "A modern static site generator that converts Markdown to beautiful HTML websites")]
#[command(long_about = "StatGen converts Markdown files to responsive HTML websites with customizable styling.
Supports inline input, file processing, batch operations, and configuration files.")]
struct Cli {
    /// Path to markdown file to process
    #[arg(short, long, help = "Specify the path to a Markdown file (.md) to convert to HTML")]
    file: Option<String>,

    /// Directory containing markdown files to process (all .md files)
    #[arg(short, long = "directory", help = "Directory containing Markdown files (.md) to convert to HTML. Processes all .md files in the directory")]
    directory: Option<String>,

    /// Inline markdown content
    #[arg(short = 'i', long, long_help = "Provide markdown content directly as a string. For multi-line content: use \\n for newlines, or shell syntax: $'Line1\\nLine2' (bash) or \"Line1`nLine2\" (PowerShell)")]
    inline: Option<String>,

    /// Output directory for generated files
    #[arg(short, long, help = "Directory where the generated HTML file(s) will be saved. Default is \"dist\".")]
    output: Option<String>,



    /// Font size for the website
    #[arg(long, help = "CSS font-size value (e.g., '16px', '1.2em', '14pt')")]
    font_size: Option<String>,

    /// Theme for the website
    #[arg(long, value_parser = ["light", "dark", "auto"], help = "Theme for the website: light, dark, or auto (detects system preference)")]
    theme: Option<String>,

    /// Accent color for the website
    #[arg(long, long_help = "Accent color for the website (affects h1/h3 colors, h1 underline, link colors). Accepts color names (red, blue, etc) or hex codes (#ff0000, #3498db)")]
    accent: Option<String>,

    /// Accent color for light mode (when theme is auto)
    #[arg(long, long_help = "Accent color for light mode when using auto theme. Accepts color names (red, blue, etc) or hex codes (#ff0000, #3498db)")]
    accent_light: Option<String>,

    /// Accent color for dark mode (when theme is auto)
    #[arg(long, long_help = "Accent color for dark mode when using auto theme. Accepts color names (red, blue, etc) or hex codes (#ff0000, #3498db)")]
    accent_dark: Option<String>,

    

    /// Font family for the website
    #[arg(short = 'F', long, value_parser = ["Arial", "Helvetica", "Times New Roman", "Georgia", "Verdana", "Courier New", "monospace", "sans-serif", "serif"], help = "Font family for the website. Options: Arial, Helvetica, Times New Roman, Georgia, Verdana, Courier New, monospace, sans-serif, serif")]
    font: Option<String>,

    /// Emoji for favicon
    #[arg(short = 'f', long, help = "Emoji to use as favicon (e.g., 🚀, 📚, 🌟)")]
    favicon: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // Load configuration file if present
    let config = load_config();

    // Apply defaults: CLI > Config > Hardcoded defaults
    let output = cli.output
        .or_else(|| config.as_ref().and_then(|c| c.output.clone()))
        .unwrap_or_else(|| "dist".to_string());

    let font_size = cli.font_size
        .or_else(|| config.as_ref().and_then(|c| c.font_size.clone()))
        .unwrap_or_else(|| "16px".to_string());

    let cli_font = cli.font.is_some();
    let config_font = config.as_ref().and_then(|c| c.font.as_ref()).is_some();
    let custom_font = cli_font || config_font;
    
    let font = cli.font
        .or_else(|| config.as_ref().and_then(|c| c.font.clone()))
        .unwrap_or_else(|| "sans-serif".to_string());

    // Show warning if custom font is specified
    if custom_font {
        eprintln!("Warning: Make sure font you requested is installed on your system");
    }

    let theme = cli.theme
        .or_else(|| config.as_ref().and_then(|c| c.theme.clone()))
        .unwrap_or_else(|| "auto".to_string());

    let accent = cli.accent
        .or_else(|| config.as_ref().and_then(|c| c.accent.clone()))
        .unwrap_or_else(|| "#3498db".to_string());

    let accent_light = cli.accent_light.as_ref()
        .or_else(|| config.as_ref().and_then(|c| c.accent_light.as_ref()));

    let accent_dark = cli.accent_dark.as_ref()
        .or_else(|| config.as_ref().and_then(|c| c.accent_dark.as_ref()));

    let favicon = cli.favicon
        .or_else(|| config.as_ref().and_then(|c| c.favicon.clone()));

    // Validate accent color
    if let Err(e) = validate_color(&accent) {
        eprintln!("Error: {}", e);
        return Err(anyhow::anyhow!("Invalid accent color"));
    }

    // Validate accent_light color
    if let Some(accent_light) = accent_light {
        if let Err(e) = validate_color(accent_light) {
            eprintln!("Error: {}", e);
            return Err(anyhow::anyhow!("Invalid accent-light color"));
        }
    }

    // Validate accent_dark color
    if let Some(accent_dark) = accent_dark {
        if let Err(e) = validate_color(accent_dark) {
            eprintln!("Error: {}", e);
            return Err(anyhow::anyhow!("Invalid accent-dark color"));
        }
    }



    match fs::create_dir_all(&output) {
        Ok(_) => {},
        Err(e) => {
            eprintln!("Error creating output directory '{}': {}", output, e);
            return Err(e.into());
        }
    }

    if let Some(dir_path) = cli.directory {
                // Batch processing: process all .md files in directory
                println!("Processing all .md files in directory: {}", dir_path);

                let dir_entries = match fs::read_dir(&dir_path) {
                    Ok(entries) => entries,
                    Err(e) => {
                        eprintln!("Error reading directory '{}': {}", dir_path, e);
                        return Err(e.into());
                    }
                };

                let mut processed_count = 0;
                for entry in dir_entries {
                    let entry = match entry {
                        Ok(e) => e,
                        Err(e) => {
                            eprintln!("Error reading directory entry: {}", e);
                            continue;
                        }
                    };

                    let path = entry.path();
                    if let Some(extension) = path.extension() {
                        if extension == "md" {
                            let file_name = path.file_stem()
                                .and_then(|s| s.to_str())
                                .unwrap_or_else(|| {
                                    eprintln!("Warning: Could not get file name for: {}", path.display());
                                    "index"
                                });
                            println!("Processing: {}", path.display());

                            let markdown_content = match fs::read_to_string(&path) {
                                Ok(content) => content,
                                Err(e) => {
                                    eprintln!("Error reading file '{}': {}", path.display(), e);
                                    continue;
                                }
                            };

let html_content = generate_html(&markdown_content, &font_size, &font, &theme, &accent, accent_light.map(|s| s.as_str()), accent_dark.map(|s| s.as_str()), favicon.as_deref());
                            let output_filename = format!("{}.html", file_name);
                            let output_path = Path::new(&output).join(output_filename);

                            match fs::write(&output_path, html_content) {
                                Ok(_) => {
                                    println!("✓ Generated: {}", output_path.display());
                                    processed_count += 1;
                                },
                                Err(e) => {
                                    eprintln!("Error writing to '{}': {}", output_path.display(), e);
                                }
                            }
                        }
                    }
                }



                if processed_count == 0 {
                    println!("No .md files found in directory '{}'", dir_path);
                } else {
                    println!("✓ Successfully processed {} Markdown file(s)", processed_count);
                }
    } else {
        // Single file or inline processing
        let markdown_content = if let Some(file_path) = cli.file {
            println!("Reading markdown from file: {}", file_path);
            match fs::read_to_string(&file_path) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("Error reading file '{}': {}", file_path, e);
                    return Err(e.into());
                }
            }
        } else if let Some(content) = cli.inline {
            println!("Using inline markdown content");
            // Handle both escaped and actual newlines
            unescape_newlines(&content)
                .replace("`n", "\n")   // PowerShell style
        } else {
            return Err(anyhow::anyhow!("Error: Either --file, --directory, or --inline must be provided. Use statgen --help for help"));
        };

        println!("Markdown content length: {} characters", markdown_content.len());

        let html_content = generate_html(&markdown_content, &font_size, &font, &theme, &accent, accent_light.map(|s| s.as_str()), accent_dark.map(|s| s.as_str()), favicon.as_deref());
        let output_path = Path::new(&output).join("index.html");

        match fs::write(&output_path, html_content) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error writing to '{}': {}", output_path.display(), e);
                return Err(e.into());
            }
        }



        println!("✓ Static site generated successfully at: {}", output_path.display());
    }

    Ok(())
}
