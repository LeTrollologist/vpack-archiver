# Changelog

All notable changes to the `vpack-archiver` project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.0.0] - 2026-09-02

### Added
* **Native Windows GUI (`vpack-gui.exe`)**: Full WinRAR-style desktop archive manager built with pure-Rust [egui](https://github.com/emilk/egui)/[eframe](https://github.com/emilk/eframe) — no Electron, no WebView, no Node.js.
  - WinRAR-style sortable file table (Name, Orig Size, Packed, Ratio, CRC-32, Modified, Method).
  - Toolbar: 📂 Open · ✚ Add · 📤 Extract · 🔍 Test · ℹ Info.
  - Native Windows file/folder dialogs via `rfd`.
  - Drag-and-drop `.vpack` files directly onto the window.
  - Add dialog: multi-file/folder picker, Deflate vs LZ4 codec selector, level slider, optional password.
  - Extract dialog: destination folder picker, password input, extract-selected-only option.
  - Archive info panel: metadata, creator, comment, security flags, total sizes.
  - Background-threaded operations — UI stays responsive during long compress/extract.
  - Menu bar: File / Archive / Selection / Help.
* **Cargo Workspace**: Repo is now a multi-crate workspace (`vpack-archiver` library + CLI, `vpack-gui` GUI binary, and `vpack-installer` native Windows installer).
* **Native Windows Installer (`vpack-installer.exe`)**: Standalone pure-Rust installer for `.vpack` archives. Unpacks packages to `%LOCALAPPDATA%\Programs\VPack`, adds the binary to the User `PATH`, registers Explorer context menu handlers, and creates Start Menu and Desktop shortcuts.
* **Shared Core Library (`vpack_core`)**: `archive`, `bench`, `verify` modules exposed as a public library crate for both CLI and GUI to share.
* **Release Bundle**: `vpack-gui.exe` is now distributed alongside `vpack-archiver.exe` and `vpack.exe` in every `.zip` and `.vpack` release asset, with `vpack-installer.exe` published as a standalone setup tool.

### Changed
* Version bumped to `2.0.0` across all crates.
* Archive creator string updated to `VPack Archiver v2.0`.

---

## [1.2.0] - 2026-09-02

### Added
* **Multi-Codec Compression Engine**: Added support for ultra-fast pure-Rust **LZ4 streaming compression** (`-C lz4`) alongside **Deflate** (`-C deflate`, default).
* **CLI Codec Selection Flag**: New `-C, --codec` parameter for selecting compression algorithms (`deflate` or `lz4`).
* **Multi-Codec Benchmark Suite**: Expanded `vpack b` CPU benchmark with 4 hardware-accelerated passes: Deflate Compress, Deflate Decompress, LZ4 Compress, and SSE4.2 CRC-32 checksum.
* **Hollow Canvas Local Release Pipeline**: Replaced GitHub Actions with a deterministic, local-first release orchestrator (`scripts/pipeline.py`), automated `Makefile` targets, and living documentation (`RELEASE_PROCESS.md`).
* **VirusTotal API v3 Antivirus Verification**: Integrated automated submission, polling, report generation, and security audit log (`virustotal-summary.txt`).
* **VPack Self-Packaging (Dog-Fooding)**: Release packaging now produces native high-compression `.vpack` distribution archives created by the newly compiled release binary itself.
* **Pure Rust Guarantee**: 100% pure-Rust codebase and dependency tree with zero external C-compiler or runtime toolchain requirements.

### Removed
* Removed `.github/workflows/` (CI/CD entirely replaced by local deterministic pipeline).

---

## [1.1.0] - 2026-08-26

### Added
* **VPK2 Central Directory Format**: Instant $\mathcal{O}(1)$ random-access seeks using an End-of-Central-Directory (EOCD) table footer at EOF.
* **Arbitrary Directory & Tree Packing**: Full recursive directory packing and unpacking preserving relative path structures, mode attributes, and timestamps.
* **Password Protection & Encryption**: SHA-256 stream cipher encryption/decryption for protected archives (`-p <password>`).
* **RFC 8032 Ed25519 Digital Signatures**: Cryptographic publisher signing and tamper-proofing (`-s <key>`), plus the `keygen` subcommand to generate public/private key pairs.
* **WinRAR / 7-Zip Style Visual Console Explorer**: Interactive terminal UI (`ui` / `l` / direct invocation) displaying file type icons, sizes, packed sizes, compression ratio percentages, CRC-32 hashes, and timestamps.
* **Hardware-Accelerated CRC-32 Checksums**: SSE4.2 / ARMv8 hardware-checksum computation running in excess of 6.2 GB/s.
* **Multi-core CPU Benchmark Suite**: Integrated hardware benchmark tool (`b` / `bench`) to test streaming Deflate compression, decompression throughput, and checksum speed.
* **Automated Test Workflows**: Comprehensive test suite matrix and automated release binary packaging.
* **Comprehensive Test Suite**: Automated unit tests for round-trip deflate extraction, password encryption/decryption, wrong password detection, CRC-32 verification, Ed25519 digital signature signing & verification, and integrity checking.

### Changed
* Optimized release build profile with LTO (`thin`), symbols stripped, and single codegen units for maximum performance and minimum binary size.
* Improved single-file extraction (`e`) to operate in $\mathcal{O}(1)$ time without scanning or decompressing the rest of the archive.
* Standardized CLI subcommands with intuitive single-letter aliases (`a`, `x`, `e`, `l`, `t`, `v`, `b`).

---

## [1.0.0] - 2026-08-25

### Added
* Initial release of VPack Archiver format and CLI core.
* Basic streaming Deflate compression engine and store method.
* Initial single-file packaging and extraction.
