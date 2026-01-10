use std::collections::HashMap;
use std::{env, fs};
use std::fs::read_dir;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use crate::assetsdb_lib::e_asset::Asset;
use crate::mesh_lib::c_mesh::Mesh;
use crate::assetsdb_lib::loaders::c_config_asset_processor::ConfigLoader;
use crate::assetsdb_lib::loaders::c_meshes_asset_processor::MeshLoader;
use crate::assetsdb_lib::loaders::c_sprite_asset_processor::ImageLoader;
use crate::assetsdb_lib::loaders::t_asset_loader::AssetLoader;
use crate::assetsdb_lib::t_from_assetref::FromAssetRef;
use crate::sprite_lib::c_sprite::SpriteTex;

pub struct AssetsDB {
    all_assets_paths: Vec<PathBuf>,
    loaders: Vec<Box<dyn AssetLoader>>,
    map: HashMap<String, Asset>,
}
impl Default for AssetsDB {
    fn default() -> Self {
        AssetsDB {
            map: HashMap::new(),
            all_assets_paths: vec![],
            loaders: vec![],
        }
    }
}

impl AssetsDB {
    pub fn new() -> AssetsDB {
        let mut default = Self::default();
        default.create_all_folders();
        default.find_all_assets_drive();
        default.call_loaders();
        default
    }

    pub fn load_dynamic(&mut self, ctx: &egui::Context){
        for n in self.loaders.iter_mut(){
            let n = n.load_dynamic_assets(&self.all_assets_paths, ctx);

            for (path, value) in n {
                println!("{}", path);
                self.map.insert(path, value);
            }
        }
    }

    pub fn root_folder() -> PathBuf {
        let n = Self::get_curr_dir().join("data");
        n
    }

    pub fn create_all_folders(&self) {
        fs::create_dir_all(Self::root_folder().join("/models/")).unwrap();
        fs::create_dir_all(Self::root_folder().join("/models/meteors")).unwrap();
        fs::create_dir_all(Self::root_folder().join("/models/entity")).unwrap();
    }

    pub fn get_curr_dir() -> PathBuf {
        match env::current_dir() {
            Ok(path) => {
                path
            }
            Err(_) => {
                panic!("Error getting current directory");
            }
        }
    }

    fn call_loaders(&mut self) {
        self.loaders = vec![
            Box::new(MeshLoader::default()),
            Box::new(ConfigLoader::default()),
            Box::new(ImageLoader::default())
        ];

        for loader in self.loaders.iter() {
            let n = loader.load_all_assets(&self.all_assets_paths);

            for (path, value) in n {
                self.map.insert(path, value);
            }
        }
    }

    fn find_all_assets_drive(&mut self) {
        let folder = Self::get_curr_dir().join("data");
        let mut folders = vec![folder.clone()];
        while let Some(n) = folders.pop() {
            let list = match read_dir(n) {
                Ok(list) => list,
                _ => {
                    continue;
                }
            };

            for entry in list {
                let dir = match entry {
                    Ok(dir) => dir,
                    _ => {
                        continue;
                    }
                };

                self.all_assets_paths.push(dir.path());
                folders.push(dir.path());
            }
        }

        for n in self.all_assets_paths.iter().enumerate() {
            println!("Loading all assets from {:?}", n.1.to_str().unwrap());
        }
    }


    pub fn get_asset<T: FromAssetRef>(&self, path: &str) -> Option<&T> {
        let mut p = Self::root_folder();
        p = p.join(path);
        self.map.get(p.to_str().unwrap()).and_then(T::from_asset)
    }

    pub fn get_sprite_by_name(&self, name: &str) -> Option<Rc<SpriteTex>> {
        self.map.iter().find_map(|(k, v)| match v {
            Asset::Sprite(m)

            if Path::new(k)
                .file_stem()
                .and_then(|s| s.to_str()) == Some(name) => Some(Rc::clone(m)),
            _ => None,
        })
    }

    pub fn get_mesh_by_name(&self, name: &str) -> Option<Rc<Mesh>> {
        self.map.iter().find_map(|(k, v)| match v {
            Asset::Mesh(m)

            if Path::new(k)
                .file_stem()
                .and_then(|s| s.to_str()) == Some(name) => Some(Rc::clone(m)),
            _ => None,
        })
    }

    pub fn get_all_assets_by_type<T: FromAssetRef>(&self) -> Option<Vec<&T>> {
        let mut list: Vec<&T> = vec![];

        for v in self.map.values() {
            if let Some(n) = T::from_asset(v) {
                list.push(n);
            }
        }
        Some(list)
    }

    pub fn get_any_asset_by_type<T: FromAssetRef>(&self) -> Option<&T> {
        for v in self.map.values() {
            if let Some(n) = T::from_asset(v) {
                return Some(n)
            }
        }
        None
    }
}