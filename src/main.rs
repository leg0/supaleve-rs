#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::{self, panel::Side};
//use std::io;
use std::path::Path;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Supaplex Level Editor",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}

enum Tile {
    Empty,
    Base,
    Bug,
    Zonk,
    Murphy,
    Electron,
    SnikSnak,
    Chip,
    Wall
    // ..
}

struct GravityPort
{
    x: u8,
    y: u8,
    enable_gravity: bool,
    freeze_enemies: bool,
    freeze_zonks: bool
}

struct Level<'a> {
    name: String,
    number: u8,
    play_area: [(Tile, &'a Path);60*24],
    required_infotrons: u8,
    has_gravity: bool,
    enemies_frozen: bool,
    gravity_ports: [Option<GravityPort>; 10],
}

enum Command {
    Draw,
    DrawRect,
    Clear
}

struct Editor<'a> {
    file_name: String,
    all_levels: Vec<Level<'a>>,
    active_level: u8, // index into all_levels
    is_saved: bool,
    undo_stack: Vec<Command>,
    redo_stack: Vec<Command>
}

struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "lego".to_owned(),
            age: 42,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::new(Side::Left, "tool_panel").show(ctx, |ui| {
            ui.heading("Tools");
            ui.set_width(100.0);
        });

        egui::TopBottomPanel::new(egui::panel::TopBottomSide::Bottom, "status").show(ctx, |ui| {
            ui.heading("Status");
            ui.set_height(100.0);
        });
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
