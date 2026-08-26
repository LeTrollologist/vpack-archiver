# 🗁 VPack Archiver (WinRAR for .vpack)

> Modern, blazing-fast, cross-platform archive manager, compressor, and explorer for the .vpack universal application package format.

---

## ⚡ Highlights

* **VPK2 Central Directory Architecture**: Instant (1)$ random-access file seeks located at the end of the file — extract single files without decompressing the entire archive.
* **Universal Cross-Platform Compression**: High-efficiency streaming Deflate compression with hardware-accelerated CRC-32 checksums ($>6\text{ GB/s}$).
* **RFC 8032 Ed25519 Cryptographic Signatures**: Built-in tamper-proofing and publisher verification.
* **WinRAR-Style Interactive Visual TUI**: Real-time attribute tables, packed vs unpacked size breakdown, compression ratio analytics, and icon badges.
* **Hardware Benchmark Suite**: Measure CPU compression, decompression, and CRC32 throughput in MB/s.

---

## 📦 Installation

`ash
# Build from source
cargo build --release

# The binary will be in target/release/vpack-archiver
`

---

## 🚀 Usage

### 1. Interactive Explorer (WinRAR TUI)
`ash
vpack-archiver my-app.vpack
`

### 2. Add / Compress Files into Archive
`ash
# Compress files with level 6 Deflate (default)
vpack-archiver a app.vpack ./bin ./assets

# Compress with custom level (0=Store to 9=Max) and sign
vpack-archiver a app.vpack ./bin -c 9 -s publisher.priv
`

### 3. Extract Archive
`ash
# Extract all files
vpack-archiver x app.vpack -o ./extracted_folder

# Extract a single file in O(1) time
vpack-archiver e app.vpack binary.exe -o ./binary.exe
`

### 4. Test Integrity & Checksums
`ash
vpack-archiver t app.vpack
`

### 5. Benchmark Performance
`ash
vpack-archiver b -m 32
`

---

## 📜 License

MIT OR Apache-2.0
