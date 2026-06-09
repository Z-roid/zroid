#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]
#![deny(clippy::restriction)]
#![allow(
    clippy::allow_attributes_without_reason,
    clippy::blanket_clippy_restriction_lints, 
    clippy::implicit_return, 
    clippy::float_arithmetic, 
    clippy::cast_precision_loss, 
    clippy::cast_possible_truncation, 
    clippy::cast_sign_loss, 
    clippy::as_conversions, 
    clippy::shadow_reuse, 
    clippy::shadow_unrelated, 
    clippy::missing_docs_in_private_items, 
    clippy::missing_trait_methods, 
    clippy::pattern_type_mismatch, 
    clippy::arithmetic_side_effects, 
    clippy::unwrap_used, 
    clippy::expect_used,
    clippy::too_many_lines, 
    clippy::single_call_fn, 
    clippy::multiple_crate_versions, 
    clippy::question_mark_used,
    clippy::indexing_slicing,
    clippy::str_to_string,
    clippy::string_add,
    clippy::std_instead_of_core,
    clippy::std_instead_of_alloc,
    clippy::print_stdout,
    clippy::print_stderr,
    clippy::wildcard_imports,
    clippy::mod_module_files,
    clippy::pub_with_shorthand,
    clippy::separated_literal_suffix,
    clippy::non_ascii_literal,
    clippy::else_if_without_else,
    clippy::partial_pub_fields,
    clippy::inline_modules,
    clippy::cast_lossless,
    clippy::suboptimal_flops,
    clippy::modulo_arithmetic,
    clippy::min_ident_chars,
    clippy::if_then_some_else_none,
    clippy::redundant_closure_for_method_calls,
    clippy::wildcard_enum_match_arm,
    clippy::let_underscore_must_use,
    clippy::let_underscore_untyped,
    clippy::assigning_clones,
    clippy::uninlined_format_args,
    clippy::absolute_paths,
    clippy::arbitrary_source_item_ordering,
    clippy::cognitive_complexity,
    clippy::default_numeric_fallback,
    clippy::collapsible_if,
    clippy::collapsible_else_if,
    clippy::bool_to_int_with_if,
    clippy::range_plus_one,
    clippy::map_unwrap_or,
    clippy::clone_on_ref_ptr,
    clippy::filetype_is_file,
    clippy::needless_pass_by_value,
    clippy::undocumented_unsafe_blocks,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
    clippy::struct_excessive_bools
)]
#![allow(deprecated)]

use eframe::egui;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use ignore::WalkBuilder;
use std::sync::mpsc::{channel, Receiver, Sender};
use ropey::Rope;
use log::info;
use memmap2::Mmap;

// ── Palette ────────────────────────────────────────────────────────────────
pub const C_BASE:    egui::Color32 = egui::Color32::from_rgb(0x1e, 0x1e, 0x2e); // editor bg
pub const C_MANTLE:  egui::Color32 = egui::Color32::from_rgb(0x18, 0x18, 0x25); // darkest
pub const C_SURFACE: egui::Color32 = egui::Color32::from_rgb(0x24, 0x27, 0x3a); // panels
pub const C_OVERLAY: egui::Color32 = egui::Color32::from_rgb(0x36, 0x3a, 0x4f); // borders / hover
pub const C_MUTED:   egui::Color32 = egui::Color32::from_rgb(0x6e, 0x73, 0x8d); // dim text
pub const C_TEXT:    egui::Color32 = egui::Color32::from_rgb(0xca, 0xd3, 0xf5); // main text
pub const C_BLUE:    egui::Color32 = egui::Color32::from_rgb(0x8a, 0xad, 0xf4); // accent
pub const C_PURPLE:  egui::Color32 = egui::Color32::from_rgb(0xc6, 0xa0, 0xf6); // keywords
pub const C_GREEN:   egui::Color32 = egui::Color32::from_rgb(0xa6, 0xda, 0x95); // strings/ok
pub const C_YELLOW:  egui::Color32 = egui::Color32::from_rgb(0xee, 0xd4, 0x9f); // warnings/types
pub const C_RED:     egui::Color32 = egui::Color32::from_rgb(0xed, 0x87, 0x96); // errors

fn zord_theme() -> egui::Visuals {
    let mut v = egui::Visuals::dark();
    v.window_fill                          = C_BASE;
    v.panel_fill                           = C_SURFACE;
    v.faint_bg_color                       = C_SURFACE;
    v.extreme_bg_color                     = C_MANTLE;
    v.code_bg_color                        = C_BASE;
    v.window_stroke                        = egui::Stroke::new(1.0_f32, C_OVERLAY);
    v.widgets.noninteractive.bg_fill       = C_SURFACE;
    v.widgets.noninteractive.weak_bg_fill  = C_SURFACE;
    v.widgets.noninteractive.bg_stroke     = egui::Stroke::new(1.0_f32, C_OVERLAY);
    v.widgets.noninteractive.fg_stroke     = egui::Stroke::new(1.0_f32, C_MUTED);
    v.widgets.inactive.bg_fill             = C_SURFACE;
    v.widgets.inactive.weak_bg_fill        = C_SURFACE;
    v.widgets.inactive.bg_stroke           = egui::Stroke::NONE;
    v.widgets.hovered.bg_fill              = C_OVERLAY;
    v.widgets.hovered.weak_bg_fill         = C_OVERLAY;
    v.widgets.hovered.bg_stroke            = egui::Stroke::new(1.0_f32, C_BLUE);
    v.widgets.active.bg_fill               = C_OVERLAY;
    v.widgets.active.weak_bg_fill          = C_OVERLAY;
    v.widgets.active.bg_stroke             = egui::Stroke::new(1.5_f32, C_BLUE);
    v.widgets.open.bg_fill                 = C_MANTLE;
    v.widgets.open.weak_bg_fill            = C_MANTLE;
    v.widgets.open.bg_stroke               = egui::Stroke::new(1.0_f32, C_BLUE);
    v.selection.bg_fill                    = egui::Color32::from_rgba_unmultiplied(0x8a, 0xad, 0xf4, 60);
    v.selection.stroke                     = egui::Stroke::NONE;
    v.override_text_color                  = Some(C_TEXT);
    v.hyperlink_color                      = C_BLUE;
    v.warn_fg_color                        = C_YELLOW;
    v.error_fg_color                       = C_RED;
    v.interact_cursor                      = Some(egui::CursorIcon::Text);
    v
}

