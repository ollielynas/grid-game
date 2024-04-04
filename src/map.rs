

use core::fmt;
use std::collections::HashMap;
use std::{collections::HashSet, fmt::Display};
use std::fs::create_dir_all;


use grid::*;
use egui_macroquad::{macroquad::{
    color::{Color, WHITE}, math::Rect, texture::Image
}};
use savefile::{load_file, save_file};
use savefile_derive::Savefile;
use strum_macros::EnumIter;

use perlin2d::PerlinNoise2D;
    
use crate::game_ui::display_message;
use crate::settings::Settings;
use crate::{entity::{Entity, EntityType}, SAVEFILE_VERSION};

// #[repr(C)] 
#[derive(Copy, Clone, PartialEq, Eq, Debug, EnumIter, Savefile, Hash)]
pub enum Pixel {
    Air,
    Sand,
    Dirt,
    Stone,
    Water,
    Candle,
    Fire,
    Grass,
    Wood,
    Bedrock,
    Smoke,
    Steam,
    Gold,
    Oil,
    Glass,
    Lava,
    Explosive,
    LiveWood,
    Seed,
    Leaf,
    Lamp,
    Loot,
}

impl Default for Pixel {
    fn default() -> Self {
        return Pixel::Air;
    }
}

impl Pixel {


    pub fn color(&self) -> Color {
        match self {
            Pixel::Air => Color::from_rgba(250, 251, 255, 0),
            Pixel::Lamp => Color::from_rgba(250, 231, 235, 255),
            Pixel::Sand => Color::from_rgba(207, 215, 157, 255),
            Pixel::Fire => Color::from_rgba(193, 84, 45, 255),
            Pixel::Wood => Color::from_rgba(139, 107, 59, 255),
            Pixel::LiveWood => Color::from_rgba(139, 107, 59, 255),
            Pixel::Seed => Color::from_rgba(113,169,44, 155),
            Pixel::Leaf => Color::from_rgba(113,149,44, 155),
            Pixel::Smoke => Color::from_rgba(190,190,190, 255),
            Pixel::Steam => Color::from_rgba(199,213,224, 255),
            Pixel::Water => Color::from_rgba(35,69,190, 150),
            Pixel::Dirt => Color::from_rgba(155,118,83, 255),
            Pixel::Stone => Color::from_rgba(168,169,173, 255),
            Pixel::Grass => Color::from_rgba(113,169,44, 255),
            Pixel::Gold => Color::from_rgba(205, 127, 50, 255),
            Pixel::Lava => Color::from_rgba(247, 104, 6, 255),
            Pixel::Oil => Color::from_rgba(0, 0, 0, 255),
            Pixel::Loot => Color::from_rgba(255, 105, 180, 255),
            Pixel::Candle => Color::from_rgba(239, 230, 211, 255),
            Pixel::Glass => Color::from_rgba(100, 104, 230, 5),
            Pixel::Bedrock => Color::from_rgba(fastrand::u8(0..255), fastrand::u8(0..255), fastrand::u8(0..255), 255),
            Pixel::Explosive => Color::from_rgba(242, 33, 5, 255),
        }
    }

    pub fn light_emission(&self) -> Color {
        match self {
            Pixel::Air => {Color::new(1.0, 1.0, 1.0, 0.4)},
            Pixel::Glass => {Color::new(1.0, 1.0, 1.0, 0.4)},
            Pixel::Steam | Pixel::Smoke => {Color::new(1.0, 1.0, 1.0, 0.4)},
            Pixel::Water => {Color::new(1.0, 1.0, 1.0, 0.5)},
            Pixel::Lava => {Color::new(1.0, 0.0, 0.0, 0.0)},
            Pixel::Lamp => {Color::new(1.0, 0.0, 0.0, 0.0)},
            Pixel::Fire => {Color::new(1.0, 0.0, 0.0, 0.0)},
            Pixel::Leaf => {Color::new(0.0, 1.0, 0.0, 0.5)},
            Pixel::Loot => {Color::new(0.0, 1.0, 0.0, 0.3)},
            _ => {Color::new(0.0, 0.0, 0.0, 1.0)}
        }
    }

    pub fn fluid(&self) -> bool {
        matches!(self , Pixel::Lava | Pixel::Water | Pixel::Oil)
    }

    

