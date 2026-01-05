use crate::assetsdb_lib::e_asset::Asset;
use crate::assetsdb_lib::loaders::t_asset_loader::AssetLoader;
use std::collections::HashMap;
use std::path::PathBuf;
use vek::Vec2;
use crate::assetsdb_lib::c_assets_db::AssetsDB;
use crate::assetsdb_lib::t_file_readable::FileReadable;
use crate::assetsdb_lib::t_file_writable::FileWritable;
use crate::config_lib::c_config::Config;
use crate::mesh_lib::c_mesh::Mesh;

#[derive(Default)]
pub struct ConfigLoader{}

impl AssetLoader for ConfigLoader {
    fn get_extension(&self) -> &str {
        "cfg"
    }


    fn load_all_assets(&self, files: &Vec<PathBuf>) -> HashMap<String, Asset> {
        let mut data: HashMap<String, Asset> = HashMap::new();

        for file in files {
            match file.extension() {
                Some(ext) => {
                    if (ext.to_str().unwrap() == self.get_extension()) {
                        let mut config = Config::default();
                        if (config.read_file(file.clone())) {
                            data.insert(file.to_str().unwrap().to_string(), Asset::Config(config));
                            break;
                        }
                    }
                }
                None => { continue; }
            }
        }

        if (data.len() == 0) {
            let mut config = Config::default();
            let path = AssetsDB::root_folder().join("config.cfg");
            config.write_file(path.clone());
            data.insert(path.to_str().unwrap().to_string(), Asset::Config(config));
        }

        data
    }
}
