// Enhanced Nerd Font Installer CLI tool with progress bars, colored output, and improved UX
use clap::{Parser, Subcommand};
use std::io::{self, Write};
use dirs;
use std::fs;
use std::path::PathBuf;
use reqwest::blocking::get;
use zip::ZipArchive;
use std::fs::File;
use indicatif::{ProgressBar, ProgressStyle};
use colored::Colorize;
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(name = "nerd-font-installer")]
#[command(about = "A CLI tool to list, download, and install Nerd Fonts", long_about = None)]
#[command(version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List available Nerd Fonts
    List {
        /// Show detailed information about fonts
        #[arg(short, long)]
        details: bool,
    },
    /// Install one or more Nerd Fonts
    Install {
        /// Names of fonts to install
        fonts: Vec<String>,
        /// Skip confirmation prompts
        #[arg(short, long)]
        yes: bool,
    },
    /// Update all installed Nerd Fonts
    Update,
    /// Remove installed Nerd Fonts
    Remove {
        /// Names of fonts to remove
        fonts: Vec<String>,
    },
    /// Show information about installed fonts
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FontInfo {
    name: String,
    asset_name: String,
    description: String,
    variants: Vec<String>,
    size_mb: f32,
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::List { details } => {
            let fonts = get_nerd_fonts();
            if *details {
                show_detailed_font_list(&fonts);
            } else {
                show_simple_font_list(&fonts);
            }
        }
        Commands::Install { fonts, yes } => {
            let os = detect_os();
            let all_fonts = get_nerd_fonts();
            let to_install = if fonts.is_empty() {
                select_fonts_interactively(&all_fonts)
            } else {
                // Match font names from command line
                fonts.iter()
                    .filter_map(|name| {
                        all_fonts.iter()
                            .find(|f| f.name.to_lowercase().contains(&name.to_lowercase()))
                            .cloned()
                    })
                    .collect()
            };

            if to_install.is_empty() {
                println!("{}", "No fonts selected for installation.".yellow());
                return;
            }

            if !*yes {
                println!("\n{}", "Fonts to install:".bright_blue().bold());
                for font in &to_install {
                    println!("  • {} ({:.1} MB)", font.name.bright_white(), font.size_mb);
                }
                print!("\n{} ", "Continue with installation? [y/N]:".bright_yellow());
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                if !input.trim().to_lowercase().starts_with('y') {
                    println!("{}", "Installation cancelled.".yellow());
                    return;
                }
            }

            println!("{} {}", "Detected OS:".bright_blue(), os.bright_white());
            let mut success_count = 0;
            let total = to_install.len();
            
            for (i, font) in to_install.iter().enumerate() {
                println!("\n{} [{}/{}] Installing {}...", 
                    "INFO".bright_blue().bold(), 
                    i + 1, 
                    total, 
                    font.name.bright_white().bold()
                );
                
                match download_and_install_font(font, &os) {
                    Ok(_) => {
                        println!("{} Successfully installed '{}'!", 
                            "✓".bright_green().bold(), 
                            font.name.bright_white()
                        );
                        success_count += 1;
                    },
                    Err(e) => {
                        eprintln!("{} Failed to install '{}': {}", 
                            "✗".bright_red().bold(), 
                            font.name.bright_white(), 
                            e.to_string().red()
                        );
                    }
                }
            }
            
            println!("\n{} Installed {}/{} fonts successfully.", 
                "SUMMARY".bright_blue().bold(), 
                success_count, 
                total
            );
        }
        Commands::Update => {
            println!("{}", "Update functionality not yet implemented.".yellow());
        }
        Commands::Remove { fonts } => {
            remove_fonts(fonts);
        }
        Commands::Info => {
            show_installed_fonts_info();
        }
    }
}

fn show_simple_font_list(fonts: &[FontInfo]) {
    println!("{}", "Available Nerd Fonts:".bright_blue().bold());
    for (i, font) in fonts.iter().enumerate() {
        println!("  {}. {}", 
            format!("{}", i + 1).bright_cyan(), 
            font.name.bright_white()
        );
    }
}

fn show_detailed_font_list(fonts: &[FontInfo]) {
    println!("{}", "Available Nerd Fonts (Detailed):".bright_blue().bold());
    for (i, font) in fonts.iter().enumerate() {
        println!("\n{}. {}", 
            format!("{}", i + 1).bright_cyan().bold(), 
            font.name.bright_white().bold()
        );
        println!("   {}: {}", "Description".bright_yellow(), font.description);
        println!("   {}: {:.1} MB", "Size".bright_yellow(), font.size_mb);
        println!("   {}: {}", "Variants".bright_yellow(), font.variants.join(", "));
    }
}

fn select_fonts_interactively(all_fonts: &[FontInfo]) -> Vec<FontInfo> {
    println!("{}", "No fonts specified. Available Nerd Fonts:".bright_blue().bold());
    for (i, font) in all_fonts.iter().enumerate() {
        println!("  {}. {} ({:.1} MB)", 
            format!("{}", i + 1).bright_cyan(), 
            font.name.bright_white(),
            font.size_mb
        );
    }
    println!("\n{}", "Enter numbers separated by commas to select fonts, or 'all' to install all:".bright_yellow());
    print!("{} ", ">".bright_green().bold());
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    
    if input.eq_ignore_ascii_case("all") {
        all_fonts.to_vec()
    } else {
        input.split(',')
            .filter_map(|s| s.trim().parse::<usize>().ok())
            .filter_map(|i| all_fonts.get(i - 1).cloned())
            .collect::<Vec<_>>()
    }
}

