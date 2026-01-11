use std::collections::HashMap;
use crate::classes::t_entity::Entity;
use crate::collisions_lib::e_col_layers::ColLayer;
use crate::scenes_lib::e_scene_event::SceneEvent;




pub const LAYER_COUNT: usize = 5;
pub const COLLISION_TABLE: [[bool; LAYER_COUNT]; LAYER_COUNT] = [
    //            Player  Asteroid BulletP BulletE, Enemy
    /* Player */ [false,  true,    false,  true , true],
    /* Astero */ [true,   false,   true,   true,  true],
    /* BulletP*/ [false,  true,    false,  false, true],
    /* BulletE*/ [true,   false,   false,  false, false],
    /* Enemy*/   [true,   true,    true,   false, false],
];

pub fn solve_collision(entity: &mut Vec<Box<dyn Entity>>) -> Vec<SceneEvent> {
    let mut events = vec![];

    for i in 0..entity.len() {
        for j in i + 1..entity.len() {
            let entity1 = &entity[i];
            let entity2 = &entity[j];

            if (entity2.get_entity_id() != entity1.get_entity_id()) {
                if (entity1.can_collide() && entity2.can_collide()) {
                    let layer1 = entity1.get_collision_layer();
                    let layer2 = entity2.get_collision_layer();


                    if (COLLISION_TABLE[layer1.idx()][layer2.idx()]) {
                        let col_mesh_1 = entity1.get_collision_mesh();
                        let col_mesh_2 = entity2.get_collision_mesh();


                        if (col_mesh_1.is_some() && col_mesh_2.is_some()) {
                            let (mesh1, transform1) = col_mesh_1.unwrap();
                            let (mesh2, transform2) = col_mesh_2.unwrap();

                            for line_col1 in mesh1.get_lines() {
                                let a = transform1.transform_point_to_world(line_col1.start);
                                let b = transform1.transform_point_to_world(line_col1.end);

                                for line_col2 in mesh2.get_lines() {
                                    let c = transform2.transform_point_to_world(line_col2.start);
                                    let d = transform2.transform_point_to_world(line_col2.end);

                                    if seg_intersect(a, b, c, d) {
                                        events.push(SceneEvent::Collision {
                                            a: entity1.get_entity_id(),
                                            b: entity2.get_entity_id(),
                                        });
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }


    for event in &events {
        if let SceneEvent::Collision { a, b } = event {
            
            let (layer_a, layer_b) = {
                let ea = entity.iter().find(|e| e.get_entity_id() == *a).unwrap();
                let eb = entity.iter().find(|e| e.get_entity_id() == *b).unwrap();
                (ea.get_collision_layer(), eb.get_collision_layer())
            };

            // теперь по одному mutable за раз
            if let Some(ea) = entity.iter_mut().find(|e| e.get_entity_id() == *a) {
                ea.on_collision(layer_b);
            }
            if let Some(eb) = entity.iter_mut().find(|e| e.get_entity_id() == *b) {
                eb.on_collision(layer_a);
            }
        }
    }

    
    events
}


#[inline]
fn cross(a: vek::Vec2<f32>, b: vek::Vec2<f32>) -> f32 {
    a.x * b.y - a.y * b.x
}

#[inline]
fn orient(a: vek::Vec2<f32>, b: vek::Vec2<f32>, c: vek::Vec2<f32>) -> f32 {
    cross(b - a, c - a)
}

#[inline]
fn on_segment(a: vek::Vec2<f32>, b: vek::Vec2<f32>, p: vek::Vec2<f32>, eps: f32) -> bool {
    let minx = a.x.min(b.x) - eps;
    let maxx = a.x.max(b.x) + eps;
    let miny = a.y.min(b.y) - eps;
    let maxy = a.y.max(b.y) + eps;
    p.x >= minx && p.x <= maxx && p.y >= miny && p.y <= maxy
}

pub fn seg_intersect(a: vek::Vec2<f32>, b: vek::Vec2<f32>, c: vek::Vec2<f32>, d: vek::Vec2<f32>) -> bool {
    let eps = 1e-6;

    let o1 = orient(a, b, c);
    let o2 = orient(a, b, d);
    let o3 = orient(c, d, a);
    let o4 = orient(c, d, b);

    // общий случай (строгое пересечение)
    if (o1 > eps && o2 < -eps || o1 < -eps && o2 > eps) &&
        (o3 > eps && o4 < -eps || o3 < -eps && o4 > eps) {
        return true;
    }

    // касания/коллинеарность
    if o1.abs() <= eps && on_segment(a, b, c, eps) { return true; }
    if o2.abs() <= eps && on_segment(a, b, d, eps) { return true; }
    if o3.abs() <= eps && on_segment(c, d, a, eps) { return true; }
    if o4.abs() <= eps && on_segment(c, d, b, eps) { return true; }

    false
}
