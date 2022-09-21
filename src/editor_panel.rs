use egui::{Layout, Align, vec2, ImageButton, Sense};

use crate::{tool_panel::{Tile, ToolPanel}, images::Images};

const PLAY_AREA_WIDTH: usize = 60;
const PLAY_AREA_HEIGHT: usize = 24;

pub struct EditorPanel
{
    heading: String,
    play_area: [Tile; PLAY_AREA_HEIGHT * PLAY_AREA_WIDTH],
    images: Images // TODO: share this between panels
}

impl EditorPanel {
    pub fn new(heading: &str) -> Self {
        Self {
            heading: String::from(heading),
            play_area: [Tile::Empty; PLAY_AREA_HEIGHT * PLAY_AREA_WIDTH],
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
                        for col in 0..PLAY_AREA_WIDTH {
                            let tile_index = col + PLAY_AREA_WIDTH*row;
                            let tile = self.play_area[tile_index];
                            let texture_id = self.images[tile].texture_id(ctx);
                            let mut btn = ImageButton::new(texture_id, vec2(32., 32.));
                            btn = btn.frame(false);
                            btn = btn.sense(Sense::hover());
                            
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
                            // - rotate: R (applies to 2-piece ram chips, ports)
                            //      
                            let response = ui.add(btn);
                            if let Some(tool) = tool_panel.selected_tool() {
                                let ptr = &ui.input().pointer;
                                if response.hovered() && ptr.primary_down() {
                                    self.play_area[tile_index] = tool.tile();
                                }
                            }
                        }
                    });
                }
            });
            // ui.horizontal(|ui| {
            //     ui.label("Your name: ");
            //     ui.text_edit_singleline(&mut self.name);
            // });
            // ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            // if ui.button("Click each year").clicked() {
            //     self.age += 1;
            // }
            // ui.label(format!("Hello '{}', age {}", self.name, self.age));
        });
    }
}