fn download_and_install_font(font: &FontInfo, os: &str) -> Result<(), Box<dyn std::error::Error>> {
    let zip_url = format!(
        "https://github.com/ryanoasis/nerd-fonts/releases/latest/download/{}",
        font.asset_name
    );
    
    println!("  {}", format!("Downloading from: {}", zip_url).dimmed());
    
    // Create progress bar
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.green} {msg}")
        .unwrap());
    pb.set_message("Downloading...");
    pb.enable_steady_tick(std::time::Duration::from_millis(120));
    
    let resp = get(&zip_url)?;
    if !resp.status().is_success() {
        pb.finish_and_clear();
        return Err(format!("Failed to download font zip: {}", resp.status()).into());
    }
    
    pb.set_message("Saving file...");
    let mut out_path = std::env::temp_dir();
    out_path.push(&font.asset_name);
    let mut out_file = File::create(&out_path)?;
    let bytes = resp.bytes()?;
    std::io::copy(&mut bytes.as_ref(), &mut out_file)?;
    
    pb.set_message("Installing fonts...");
    
    // Unzip to font directory
    let font_dir = get_user_font_dir(os)?;
    fs::create_dir_all(&font_dir)?;
    let zip_file = File::open(&out_path)?;
    let mut archive = ZipArchive::new(zip_file)?;
    
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        if let Some(name) = file.enclosed_name() {
            if let Some(file_name) = name.file_name() {
                // Only extract .ttf, .otf font files
                if let Some(ext) = file_name.to_str() {
                    if ext.ends_with(".ttf") || ext.ends_with(".otf") {
                        let outpath = font_dir.join(file_name);
                        let mut outfile = File::create(&outpath)?;
                        std::io::copy(&mut file, &mut outfile)?;
                    }
                }
            }
        }
    }
    
    // Cleanup temp file
    let _ = fs::remove_file(&out_path);
    
    pb.finish_and_clear();
    println!("  {}", format!("Installed to: {:?}", font_dir).dimmed());
    Ok(())
}

fn remove_fonts(fonts: &[String]) {
    println!("{}", "Font removal not yet implemented.".yellow());
    for font in fonts {
        println!("  Would remove: {}", font.bright_white());
    }
}

fn show_installed_fonts_info() {
    println!("{}", "Installed font detection not yet implemented.".yellow());
}

fn get_user_font_dir(os: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let dir = match os {
        "macOS" => {
            let mut d = dirs::home_dir().ok_or("Cannot find home directory")?;
            d.push("Library/Fonts");
            d
        },
        "Linux" => {
            let mut d = dirs::home_dir().ok_or("Cannot find home directory")?;
            d.push(".local/share/fonts");
            d
        },
        "Windows" => {
            let mut d = dirs::data_dir().ok_or("Cannot find data directory")?;
            d.push("Microsoft/Windows/Fonts");
            d
        },
        _ => return Err("Unsupported OS".into()),
    };
    Ok(dir)
}

fn get_nerd_fonts() -> Vec<FontInfo> {
    vec![
        FontInfo {
            name: "FiraCode Nerd Font".to_string(),
            asset_name: "FiraCode.zip".to_string(),
            description: "Monospaced font with programming ligatures".to_string(),
            variants: vec!["Regular".to_string(), "Bold".to_string(), "Light".to_string()],
            size_mb: 2.1,
        },
        FontInfo {
            name: "Hack Nerd Font".to_string(),
            asset_name: "Hack.zip".to_string(),
            description: "A typeface designed for source code".to_string(),
            variants: vec!["Regular".to_string(), "Bold".to_string(), "Italic".to_string()],
            size_mb: 1.8,
        },
        FontInfo {
            name: "JetBrainsMono Nerd Font".to_string(),
            asset_name: "JetBrainsMono.zip".to_string(),
            description: "Typeface for developers by JetBrains".to_string(),
            variants: vec!["Regular".to_string(), "Bold".to_string(), "Italic".to_string()],
            size_mb: 2.3,
        },
        FontInfo {
            name: "SourceCodePro Nerd Font".to_string(),
            asset_name: "SourceCodePro.zip".to_string(),
            description: "Monospaced font family by Adobe".to_string(),
            variants: vec!["Regular".to_string(), "Bold".to_string(), "Light".to_string()],
            size_mb: 1.9,
        },
        FontInfo {
            name: "DejaVuSansMono Nerd Font".to_string(),
            asset_name: "DejaVuSansMono.zip".to_string(),
            description: "Monospaced version of DejaVu Sans".to_string(),
            variants: vec!["Regular".to_string(), "Bold".to_string(), "Oblique".to_string()],
            size_mb: 1.5,
        },
        FontInfo {
            name: "CascadiaCode Nerd Font".to_string(),
            asset_name: "CascadiaCode.zip".to_string(),
            description: "Microsoft's programming font with ligatures".to_string(),
            variants: vec!["Regular".to_string(), "SemiLight".to_string(), "Light".to_string()],
            size_mb: 2.0,
        },
        FontInfo {
            name: "Meslo Nerd Font".to_string(),
            asset_name: "Meslo.zip".to_string(),
            description: "Customized version of Apple's Menlo font".to_string(),
            variants: vec!["Regular".to_string(), "Bold".to_string(), "Italic".to_string()],
            size_mb: 1.7,
        },
    ]
}

fn detect_os() -> &'static str {
    #[cfg(target_os = "macos")]
    { "macOS" }
    #[cfg(target_os = "linux")]
    { "Linux" }
    #[cfg(target_os = "windows")]
    { "Windows" }
    #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
    { "Unknown" }
}
