
use std::marker::Copy;

use egui::{Layout, Align, vec2, ImageButton, Sense, Modifiers, Color32, InputState, Key};

use crate::{tool_panel::{Tile, ToolPanel}, images::Images};

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
    Draw,
    Line { start: usize, mode: LineMode }, // start of line
    Rect { start: usize }, // start of rectangle
}

impl ToolMode {
    fn toggle_line_mode(&self) -> Self {
        match self {
            Self::Line { start, mode } => Self::Line { start: *start, mode: mode.toggle() },
            _ => *self
        }
    }
}

pub struct EditorPanel
{
    heading: String,
    play_area: [Tile; PLAY_AREA_SIZE],
    highlight: [bool; PLAY_AREA_SIZE],
    tool_mode: Option<ToolMode>,
    images: Images // TODO: share this between panels
}

impl EditorPanel {
    pub fn new(heading: &str) -> Self {
        Self {
            heading: String::from(heading),
            play_area: [Tile::Empty; PLAY_AREA_SIZE],
            highlight: [false; PLAY_AREA_SIZE],
            tool_mode: None,
            images: Images::new()
        }
    }

    pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame, tool_panel: &ToolPanel) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&self.heading);

            let spacing = ui.spacing_mut();
            spacing.item_spacing = vec2(0., 0.);

            let vlayout = Layout::top_down(Align::Min);
            ui.with_layout(vlayout, |ui| {
                for row in 0..PLAY_AREA_HEIGHT {
                    let hlayout = Layout::left_to_right(Align::Min).with_main_wrap(false);
                    ui.with_layout(hlayout, |ui| {
                        let sel_tool = tool_panel.selected_tool().map(|tool| tool.tile());
                        for col in 0..PLAY_AREA_WIDTH {
                            let tile_index = index(col, row);
                            let tile = match (sel_tool, self.highlight[tile_index]) {
                                (Some(t), true) => t,
                                _ => self.play_area[tile_index],
                            };
                            let texture_id = self.images[tile].texture_id(ctx);
                            let mut btn = ImageButton::new(texture_id, vec2(32., 32.));
                            btn = btn.frame(false);
                            btn = btn.sense(Sense::hover());
                            if self.highlight[tile_index] {
                                btn = btn.tint(Color32::DARK_GRAY);
                            }
                            // TODO: different modes
                            // - draw: left_mouse place tile where the cursor is
                            // - line: shift+left_mouse to draw L-shaped line between start and end points.
                            //      commit drawing on mouse-up
                            //      cancel by releasing shift while mouse is down.
                            // - filled rect: ctrl+left_mouse to draw rectangle
                            //      commit drawing on mouse-up
                            //      cancel by releasing ctrl while mouse is down.
                            // - delete: right_mouse remove tile where the cursor is
                            // - delete_line: shift+right_mouse to delete L-shaped line between start and end points
                            // - delete_rect: ctrl+right_mouse to delete rectangle

                            // - toggle between variants: T
                            //      * yellow, red, orange disk
                            //      * port, gravity port
                            //      * different walls
                            //      * ram chips
                            //      * base/bug
                            //      * switch between line draw modes (HV/VH)
                            // - rotate: R (applies to 2-piece ram chips, ports)
                            //      
                            let response = ui.add(btn);
                            if let Some(tool) = tool_panel.selected_tool() {
                                if response.hovered() && ui.input().pointer.primary_down() {
                                    let mode = self.get_tool_mode(tile_index, &ui.input());
                                    self.tool_mode = Some(mode);
                                    self.highlight.fill(false);
                                    match mode {
                                        ToolMode::Draw => self.play_area[tile_index] = tool.tile(),
                                        ToolMode::Line { start, mode: LineMode::HorizontalFirst }
                                            => self.line_horizontal_first(start, tile_index),
                                        ToolMode::Line { start, mode: LineMode::VerticalFirst }
                                            => self.line_vertical_first(start, tile_index),
                                        _ => {},
                                        // ToolMode::Rect { start } => {}
                                    }
                                }
                            }
                        }

                        if let Some(_) = self.tool_mode {
                            println!("tool mode={:?}", self.tool_mode);
                            let input = &ui.input();
                            let modifiers = &input.modifiers;
                            let ptr = &input.pointer;
                            println!("tool mode={:?}, shift={}, ctrl={}, prim_rel={}, sec_rel={}, prim_dn={}, sec_dn={}"
                            , self.tool_mode, modifiers.shift, modifiers.ctrl, ptr.primary_released(), ptr.secondary_released()
                            , ptr.primary_down(), ptr.secondary_down());

                            if (modifiers.shift || modifiers.ctrl) && ptr.primary_released() {
                                // Commit draw line / draw rect
                                println!("Committing draw line");
                                self.highlight.iter_mut().enumerate().filter(|(_, &mut x)| x).for_each(|(i, y)| {
                                    self.play_area[i] = sel_tool.unwrap();
                                    *y = false;
                                });
                                self.tool_mode = None;
                            }
                            else if (modifiers.shift || modifiers.ctrl) && ptr.secondary_released() {
                                // Commit delete rect / delete rect
                                self.highlight.iter_mut().enumerate().filter(|(_, &mut x)| x).for_each(|(i, y)| {
                                    self.play_area[i] = Tile::Empty;
                                    *y = false;
                                });
                                self.tool_mode = None;
                            }
                            else if !modifiers.ctrl && !modifiers.shift {
                                // Cancel the tool
                                println!("Canceling the tool");
                                self.highlight.fill(false);
                                self.tool_mode = None;
                            }
                        }
                    });
                }
            });
        });
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

    fn get_tool_mode(&self, tile_index: usize, input: &InputState) -> ToolMode {
        let modifiers = &input.modifiers;
        match self.tool_mode {
            Some(tool_mode) => Self::toggle_line_mode(tool_mode, input.key_released(Key::T)),
            None => match (modifiers.shift, modifiers.ctrl) {
                (false, false) => ToolMode::Draw,
                (true, false) => ToolMode::Line { start: tile_index, mode: LineMode::HorizontalFirst },
                (_, true) => ToolMode::Rect { start: tile_index },
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