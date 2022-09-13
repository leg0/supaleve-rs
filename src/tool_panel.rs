// Tool panel contains all the tiles that can be placed in the play area. Also indicates 
// which tool is currently selected

use egui::{panel::Side, ImageButton, vec2, TextureId};

#[derive(Copy, Clone, Debug)]
enum Tool {
    //Empty,
    Zonk,
    Electron,
    //Murphy
}

pub struct ToolPanel
{
    heading: String,
    width: f32,
    //tools: [Tool; 10], // TODO: appropriate size
    selected_tool: Option<Tool>
}

impl ToolPanel {
    pub fn new(heading: &str, width: f32) -> Self {
        Self {
            heading: heading.to_owned(),
            width,
            //tools: [Tool::Empty; 10],
            selected_tool: None
        }
    }

    pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::new(Side::Left, "tool_panel").show(ctx, |ui| {
            ui.heading(&self.heading);
            ui.set_width(self.width);

            let mut btn = ImageButton::new(TextureId::default(), vec2(10.0, 10.0));
            btn = btn.selected(matches!(self.selected_tool, Some(Tool::Electron)));
            if ui.add(btn).clicked() {
                self.selected_tool = if let Some(Tool::Electron) = self.selected_tool {None} else { Some(Tool::Electron) };
                println!("{:?}", self.selected_tool);
            }

            let r = ui.selectable_label(matches!(self.selected_tool, Some(Tool::Zonk)), "Zonk");
            if r.clicked() {
                self.selected_tool = if let Some(Tool::Zonk) = self.selected_tool {None}else{ Some(Tool::Zonk) };
                println!("{:?}", self.selected_tool);
            }
        });
    }
}