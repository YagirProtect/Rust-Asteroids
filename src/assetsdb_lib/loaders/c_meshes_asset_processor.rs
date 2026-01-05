use crate::assetsdb_lib::e_asset::Asset;
use crate::assetsdb_lib::loaders::t_asset_loader::AssetLoader;
use std::collections::HashMap;
use std::path::PathBuf;
use crate::assetsdb_lib::t_file_readable::FileReadable;
use crate::mesh_lib::c_mesh::Mesh;

#[derive(Default)]
pub struct MeshLoader{}

impl AssetLoader for MeshLoader{
    fn get_extension(&self) -> &str {
        "mesh"
    }


    fn load_all_assets(&self, files: &Vec<PathBuf>) -> HashMap<String, Asset> {
        let mut data: HashMap<String, Asset> = HashMap::new();

        for file in files {
            match file.extension(){
                Some(ext) => {
                    if (ext.to_str().unwrap() == self.get_extension()){
                        let mut mesh = Mesh::new(vec![], false);
                        if (mesh.read_file(file.clone())){
                            data.insert(file.to_str().unwrap().to_string(), Asset::Mesh(mesh));
                        }
                    }
                }
                None => {continue;}
            }
        }
        data
    }
}
