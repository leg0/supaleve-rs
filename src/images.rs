use std::{ops::Index, rc::Rc};

use egui_extras::RetainedImage;

use crate::tool_panel::Tile;

pub const TILE_IMAGES:[&str; 40] = [
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

pub(crate) struct Images {
    images: [Rc<RetainedImage>; 40]
}

impl Images {
    pub(crate) fn new() -> Self {
        let images = TILE_IMAGES.map(|name| {
            let image_path = format!("img/{}", name);
            let image_bytes = std::fs::read(image_path).unwrap();
            Rc::new(RetainedImage::from_image_bytes(format!("{:?}", name), &image_bytes).unwrap())
        });
        Self {
            images
        }
    }

    pub(crate) fn name_of(tile: Tile) -> &'static str {
        TILE_IMAGES[tile as usize]
    }
}

impl Index<Tile> for Images {
    type Output = Rc<RetainedImage>;

    fn index(&self, index: Tile) -> &Self::Output {
        &self.images[index as usize]
    }
}
