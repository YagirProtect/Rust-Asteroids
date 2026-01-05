use std::path::PathBuf;
use serde::Deserialize;
use crate::assetsdb_lib::json_utils::read_json_file;

pub trait FileReadable {
    fn read_file(&mut self, path: PathBuf) -> bool
    where for<'de> Self: Deserialize<'de> {
        match read_json_file(&path) {
            Ok(mesh) => {
                *self = mesh;
                return true;
            }
            _ => {
                println!("Error reading json file {}", path.to_str().unwrap());
                
                return false;
            }
        }
    }
}