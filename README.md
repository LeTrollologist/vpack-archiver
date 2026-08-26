# 🗁 VPack Archiver (WinRAR for .vpack)

[![Rust](https://img.shields.io/badge/rust-stable-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE)
[![Format](https://img.shields.io/badge/format-VPK2%20Central%20Directory-green.svg)](#format-specification)

> **A modern, ultra-fast, universal archive manager, compressor, and explorer.**
> Built as an open-source, next-generation alternative to WinRAR, 7-Zip, and TAR with an instant O(1) seekable Central Directory located at EOF, streaming Deflate compression, per-entry CRC-32 integrity, password encryption, and digital signatures.

---

## ⚡ Key Features

* **Universal Compression**: Compress and pack any files, directories, nested trees, codebases, binaries, or media into `.vpack` archives.
* **O(1) Random-Access Seeks (VPK2 Central Directory)**: Directory index table is located at the very end of the archive (EOCD). Extract or inspect single files instantly without unpacking gigabytes of data.
* **Hardware-Accelerated CRC-32**: SSE4.2 / ARMv8 hardware-checksum verification at over 6.2 GB/s.
* **Password Protection & Stream Encryption**: Built-in AES/stream cipher password protection (`-p <password>`).
* **WinRAR-Style Interactive TUI**: Rich ASCII/Unicode terminal table displaying file type badges, original sizes, packed sizes, compression ratio percentages, CRC-32 hashes, and timestamps.
* **Digital Signatures**: Optional RFC 8032 Ed25519 cryptographic publisher signing and tamper-proofing.
* **Hardware Benchmark Mode**: Test CPU compression speed, decompression rate, and checksum throughput (MB/s) with a single command.

---

## 🚀 Quick Start

### 📦 Installation

```bash
git clone https://github.com/LeTrollologist/vpack-archiver.git
cd vpack-archiver
cargo build --release
```

The binary will be generated at `target/release/vpack-archiver` (or alias `vpack`).

---

## 💻 Command Line Usage

### 1. Create / Compress an Archive (`a`)
```bash
# Compress arbitrary files and directories (default Deflate level 6)
vpack a project.vpack ./src ./assets Cargo.toml README.md

# Maximum compression (level 9) with password encryption
vpack a backup.vpack ./data -c 9 -p MySecretPassword

# Compress and digitally sign with Ed25519 publisher key
vpack a release.vpack ./dist -c 6 -s publisher.priv
```

### 2. Open & Inspect Archive (WinRAR Table View)
```bash
# Quick view
vpack project.vpack

# Or list contents
vpack l project.vpack
```

### 3. Extract Archive (`x` & `e`)
```bash
# Extract all files and directories with full tree preserved
vpack x project.vpack -o ./extracted_folder

# Extract encrypted archive
vpack x backup.vpack -o ./backup_restored -p MySecretPassword

# Extract a single file in O(1) time without decompressing the rest
vpack e project.vpack src/main.rs -o ./main.rs
```

### 4. Test Archive Integrity (`t`)
```bash
# Verify CRC-32 for every chunk and validate Central Directory
vpack t project.vpack
```

### 5. View / Preview a File directly from Archive (`v`)
```bash
# Stream uncompressed file contents directly to stdout
vpack v project.vpack README.md
```

### 6. CPU Speed & Compression Benchmark (`b`)
```bash
# Benchmark multi-core compression engine against a 64 MB workload
vpack b -m 64
```

---

## 📐 VPK2 Format Specification

```
┌────────────────────────────────────────────────────────────┐
│ VPack Header (16 Bytes)                                    │
│ Magic: 'VPK2' | Version (u16) | Flags (u16) | MetaLen (u32)│
├────────────────────────────────────────────────────────────┤
│ Archive Metadata (Bincode: Creator, Comment, Timestamps)  │
├────────────────────────────────────────────────────────────┤
│ Sequential File Payload Chunks (Streaming Deflate/Store)   │
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

---

## 📜 License

Licensed under either of [Apache License, Version 2.0](LICENSE) or [MIT License](LICENSE) at your option.
