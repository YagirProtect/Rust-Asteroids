use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use vek::{Lerp, Vec2};
use crate::assetsdb_lib::e_asset::Asset;
use crate::assetsdb_lib::json_utils::{read_json_file, ReadJsonError};
use crate::assetsdb_lib::t_file_readable::FileReadable;
use crate::assetsdb_lib::t_file_writable::FileWritable;
use crate::assetsdb_lib::t_from_assetref::FromAssetRef;

#[derive(Serialize, Deserialize)]
pub struct MeshLine{
    pub start: Vec2<f32>,
    pub end: Vec2<f32>,
}

impl MeshLine {
    pub fn new(start: Vec2<f32>, end: Vec2<f32>) -> Self {
        Self{
            start,
            end
        }
    }    
}

#[derive(Serialize, Deserialize)]
pub struct Mesh{
    points: Vec<MeshLine>,
    filled: bool
}


impl FileWritable for Mesh{}

impl FileReadable for Mesh {}

impl FromAssetRef for Mesh {
    fn from_asset(a: &Asset) -> Option<&Self> {
        match a {
            Asset::Mesh(m) => Some(m),
            _ => None,
        }
    }
}

impl Mesh {
    pub fn new(points: Vec<MeshLine>, filled: bool) -> Mesh {
        Mesh { points, filled }
    }
    pub fn get_points(&self) -> &Vec<MeshLine> { &self.points }
}

