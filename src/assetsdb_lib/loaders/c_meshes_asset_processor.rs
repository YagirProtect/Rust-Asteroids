use crate::assetsdb_lib::e_asset::Asset;
use crate::assetsdb_lib::loaders::t_asset_loader::AssetLoader;
use std::collections::HashMap;
use std::path::PathBuf;
use std::rc::Rc;
use egui::TextBuffer;
use crate::assetsdb_lib::t_file_readable::FileReadable;
use crate::mesh_lib::c_mesh::Mesh;

#[derive(Default)]
pub struct MeshLoader{}

impl AssetLoader for MeshLoader{
    fn get_extension(&self) -> String {
        Self::get_extension_static()
    }

    fn load_all_assets(&self, files: &Vec<PathBuf>) -> HashMap<String, Asset> {
        let mut data: HashMap<String, Asset> = HashMap::new();

        for file in files {
            match file.extension(){
                Some(ext) => {
                    if (ext.to_str().unwrap() == self.get_extension()){
                        let mut mesh = Mesh::new(vec![], false);
                        if (mesh.read_file(file.clone())){
                            data.insert(file.to_str().unwrap().to_string(), Asset::Mesh(Rc::new(mesh)));
                        }
                    }
                }
                None => {continue;}
            }
        }
        data
    }
}

impl MeshLoader {
    pub fn get_extension_static() -> String {
        "mesh".to_string()
    }
}
