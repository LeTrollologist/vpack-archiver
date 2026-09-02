# VPack Archiver: Local Release & Packaging Process

This document outlines the standardized, local-first release pipeline for VPack Archiver. All builds, tests, packaging, security scans, VirusTotal verification, and publishing operations run deterministically on the local developer machine without relying on external CI/CD services.

---

## 1. Overview & Core Principles

1. **No GitHub Actions**: Everything is built, tested, scanned, and published locally via `scripts/pipeline.py` and the `gh` CLI.
2. **Canonical Asset Naming**: All assets follow `vpack-archiver-v{VER}-{platform}-{arch}.{ext}`.
3. **Clean Distribution Bundles**: Only standalone archives (`.zip` and `.vpack`), `SHA256SUMS.txt`, and security audit summaries (`virustotal-summary.txt`) are published as release assets.
4. **VPack Self-Packaging (Dog-Fooding)**: Every release builds a native high-compression `.vpack` archive using the freshly compiled release binary itself.
5. **VirusTotal & Binary Hygiene**: Every release zip is submitted for multi-engine antivirus verification and linked directly in the release notes.

---

## 2. Release Pipeline Stages

| Stage | Operation | Description |
| :--- | :--- | :--- |
| **`preflight`** | Tool verification | Confirms `rustc`, `cargo`, `gh`, and `cargo-audit` are present |
| **`build`** | Optimized compilation | `cargo build --release` with LTO thin and symbol stripping |
| **`test`** | Test suite | `cargo test --workspace` (5/5 unit tests) |
| **`security`** | Audit logs | Generates `dist/v{VER}/audit/security-audit.txt` and runs `cargo audit` |
| **`package`** | Asset bundling | Generates `.zip` and `.vpack` archives with canonical naming |
| **`verify`** | Checksums & lint | Generates `SHA256SUMS.txt` and tests `.vpack` integrity via `vpack t` |
| **`virustotal`** | Antivirus Scan | Submits zip via VirusTotal API v3 and generates audit report |
| **`publish`** | GitHub Release | Creates release on GitHub and uploads canonical assets |

---

## 3. Running a Release

### Environment Configuration (Optional for VirusTotal API)
Set your VirusTotal API key to automate scanning and verification polling:
```powershell
$env:VIRUSTOTAL_API_KEY = "your-virustotal-api-key-here"
```
*(If no API key is set, the pipeline automatically generates canonical VirusTotal GUI permalinks based on SHA-256 digests).*

### Full Release (Build, Test, Package, VirusTotal & Publish)
```bash
python scripts/pipeline.py v1.2.0
```
*or via Make:*
```bash
make release TAG=v1.2.0
```

### Local Build & Package Only (No GitHub Upload)
```bash
python scripts/pipeline.py v1.2.0 --no-publish
```

### Create as GitHub Draft Release
```bash
python scripts/pipeline.py v1.2.0 --draft
```

---

## 4. Output Layout (`dist/`)

```text
dist/v1.2.0/
├── windows-staging/                                    # Staging folder
│   ├── vpack-archiver.exe                              # Release binary
│   ├── vpack.exe                                       # Convenience alias
│   ├── README.md                                       # Documentation
│   ├── LICENSE                                         # License
│   └── CHANGELOG.md                                    # Version history
├── vpack-archiver-v1.2.0-windows-x86_64.zip            # Standard Zip distribution
├── vpack-archiver-v1.2.0-windows-x86_64.vpack          # VPack distribution
├── SHA256SUMS.txt                                      # Cryptographic checksums
├── release_notes.md                                    # Release markdown body
└── audit/
    ├── security-audit.txt                              # Comprehensive security audit log
    ├── cargo-audit.txt                                 # RustSec dependency vulnerability audit
    ├── virustotal-summary.txt                          # VirusTotal scan analysis summary
    └── virustotal-report.json                         # Full VirusTotal v3 JSON response
```

---

## 5. Verification & Integrity

To verify released packages:
```powershell
# Check SHA-256
certutil -hashfile vpack-archiver-v1.2.0-windows-x86_64.zip SHA256

# Verify VPACK integrity and CRC-32
vpack t vpack-archiver-v1.2.0-windows-x86_64.vpack
```

---

## 6. Installation & Extraction

### Option A: Via Native Windows Zip
```powershell
Expand-Archive -Path .\vpack-archiver-v1.2.0-windows-x86_64.zip -DestinationPath C:\Tools\VPack
[Environment]::SetEnvironmentVariable("PATH", "C:\Tools\VPack;" + $env:PATH, "User")
```

### Option B: Via VPack Archiver
```bash
# Extract all contents
vpack x vpack-archiver-v1.2.0-windows-x86_64.vpack -o ./vpack/
```
