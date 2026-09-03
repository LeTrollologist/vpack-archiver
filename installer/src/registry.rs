use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use winreg::enums::HKEY_CURRENT_USER;
use winreg::RegKey;

/// Add target directory to User environment PATH in Windows Registry
pub fn add_to_user_path(bin_dir: &Path) -> Result<bool> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (env_key, _) = hkcu
        .create_subkey("Environment")
        .context("failed to open HKCU\\Environment")?;

    let current_path: String = env_key.get_value("Path").unwrap_or_default();
    let bin_str = bin_dir.to_string_lossy().to_string();

    let paths: Vec<&str> = current_path.split(';').map(|s| s.trim()).collect();
    for p in &paths {
        if p.eq_ignore_ascii_case(&bin_str) {
            return Ok(false); // Already present
        }
    }

    let new_path = if current_path.is_empty() {
        bin_str
    } else {
        format!("{};{}", current_path.trim_end_matches(';'), bin_str)
    };

    env_key
        .set_value("Path", &new_path)
        .context("failed to update User PATH environment variable")?;

    Ok(true)
}

/// Register .vpack file association in HKCU\Software\Classes
pub fn register_vpack_file_association(gui_path: &Path, cli_path: &Path) -> Result<()> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (classes_key, _) = hkcu
        .create_subkey("Software\\Classes")
        .context("failed to open HKCU\\Software\\Classes")?;

    // 1. Associate .vpack extension with ProgID "VPack.Archive"
    let (ext_key, _) = classes_key.create_subkey(".vpack")?;
    ext_key.set_value("", &"VPack.Archive")?;

    // 2. Configure ProgID "VPack.Archive"
    let (progid_key, _) = classes_key.create_subkey("VPack.Archive")?;
    progid_key.set_value("", &"VPack Compressed Archive")?;

    // Set DefaultIcon
    let (icon_key, _) = progid_key.create_subkey("DefaultIcon")?;
    let icon_val = format!("\"{}\",0", gui_path.to_string_lossy());
    icon_key.set_value("", &icon_val)?;

    // Shell action: "Open" (Default action -> Opens in GUI)
    let (shell_key, _) = progid_key.create_subkey("shell")?;
    shell_key.set_value("", &"open")?;

    let (open_key, _) = shell_key.create_subkey("open")?;
    open_key.set_value("", &"Open with VPack")?;
    let (open_cmd_key, _) = open_key.create_subkey("command")?;
    let open_cmd = format!("\"{}\" \"%1\"", gui_path.to_string_lossy());
    open_cmd_key.set_value("", &open_cmd)?;

    // Shell action: "Extract Here" -> Invokes CLI vpack x "%1"
    let (extract_key, _) = shell_key.create_subkey("extract")?;
    extract_key.set_value("", &"Extract with VPack")?;
    let (extract_cmd_key, _) = extract_key.create_subkey("command")?;
    let extract_cmd = format!("\"{}\" x \"%1\"", cli_path.to_string_lossy());
    extract_cmd_key.set_value("", &extract_cmd)?;

    Ok(())
}

/// Create a Windows Shortcut (.lnk) using PowerShell Script via standard library
pub fn create_shortcut(target: &Path, shortcut_path: &Path, description: &str) -> Result<()> {
    let target_str = target.to_string_lossy();
    let shortcut_str = shortcut_path.to_string_lossy();
    let parent = shortcut_path.parent();
    if let Some(p) = parent {
        let _ = std::fs::create_dir_all(p);
    }

    let script = format!(
        "$ws = New-Object -ComObject WScript.Shell; \
         $s = $ws.CreateShortcut('{}'); \
         $s.TargetPath = '{}'; \
         $s.Description = '{}'; \
         $s.WorkingDirectory = '{}'; \
         $s.Save()",
        shortcut_str.replace('\'', "''"),
        target_str.replace('\'', "''"),
        description.replace('\'', "''"),
        target
            .parent()
            .unwrap_or(target)
            .to_string_lossy()
            .replace('\'', "''")
    );

    let _ = std::process::Command::new("powershell")
        .args(["-NoProfile", "-Command", &script])
        .output();

    Ok(())
}

/// Get Start Menu Programs directory
pub fn get_start_menu_programs_dir() -> Option<PathBuf> {
    std::env::var_os("APPDATA").map(|appdata| {
        PathBuf::from(appdata)
            .join("Microsoft")
            .join("Windows")
            .join("Start Menu")
            .join("Programs")
    })
}

/// Get Desktop directory
pub fn get_desktop_dir() -> Option<PathBuf> {
    std::env::var_os("USERPROFILE").map(|u| PathBuf::from(u).join("Desktop"))
}
