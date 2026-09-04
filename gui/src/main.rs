/*!
VPack Archiver 2.0 — Modern Desktop Archive Manager
Built with wry + tao (native Evergreen WebView2 engine).
Universal, hardware-accelerated, high-ratio compression and security suite.
*/

// Hide console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod html;

use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use serde::{Deserialize, Serialize};
use tao::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
    window::WindowBuilder,
};
use vpack_core::archive::{
    collect_directory_entries, CentralDirEntry, VpackArchive,
    FLAG_COMPRESSED, FLAG_ENCRYPTED, FLAG_SIGNED,
};
use wry::WebViewBuilder;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action")]
enum IpcCommand {
    #[serde(rename = "ui_ready")]
    UiReady,
    #[serde(rename = "open_dialog")]
    OpenDialog,
    #[serde(rename = "open_path")]
    OpenPath { path: String },
    #[serde(rename = "extract_dialog")]
    ExtractDialog { password: Option<String> },
    #[serde(rename = "test_integrity")]
    TestIntegrity { password: Option<String> },
    #[serde(rename = "run_benchmark")]
    RunBenchmark { size_mb: Option<usize> },
    #[serde(rename = "pick_source_dir")]
    PickSourceDir,
    #[serde(rename = "pick_output_file")]
    PickOutputFile,
    #[serde(rename = "create_archive")]
    CreateArchive {
        src_dir: String,
        out_path: String,
        codec: String,
        compress_level: u32,
        password: Option<String>,
    },
}

#[derive(Serialize)]
struct ArchivePayload {
    path: String,
    name: String,
    metadata: MetadataPayload,
    entries: Vec<CentralDirEntry>,
    flags: FlagsPayload,
}

#[derive(Serialize)]
struct MetadataPayload {
    created_at: i64,
    creator: String,
    comment: Option<String>,
    total_uncompressed_bytes: u64,
    total_compressed_bytes: u64,
    total_files: u32,
}

#[derive(Serialize)]
struct FlagsPayload {
    compressed: bool,
    encrypted: bool,
    signed: bool,
}

#[derive(Serialize)]
struct BenchResultPayload {
    deflate_comp_mbps: f64,
    deflate_decomp_mbps: f64,
    deflate_ratio: f64,
    lz4_comp_mbps: f64,
    lz4_decomp_mbps: f64,
    lz4_ratio: f64,
}

enum UserEvent {
    Ipc(String),
}

struct AppState {
    current_path: Option<PathBuf>,
    current_archive: Option<VpackArchive>,
    initial_file: Option<PathBuf>,
}

fn main() {
    // Collect CLI argument if present (e.g. vpack gui path/to/archive.vpack)
    let initial_file = std::env::args().nth(1).map(PathBuf::from);

    let state = Arc::new(Mutex::new(AppState {
        current_path: None,
        current_archive: None,
        initial_file,
    }));

    let event_loop = EventLoopBuilder::<UserEvent>::with_user_event().build();
    let proxy = event_loop.create_proxy();

    let window = WindowBuilder::new()
        .with_title("VPack Archiver 2.0 — Universal Archive Manager")
        .with_inner_size(LogicalSize::new(1060.0, 680.0))
        .with_min_inner_size(LogicalSize::new(800.0, 500.0))
        .build(&event_loop)
        .expect("failed to create tao window");

    let webview = WebViewBuilder::new()
        .with_html(html::INDEX_HTML)
        .with_ipc_handler(move |req| {
            let _ = proxy.send_event(UserEvent::Ipc(req.body().clone()));
        })
        .build(&window)
        .expect("failed to initialize WebView2");

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::UserEvent(UserEvent::Ipc(msg)) => {
                handle_ipc_message(&msg, &webview, &state);
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => (),
        }
    });
}

