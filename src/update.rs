use std::mem::swap;
use gaussian_blur::blur;
use grid::*;
use macroquad::{color::{BLACK, WHITE}, texture::Image};

use crate::map::{Map, Pixel};


impl Map {
    pub fn update_state(&mut self) {
        for _ in 0..100000 {
            let point = (fastrand::i32(2..self.size as i32-2),
            fastrand::i32(2..self.size as i32-2));
            // for i in 0..0 {
            //     let point2 = (fastrand::i32(2..self.size as i32-2),
            //     fastrand::i32(2..self.size as i32-2));
            //     if self.heatmap.get_pixel(point2.1 as u32, point2.0 as u32).a == 1.0 {
            //         point = point2;
            //     }
            // }
            self.update_px(
                point.0,
                point.1,
            );
        }
        // self.heatmap = Image::gen_image_color(self.size as u16, self.size as u16, WHITE);
        // for (row, col) in &self.update_texture_px {
        //     for x in 0..3 {
        //         for y in 0..3 {
        //             let x2 = (col + x).clamp(2, self.size as usize-1) as u32;
        //             let y2 = (row + y).clamp(2, self.size as usize-1) as u32;
        //             let px = self.heatmap.get_pixel(x2 -1, y2 -1);
        //             self.heatmap.set_pixel(x2 -1, y2 -1, BLACK);
        //         }
        //     }
        // }

    }

    pub fn swap_px(&mut self, a: (i32, i32), b: (i32, i32)) {
        let temp1 = self.grid[(a.0 as usize, a.1 as usize)];
        self.grid[(a.0 as usize, a.1 as usize)] = self.grid[(b.0 as usize, b.1 as usize)];
        self.grid[(b.0 as usize, b.1 as usize)] = temp1;
    }

    pub fn update_px(&mut self, col: i32, row: i32) {

        
        
        let num = fastrand::i8(0..100);
        let u_row = row as usize;
        let u_col = col as usize;

        if self.grid[(u_row+1,u_col)].less_dense(self.grid[(u_row,u_col)]) && self.grid[(u_row,u_col)].fluid_density().is_some() {
            if !self.grid[(u_row+1,u_col)].is_airy() || num > 75 {
                self.swap_px((row, col), (row + 1, col));
                self.update_texture_px.push((row as usize, col as usize));
                self.update_texture_px.push((row as usize +1, col as usize));
            }
        }

        match self.grid[(u_row,u_col)] {
            Pixel::Sand  => {
                if self.grid[(u_row+1,u_col)] == Pixel::Sand {
                    let side = fastrand::choice([0,2]).unwrap_or(1);
                    if self.grid[(u_row+1,u_col-1+side)].is_airy() {
                        self.swap_px((row, col), (row + 1, col + side as i32 -1));
                        self.update_texture_px.push((row as usize, col as usize));
                        self.update_texture_px.push((row as usize+1, col as usize + side -1));
                    }
                }
            }

            Pixel::Water => {
                if !(self.grid[(u_row+1,u_col)].less_dense(self.grid[(u_row,u_col)])) {
                    let side = fastrand::choice([0,2]).unwrap_or(1);
                    if self.grid[(u_row,u_col-1+side)].is_airy() {
                        self.swap_px((row, col), (row, col + side as i32 -1));
                        self.update_texture_px.push((row as usize, col as usize));
                        self.update_texture_px.push((row as usize, col as usize + side -1));
                    }else if self.grid[(u_row,u_col-1+2-side)].is_airy() {
                        self.swap_px((row, col), (row, col + 1-side as i32));
                        self.update_texture_px.push((row as usize, col as usize));
                        self.update_texture_px.push((row as usize, col as usize + 1-side));
                    
                }
            }
            }

            Pixel::Fire => {

                let mut list1 = [0,1,2];
                let mut list2 = [0,1,2];

                fastrand::shuffle(&mut list1);
                fastrand::shuffle(&mut list2);
                
                for row2 in list1 {
                    for col2 in list2 {
                        if self.grid[(row as usize -1 + row2, col as usize -1 + col2)].is_flammable() && num < 5 {
                            self.grid[(row as usize -1 + row2, col as usize -1 + col2)] = Pixel::Fire;
                            self.update_texture_px.push((row as usize -1, col as usize));
                        }
                    }
                }
                

                if num == 5 {
                    self.grid[(u_row,u_col)] = Pixel::Smoke;
                    self.update_texture_px.push((row as usize, col as usize));
                }

            },
            Pixel::Wood => {
               
            }
            Pixel::Smoke => {
                if num <= 4 && self.grid[(u_row-1,u_col)].is_airy() {
                    self.swap_px((row, col), (row - 1, col));
                    self.update_texture_px.push((row as usize, col as usize));
                    self.update_texture_px.push((row as usize -1, col as usize));
                }
                if num > 80 {
                    self.grid[(u_row,u_col)] = Pixel::Air;
                    self.update_texture_px.push((row as usize, col as usize));
                }
            }
            Pixel::Air => {},
            Pixel::Bedrock => {
                self.update_texture_px.push((row as usize, col as usize));
            }
    }
    }

}
