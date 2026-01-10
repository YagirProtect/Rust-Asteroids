use std::collections::HashMap;
use std::path::{Path, PathBuf};
use egui::Context;
use crate::assetsdb_lib::e_asset::Asset;

pub trait AssetLoader {
    fn get_extension(&self) -> String;
    fn load_all_assets(&self, files: &Vec<PathBuf>) -> HashMap<String, Asset>{
        let mut assets = HashMap::new();
        assets
    }

    fn load_dynamic_assets(&self, files: &Vec<PathBuf>, context: &Context) -> HashMap<String, Asset> {
        let mut assets = HashMap::new();
        assets
    }
}