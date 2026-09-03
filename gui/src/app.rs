/*!
VPack Archiver 2.0 — GUI Application
WinRAR-style native desktop archive manager built with egui/eframe.
*/

use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use std::time::SystemTime;

use chrono::Local;
use eframe::egui::{self, Color32, RichText, Stroke, Ui, Vec2};
use egui_extras::{Column, TableBuilder};
use vpack_core::archive::{
    collect_directory_entries, ArchiveInputEntry, VpackArchive, FLAG_ENCRYPTED, FLAG_SIGNED,
    METHOD_DEFLATE, METHOD_LZ4,
};

// ──────────────────────────────────────────────────────────────────────────
// Background operation results
// ──────────────────────────────────────────────────────────────────────────

enum WorkResult {
    Opened {
        archive: VpackArchive,
        path: PathBuf,
    },
    Created {
        path: PathBuf,
        count: usize,
    },
    Extracted {
        count: usize,
        dest: String,
    },
    IntegrityPassed {
        count: usize,
    },
    Error(String),
}

// ──────────────────────────────────────────────────────────────────────────
// Dialog sub-structures
// ──────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Default)]
enum Codec {
    #[default]
    Deflate,
    Lz4,
}

impl Codec {
    fn as_str(self) -> &'static str {
        match self {
            Codec::Deflate => "deflate",
            Codec::Lz4 => "lz4",
        }
    }
    fn label(self) -> &'static str {
        match self {
            Codec::Deflate => "Deflate (balanced)",
            Codec::Lz4 => "LZ4 (ultra fast)",
        }
    }
}

#[derive(Default)]
struct AddDialog {
    open: bool,
    /// Files/directories selected to add
    input_paths: Vec<PathBuf>,
    /// Destination .vpack path
    output_path: String,
    codec: Codec,
    level: u32,
    password: String,
    show_password: bool,
}

#[derive(Default)]
struct ExtractDialog {
    open: bool,
    destination: String,
    password: String,
    show_password: bool,
    selected_only: bool,
}

// ──────────────────────────────────────────────────────────────────────────
// Sort state
// ──────────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Default)]
enum SortCol {
    #[default]
    Name,
    OrigSize,
    Packed,
    Ratio,
    Crc32,
    Date,
    Method,
}

// ──────────────────────────────────────────────────────────────────────────
// Main App
// ──────────────────────────────────────────────────────────────────────────

pub struct VpackApp {
    // Archive state
    archive: Option<VpackArchive>,
    archive_path: Option<PathBuf>,
    /// Sorted indices into archive.central_directory
    display_order: Vec<usize>,
    selected: HashSet<usize>,
    sort_col: SortCol,
    sort_asc: bool,

    // Background op channel
    busy: bool,
    busy_msg: String,
    result_rx: Option<mpsc::Receiver<WorkResult>>,

    // Dialogs
    add_dialog: AddDialog,
    extract_dialog: ExtractDialog,

    // Modals
    error_msg: Option<String>,
    info_msg: Option<String>,

    // Status bar
    status: String,
}

impl Default for VpackApp {
    fn default() -> Self {
        Self {
            archive: None,
            archive_path: None,
            display_order: Vec::new(),
            selected: HashSet::new(),
            sort_col: SortCol::Name,
            sort_asc: true,
            busy: false,
            busy_msg: String::new(),
            result_rx: None,
            add_dialog: AddDialog {
                level: 6,
                ..Default::default()
            },
            extract_dialog: Default::default(),
            error_msg: None,
            info_msg: None,
            status: "Ready. Open a .vpack archive or drag one here to begin.".into(),
        }
    }
}

