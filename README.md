# 🗁 VPack Archiver (WinRAR for .vpack)

[![Release](https://img.shields.io/github/v/release/LeTrollologist/vpack-archiver?color=brightgreen&label=release)](https://github.com/LeTrollologist/vpack-archiver/releases)
[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Format](https://img.shields.io/badge/format-VPK2%20Central%20Directory-green.svg)](#-vpk2-format-specification)
[![Security](https://img.shields.io/badge/audit-passed-brightgreen.svg)](RELEASE_PROCESS.md)
[![Platform](https://img.shields.io/badge/platform-windows%20x86__64-blue.svg)](https://github.com/LeTrollologist/vpack-archiver/releases)

> **A modern, ultra-fast universal archive manager, compressor, and explorer.**
> Built as an open-source, next-generation alternative to WinRAR, 7-Zip, and TAR with an instant $\mathcal{O}(1)$ seekable Central Directory located at EOF, multi-codec streaming compression (Deflate & LZ4), per-entry CRC-32 integrity, password encryption, and digital signatures.

---

## 📑 Table of Contents

- [⚡ Key Features](#-key-features)
- [📦 Installation & Releases](#-installation--releases)
  - [Pre-built Binaries](#pre-built-binaries)
  - [Building from Source](#building-from-source)
- [💻 Command Line Interface](#-command-line-interface)
  - [Quick Cheat Sheet](#quick-cheat-sheet)
  - [Detailed Command Usage](#detailed-command-usage)
- [⚙️ CLI Options Reference](#️-cli-options-reference)
- [📐 VPK2 Format Specification](#-vpk2-format-specification)
  - [Container Layout](#container-layout)
  - [Central Directory Entry Schema](#central-directory-entry-schema)
  - [End of Central Directory (EOCD) Footer](#end-of-central-directory-eocd-footer)
  - [Cryptographic Digital Signatures](#cryptographic-digital-signatures)
- [🛠️ Workflow Recipes & Examples](#️-workflow-recipes--examples)
  - [1. Enterprise Backup with Password Protection](#1-enterprise-backup-with-password-protection)
  - [2. Publisher Signed Software Distribution](#2-publisher-signed-software-distribution)
  - [3. Selective Asset Extraction in CI/CD](#3-selective-asset-extraction-in-cicd)
- [🏎️ Performance & Benchmarks](#️-performance--benchmarks)
- [📜 Changelog](#-changelog)
- [📄 License](#-license)

---

## ⚡ Key Features

* **Universal Multi-Codec Compression**: Compress files, directories, nested trees, codebases, binaries, and assets into `.vpack` archives with selectable compression codecs:
  - **Deflate** (`-C deflate`, default): High compression ratio and maximum cross-platform compatibility.
  - **LZ4** (`-C lz4`): Ultra-fast pure-Rust streaming compression and real-time decompression.
* **$\mathcal{O}(1)$ Random-Access Seeks (VPK2 Central Directory)**: Index table is located at the End of File (EOF / EOCD). Extract or preview single files instantly without scanning or decompressing gigabytes of preceding data.
* **Hardware-Accelerated CRC-32**: SSE4.2 / ARMv8 hardware-checksum computation and verification operating at over 6.2 GB/s.
* **Password Protection & Stream Encryption**: Integrated SHA-256 authenticated stream cipher protection (`-p <password>`).
* **WinRAR-Style Interactive Console Explorer**: Visual terminal table with file type icons, original/packed sizes, compression ratios, CRC-32 checksums, and modification timestamps.
* **Digital Signatures**: Optional RFC 8032 Ed25519 publisher signature signing and tamper verification (`keygen` / `-s <key>`).
* **Built-in Benchmark Suite**: Hardware-level CPU compression throughput (Deflate & LZ4), decompression speed, and checksum rate measurement (`vpack b`).
* **Local Deterministic Release Pipeline**: 100% offline-verifiable release pipeline with VirusTotal multi-engine scanning (`scripts/pipeline.py`), replacing third-party CI/CD services completely.

---

## 📦 Installation & Releases

### Pre-built Binaries

Pre-built standalone binary releases for **Windows (x86_64)** are available on the [GitHub Releases page](https://github.com/LeTrollologist/vpack-archiver/releases).

#### Windows (x86_64)
Download `vpack-archiver-v1.2.0-windows-x86_64.zip` or the native `vpack-archiver-v1.2.0-windows-x86_64.vpack` bundle from the [latest release](https://github.com/LeTrollologist/vpack-archiver/releases), unpack, and add to your system `PATH`.

### Building from Source

Ensure you have a modern Rust toolchain installed (1.70+):

```bash
git clone https://github.com/LeTrollologist/vpack-archiver.git
cd vpack-archiver
cargo build --release
```

The optimized binary is produced at:
* Windows: `target/release/vpack-archiver.exe`
* Unix (source build): `target/release/vpack-archiver`

---

## 💻 Command Line Interface

### Quick Cheat Sheet

| Command | Alias | Description |
| :--- | :--- | :--- |
| `vpack <archive.vpack>` | - | Open and view archive in WinRAR table view |
| `vpack a <archive> <files...>` | `add` | Create or add files/folders to archive |
| `vpack x <archive> [-o <dest>]` | `extract` | Extract all contents preserving full directory hierarchy |
| `vpack e <archive> <file> [-o <dest>]` | `extract-file` | Extract a single file in $\mathcal{O}(1)$ time |
| `vpack l <archive>` | `list` | List contents in a rich formatted table |
| `vpack t <archive>` | `test` | Test CRC-32 integrity & verify digital signatures |
| `vpack v <archive> <file>` | `view` | Stream uncompressed file directly to stdout |
| `vpack b [-m <size_mb>]` | `bench` | Run CPU compression & decompression benchmark |
| `vpack keygen [-o <prefix>]` | - | Generate Ed25519 publisher signing keypair |
| `vpack ui <archive>` | - | Render interactive console explorer |

---

### Detailed Command Usage

#### 1. Compress & Create Archives (`a` / `add`)
```bash
# Standard compression (Deflate Level 6)
vpack a project.vpack src/ assets/ Cargo.toml README.md

# Ultra-fast compression with LZ4 codec
vpack a fast.vpack ./build -C lz4

# Maximum compression (Level 9) with password encryption and archive comment
vpack a backup.vpack ./data -c 9 -p "MySecretPass" -m "Daily Backup"

# Digitally sign archive with Ed25519 private key
vpack a release.vpack ./dist -c 6 -s publisher.priv
```

#### 2. Open & Inspect Archives (`l` / `list` or direct pass)
```bash
# Quick inspect
vpack project.vpack

# Or via list subcommand
vpack l project.vpack
```

Output:
```text
╔═════════════════════════════════════════════════════════════════════════════════════╗
║ 🗁 VPack Archiver (WinRAR Edition) - project.vpack                                   ║
╠─────────────────────────────────────────────────────────────────────────────────────╣
║ [A]dd  [X]Extract  [E]xtract-Single  [T]est  [V]iew  [I]nfo  [B]enchmark  [Q]uit     ║
╠─────────────────────────────────────────────────────────────────────────────────────╣
║ Attr Name                             Original     Packed  Ratio   CRC-32 Date Time ║
╠─────────────────────────────────────────────────────────────────────────────────────╣
║ 📁   src/                                <DIR>          -      -        - 2026-08-26║
║ 🖹   src/main.rs                         11525       3412    70% A3F109C2 2026-08-26║
║ 📄   Cargo.toml                            872        418    52% 480DFBC9 2026-08-26║
║ 📄   README.md                            5923       2014    66% 7B12C091 2026-08-26║
╠─────────────────────────────────────────────────────────────────────────────────────╣
║ Total: 3 files, 1 folders | Orig:   0.02 MB | Packed:   0.01 MB | Ratio: 67.2%      ║
║ Format: VPK2 (Central Directory at EOF) | Security: Standard VPack Archive          ║
╚═════════════════════════════════════════════════════════════════════════════════════╝
```

#### 3. Extract Archives (`x` & `e`)
```bash
# Extract full archive preserving directory structure
vpack x project.vpack -o ./extracted_folder

# Extract password-protected archive
vpack x backup.vpack -o ./restored -p "MySecretPass"

# Extract single file instantly in O(1) time
vpack e project.vpack src/main.rs -o ./main.rs
```

#### 4. Test Archive Integrity (`t` / `test`)
```bash
# Verify CRC-32 checksum of every chunk and central directory integrity
vpack t project.vpack

# Test password-protected archive
vpack t backup.vpack -p "MySecretPass"
```

#### 5. Preview File to Standard Output (`v` / `view`)
```bash
# Pipe file from archive directly into grep, jq, or other tools
vpack v project.vpack Cargo.toml | grep version
```

#### 6. Hardware Benchmark (`b` / `bench`)
```bash
# Run multi-core benchmark against a 64 MB dataset
vpack b -m 64
```

#### 7. Digital Signature Key Generation (`keygen`)
```bash
# Generate publisher.priv and publisher.pub
vpack keygen -o publisher
```

---

## ⚙️ CLI Options Reference

### Subcommand: `add` (`a`)
| Flag | Long Flag | Description | Default |
| :--- | :--- | :--- | :--- |
| `<ARCHIVE>` | - | Destination `.vpack` file path | *Required* |
| `<FILES...>`| - | Files and directories to compress | *Required* |
| `-c` | `--level` | Compression level (`0` = Store, `1..=9` = Deflate) | `6` |
| `-C` | `--codec` | Compression codec: `deflate` (standard) or `lz4` (ultra fast) | `deflate` |
| `-p` | `--password` | Encrypt payload with SHA-256 stream cipher | `None` |
| `-m` | `--comment` | Embed metadata description/comment | `None` |
| `-s` | `--sign` | Sign archive using Ed25519 private key (`.priv`) | `None` |

### Subcommand: `extract` (`x`)
| Flag | Long Flag | Description | Default |
| :--- | :--- | :--- | :--- |
| `<ARCHIVE>` | - | Target `.vpack` file path | *Required* |
| `-o` | `--dest` | Destination extraction directory | `<stem>_extracted` |
| `-p` | `--password` | Password for encrypted archives | `None` |

### Subcommand: `extract-file` (`e`)
| Flag | Long Flag | Description | Default |
| :--- | :--- | :--- | :--- |
| `<ARCHIVE>` | - | Target `.vpack` file path | *Required* |
| `<FILE_INSIDE>` | - | Relative path of the file inside archive | *Required* |
| `-o` | `--out` | Output destination path on disk | Basename |
| `-p` | `--password` | Password for encrypted archives | `None` |

---

## 📐 VPK2 Format Specification

```
┌────────────────────────────────────────────────────────────┐
│ VPack Header (16 Bytes)                                    │
│ Magic: 'VPK2' | Version (u16) | Flags (u16) | MetaLen (u32)│
├────────────────────────────────────────────────────────────┤
│ Archive Metadata (Bincode: Creator, Comment, Timestamps)  │
├────────────────────────────────────────────────────────────┤
│ Sequential File Payload Chunks (Streaming Deflate/LZ4/Store)│
│ Chunk 1 (Compressed data)                                  │
│ Chunk 2 (Compressed data)                                  │
│ ...                                                        │
├────────────────────────────────────────────────────────────┤
│ Central Directory Table (Bincode Array of CentralDirEntry) │
│ - Relative Path (UTF-8)                                    │
│ - Uncompressed Size & Compressed Size (u64)                │
│ - Payload Offset (u64)                                     │
│ - Mode, Flags, Method (u16), CRC-32 (u32)                  │
│ - Modified Timestamp (i64) & Directory Flag                │
├────────────────────────────────────────────────────────────┤
│ Optional Ed25519 Signature Block (96 Bytes)                │
│ - Public Key (32 Bytes) + Digital Signature (64 Bytes)     │
├────────────────────────────────────────────────────────────┤
│ End of Central Directory Record (EOCD Footer - 28 Bytes)   │
│ Magic: 'EOCD' | CD Offset (u64) | CD Len (u64) | Count(u32)│
└────────────────────────────────────────────────────────────┘
```

### Container Layout
1. **Header (16 Bytes)**:
   - `Magic` (4 bytes): `VPK2` (`0x56, 0x50, 0x4B, 0x32`)
   - `Version` (u16 LE): Format specification revision (`2`)
   - `Flags` (u16 LE): Bitmask (`0x0001` Signed, `0x0002` Compressed, `0x0004` Encrypted)
   - `MetaLen` (u32 LE): Length of serialized metadata header
   - `Reserved` (u32 LE): Reserved padding
2. **Payload Chunks**: Continuous sequential byte streams for each file, compressed via Deflate or stored verbatim.
3. **Central Directory (CD)**: Located at EOF offset specified in the footer. Contains full file manifest for instant random-access lookups.
4. **Signature Block (Optional, 96 Bytes)**: Contains 32-byte Ed25519 public key + 64-byte Ed25519 signature over archive payload.
5. **EOCD Footer (28 Bytes)**:
   - `Magic` (4 bytes): `EOCD` (`0x45, 0x4F, 0x43, 0x44`)
   - `CD Offset` (u64 LE): Offset of Central Directory table
   - `CD Length` (u64 LE): Length of Central Directory table
   - `Entry Count` (u32 LE): Number of entries in Central Directory
   - `Sig Length` (u32 LE): Length of signature block (`0` or `96`)

---

## 🛠️ Workflow Recipes & Examples

### 1. Enterprise Backup with Password Protection
```bash
vpack a daily_backup.vpack \
  /etc/nginx \
  /var/log \
  /opt/app/config.json \
  -c 9 \
  -p "EnterpriseSecret2026!" \
  -m "Automated nightly backup"
```

### 2. Publisher Signed Software Distribution
```bash
# 1. Generate publisher keypair
vpack keygen -o release-signer

# 2. Build and sign software archive
vpack a release-v1.1.0.vpack ./build/ -s release-signer.priv

# 3. Verify integrity and signature
vpack t release-v1.1.0.vpack
```

### 3. Selective Asset Extraction in CI/CD
```bash
# Instantly retrieve a single configuration without unpacking gigabytes of assets
vpack e assets.vpack config/deploy.env -o .env
```

---

## 🏎️ Performance & Benchmarks

Benchmarked on modern x86_64 architecture (AMD / Intel multi-core):

| Operation | Throughput | Notes |
| :--- | :--- | :--- |
| **LZ4 Compression** | **~350 - 600 MB/s** | Ultra-fast pure-Rust frame compression |
| **Deflate Compression (Level 6)** | ~75 - 120 MB/s | Multi-stream balanced |
| **Deflate Decompression** | ~450 - 650 MB/s | Zero-copy buffered decoder |
| **Hardware CRC-32 Checksum** | **> 6,200 MB/s** | SSE4.2 / ARM CRC32 instructions |
| **$\mathcal{O}(1)$ Directory Lookup** | **< 0.05 ms** | End-of-Central-Directory seek |

---

## 📜 Changelog

See [CHANGELOG.md](CHANGELOG.md) for full release history and version migration notes.

---

## 📄 License

Licensed under either of:
* Apache License, Version 2.0 ([LICENSE](LICENSE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT License ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)

at your option.