fn main() -> eframe::Result {
    env_logger::init();
    // Force FXC on D3D12 — DXC (DirectX Shader Compiler) is not installed on all Windows systems.
    // FXC is always available and still gives uncapped Immediate present mode.
    if std::env::var("WGPU_DX12_COMPILER").is_err() {
        unsafe { std::env::set_var("WGPU_DX12_COMPILER", "fxc"); }
    }
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_min_inner_size([400.0, 300.0])
            .with_title("Zord"),
        renderer: eframe::Renderer::Wgpu,
        vsync: false,
        ..Default::default()
    };
    eframe::run_native(
        "Zord",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(zord_theme());
            let mut style = (*cc.egui_ctx.style()).clone();
            style.spacing.item_spacing = egui::vec2(6.0, 4.0);
            style.spacing.button_padding = egui::vec2(8.0, 3.0);
            style.spacing.menu_margin = egui::Margin::same(6);
            cc.egui_ctx.set_style(style);
            Ok(Box::new(ZordApp::new()))
        }),
    )
}

enum AppMessage {
    FileLoaded { id: usize, path: PathBuf, rope: Rope, lines: usize, size_mb: f32 },
}

struct HistoryEntry {
    rope: Rope,
    cursor: usize,
    selection: Option<usize>,
}

#[derive(Clone)]
struct EditorView {
    buffer_idx: usize,
    scroll_y: f64,
    is_dragging_scrollbar: bool,
    cursor_char_idx: usize,
    selection_start: Option<usize>,
}

impl EditorView {
    fn for_buffer(buffer_idx: usize) -> Self {
        Self { buffer_idx, scroll_y: 0.0, is_dragging_scrollbar: false, cursor_char_idx: 0, selection_start: None }
    }
}

impl Default for EditorView {
    fn default() -> Self { Self::for_buffer(0) }
}

struct Buffer {
    rope: Rope,
    path: Option<PathBuf>,
    line_count: usize,
    gutter_width: f32,
    dirty_text: bool,
    unsaved_changes: bool,
    undo_stack: Vec<HistoryEntry>,
    redo_stack: Vec<HistoryEntry>,
}

impl Buffer {
    fn new(rope: Rope) -> Self {
        let lc = rope.len_lines();
        let gw = (lc.to_string().len() as f32 * 8.0 + 16.0).max(45.0);
        Self { rope, path: None, line_count: lc, gutter_width: gw, dirty_text: false, unsaved_changes: false, undo_stack: Vec::new(), redo_stack: Vec::new() }
    }
    fn recalc(&mut self) {
        if self.dirty_text {
            self.line_count = self.rope.len_lines();
            self.gutter_width = (self.line_count.to_string().len() as f32 * 8.0 + 16.0).max(45.0);
            self.dirty_text = false;
        }
    }
    fn push_undo(&mut self, cursor: usize, sel: Option<usize>) {
        self.undo_stack.push(HistoryEntry { rope: self.rope.clone(), cursor, selection: sel });
        if self.undo_stack.len() > 200 { self.undo_stack.remove(0); }
        self.redo_stack.clear();
        self.unsaved_changes = true;
        self.dirty_text = true;
    }
}

struct ZordApp {
    buffers: Vec<Buffer>,
    file_tree: Arc<Mutex<Vec<PathBuf>>>,
    explorer_cache: Vec<PathBuf>,
    project_root: PathBuf,
    tx: Sender<AppMessage>,
    rx: Receiver<AppMessage>,
    loading_file: bool,
    current_load_id: usize,
    last_frame_time: std::time::Instant,
    internal_fps: f64,
    logs: Vec<String>,
    views: Vec<EditorView>,
    editor_focused: bool,
    show_right_panel: bool,
    show_bottom_panel: bool,
    show_command_palette: bool,
    command_palette_input: String,
    show_find_bar: bool,
    find_query: String,
    active_view: usize,
    split_ratio: f32,
    dragging_split: bool,
}

impl ZordApp {
    fn new() -> Self {
        let root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let (tx, rx) = channel();
        let welcome = Buffer::new(Rope::from_str("// Welcome to Zord IDE\nfn main() {\n    println!(\"Hello, Zord!\");\n}"));
        let app = Self {
            buffers: vec![welcome],
            file_tree: Arc::new(Mutex::new(Vec::new())),
            explorer_cache: Vec::new(),
            project_root: root,
            tx,
            rx,
            loading_file: false,
            current_load_id: 0,
            last_frame_time: std::time::Instant::now(),
            internal_fps: 0.0,
            logs: vec!["[System] Zord IDE initialized.".to_owned()],
            views: vec![EditorView::default()],
            editor_focused: true,
            show_right_panel: false,
            show_bottom_panel: true,
            show_command_palette: false,
            command_palette_input: String::new(),
            show_find_bar: false,
            find_query: String::new(),
            active_view: 0,
            split_ratio: 0.5,
            dragging_split: false,
        };
        app.refresh_files();
        app
    }

    fn active_buf(&self) -> usize {
        let av = self.active_view.min(self.views.len().saturating_sub(1));
        self.views[av].buffer_idx.min(self.buffers.len().saturating_sub(1))
    }

    fn undo(&mut self) {
        let bi = self.active_buf();
        let av = self.active_view.min(self.views.len().saturating_sub(1));
        let cursor = self.views[av].cursor_char_idx;
        let sel    = self.views[av].selection_start;
        if let Some(entry) = self.buffers[bi].undo_stack.pop() {
            let current_rope = self.buffers[bi].rope.clone();
            self.buffers[bi].redo_stack.push(HistoryEntry { rope: current_rope, cursor, selection: sel });
            self.buffers[bi].rope = entry.rope;
            self.views[av].cursor_char_idx = entry.cursor;
            self.views[av].selection_start = entry.selection;
            self.buffers[bi].dirty_text = true;
            self.buffers[bi].unsaved_changes = true;
        }
    }

