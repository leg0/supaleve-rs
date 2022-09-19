// Tool panel contains all the tiles that can be placed in the play area. Also indicates 
// which tool is currently selected

use std::ops::Index;

use egui::{panel::Side, ImageButton, vec2, Layout, Ui, Align, Context, ScrollArea};
use egui_extras::RetainedImage;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Tile {
    Empty = 0,
    Zonk = 1,
    Base = 2,
    Murphy = 3,
    Infotron = 4,
    RamChip = 5,
    Wall = 6,
    Exit = 7,
    FloppyOrange = 8,
    PortRight = 9,
    PortDown = 10,
    PortLeft = 11,
    PortUp = 12,
    GravityPortRight = 13,
    GravityPortDown = 14,
    GravityPortLeft = 15,
    GravityPortUp = 16,
    SnikSnak = 17,
    FloppyYellow = 18,
    Terminal = 19,
    FloppyRed = 20,
    Port2WayVertical = 21,
    Port2WayHorizontal = 22,
    Port4Way = 23,
    Electron = 24,
    Bug = 25,
    RamLeft = 26,
    RamRight = 27,
    Hardware1 = 28,
    Hardware2 = 29,
    Hardware3 = 30,
    Hardware4 = 31,
    Hardware5 = 32,
    Hardware6 = 33,
    Hardware7 = 34,
    Hardware8 = 35,
    Hardware9 = 36,
    Hardware10 = 37,
    RamTop = 38,
    RamBottom = 39,
}

pub struct Tool {
    tile1: Tile,
    tile2: Option<Tile>,
    size: (u8, u8),
    image1: RetainedImage,
    image2: Option<RetainedImage>,
}

impl Tool {
    pub fn new(tile: Tile) -> Self {
        Self {
            tile1: tile,
            tile2: None,
            size: (1, 1),
            image1: Self::load_tile_image(tile),
            image2: None
        }
    }

    pub fn new1x2(left_tile: Tile, right_tile: Tile) -> Self {
        Self::new_n_by_n((left_tile, right_tile), (1, 2))
    }

    pub fn new2x1(top_tile: Tile, bottom_tile: Tile) -> Self {
        Self::new_n_by_n((top_tile, bottom_tile), (2, 1))
        
    }

    fn load_tile_image(tile: Tile) -> RetainedImage {

        let image_path = format!("img/{}", TILE_IMAGES[tile as usize]);
        println!("Loading tile image {tile:?} from {image_path}");
        let image_bytes = std::fs::read(image_path).unwrap();
        RetainedImage::from_image_bytes(format!("{:?}", tile), &image_bytes).unwrap()
    }

    fn new_n_by_n(tile: (Tile, Tile), size: (u8, u8)) -> Self {
        Self {
            tile1: tile.0,
            tile2: Some(tile.1),
            size,
            image1: Self::load_tile_image(tile.0),
            image2: Some(Self::load_tile_image(tile.1))
        }
    }
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
    "ramh.png",
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
    "ramv.png",
    "rambottom.png",
];

pub struct ToolPanel
{
    heading: String,
    width: f32,
    tools: Vec<Tool>,
    selected_tool: Option<usize>
}

impl ToolPanel {
    pub fn new(heading: &str, width: f32) -> Self {
        Self {
            heading: heading.to_owned(),
            width,
            tools: vec![
                Tool::new(Tile::Empty),
                Tool::new(Tile::Zonk),
                Tool::new(Tile::Base),
                Tool::new(Tile::Bug),
                Tool::new(Tile::Murphy),
                Tool::new(Tile::Infotron),
                Tool::new(Tile::Exit),
                Tool::new(Tile::PortRight),
                Tool::new(Tile::PortDown),
                Tool::new(Tile::PortLeft),
                Tool::new(Tile::PortUp),
                Tool::new(Tile::GravityPortRight),
                Tool::new(Tile::GravityPortDown),
                Tool::new(Tile::GravityPortLeft),
                Tool::new(Tile::GravityPortUp),
                Tool::new(Tile::Port2WayVertical),
                Tool::new(Tile::Port2WayHorizontal),
                Tool::new(Tile::Port4Way),
                Tool::new(Tile::FloppyOrange),
                Tool::new(Tile::SnikSnak),
                Tool::new(Tile::FloppyRed),
                Tool::new(Tile::Electron),
                Tool::new(Tile::FloppyYellow),
                Tool::new(Tile::Terminal),
                Tool::new2x1(Tile::RamLeft, Tile::RamRight),
                Tool::new1x2(Tile::RamTop, Tile::RamBottom),
                Tool::new(Tile::RamChip),
                Tool::new(Tile::Wall),
                Tool::new(Tile::Hardware1),
                Tool::new(Tile::Hardware2),
                Tool::new(Tile::Hardware3),
                Tool::new(Tile::Hardware4),
                Tool::new(Tile::Hardware5),
                Tool::new(Tile::Hardware6),
                Tool::new(Tile::Hardware7),
                Tool::new(Tile::Hardware8),
                Tool::new(Tile::Hardware9),
                Tool::new(Tile::Hardware10),
            ],
            selected_tool: None
        }
    }

    pub fn update(& mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("tool_panel")
            .default_width(self.width)
            .show(ctx, |ui| {
                ui.heading(&self.heading);
                ui.set_width(self.width);
                ScrollArea::vertical().show(ui, |ui| {
                    let layout = Layout::left_to_right(Align::Min).with_main_wrap(true);
                    ui.with_layout(layout, |ui| {
                        self.draw_buttons(ctx, ui);
                    });
                });
            });
    }

    fn draw_buttons(&mut self, ctx: &Context, ui: &mut Ui) {
        self.tools.iter().enumerate().for_each(|(tool_idx, tool)| {
            let button_size = match tool.size {
                (2, _) => vec2(80., 40.),
                (_, 2) => vec2(40., 80.),
                _ => vec2(40., 40.)
            };

            let mut btn = ImageButton::new(tool.image1.texture_id(ctx), button_size);
            let tile = tool.tile1;

            let is_selected = match self.selected_tool {
                Some(idx) => self.tools[idx].tile1 == tile,
                None => false,
            };
            btn = btn.selected(is_selected);
            if ui.add(btn).clicked() {
                self.selected_tool = Some(tool_idx);
            }
        });
    }

}