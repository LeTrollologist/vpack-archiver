use anyhow::{Context, Result};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use vpack_core::archive::VpackArchive;

pub struct InstallOptions {
    pub target_dir: PathBuf,
    pub add_to_path: bool,
    pub register_associations: bool,
    pub create_shortcuts: bool,
}

pub fn run_install(vpack_file: &Path, opts: &InstallOptions) -> Result<()> {
    println!();
    println!("═══════════════════════════════════════════════════════════════");
    println!("  📦 VPack Archiver Native Windows Installer");
    println!("═══════════════════════════════════════════════════════════════");
    println!("  Package Source: {}", vpack_file.display());
    println!("  Install Target: {}", opts.target_dir.display());
    println!("───────────────────────────────────────────────────────────────");

    // 1. Verify and read package
    print!("  [1/4] Reading and validating VPack package... ");
    io::stdout().flush().ok();
    let archive = VpackArchive::open(vpack_file).with_context(|| {
        format!(
            "failed to open .vpack package at '{}'",
            vpack_file.display()
        )
    })?;

    // Quick test integrity
    archive
        .test_integrity(None)
        .context("package integrity validation failed — archive may be corrupted")?;
    println!("✓ OK ({} items found)", archive.central_directory.len());

    // 2. Extract into target directory
    print!("  [2/4] Deploying application binaries... ");
    io::stdout().flush().ok();
    fs::create_dir_all(&opts.target_dir)?;
    archive.extract_all(&opts.target_dir, None)?;
    println!("✓ Done");

    let cli_exe = opts.target_dir.join("vpack.exe");
    let gui_exe = opts.target_dir.join("vpack-gui.exe");

    // Also ensure vpack-archiver.exe alias exists if only vpack.exe was unpacked, or vice versa
    if cli_exe.exists() && !opts.target_dir.join("vpack-archiver.exe").exists() {
        let _ = fs::copy(&cli_exe, opts.target_dir.join("vpack-archiver.exe"));
    } else if opts.target_dir.join("vpack-archiver.exe").exists() && !cli_exe.exists() {
        let _ = fs::copy(opts.target_dir.join("vpack-archiver.exe"), &cli_exe);
    }

    // 3. User PATH Registration
    if opts.add_to_path {
        print!("  [3/4] Updating User PATH environment variable... ");
        io::stdout().flush().ok();
        match crate::registry::add_to_user_path(&opts.target_dir) {
            Ok(added) => {
                if added {
                    println!("✓ Added to HKCU\\Environment\\Path");
                } else {
                    println!("✓ (Already in PATH)");
                }
            }
            Err(e) => println!("⚠ Notice: {}", e),
        }
    } else {
        println!("  [3/4] User PATH update skipped.");
    }

    // 4. File Associations & Shortcuts
    if opts.register_associations || opts.create_shortcuts {
        print!("  [4/4] Configuring Windows Explorer integration... ");
        io::stdout().flush().ok();

        if opts.register_associations {
            if let Err(e) = crate::registry::register_vpack_file_association(&gui_exe, &cli_exe) {
                println!("\n  ⚠ Warning registering .vpack file association: {}", e);
            }
        }

        if opts.create_shortcuts {
            // Start menu shortcut
            if let Some(programs_dir) = crate::registry::get_start_menu_programs_dir() {
                let shortcut_dir = programs_dir.join("VPack");
                let gui_shortcut = shortcut_dir.join("VPack Archiver.lnk");
                let _ = crate::registry::create_shortcut(
                    &gui_exe,
                    &gui_shortcut,
                    "VPack Archiver Desktop GUI",
                );
            }
            // Desktop shortcut
            if let Some(desktop_dir) = crate::registry::get_desktop_dir() {
                let desk_shortcut = desktop_dir.join("VPack Archiver.lnk");
                let _ = crate::registry::create_shortcut(
                    &gui_exe,
                    &desk_shortcut,
                    "VPack Archiver Desktop GUI",
                );
            }
        }

        println!("✓ Done");
    } else {
        println!("  [4/4] Shell integration skipped.");
    }

    println!("═══════════════════════════════════════════════════════════════");
    println!("  ✨ Installation successfully completed!");
    println!("  You can now launch 'vpack-gui.exe' or type 'vpack' in any terminal.");
    println!("═══════════════════════════════════════════════════════════════\n");

    Ok(())
}
