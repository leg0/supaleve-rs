#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui;

mod tool_panel;
use tool_panel::ToolPanel;

mod status_panel;
use status_panel::StatusPanel;

mod editor_panel;
use editor_panel::EditorPanel;

mod top_panel;
use top_panel::TopPanel;

mod images;

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.fullscreen = false;

    eframe::run_native(
        "Supaplex Level Editor",
        options,
        Box::new(|_cc| Box::new(SupaleveApp::default())),
    );
}


struct SupaleveApp {
    tool_panel: ToolPanel,
    status_panel: StatusPanel,
    editor_panel: EditorPanel,
    top_panel: TopPanel,
}

impl Default for SupaleveApp {
    fn default() -> Self {
        Self {
            tool_panel: ToolPanel::new("Tools", 120.0),
            status_panel: StatusPanel::new(120.0),
            editor_panel: EditorPanel::new("Supaplex level editor"),
            top_panel: TopPanel::new(),
        }
    }
}

impl eframe::App for SupaleveApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        
        self.top_panel.update(ctx);
        if self.top_panel.open_selected() {
            if self.top_panel.path_selected() {
                //todo!("Open: path: {:?}", self.top_panel.path());
                self.top_panel.close_dialog();
            }
        }
        else if self.top_panel.save_as_selected() {
            if self.top_panel.path_selected() {
                //todo!("Save as: path: {:?}", self.top_panel.path());
                self.top_panel.close_dialog();
            }
        }
        else {
            self.tool_panel.update(ctx, frame);
            self.status_panel.update(ctx, frame);
            self.editor_panel.update(ctx, frame, &self.tool_panel);
        }
    }
}
