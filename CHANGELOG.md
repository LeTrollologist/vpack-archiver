# Changelog

All notable changes to the `vpack-archiver` project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
* **Automated CI / CD Workflows**: GitHub Actions test suite matrix across Windows, Linux, and macOS, with automated multi-platform release binary packaging.
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
