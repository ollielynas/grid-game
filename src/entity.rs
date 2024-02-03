use std::io::Read;

use grid::Grid;
use macroquad::{experimental::scene::camera_pos, miniquad::FilterMode, texture::Texture2D, time::get_frame_time};

use crate::map::{Map, Pixel};

pub struct Entity {
    pub x: f32,
    pub y: f32,
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
            height,
            width,
            texture,
            entity_type,
        };
    }

    pub fn update(&mut self, grid: &Grid<Pixel>) -> bool {
        let pixel = grid[(self.y as usize, self.x as usize)];
        let delta = get_frame_time();
        if self.y >= grid.size().0 as f32 {
            return false;
        }
        if self.x > grid.size().1 as f32 {
            return false;
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
                self.y -= 10.0*delta;
                if self.y < 0.0 {
                    return false;
                }
            },
            EntityType::Fish{mut air} => {

                if pixel.is_airy() {
                    self.entity_type = EntityType::Fish { air: air-delta*5.0 };
                    self.y += 10.0*delta
                }else if pixel != Pixel::Water {
                    self.y -= 11.0*delta;
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

pub enum EntityType {
    Tree,
    Soul,
    Fish{air: f32},
}

/// (width, height)
impl EntityType {
    pub fn scale(&self) -> f32 {
        match self {
            EntityType::Tree => 1.0/4.0,
            EntityType::Soul => 1.0/8.0,
            EntityType::Fish{air:_} => 1.0/5.0,
        }
    }
    pub fn gravity(&self) -> f32 {
        match self {
            EntityType::Tree => 0.0,
            EntityType::Soul => -1.0,
            EntityType::Fish{air:_} => 1.0,
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
                }
        ).unwrap_or(include_bytes!("textures/error.png").to_vec()).as_slice(),
            None,
        )
    }


    
}