impl VpackApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }

    // ── helpers ──────────────────────────────────────────────────────────

    fn rebuild_display_order(&mut self) {
        let Some(archive) = &self.archive else {
            self.display_order.clear();
            return;
        };
        let mut indices: Vec<usize> = (0..archive.central_directory.len()).collect();
        let col = self.sort_col;
        let asc = self.sort_asc;
        let dir = &archive.central_directory;
        indices.sort_by(|&a, &b| {
            let ord = match col {
                SortCol::Name => dir[a].path.cmp(&dir[b].path),
                SortCol::OrigSize => dir[a].uncompressed_size.cmp(&dir[b].uncompressed_size),
                SortCol::Packed => dir[a].compressed_size.cmp(&dir[b].compressed_size),
                SortCol::Ratio => {
                    let ra = ratio(dir[a].uncompressed_size, dir[a].compressed_size);
                    let rb = ratio(dir[b].uncompressed_size, dir[b].compressed_size);
                    ra.partial_cmp(&rb).unwrap_or(std::cmp::Ordering::Equal)
                }
                SortCol::Crc32 => dir[a].crc32.cmp(&dir[b].crc32),
                SortCol::Date => dir[a].modified_timestamp.cmp(&dir[b].modified_timestamp),
                SortCol::Method => dir[a].method.cmp(&dir[b].method),
            };
            if asc {
                ord
            } else {
                ord.reverse()
            }
        });
        self.display_order = indices;
    }

    fn open_archive_path(&mut self, path: PathBuf, password: Option<String>) {
        let (tx, rx) = mpsc::channel();
        self.result_rx = Some(rx);
        self.busy = true;
        self.busy_msg = format!("Opening {}…", path.display());
        thread::spawn(move || {
            match VpackArchive::open(&path) {
                Ok(archive) => {
                    // If password-protected, try integrity test
                    if let Some(_pwd) = password {
                        // Integrity check — just open succeeds; deeper check on extract
                    }
                    tx.send(WorkResult::Opened { archive, path }).ok();
                }
                Err(e) => {
                    tx.send(WorkResult::Error(e.to_string())).ok();
                }
            }
        });
    }

    fn poll_results(&mut self) {
        let Some(rx) = &self.result_rx else {
            return;
        };
        if let Ok(result) = rx.try_recv() {
            self.busy = false;
            self.result_rx = None;
            match result {
                WorkResult::Opened { archive, path } => {
                    let file_count = archive
                        .central_directory
                        .iter()
                        .filter(|e| !e.is_dir)
                        .count();
                    let total_orig: u64 = archive
                        .central_directory
                        .iter()
                        .map(|e| e.uncompressed_size)
                        .sum();
                    self.status = format!(
                        "Opened: {}   |   {} files   |   {}",
                        path.display(),
                        file_count,
                        fmt_size(total_orig)
                    );
                    self.archive_path = Some(path);
                    self.archive = Some(archive);
                    self.selected.clear();
                    self.rebuild_display_order();
                }
                WorkResult::Created { path, count } => {
                    self.status = format!(
                        "Archive created: {}  ({} items packed)",
                        path.display(),
                        count
                    );
                    // Auto-open the newly created archive
                    let p = path.clone();
                    self.open_archive_path(p, None);
                }
                WorkResult::Extracted { count, dest } => {
                    self.status = format!("Extracted {count} files → {dest}");
                    self.info_msg = Some(format!(
                        "✓ Extracted {count} files successfully.\n\nDestination:\n{dest}"
                    ));
                }
                WorkResult::IntegrityPassed { count } => {
                    self.status = format!("Integrity OK — {count} files verified.");
                    self.info_msg = Some(format!(
                        "✓ All {count} CRC-32 checksums verified.\nArchive is intact."
                    ));
                }
                WorkResult::Error(msg) => {
                    self.status = format!("Error: {msg}");
                    self.error_msg = Some(msg);
                }
            }
        }
    }

    // ── toolbar ──────────────────────────────────────────────────────────

    fn show_toolbar(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.set_height(36.0);
            ui.spacing_mut().button_padding = Vec2::new(10.0, 6.0);

            let btn_open = ui.button(RichText::new("📂  Open").size(13.5));
            if btn_open.clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("VPack Archives", &["vpack"])
                    .add_filter("All Files", &["*"])
                    .pick_file()
                {
                    self.open_archive_path(path, None);
                }
            }

            ui.separator();

            let btn_add = ui.add_enabled(
                !self.busy,
                egui::Button::new(RichText::new("✚  Add").size(13.5)),
            );
            if btn_add.clicked() {
                self.add_dialog.open = true;
            }

            let has_archive = self.archive.is_some();

            let btn_extract = ui.add_enabled(
                has_archive && !self.busy,
                egui::Button::new(RichText::new("📤  Extract").size(13.5)),
            );
            if btn_extract.clicked() {
                if let Some(path) = &self.archive_path {
                    self.extract_dialog.destination =
                        path.parent().unwrap_or(path).to_string_lossy().to_string();
                }
                self.extract_dialog.selected_only = !self.selected.is_empty();
                self.extract_dialog.open = true;
            }

            ui.separator();

            let btn_test = ui.add_enabled(
                has_archive && !self.busy,
                egui::Button::new(RichText::new("🔍  Test").size(13.5)),
            );
            if btn_test.clicked() {
                self.do_test_integrity(None);
            }

            ui.separator();

            let btn_info = ui.add_enabled(
                has_archive,
                egui::Button::new(RichText::new("ℹ  Info").size(13.5)),
            );
            if btn_info.clicked() {
                self.show_archive_info();
            }

            // Busy spinner (right side)
            if self.busy {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.spinner();
                    ui.label(
                        RichText::new(&self.busy_msg)
                            .italics()
                            .color(Color32::LIGHT_GRAY),
                    );
                });
            }
        });
    }

    // ── file table ───────────────────────────────────────────────────────

    fn show_file_table(&mut self, ui: &mut Ui) {
        let Some(archive) = &self.archive else {
            // Empty state
            ui.add_space(100.0);
            ui.vertical_centered(|ui| {
                ui.label(RichText::new("🗁").size(56.0).color(Color32::from_gray(80)));
                ui.add_space(8.0);
                ui.label(
                    RichText::new("No archive open")
                        .size(16.0)
                        .color(Color32::from_gray(140)),
                );
                ui.add_space(4.0);
                ui.label(
                    RichText::new("Click 📂 Open or drag a .vpack file here")
                        .size(12.0)
                        .color(Color32::from_gray(100)),
                );
            });
            return;
        };

        let text_height = egui::TextStyle::Body
            .resolve(ui.style())
            .size
            .max(ui.spacing().interact_size.y);

        // Column header click handler (returns new SortCol if a header was clicked)
        let mut clicked_col: Option<SortCol> = None;

        let col_header = |ui: &mut Ui, label: &str, col: SortCol, current: SortCol, asc: bool| {
            let is_active = col == current;
            let arrow = if is_active {
                if asc {
                    " ▲"
                } else {
                    " ▼"
                }
            } else {
                ""
            };
            ui.strong(format!("{label}{arrow}"))
        };

        let table = TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto().at_least(28.0)) // icon
            .column(Column::remainder().at_least(200.0)) // name
            .column(Column::auto().at_least(85.0)) // orig
            .column(Column::auto().at_least(85.0)) // packed
            .column(Column::auto().at_least(60.0)) // ratio
            .column(Column::auto().at_least(90.0)) // crc32
            .column(Column::auto().at_least(135.0)) // date
            .column(Column::auto().at_least(70.0)) // method
            .sense(egui::Sense::click());

        let asc = self.sort_asc;
        let current_col = self.sort_col;

        let table = table.header(22.0, |mut header| {
            header.col(|ui| {
                ui.strong("");
            });
            header.col(|ui| {
                if col_header(ui, "Name", SortCol::Name, current_col, asc).clicked() {
                    clicked_col = Some(SortCol::Name);
                }
            });
            header.col(|ui| {
                if col_header(ui, "Orig Size", SortCol::OrigSize, current_col, asc).clicked() {
                    clicked_col = Some(SortCol::OrigSize);
                }
            });
            header.col(|ui| {
                if col_header(ui, "Packed", SortCol::Packed, current_col, asc).clicked() {
                    clicked_col = Some(SortCol::Packed);
                }
            });
            header.col(|ui| {
                if col_header(ui, "Ratio", SortCol::Ratio, current_col, asc).clicked() {
                    clicked_col = Some(SortCol::Ratio);
                }
            });
            header.col(|ui| {
                if col_header(ui, "CRC-32", SortCol::Crc32, current_col, asc).clicked() {
                    clicked_col = Some(SortCol::Crc32);
                }
            });
            header.col(|ui| {
                if col_header(ui, "Modified", SortCol::Date, current_col, asc).clicked() {
                    clicked_col = Some(SortCol::Date);
                }
            });
            header.col(|ui| {
                if col_header(ui, "Method", SortCol::Method, current_col, asc).clicked() {
                    clicked_col = Some(SortCol::Method);
                }
            });
        });

        // Snapshot data we need for body (to avoid borrow conflict)
        let order = self.display_order.clone();
        let selected = self.selected.clone();
        let entries: Vec<_> = archive.central_directory.iter().cloned().collect();

        let mut toggle_row: Option<usize> = None;

        table.body(|body| {
            body.rows(text_height + 4.0, order.len(), |mut row| {
                let idx = order[row.index()];
                let entry = &entries[idx];
                let is_selected = selected.contains(&idx);
                let is_dir = entry.is_dir;

                row.set_selected(is_selected);

                row.col(|ui| {
                    ui.label(file_icon(entry.is_dir, &entry.path));
                });
                row.col(|ui| {
                    let name_label = if entry.path.len() > 60 {
                        format!("…{}", &entry.path[entry.path.len() - 57..])
                    } else {
                        entry.path.clone()
                    };
                    let color = if is_dir {
                        Color32::from_rgb(120, 170, 230)
                    } else {
                        Color32::LIGHT_GRAY
                    };
                    if ui
                        .add(
                            egui::Label::new(RichText::new(&name_label).color(color))
                                .sense(egui::Sense::click()),
                        )
                        .clicked()
                    {
                        toggle_row = Some(idx);
                    }
                });
                row.col(|ui| {
                    if is_dir {
                        ui.label(RichText::new("<DIR>").color(Color32::from_gray(120)));
                    } else {
                        ui.label(fmt_size(entry.uncompressed_size));
                    }
                });
                row.col(|ui| {
                    if is_dir {
                        ui.label("-");
                    } else {
                        ui.label(fmt_size(entry.compressed_size));
                    }
                });
                row.col(|ui| {
                    if is_dir {
                        ui.label("-");
                    } else {
                        let r = ratio(entry.uncompressed_size, entry.compressed_size);
                        let color = ratio_color(r);
                        ui.label(RichText::new(format!("{:.1}%", r)).color(color));
                    }
                });
                row.col(|ui| {
                    if is_dir {
                        ui.label("-");
                    } else {
                        ui.label(
                            RichText::new(format!("{:08X}", entry.crc32))
                                .monospace()
                                .color(Color32::from_gray(160)),
                        );
                    }
                });
                row.col(|ui| {
                    let ts = entry.modified_timestamp;
                    let s = if ts > 0 {
                        if let Some(dt) = chrono::DateTime::from_timestamp(ts, 0) {
                            dt.with_timezone(&Local)
                                .format("%Y-%m-%d %H:%M")
                                .to_string()
                        } else {
                            "—".to_string()
                        }
                    } else {
                        "—".to_string()
                    };
                    ui.label(RichText::new(s).color(Color32::from_gray(160)));
                });
                row.col(|ui| {
                    let m = match entry.method {
                        METHOD_DEFLATE => "Deflate",
                        METHOD_LZ4 => "LZ4",
                        0 => "Store",
                        _ => "Unknown",
                    };
                    ui.label(RichText::new(m).color(Color32::from_gray(160)));
                });
            });
        });

        // Apply sort change
        if let Some(col) = clicked_col {
            if self.sort_col == col {
                self.sort_asc = !self.sort_asc;
            } else {
                self.sort_col = col;
                self.sort_asc = true;
            }
            self.rebuild_display_order();
        }

        // Apply selection toggle
        if let Some(idx) = toggle_row {
            if self.selected.contains(&idx) {
                self.selected.remove(&idx);
            } else {
                self.selected.insert(idx);
            }
        }
    }

    // ── status bar ───────────────────────────────────────────────────────

    fn show_status_bar(&self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            // Security badge
            if let Some(archive) = &self.archive {
                if (archive.flags & FLAG_SIGNED) != 0 {
                    ui.label(
                        RichText::new("🔏 Signed")
                            .small()
                            .color(Color32::from_rgb(130, 210, 130)),
                    );
                    ui.separator();
                }
                if (archive.flags & FLAG_ENCRYPTED) != 0 {
                    ui.label(
                        RichText::new("🔒 Encrypted")
                            .small()
                            .color(Color32::from_rgb(210, 180, 80)),
                    );
                    ui.separator();
                }
                // Selection count
                if !self.selected.is_empty() {
                    ui.label(
                        RichText::new(format!("{} selected", self.selected.len()))
                            .small()
                            .color(Color32::from_rgb(100, 160, 230)),
                    );
                    ui.separator();
                }
            }
            ui.label(
                RichText::new(&self.status)
                    .small()
                    .color(Color32::LIGHT_GRAY),
            );
        });
    }

    // ── Add dialog ────────────────────────────────────────────────────────

    fn show_add_dialog(&mut self, ctx: &egui::Context) {
        if !self.add_dialog.open {
            return;
        }

        let mut open = true;
        egui::Window::new("✚  Create / Add to Archive")
            .collapsible(false)
            .resizable(true)
            .min_width(480.0)
            .open(&mut open)
            .show(ctx, |ui| {
                egui::Grid::new("add_grid")
                    .num_columns(2)
                    .spacing([12.0, 8.0])
                    .show(ui, |ui| {
                        ui.strong("Output archive:");
                        ui.horizontal(|ui| {
                            ui.add(
                                egui::TextEdit::singleline(&mut self.add_dialog.output_path)
                                    .desired_width(280.0)
                                    .hint_text("path/to/archive.vpack"),
                            );
                            if ui.button("…").clicked() {
                                if let Some(p) = rfd::FileDialog::new()
                                    .add_filter("VPack Archives", &["vpack"])
                                    .save_file()
                                {
                                    self.add_dialog.output_path = p.to_string_lossy().to_string();
                                }
                            }
                        });
                        ui.end_row();

                        ui.strong("Codec:");
                        egui::ComboBox::from_id_salt("codec_combo")
                            .selected_text(self.add_dialog.codec.label())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.add_dialog.codec,
                                    Codec::Deflate,
                                    Codec::Deflate.label(),
                                );
                                ui.selectable_value(
                                    &mut self.add_dialog.codec,
                                    Codec::Lz4,
                                    Codec::Lz4.label(),
                                );
                            });
                        ui.end_row();

                        let level_enabled = self.add_dialog.codec == Codec::Deflate;
                        ui.add_enabled(level_enabled, egui::Label::new(RichText::new("Level:")));
                        let level_label = match self.add_dialog.level {
                            0 => "Store",
                            1..=3 => "Fast",
                            4..=6 => "Normal",
                            _ => "Max",
                        };
                        ui.horizontal(|ui| {
                            ui.add_enabled(
                                level_enabled,
                                egui::Slider::new(&mut self.add_dialog.level, 0..=9)
                                    .text(level_label),
                            );
                        });
                        ui.end_row();

                        ui.strong("Password:");
                        ui.horizontal(|ui| {
                            ui.add(
                                egui::TextEdit::singleline(&mut self.add_dialog.password)
                                    .password(!self.add_dialog.show_password)
                                    .desired_width(200.0)
                                    .hint_text("optional"),
                            );
                            ui.checkbox(&mut self.add_dialog.show_password, "Show");
                        });
                        ui.end_row();
                    });

                ui.add_space(6.0);
                ui.separator();
                ui.add_space(4.0);

                ui.label(RichText::new("Files & Directories to add:").strong());
                ui.add_space(4.0);

                // File list
                let scroll_height = (self.add_dialog.input_paths.len() as f32 * 22.0)
                    .min(160.0)
                    .max(60.0);
                egui::ScrollArea::vertical()
                    .max_height(scroll_height)
                    .show(ui, |ui| {
                        let mut remove_idx: Option<usize> = None;
                        for (i, p) in self.add_dialog.input_paths.iter().enumerate() {
                            ui.horizontal(|ui| {
                                ui.label(RichText::new("  •  ").color(Color32::from_gray(120)));
                                ui.label(p.to_string_lossy().as_ref());
                                if ui
                                    .small_button(RichText::new("✕").color(Color32::LIGHT_RED))
                                    .clicked()
                                {
                                    remove_idx = Some(i);
                                }
                            });
                        }
                        if let Some(i) = remove_idx {
                            self.add_dialog.input_paths.remove(i);
                        }
                    });

                ui.add_space(4.0);
                ui.horizontal(|ui| {
                    if ui.button("➕ Add Files…").clicked() {
                        if let Some(paths) = rfd::FileDialog::new()
                            .add_filter("All Files", &["*"])
                            .pick_files()
                        {
                            self.add_dialog.input_paths.extend(paths);
                        }
                    }
                    if ui.button("📁 Add Folder…").clicked() {
                        if let Some(path) = rfd::FileDialog::new().pick_folder() {
                            self.add_dialog.input_paths.push(path);
                        }
                    }
                    if !self.add_dialog.input_paths.is_empty() {
                        if ui
                            .small_button(RichText::new("Clear all").color(Color32::from_gray(160)))
                            .clicked()
                        {
                            self.add_dialog.input_paths.clear();
                        }
                    }
                });

                ui.add_space(10.0);
                ui.separator();
                ui.add_space(6.0);

                ui.horizontal(|ui| {
                    let can_create = !self.add_dialog.output_path.is_empty()
                        && !self.add_dialog.input_paths.is_empty();
                    if ui
                        .add_enabled(
                            can_create && !self.busy,
                            egui::Button::new(RichText::new("✚ Create Archive").strong()),
                        )
                        .clicked()
                    {
                        self.do_create_archive();
                        self.add_dialog.open = false;
                    }
                    if ui.button("Cancel").clicked() {
                        self.add_dialog.open = false;
                    }
                });
            });

        if !open {
            self.add_dialog.open = false;
        }
    }

    // ── Extract dialog ────────────────────────────────────────────────────

    fn show_extract_dialog(&mut self, ctx: &egui::Context) {
        if !self.extract_dialog.open {
            return;
        }

        let has_selection = !self.selected.is_empty();
        let mut open = true;

        egui::Window::new("📤  Extract Archive")
            .collapsible(false)
            .resizable(false)
            .min_width(420.0)
            .open(&mut open)
            .show(ctx, |ui| {
                egui::Grid::new("extract_grid")
                    .num_columns(2)
                    .spacing([12.0, 8.0])
                    .show(ui, |ui| {
                        ui.strong("Destination:");
                        ui.horizontal(|ui| {
                            ui.add(
                                egui::TextEdit::singleline(&mut self.extract_dialog.destination)
                                    .desired_width(260.0)
                                    .hint_text("output folder"),
                            );
                            if ui.button("…").clicked() {
                                if let Some(p) = rfd::FileDialog::new().pick_folder() {
                                    self.extract_dialog.destination =
                                        p.to_string_lossy().to_string();
                                }
                            }
                        });
                        ui.end_row();

                        ui.strong("Password:");
                        ui.horizontal(|ui| {
                            ui.add(
                                egui::TextEdit::singleline(&mut self.extract_dialog.password)
                                    .password(!self.extract_dialog.show_password)
                                    .desired_width(180.0)
                                    .hint_text("leave blank if none"),
                            );
                            ui.checkbox(&mut self.extract_dialog.show_password, "Show");
                        });
                        ui.end_row();

                        if has_selection {
                            ui.strong("Scope:");
                            ui.checkbox(
                                &mut self.extract_dialog.selected_only,
                                format!("Extract selected ({} files)", self.selected.len()),
                            );
                            ui.end_row();
                        }
                    });

                ui.add_space(12.0);
                ui.separator();
                ui.add_space(6.0);
                ui.horizontal(|ui| {
                    let can_extract = !self.extract_dialog.destination.is_empty();
                    if ui
                        .add_enabled(
                            can_extract && !self.busy,
                            egui::Button::new(RichText::new("📤 Extract").strong()),
                        )
                        .clicked()
                    {
                        let selected_paths: Option<Vec<String>> =
                            if self.extract_dialog.selected_only && has_selection {
                                let archive = self.archive.as_ref().unwrap();
                                Some(
                                    self.selected
                                        .iter()
                                        .map(|&i| archive.central_directory[i].path.clone())
                                        .collect(),
                                )
                            } else {
                                None
                            };
                        self.do_extract(
                            self.extract_dialog.destination.clone(),
                            if self.extract_dialog.password.is_empty() {
                                None
                            } else {
                                Some(self.extract_dialog.password.clone())
                            },
                            selected_paths,
                        );
                        self.extract_dialog.open = false;
                    }
                    if ui.button("Cancel").clicked() {
                        self.extract_dialog.open = false;
                    }
                });
            });

        if !open {
            self.extract_dialog.open = false;
        }
    }

    // ── Modals ────────────────────────────────────────────────────────────

    fn show_error_modal(&mut self, ctx: &egui::Context) {
        if self.error_msg.is_none() {
            return;
        }
        let msg = self.error_msg.clone().unwrap();
        let mut open = true;
        egui::Window::new("⚠  Error")
            .collapsible(false)
            .resizable(false)
            .min_width(360.0)
            .open(&mut open)
            .show(ctx, |ui| {
                ui.label(RichText::new(&msg).color(Color32::LIGHT_RED));
                ui.add_space(12.0);
                if ui.button("Dismiss").clicked() {
                    self.error_msg = None;
                }
            });
        if !open {
            self.error_msg = None;
        }
    }

    fn show_info_modal(&mut self, ctx: &egui::Context) {
        if self.info_msg.is_none() {
            return;
        }
        let msg = self.info_msg.clone().unwrap();
        let mut open = true;
        egui::Window::new("✓  Success")
            .collapsible(false)
            .resizable(false)
            .min_width(320.0)
            .open(&mut open)
            .show(ctx, |ui| {
                ui.label(RichText::new(&msg).color(Color32::from_rgb(120, 210, 120)));
                ui.add_space(12.0);
                if ui.button("OK").clicked() {
                    self.info_msg = None;
                }
            });
        if !open {
            self.info_msg = None;
        }
    }

    // ── Background operations ─────────────────────────────────────────────

    fn do_create_archive(&mut self) {
        let out_path = PathBuf::from(&self.add_dialog.output_path);
        let input_paths = self.add_dialog.input_paths.clone();
        let codec = self.add_dialog.codec.as_str().to_string();
        let level = self.add_dialog.level;
        let password = if self.add_dialog.password.is_empty() {
            None
        } else {
            Some(self.add_dialog.password.clone())
        };

        let (tx, rx) = mpsc::channel();
        self.result_rx = Some(rx);
        self.busy = true;
        self.busy_msg = "Creating archive…".into();

        thread::spawn(move || {
            let mut entries: Vec<ArchiveInputEntry> = Vec::new();

            for path in &input_paths {
                if path.is_dir() {
                    match collect_directory_entries(path, path) {
                        Ok(mut e) => entries.append(&mut e),
                        Err(e) => {
                            tx.send(WorkResult::Error(e.to_string())).ok();
                            return;
                        }
                    }
                } else {
                    let data = match std::fs::read(path) {
                        Ok(d) => d,
                        Err(e) => {
                            tx.send(WorkResult::Error(e.to_string())).ok();
                            return;
                        }
                    };
                    let modified = path
                        .metadata()
                        .ok()
                        .and_then(|m| m.modified().ok())
                        .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
                        .map(|d| d.as_secs() as i64)
                        .unwrap_or(0);
                    let rel_path = path
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_else(|| "file".to_string());
                    entries.push(ArchiveInputEntry {
                        rel_path,
                        data,
                        mode: 0o644,
                        modified,
                        is_dir: false,
                    });
                }
            }

            let count = entries.iter().filter(|e| !e.is_dir).count();
            let pwd = password.as_deref();
            match VpackArchive::create_archive(&out_path, entries, level, &codec, pwd, None, None) {
                Ok(()) => tx
                    .send(WorkResult::Created {
                        path: out_path,
                        count,
                    })
                    .ok(),
                Err(e) => tx.send(WorkResult::Error(e.to_string())).ok(),
            };
        });
    }

    fn do_extract(
        &mut self,
        dest: String,
        password: Option<String>,
        selected_paths: Option<Vec<String>>,
    ) {
        let archive = match &self.archive {
            Some(a) => a.clone(),
            None => return,
        };

        let (tx, rx) = mpsc::channel();
        self.result_rx = Some(rx);
        self.busy = true;
        self.busy_msg = format!("Extracting → {dest}…");

        let dest_path = PathBuf::from(&dest);
        let dest_clone = dest.clone();

        thread::spawn(move || {
            let pwd = password.as_deref();

            let result = if let Some(paths) = selected_paths {
                // Extract only selected
                let mut count = 0usize;
                for p in &paths {
                    match archive.extract_file(p, pwd) {
                        Ok(data) => {
                            let out = dest_path.join(p.replace('/', std::path::MAIN_SEPARATOR_STR));
                            if let Some(parent) = out.parent() {
                                let _ = std::fs::create_dir_all(parent);
                            }
                            if std::fs::write(&out, &data).is_ok() {
                                count += 1;
                            }
                        }
                        Err(e) => {
                            tx.send(WorkResult::Error(e.to_string())).ok();
                            return;
                        }
                    }
                }
                Ok(count)
            } else {
                archive.extract_all(&dest_path, pwd)
            };

            match result {
                Ok(count) => tx
                    .send(WorkResult::Extracted {
                        count,
                        dest: dest_clone,
                    })
                    .ok(),
                Err(e) => tx.send(WorkResult::Error(e.to_string())).ok(),
            };
        });
    }

    fn do_test_integrity(&mut self, password: Option<String>) {
        let archive = match &self.archive {
            Some(a) => a.clone(),
            None => return,
        };

        let (tx, rx) = mpsc::channel();
        self.result_rx = Some(rx);
        self.busy = true;
        self.busy_msg = "Verifying integrity…".into();

        thread::spawn(move || {
            let pwd = password.as_deref();
            match archive.test_integrity(pwd) {
                Ok(count) => tx.send(WorkResult::IntegrityPassed { count }).ok(),
                Err(e) => tx.send(WorkResult::Error(e.to_string())).ok(),
            };
        });
    }

    fn show_archive_info(&mut self) {
        let Some(archive) = &self.archive else {
            return;
        };
        let m = &archive.metadata;
        let path_str = self
            .archive_path
            .as_ref()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_default();
        let file_count = archive
            .central_directory
            .iter()
            .filter(|e| !e.is_dir)
            .count();
        let dir_count = archive
            .central_directory
            .iter()
            .filter(|e| e.is_dir)
            .count();
        let security =
            if (archive.flags & FLAG_SIGNED) != 0 && (archive.flags & FLAG_ENCRYPTED) != 0 {
                "Signed + Encrypted"
            } else if (archive.flags & FLAG_SIGNED) != 0 {
                "Digitally Signed (Ed25519)"
            } else if (archive.flags & FLAG_ENCRYPTED) != 0 {
                "Password Encrypted"
            } else {
                "Standard (no encryption)"
            };
        let comment_str = m
            .comment
            .as_ref()
            .map(|c| format!("\nComment: {c}"))
            .unwrap_or_default();
        self.info_msg = Some(format!(
            "Archive: {path_str}\nCreator: {creator}\nCreated: {created}\n\
             Files: {file_count}   Folders: {dir_count}\n\
             Orig: {orig}   Packed: {packed}   Ratio: {ratio:.1}%\n\
             Security: {security}{comment_str}",
            creator = m.creator,
            created = fmt_timestamp(m.created_at),
            orig = fmt_size(m.total_uncompressed_bytes),
            packed = fmt_size(m.total_compressed_bytes),
            ratio = ratio(m.total_uncompressed_bytes, m.total_compressed_bytes),
        ));
    }
}