    pub fn is_airy(&self) -> bool {
        return matches!(self, Pixel::Air | Pixel::Fire | Pixel::Smoke | Pixel::Steam);
    }

    pub fn fluid_density(&self) -> Option<i32> {
        match self {
            Pixel::Air => Some(3),
            Pixel::Sand|Pixel::Dirt|Pixel::Lava|Pixel::Grass|Pixel::Explosive| Pixel::Seed => Some(30),
            Pixel::Smoke => Some(1),
            Pixel::Steam => Some(1),
            Pixel::Water => Some(15),
            Pixel::Oil => Some(10),
            Pixel::Fire => Some(2),
            Pixel::Bedrock
            |Pixel::Wood 
            |Pixel::Lamp 
            |Pixel::Leaf 
            |Pixel::LiveWood 
            |Pixel::Stone
            |Pixel::Loot
            |Pixel::Candle
            |Pixel::Glass
            |Pixel::Gold => None,
        }
    }

    pub fn heat_product(&self) -> Option<Self> {
        match self {
            Pixel::Wood => Some(Pixel::Fire),
            Pixel::LiveWood => Some(Pixel::Fire),
            Pixel::Oil => Some(Pixel::Fire),
            Pixel::Sand => Some(Pixel::Glass),
            Pixel::Water => Some(Pixel::Steam),
            Pixel::Leaf => if fastrand::f32() > 0.95 {Some(Pixel::Seed)} else {Some(Pixel::Fire)},
            _ => None
        }
    }

    pub fn ignition_probability(&self) -> f32 {
        match self {
            Pixel::Wood => 5.0,
            Pixel::LiveWood => 40.0,
            Pixel::Leaf => 40.0,
            Pixel::Oil => 20.0,
            Pixel::Water => 50.0,
            Pixel::Sand => 0.01,
            Pixel::Explosive => 100.0,
            _ => 0.0
        }
    }

    pub fn extinguish_fire(&self) -> bool {
        match self {
            Pixel::Water => true,
            _ => false
        }
    }

    pub fn player_damage(&self) -> f32 {
        match self {
            Self::Fire => 1.0,
            Self::Steam => 0.1,
            Self::Lava => 10.0,
            _ => 0.0,
        }
    } 

    pub fn can_hit(&self) -> bool {
        match self {
            Pixel::Loot | Pixel::Candle | Pixel::Glass |Pixel::Sand | Pixel::Dirt | Pixel::Bedrock | Pixel::Wood | Pixel::Stone | Pixel::Gold | Pixel::Grass | Pixel::Explosive => true,
            Pixel::Lamp | Pixel::LiveWood | Pixel::Leaf | Pixel::Seed | Pixel::Oil |Pixel::Air | Pixel::Lava | Pixel::Steam | Pixel::Water | Pixel::Fire | Pixel::Smoke => false
        }
    }

    pub fn less_dense(&self, p: Pixel) -> bool {
        self.fluid_density().unwrap_or(69) < p.fluid_density().unwrap_or(98)
    }


}

pub enum Biome {
    Surface,
    Space,
    Cave,
}


impl Display for Biome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Biome::Surface => "Surface",
            Biome::Space => "Space",
            Biome::Cave => "Cave",
        };
        write!(f, "{s}")
    }
}

#[derive(Savefile)]
struct MapSave {
    pixel_vector: Vec<Pixel>,
    size: u32,
    realistic_fluid: bool,
    name: String,

}

impl MapSave {
    fn from_map(map:&Map) -> MapSave {
        MapSave { 
            name: map.name.clone(),
            pixel_vector: map.grid.clone().into_vec(),
            size: map.size,
            realistic_fluid: map.realistic_fluid,
        }
    }

    fn to_map(self) -> Map {
        let mut new_map = Map::new_square(self.size as usize, self.name);

        new_map.grid = Grid::from_vec(self.pixel_vector.clone(), self.size as usize);

        return new_map;
    }

}
// #[derive(Clone)]
pub struct Map {
    pub grid: Grid<Pixel>,
    pub size: u32,
    pub update_texture_px: HashSet::<(usize, usize)>,
    pub image: Image,
    pub light_mask: Image,
    pub entities: Vec<Entity>,
    pub name: String,
    pub detected_air: Grid<i16>,
    pub detected_fluids: Grid<bool>,
    pub realistic_fluid: bool,
    pub sky_light: Vec<usize>,
    pub block_percent: HashMap<Pixel, i16>,
    pub biome: Biome,
    pub settings: Settings,
    // pub heatmap: Image,
}

