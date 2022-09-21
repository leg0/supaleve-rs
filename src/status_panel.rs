pub struct StatusPanel
{
    height: f32
}


impl StatusPanel {
    pub fn new(height: f32) -> Self {
        Self { height }
    }

    pub fn update(&self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::new(egui::panel::TopBottomSide::Bottom, "status").show(ctx, |ui| {
            ui.heading("Status");
            ui.set_height(self.height);
        });
    }
}