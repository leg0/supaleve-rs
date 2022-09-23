use std::path::PathBuf;
use egui::TopBottomPanel;
use egui_file::FileDialog;

enum DialogType {
    Open, Save
}

/// The top panel is responsible for the menu bar, and the file dialogs for selecting the
/// file to load or save as.
pub struct TopPanel {
    file_dialog: Option<FileDialog>,
    dialog_type: Option<DialogType>,
}

impl TopPanel {
    pub fn new() -> Self {
        Self {
            file_dialog: None,
            dialog_type: None
        }
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        use egui::menu;
        TopBottomPanel::top("top panel")
            .show(ctx, |ui| {
                menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New").clicked() { todo!(); }
                    if ui.button("Open...").clicked() { self.on_open(ui); }
                    if ui.button("Save").clicked() { todo!(); }
                    if ui.button("Save As...").clicked() { self.on_save_as(ui); }
                    ui.separator();
                    if ui.button("Exit").clicked() { todo!(); }
                });
            });
        });
        if let Some(ref mut dlg) = self.file_dialog {
            if dlg.visible() {
                dlg.show(ctx);
            }
        }
    }

    /// Indicates whether "Open..." menu item was selected.
    pub fn open_selected(&self) -> bool {
        matches!(self.dialog_type, Some(DialogType::Open))
    }

    /// Indicates whether "Save as..." menu item was selected.
    pub fn save_as_selected(&self) -> bool {
        matches!(self.dialog_type, Some(DialogType::Save))
    }

    /// Indicates whether a path was selected using the file dialog.
    pub fn path_selected(&self) -> bool {
        match &self.file_dialog {
            Some(dlg) => dlg.selected(),
            _ => false
        }
    }

    /// Returns the path selected using the file dialog, or None.
    pub fn path(&self) -> Option<PathBuf> {
        match &self.file_dialog {
            Some(dlg) => dlg.path(),
            _ => None
        }
    }

    /// Close the dialog so that it's not available when next frame is drawn.
    pub fn close_dialog(&mut self) {
        
        self.file_dialog = None;
        self.dialog_type = None;
    }

    fn on_save_as(&mut self, ui: &mut egui::Ui) {
        let mut dlg = FileDialog::save_file(None).show_rename(false);
        dlg.open();
        self.file_dialog = Some(dlg);
        self.dialog_type = Some(DialogType::Save);
        ui.close_menu();
    }

    fn on_open(&mut self, ui: &mut egui::Ui) {
        let mut dlg = FileDialog::open_file(None).show_new_folder(false).show_rename(false);//.filter(String::from("*.dat"));
        dlg.open();
        self.file_dialog = Some(dlg);
        self.dialog_type = Some(DialogType::Open);
        ui.close_menu();
    }
}
