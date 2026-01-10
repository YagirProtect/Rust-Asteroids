use std::path::PathBuf;
use image::{ImageResult, RgbaImage};
use vek::Vec2;
use crate::assetsdb_lib::e_asset::Asset;
use crate::assetsdb_lib::t_from_assetref::FromAssetRef;
use crate::mesh_lib::c_mesh::Mesh;

#[derive(Default)]
pub struct SpriteTex {
    name: String,
    size: Vec2<u32>,
    image: RgbaImage,
    gui_texture: Option<egui::TextureHandle>
}
impl FromAssetRef for SpriteTex {
    fn from_asset(a: &Asset) -> Option<&Self> {
        match a {
            Asset::Sprite(m) => Some(m),
            _ => None,
        }
    }
}

impl SpriteTex {
    pub fn new(path: &PathBuf) -> Self{

        let bytes = match std::fs::read(path) {
            Ok(bytes) => bytes,
            _ => {
                panic!("Unable to read file {}", path.display());
            }
        };

        let img = match image::load_from_memory(&bytes) {
            Ok(img) => img,
            _ => {
                panic!("Unable to load file {}", path.display());
            }
        }.to_rgba8();



        Self{
            name: path.to_str().unwrap().to_string(),
            size: Vec2::new(img.width(), img.height()),
            image: img,
            gui_texture: None,
        }
    }

    pub fn create_gui(&mut self, ctx: &egui::Context) {
        if (self.gui_texture.is_none()) {
            let img = egui::ColorImage::from_rgba_unmultiplied(
                [self.image.width() as usize, self.image.height() as usize],
                self.image.as_ref(),
            );
            let handle = ctx.load_texture(self.name.clone(), img, egui::TextureOptions::NEAREST);
            self.gui_texture = Some(handle)
        }
    }

    pub fn get_gui_texture(&self) -> egui::TextureHandle {
        self.gui_texture.clone().unwrap()
    }

}