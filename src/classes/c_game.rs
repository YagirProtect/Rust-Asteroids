use crate::assetsdb_lib::c_assets_db::AssetsDB;
use crate::config_lib::c_config::Config;
use crate::render_lib::t_screen_data::Screen;

pub struct Game{
    screen: Screen,
    config: Config,
    assets_db: AssetsDB,
}

impl Game {
    pub fn new() -> Self {

        let assets_db = AssetsDB::new();
        let config = assets_db.get_any_asset_by_type::<Config>().cloned().unwrap_or_default();
        let screen = Screen::new(config.x(), config.y());

        Self{
            assets_db,
            config,
            screen
        }
    }
    
    
    pub fn get_screen(&self) -> &Screen {
        &self.screen
    }

    pub fn get_screen_mut(&mut self) -> &mut Screen {
        &mut self.screen
    }
    pub fn get_config(&self) -> &Config {
        &self.config
    }
    
    pub fn get_assets_db(&self) -> &AssetsDB {
        &self.assets_db
    }
}