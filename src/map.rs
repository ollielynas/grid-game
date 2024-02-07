use grid::*;
use macroquad::prelude::Rect;
use macroquad::{
    color::{Color, WHITE},
    texture::Image,
};
use std::default;

use perlin2d::PerlinNoise2D;

use crate::entity::{Entity, EntityType};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Pixel {
    Air,
    Sand,
    Dirt,
    Stone,
    Water,
    Fire,
    Grass,
    Wood,
    Bedrock,
    Smoke,
    Steam,
    Gold,
    Lava,
}

impl Default for Pixel {
    fn default() -> Self {
        return Pixel::Air;
    }
}

impl Pixel {
    fn color(&self) -> Color {
        match self {
            Pixel::Air => Color::from_rgba(250, 251, 255, 0),
            Pixel::Sand => Color::from_rgba(207, 215, 157, 255),
            Pixel::Fire => Color::from_rgba(193, 84, 45, 255),
            Pixel::Wood => Color::from_rgba(139, 107, 59, 255),
            Pixel::Smoke => Color::from_rgba(190,190,190, 255),
            Pixel::Steam => Color::from_rgba(199,213,224, 255),
            Pixel::Water => Color::from_rgba(35,69,190, 150),
            Pixel::Dirt => Color::from_rgba(155,118,83, 255),
            Pixel::Stone => Color::from_rgba(168,169,173, 255),
            Pixel::Grass => Color::from_rgba(113,169,44, 255),
            Pixel::Gold => Color::from_rgba(205, 127, 50, 255),
            Pixel::Lava => Color::from_rgba(247, 104, 6, 255),
            Pixel::Bedrock => Color::from_rgba(fastrand::u8(0..255), fastrand::u8(0..255), fastrand::u8(0..255), 255),
        }
    }

    pub fn cycle(&self) -> Pixel {
        return match self {
            Pixel::Air => Pixel::Sand,
            Pixel::Sand => Pixel::Fire,
            Pixel::Fire =>Pixel::Wood,
            Pixel::Wood => Pixel::Smoke,
            Pixel::Smoke => Pixel::Water,
            Pixel::Water => Pixel::Bedrock,
            Pixel::Bedrock => Pixel::Stone,
            Pixel::Stone => Pixel::Dirt,
            Pixel::Dirt => Pixel::Grass,
            Pixel::Grass => Pixel::Gold,
            Pixel::Gold => Pixel::Lava,
            Pixel::Lava => Pixel::Steam,
            Pixel::Steam => Pixel::Air,
        }

    }

    pub fn is_airy(&self) -> bool {
        return matches!(self, Pixel::Air | Pixel::Fire | Pixel::Smoke | Pixel::Steam);
    }
    pub fn fluid_density(&self) -> Option<i32> {
        match self {
            Pixel::Air => Some(3),
            Pixel::Sand|Pixel::Dirt|Pixel::Lava => Some(30),
            Pixel::Smoke => Some(1),
            Pixel::Steam => Some(1),
            Pixel::Water => Some(15),
            Pixel::Fire => Some(2),
            Pixel::Bedrock
            |Pixel::Wood 
            |Pixel::Stone
            |Pixel::Gold
            |Pixel::Grass => None ,
        }
    }

    pub fn can_hit(&self) -> bool {
        match self {
            Pixel::Sand | Pixel::Dirt | Pixel::Bedrock | Pixel::Wood | Pixel::Stone | Pixel::Gold | Pixel::Grass => true,
            Pixel::Air | Pixel::Lava | Pixel::Steam | Pixel::Water | Pixel::Fire | Pixel::Smoke => false
        }
    }

    pub fn less_dense(&self, p: Pixel) -> bool {
        self.fluid_density().unwrap_or(69) < p.fluid_density().unwrap_or(98)
    }

    pub fn is_flammable(&self) -> bool {
        return matches!(self, Pixel::Wood);
    }
}

pub struct Map {
    pub grid: Grid<Pixel>,
    pub size: u32,
    pub update_texture_px: Vec<(usize, usize)>,
    pub image: Image,
    pub entities: Vec<Entity>,
    // pub heatmap: Image,
}

impl Map {


