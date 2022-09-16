// Tool panel contains all the tiles that can be placed in the play area. Also indicates 
// which tool is currently selected

use egui::{panel::Side, ImageButton, vec2, TextureId};
use egui_extras::RetainedImage;

#[derive(Copy, Clone, Debug)]
enum Tool {
    //Empty,
    Zonk,
    Electron,
    //Murphy
}

const TILE_IMAGES:[&str; 40] = [
    "empty.png",
    "zonk.png",
    "base.png",
    "murphy.png",
    "info.png",
    "ram.png",
    "hw8.png",
    "exit.png",
    "disk-orange.png",
    "port-lr.png",
    "port-ud.png",
    "port-rl.png",
    "port-du.png",
    "gport-lr.png",
    "gport-ud.png",
    "gport-rl.png",
    "gport-du.png",
    "ssnak.png",
    "disk-yellow.png",
    "terminal.png",
    "disk-red.png",
    "port-v.png",
    "port-h.png",
    "port-x.png",
    "electron.png",
    "bug.png",
    "ramleft.png",
    "ramright.png",
    "hw1.png",
    "hw2.png",
    "hw3.png",
    "hw4.png",
    "hw5.png",
    "hw6.png",
    "hw7.png",
    "hw8.png",
    "hw9.png",
    "hw10.png",
    "ramtop.png",
    "rambottom.png",
];

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

            let mut btn = ImageButton::new(TextureId::default(), vec2(40.0, 40.0));
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

    pub fn load_images(&mut self) {
        for name in TILE_IMAGES {
            let image = RetainedImage::from_image_bytes(
                "../img/zonk.png", include_bytes!("../img/zonk.png") );
        }
    }
}