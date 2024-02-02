use grid::*;
use macroquad::prelude::Rect;
use macroquad::{
    color::{Color, WHITE},
    texture::Image,
};
use std::default;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Pixel {
    Air,
    Sand,
    Water,
    Fire,
    Wood,
    Bedrock,
    Smoke,
}

impl Default for Pixel {
    fn default() -> Self {
        return Pixel::Air;
    }
}

impl Pixel {
    fn color(&self) -> Color {
        match self {
            Pixel::Air => Color::from_rgba(250, 251, 255, 255),
            Pixel::Sand => Color::from_rgba(207, 215, 157, 255),
            Pixel::Fire => Color::from_rgba(193, 84, 45, 255),
            Pixel::Wood => Color::from_rgba(139, 107, 59, 255),
            Pixel::Smoke => Color::from_rgba(190,190,190, 255),
            Pixel::Water => Color::from_rgba(35,69,190, 255),
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
            Pixel::Bedrock => Pixel::Air
        }

    }

    pub fn is_airy(&self) -> bool {
        return matches!(self, Pixel::Air | Pixel::Fire | Pixel::Smoke);
    }
    pub fn fluid_density(&self) -> Option<i32> {
        match self {
            Pixel::Air => Some(2),
            Pixel::Sand => Some(20),
            Pixel::Smoke => Some(1),
            Pixel::Water => Some(5),
            Pixel::Fire => Some(1),
            Pixel::Bedrock | Pixel::Wood => None,
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
    // pub heatmap: Image,
}

impl Map {
    pub fn new_square(size: usize) -> Map {
        let grid = Grid::from_vec((0..size.pow(2)).map(|_| Pixel::Air).collect(), size);

        return Map {
            grid,
            size: size as u32,
            update_texture_px: vec![],
            image: Image::gen_image_color(size as u16, size as u16, WHITE),
            // heatmap: Image::gen_image_color(size as u16, size as u16, WHITE),
        };
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