    pub fn gen_terrain(&mut self) {

        self.entities = vec![];

        self.make_square(Pixel::Air);
        let new_grid = self.grid.clone();

    let perlin = PerlinNoise2D::new(
    // octaves - The amount of detail in Perlin noise.
    5, 
    // amplitude - The maximum absolute value that the Perlin noise can output.
    10.0, 
    // frequeny - The number of cycles per unit length that the Perlin noise outputs.
    1.5, 
    // persistence - A multiplier that determines how quickly the amplitudes diminish for each successive octave in a Perlin-noise function.
    4.0, 
    // lacunarity - A multiplier that determines how quickly the frequency increases for each successive octave in a Perlin-noise function.
    2.0, 
    // scale - A Tuple. A number that determines at what distance to view the noisemap.
    (100.0, 100.0), 
    // bias - Amount of change in Perlin noise. U
    0.1, 
    // seed - A value that changes the output of a coherent-noise function.
    fastrand::i32(0..200)
);
    let perlin2 = PerlinNoise2D::new(
    // octaves - The amount of detail in Perlin noise.
    5, 
    // amplitude - The maximum absolute value that the Perlin noise can output.
    10.0, 
    // frequeny - The number of cycles per unit length that the Perlin noise outputs.
    1.5, 
    // persistence - A multiplier that determines how quickly the amplitudes diminish for each successive octave in a Perlin-noise function.
    4.0, 
    // lacunarity - A multiplier that determines how quickly the frequency increases for each successive octave in a Perlin-noise function.
    2.0, 
    // scale - A Tuple. A number that determines at what distance to view the noisemap.
    (500.0, 500.0), 
    // bias - Amount of change in Perlin noise. U
    0.1, 
    // seed - A value that changes the output of a coherent-noise function.
    fastrand::i32(0..200)
);
    let perlin3 = PerlinNoise2D::new(
    // octaves - The amount of detail in Perlin noise.
    5, 
    // amplitude - The maximum absolute value that the Perlin noise can output.
    10.0, 
    // frequeny - The number of cycles per unit length that the Perlin noise outputs.
    1.5, 
    // persistence - A multiplier that determines how quickly the amplitudes diminish for each successive octave in a Perlin-noise function.
    4.0, 
    // lacunarity - A multiplier that determines how quickly the frequency increases for each successive octave in a Perlin-noise function.
    2.0, 
    // scale - A Tuple. A number that determines at what distance to view the noisemap.
    (60.0, 60.0), 
    // bias - Amount of change in Perlin noise. U
    0.1, 
    // seed - A value that changes the output of a coherent-noise function.
    fastrand::i32(0..200)
);
    

        for ((row, col), p) in new_grid.indexed_iter() {
            if perlin.get_noise(col as f64, row as f64) > -10.0 {
                if perlin2 .get_noise(col as f64, row as f64) > 100.0 {
                    self.grid[(row,col)] = Pixel::Sand;

                }else {
                self.grid[(row,col)] = Pixel::Dirt;
                }
            }
            if perlin.get_noise(col as f64, row as f64) > 80.0 {
                
                if perlin3.get_noise(col as f64, row as f64) > 1200.0 {
                    self.grid[(row,col)] = Pixel::Gold;

                }else {
                self.grid[(row,col)] = Pixel::Stone;
                }
            }
            if perlin.get_noise(col as f64, row as f64) < -1000.0 {
                if row as f32 > self.size as f32 * 0.75 {
                self.grid[(row,col)] = Pixel::Lava;
            }else {
                self.grid[(row,col)] = Pixel::Water;
            }
                
            }
            self.update_texture_px.push((row, col));
        }
        for _ in 0..10 {self.update_state()}
        for ((row, col), p) in new_grid.indexed_iter() {
            let num = fastrand::u32(0..1000);
            match self.grid[(col,row)] {
                Pixel::Water => {
                    if num < 10 {
                        self.spawn_entity(EntityType::Fish{air:20.0}, row as f32, col as f32);
                    }
                },
                Pixel::Grass => {
                    if num < 100 {
                        self.spawn_entity(EntityType::Tree, row as f32, col as f32-1.0);
                    }
                },
                |Pixel::Air
                |Pixel::Sand
                |Pixel::Dirt
                |Pixel::Stone
                |Pixel::Fire
                |Pixel::Gold
                |Pixel::Steam
                |Pixel::Wood 
                |Pixel::Bedrock
                |Pixel::Lava
                |Pixel::Smoke => {}, 
            }
        }
    } 

    pub fn new_square(size: usize) -> Map {
        let grid = Grid::from_vec((0..size.pow(2)).map(|_| Pixel::Air).collect(), size);

        return Map {
            grid,
            size: size as u32,
            update_texture_px: vec![],
            image: Image::gen_image_color(size as u16, size as u16, WHITE),
            entities: vec![],
            // heatmap: Image::gen_image_color(size as u16, size as u16, WHITE),
        };
    }

    pub fn spawn_entity(&mut self, entity_type:EntityType, x: f32,y:f32) {
        self.entities.push(Entity::new(entity_type, x, y));
    }

    pub fn make_square(&mut self, pixel: Pixel) {
        let third = (self.size / 3) as usize;

        for ((row, col), i) in self.grid.indexed_iter_mut() {
            *i = if row > third && row < third * 2 && col > third && col < third * 2 {
                self.update_texture_px.push((row, col));
                pixel
            } else {
                self.update_texture_px.push((row, col));
                Pixel::Air
            };
        }
    }
    pub fn make_log(&mut self) {
        let third = (self.size / 3) as usize;

        for ((row, col), i) in self.grid.indexed_iter_mut() {
            *i = if row > third && row < third * 2 && col > third && col < third * 2 {
                self.update_texture_px.push((row, col));
                Pixel::Wood
            } else {
                self.update_texture_px.push((row, col));
                Pixel::Air
            };
        }
        self.grid[(third,third+1)] = Pixel::Fire;
        self.grid[(third+1,third)] = Pixel::Fire;
        self.grid[(third+1,third+1)] = Pixel::Fire;
        self.grid[(third+2,third+1)] = Pixel::Fire;
        self.grid[(third+3,third+2)] = Pixel::Fire;
    }

    pub fn update_image(&mut self) {

        for (row, col) in &self.update_texture_px {


            self.image.set_pixel(
                *col as u32,
                *row as u32,
                self.grid.get(*row, *col).unwrap_or(&Pixel::Air).color(),
            )
        }


    }
}