    fn redo(&mut self) {
        let bi = self.active_buf();
        let av = self.active_view.min(self.views.len().saturating_sub(1));
        let cursor = self.views[av].cursor_char_idx;
        let sel    = self.views[av].selection_start;
        if let Some(entry) = self.buffers[bi].redo_stack.pop() {
            let current_rope = self.buffers[bi].rope.clone();
            self.buffers[bi].undo_stack.push(HistoryEntry { rope: current_rope, cursor, selection: sel });
            self.buffers[bi].rope = entry.rope;
            self.views[av].cursor_char_idx = entry.cursor;
            self.views[av].selection_start = entry.selection;
            self.buffers[bi].dirty_text = true;
            self.buffers[bi].unsaved_changes = true;
        }
    }

    fn save_file(&mut self) {
        let bi = self.active_buf();
        if let Some(path) = self.buffers[bi].path.clone() {
            let content = self.buffers[bi].rope.to_string();
            match std::fs::write(&path, content) {
                Ok(()) => { self.buffers[bi].unsaved_changes = false; self.add_log(&format!("Saved {}", path.file_name().unwrap_or_default().to_string_lossy())); }
                Err(e) => { self.add_log(&format!("Save failed: {e}")); }
            }
        } else {
            self.add_log("No file open to save.");
        }
    }

    fn add_log(&mut self, msg: &str) {
        let timestamp = chrono::Local::now().format("%H:%M:%S").to_string();
        self.logs.push(format!("[{timestamp}] {msg}"));
        info!("{msg}");
        if self.logs.len() > 200 { self.logs.remove(0); }
    }

    fn refresh_files(&self) {
        let tree = Arc::clone(&self.file_tree);
        let root = self.project_root.clone();
        std::thread::spawn(move || {
            let mut paths = Vec::new();
            for entry in WalkBuilder::new(&root)
                .max_depth(Some(8))
                .git_ignore(true)
                .git_global(true)
                .hidden(false)
                .build()
                .flatten()
            {
                if entry.file_type().is_some_and(|ft| !ft.is_dir()) {
                    paths.push(entry.path().to_path_buf());
                }
                if paths.len() >= 8_000 { break; }
            }
            paths.sort_unstable();
            if let Ok(mut lock) = tree.lock() { *lock = paths; }
        });
    }

    fn load_file_async(&mut self, path: PathBuf) {
        self.current_load_id += 1;
        let load_id = self.current_load_id;
        self.loading_file = true;
        let tx = self.tx.clone();
        let path_clone = path.clone();
        self.add_log(&format!("Streaming: {}", path.display()));
        std::thread::spawn(move || {
            if let Ok(file) = std::fs::File::open(&path_clone) {
                let size_mb = file.metadata().map_or(0.0, |m| m.len() as f32 / 1_048_576.0);
                if let Ok(map) = unsafe { Mmap::map(&file) } {
                    if let Ok(content) = std::str::from_utf8(&map) {
                        let rope = Rope::from_str(content);
                        let lines = rope.len_lines();
                        let _ = tx.send(AppMessage::FileLoaded { id: load_id, path: path_clone, rope, lines, size_mb });
                    } else if let Ok(rope) = Rope::from_reader(std::io::BufReader::new(file)) {
                        let lines = rope.len_lines();
                        let _ = tx.send(AppMessage::FileLoaded { id: load_id, path: path_clone, rope, lines, size_mb });
                    }
                }
            }
        });
    }

