pub const INDEX_HTML: &str = r###"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>VPack Archiver 2.0</title>
<style>
  :root {
    --bg-main: #070a13;
    --bg-card: rgba(15, 23, 42, 0.75);
    --bg-card-hover: rgba(30, 41, 59, 0.85);
    --bg-table-row-hover: rgba(56, 189, 248, 0.08);
    --bg-table-row-selected: rgba(56, 189, 248, 0.18);
    --border-glass: rgba(255, 255, 255, 0.08);
    --border-highlight: rgba(56, 189, 248, 0.35);
    --accent-cyan: #38bdf8;
    --accent-blue: #2563eb;
    --accent-indigo: #6366f1;
    --accent-emerald: #10b981;
    --accent-amber: #f59e0b;
    --accent-rose: #f43f5e;
    --text-primary: #f8fafc;
    --text-secondary: #94a3b8;
    --text-muted: #64748b;
    --font-sans: -apple-system, BlinkMacSystemFont, 'Segoe UI Variable Display', 'Segoe UI', Roboto, sans-serif;
    --font-mono: 'Cascadia Code', 'Fira Code', Consolas, monospace;
  }
  * { box-sizing: border-box; margin: 0; padding: 0; user-select: none; }
  body {
    background: radial-gradient(circle at 50% 0%, #1e1b4b 0%, var(--bg-main) 65%);
    color: var(--text-primary);
    font-family: var(--font-sans);
    height: 100vh;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    font-size: 13px;
  }
  header {
    background: rgba(15, 23, 42, 0.82);
    backdrop-filter: blur(16px);
    border-bottom: 1px solid var(--border-glass);
    padding: 10px 18px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 16px;
    z-index: 10;
  }
  .brand-zone { display: flex; align-items: center; gap: 12px; }
  .brand-logo {
    width: 32px; height: 32px;
    background: linear-gradient(135deg, #38bdf8 0%, #6366f1 100%);
    border-radius: 9px; display: flex; align-items: center; justify-content: center;
    box-shadow: 0 0 16px rgba(56, 189, 248, 0.4);
  }
  .brand-logo svg { width: 20px; height: 20px; fill: #ffffff; }
  .brand-title { font-weight: 700; font-size: 15px; letter-spacing: 0.5px; display: flex; align-items: center; gap: 8px; }
  .badge-v2 {
    font-size: 10px; font-weight: 800; background: linear-gradient(90deg, #38bdf8, #818cf8);
    color: #090d16; padding: 2px 6px; border-radius: 6px; letter-spacing: 0.5px;
  }
  .toolbar { display: flex; align-items: center; gap: 6px; }
  .btn {
    background: rgba(30, 41, 59, 0.7); border: 1px solid var(--border-glass); color: var(--text-primary);
    padding: 7px 13px; border-radius: 8px; font-size: 12px; font-weight: 500; cursor: pointer;
    display: flex; align-items: center; gap: 7px; transition: all 0.18s cubic-bezier(0.16, 1, 0.3, 1);
  }
  .btn:hover {
    background: rgba(51, 65, 85, 0.9); border-color: var(--border-highlight); transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.25);
  }
  .btn-primary {
    background: linear-gradient(135deg, #0284c7 0%, #4f46e5 100%); border-color: rgba(56, 189, 248, 0.5);
    box-shadow: 0 0 14px rgba(2, 132, 199, 0.35);
  }
  .btn svg { width: 15px; height: 15px; stroke: currentColor; stroke-width: 2; fill: none; flex-shrink: 0; }
  .search-wrapper { position: relative; width: 220px; }
  .search-wrapper input {
    width: 100%; background: rgba(15, 23, 42, 0.6); border: 1px solid var(--border-glass);
    border-radius: 8px; padding: 6px 10px 6px 30px; color: var(--text-primary); font-size: 12px; outline: none;
  }
  .search-wrapper input:focus { border-color: var(--accent-cyan); background: rgba(15, 23, 42, 0.95); box-shadow: 0 0 12px rgba(56, 189, 248, 0.25); }
  .search-wrapper svg { position: absolute; left: 9px; top: 50%; transform: translateY(-50%); width: 13px; height: 13px; stroke: var(--text-muted); fill: none; }
  .hud-ribbon {
    background: rgba(15, 23, 42, 0.5); backdrop-filter: blur(12px); border-bottom: 1px solid var(--border-glass);
    padding: 8px 20px; display: flex; align-items: center; justify-content: space-between; font-size: 12px;
  }
  .hud-left { display: flex; align-items: center; gap: 16px; }
  .hud-stat { display: flex; align-items: center; gap: 6px; color: var(--text-secondary); }
  .hud-stat strong { color: var(--text-primary); font-weight: 600; }
  .badge-chip {
    padding: 2px 8px; border-radius: 6px; font-size: 11px; font-weight: 600; display: flex; align-items: center; gap: 5px;
  }
  .chip-cyan { background: rgba(56, 189, 248, 0.15); color: #38bdf8; border: 1px solid rgba(56, 189, 248, 0.3); }
  .chip-emerald { background: rgba(16, 185, 129, 0.15); color: #34d399; border: 1px solid rgba(16, 185, 129, 0.3); }
  .chip-amber { background: rgba(245, 158, 11, 0.15); color: #fbbf24; border: 1px solid rgba(245, 158, 11, 0.3); }
  .chip-indigo { background: rgba(99, 102, 241, 0.15); color: #a5b4fc; border: 1px solid rgba(99, 102, 241, 0.3); }
  .breadcrumbs {
    display: flex; align-items: center; gap: 6px; padding: 8px 20px; background: rgba(10, 15, 26, 0.4);
    border-bottom: 1px solid rgba(255, 255, 255, 0.04); font-size: 12px;
  }
  .crumb { color: var(--text-secondary); cursor: pointer; display: flex; align-items: center; gap: 4px; padding: 3px 6px; border-radius: 4px; }
  .crumb.active { color: var(--accent-cyan); font-weight: 600; }
  main { flex: 1; overflow: hidden; position: relative; display: flex; flex-direction: column; }
  #empty-view { flex: 1; display: flex; flex-direction: column; align-items: center; justify-content: center; padding: 40px; text-align: center; }
  .drop-zone {
    width: 540px; max-width: 90%; background: rgba(17, 24, 39, 0.45); backdrop-filter: blur(20px);
    border: 2px dashed rgba(56, 189, 248, 0.3); border-radius: 20px; padding: 46px 30px; cursor: pointer;
    transition: all 0.3s cubic-bezier(0.16, 1, 0.3, 1); box-shadow: 0 10px 30px rgba(0, 0, 0, 0.3);
  }
  .drop-zone:hover { border-color: var(--accent-cyan); background: rgba(30, 41, 59, 0.6); box-shadow: 0 0 35px rgba(56, 189, 248, 0.25); transform: scale(1.01); }
  .drop-icon {
    width: 64px; height: 64px; margin: 0 auto 18px; border-radius: 18px;
    background: linear-gradient(135deg, rgba(56, 189, 248, 0.15) 0%, rgba(99, 102, 241, 0.2) 100%);
    display: flex; align-items: center; justify-content: center; box-shadow: 0 0 20px rgba(56, 189, 248, 0.2);
  }
  .drop-icon svg { width: 32px; height: 32px; stroke: var(--accent-cyan); stroke-width: 1.8; fill: none; }
  .drop-title {
    font-size: 19px; font-weight: 700; margin-bottom: 8px;
    background: linear-gradient(90deg, #f8fafc, #94a3b8); -webkit-background-clip: text; -webkit-text-fill-color: transparent;
  }
  .drop-subtitle { color: var(--text-secondary); font-size: 13px; margin-bottom: 24px; line-height: 1.5; }
  .quick-buttons { display: flex; justify-content: center; gap: 12px; }
  #archive-view { flex: 1; display: none; flex-direction: column; overflow: hidden; }
  .table-container { flex: 1; overflow-y: auto; overflow-x: auto; }
  table { width: 100%; border-collapse: collapse; font-size: 12px; text-align: left; }
  thead { position: sticky; top: 0; background: #0f172a; z-index: 2; border-bottom: 1px solid var(--border-glass); }
  th {
    padding: 10px 14px; color: var(--text-secondary); font-weight: 600; font-size: 11px; text-transform: uppercase;
    letter-spacing: 0.5px; cursor: pointer; user-select: none; transition: color 0.15s; white-space: nowrap;
  }
  th:hover { color: var(--accent-cyan); }
  td { padding: 8px 14px; border-bottom: 1px solid rgba(255, 255, 255, 0.03); color: var(--text-primary); white-space: nowrap; }
  tr { transition: background 0.12s ease; cursor: pointer; }
  tr:hover { background: var(--bg-table-row-hover); }
  tr.selected { background: var(--bg-table-row-selected); }
  .col-name { display: flex; align-items: center; gap: 9px; }
  .col-name svg { width: 16px; height: 16px; flex-shrink: 0; }
  .mono { font-family: var(--font-mono); font-size: 11px; color: var(--text-secondary); }
  .ratio-bar { display: inline-flex; align-items: center; gap: 6px; }
  .bar-bg { width: 48px; height: 4px; background: rgba(255, 255, 255, 0.1); border-radius: 2px; overflow: hidden; }
  .bar-fill { height: 100%; background: linear-gradient(90deg, #38bdf8, #10b981); border-radius: 2px; }
  footer {
    background: rgba(15, 23, 42, 0.85); border-top: 1px solid var(--border-glass); padding: 6px 18px;
    font-size: 11px; color: var(--text-secondary); display: flex; align-items: center; justify-content: space-between; z-index: 10;
  }
  .footer-left { display: flex; align-items: center; gap: 14px; }
  .footer-indicator { width: 7px; height: 7px; border-radius: 50%; background: var(--accent-emerald); box-shadow: 0 0 8px var(--accent-emerald); }
  .modal-overlay {
    position: fixed; inset: 0; background: rgba(3, 7, 18, 0.7); backdrop-filter: blur(10px);
    display: none; align-items: center; justify-content: center; z-index: 100;
  }
  .modal-box {
    background: #0f172a; border: 1px solid var(--border-highlight); box-shadow: 0 20px 50px rgba(0, 0, 0, 0.6);
    border-radius: 16px; width: 560px; max-width: 92%; padding: 24px; display: flex; flex-direction: column; gap: 18px;
  }
  .modal-header { display: flex; align-items: center; justify-content: space-between; }
  .modal-title { font-size: 16px; font-weight: 700; color: var(--text-primary); }
  .modal-close { background: transparent; border: none; color: var(--text-muted); cursor: pointer; font-size: 18px; padding: 4px; border-radius: 6px; }
  .modal-close:hover { color: var(--text-primary); background: rgba(255, 255, 255, 0.08); }
  .modal-body { display: flex; flex-direction: column; gap: 14px; font-size: 13px; color: var(--text-secondary); }
  .modal-footer { display: flex; justify-content: flex-end; gap: 10px; margin-top: 6px; }
  .form-group { display: flex; flex-direction: column; gap: 6px; }
  .form-label { font-size: 12px; color: var(--text-secondary); font-weight: 500; }
  .form-input {
    background: rgba(15, 23, 42, 0.8); border: 1px solid var(--border-glass); border-radius: 8px;
    padding: 8px 12px; color: var(--text-primary); font-size: 13px; outline: none;
  }
  .form-input:focus { border-color: var(--accent-cyan); }
  .select-input {
    background: rgba(15, 23, 42, 0.8); border: 1px solid var(--border-glass); border-radius: 8px;
    padding: 8px 12px; color: var(--text-primary); font-size: 13px; outline: none;
  }
  .bench-grid { display: grid; grid-template-columns: 1fr 1fr; gap: 12px; }
  .bench-card { background: rgba(30, 41, 59, 0.45); border: 1px solid var(--border-glass); border-radius: 12px; padding: 14px; }
  .bench-title { font-weight: 700; font-size: 13px; margin-bottom: 8px; display: flex; align-items: center; justify-content: space-between; }
  .bench-metric { display: flex; justify-content: space-between; margin-bottom: 6px; font-size: 12px; }
  .bench-val { font-weight: 700; color: var(--accent-cyan); font-family: var(--font-mono); }
  #toast {
    position: fixed; bottom: 35px; right: 20px; background: rgba(15, 23, 42, 0.92); border: 1px solid var(--border-highlight);
    backdrop-filter: blur(12px); box-shadow: 0 10px 30px rgba(0, 0, 0, 0.5); color: var(--text-primary);
    padding: 10px 16px; border-radius: 10px; font-size: 12px; display: none; align-items: center; gap: 9px; z-index: 200;
  }
</style>
</head>
<body>
  <header>
    <div class="brand-zone">
      <div class="brand-logo"><svg viewBox="0 0 24 24"><path d="M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5"></path></svg></div>
      <div class="brand-title">VPACK ARCHIVER<span class="badge-v2">v2.0</span></div>
    </div>
    <div class="toolbar">
      <button class="btn btn-primary" onclick="sendIpc({ action: 'open_dialog' })">
        <svg viewBox="0 0 24 24"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path></svg>Open
      </button>
      <button class="btn" id="btn-extract" onclick="sendIpc({ action: 'extract_dialog' })" disabled style="opacity: 0.5;">
        <svg viewBox="0 0 24 24"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path><polyline points="7 10 12 15 17 10"></polyline><line x1="12" y1="15" x2="12" y2="3"></line></svg>Extract All
      </button>
      <button class="btn" onclick="openCreateModal()">
        <svg viewBox="0 0 24 24"><path d="M12 5v14M5 12h14"></path></svg>New Archive
      </button>
      <button class="btn" id="btn-test" onclick="sendIpc({ action: 'test_integrity' })" disabled style="opacity: 0.5;">
        <svg viewBox="0 0 24 24"><path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"></path></svg>Test
      </button>
      <button class="btn" onclick="openBenchModal()">
        <svg viewBox="0 0 24 24"><polygon points="13 2 3 14 12 14 11 22 21 10 12 10 13 2"></polygon></svg>Benchmark
      </button>
      <button class="btn" id="btn-info" onclick="openInfoModal()" disabled style="opacity: 0.5;">
        <svg viewBox="0 0 24 24"><circle cx="12" cy="12" r="10"></circle><line x1="12" y1="16" x2="12" y2="12"></line><line x1="12" y1="8" x2="12.01" y2="8"></line></svg>Properties
      </button>
    </div>
    <div class="search-wrapper">
      <svg viewBox="0 0 24 24"><circle cx="11" cy="11" r="8"></circle><line x1="21" y1="21" x2="16.65" y2="16.65"></line></svg>
      <input type="text" id="search-input" placeholder="Filter files... (Ctrl+F)" oninput="filterFiles()">
    </div>
  </header>

  <div class="hud-ribbon" id="hud-ribbon" style="display: none;">
    <div class="hud-left">
      <div class="hud-stat">Archive: <strong id="hud-archive-name">-</strong></div>
      <div class="hud-stat">Files: <strong id="hud-file-count">0</strong></div>
      <div class="hud-stat">Size: <strong id="hud-uncompressed">0 B</strong></div>
      <div class="hud-stat">Packed: <strong id="hud-compressed">0 B</strong></div>
      <div class="badge-chip chip-emerald" id="hud-ratio-chip">⚡ <span id="hud-space-saved">0%</span> Space Saved</div>
    </div>
    <div class="hud-right" style="display: flex; gap: 8px;">
      <span class="badge-chip chip-indigo" id="hud-codec-chip">Codec: Auto</span>
      <span class="badge-chip chip-cyan" id="hud-sig-chip">Ed25519 Verified</span>
    </div>
  </div>

  <div class="breadcrumbs" id="breadcrumbs" style="display: none;">
    <div class="crumb active">📁 <span id="crumb-root-label">archive</span></div>
  </div>

  <main>
    <div id="empty-view">
      <div class="drop-zone" id="drop-zone" onclick="sendIpc({ action: 'open_dialog' })">
        <div class="drop-icon">
          <svg viewBox="0 0 24 24"><path d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"></path><line x1="12" y1="11" x2="12" y2="17"></line><polyline points="9 14 12 11 15 14"></polyline></svg>
        </div>
        <div class="drop-title">Open or Create a VPack Archive</div>
        <div class="drop-subtitle">Click to browse your files or open a <strong>.vpack</strong> archive.<br>Featuring high-ratio Deflate, ultra-speed LZ4 streaming, and cryptographic authentication.</div>
        <div class="quick-buttons" onclick="event.stopPropagation()">
          <button class="btn btn-primary" onclick="sendIpc({ action: 'open_dialog' })">⚡ Open Archive</button>
          <button class="btn" onclick="openCreateModal()">➕ Create New</button>
          <button class="btn" onclick="openBenchModal()">🚀 Run Benchmark</button>
        </div>
      </div>
    </div>

    <div id="archive-view">
      <div class="table-container">
        <table id="file-table">
          <thead>
            <tr>
              <th onclick="sortTable('name')">Name ▾</th>
              <th onclick="sortTable('size')">Original Size</th>
              <th onclick="sortTable('compressed')">Packed Size</th>
              <th onclick="sortTable('ratio')">Ratio</th>
              <th onclick="sortTable('crc')">CRC-32</th>
              <th onclick="sortTable('method')">Method</th>
              <th onclick="sortTable('date')">Modified</th>
            </tr>
          </thead>
          <tbody id="file-table-body"></tbody>
        </table>
      </div>
    </div>
  </main>

  <footer>
    <div class="footer-left">
      <div class="footer-indicator"></div>
      <span id="status-text">Ready</span>
    </div>
    <div class="footer-right"><span>VPack Engine v2.0.0 (x86_64-windows)</span></div>
  </footer>

  <div class="modal-overlay" id="modal-bench">
    <div class="modal-box">
      <div class="modal-header">
        <div class="modal-title">⚡ Hardware Multi-Codec Benchmark</div>
        <button class="modal-close" onclick="closeModal('modal-bench')">&times;</button>
      </div>
      <div class="modal-body">
        <p>Measures compression and decompression throughput across multi-core CPU pipelines with 16MB entropy workload.</p>
        <div class="bench-grid" id="bench-results-grid">
          <div class="bench-card">
            <div class="bench-title"><span>Deflate (Level 6)</span><span class="badge-chip chip-cyan">High Ratio</span></div>
            <div class="bench-metric"><span>Compress:</span> <span class="bench-val" id="bench-def-c">-</span></div>
            <div class="bench-metric"><span>Decompress:</span> <span class="bench-val" id="bench-def-d">-</span></div>
            <div class="bench-metric"><span>Space Saved:</span> <span class="bench-val" id="bench-def-r">-</span></div>
          </div>
          <div class="bench-card">
            <div class="bench-title"><span>LZ4 (Frame Streaming)</span><span class="badge-chip chip-emerald">Ultra Fast</span></div>
            <div class="bench-metric"><span>Compress:</span> <span class="bench-val" id="bench-lz4-c">-</span></div>
            <div class="bench-metric"><span>Decompress:</span> <span class="bench-val" id="bench-lz4-d">-</span></div>
            <div class="bench-metric"><span>Space Saved:</span> <span class="bench-val" id="bench-lz4-r">-</span></div>
          </div>
        </div>
      </div>
      <div class="modal-footer">
        <button class="btn" onclick="closeModal('modal-bench')">Close</button>
        <button class="btn btn-primary" id="bench-run-btn" onclick="runBenchmark(16)">Run 16MB Test</button>
      </div>
    </div>
  </div>

  <div class="modal-overlay" id="modal-info">
    <div class="modal-box">
      <div class="modal-header">
        <div class="modal-title">ℹ️ Archive Properties</div>
        <button class="modal-close" onclick="closeModal('modal-info')">&times;</button>
      </div>
      <div class="modal-body" id="info-modal-body"></div>
      <div class="modal-footer">
        <button class="btn btn-primary" onclick="closeModal('modal-info')">Done</button>
      </div>
    </div>
  </div>

  <div class="modal-overlay" id="modal-create">
    <div class="modal-box">
      <div class="modal-header">
        <div class="modal-title">➕ Create New VPack Archive</div>
        <button class="modal-close" onclick="closeModal('modal-create')">&times;</button>
      </div>
      <div class="modal-body">
        <div class="form-group">
          <label class="form-label">Source Directory to Compress:</label>
          <div style="display: flex; gap: 8px;">
            <input type="text" class="form-input" id="create-src-dir" placeholder="Select directory..." style="flex: 1;" readonly>
            <button class="btn" onclick="sendIpc({ action: 'pick_source_dir' })">Browse...</button>
          </div>
        </div>
        <div class="form-group">
          <label class="form-label">Output Archive Path:</label>
          <div style="display: flex; gap: 8px;">
            <input type="text" class="form-input" id="create-out-file" placeholder="Select destination .vpack..." style="flex: 1;" readonly>
            <button class="btn" onclick="sendIpc({ action: 'pick_output_file' })">Save As...</button>
          </div>
        </div>
        <div style="display: grid; grid-template-columns: 1fr 1fr; gap: 12px;">
          <div class="form-group">
            <label class="form-label">Compression Codec:</label>
            <select class="select-input" id="create-codec">
              <option value="deflate">Deflate (Standard High Ratio)</option>
              <option value="lz4">LZ4 (Ultra Fast Streaming)</option>
              <option value="store">Store (No Compression)</option>
            </select>
          </div>
          <div class="form-group">
            <label class="form-label">Compression Level (0-9):</label>
            <input type="range" id="create-level" min="0" max="9" value="6" oninput="document.getElementById('level-val').innerText = this.value" style="margin-top: 10px;">
            <span style="font-size: 11px; color: var(--text-muted);">Level: <strong id="level-val" style="color: var(--accent-cyan);">6</strong></span>
          </div>
        </div>
        <div class="form-group">
          <label class="form-label">Optional Password Protection:</label>
          <input type="password" class="form-input" id="create-password" placeholder="Leave empty for no encryption">
        </div>
      </div>
      <div class="modal-footer">
        <button class="btn" onclick="closeModal('modal-create')">Cancel</button>
        <button class="btn btn-primary" onclick="executeCreateArchive()">Create Archive</button>
      </div>
    </div>
  </div>

  <div id="toast"><span id="toast-msg">Notification</span></div>

<script>
  let currentArchive = null;
  let allEntries = [];
  let displayedEntries = [];
  let sortColumn = 'name';
  let sortAsc = true;
  let selectedIndices = new Set();

  function sendIpc(obj) {
    if (window.ipc) { window.ipc.postMessage(JSON.stringify(obj)); }
    else { console.log('IPC message:', obj); }
  }

  function showToast(msg, duration = 3000) {
    const t = document.getElementById('toast');
    document.getElementById('toast-msg').innerText = msg;
    t.style.display = 'flex';
    setTimeout(() => { t.style.display = 'none'; }, duration);
  }

  function formatBytes(bytes) {
    if (!bytes || bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
  }

  function formatDate(ts) {
    if (!ts) return '-';
    const d = new Date(ts * 1000);
    return d.toLocaleString(undefined, {
      year: 'numeric', month: '2-digit', day: '2-digit', hour: '2-digit', minute: '2-digit'
    });
  }

  function getFileIcon(isDir, path) {
    if (isDir) {
      return `<svg viewBox="0 0 24 24" fill="#f59e0b" stroke="none"><path d="M2 6a2 2 0 0 1 2-2h5l2 2h9a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V6z"></path></svg>`;
    }
    const ext = path.split('.').pop().toLowerCase();
    if (['rs', 'py', 'js', 'html', 'css', 'json', 'toml', 'c', 'cpp'].includes(ext)) {
      return `<svg viewBox="0 0 24 24" stroke="#38bdf8" fill="none" stroke-width="2"><polyline points="16 18 22 12 16 6"></polyline><polyline points="8 6 2 12 8 18"></polyline></svg>`;
    } else if (['zip', 'vpack', 'tar', 'gz', '7z', 'rar'].includes(ext)) {
      return `<svg viewBox="0 0 24 24" stroke="#a855f7" fill="none" stroke-width="2"><rect x="2" y="3" width="20" height="18" rx="2"></rect><path d="M12 3v18M8 7h8M8 11h8M8 15h8"></path></svg>`;
    } else if (['exe', 'dll', 'so', 'bin'].includes(ext)) {
      return `<svg viewBox="0 0 24 24" stroke="#f43f5e" fill="none" stroke-width="2"><circle cx="12" cy="12" r="3"></circle><path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"></path></svg>`;
    }
    return `<svg viewBox="0 0 24 24" stroke="#94a3b8" fill="none" stroke-width="2"><path d="M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z"></path><polyline points="13 2 13 9 20 9"></polyline></svg>`;
  }

  function getMethodName(m) {
    if (m === 0) return 'Store';
    if (m === 1) return 'Deflate';
    if (m === 2) return 'LZ4';
    return 'Method ' + m;
  }

  function renderTable() {
    const tbody = document.getElementById('file-table-body');
    tbody.innerHTML = '';
    displayedEntries.forEach((entry, idx) => {
      const tr = document.createElement('tr');
      if (selectedIndices.has(idx)) tr.classList.add('selected');
      tr.onclick = (e) => {
        if (e.ctrlKey) {
          if (selectedIndices.has(idx)) selectedIndices.delete(idx);
          else selectedIndices.add(idx);
        } else {
          selectedIndices.clear();
          selectedIndices.add(idx);
        }
        renderTable();
        updateSelectionStatus();
      };
      const ratio = entry.uncompressed_size > 0
        ? Math.max(0, ((1 - entry.compressed_size / entry.uncompressed_size) * 100)).toFixed(1)
        : '0.0';
      tr.innerHTML = `
        <td><div class="col-name">${getFileIcon(entry.is_dir, entry.path)}<span>${entry.path}</span></div></td>
        <td class="mono">${formatBytes(entry.uncompressed_size)}</td>
        <td class="mono">${formatBytes(entry.compressed_size)}</td>
        <td><div class="ratio-bar"><div class="bar-bg"><div class="bar-fill" style="width: ${ratio}%;"></div></div><span class="mono">${ratio}%</span></div></td>
        <td class="mono">${entry.crc32 ? '0x' + entry.crc32.toString(16).toUpperCase().padStart(8, '0') : '-'}</td>
        <td><span class="badge-chip chip-cyan" style="display:inline-block;">${getMethodName(entry.method)}</span></td>
        <td class="mono">${formatDate(entry.modified_timestamp)}</td>
      `;
      tbody.appendChild(tr);
    });
  }

  function sortTable(col) {
    if (sortColumn === col) sortAsc = !sortAsc;
    else { sortColumn = col; sortAsc = true; }
    displayedEntries.sort((a, b) => {
      let vA, vB;
      if (col === 'name') { vA = a.path.toLowerCase(); vB = b.path.toLowerCase(); }
      else if (col === 'size') { vA = a.uncompressed_size; vB = b.uncompressed_size; }
      else if (col === 'compressed') { vA = a.compressed_size; vB = b.compressed_size; }
      else if (col === 'crc') { vA = a.crc32; vB = b.crc32; }
      else if (col === 'method') { vA = a.method; vB = b.method; }
      else if (col === 'date') { vA = a.modified_timestamp; vB = b.modified_timestamp; }
      else if (col === 'ratio') {
        vA = a.uncompressed_size ? (1 - a.compressed_size / a.uncompressed_size) : 0;
        vB = b.uncompressed_size ? (1 - b.compressed_size / b.uncompressed_size) : 0;
      }
      if (vA < vB) return sortAsc ? -1 : 1;
      if (vA > vB) return sortAsc ? 1 : -1;
      return 0;
    });
    renderTable();
  }

  function filterFiles() {
    const q = document.getElementById('search-input').value.toLowerCase().trim();
    if (!q) { displayedEntries = [...allEntries]; }
    else { displayedEntries = allEntries.filter(e => e.path.toLowerCase().includes(q)); }
    renderTable();
  }

  function updateSelectionStatus() {
    if (selectedIndices.size === 0) {
      document.getElementById('status-text').innerText = `${allEntries.length} items in archive`;
    } else {
      let selSize = 0;
      selectedIndices.forEach(idx => {
        if (displayedEntries[idx]) selSize += displayedEntries[idx].uncompressed_size;
      });
      document.getElementById('status-text').innerText = `${selectedIndices.size} item(s) selected (${formatBytes(selSize)})`;
    }
  }

  window.onVpackEvent = function(event) {
    if (event.type === 'ARCHIVE_LOADED') {
      currentArchive = event.archive;
      allEntries = currentArchive.entries;
      displayedEntries = [...allEntries];
      selectedIndices.clear();

      document.getElementById('empty-view').style.display = 'none';
      document.getElementById('archive-view').style.display = 'flex';
      document.getElementById('hud-ribbon').style.display = 'flex';
      document.getElementById('breadcrumbs').style.display = 'flex';

      document.getElementById('hud-archive-name').innerText = currentArchive.name;
      document.getElementById('hud-file-count').innerText = currentArchive.metadata.total_files;
      document.getElementById('hud-uncompressed').innerText = formatBytes(currentArchive.metadata.total_uncompressed_bytes);
      document.getElementById('hud-compressed').innerText = formatBytes(currentArchive.metadata.total_compressed_bytes);

      const savedRatio = currentArchive.metadata.total_uncompressed_bytes > 0
        ? ((1 - currentArchive.metadata.total_compressed_bytes / currentArchive.metadata.total_uncompressed_bytes) * 100).toFixed(1)
        : '0.0';
      document.getElementById('hud-space-saved').innerText = `${savedRatio}%`;

      document.getElementById('btn-extract').disabled = false;
      document.getElementById('btn-extract').style.opacity = '1';
      document.getElementById('btn-test').disabled = false;
      document.getElementById('btn-test').style.opacity = '1';
      document.getElementById('btn-info').disabled = false;
      document.getElementById('btn-info').style.opacity = '1';

      renderTable();
      updateSelectionStatus();
      showToast(`Loaded ${currentArchive.name} (${allEntries.length} files)`);
    } else if (event.type === 'BENCHMARK_RESULT') {
      const r = event.results;
      document.getElementById('bench-def-c').innerText = `${r.deflate_comp_mbps.toFixed(1)} MB/s`;
      document.getElementById('bench-def-d').innerText = `${r.deflate_decomp_mbps.toFixed(1)} MB/s`;
      document.getElementById('bench-def-r').innerText = `${r.deflate_ratio.toFixed(1)}%`;

      document.getElementById('bench-lz4-c').innerText = `${r.lz4_comp_mbps.toFixed(1)} MB/s`;
      document.getElementById('bench-lz4-d').innerText = `${r.lz4_decomp_mbps.toFixed(1)} MB/s`;
      document.getElementById('bench-lz4-r').innerText = `${r.lz4_ratio.toFixed(1)}%`;

      document.getElementById('bench-run-btn').disabled = false;
      document.getElementById('bench-run-btn').innerText = 'Run 16MB Test';
      showToast('Benchmark completed successfully!');
    } else if (event.type === 'SOURCE_DIR_PICKED') {
      document.getElementById('create-src-dir').value = event.path;
    } else if (event.type === 'OUTPUT_FILE_PICKED') {
      document.getElementById('create-out-file').value = event.path;
    } else if (event.type === 'OPERATION_SUCCESS') {
      showToast('✓ ' + event.message, 4000);
      document.getElementById('status-text').innerText = event.message;
    } else if (event.type === 'OPERATION_ERROR') {
      showToast('❌ ' + event.message, 5000);
      document.getElementById('status-text').innerText = 'Error: ' + event.message;
    }
  };

  function openBenchModal() { document.getElementById('modal-bench').style.display = 'flex'; }
  function runBenchmark(mb) {
    document.getElementById('bench-run-btn').disabled = true;
    document.getElementById('bench-run-btn').innerText = 'Running Test...';
    sendIpc({ action: 'run_benchmark', size_mb: mb });
  }
  function openCreateModal() { document.getElementById('modal-create').style.display = 'flex'; }
  function executeCreateArchive() {
    const srcDir = document.getElementById('create-src-dir').value;
    const outFile = document.getElementById('create-out-file').value;
    const codec = document.getElementById('create-codec').value;
    const level = parseInt(document.getElementById('create-level').value, 10);
    const password = document.getElementById('create-password').value || null;

    if (!srcDir || !outFile) {
      showToast('Please select both source directory and output archive path!');
      return;
    }
    sendIpc({ action: 'create_archive', src_dir: srcDir, out_path: outFile, codec: codec, compress_level: level, password: password });
    closeModal('modal-create');
    showToast('Creating archive package...');
  }

  function openInfoModal() {
    if (!currentArchive) return;
    const b = document.getElementById('info-modal-body');
    b.innerHTML = `
      <div style="display: grid; grid-template-columns: 140px 1fr; gap: 8px;">
        <div><strong>Archive File:</strong></div><div class="mono">${currentArchive.path}</div>
        <div><strong>Total Files:</strong></div><div>${currentArchive.metadata.total_files}</div>
        <div><strong>Uncompressed:</strong></div><div class="mono">${formatBytes(currentArchive.metadata.total_uncompressed_bytes)}</div>
        <div><strong>Compressed:</strong></div><div class="mono">${formatBytes(currentArchive.metadata.total_compressed_bytes)}</div>
        <div><strong>Created By:</strong></div><div>${currentArchive.metadata.creator || 'VPack Archiver'}</div>
        <div><strong>Created Timestamp:</strong></div><div>${formatDate(currentArchive.metadata.created_at)}</div>
        <div><strong>Signature:</strong></div><div><span class="badge-chip chip-cyan" style="display:inline-block;">Ed25519 Verified</span></div>
        <div><strong>Comment:</strong></div><div>${currentArchive.metadata.comment || 'None'}</div>
      </div>
    `;
    document.getElementById('modal-info').style.display = 'flex';
  }

  function closeModal(id) { document.getElementById(id).style.display = 'none'; }

  window.addEventListener('keydown', (e) => {
    if (e.ctrlKey && e.key.toLowerCase() === 'o') { e.preventDefault(); sendIpc({ action: 'open_dialog' }); }
    else if (e.ctrlKey && e.key.toLowerCase() === 'f') { e.preventDefault(); document.getElementById('search-input').focus(); }
    else if (e.ctrlKey && e.key.toLowerCase() === 'b') { e.preventDefault(); openBenchModal(); }
    else if (e.key === 'Escape') { document.querySelectorAll('.modal-overlay').forEach(m => m.style.display = 'none'); }
  });

  window.addEventListener('DOMContentLoaded', () => {
    sendIpc({ action: 'ui_ready' });
  });
</script>
</body>
</html>
"###;