// ──────────────────────────────────────────────────────────────────────────
// eframe App impl
// ──────────────────────────────────────────────────────────────────────────

impl eframe::App for VpackApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Poll background operations
        self.poll_results();

        // Handle drag-and-drop
        let dropped: Vec<PathBuf> = ctx.input(|i| {
            i.raw
                .dropped_files
                .iter()
                .filter_map(|f| f.path.clone())
                .collect()
        });
        for path in dropped {
            if path.extension().and_then(|e| e.to_str()) == Some("vpack") {
                self.open_archive_path(path, None);
                break;
            }
        }

        // Top menu bar
        egui::TopBottomPanel::top("menubar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("📂  Open…").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("VPack Archives", &["vpack"])
                            .add_filter("All Files", &["*"])
                            .pick_file()
                        {
                            self.open_archive_path(path, None);
                        }
                        ui.close_menu();
                    }
                    if ui.button("✚  New Archive…").clicked() {
                        self.add_dialog.open = true;
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("✕  Close Archive").clicked() {
                        self.archive = None;
                        self.archive_path = None;
                        self.display_order.clear();
                        self.selected.clear();
                        self.status = "Archive closed.".into();
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("Archive", |ui| {
                    let has_archive = self.archive.is_some();
                    if ui
                        .add_enabled(has_archive, egui::Button::new("📤  Extract All…"))
                        .clicked()
                    {
                        if let Some(path) = &self.archive_path {
                            self.extract_dialog.destination =
                                path.parent().unwrap_or(path).to_string_lossy().to_string();
                        }
                        self.extract_dialog.selected_only = false;
                        self.extract_dialog.open = true;
                        ui.close_menu();
                    }
                    if ui
                        .add_enabled(
                            has_archive && !self.selected.is_empty(),
                            egui::Button::new("📤  Extract Selected…"),
                        )
                        .clicked()
                    {
                        self.extract_dialog.selected_only = true;
                        self.extract_dialog.open = true;
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui
                        .add_enabled(
                            has_archive && !self.busy,
                            egui::Button::new("🔍  Test Integrity"),
                        )
                        .clicked()
                    {
                        self.do_test_integrity(None);
                        ui.close_menu();
                    }
                    if ui
                        .add_enabled(has_archive, egui::Button::new("ℹ  Archive Info"))
                        .clicked()
                    {
                        self.show_archive_info();
                        ui.close_menu();
                    }
                });

                ui.menu_button("Selection", |ui| {
                    if ui.button("Select All").clicked() {
                        if let Some(archive) = &self.archive {
                            self.selected = (0..archive.central_directory.len()).collect();
                        }
                        ui.close_menu();
                    }
                    if ui.button("Deselect All").clicked() {
                        self.selected.clear();
                        ui.close_menu();
                    }
                    if ui.button("Invert Selection").clicked() {
                        if let Some(archive) = &self.archive {
                            let all: HashSet<usize> =
                                (0..archive.central_directory.len()).collect();
                            self.selected = all.difference(&self.selected).copied().collect();
                        }
                        ui.close_menu();
                    }
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("About VPack Archiver").clicked() {
                        self.info_msg = Some(
                            "VPack Archiver 2.0\n\
                             \n\
                             The WinRAR for .vpack files.\n\
                             Next-generation universal archive manager.\n\
                             \n\
                             Formats: VPK2 (Central Directory at EOF)\n\
                             Codecs:  Deflate · LZ4\n\
                             Security: Ed25519 signatures · Stream encryption\n\
                             \n\
                             https://github.com/LeTrollologist/vpack-archiver"
                                .to_string(),
                        );
                        ui.close_menu();
                    }
                });
            });
        });

        // Toolbar
        egui::TopBottomPanel::top("toolbar").show(ctx, |ui| {
            self.show_toolbar(ui);
        });

        // Status bar
        egui::TopBottomPanel::bottom("statusbar")
            .min_height(22.0)
            .show(ctx, |ui| {
                self.show_status_bar(ui);
            });

        // Main file table
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::new()
                .stroke(Stroke::new(1.0_f32, Color32::from_gray(50)))
                .show(ui, |ui| {
                    egui::ScrollArea::both().auto_shrink(false).show(ui, |ui| {
                        self.show_file_table(ui);
                    });
                });
        });

        // Dialogs
        self.show_add_dialog(ctx);
        self.show_extract_dialog(ctx);
        self.show_error_modal(ctx);
        self.show_info_modal(ctx);

        // Request repaint while busy so the spinner animates
        if self.busy {
            ctx.request_repaint();
        }
    }
}

