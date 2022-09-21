use egui::{Layout, Align, TextureId, vec2};

use crate::tool_panel::Tile;

const PLAY_AREA_WIDTH: usize = 60;
const PLAY_AREA_HEIGHT: usize = 24;

pub struct EditorPanel
{
    heading: String,
    play_area: [Tile; PLAY_AREA_HEIGHT * PLAY_AREA_WIDTH],
}

impl EditorPanel {
    pub fn new(heading: &str) -> Self {
        Self {
            heading: String::from(heading),
            play_area: [Tile::Empty; PLAY_AREA_HEIGHT * PLAY_AREA_WIDTH],
        }
    }

    pub fn update(&self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(&self.heading);
            let vlayout = Layout::top_down(Align::Min);
            ui.with_layout(vlayout, |ui| {
                for _row in 0..PLAY_AREA_HEIGHT {
                    let hlayout = Layout::left_to_right(Align::Min).with_main_wrap(false);
                    ui.with_layout(hlayout, |ui| {
                        for _col in 0..PLAY_AREA_WIDTH {
                            //ui.label((row*PLAY_AREA_WIDTH + col).to_string());
                            ui.image(TextureId::default(), vec2(32., 32.));
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