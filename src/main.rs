#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use eframe::egui;

mod tool_panel;
use egui_file::FileDialog;
use tool_panel::ToolPanel;

mod status_panel;
use status_panel::StatusPanel;

mod editor_panel;
use editor_panel::EditorPanel;

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
    open_file_dialog: FileDialog,
    save_file_dialog: FileDialog,
}

impl Default for SupaleveApp {
    fn default() -> Self {
        Self {
            tool_panel: ToolPanel::new("Tools", 120.0),
            status_panel: StatusPanel::new(120.0),
            editor_panel: EditorPanel::new("Supaplex level editor"),
            open_file_dialog: FileDialog::open_file(None).show_new_folder(false).show_rename(false).filter(String::from("*.dat")),
            save_file_dialog: FileDialog::save_file(None).show_rename(false),
        }
    }
}

fn show_menu(ui: &mut egui::Ui, open_file_dialog: &mut FileDialog, save_file_dialog: &mut FileDialog) {
    use egui::menu;
    menu::bar(ui, |ui| {
        ui.menu_button("File", |ui| {
            if ui.button("New").clicked() { }
            if ui.button("Open...").clicked() {
                open_file_dialog.open();
                ui.close_menu();
            }
            if ui.button("Save").clicked() { }
            if ui.button("Save As...").clicked() {
                save_file_dialog.open();
                ui.close_menu();
             }
            ui.separator();
            if ui.button("Exit").clicked() {
                
            }
        });
    });
}    

impl eframe::App for SupaleveApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        
        if self.open_file_dialog.visible() {
            self.open_file_dialog.show(ctx);
            if self.open_file_dialog.selected() {
                println!("Selected {:?}", self.open_file_dialog.path());
                // TODO: open selected levels file.
            }
        }
        else if self.save_file_dialog.visible() {
            self.save_file_dialog.show(ctx);
            if self.save_file_dialog.selected() {
                println!("Selected {:?}", self.save_file_dialog.path());
                // TODO: save to specified file.
            }
        }
        else {
            egui::TopBottomPanel::top("top panel")
                .show(ctx, |ui| show_menu(ui, &mut self.open_file_dialog, &mut self.save_file_dialog));
            self.tool_panel.update(ctx, frame);
            self.status_panel.update(ctx, frame);
            self.editor_panel.update(ctx, frame, &self.tool_panel);
        }
    }
}