// ──────────────────────────────────────────────────────────────────────────
// Utility helpers
// ──────────────────────────────────────────────────────────────────────────

fn file_icon(is_dir: bool, path: &str) -> &'static str {
    if is_dir {
        return "📁";
    }
    let ext = path.rsplit('.').next().unwrap_or("").to_lowercase();
    match ext.as_str() {
        "exe" | "dll" | "bin" | "so" | "dylib" => "⚙",
        "rs" | "py" | "js" | "ts" | "c" | "cpp" | "h" | "go" | "rb" => "🖹",
        "zip" | "tar" | "gz" | "bz2" | "xz" | "7z" | "rar" | "vpack" => "🗀",
        "png" | "jpg" | "jpeg" | "gif" | "svg" | "bmp" | "ico" | "webp" => "🖼",
        "mp4" | "mkv" | "mov" | "avi" | "webm" => "🎬",
        "mp3" | "wav" | "ogg" | "flac" | "aac" => "🎵",
        "pdf" => "📕",
        "md" | "txt" | "rst" | "log" => "📝",
        "json" | "toml" | "yaml" | "yml" | "xml" | "ini" | "cfg" => "⚙",
        _ => "📄",
    }
}

fn fmt_size(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;
    let b = bytes as f64;
    if b < KB {
        format!("{bytes} B")
    } else if b < MB {
        format!("{:.1} KB", b / KB)
    } else if b < GB {
        format!("{:.2} MB", b / MB)
    } else {
        format!("{:.2} GB", b / GB)
    }
}

fn ratio(orig: u64, packed: u64) -> f64 {
    if orig == 0 {
        return 0.0;
    }
    (1.0 - (packed as f64 / orig as f64)) * 100.0
}

fn ratio_color(r: f64) -> Color32 {
    if r >= 60.0 {
        Color32::from_rgb(80, 200, 120)
    } else if r >= 30.0 {
        Color32::from_rgb(200, 200, 80)
    } else if r > 0.0 {
        Color32::LIGHT_GRAY
    } else {
        Color32::from_gray(120)
    }
}

fn fmt_timestamp(ts: i64) -> String {
    if ts == 0 {
        return "—".to_string();
    }
    if let Some(dt) = chrono::DateTime::from_timestamp(ts, 0) {
        dt.with_timezone(&Local)
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
    } else {
        "—".to_string()
    }
}
