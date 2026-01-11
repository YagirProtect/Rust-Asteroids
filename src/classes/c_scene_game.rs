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
use crate::classes::c_enemy_entity::EnemyEntity;
use crate::mesh_lib::c_mesh::Mesh;
use crate::render_lib::f_drawers::{ui_button, ui_draw_icon, ui_get_card_rect, ui_title_rect, ui_transparent_frame};
use crate::scenes_lib::e_scene_event::SceneEvent;
use crate::scenes_lib::e_scene_switch::SceneSwitch;
use crate::scenes_lib::e_sceneid::SceneId;
use crate::sprite_lib::c_sprite::SpriteTex;
use crate::web_lib::c_web_client::WebClient;

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
    debris_models: Vec<Rc<Mesh>>,
    web_client: WebClient,


    state: GameState,
    player_healths: i8,
    scores: u32,
    health_icon: Rc<SpriteTex>,

    enemy_timer: f32,

    asteroids_count: u32
}

impl GameScene {
    pub fn spawn_asteroids(&mut self, config: &Config) {
        let mut rng = rand::rng();


        let asteroid_radius = 120.0;
        let player_entity = self.entities.iter().find(|x| x.get_entity_id() == self.player_id).unwrap();

        let player_pos: Vec2<f32> = player_entity.get_position().clone();
        while self.asteroids_ids.len() < self.asteroids_count as usize {
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

    pub fn spawn_asteroid(&mut self, config: &Config, mut rng: &mut ThreadRng, random_pos: Vec2<f32>,min_scale: f32, max_scale: f32) {



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

    pub fn spawn_enemy(&mut self, pos: Vec2<f32>, config: &Config, assets_db: &AssetsDB){
        let mut enemy = EnemyEntity::new(
            Transform::new(
                pos,
                Vec2::new(0.7, 0.7),
                0.0,
                config.size()
            ),
            assets_db.get_mesh_by_name("ufo_01").unwrap_or_default()
        );
        self.add_entity(Box::new(enemy));
    }

    pub fn spawn_enemy_timer(&mut self, delta_time: f32, config: &Config, assets_db: &AssetsDB) {
        self.enemy_timer += delta_time;
        if (self.enemy_timer >= 5.0){

            let mut is_can_spawn = true;
            let min_dist = 400.0;

            let mut rng = rand::rng();
            let random_pos = Vec2::new(
                rng.random_range(100..(config.x()-100)) as f32,
                rng.random_range(100..(config.y() - 100)) as f32
            );

            for entity in self.entities.iter() {
                if (entity.get_position().distance(random_pos) < min_dist){
                    is_can_spawn = false;
                    break;
                }
            }

            if (is_can_spawn) {
                self.spawn_enemy(random_pos, config, assets_db);
                self.enemy_timer = -10.0
            }
        }
    }


    pub fn spawn_debris(&mut self, config: &Config, assets_db: &AssetsDB){
        let mut rand = rand::rng();
        
        let count = rand.random_range(3..6);


        for i in 0..count {
            
        }
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

        self.asteroids_count = 5;
        self.health_icon = assets_db.get_sprite_by_name("heart").unwrap_or_default();
        self.player_id = self.add_entity(Box::new(player));

        self.spawn_asteroids(&config);
    }

    fn custom_events_solve(&mut self, scene_event: &Vec<SceneEvent>, config: &Config, asset_db: &AssetsDB, dt: f32) {


        self.spawn_enemy_timer(dt, config, asset_db);

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
                        self.remove_entity(self.player_id);
                    }
                }
                _=>{}
            }
        }

        if (self.asteroids_ids.len() == 0){
            self.asteroids_count += 2;
            self.spawn_asteroids(&config)
        }
    }

    fn ui(&mut self, ctx: &Context) -> SceneSwitch {
        match self.state {
            GameState::Active => {
                let frame = ui_transparent_frame();
                egui::TopBottomPanel::top("bottom_data")
                    .frame(frame)
                    .show_separator_line(false)
                    .min_height(35.0)
                    .show(ctx, |ui| {
                        ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                            ui.label(
                                egui::RichText::new(format!("SCORES: {}", self.scores))
                                    .size(20.0)
                                    .strong(),
                            );

                            let space = ui.available_width();
                            ui.add_space(space - (30.0 * self.player_healths as f32));

                            ui.horizontal(|ui| {
                                for _ in 0..self.player_healths {
                                    ui_draw_icon(ui, &self.health_icon, Vec2::new(20.0, 20.0));
                                }
                            });
                        });
                    });
            }
            GameState::End => {
                let can_send = self.web_client.is_available_name();
                let mut nickname = self.web_client.get_nickname();

                let frame = ui_transparent_frame();
                let mut scene_switch = SceneSwitch::None;

                egui::CentralPanel::default().frame(frame).show(ctx, |ui| {
                    let (avail, title_rect) = ui_title_rect(ui);
                    let (card_rect, card_frame) = ui_get_card_rect(avail);

                    ui.allocate_ui_at_rect(card_rect, |ui| {
                        card_frame.show(ui, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.label(
                                    egui::RichText::new(format!("SCORES\n{:06}", self.scores))
                                        .size(64.0)
                                        .strong(),
                                );

                                ui.add_space(18.0);

                                ui.label(egui::RichText::new("Enter your nickname").size(20.0));
                                ui.add_space(8.0);

                                let resp = ui.add_sized(
                                    [320.0, 40.0],
                                    egui::TextEdit::singleline(&mut nickname)
                                        .hint_text("Name...")
                                        .font(egui::FontId::proportional(26.0)),
                                );

                                if (resp.changed()) {
                                    self.web_client.change_nickname(nickname);
                                }
                                ui.add_space(18.0);


                                ui.add_enabled_ui(can_send, |ui| {
                                    if ui_button(ui, "Send")
                                    {
                                        self.web_client.send_web_data(self.scores);
                                        scene_switch = SceneSwitch::Switch(SceneId::Menu);
                                    }
                                });
                                ui.add_space(10.0);
                                if ui_button(ui, "Menu")
                                {
                                    scene_switch = SceneSwitch::Switch(SceneId::Menu);
                                }
                            });
                        });
                    });
                });


                return scene_switch;
            }
        }

        SceneSwitch::None
    }

    fn get_scene_name(&self) -> String{
        String::from("GameScene")
    }

    fn get_entities(&self) -> &Vec<Box<dyn Entity>> {
        &self.entities
    }

    fn get_entities_mut(&mut self) -> &mut Vec<Box<dyn Entity>> {
        &mut self.entities
    }
}



