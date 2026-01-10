use crate::assetsdb_lib::e_asset::Asset;
use crate::assetsdb_lib::loaders::t_asset_loader::AssetLoader;
use crate::sprite_lib::c_sprite::SpriteTex;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use egui::Context;

#[derive(Default)]
pub struct ImageLoader {}

impl AssetLoader for ImageLoader {
    fn get_extension(&self) -> String {
        "png".to_string()
    }

    fn load_dynamic_assets(&self, files: &Vec<PathBuf>, context: &Context) -> HashMap<String, Asset> {
        let mut data: HashMap<String, Asset> = HashMap::new();

        for file in files {
            match file.extension() {
                Some(ext) => {
                    if (ext.to_str().unwrap() == self.get_extension()) {
                        let mut sprite = SpriteTex::new(file);
                        sprite.create_gui(context);
                        data.insert(file.to_str().unwrap().to_string(), Asset::Sprite(Rc::new(sprite)));
                    }
                }
                None => { continue; }
            }
        }

        data
    }
}
