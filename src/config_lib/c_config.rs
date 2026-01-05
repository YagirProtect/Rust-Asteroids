use serde::{Deserialize, Serialize};
use vek::Vec2;
use crate::assetsdb_lib::e_asset::Asset;
use crate::assetsdb_lib::t_file_readable::FileReadable;
use crate::assetsdb_lib::t_file_writable::FileWritable;
use crate::assetsdb_lib::t_from_assetref::FromAssetRef;

#[derive(Serialize, Deserialize)]
pub struct Config{
    window_size: Vec2<usize>
}

impl Default for Config {
    fn default() -> Self {
        Self{
            window_size: Vec2::new(800, 600)
        }
    }
}

impl Config{    
    pub fn new(window_size: Vec2<usize>) -> Config{
        Config{window_size}
    }

    pub fn size(&self) -> Vec2<usize>{
        self.window_size
    }
    
    pub fn x(&self) -> usize{self.window_size.x}

    pub fn y(&self) -> usize{self.window_size.y}
}

impl FileWritable for Config{}
impl FileReadable for Config{}

impl FromAssetRef for Config {
    fn from_asset(a: &Asset) -> Option<&Self> {
        match a {
            Asset::Config(c) => Some(c),
            _ => None,
        }
    }
}