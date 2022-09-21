#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use eframe::egui::{self};

mod tool_panel;
use tool_panel::ToolPanel;

mod status_panel;
use status_panel::StatusPanel;

mod editor_panel;
use editor_panel::EditorPanel;

mod images;

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.fullscreen = true;

    eframe::run_native(
        "Supaplex Level Editor",
        options,
        Box::new(|_cc| Box::new(SupaleveApp::default())),
    );
}

// struct GravityPort
// {
//     x: u8,
//     y: u8,
//     enable_gravity: bool,
//     freeze_enemies: bool,
//     freeze_zonks: bool
// }

// struct Level<'a> {
//     name: String,
//     number: u8,
//     play_area: [(Tile, &'a Path);60*24],
//     required_infotrons: u8,
//     has_gravity: bool,
//     enemies_frozen: bool,
//     gravity_ports: [Option<GravityPort>; 10],
// }

// enum Command {
//     Draw,
//     DrawRect,
//     Clear
// }

// struct Editor<'a> {
//     file_name: String,
//     all_levels: Vec<Level<'a>>,
//     active_level: u8, // index into all_levels
//     is_saved: bool,
//     undo_stack: Vec<Command>,
//     redo_stack: Vec<Command>
// }

struct SupaleveApp {
    tool_panel: ToolPanel,
    status_panel: StatusPanel,
    editor_panel: EditorPanel,
}

impl Default for SupaleveApp {
    fn default() -> Self {
        Self {
            tool_panel: ToolPanel::new("Tools", 120.0),
            status_panel: StatusPanel::new(120.0),
            editor_panel: EditorPanel::new("Supaplex level editor")
        }
    }
}

impl eframe::App for SupaleveApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.tool_panel.update(ctx, frame);
        self.status_panel.update(ctx, frame);
        self.editor_panel.update(ctx, frame, &self.tool_panel);
    }
}
