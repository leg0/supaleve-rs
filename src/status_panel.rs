pub struct StatusPanel
{

}


impl StatusPanel {
    pub fn new() -> Self {
        Self {}
    }

    pub fn update(&self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::new(egui::panel::TopBottomSide::Bottom, "status").show(ctx, |ui| {
            ui.heading("Status");
            ui.set_height(100.0);
        });
    }
}