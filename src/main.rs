#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::{self};

mod tool_panel;
use tool_panel::ToolPanel;

mod status_panel;
use status_panel::StatusPanel;

fn main() {
    let options = eframe::NativeOptions::default();
    //options.fullscreen = true;

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
    name: String,
    age: u32,
    tool_panel: ToolPanel,
    status_panel: StatusPanel,
}

impl Default for SupaleveApp {
    fn default() -> Self {
        Self {
            name: "lego".to_owned(),
            age: 42,
            tool_panel: ToolPanel::new("Tools", 120.0),
            status_panel: StatusPanel::new()
        }
    }
}

impl eframe::App for SupaleveApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.tool_panel.update(ctx, frame);

        self.status_panel.update(ctx, frame);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Supaplex level editor");
            ui.horizontal(|ui| {
                ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}