impl Map {

    
    pub fn save(&self) {
        if let Err(error) = create_dir_all("saves/maps/") {
            println!("error {error}");
        }
        let save = MapSave::from_map(self);
        save_file(format!("saves/maps/{}.map_save", self.name), SAVEFILE_VERSION, &save);
    }
    pub fn load(name: &str) -> Map {
        let save: Result<MapSave, savefile::prelude::SavefileError> = load_file(format!("saves/maps/{}.map_save", name), SAVEFILE_VERSION);
        if let Ok(save) = save {
            return save.to_map();
        }else {
            return Map::new_square(150, "error".to_owned());
        }
    }
    /// creates a randomly generated map based on perlin noise
    pub fn gen_terrain(&mut self) {

        self.entities = vec![];

        self.make_square(Pixel::Air);
        let new_grid = self.grid.clone();
    
    // display_message("making perlin noise").await;

    let perlin = PerlinNoise2D::new(
    // octaves - The amount of detail in Perlin noise.
    5, 
    // amplitude - The maximum absolute value that the Perlin noise can output.
    10.0, 
    // frequency - The number of cycles per unit length that the Perlin noise outputs.
    1.5, 
    // persistence - A multiplier that determines how quickly the amplitudes diminish for each successive octave in a Perlin-noise function.
    4.0, 
    // lacunarity - A multiplier that determines how quickly the frequency increases for each successive octave in a Perlin-noise function.
    2.0, 
    // scale - A Tuple. A number that determines at what distance to view the noise map.
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
    // frequency - The number of cycles per unit length that the Perlin noise outputs.
    1.5, 
    // persistence - A multiplier that determines how quickly the amplitudes diminish for each successive octave in a Perlin-noise function.
    4.0, 
    // lacunarity - A multiplier that determines how quickly the frequency increases for each successive octave in a Perlin-noise function.
    2.0, 
    // scale - A Tuple. A number that determines at what distance to view the noise map.
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
    // frequency - The number of cycles per unit length that the Perlin noise outputs.
    1.5, 
    // persistence - A multiplier that determines how quickly the amplitudes diminish for each successive octave in a Perlin-noise function.
    4.0, 
    // lacunarity - A multiplier that determines how quickly the frequency increases for each successive octave in a Perlin-noise function.
    2.0, 
    // scale - A Tuple. A number that determines at what distance to view the noise map.
    (60.0, 60.0), 
    // bias - Amount of change in Perlin noise. U
    0.1, 
    // seed - A value that changes the output of a coherent-noise function.
    fastrand::i32(0..200)
);

        for ((row, col), _) in new_grid.indexed_iter() {
            if row%10 == 0 && col%10 == 0 {
                // display_message(&format!("generating terrain {row}X{col} / {}{} ", self.size,self.size)).await;
            }
            if perlin.get_noise(col as f64, row as f64) > -10.0 {
                if perlin2 .get_noise(col as f64, row as f64) > 100.0 {
                    self.grid[(row,col)] = Pixel::Sand;

                }else {
                self.grid[(row,col)] = Pixel::Dirt;
                }
            }
            if perlin.get_noise(col as f64, row as f64) > 80.0 || ((row as f32) < self.size as f32 * 0.35 && perlin3.get_noise(col as f64, row as f64) > 50.0) {
                
                if perlin3.get_noise(col as f64, row as f64) > 1200.0 {
                    self.grid[(row,col)] = Pixel::Gold;
                }else if fastrand::f32() < 0.0005 && row as f32 > self.size as f32 * 0.6 {
                    self.grid[(row,col)] = Pixel::Loot;
                } else {
                    self.grid[(row,col)] = Pixel::Stone;
                
                }
            }
            if perlin.get_noise(col as f64, row as f64) > 150.0 && row as f32 > self.size as f32 * 0.85 {
                
                if perlin3.get_noise(col as f64, row as f64) > 1000.0 {
                    self.grid[(row,col)] = Pixel::Oil;
                }
            }

            if perlin.get_noise(col as f64, row as f64) < -1000.0 {
                if row as f32 > self.size as f32 * 0.75 {
                self.grid[(row,col)] = Pixel::Lava;
            }else {
                self.grid[(row,col)] = Pixel::Water;
            }
            
            
        }
        if (row as f32) < self.size as f32 * 0.25 {
            self.grid[(row,col)] = Pixel::Air;
            if (row as f32) > self.size as f32 * 0.22 {
                self.grid[(row,col)] = Pixel::Dirt;
            }
        }
            self.update_texture_px.insert((row, col));
        }
        for ((row, col), _) in new_grid.indexed_iter() {
            let num = fastrand::u32(0..1000);
            match self.grid[(col,row)] {
                Pixel::Water => {
                    
                },
                Pixel::Grass => {

                },
                Pixel::Air if col < self.size as usize / 4 => {
                    if num < 2 {
                        // self.spawn_entity(EntityType::Boid, row as f32, col as f32);
                    }
                }
                _ => {}, 
            }
        }
        for i in 2..(self.size -2) {
            if fastrand::f32() > 0.95 {
            self.grid[((self.size as f32 * 0.22) as usize -1, i as usize)] = Pixel::Seed;
            }
        }
    } 

