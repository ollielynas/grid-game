use grid::*;
use macroquad::{
    color::{Color, WHITE}, math::Rect, texture::Image
};
use strum_macros::EnumIter;

use perlin2d::PerlinNoise2D;

use crate::entity::{Entity, EntityType};

#[derive(Copy, Clone, PartialEq, Eq, Debug, EnumIter)]
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
}

impl Default for Pixel {
    fn default() -> Self {
        return Pixel::Air;
    }
}

impl Pixel {
    pub fn all() -> impl Iterator<Item = Self> {
        static ALL: [Pixel; 16] = [
            Pixel::Air, Pixel::Sand, Pixel::Dirt, Pixel::Stone,
            Pixel::Water, Pixel::Fire, Pixel::Grass, Pixel::Wood,
            Pixel::Bedrock, Pixel::Smoke, Pixel::Steam, Pixel::Gold,
            Pixel::Oil, Pixel::Glass, Pixel::Lava, Pixel::Explosive
        ];
        
        ALL.into_iter()
    }

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
            Pixel::Oil => Color::from_rgba(0, 0, 0, 255),
            Pixel::Candle => Color::from_rgba(239, 230, 211, 255),
            Pixel::Glass => Color::from_rgba(100, 104, 230, 5),
            Pixel::Bedrock => Color::from_rgba(fastrand::u8(0..255), fastrand::u8(0..255), fastrand::u8(0..255), 255),
            Pixel::Explosive => Color::from_rgba(242, 33, 5, 255),
        }
    }

    pub fn light_emission(&self) -> Color {
        match self {
            Pixel::Air => {Color::new(1.0, 1.0, 1.0, 0.05)},
            Pixel::Glass => {Color::new(1.0, 1.0, 1.0, 0.05)},
            Pixel::Steam | Pixel::Smoke => {Color::new(1.0, 1.0, 1.0, 0.1)},
            Pixel::Water => {Color::new(1.0, 1.0, 1.0, 0.5)},
            Pixel::Lava => {Color::new(1.0, 0.0, 0.0, 0.0)},
            Pixel::Fire => {Color::new(1.0, 0.0, 0.0, 0.0)},
            _ => {Color::new(0.0, 0.0, 0.0, 1.0)}
        }
    }

    pub fn fluid(&self) -> bool {
        matches!(self , Pixel::Lava | Pixel::Water | Pixel::Oil)
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
            Pixel::Steam => Pixel::Glass,
            Pixel::Glass => Pixel::Oil,
            Pixel::Oil => Pixel::Explosive,
            Pixel::Explosive => Pixel::Candle,
            Pixel::Candle => Pixel::Air
        }

    }

    pub fn is_airy(&self) -> bool {
        return matches!(self, Pixel::Air | Pixel::Fire | Pixel::Smoke | Pixel::Steam);
    }

    pub fn fluid_density(&self) -> Option<i32> {
        match self {
            Pixel::Air => Some(3),
            Pixel::Sand|Pixel::Dirt|Pixel::Lava|Pixel::Grass|Pixel::Explosive => Some(30),
            Pixel::Smoke => Some(1),
            Pixel::Steam => Some(1),
            Pixel::Water => Some(15),
            Pixel::Oil => Some(10),
            Pixel::Fire => Some(2),
            Pixel::Bedrock
            |Pixel::Wood 
            |Pixel::Stone
            |Pixel::Candle
            |Pixel::Glass
            |Pixel::Gold => None,
        }
    }

    pub fn heat_product(&self) -> Option<Self> {
        match self {
            Pixel::Wood => Some(Pixel::Fire),
            Pixel::Oil => Some(Pixel::Fire),
            Pixel::Sand => Some(Pixel::Glass),
            Pixel::Water => Some(Pixel::Steam),
            _ => None
        }
    }

    pub fn ignition_probability(&self) -> f32 {
        match self {
            Pixel::Wood => 5.0,
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
            Pixel::Candle | Pixel::Glass |Pixel::Sand | Pixel::Dirt | Pixel::Bedrock | Pixel::Wood | Pixel::Stone | Pixel::Gold | Pixel::Grass | Pixel::Explosive => true,
            Pixel::Oil |Pixel::Air | Pixel::Lava | Pixel::Steam | Pixel::Water | Pixel::Fire | Pixel::Smoke => false
        }
    }

    pub fn less_dense(&self, p: Pixel) -> bool {
        self.fluid_density().unwrap_or(69) < p.fluid_density().unwrap_or(98)
    }


}

pub struct Map {
    pub grid: Grid<Pixel>,
    pub size: u32,
    pub update_texture_px: Vec<(usize, usize)>,
    pub image: Image,
    pub light_mask: Image,
    pub entities: Vec<Entity>,
    pub name: String,
    // pub heatmap: Image,
}

impl Map {

    /// creates a randomly generated map based on perlin noise
    pub fn gen_terrain(&mut self) {

        self.entities = vec![];

        self.make_square(Pixel::Air);
        let new_grid = self.grid.clone();

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
                }else {
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
            self.update_texture_px.push((row, col));
        }
        for ((row, col), _) in new_grid.indexed_iter() {
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
                |Pixel::Candle
                |Pixel::Dirt
                |Pixel::Stone
                |Pixel::Glass
                |Pixel::Oil
                |Pixel::Fire
                |Pixel::Gold
                |Pixel::Steam
                |Pixel::Wood 
                |Pixel::Bedrock
                |Pixel::Lava
                |Pixel::Smoke
                |Pixel::Explosive => {}, 
            }
        }
    } 

    /// makes a new square map of the given `usize`
    pub fn new_square(size: usize, name: String) -> Map {
        let grid = Grid::from_vec(
            vec![Pixel::Air;size.pow(2)], size);

        return Map {
            grid,
            size: size as u32,
            update_texture_px: vec![],
            image: Image::gen_image_color(size as u16, size as u16, WHITE),
            light_mask: Image::gen_image_color(size as u16, size as u16, Color { r: 0.0, g: 0.0, b: 0.0, a: 0.3 }),
            entities: vec![],
            name,
        };
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
                self.update_texture_px.push((row, col));
                pixel
            } else {
                self.update_texture_px.push((row, col));
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
            )
        }




    }
}
