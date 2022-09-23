
use std::marker::Copy;

use egui::{Layout, Align, vec2, ImageButton, Sense, Color32, InputState, Key, hex_color, containers::ComboBox};

use crate::{tool_panel::{Tile, ToolPanel, OperatingMode}, images::Images};

const PLAY_AREA_WIDTH: usize = 60;
const PLAY_AREA_HEIGHT: usize = 24;
const PLAY_AREA_SIZE: usize = PLAY_AREA_WIDTH * PLAY_AREA_HEIGHT;

fn col_row(index: usize) -> (usize, usize) {
    (index % PLAY_AREA_WIDTH, index / PLAY_AREA_WIDTH)
}

fn index(col: usize, row: usize) -> usize {
    return col + PLAY_AREA_WIDTH * row
}

fn minmax<T : Ord + Copy>(a: T, b: T) -> (T, T) {
    (a.min(b), a.max(b))
}

#[derive(Copy, Clone, Debug)]
enum LineMode
{
    HorizontalFirst,
    VerticalFirst
}

impl LineMode {
    pub(crate) fn toggle(&self) -> Self {
        match self {
            Self::HorizontalFirst => Self::VerticalFirst,
            _ => Self::HorizontalFirst
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum ToolMode {
    Nop,
    Draw { tile: Tile },
    Line { tile: Tile, start: usize, mode: LineMode },
    Rect { tile: Tile, start: usize },
}

impl ToolMode {
    fn toggle_line_mode(&self) -> Self {
        match *self {
            Self::Line { tile, start, mode } => Self::Line { tile, start, mode: mode.toggle() },
            other => other
        }
    }
}

pub struct EditorPanel
{
    heading: String,
    play_area: [Tile; PLAY_AREA_SIZE],
    highlight: [bool; PLAY_AREA_SIZE],
    tool_mode: Option<ToolMode>,
    images: Images, // TODO: share this between panels
    ptr_primary: bool, // XXX: workaround for not detecting button release events 
    ptr_secondary: bool,
    selected_level_index: usize,
    selected_tile_index: Option<usize>,
}

impl EditorPanel {
    pub fn new(heading: &str) -> Self {
        Self {
            heading: String::from(heading),
            play_area: [Tile::Empty; PLAY_AREA_SIZE],
            highlight: [false; PLAY_AREA_SIZE],
            tool_mode: None,
            images: Images::new(),
            ptr_primary: false,
            ptr_secondary: false,
            selected_level_index: 0,
            selected_tile_index: None
        }
    }

    pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, tool_panel: &ToolPanel) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&self.heading);

            // TODO: get this from levels.dat
            let levels = ["--- FIRST ---", "--- SECOND ---", "-- THIRD --"];

            let cmb = ComboBox::from_label("level");
            let cmb_res = cmb.show_index(ui, &mut self.selected_level_index, levels.len(), |i| levels[i].to_owned());
            if cmb_res.changed() {
                println!("{}", levels[self.selected_level_index]);
                // TODO: switch to selected level
            }

            let spacing = ui.spacing_mut();
            spacing.item_spacing = vec2(0., 0.);

            let vlayout = Layout::top_down(Align::Min);
            ui.with_layout(vlayout, |ui| {
                match tool_panel.operating_mode() {
                    OperatingMode::Draw => self.do_draw(ui, tool_panel, ctx),
                    OperatingMode::Select => self.do_select(ui, tool_panel, ctx)
                }
            });
        });
    }

    fn do_draw(&mut self, ui: &mut egui::Ui, tool_panel: &ToolPanel, ctx: &egui::Context) {
        for row in 0..PLAY_AREA_HEIGHT {
            let hlayout = Layout::left_to_right(Align::Min).with_main_wrap(false);
            ui.with_layout(hlayout, |ui| {

                let selected_tool_tile = tool_panel.selected_tool_tile();
                for col in 0..PLAY_AREA_WIDTH {
                    let is_delete = ui.input().pointer.secondary_down();
                    let tile_index = index(col, row);
                    let response = self.add_image_button_draw(tile_index, selected_tool_tile, is_delete, ctx, ui);

                    // - toggle between variants: T
                    //      * yellow, red, orange disk
                    //      * port, gravity port
                    //      * different walls
                    //      * ram chips
                    //      * base/bug
                    //      * switch between line draw modes (HV/VH)
                    // - rotate: R (applies to 2-piece ram chips, ports)
                    //
                    let ptr = &ui.input().pointer;
                    if response.hovered() && (ptr.primary_down() || ptr.secondary_down()) {
                        let mode = self.get_tool_mode(tile_index, selected_tool_tile, &ui.input());
                        self.tool_mode = Some(mode);
                        self.highlight.fill(false);
                        match mode {
                            ToolMode::Draw{tile}
                                => self.play_area[tile_index] = tile,
                            ToolMode::Line { tile: _, start, mode: LineMode::HorizontalFirst }
                                => self.line_horizontal_first(start, tile_index),
                            ToolMode::Line { tile: _, start, mode: LineMode::VerticalFirst }
                                => self.line_vertical_first(start, tile_index),
                            ToolMode::Rect { tile: _, start }
                                => self.rect(start, tile_index),
                            _ => {},
                        }
                    }
                }

                self.try_complete_tool(ui, selected_tool_tile);
            });
        }
    }

    fn do_select(&mut self, ui: &mut egui::Ui, tool_panel: &ToolPanel, ctx: &egui::Context) {
        for row in 0..PLAY_AREA_HEIGHT {
            let hlayout = Layout::left_to_right(Align::Min).with_main_wrap(false);
            ui.with_layout(hlayout, |ui| {
                for col in 0..PLAY_AREA_WIDTH {
                    let tile_index = index(col, row);
                    let is_selected = match self.selected_tile_index {
                        Some(x) => x == tile_index,
                        None => false,
                    };
                    if self.add_image_button_select(tile_index, is_selected, ctx, ui).clicked() {
                        self.selected_tile_index = (!is_selected).then(|| tile_index) ;
                    }
                }
            });
        }
    }
    
    fn add_image_button_draw(&self, tile_index: usize, tool_tile: Tile, is_delete: bool, ctx: &egui::Context, ui: &mut egui::Ui) -> egui::Response {
        let tile = if self.highlight[tile_index] && !is_delete { tool_tile } else { self.play_area[tile_index] };
        let texture_id = self.images[tile].texture_id(ctx);
        let mut btn = ImageButton::new(texture_id, vec2(32., 32.));
        btn = btn.frame(false);
        btn = btn.sense(Sense::hover());
        if self.highlight[tile_index] {
            let tint_color = if is_delete { hex_color!("#ff808080") } else { Color32::DARK_GRAY };
            btn = btn.tint(tint_color);
        }
        ui.add(btn)
    }

    fn add_image_button_select(&self, tile_index: usize, is_selected: bool, ctx: &egui::Context, ui: &mut egui::Ui) -> egui::Response {
        let tile = self.play_area[tile_index];
        let texture_id = self.images[tile].texture_id(ctx);
        let mut btn = ImageButton::new(texture_id, vec2(32., 32.));
        btn = btn.frame(false);
        if is_selected {
            btn = btn.tint(hex_color!("#80FF8080"));
        }
        ui.add(btn)
    }

    fn try_complete_tool(&mut self, ui: &mut egui::Ui, selected_tool: Tile) {
        if let Some(_) = self.tool_mode {
            let input = ui.input();
            let modifiers = &input.modifiers;
            let ptr = &input.pointer;
            let primary_released = || self.ptr_primary && !ptr.primary_down();
            let secondary_released = || self.ptr_secondary && !ptr.secondary_down();

            if (modifiers.shift || modifiers.ctrl) && primary_released() {
                self.commit_draw(selected_tool);
            }
            else if (modifiers.shift || modifiers.ctrl) && secondary_released() {
                self.commit_delete();
            }
            else if !modifiers.ctrl && !modifiers.shift {
                self.cancel_tool();
            }
    
            self.ptr_primary = ptr.primary_down();
            self.ptr_secondary = ptr.secondary_down();
        }
    }

    fn cancel_tool(&mut self) {
        println!("Canceling the tool");
        self.highlight.fill(false);
        self.tool_mode = None;
    }

    fn commit_delete(&mut self) {
        self.commit_draw(Tile::Empty);
    }

    fn commit_draw(&mut self, sel_tool: Tile) {
        self.highlight.iter_mut().enumerate().filter(|(_, &mut x)| x).for_each(|(i, y)| {
            self.play_area[i] = sel_tool;
            *y = false;
        });
        self.tool_mode = None;
    }

    fn line_horizontal_first(&mut self, start: usize, tile_index: usize) {
        let (start_x, start_y) = col_row(start);
        let (end_x, end_y) = col_row(tile_index);
        let (min_x, max_x) = minmax(start_x, end_x);
        let (min_y, max_y) = minmax(start_y, end_y);
        let h_start = index(min_x, start_y);
        let h_end = index(max_x, start_y);
        self.highlight[h_start..h_end].fill(true);
        let v_start = index(end_x, min_y);
        let v_end = index(end_x, max_y);
        for x in self.highlight[v_start..=v_end].iter_mut().step_by(PLAY_AREA_WIDTH) {
            *x = true;
        }
    }

    fn line_vertical_first(&mut self, start: usize, tile_index: usize) {
        let (start_x, start_y) = col_row(start);
        let (end_x, end_y) = col_row(tile_index);
        let (min_x, max_x) = minmax(start_x, end_x);
        let (min_y, max_y) = minmax(start_y, end_y);
        let h_start = index(min_x, end_y);
        let h_end = index(max_x, end_y);
        self.highlight[h_start..h_end].fill(true);
        let v_start = index(start_x, min_y);
        let v_end = index(start_x, max_y);
        for x in self.highlight[v_start..=v_end].iter_mut().step_by(PLAY_AREA_WIDTH) {
            *x = true;
        }
    }

    fn rect(&mut self, start: usize, end: usize) {
        let (start_x, start_y) = col_row(start);
        let (end_x, end_y) = col_row(end);
        let (start_x, end_x) = minmax(start_x, end_x);
        let (start_y, end_y) = minmax(start_y, end_y);

        for row in start_y..=end_y {
            let start = index(start_x, row);
            let end = index(end_x, row);
            self.highlight[start..=end].fill(true);
        }
    }

    fn get_tool_mode(&self, start_tile_index: usize, selected_tool_tile: Tile, input: &InputState) -> ToolMode {
        // shift = line
        // ctrl = rect
        // primary = draw
        // secondary = delete
        let tile = if input.pointer.primary_down() { selected_tool_tile } else { Tile::Empty };
        let modifiers = &input.modifiers;
        match self.tool_mode {
            Some(tool_mode) => Self::toggle_line_mode(tool_mode, input.key_released(Key::T)),
            None => match (modifiers.shift, modifiers.ctrl) {
                (false, false) => ToolMode::Draw { tile },
                (true, false) => ToolMode::Line { tile, start: start_tile_index, mode: LineMode::HorizontalFirst },
                (false, true) => ToolMode::Rect { tile, start: start_tile_index },
                (true, true) => ToolMode::Nop,
            }
        }
    }

    fn toggle_line_mode(tool_mode : ToolMode, toggle: bool) -> ToolMode {
        if toggle {
           tool_mode.toggle_line_mode()
        }
        else {
            tool_mode
        }
    }
}