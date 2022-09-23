// Tool panel contains all the tiles that can be placed in the play area. Also indicates 
// which tool is currently selected

use std::rc::Rc;

use egui::{ImageButton, vec2, Layout, Ui, Align, Context, ScrollArea};
use egui_extras::RetainedImage;

use crate::images::{Images};

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

struct DrawingTool {
    tile1: Tile,
    size: (u8, u8),
    image1: Rc<RetainedImage>,
}

impl DrawingTool {
    pub fn new(tile: Tile, tool_panel: &ToolPanel) -> Self {
        Self {
            tile1: tile,
            size: (1, 1),
            image1: tool_panel.images[tile].clone(),
        }
    }

    fn new_n_by_n(tile: (Tile, Tile), size: (u8, u8), tool_panel: &ToolPanel) -> Self {
        Self {
            tile1: tile.0,
            size,
            image1: tool_panel.images[tile.0].clone(),
        }
    }

    pub fn tile(&self) -> Tile {
        self.tile1
    }
}

#[derive(Copy, Clone, Debug)]
pub enum OperatingMode {
    Select, Draw
}

pub struct ToolPanel
{
    heading: String,
    width: f32,
    drawing_tools: Vec<DrawingTool>,
    selected_tool: usize,
    images: Images,
    operating_mode: OperatingMode,
}

impl ToolPanel {

    pub fn new(heading: &str, width: f32) -> Self {
        let mut res = Self {
            heading: heading.to_owned(),
            width,
            drawing_tools: vec![],
            selected_tool: 0,
            images: Images::new(),
            operating_mode: OperatingMode::Draw
        };
        res.drawing_tools = res.make_tools();
        res
    }

    pub fn update(& mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("tool_panel")
            .default_width(self.width)
            .show(ctx, |ui| {
                ui.heading(&self.heading);
                ui.set_width(self.width);

                let r1 = egui::RadioButton::new(matches!(self.operating_mode, OperatingMode::Draw), "Draw");
                if ui.add(r1).clicked() {
                    self.operating_mode = OperatingMode::Draw;
                }

                let r2 = egui::RadioButton::new(matches!(self.operating_mode, OperatingMode::Select), "Select");
                if ui.add(r2).clicked() {
                    self.operating_mode = OperatingMode::Select;
                }

                if matches!(self.operating_mode, OperatingMode::Draw) {
                    self.show_drawing_tools(ui, ctx);
                }
                else if matches!(self.operating_mode, OperatingMode::Select) {

                }
            });
    }

    pub fn operating_mode(&self) -> OperatingMode {
        self.operating_mode
    }

    fn show_drawing_tools(& mut self, ui: &mut Ui, ctx: &Context) {
        ScrollArea::vertical().show(ui, |ui| {
            let layout = Layout::left_to_right(Align::Min).with_main_wrap(true);
            ui.with_layout(layout, |ui| {
                self.create_buttons(ctx, ui);
            });
        });
    }

    fn create_buttons(&mut self, ctx: &Context, ui: &mut Ui) {
        self.drawing_tools.iter().enumerate().for_each(|(tool_idx, tool)| {
            let button_size = match tool.size {
                (2, _) => vec2(64., 32.),
                (_, 2) => vec2(32., 64.),
                _ => vec2(32., 32.)
            };

            let mut btn = ImageButton::new(tool.image1.texture_id(ctx), button_size);
            let tile = tool.tile1;

            let is_selected =self.drawing_tools[self.selected_tool].tile1 == tile;
            btn = btn.selected(is_selected);
            if ui.add(btn).clicked() {
                self.selected_tool = tool_idx;
            }
        });
    }

    pub fn selected_tool_tile(&self) -> Tile {
        self.drawing_tools[self.selected_tool].tile()
    }

    fn new_tool(&self, tile: Tile) -> DrawingTool {
        DrawingTool::new(tile, self)
    }

    fn new_tool2(&self, tile1: Tile, tile2: Tile, size: (u8,u8)) -> DrawingTool {
        DrawingTool::new_n_by_n((tile1, tile2), size, self)
    }

    fn make_tools(&self) -> Vec<DrawingTool> {
        vec![
            self.new_tool(Tile::Empty),
            self.new_tool(Tile::Zonk),
            self.new_tool(Tile::Base),
            self.new_tool(Tile::Bug),
            self.new_tool(Tile::Murphy),
            self.new_tool(Tile::Infotron),
            self.new_tool(Tile::Exit),
            self.new_tool(Tile::PortRight),
            self.new_tool(Tile::PortDown),
            self.new_tool(Tile::PortLeft),
            self.new_tool(Tile::PortUp),
            // The gravity ports will be created by editing already placed regular ports.
            // self.new_tool(Tile::GravityPortRight),
            // self.new_tool(Tile::GravityPortDown),
            // self.new_tool(Tile::GravityPortLeft),
            // self.new_tool(Tile::GravityPortUp),
            self.new_tool(Tile::Port2WayVertical),
            self.new_tool(Tile::Port2WayHorizontal),
            self.new_tool(Tile::Port4Way),
            self.new_tool(Tile::FloppyOrange),
            self.new_tool(Tile::SnikSnak),
            self.new_tool(Tile::FloppyRed),
            self.new_tool(Tile::Electron),
            self.new_tool(Tile::FloppyYellow),
            self.new_tool(Tile::Terminal),
            self.new_tool2(Tile::RamLeft, Tile::RamRight, (2, 1)),
            self.new_tool(Tile::RamChip),
            self.new_tool2(Tile::RamTop, Tile::RamBottom, (1, 2)),
            self.new_tool(Tile::Wall),
            self.new_tool(Tile::Hardware1),
            self.new_tool(Tile::Hardware2),
            self.new_tool(Tile::Hardware3),
            self.new_tool(Tile::Hardware4),
            self.new_tool(Tile::Hardware5),
            self.new_tool(Tile::Hardware6),
            self.new_tool(Tile::Hardware7),
            self.new_tool(Tile::Hardware8),
            self.new_tool(Tile::Hardware9),
            self.new_tool(Tile::Hardware10)
        ]
    }
}