fn handle_ipc_message(msg: &str, webview: &wry::WebView, state_lock: &Arc<Mutex<AppState>>) {
    let cmd: IpcCommand = match serde_json::from_str(msg) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Invalid IPC message: {} ({})", msg, e);
            return;
        }
    };

    match cmd {
        IpcCommand::UiReady => {
            let mut state = state_lock.lock().unwrap();
            if let Some(path) = state.initial_file.take() {
                if path.exists() {
                    load_and_dispatch_archive(&path, webview, &mut state);
                }
            }
        }
        IpcCommand::OpenDialog => {
            if let Some(file) = rfd::FileDialog::new()
                .add_filter("VPack Archive (*.vpack)", &["vpack"])
                .add_filter("All Files (*.*)", &["*"])
                .pick_file()
            {
                let mut state = state_lock.lock().unwrap();
                load_and_dispatch_archive(&file, webview, &mut state);
            }
        }
        IpcCommand::OpenPath { path } => {
            let p = PathBuf::from(path);
            if p.exists() {
                let mut state = state_lock.lock().unwrap();
                load_and_dispatch_archive(&p, webview, &mut state);
            } else {
                emit_error(webview, "Specified file does not exist.");
            }
        }
        IpcCommand::ExtractDialog { password } => {
            let state = state_lock.lock().unwrap();
            if let Some(ref archive) = state.current_archive {
                if let Some(dest) = rfd::FileDialog::new().pick_folder() {
                    let pwd_filter = password.filter(|p| !p.is_empty());
                    match archive.extract_all(&dest, pwd_filter.as_deref()) {
                        Ok(count) => {
                            emit_success(
                                webview,
                                &format!(
                                    "Successfully extracted {} file(s) to {}",
                                    count,
                                    dest.display()
                                ),
                            );
                        }
                        Err(e) => {
                            emit_error(webview, &format!("Extraction failed: {}", e));
                        }
                    }
                }
            } else {
                emit_error(webview, "No archive currently loaded.");
            }
        }
        IpcCommand::TestIntegrity { password } => {
            let state = state_lock.lock().unwrap();
            if let Some(ref archive) = state.current_archive {
                let pwd_filter = password.filter(|p| !p.is_empty());
                match archive.test_integrity(pwd_filter.as_deref()) {
                    Ok(count) => {
                        let mut msg = format!(
                            "Integrity Verified: all {} entries matched CRC-32 checksums perfectly!",
                            count
                        );
                        if (archive.flags & FLAG_SIGNED) != 0 || archive.signature.is_some() {
                            match vpack_core::verify::verify_signature(archive, None) {
                                Ok(true) => msg.push_str(" (Ed25519 signature authentic!)"),
                                Ok(false) => msg.push_str(" (⚠ Ed25519 signature INVALID!)"),
                                Err(e) => msg.push_str(&format!(" (⚠ Signature error: {})", e)),
                            }
                        }
                        emit_success(webview, &msg);
                    }
                    Err(e) => {
                        emit_error(webview, &format!("Integrity check failed: {}", e));
                    }
                }
            } else {
                emit_error(webview, "No archive currently loaded.");
            }
        }
        IpcCommand::RunBenchmark { size_mb } => {
            let mb = size_mb.unwrap_or(16);
            match run_benchmark_computation(mb) {
                Ok(results) => {
                    let json = serde_json::to_string(&serde_json::json!({
                        "type": "BENCHMARK_RESULT",
                        "results": results
                    }))
                    .unwrap();
                    let js = format!("window.onVpackEvent({});", json);
                    let _ = webview.evaluate_script(&js);
                }
                Err(e) => {
                    emit_error(webview, &format!("Benchmark failed: {}", e));
                }
            }
        }
        IpcCommand::PickSourceDir => {
            if let Some(folder) = rfd::FileDialog::new().pick_folder() {
                let json = serde_json::to_string(&serde_json::json!({
                    "type": "SOURCE_DIR_PICKED",
                    "path": folder.to_string_lossy()
                }))
                .unwrap();
                let js = format!("window.onVpackEvent({});", json);
                let _ = webview.evaluate_script(&js);
            }
        }
        IpcCommand::PickOutputFile => {
            if let Some(file) = rfd::FileDialog::new()
                .add_filter("VPack Archive (*.vpack)", &["vpack"])
                .save_file()
            {
                let json = serde_json::to_string(&serde_json::json!({
                    "type": "OUTPUT_FILE_PICKED",
                    "path": file.to_string_lossy()
                }))
                .unwrap();
                let js = format!("window.onVpackEvent({});", json);
                let _ = webview.evaluate_script(&js);
            }
        }
        IpcCommand::CreateArchive {
            src_dir,
            out_path,
            codec,
            compress_level,
            password,
        } => {
            let src = PathBuf::from(&src_dir);
            let out = PathBuf::from(&out_path);

            if !src.exists() {
                emit_error(webview, "Source directory does not exist.");
                return;
            }

            match collect_directory_entries(&src, &src) {
                Ok(entries) => {
                    let pwd_filter = password.filter(|p| !p.trim().is_empty());
                    let pwd_ref = pwd_filter.as_deref();
                    match VpackArchive::create_archive(
                        &out,
                        entries,
                        compress_level,
                        &codec,
                        pwd_ref,
                        Some("Created with VPack Archiver 2.0 Modern Desktop Suite".into()),
                        None,
                    ) {
                        Ok(()) => {
                            emit_success(
                                webview,
                                &format!("Archive created successfully: {}", out.display()),
                            );
                            // Auto open
                            let mut state = state_lock.lock().unwrap();
                            load_and_dispatch_archive(&out, webview, &mut state);
                        }
                        Err(e) => {
                            emit_error(webview, &format!("Failed to create archive: {}", e));
                        }
                    }
                }
                Err(e) => {
                    emit_error(webview, &format!("Failed to read source directory: {}", e));
                }
            }
        }
    }
}