    fn update_logic(&mut self, ctx: &egui::Context) {
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::P)) { self.show_command_palette = !self.show_command_palette; }
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::B)) { self.show_right_panel = !self.show_right_panel; }
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::J)) { self.show_bottom_panel = !self.show_bottom_panel; }
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::S)) { self.save_file(); }
        if ctx.input(|i| i.modifiers.ctrl && !i.modifiers.shift && i.key_pressed(egui::Key::Z)) { self.undo(); }
        if ctx.input(|i| i.modifiers.ctrl && (i.key_pressed(egui::Key::Y) || (i.modifiers.shift && i.key_pressed(egui::Key::Z)))) { self.redo(); }
        if ctx.input(|i| i.modifiers.ctrl && i.key_pressed(egui::Key::F)) { self.show_find_bar = !self.show_find_bar; }
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) { self.show_find_bar = false; self.show_command_palette = false; }

        let now = std::time::Instant::now();
        let elapsed = now.duration_since(self.last_frame_time).as_secs_f64();
        if elapsed > 0.0 { self.internal_fps = (self.internal_fps * 0.95) + ((1.0 / elapsed) * 0.05); }
        self.last_frame_time = now;

        while let Ok(msg) = self.rx.try_recv() {
            match msg {
                AppMessage::FileLoaded { id, path, rope, lines, size_mb } => {
                    if id == self.current_load_id {
                        // Load into the active view's buffer, or push a new buffer
                        let av = self.active_view.min(self.views.len().saturating_sub(1));
                        let bi = self.views[av].buffer_idx;
                        if bi < self.buffers.len() {
                            self.buffers[bi].rope = rope;
                            self.buffers[bi].line_count = lines;
                            self.buffers[bi].path = Some(path.clone());
                            self.buffers[bi].dirty_text = true;
                            self.buffers[bi].unsaved_changes = false;
                            self.buffers[bi].undo_stack.clear();
                            self.buffers[bi].redo_stack.clear();
                        }
                        self.views[av].cursor_char_idx = 0;
                        self.views[av].selection_start = None;
                        self.views[av].scroll_y = 0.0;
                        self.loading_file = false;
                        self.add_log(&format!("Loaded {} ({:.1} MB)", path.file_name().unwrap().to_string_lossy(), size_mb));
                    }
                }
            }
        }
        if self.explorer_cache.is_empty() { if let Ok(tree) = self.file_tree.try_lock() { self.explorer_cache.clone_from(&tree); } }

        if self.editor_focused {
            let av = self.active_view.min(self.views.len().saturating_sub(1));
            let bi = self.views[av].buffer_idx.min(self.buffers.len().saturating_sub(1));
            let mut cursor  = self.views[av].cursor_char_idx;
            let mut sel     = self.views[av].selection_start;
            let mut modified = false;
            let mut copied_text: Option<String> = None;
            ctx.input(|i| {
                for event in &i.events {
                    match event {
                        egui::Event::Copy => {
                            if let Some(start) = sel {
                                let lo = start.min(cursor); let hi = start.max(cursor);
                                if lo < hi { copied_text = Some(self.buffers[bi].rope.slice(lo..hi).to_string()); }
                            }
                        }
                        egui::Event::Paste(text) => {
                            self.buffers[bi].push_undo(cursor, sel);
                            if let Some(start) = sel {
                                let lo = start.min(cursor); let hi = start.max(cursor);
                                self.buffers[bi].rope.remove(lo..hi); cursor = lo; sel = None;
                            }
                            self.buffers[bi].rope.insert(cursor, text); cursor += text.chars().count(); modified = true;
                        }
                        egui::Event::Text(text) => {
                            if text.chars().all(|c| !c.is_control()) {
                                self.buffers[bi].push_undo(cursor, sel);
                                if let Some(start) = sel {
                                    let lo = start.min(cursor); let hi = start.max(cursor);
                                    self.buffers[bi].rope.remove(lo..hi); cursor = lo; sel = None;
                                }
                                self.buffers[bi].rope.insert(cursor, text); cursor += text.chars().count(); modified = true;
                            }
                        }
                        egui::Event::Key { key, pressed: true, modifiers, .. } => {
                            if !modifiers.shift && matches!(key, egui::Key::ArrowLeft | egui::Key::ArrowRight | egui::Key::ArrowUp | egui::Key::ArrowDown) { sel = None; }
                            match key {
                                egui::Key::Backspace => {
                                    self.buffers[bi].push_undo(cursor, sel);
                                    if let Some(start) = sel {
                                        let lo = start.min(cursor); let hi = start.max(cursor);
                                        self.buffers[bi].rope.remove(lo..hi); cursor = lo; sel = None; modified = true;
                                    } else if cursor > 0 {
                                        cursor -= 1; self.buffers[bi].rope.remove(cursor..=cursor); modified = true;
                                    }
                                }
                                egui::Key::Enter => { self.buffers[bi].push_undo(cursor, sel); self.buffers[bi].rope.insert(cursor, "\n"); cursor += 1; modified = true; }
                                egui::Key::ArrowLeft  => { if cursor > 0 { cursor -= 1; } }
                                egui::Key::ArrowRight => { if cursor < self.buffers[bi].rope.len_chars() { cursor += 1; } }
                                egui::Key::ArrowUp => {
                                    let cl = self.buffers[bi].rope.char_to_line(cursor);
                                    if cl > 0 {
                                        let col = cursor - self.buffers[bi].rope.line_to_char(cl);
                                        let ps = self.buffers[bi].rope.line_to_char(cl - 1);
                                        cursor = ps + col.min(self.buffers[bi].rope.line(cl - 1).len_chars().saturating_sub(1));
                                    }
                                }
                                egui::Key::ArrowDown => {
                                    let cl = self.buffers[bi].rope.char_to_line(cursor);
                                    if cl + 1 < self.buffers[bi].rope.len_lines() {
                                        let col = cursor - self.buffers[bi].rope.line_to_char(cl);
                                        let ns = self.buffers[bi].rope.line_to_char(cl + 1);
                                        cursor = ns + col.min(self.buffers[bi].rope.line(cl + 1).len_chars().saturating_sub(1));
                                    }
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                }
            });
            self.views[av].cursor_char_idx = cursor;
            self.views[av].selection_start = sel;
            if let Some(text) = copied_text { ctx.copy_text(text); }
            if modified { self.buffers[bi].dirty_text = true; }
        }

        for buf in &mut self.buffers { buf.recalc(); }
    }

    fn render_editor(&mut self, ui: &mut egui::Ui, view_idx: usize) {
        let bi = self.views[view_idx].buffer_idx.min(self.buffers.len().saturating_sub(1));
        let font_id = egui::FontId::monospace(13.0);
        let rh = ui.ctx().fonts_mut(|f| f.row_height(&font_id)) + 2.0;
        let cw = ui.ctx().fonts_mut(|f| f.glyph_width(&font_id, 'a'));
        let lc = self.buffers[bi].line_count.max(1);
        let gw = self.buffers[bi].gutter_width;
        let doc_h = lc as f64 * rh as f64;

        let (rect, response) = ui.allocate_exact_size(ui.available_size(), egui::Sense::click_and_drag());
        let sbr = egui::Rect::from_min_max(egui::pos2(rect.max.x - 12.0, rect.min.y), rect.max);
        let max_s = (doc_h - rect.height() as f64).max(0.0);
        let hh = (rect.height() as f64 * (rect.height() as f64 / doc_h).clamp(0.0, 1.0)).max(20.0);

        if response.hovered() { self.views[view_idx].scroll_y = (self.views[view_idx].scroll_y - ui.input(|i| i.smooth_scroll_delta.y as f64)).clamp(0.0, max_s); }
        let is_sb = ui.input(|i| i.pointer.interact_pos().is_some_and(|p| sbr.contains(p)));

        if response.drag_started() && is_sb { self.views[view_idx].is_dragging_scrollbar = true; }
        if response.drag_stopped() { self.views[view_idx].is_dragging_scrollbar = false; }
        if response.drag_started() && !is_sb {
            self.editor_focused = true;
            self.active_view = view_idx;
            let sy = self.views[view_idx].scroll_y;
            if let Some(pos) = response.interact_pointer_pos() {
                let idx = Self::calc_char_pos(&self.buffers[bi].rope, pos, rect, sy, gw, rh, cw);
                self.views[view_idx].cursor_char_idx = idx;
                self.views[view_idx].selection_start = Some(idx);
            }
        }
        if response.dragged() {
            if self.views[view_idx].is_dragging_scrollbar {
                let dy = response.drag_delta().y as f64;
                if rect.height() as f64 > hh { self.views[view_idx].scroll_y = (self.views[view_idx].scroll_y + (dy / (rect.height() as f64 - hh)) * max_s).clamp(0.0, max_s); }
            } else {
                let sy = self.views[view_idx].scroll_y;
                if let Some(pos) = response.interact_pointer_pos() {
                    self.views[view_idx].cursor_char_idx = Self::calc_char_pos(&self.buffers[bi].rope, pos, rect, sy, gw, rh, cw);
                }
            }
        }
        if response.clicked() && !is_sb {
            self.editor_focused = true;
            self.active_view = view_idx;
            let sy = self.views[view_idx].scroll_y;
            if let Some(pos) = response.interact_pointer_pos() {
                let idx = Self::calc_char_pos(&self.buffers[bi].rope, pos, rect, sy, gw, rh, cw);
                if !ui.input(|i| i.modifiers.shift) { self.views[view_idx].selection_start = None; }
                else if self.views[view_idx].selection_start.is_none() { self.views[view_idx].selection_start = Some(self.views[view_idx].cursor_char_idx); }
                self.views[view_idx].cursor_char_idx = idx;
            }
        }
        if ui.input(|i| i.pointer.any_click()) && !response.contains_pointer() { self.editor_focused = false; }

        let painter = ui.painter().with_clip_rect(rect);
        let sy = self.views[view_idx].scroll_y;
        let f_row = (sy / rh as f64).floor() as usize;
        let l_row = ((sy + rect.height() as f64) / rh as f64).ceil() as usize;
        let f_row = f_row.clamp(0, lc.saturating_sub(1));
        let l_row = l_row.clamp(0, lc);

        painter.rect_filled(egui::Rect::from_min_max(rect.min, egui::pos2(rect.min.x + gw, rect.max.y)), 0.0, ui.visuals().faint_bg_color);
        painter.vline(rect.min.x + gw, rect.min.y..=rect.max.y, ui.visuals().widgets.noninteractive.bg_stroke);

        let cursor_idx = self.views[view_idx].cursor_char_idx;
        let sel = self.views[view_idx].selection_start.map(|s| (s.min(cursor_idx), s.max(cursor_idx)));
        let sel_bg = egui::Color32::from_rgba_unmultiplied(0x8a, 0xad, 0xf4, 55);

        for i in f_row..l_row {
            if let Some(line) = self.buffers[bi].rope.get_line(i) {
                let scy = rect.min.y + (i as f64 * rh as f64 - sy) as f32;
                if let Some((ss, se)) = sel {
                    let lsc = self.buffers[bi].rope.line_to_char(i); let lec = lsc + line.len_chars();
                    if se > lsc && ss < lec {
                        let xs = rect.min.x + gw + 8.0 + (ss.saturating_sub(lsc)) as f32 * cw;
                        let mut xe = rect.min.x + gw + 8.0 + ((se - lsc).min(line.len_chars().saturating_sub(1))) as f32 * cw;
                        if se >= lec { xe += cw; }
                        painter.rect_filled(egui::Rect::from_min_max(egui::pos2(xs, scy), egui::pos2(xe.max(xs + 4.0), scy + rh)), 0.0, sel_bg);
                    }
                }
                painter.text(egui::pos2(rect.min.x + gw - 8.0, scy + 2.0), egui::Align2::RIGHT_TOP, (i + 1).to_string(), egui::FontId::monospace(11.0), C_MUTED);
                let line_str = line.to_string();
                let display_str = line_str.trim_end_matches(['\n', '\r']);
                let mut x_off = rect.min.x + gw + 8.0;
                for part in display_str.split_inclusive(|c: char| !c.is_alphanumeric() && c != '_') {
                    let clean = part.trim_end_matches(|c: char| !c.is_alphanumeric() && c != '_');
                    let col = if matches!(clean, "pub" | "fn" | "let" | "mut" | "use" | "mod" | "impl" | "trait" | "enum" | "match" | "if" | "else" | "for" | "while" | "loop" | "return" | "struct" | "type" | "where" | "async" | "await" | "move" | "ref" | "in" | "const" | "static" | "unsafe" | "extern" | "public" | "private" | "protected" | "internal" | "class" | "void" | "int" | "string" | "bool" | "using" | "namespace" | "override" | "virtual" | "new" | "foreach" | "case" | "switch" | "break" | "var") {
                        C_PURPLE
                    } else if clean.chars().next().is_some_and(|c| c.is_uppercase()) {
                        C_YELLOW
                    } else if matches!(clean, "true" | "false" | "null" | "self" | "Self" | "this" | "base" | "super") {
                        C_RED
                    } else {
                        C_TEXT
                    };
                    let g = ui.ctx().fonts_mut(|f| f.layout_no_wrap(part.to_owned(), font_id.clone(), col));
                    painter.galley(egui::pos2(x_off, scy + 2.0), g.clone(), col);
                    x_off += g.size().x;
                }
            }
        }
        if self.editor_focused && self.active_view == view_idx {
            let cl = self.buffers[bi].rope.char_to_line(cursor_idx);
            if cl >= f_row && cl <= l_row {
                let cc = cursor_idx - self.buffers[bi].rope.line_to_char(cl);
                let up = self.buffers[bi].rope.line(cl).to_string().chars().take(cc).collect::<String>();
                let g = ui.ctx().fonts_mut(|f| f.layout_no_wrap(up, font_id.clone(), C_TEXT));
                let cx = rect.min.x + gw + 8.0 + g.size().x;
                let c_scy = rect.min.y + (cl as f64 * rh as f64 - sy) as f32;
                if ui.input(|i| i.time) % 1.0_f64 < 0.5_f64 {
                    painter.rect_filled(egui::Rect::from_min_size(egui::pos2(cx, c_scy + 2.0), egui::vec2(2.0, rh - 4.0)), 0.0, C_BLUE);
                }
            }
        }
        if doc_h > rect.height() as f64 {
            let sw_v = 8.0_f32;
            let sr = egui::Rect::from_min_max(egui::pos2(rect.max.x - sw_v, rect.min.y), rect.max);
            painter.rect_filled(sr, 0.0, ui.visuals().extreme_bg_color);
            let hy = rect.min.y as f64 + (sy / max_s) * (rect.height() as f64 - hh);
            painter.rect_filled(egui::Rect::from_min_size(egui::pos2(rect.max.x - sw_v, hy as f32), egui::vec2(sw_v, hh as f32)), 2.0, ui.visuals().widgets.inactive.bg_fill);
        }
    }

    fn calc_char_pos(rope: &Rope, pos: egui::Pos2, rect: egui::Rect, sy: f64, gw: f32, rh: f32, cw: f32) -> usize {
        let cl = (((pos.y - rect.min.y) as f64 + sy) / rh as f64).floor() as usize;
        let cc = ((pos.x - (rect.min.x + gw + 8.0)).max(0.0) / cw).round() as usize;
        let cl = cl.clamp(0, rope.len_lines().saturating_sub(1));
        rope.line_to_char(cl) + cc.min(rope.line(cl).len_chars().saturating_sub(usize::from(cl < rope.len_lines() - 1)))
    }
}

impl eframe::App for ZordApp {
    // eframe 0.34 requires `ui` but uses `update` when overridden
    fn ui(&mut self, _ui: &mut egui::Ui, _frame: &mut eframe::Frame) {}

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_logic(ctx);
        ctx.request_repaint();

        if self.show_command_palette {
            egui::Window::new("CP").collapsible(false).resizable(false).title_bar(false).anchor(egui::Align2::CENTER_TOP, egui::vec2(0.0, 50.0)).fixed_size(egui::vec2(600.0, 40.0)).show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("🔍");
                    let resp = ui.add_sized([550.0, 24.0], egui::TextEdit::singleline(&mut self.command_palette_input).hint_text("Type..."));
                    resp.request_focus();
                    if resp.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) { self.show_command_palette = false; self.add_log(&format!("Cmd: {}", self.command_palette_input)); self.command_palette_input.clear(); }
                    if ui.input(|i| i.key_pressed(egui::Key::Escape)) { self.show_command_palette = false; }
                });
            });
        }

        // ── Status bar (bottom, must be before top panel) ─────────────────
        egui::TopBottomPanel::bottom("sb")
            .exact_height(26.0)
            .frame(egui::Frame::NONE.fill(C_MANTLE).inner_margin(egui::Margin::symmetric(0, 0)))
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    // Left accent badge (classic Code::Blocks style indicator)
                    let badge_rect = egui::Rect::from_min_size(ui.min_rect().min, egui::vec2(6.0, 26.0));
                    ui.painter().rect_filled(badge_rect, 0.0, C_BLUE);
                    ui.add_space(10.0);
                    ui.label(egui::RichText::new("ZORD").small().strong().color(C_BLUE));
                    ui.separator();
                    {
                        let av = self.active_view.min(self.views.len().saturating_sub(1));
                        let bi = self.views[av].buffer_idx.min(self.buffers.len().saturating_sub(1));
                        if let Some(file) = &self.buffers[bi].path {
                            let name = file.file_name().unwrap_or_default().to_string_lossy();
                            let label = if self.buffers[bi].unsaved_changes { format!("● {name}") } else { name.into_owned() };
                            ui.label(egui::RichText::new(label).small().color(C_TEXT));
                            ui.separator();
                            let ext = file.extension().and_then(|s| s.to_str()).unwrap_or("txt").to_uppercase();
                            ui.label(egui::RichText::new(ext).small().color(C_YELLOW));
                        }
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(8.0);
                        let av = self.active_view.min(self.views.len().saturating_sub(1));
                        let bi = self.views[av].buffer_idx.min(self.buffers.len().saturating_sub(1));
                        let cur = self.views[av].cursor_char_idx;
                        let cl = self.buffers[bi].rope.char_to_line(cur);
                        let col = cur - self.buffers[bi].rope.line_to_char(cl) + 1;
                        ui.label(egui::RichText::new(format!("Ln {}, Col {col}", cl + 1)).small().color(C_TEXT));
                        ui.separator();
                        ui.label(egui::RichText::new("UTF-8").small().color(C_MUTED));
                        ui.separator();
                        ui.label(egui::RichText::new("Spaces: 4").small().color(C_MUTED));
                    });
                });
            });

        // ── Menu bar ───────────────────────────────────────────────────────
        egui::TopBottomPanel::top("tp")
            .frame(egui::Frame::NONE.fill(C_MANTLE).inner_margin(egui::Margin::symmetric(4, 2)))
            .show(ctx, |ui| {
                egui::MenuBar::new().ui(ui, |ui| {
                    ui.menu_button("File", |ui| {
                        if ui.button("New File").clicked() { ui.close_kind(egui::UiKind::Menu); }
                        if ui.button("Refresh Explorer").clicked() { self.refresh_files(); self.explorer_cache.clear(); ui.close_kind(egui::UiKind::Menu); }
                        ui.separator();
                        if ui.button("Save         Ctrl+S").clicked() { self.save_file(); ui.close_kind(egui::UiKind::Menu); }
                    });
                    ui.menu_button("Edit", |ui| {
                        if ui.button("Undo         Ctrl+Z").clicked() { self.undo(); ui.close_kind(egui::UiKind::Menu); }
                        if ui.button("Redo         Ctrl+Y").clicked() { self.redo(); ui.close_kind(egui::UiKind::Menu); }
                        ui.separator();
                        if ui.button("Find         Ctrl+F").clicked() { self.show_find_bar = true; ui.close_kind(egui::UiKind::Menu); }
                    });
                    ui.menu_button("View", |ui| {
                        if ui.button("Split Editor").clicked() {
                            // New split pane gets its own fresh buffer so two files can be open side-by-side
                            let new_buf_idx = self.buffers.len();
                            self.buffers.push(Buffer::new(Rope::from_str("")));
                            self.views.push(EditorView::for_buffer(new_buf_idx));
                            ui.close_kind(egui::UiKind::Menu);
                        }
                        ui.separator();
                        if ui.button(if self.show_bottom_panel { "Hide Output   Ctrl+J" } else { "Show Output   Ctrl+J" }).clicked() { self.show_bottom_panel = !self.show_bottom_panel; ui.close_kind(egui::UiKind::Menu); }
                        if ui.button(if self.show_right_panel { "Hide AI Panel Ctrl+B" } else { "Show AI Panel Ctrl+B" }).clicked() { self.show_right_panel = !self.show_right_panel; ui.close_kind(egui::UiKind::Menu); }
                    });
                    ui.menu_button("Build", |ui| {
                        if ui.button("Build Project  F7").clicked() { self.add_log("Build not wired yet."); ui.close_kind(egui::UiKind::Menu); }
                        if ui.button("Run            F5").clicked() { self.add_log("Run not wired yet."); ui.close_kind(egui::UiKind::Menu); }
                    });
                    if self.loading_file {
                        ui.spinner();
                        ui.label(egui::RichText::new("Loading…").small().color(C_BLUE));
                    }
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(egui::RichText::new(format!("⚡ {:.0} fps", self.internal_fps)).small().color(C_MUTED));
                    });
                });
            });

        // ── Toolbar ────────────────────────────────────────────────────────
        egui::TopBottomPanel::top("toolbar")
            .exact_height(34.0)
            .frame(egui::Frame::NONE.fill(C_SURFACE).inner_margin(egui::Margin::symmetric(6, 4)))
            .show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    ui.spacing_mut().item_spacing.x = 2.0;
                    let btn = |label: &str| egui::Button::new(egui::RichText::new(label).size(12.0)).min_size(egui::vec2(0.0, 22.0)).frame(true);
                    let sep = |ui: &mut egui::Ui| { ui.add_space(3.0); ui.separator(); ui.add_space(3.0); };

                    if ui.add(btn("New")).clicked()   { self.add_log("New file — coming soon."); }
                    if ui.add(btn("Open")).clicked()  { self.add_log("Open — coming soon."); }
                    if ui.add(btn("Save")).clicked()  { self.save_file(); }
                    sep(ui);
                    if ui.add(btn("Undo")).clicked()  { self.undo(); }
                    if ui.add(btn("Redo")).clicked()  { self.redo(); }
                    sep(ui);
                    if ui.add(btn("Find")).clicked()  { self.show_find_bar = !self.show_find_bar; }
                    sep(ui);
                    if ui.add(btn("Split")).clicked() {
                        let new_buf_idx = self.buffers.len();
                        self.buffers.push(Buffer::new(Rope::from_str("")));
                        self.views.push(EditorView::for_buffer(new_buf_idx));
                    }
                    if self.views.len() > 1 && ui.add(btn("Merge")).clicked() {
                        let removed = self.views.pop();
                        // remove orphaned buffer if no other view references it
                        if let Some(rv) = removed {
                            let bi = rv.buffer_idx;
                            if !self.views.iter().any(|v| v.buffer_idx == bi) && bi > 0 {
                                self.buffers.remove(bi);
                                // fix up indices of views pointing past the removed slot
                                for v in &mut self.views { if v.buffer_idx > bi { v.buffer_idx -= 1; } }
                            }
                        }
                        self.active_view = self.active_view.min(self.views.len().saturating_sub(1));
                    }
                    sep(ui);
                    if ui.add(btn("Build")).clicked() { self.add_log("Build not wired yet."); }
                    if ui.add(btn("Run")).clicked()   { self.add_log("Run not wired yet."); }
                });
            });

        // ── Terminal panel (bottom) ─────────────────────────────────────────
        if self.show_bottom_panel {
            egui::TopBottomPanel::bottom("bp")
                .resizable(true)
                .default_height(160.0)
                .frame(egui::Frame::NONE.fill(C_MANTLE).inner_margin(egui::Margin::symmetric(8, 4)))
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new("OUTPUT").small().strong().color(C_MUTED));
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.small_button("✕").clicked() { self.logs.clear(); }
                        });
                    });
                    ui.add_space(2.0);
                    egui::ScrollArea::vertical().id_salt("ls").stick_to_bottom(true).show(ui, |ui| {
                        for log in &self.logs {
                            let color = if log.contains("ERROR") || log.contains("failed") { C_RED }
                                else if log.contains("Saved") || log.contains("Loaded") { C_GREEN }
                                else { C_MUTED };
                            ui.label(egui::RichText::new(log).monospace().size(11.5).color(color));
                        }
                    });
                });
        }

        // ── File explorer (left) ────────────────────────────────────────────
        egui::SidePanel::left("lp")
            .resizable(true)
            .default_width(220.0)
            .frame(egui::Frame::NONE.fill(C_SURFACE).inner_margin(egui::Margin::same(0)))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    egui::Frame::NONE
                        .fill(C_MANTLE)
                        .inner_margin(egui::Margin::symmetric(12, 6))
                        .show(ui, |ui| {
                            ui.label(egui::RichText::new("PROJECT").size(10.5).strong().color(C_MUTED));
                        });
                    let cache_len = self.explorer_cache.len();
                    let root = self.project_root.clone();
                    let av = self.active_view.min(self.views.len().saturating_sub(1));
                    let current: Option<PathBuf> = self.buffers[self.views[av].buffer_idx.min(self.buffers.len().saturating_sub(1))].path.clone();
                    egui::ScrollArea::vertical().id_salt("es").show_rows(ui, 22.0, cache_len, |ui, range| {
                        let mut to_load: Option<PathBuf> = None;
                        for i in range {
                            let p = &self.explorer_cache[i];
                            let rel = p.strip_prefix(&root).unwrap_or(p);
                            let depth = rel.components().count().saturating_sub(1);
                            let indent = depth as f32 * 12.0;
                            let is_current = current.as_deref() == Some(p.as_path());
                            let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("");
                            let icon = match ext {
                                "rs"                    => "rs",
                                "cs"                    => "cs",
                                "js"                    => "js",
                                "ts"                    => "ts",
                                "cpp" | "cc" | "cxx"    => "c+",
                                "h" | "hpp"             => "h ",
                                "json"                  => "{}",
                                "toml" | "yaml" | "yml" => "cf",
                                "md"                    => "md",
                                "txt"                   => "tx",
                                _                       => "  ",
                            };
                            let file_name = rel.file_name().map(|f| f.to_string_lossy().into_owned()).unwrap_or_default();
                            let label_text = egui::RichText::new(format!("{icon} {file_name}"))
                                .size(12.5)
                                .color(if is_current { C_BLUE } else { C_TEXT });
                            let path_clone = p.clone();
                            let hover_str = p.display().to_string();
                            ui.horizontal(|ui| {
                                ui.add_space(8.0 + indent);
                                let resp = ui.selectable_label(is_current, label_text);
                                if resp.clicked() { to_load = Some(path_clone); }
                                resp.on_hover_text(hover_str);
                            });
                        }
                        if let Some(path) = to_load { self.load_file_async(path); }
                    });
                });
            });

        // ── AI panel (right) ────────────────────────────────────────────────
        if self.show_right_panel {
            egui::SidePanel::right("rp")
                .resizable(true)
                .default_width(260.0)
                .frame(egui::Frame::NONE.fill(egui::Color32::from_rgb(33, 37, 43)).inner_margin(egui::Margin::same(12)))
                .show(ctx, |ui| {
                    ui.label(egui::RichText::new("AI ASSISTANT").size(10.5).strong().color(egui::Color32::from_rgb(92, 99, 112)));
                    ui.add_space(8.0);
                    ui.label(egui::RichText::new("Coming soon…").color(egui::Color32::from_rgb(92, 99, 112)));
                });
        }

        // ── Central editor area ─────────────────────────────────────────────
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE.fill(C_BASE))
            .show(ctx, |ui| {

        // ── Tab strip ──────────────────────────────────────────────────────
        egui::Frame::NONE.fill(C_MANTLE).show(ui, |ui| {
            ui.set_min_height(30.0);
            ui.horizontal(|ui| {
                let mut any_tab = false;
                for (vi, view) in self.views.iter().enumerate() {
                    let bi = view.buffer_idx.min(self.buffers.len().saturating_sub(1));
                    if let Some(file) = &self.buffers[bi].path {
                        any_tab = true;
                        let name = file.file_name().unwrap_or_default().to_string_lossy();
                        let is_active = vi == self.active_view;
                        let label = if self.buffers[bi].unsaved_changes { format!("  ● {name}  ") } else { format!("  {name}  ") };
                        let tab_rect = ui.available_rect_before_wrap();
                        let fill = if is_active { C_BASE } else { C_SURFACE };
                        egui::Frame::NONE.fill(fill).inner_margin(egui::Margin::symmetric(8, 5)).show(ui, |ui| {
                            let r = ui.label(egui::RichText::new(&label).size(12.5).color(if is_active { C_TEXT } else { C_MUTED }));
                            if r.clicked() { /* already active or click switches focus — handled by render_editor */ }
                        });
                        if is_active {
                            let r = ui.min_rect();
                            let tab_top = egui::Rect::from_min_max(egui::pos2(tab_rect.min.x, r.min.y), egui::pos2(r.max.x, r.min.y + 2.0));
                            ui.painter().rect_filled(tab_top, 0.0, C_BLUE);
                        }
                    }
                }
                if !any_tab {
                    ui.label(egui::RichText::new("  No file open  ").size(12.5).color(C_MUTED));
                }
            });
        });

        // Editor view(s)
        let vc = self.views.len();
        if vc == 0 {
            ui.centered_and_justified(|ui| {
                ui.label(egui::RichText::new("Open a file from the explorer").color(C_MUTED));
            });
        } else if vc == 1 {
            self.render_editor(ui, 0);
        } else {
            // Fluid drag-handle split: left pane | 5px handle | right pane
            // For >2 views, first pane is left, all others stack in the right half
            let avail = ui.available_rect_before_wrap();
            let handle_w = 5.0_f32;
            let total_w = avail.width();
            let left_w = (total_w * self.split_ratio).floor();
            let right_w = total_w - left_w - handle_w;

            let left_rect  = egui::Rect::from_min_size(avail.min, egui::vec2(left_w, avail.height()));
            let handle_rect = egui::Rect::from_min_size(egui::pos2(avail.min.x + left_w, avail.min.y), egui::vec2(handle_w, avail.height()));
            let right_rect = egui::Rect::from_min_size(egui::pos2(avail.min.x + left_w + handle_w, avail.min.y), egui::vec2(right_w, avail.height()));

            // drag handle interaction
            let handle_id = ui.id().with("split_handle");
            let handle_resp = ui.interact(handle_rect, handle_id, egui::Sense::click_and_drag());
            if handle_resp.drag_started() { self.dragging_split = true; }
            if handle_resp.drag_stopped()  { self.dragging_split = false; }
            if self.dragging_split {
                let dx = handle_resp.drag_delta().x;
                self.split_ratio = ((self.split_ratio * total_w + dx) / total_w).clamp(0.15, 0.85);
            }
            let handle_color = if handle_resp.hovered() || self.dragging_split { C_BLUE } else { C_OVERLAY };
            ui.painter().rect_filled(handle_rect, 0.0, handle_color);
            if handle_resp.hovered() || self.dragging_split {
                ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeHorizontal);
            }

            // Left pane — view 0
            {
                let mut child = ui.new_child(egui::UiBuilder::new().max_rect(left_rect));
                self.render_editor(&mut child, 0);
            }

            // Right pane(s) — remaining views stacked vertically
            let right_count = vc - 1;
            let pane_h = avail.height() / right_count as f32;
            for i in 0..right_count {
                let pane_rect = egui::Rect::from_min_size(
                    egui::pos2(right_rect.min.x, right_rect.min.y + i as f32 * pane_h),
                    egui::vec2(right_w, pane_h),
                );
                let mut child = ui.new_child(egui::UiBuilder::new().max_rect(pane_rect));
                self.render_editor(&mut child, i + 1);
                if i + 1 < right_count {
                    // thin separator between stacked right panes
                    let sep_y = pane_rect.max.y;
                    ui.painter().hline(right_rect.min.x..=right_rect.max.x, sep_y, egui::Stroke::new(1.0_f32, C_OVERLAY));
                }
            }
            // allocate the full area so egui doesn't complain about unused space
            ui.allocate_rect(avail, egui::Sense::hover());
        }
        }); // close CentralPanel
    }
}
