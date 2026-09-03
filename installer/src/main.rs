/*!
VPack Windows Native Installer
Lightweight, pure-Rust installer for deploying VPack Archiver from .vpack archives.
*/

mod install;
mod registry;

use anyhow::{bail, Result};
use clap::Parser;
use std::env;
use std::io::{self, BufRead, Write};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(
    name = "vpack-installer",
    version = "2.0.0",
    about = "VPack Archiver - Native Windows Installer and Shell Integrator"
)]
struct Args {
    /// Path to .vpack package to install (defaults to auto-detecting vpack-archiver-*.vpack in same dir)
    #[arg(short, long)]
    package: Option<PathBuf>,

    /// Installation directory (defaults to %LOCALAPPDATA%\Programs\VPack)
    #[arg(short = 'd', long)]
    dir: Option<PathBuf>,

    /// Perform unattended / silent installation without interactive prompts
    #[arg(short = 'y', long, alias = "silent")]
    unattended: bool,

    /// Skip updating the user PATH environment variable
    #[arg(long)]
    no_path: bool,

    /// Skip registering .vpack file associations and context menu actions
    #[arg(long)]
    no_assoc: bool,

    /// Skip creating Desktop and Start Menu shortcuts
    #[arg(long)]
    no_shortcuts: bool,
}

fn find_candidate_package() -> Option<PathBuf> {
    // 1. Look in current working directory
    if let Ok(entries) = std::fs::read_dir(".") {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext.eq_ignore_ascii_case("vpack") {
                    return Some(path);
                }
            }
        }
    }

    // 2. Look alongside installer executable
    if let Ok(exe_path) = env::current_exe() {
        if let Some(dir) = exe_path.parent() {
            if let Ok(entries) = std::fs::read_dir(dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if let Some(ext) = path.extension() {
                        if ext.eq_ignore_ascii_case("vpack") {
                            return Some(path);
                        }
                    }
                }
            }
        }
    }

    None
}

fn get_default_install_dir() -> PathBuf {
    if let Ok(local_appdata) = env::var("LOCALAPPDATA") {
        PathBuf::from(local_appdata).join("Programs").join("VPack")
    } else if let Ok(user_profile) = env::var("USERPROFILE") {
        PathBuf::from(user_profile).join("VPack")
    } else {
        PathBuf::from("C:\\Program Files\\VPack")
    }
}

fn prompt_confirm(prompt: &str, default: bool) -> bool {
    let suffix = if default { "[Y/n]" } else { "[y/N]" };
    print!("{} {} ", prompt, suffix);
    io::stdout().flush().ok();

    let mut line = String::new();
    let stdin = io::stdin();
    if stdin.lock().read_line(&mut line).is_err() {
        return default;
    }
    let trimmed = line.trim();
    if trimmed.is_empty() {
        default
    } else {
        trimmed.eq_ignore_ascii_case("y") || trimmed.eq_ignore_ascii_case("yes")
    }
}

fn prompt_path(prompt: &str, default: &Path) -> PathBuf {
    print!("{} [default: {}]: ", prompt, default.display());
    io::stdout().flush().ok();

    let mut line = String::new();
    let stdin = io::stdin();
    if stdin.lock().read_line(&mut line).is_err() {
        return default.to_path_buf();
    }
    let trimmed = line.trim();
    if trimmed.is_empty() {
        default.to_path_buf()
    } else {
        PathBuf::from(trimmed)
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    // 1. Locate package
    let vpack_package = match args.package {
        Some(p) => {
            if !p.exists() {
                bail!("specified package '{}' does not exist", p.display());
            }
            p
        }
        None => match find_candidate_package() {
            Some(p) => p,
            None => {
                bail!(
                    "no .vpack archive found in current directory. \
                     Please pass --package <file.vpack>"
                );
            }
        },
    };

    // 2. Determine target folder
    let default_dir = get_default_install_dir();

    let (target_dir, add_to_path, register_associations, create_shortcuts) = if args.unattended {
        (
            args.dir.unwrap_or(default_dir),
            !args.no_path,
            !args.no_assoc,
            !args.no_shortcuts,
        )
    } else {
        println!("===============================================================");
        println!("  🗁 VPack Archiver Setup");
        println!("===============================================================");
        println!("  Package: {}", vpack_package.display());
        println!();

        let chosen_dir = args
            .dir
            .unwrap_or_else(|| prompt_path("  Select destination folder", &default_dir));

        let add_path = if !args.no_path {
            prompt_confirm("  Add VPack to User PATH environment variable?", true)
        } else {
            false
        };

        let assoc = if !args.no_assoc {
            prompt_confirm(
                "  Associate .vpack files with VPack Archiver GUI & Explorer context menu?",
                true,
            )
        } else {
            false
        };

        let shortcuts = if !args.no_shortcuts {
            prompt_confirm(
                "  Create Start Menu and Desktop shortcuts for VPack Archiver GUI?",
                true,
            )
        } else {
            false
        };

        (chosen_dir, add_path, assoc, shortcuts)
    };

    let opts = install::InstallOptions {
        target_dir,
        add_to_path,
        register_associations,
        create_shortcuts,
    };

    install::run_install(&vpack_package, &opts)
}
