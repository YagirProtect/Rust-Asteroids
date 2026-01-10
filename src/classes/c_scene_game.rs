use std::rc::Rc;
use egui::{vec2, Align, Context, Layout};
use rand::prelude::ThreadRng;
use rand::Rng;
use crate::assetsdb_lib::c_assets_db::AssetsDB;
use crate::classes::c_asteroid_entity::AsteroidEntity;
use crate::classes::c_player_entity::PlayerEntity;
use crate::classes::t_entity::Entity;
use crate::config_lib::c_config::Config;
use crate::render_lib::t_screen_data::Screen;
use crate::scenes_lib::t_scene::Scene;
use crate::transform_lib::c_transform::Transform;
use rand::seq::IteratorRandom;
use vek::Vec2;
use crate::mesh_lib::c_mesh::Mesh;
use crate::render_lib::f_drawers::ui_draw_icon;
use crate::scenes_lib::e_scene_event::SceneEvent;
use crate::scenes_lib::e_scene_switch::SceneSwitch;
use crate::sprite_lib::c_sprite::SpriteTex;

#[derive(Default)]
pub enum GameState{
    #[default]
    Active,
    End
}

#[derive(Default)]
pub struct GameScene {
    entities: Vec<Box<dyn Entity>>,

    asteroids_ids: Vec<u32>,
    player_id: u32,
    asteroids_models: Vec<Rc<Mesh>>,

    state: GameState,
    player_healths: i8,
    scores: u32,
    health_icon: Rc<SpriteTex>
}

impl GameScene {
    fn spawn_asteroids(&mut self, config: &Config) {
        let mut rng = rand::rng();


        let asteroid_radius = 120.0;
        let player_entity = self.entities.iter().find(|x| x.get_entity_id() == self.player_id).unwrap();

        let player_pos: Vec2<f32> = player_entity.get_position().clone();
        while self.asteroids_ids.len() < 5 {
            let random_pos = Vec2::new(
                rng.random_range(0..config.x()) as f32,
                rng.random_range(0..config.y()) as f32
            );

            let dist = Vec2::distance(player_pos, random_pos);
            if (dist >= asteroid_radius) {

                let max_scale = 1.0;
                self.spawn_asteroid(config, &mut rng, random_pos, 0.8, max_scale);
            }
        }
    }

    fn spawn_asteroid(&mut self, config: &Config, mut rng: &mut ThreadRng, random_pos: Vec2<f32>,min_scale: f32, max_scale: f32) {



        let scale = rng.random_range(min_scale..max_scale);

        let mesh: Rc<Mesh> = self.asteroids_models
            .iter()
            .choose(&mut rng)
            .cloned()
            .unwrap();


        let mut asteroid = AsteroidEntity::new(
            Transform::new(
                random_pos,
                Vec2::new(1.0, 1.0) * scale,
                rng.random_range(0.0..360.0),
                config.size()
            ),
            mesh
        );

        asteroid.set_velocity(
            Vec2::new(
                rng.random_range(-1.0..1.0),
                rng.random_range(-1.0..1.0)
            ) * rng.random_range(30.0..100.0)
        );


        let id = self.add_entity(Box::new(asteroid));

        self.asteroids_ids.push(id);
    }
}

impl Scene for GameScene
{
    fn create_scene(&mut self, config: &Config, screen: &Screen, assets_db: &AssetsDB) {
        let mut player = PlayerEntity::new(
            Transform::new(
                screen.center(),
                Vec2::new(0.3, 0.3),
                0.0,
                config.size()
            ),
            assets_db.get_mesh_by_name("player").unwrap_or_default()
        );

        self.player_healths = 3;

        self.asteroids_models = vec![
            assets_db.get_mesh_by_name("asteroid_01").unwrap_or_default(),
            assets_db.get_mesh_by_name("asteroid_02").unwrap_or_default(),
            assets_db.get_mesh_by_name("asteroid_03").unwrap_or_default(),
            assets_db.get_mesh_by_name("asteroid_04").unwrap_or_default()
        ];


        self.health_icon = assets_db.get_sprite_by_name("heart").unwrap_or_default();

        println!("has icon: {:?}", assets_db.get_sprite_by_name("heart").is_some());

        self.player_id = self.add_entity(Box::new(player));

        self.spawn_asteroids(&config);
    }

    fn custom_events_solve(&mut self, scene_event: &Vec<SceneEvent>, config: &Config, asset_db: &AssetsDB) {
        for n in scene_event {
            match n {
                SceneEvent::DemolishAsteroid { pos, scale, id  } => {

                    self.scores += (*scale  * 50.0) as u32;

                    if let Some(i) = self.asteroids_ids.iter().position(|x| *x == *id) {
                        self.asteroids_ids.remove(i); // O(n), сдвигает элементы
                    }
                    let max_scale = scale/2.0;
                    let min_scale = max_scale * 0.55;
                    if (min_scale< 0.15) {continue};

                    let mut rng = rand::rng();

                    let count = rng.random_range(2..4);

                    for i in 0..count {
                        self.spawn_asteroid(config, &mut rng, *pos, min_scale, max_scale);
                    }

                },
                SceneEvent::PlayerDeath =>{
                    self.player_healths -= 1;
                    if (self.player_healths <= 0) {
                        self.state = GameState::End;

                        self.entities.clear();
                    }
                }
                _=>{}
            }
        }

        if (self.asteroids_ids.len() == 0){
            self.spawn_asteroids(&config)
        }
    }

    fn ui(&mut self, ctx: &Context) -> SceneSwitch {
        match self.state {
            GameState::Active =>{
                let frame = egui::Frame::none()
                    .fill(egui::Color32::from_rgba_unmultiplied(0, 0, 0, 0))
                    .stroke(egui::Stroke::NONE);
                egui::TopBottomPanel::bottom("bottom_data")
                    .frame(frame)
                    .show_separator_line(false)
                    .min_height(35.0)
                    .show(ctx, |ui| {
                        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                            // LEFT
                            ui.label(
                                egui::RichText::new(format!("SCORES: {}", self.scores))
                                    .size(20.0)
                                    .strong(),
                            );

                            // SPRING: забираем всё оставшееся место
                            let space = ui.available_width();
                            ui.add_space(space-(30.0*self.player_healths as f32));

                            // RIGHT
                            ui.horizontal(|ui| {
                                for _ in 0..self.player_healths {
                                    ui_draw_icon(ui, &self.health_icon, Vec2::new(20.0, 20.0));
                                }
                            });
                        });
                    });
            }
            GameState::End => {

            }
        }

        SceneSwitch::None
    }

    fn get_scene_name(&self) -> String{
        String::from("TestScene")
    }

    fn get_entities(&self) -> &Vec<Box<dyn Entity>> {
        &self.entities
    }

    fn get_entities_mut(&mut self) -> &mut Vec<Box<dyn Entity>> {
        &mut self.entities
    }


}

