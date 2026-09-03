# 📦 VPack Archiver Installation Guide

Complete, step-by-step installation instructions for **VPack Archiver 2.0** on Windows (x86_64).

---

## 📑 Methods Overview

| Method | Target Audience | Highlights |
| :--- | :--- | :--- |
| **Method 1: One-Click Rust Installer** *(Recommended)* | End Users & Desktop | Automated directory setup, User `PATH` updates, Desktop & Start Menu shortcuts, `.vpack` Explorer context menu integration |
| **Method 2: Native `.vpack` Deployment** | Power Users & Developers | Extract and deploy directly using existing `vpack` CLI binary or dog-fooding |
| **Method 3: Portable ZIP Setup** | System Admins & Portable Drives | Zero installation, extract anywhere, no registry entries required |
| **Method 4: Building from Source** | Developers & Contributors | Pure-Rust Cargo build with SSE4.2 / hardware acceleration |

---

## Method 1: One-Click Rust Installer (`vpack-installer.exe`)

The official installer is written in 100% pure Rust and requires **no administrative privileges**. It unpacks `.vpack` packages directly and configures your Windows shell.

### 1. Download Release Files
From the [GitHub Releases](https://github.com/LeTrollologist/vpack-archiver/releases):
- `vpack-installer.exe`
- `vpack-archiver-v2.0.0-windows-x86_64.vpack`

Place both files into the same directory (e.g. `Downloads`).

### 2. Run the Installer
Double-click `vpack-installer.exe` or run from PowerShell:

```powershell
.\vpack-installer.exe
```

The installer will guide you interactively through:
1. **Destination Directory** (Default: `%LOCALAPPDATA%\Programs\VPack`)
2. **PATH Configuration**: Appends the binary directory to your user `PATH` environment variable in the registry (`HKCU\Environment\Path`) so you can type `vpack` from any terminal.
3. **File Associations**: Registers `.vpack` in Windows Explorer:
   - Double-clicking opens in **VPack Archiver GUI** (`vpack-gui.exe`).
   - Right-click menu adds **"Extract with VPack"**.
4. **Shortcuts**: Creates Desktop and Start Menu shortcuts.

### 3. Automated / Unattended Installation (For Scripts & CI)
To install silently with all defaults enabled:
```powershell
.\vpack-installer.exe --silent
```

Custom installation directory unattended:
```powershell
.\vpack-installer.exe -p .\vpack-archiver-v2.0.0-windows-x86_64.vpack -d C:\Tools\VPack --silent
```

---

## Method 2: Native `.vpack` Archive Extraction

If you already have a `vpack.exe` binary on your system, you can dog-food and unpack the native release package:

```bash
# 1. Download release package
# vpack-archiver-v2.0.0-windows-x86_64.vpack

# 2. Test package integrity
vpack t vpack-archiver-v2.0.0-windows-x86_64.vpack

# 3. Extract to target directory
vpack x vpack-archiver-v2.0.0-windows-x86_64.vpack -o C:\Tools\VPack
```

---

## Method 3: Portable ZIP Installation

For portable USB drives or environments without registry modifications:

1. Download `vpack-archiver-v2.0.0-windows-x86_64.zip`.
2. Extract the archive into your preferred directory:
   ```powershell
   Expand-Archive -Path .\vpack-archiver-v2.0.0-windows-x86_64.zip -DestinationPath C:\Tools\VPack
   ```
3. (Optional) Add to your User PATH manually:
   ```powershell
   [Environment]::SetEnvironmentVariable("PATH", "C:\Tools\VPack;" + [Environment]::GetEnvironmentVariable("PATH", "User"), "User")
   ```
4. Verify from a new PowerShell or Command Prompt:
   ```powershell
   vpack --version
   ```

---

## Method 4: Building from Source

Requires a standard Rust toolchain (1.70+):

```bash
# Clone the repository
git clone https://github.com/LeTrollologist/vpack-archiver.git
cd vpack-archiver

# Build all workspace binaries (core lib, CLI, GUI, and installer)
cargo build --release --workspace

# Run tests
cargo test --workspace
```

The resulting binaries will be available in `target/release/`:
- `vpack-archiver.exe` (CLI suite)
- `vpack-gui.exe` (Desktop GUI application)
- `vpack-installer.exe` (Standalone Windows installer)

---

## 🔒 Cryptographic Verification

All release assets include SHA-256 digests in `SHA256SUMS.txt`. You can verify file authenticity:

```powershell
# Using certutil on Windows
certutil -hashfile vpack-installer.exe SHA256
certutil -hashfile vpack-archiver-v2.0.0-windows-x86_64.vpack SHA256
certutil -hashfile vpack-archiver-v2.0.0-windows-x86_64.zip SHA256

# Verify .vpack integrity via VPack CLI
vpack t vpack-archiver-v2.0.0-windows-x86_64.vpack
```