    /// makes a new square map of the given `usize`
    pub fn new_square(size: usize, name: String) -> Map {

        let mut settings = Settings::default();

        settings.dynamic_simulation_distance = size > 600;

        let grid = Grid::from_vec(
            vec![Pixel::Air;size.pow(2)], size);

        Map {
            grid,
            size: size as u32,
            update_texture_px: HashSet::default(),
            image: Image::gen_image_color(size as u16, size as u16, WHITE),
            light_mask: Image::gen_image_color(size as u16, size as u16, Color { r: 0.0, g: 0.0, b: 0.0, a: 0.3 }),
            entities: vec![],
            detected_air: Grid::from_vec(vec![0; size.pow(2) as usize], size as usize),
            detected_fluids: Grid::from_vec(vec![false; size.pow(2) as usize], size as usize),
            name,
            realistic_fluid: true,
            sky_light: vec![0;size],
            block_percent: HashMap::default(),
            biome: Biome::Surface,
            settings,
        }
    }


    /// add an entity at the given coords
    pub fn spawn_entity(&mut self, entity_type:EntityType, x: f32,y:f32) {
        self.entities.push(Entity::new(entity_type, x, y));
    }

    /// makes a square of any malarial in center of map
    pub fn make_square(&mut self, pixel: Pixel) {
        let third = (self.size / 3) as usize;


        for ((row, col), i) in self.grid.indexed_iter_mut() {
            *i = if row > third && row < third * 2 && col > third && col < third * 2 {
                self.update_texture_px.insert((row, col));
                pixel
            } else {
                self.update_texture_px.insert((row, col));
                Pixel::Air
            };
        }
    }
    

    pub fn get_region(&self, rect: Rect) -> Grid<Pixel> {
        let low_col = (rect.left().floor() as i64).clamp(0, self.size as i64 - 1) as usize;
        let hi_col = (rect.right().ceil() as i64).clamp(0, self.size as i64) as usize;

        let low_row = (rect.top().floor() as i64).clamp(0, self.size as i64 - 1) as usize;
        let hi_row = (rect.bottom().ceil() as i64).clamp(0, self.size as i64) as usize;

        let mut grid = Grid::new(hi_row - low_row, hi_col - low_col);

        for i in low_row..hi_row {
            for j in low_col..hi_col {
                grid[(i - low_row, j - low_col)] = self.grid[(i, j)]; 
            }
        }

        return grid;
    }

    /// updates the image based on the pixels listed in the 'update_texture_px' list
    pub fn update_image(&mut self) {
        for (row, col) in &self.update_texture_px {
            self.image.set_pixel(
                *col as u32,
                *row as u32,
                self.grid[(*row, *col)].color(),
            );

            if row > &2 && self.sky_light[*col] > *row && !(self.grid[(*row, *col)].is_airy() || self.grid[(*row, *col)] == Pixel::Glass) {
                self.sky_light[*col]  = *row - 1;
            }
        }
    }
}
