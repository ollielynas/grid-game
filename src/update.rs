
    
use crate::map::{Map, Pixel};
use grid::grid;
use macroquad::color::Color;

impl Map {


    pub fn update_state(&mut self) {
        for _ in 0..(0.5 * (self.size as f32).powi(2)) as usize {
            let point = (fastrand::i32(2..self.size as i32-2),
            fastrand::i32(2..self.size as i32-2));
            self.update_px(
                point.0,
                point.1,
            );
        }
    }
    
    /// swaps 2 pixels and also updates texture
    pub fn swap_px(&mut self, a: (i32, i32), b: (i32, i32)) {
        let temp1 = self.grid[(a.0 as usize, a.1 as usize)];
        self.grid[(a.0 as usize, a.1 as usize)] = self.grid[(b.0 as usize, b.1 as usize)];
        self.grid[(b.0 as usize, b.1 as usize)] = temp1;
        self.update_texture_px.push((b.0 as usize, b.1 as usize));
        self.update_texture_px.push((a.0 as usize, a.1 as usize));
    }

    pub fn update_px(&mut self, col: i32, row: i32) {

        let num = fastrand::f32()*100.0;
        let u_row = row as usize;
        let u_col = col as usize;


        let light_mask_surroundings = [
            self.light_mask.get_pixel(col as u32 - 1, row as u32 - 1).a,
            self.light_mask.get_pixel(col as u32 - 1, row as u32).a,
            self.light_mask.get_pixel(col as u32 - 1, row as u32 + 1).a ,
            self.light_mask.get_pixel(col as u32, row as u32 - 1).a,
            self.light_mask.get_pixel(col as u32, row as u32 + 1).a,
            self.light_mask.get_pixel(col as u32 + 1, row as u32 - 1).a,
            self.light_mask.get_pixel(col as u32 + 1, row as u32).a,
            self.light_mask.get_pixel(col as u32 + 1, row as u32 + 1).a,
            self.grid[(u_row,u_col)].light_emission(),
        ];

        self.light_mask.set_pixel(col as u32, row as u32, Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: (light_mask_surroundings.iter().fold(std::f32::MAX, |a,b| a.min(*b)) + 0.15).clamp(0.0,1.0),
        });



        let is_less_dense = self.grid[(u_row+1,u_col)].less_dense(self.grid[(u_row,u_col)]);

        if is_less_dense && self.grid[(u_row,u_col)].fluid_density().is_some() {
            if (!self.grid[(u_row,u_col)].is_airy()) || num > 85.0 {
                self.swap_px((row, col), (row + 1, col));
            }
        }


        // updates based on what pixel is being updated
        match self.grid[(u_row,u_col)] {

            Pixel::Sand  => {
                if self.grid[(u_row+1,u_col)] == Pixel::Sand {
                    let side = fastrand::choice([0,2]).unwrap_or(1);
                    if self.grid[(u_row+1,u_col-1+side)].less_dense(Pixel::Sand) {
                        self.swap_px((row, col), (row + 1, col + side as i32 -1));
                    }
                }
            }

            Pixel::Dirt  => {
                if self.grid[(u_row+1,u_col)] == Pixel::Dirt && self.grid[(u_row-1,u_col)] == Pixel::Dirt {
                    let side = fastrand::choice([0,2]).unwrap_or(1);
                    if self.grid[(u_row+1,u_col-1+side)].less_dense(Pixel::Dirt) {
                        self.swap_px((row, col), (row + 1, col + side as i32 -1));
                    }
                }
                if self.grid[(u_row-1,u_col)] == Pixel::Air {
                    self.grid[(u_row,u_col)] = Pixel::Grass;
                    self.update_texture_px.push((row as usize, col as usize));
                }
            }

            Pixel::Water => {
                if !(is_less_dense) {
                    let side = fastrand::choice([0,2]).unwrap_or(1);
                    if self.grid[(u_row,u_col-1+side)].is_airy() {
                        self.swap_px((row, col), (row, col + side as i32 -1));
                    }else if self.grid[(u_row,u_col-1+2-side)].is_airy() {
                        self.swap_px((row, col), (row, col + 1-side as i32));
                    
                }
            }
            }
            Pixel::Lava => {
                if !is_less_dense && num < 10.0 {
                    let side = fastrand::choice([0,2]).unwrap_or(1);
                    if self.grid[(u_row,u_col-1+side)].less_dense(Pixel::Lava) {
                        self.swap_px((row, col), (row, col + side as i32 -1));
                    }else if self.grid[(u_row,u_col-1+2-side)].less_dense(Pixel::Lava) {
                        self.swap_px((row, col), (row, col + 1-side as i32));
                    
                }
            }
            if self.grid[(u_row - 1,u_col)] == Pixel::Water {
                if num < 1.5 {
                    self.grid[(u_row,u_col)] = Pixel::Stone;
                    self.update_texture_px.push((row as usize, col as usize));
                }
                self.update_texture_px.push((row as usize -1, col as usize));
                self.grid[(u_row - 1,u_col)] = Pixel::Steam;
            
            }
        }

            Pixel::Fire => {

                let mut list1 = [0,1,2];
                let mut list2 = [0,1,2];

                fastrand::shuffle(&mut list1);
                fastrand::shuffle(&mut list2);
                
                for row2 in list1 {
                    for col2 in list2 {
                        if self.grid[(row as usize -1 + row2, col as usize -1 + col2)].is_flammable() && num < 5.0 {
                            self.grid[(row as usize -1 + row2, col as usize -1 + col2)] = Pixel::Fire;
                            self.update_texture_px.push((row as usize -1, col as usize));
                        }
                    }
                }
                

                if num < 1.0 {
                    self.grid[(u_row,u_col)] = Pixel::Smoke;
                    self.update_texture_px.push((row as usize, col as usize));
                }

            },
            Pixel::Wood => {
               
            }
            Pixel::Smoke => {
                if num > 98.0 {
                    self.grid[(u_row,u_col)] = Pixel::Air;
                    self.update_texture_px.push((row as usize, col as usize));
                }
            }
            Pixel::Steam => {
                if num < 0.1 {
                    self.grid[(u_row,u_col)] = Pixel::Water;
                    self.update_texture_px.push((row as usize, col as usize));
                }
                if self.grid[(u_row-1,u_col)].fluid_density().unwrap_or(99) != 3 {
                    let side = fastrand::choice([0,2]).unwrap_or(1);
                    if self.grid[(u_row,u_col-1+side)].is_airy() {
                        self.swap_px((row, col), (row, col + side as i32 -1));
                    }else if self.grid[(u_row,u_col-1+2-side)].is_airy() {
                        self.swap_px((row, col), (row, col + 1-side as i32));
                    
                }
            }
            }
            Pixel::Air => {},
            Pixel::Stone => {},
            Pixel::Gold => {},
            Pixel::Grass => {
                if !self.grid[(u_row-1,u_col)].is_airy() || self.grid[(u_row+1,u_col)].is_airy() {
                    self.grid[(u_row,u_col)] = Pixel::Dirt;
                    self.update_texture_px.push((row as usize, col as usize));
                }
            },
            Pixel::Bedrock => {
                self.update_texture_px.push((row as usize, col as usize));
            }
    }
    }

}
