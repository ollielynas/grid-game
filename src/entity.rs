
use std::clone;

use grid::Grid;
use egui_macroquad::macroquad::{math::{Rect, Vec2}, miniquad::FilterMode, texture::Texture2D, time::get_frame_time};
use savefile_derive::Savefile;

use crate::{map::{Map, Pixel}, physics::{self, CollisionDirection}};
#[derive(PartialEq, Debug, Clone)]
// #[derive(PartialEq, Debug, Clone, Savefile)]

pub struct BoidData {
    pub vx: f32,
    pub vy: f32,
    pub x: f32,
    pub y: f32,
}
pub struct Entity {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub height: f32,
    pub width: f32,
    pub texture: Texture2D,
    pub entity_type: EntityType,
}

impl Entity {
    pub fn new(entity_type: EntityType, x: f32, y: f32) -> Entity {
        let scale = entity_type.scale();
        let texture = entity_type.texture();
        texture.set_filter(FilterMode::Nearest);
        let height = texture.height() * scale;
        let width = texture.width() * scale;


        return Entity {
            x,
            y,
            vx: 0.0,
            vy: 0.,
            height,
            width,
            texture,
            entity_type,
        };
    }

    pub fn update(&mut self, grid: &Grid<Pixel>, boid_data: &Vec<BoidData>) -> bool {
        let pixel = grid[(self.y as usize, self.x as usize)];
        let delta = get_frame_time();

        if self.y >= grid.size().0 as f32 || self.y < 0.0 {
            return false;
        }
        if self.x > grid.size().1 as f32 || self.x < 0.0 {
            return false;
        }

        let physics = match self.entity_type {
            EntityType::Fish {..}|
            EntityType::Boid => true,
         _ => false
        };

        let terrain_hit = physics::make_map_box(
            grid, 
            Rect::new(self.x - 5.0, self.y - 5.0, 10.0, 10.0), 
            true, 
            self.x, 
            self.y
        );

        if physics {
            let mut remaining = delta;

            while remaining > 0.0 {
                let dp = Vec2::new(self.vx, self.vy) * remaining;
    
                /*let collision = self
                    .get_player_box(0.0, 0.0)
                    .get_collision_with(&terrain_hit, dp);*/

                let bb = physics::make_bounding_box(Rect::new(self.x, self.y, self.texture.width() * self.entity_type.scale(), self.texture.height() * self.entity_type.scale()));
                let collision = bb.get_collision_with(&terrain_hit, dp);

                match collision {
                    None => {
                        self.x += self.vx * remaining;
                        self.y += self.vy * remaining;
    
                        remaining = 0.0;
                    }
    
                    Some(collision) => {
                        self.x += self.vx * collision.time * delta;
                        self.y += self.vy * collision.time * delta;
    
                        match collision.dir {
                            CollisionDirection::Left | CollisionDirection::Right => {
                                self.vx = 0.0;
                            }
    
                            CollisionDirection::Down | CollisionDirection::Up => {
                                self.vy = 0.0;
                            }
                        }
    
                        remaining -= collision.time;
                    }
                }
            }
        } else {
            self.x += self.vx * delta;
            self.y += self.vy * delta;
        }

        match self.entity_type {
            EntityType::Tree => {
                if !pixel.is_airy() {
                    return false;
                }
                if grid[(self.y as usize +1, self.x as usize)].is_airy() {
                    return false;
                }
            },
            EntityType::Soul => {
                self.vy = -10.0;
                if self.y <= 2.0 {
                    return false;
                }
            },

            
            
            EntityType::Boid => {
                const RANDOMNESS: f32 = 10.0; 
                const RANGE: f32 = 8.0; 
                const REPEL: f32 = 4.0;
                const SPEED: f32 = 6.0;
                const FOLLOW: f32 = 3.0;
                const MOMENTUM: f32 = 8.0;

                self.vx *= MOMENTUM;
                self.vy += MOMENTUM;

                self.vx += (fastrand::f32() - 0.5) * RANDOMNESS;
                self.vy +=(fastrand::f32() - 0.5) * RANDOMNESS;

                for data in boid_data {


                    if self.x == data.x || self.y == data.y {
                        continue;
                    }
                    
                    let distance_x = self.x - data.x;
                    let distance_y = self.y - data.y;
                    let distance = (distance_x).hypot(distance_y);
                    if distance < 0.01 {
                        continue;
                    }

                    if distance < RANGE * 0.25 {
                        self.vx += REPEL / distance_x;
                        self.vy += REPEL / distance_y;
                    }
                    if distance > RANGE * 0.75 && distance < RANGE {
                        self.vx -= REPEL / distance_x;
                        self.vy -= REPEL / distance_y;
                    }
                    
                    if distance < RANGE {
                        self.vx += data.vx * FOLLOW;
                        self.vy += data.vy * FOLLOW;
                    }
                    
                    
                }

                let mut h = self.vx.hypot(self.vy) / SPEED;

                if h < 0.00000001 {
                    h = 1.0;
                }

                self.vx /= h;
                self.vy /= h;
                // self.vx = 0.1;
                // self.vy = 0.1;


            },
            EntityType::Fish{air} => {
                
                
                
                if pixel.is_airy() {
                    self.vy = 5.0;
                    self.entity_type = EntityType::Fish { air: air-delta*5.0 };
                }else {
                    if fastrand::f32() > 0.99 {
                        self.vx += (fastrand::f32() - 0.5) * 10.0
                    }
                    if fastrand::f32() > 0.99 {
                        self.vy += (fastrand::f32() - 0.5) * 10.0
                    }

                    self.vx *= 0.9;
                    self.vy *= 0.9;
                }

                if air <= 0.0 {
                    let new = Entity::new(EntityType::Soul, self.x, self.y);
                    self.texture = new.texture;
                    self.height = new.height;
                    self.width = new.width;
                    self.entity_type = new.entity_type;
                }
            },
        }

        self.y = self.y.clamp(2.0, grid.size().0 as f32-2.0);
        self.x = self.x.clamp(2.0, grid.size().0 as f32-2.0);
        
        return true;
    }
}


#[derive(PartialEq, Clone, Copy, Debug)]
pub enum EntityType {
    Tree,
    Soul,
    Fish{air: f32},
    Boid,
}

/// (width, height)
impl EntityType {
    pub fn scale(&self) -> f32 {
        match self {
            EntityType::Tree => 1.0/4.0,
            EntityType::Soul => 1.0/8.0,
            EntityType::Fish{air:_} => 1.0/5.0,
            EntityType::Boid => 1.0/5.0,
        }
    }
    
    
    fn texture(&self) -> Texture2D {
        
        Texture2D::from_file_with_format(
            (
                match self {
                EntityType::Tree => fastrand::choice(vec![
                    include_bytes!("textures/tree/tree1.png").to_vec(),
                    include_bytes!("textures/tree/tree2.png").to_vec()
                    ]),
                EntityType::Soul => fastrand::choice(vec![
                    include_bytes!("textures/soul/soul1.png").to_vec(),
                    ]),
                EntityType::Fish{air:_} => fastrand::choice(vec![
                    include_bytes!("textures/fish/fish1.png").to_vec(),
                    ]),
                EntityType::Boid => fastrand::choice(vec![
                    include_bytes!("textures/fish/fish1.png").to_vec(),
                    ]),
                
                }
        ).unwrap_or(include_bytes!("textures/error.png").to_vec()).as_slice(),
            None,
        )
    }


    
}
