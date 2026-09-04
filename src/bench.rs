/*!
Hardware-accelerated benchmark suite for Deflate, LZ4, and CRC-32.
*/

use anyhow::Result;
use flate2::read::DeflateDecoder;
use flate2::write::DeflateEncoder;
use flate2::Compression;
use std::io::{Read, Write};
use std::time::Instant;

pub fn run_benchmark(size_mb: usize) -> Result<()> {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!(" ⚡ VPack Archiver Multi-Codec Benchmark Suite");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!(
        " Workload: {} MB entropy dataset  (mixed data pattern)",
        size_mb
    );

    let total_bytes = size_mb * 1024 * 1024;
    let mut data = Vec::with_capacity(total_bytes);
    for i in 0..total_bytes {
        data.push(((i * 31 + (i >> 3)) ^ (i >> 7)) as u8);
    }

    // [1/5] Deflate compression
    print!(" [1/5] Deflate Compress  (level 6) ... ");
    let t = Instant::now();
    let mut encoder = DeflateEncoder::new(Vec::new(), Compression::new(6));
    encoder.write_all(&data)?;
    let compressed_deflate = encoder.finish()?;
    let deflate_comp_s = t.elapsed().as_secs_f64();
    let deflate_comp_mbps = size_mb as f64 / deflate_comp_s;
    let deflate_ratio = (1.0 - compressed_deflate.len() as f64 / data.len() as f64) * 100.0;
    println!(
        "{:.1} MB/s  ({:.1}% space saved)",
        deflate_comp_mbps,
        deflate_ratio.max(0.0)
    );

    // [2/5] Deflate decompression
    print!(" [2/5] Deflate Decompress          ... ");
    let t = Instant::now();
    let mut decoder = DeflateDecoder::new(&compressed_deflate[..]);
    let mut decompressed = Vec::with_capacity(data.len());
    decoder.read_to_end(&mut decompressed)?;
    let deflate_decomp_mbps = size_mb as f64 / t.elapsed().as_secs_f64();
    println!("{:.1} MB/s", deflate_decomp_mbps);

    // [3/5] LZ4 compression (pure Rust, ultra fast)
    print!(" [3/5] LZ4 Compress     (frame)    ... ");
    let t = Instant::now();
    let compressed_lz4 = lz4_flex::compress_prepend_size(&data);
    let lz4_comp_s = t.elapsed().as_secs_f64();
    let lz4_comp_mbps = size_mb as f64 / lz4_comp_s;
    let lz4_ratio: f64 = (1.0 - (compressed_lz4.len() as f64 / data.len() as f64)) * 100.0;
    println!(
        "{:.1} MB/s  ({:.1}% space saved)",
        lz4_comp_mbps,
        lz4_ratio.max(0.0)
    );

    // [4/5] LZ4 decompression
    print!(" [4/5] LZ4 Decompress   (frame)    ... ");
    let t = Instant::now();
    let _decompressed_lz4 = lz4_flex::decompress_size_prepended(&compressed_lz4)?;
    let lz4_decomp_s = t.elapsed().as_secs_f64();
    let lz4_decomp_mbps = size_mb as f64 / lz4_decomp_s;
    println!("{:.1} MB/s", lz4_decomp_mbps);

    // [5/5] CRC-32 checksum
    print!(" [5/5] CRC-32 Checksum  (SSE4.2)   ... ");
    let t = Instant::now();
    let crc = crate::archive::crc32_compute(&data);
    let crc_mbps = size_mb as f64 / t.elapsed().as_secs_f64();
    println!("{:.1} MB/s  (CRC32: {:08X})", crc_mbps, crc);

    println!("───────────────────────────────────────────────────────────────────────");
    println!(" Summary:");
    println!(
        "   Deflate compress:    {:>8.1} MB/s   ratio: {:.1}%",
        deflate_comp_mbps,
        deflate_ratio.max(0.0)
    );
    println!("   Deflate decompress:  {:>8.1} MB/s", deflate_decomp_mbps);
    println!(
        "   LZ4 compress:        {:>8.1} MB/s   ratio: {:.1}%   ← ultra fast",
        lz4_comp_mbps,
        lz4_ratio.max(0.0)
    );
    println!(
        "   LZ4 decompress:      {:>8.1} MB/s   ← instant",
        lz4_decomp_mbps
    );
    println!("   CRC-32 checksum:     {:>8.1} MB/s", crc_mbps);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    Ok(())
}