fn load_and_dispatch_archive(
    path: &Path,
    webview: &wry::WebView,
    state: &mut std::sync::MutexGuard<'_, AppState>,
) {
    match VpackArchive::open(path) {
        Ok(archive) => {
            let name = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "archive.vpack".to_string());

            let payload = ArchivePayload {
                path: path.to_string_lossy().to_string(),
                name,
                metadata: MetadataPayload {
                    created_at: archive.metadata.created_at,
                    creator: archive.metadata.creator.clone(),
                    comment: archive.metadata.comment.clone(),
                    total_uncompressed_bytes: archive.metadata.total_uncompressed_bytes,
                    total_compressed_bytes: archive.metadata.total_compressed_bytes,
                    total_files: archive.metadata.total_files,
                },
                entries: archive.central_directory.clone(),
                flags: FlagsPayload {
                    compressed: (archive.flags & FLAG_COMPRESSED) != 0,
                    encrypted: (archive.flags & FLAG_ENCRYPTED) != 0,
                    signed: (archive.flags & FLAG_SIGNED) != 0,
                },
            };

            state.current_path = Some(path.to_path_buf());
            state.current_archive = Some(archive);

            let json = serde_json::to_string(&serde_json::json!({
                "type": "ARCHIVE_LOADED",
                "archive": payload
            }))
            .unwrap();
            let js = format!("window.onVpackEvent({});", json);
            let _ = webview.evaluate_script(&js);
        }
        Err(e) => {
            emit_error(webview, &format!("Failed to open archive: {}", e));
        }
    }
}

fn emit_success(webview: &wry::WebView, message: &str) {
    let json = serde_json::to_string(&serde_json::json!({
        "type": "OPERATION_SUCCESS",
        "message": message
    }))
    .unwrap();
    let js = format!("window.onVpackEvent({});", json);
    let _ = webview.evaluate_script(&js);
}

fn emit_error(webview: &wry::WebView, message: &str) {
    let json = serde_json::to_string(&serde_json::json!({
        "type": "OPERATION_ERROR",
        "message": message
    }))
    .unwrap();
    let js = format!("window.onVpackEvent({});", json);
    let _ = webview.evaluate_script(&js);
}

fn run_benchmark_computation(size_mb: usize) -> anyhow::Result<BenchResultPayload> {
    let total_bytes = size_mb * 1024 * 1024;
    let mut data = Vec::with_capacity(total_bytes);
    for i in 0..total_bytes {
        data.push(((i * 31 + (i >> 3)) ^ (i >> 7)) as u8);
    }

    // Deflate
    let t = Instant::now();
    let mut enc = flate2::write::DeflateEncoder::new(Vec::new(), flate2::Compression::new(6));
    enc.write_all(&data)?;
    let compressed_def = enc.finish()?;
    let def_c_s = t.elapsed().as_secs_f64();
    let deflate_comp_mbps = size_mb as f64 / def_c_s;
    let deflate_ratio = (1.0 - compressed_def.len() as f64 / data.len() as f64) * 100.0;

    let t = Instant::now();
    let mut dec = flate2::read::DeflateDecoder::new(&compressed_def[..]);
    let mut decomp = Vec::with_capacity(data.len());
    dec.read_to_end(&mut decomp)?;
    let deflate_decomp_mbps = size_mb as f64 / t.elapsed().as_secs_f64();

    // LZ4
    let t = Instant::now();
    let compressed_lz4 = lz4_flex::compress_prepend_size(&data);
    let lz4_c_s = t.elapsed().as_secs_f64();
    let lz4_comp_mbps = size_mb as f64 / lz4_c_s;
    let lz4_ratio = (1.0 - compressed_lz4.len() as f64 / data.len() as f64) * 100.0;

    let t = Instant::now();
    let _ = lz4_flex::decompress_size_prepended(&compressed_lz4)?;
    let lz4_decomp_mbps = size_mb as f64 / t.elapsed().as_secs_f64();

    Ok(BenchResultPayload {
        deflate_comp_mbps,
        deflate_decomp_mbps,
        deflate_ratio: deflate_ratio.max(0.0),
        lz4_comp_mbps,
        lz4_decomp_mbps,
        lz4_ratio: lz4_ratio.max(0.0),
    })
}

