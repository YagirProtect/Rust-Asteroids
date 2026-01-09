use std::collections::HashMap;
use std::path::{Path, PathBuf};
use crate::assetsdb_lib::e_asset::Asset;

pub trait AssetLoader {
    fn get_extension(&self) -> String;
    fn load_all_assets(&self, files: &Vec<PathBuf>) -> HashMap<String, Asset>;
}