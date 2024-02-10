use crate::{map::{Map, Pixel}, player::Player};
use grid::Grid;
use macroquad::{color::{Color, GREEN}, math::Vec2, time::get_fps};
use rayon::prelude::*;

impl Map {
    pub fn update_state(&mut self, player: &Player) {



        for point in vec![0; (0.5 * (self.size as f32).powi(2)) as usize].par_iter().map(|_| (
            fastrand::i32(2..self.size as i32 - 2),
            fastrand::i32(2..self.size as i32 - 2),
        )).collect::<Vec<(i32,i32)>>() {
            self.update_px(point.0, point.1, player);
        }
        self.move_fluids()
    }
    
/// TODO: https://www.codeproject.com/Articles/16405/Queue-Linear-Flood-Fill-A-Fast-Flood-Fill-Algorith
    pub fn move_fluids(&mut self) {

        let mut fluid_surfaces: Vec<Vec<(usize, usize)>> = vec![];
        let mut detected_fluids = Grid::from_vec(
            vec![false; self.size.pow(2) as usize],
            self.size as usize,
        );
        let mut detected_air = Grid::from_vec(
            vec![0; self.size.pow(2) as usize],
            self.size as usize,
        );

        // let gas = vec![0; self.size.pow(2) as usize].par_iter().map(|x| (x % self.size as usize, x/ self.size as usize)).filter(|a| self.grid[*a] == Pixel::Air);
        // let fluid = vec![0; self.size.pow(2) as usize].par_iter().map(|x| (x % self.size as usize, x/ self.size as usize)).filter(|a| self.grid[*a].fluid());

        let mut air_id;
        let mut check: Vec<(usize, usize)> = vec![];
        for row in 0..(self.size as usize - 2) {
            for col in 0..(self.size as usize - 2) {
                air_id = 0;
                if detected_air[(row, col)] == 0 && self.grid[(row, col)] == Pixel::Air {
                    check = vec![(row, col)];
                    air_id = fastrand::i32(0..i32::MAX)
                }
                while !check.is_empty() {
                    let a = check.pop().unwrap_or((0, 0));
                    

                    if a.1 < 2 || a.1 > self.size as usize - 2 || a.0 > self.size as usize - 2 || a.0 < 2 {
                        continue;
                    }


                    if [
                        (a.0 + 1, a.1),
                        (a.0 - 1, a.1),
                        (a.0, a.1 + 1),
                        (a.0, a.1 - 1),
                        (a.0 +1, a.1 + 1),
                        (a.0 +1 , a.1 - 1),
                        (a.0 -1, a.1 + 1),
                        (a.0 -1 , a.1 - 1),
                    ].iter().all(|f| self.grid[*f] == self.grid[a]) {
                        continue;
                    };



                    detected_air[a] = air_id;
                    
                    let row = a.0;
                    let col = a.1;
                    for i in [
                        (row + 1, col),
                        (row - 1, col),
                        (row, col + 1),
                        (row, col - 1),
                    ] {
                        if detected_air[i] != 0 {
                            continue;
                        }
                        if self.grid[i] == self.grid[(row, col)] {
                            check.push(i)
                        }
                    }
                }
            }}


        for row in 2..(self.size as usize - 2) {
            for col in 2..(self.size as usize - 2) {
                    if [
                        (row + 1, col),
                        (row - 1, col),
                        (row, col + 1),
                        (row, col - 1),
                    ].iter().all(|f| !self.grid[*f].fluid()) {
                        continue;
                    };
                if self.grid[(row, col)].fluid() && !detected_fluids[(row, col)] {
                    fluid_surfaces.insert(0,vec![]);
                    check = vec![(row, col)];
                }
                while !check.is_empty() {
                    let a = check.pop().unwrap_or((0, 0));
                    if a.1 < 2 || a.1 > self.size as usize - 2 || a.0 > self.size as usize - 2 || a.0 < 2 {
                        continue;
                    }
                    if ![
                        (a.0 + 1, a.1),
                        (a.0 - 1, a.1),
                        (a.0, a.1 + 1),
                        (a.0, a.1 - 1),
                        (a.0 +1, a.1 + 1),
                        (a.0 +1 , a.1 - 1),
                        (a.0 -1, a.1 + 1),
                        (a.0 -1 , a.1 - 1),
                    ].iter().any(|f| self.grid[*f] != self.grid[a]) {
                        continue;
                    };


                    detected_fluids[a] = true;
                    if self.grid[(a.0 - 1, a.1)] == Pixel::Air {
                        fluid_surfaces[0].push(a)
                    }
                    let row = a.0;
                    let col = a.1;
                    for i in [
                        (row + 1, col),
                        (row - 1, col),
                        (row, col + 1),
                        (row, col - 1),
                    ] {
                        if detected_fluids[i] {
                            continue;
                        }
                        
                        if self.grid[i] == self.grid[(row, col)] {
                            check.push(i)
                        }
                    }
                }
            }
        }

        fluid_surfaces.retain(|f| f.len() > 2);

        if fluid_surfaces.is_empty() {
            return;
        }

        for surface in fluid_surfaces {
            let swap = fastrand::choose_multiple(surface.iter(), 2);
            if detected_air[((swap[1].0 - 1), swap[1].1)] != detected_air[((swap[0].0 - 1), swap[0].1)]
                || detected_air[((swap[1].0 - 1), swap[1].1)] == 0
                || detected_air[((swap[0].0 - 1), swap[1].1)] == 0
            {
                continue;
            }
            if swap[0].0 > swap[1].0 {
                self.swap_px(
                    (swap[1].0 as i32, swap[1].1 as i32),
                    ((swap[0].0 - 1) as i32, swap[0].1 as i32),
                )
            }
            if swap[0].0 < swap[1].0 {
                self.swap_px(
                    (swap[0].0 as i32, swap[0].1 as i32),
                    ((swap[1].0 - 1) as i32, swap[1].1 as i32),
                )
            }
            
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

    pub fn ignite_px(&mut self, col: i32, row: i32, force: bool) -> bool {
        if col < 0 || row < 0 || col >= self.size as i32 || row >= self.size as i32 {
            return false;
        }

        if force || fastrand::f32() * 100.0 >= self.grid[(row as usize, col as usize)].ignition_probability() {
            return false;
        }

        match self.grid[(row as usize, col as usize)] {
            px if px.heat_product().is_some() => {
                let extinguish = px.extinguish_fire();
                self.grid[(row as usize, col as usize)] = px.heat_product().unwrap();
                self.update_texture_px.push((row as usize, col as usize));

                return extinguish;
            },

            Pixel::Explosive => {
                const RADIUS: i32 = 7;

                self.grid[(row as usize, col as usize)] = Pixel::Air;

                for dr in -RADIUS..=RADIUS {
                    let target_row = row + dr;

                    if target_row < 0 || target_row >= self.size as i32 {
                        continue;
                    }

                    let height = ((RADIUS * RADIUS - dr * dr) as f64).sqrt().round() as i32;
                    
                    for dc in -height..=height {
                        let target_col = col + dc;

                        if target_col < 0 || target_col >= self.size as i32 {
                            continue;
                        }

                        let target_px = &mut self.grid[(target_row as usize, target_col as usize)];
                        self.update_texture_px.push((target_row as usize, target_col as usize));
                        
                        if fastrand::f32() < 0.8 {
                            if target_px.ignition_probability() > 0.0 {
                                    self.ignite_px(target_col, target_row, true);
                            } else if *target_px != Pixel::Bedrock {
                                        *target_px = Pixel::Fire;
                            }
                        }
                    }
                }
            },

            _ => {}
        }

        return false;
    }

    pub fn ignite_neighbors(&mut self, col: i32, row: i32, count: usize) -> i32 {
        let mut neighbors = vec![(-1, 0), (0, -1), (1, 0), (0, 1)];

        fastrand::shuffle(&mut neighbors);

        let mut ignited = 0;

        for i in 0..count {
            if self.ignite_px(col + neighbors[i].0, row + neighbors[i].1, false) {
                ignited += 1;
            }
        }

        ignited
    }

    pub fn update_px(&mut self, col: i32, row: i32, player: &Player) {
        let num = fastrand::f32() * 100.0;
        let u_row = row as usize;
        let u_col = col as usize;

        let is_less_dense = self.grid[(u_row + 1, u_col)].less_dense(self.grid[(u_row, u_col)]);

        if is_less_dense && self.grid[(u_row, u_col)].fluid_density().is_some() {
            if num > 85.0 || (!self.grid[(u_row, u_col)].is_airy()) {
                self.swap_px((row, col), (row + 1, col));
            }
        }


        // updates based on what pixel is being updated
        match self.grid[(u_row, u_col)] {
            Pixel::Sand => {
                if self.grid[(u_row + 1, u_col)] == Pixel::Sand {
                    let side = fastrand::choice([0, 2]).unwrap_or(1);
                    if self.grid[(u_row + 1, u_col - 1 + side)].less_dense(Pixel::Sand) {
                        self.swap_px((row, col), (row + 1, col + side as i32 - 1));
                    }
                }
            }

            Pixel::Candle => {
                if self.grid[(u_row - 1, u_col)]==Pixel::Air && num > 80.0 {
                    self.grid[(u_row - 1, u_col)] = Pixel::Fire;
                }
                self.ignite_px(u_col as i32, u_row as i32 - 1, false);
            }
            
            Pixel::Explosive => {
                if self.grid[(u_row + 1, u_col)] == Pixel::Explosive {
                    let side = fastrand::choice([0, 2]).unwrap_or(1);
                    if self.grid[(u_row + 1, u_col - 1 + side)].less_dense(Pixel::Explosive) {
                        self.swap_px((row, col), (row + 1, col + side as i32 - 1));
                    }
                }
            }

            Pixel::Dirt => {
                if self.grid[(u_row + 1, u_col)] == Pixel::Dirt
                    && self.grid[(u_row - 1, u_col)] == Pixel::Dirt
                {
                    let side = fastrand::choice([0, 2]).unwrap_or(1);
                    if self.grid[(u_row + 1, u_col - 1 + side)].less_dense(Pixel::Dirt) {
                        self.swap_px((row, col), (row + 1, col + side as i32 - 1));
                    }
                }
                if self.grid[(u_row - 1, u_col)] == Pixel::Air {
                    self.grid[(u_row, u_col)] = Pixel::Grass;
                    self.update_texture_px.push((row as usize, col as usize));
                }
            }

            Pixel::Water => {
                if !(is_less_dense) {
                    let side = fastrand::choice([0, 2]).unwrap_or(1);
                    if self.grid[(u_row, u_col - 1 + side)].is_airy() {
                        self.swap_px((row, col), (row, col + side as i32 - 1));
                    } else if self.grid[(u_row, u_col - 1 + 2 - side)].is_airy() {
                        self.swap_px((row, col), (row, col + 1 - side as i32));
                    }
                }
            }
            Pixel::Lava => {
                if !is_less_dense && num < 10.0 {
                    let side = fastrand::choice([0, 2]).unwrap_or(1);
                    if self.grid[(u_row, u_col - 1 + side)].less_dense(Pixel::Lava) {
                        self.swap_px((row, col), (row, col + side as i32 - 1));
                    } else if self.grid[(u_row, u_col - 1 + 2 - side)].less_dense(Pixel::Lava) {
                        self.swap_px((row, col), (row, col + 1 - side as i32));
                    }
                }

                let num_extiniguish = self.ignite_neighbors(col, row, 4);

                if num < 1.5 * num_extiniguish as f32 {
                    self.grid[(u_row, u_col)] = Pixel::Stone;
                    self.update_texture_px.push((row as usize, col as usize));
                }
            }

            Pixel::Fire => {
                if self.ignite_neighbors(col, row, 4) > 0 || num < 1.0 {
                    self.grid[(u_row, u_col)] = Pixel::Smoke;
                    self.update_texture_px.push((row as usize, col as usize));
                }
            }
            Pixel::Oil => {
                if !(is_less_dense) {
                    let side = fastrand::choice([0, 2]).unwrap_or(1);
                    if self.grid[(u_row, u_col - 1 + side)].is_airy() {
                        self.swap_px((row, col), (row, col + side as i32 - 1));
                    } else if self.grid[(u_row, u_col - 1 + 2 - side)].is_airy() {
                        self.swap_px((row, col), (row, col + 1 - side as i32));
                    }
                }
            }
            Pixel::Smoke => {
                if num > 98.0 {
                    self.grid[(u_row, u_col)] = Pixel::Air;
                    self.update_texture_px.push((row as usize, col as usize));
                }
            }
            Pixel::Steam => {
                if num < 0.1 {
                    self.grid[(u_row, u_col)] = Pixel::Water;
                    self.update_texture_px.push((row as usize, col as usize));
                }
                if self.grid[(u_row - 1, u_col)].fluid_density().unwrap_or(99) != 3 {
                    let side = fastrand::choice([0, 2]).unwrap_or(1);
                    if self.grid[(u_row, u_col - 1 + side)].is_airy() {
                        self.swap_px((row, col), (row, col + side as i32 - 1));
                    } else if self.grid[(u_row, u_col - 1 + 2 - side)].is_airy() {
                        self.swap_px((row, col), (row, col + 1 - side as i32));
                    }
                }
            }
            Pixel::Glass => {
                if self.grid[(u_row - 1, u_col)] == Pixel::Fire 
                || self.grid[(u_row + 1, u_col)] == Pixel::Fire 
                || self.grid[(u_row, u_col + 1)] == Pixel::Fire 
                || self.grid[(u_row, u_col - 1)] == Pixel::Fire {
                    self.grid[(u_row,u_col)] = Pixel::Glass
                }
            }
            Pixel::Grass => {
                if !self.grid[(u_row - 1, u_col)].is_airy()
                || self.grid[(u_row + 1, u_col)].is_airy()
                {
                    self.grid[(u_row, u_col)] = Pixel::Dirt;
                    self.update_texture_px.push((row as usize, col as usize));
                }
            }
            Pixel::Bedrock => {
                self.update_texture_px.push((row as usize, col as usize));
            }
            Pixel::Gold | Pixel::Stone | Pixel::Air | Pixel::Wood => {}
        }



        if num > 80.0 || player.get_view_port().contains(Vec2::new(col as f32, row as f32))  {
        let light_mask_surroundings = [
            self.light_mask.get_pixel(col as u32 - 1, row as u32 - 1),
            self.light_mask.get_pixel(col as u32 - 1, row as u32),
            self.light_mask.get_pixel(col as u32 - 1, row as u32 + 1),
            self.light_mask.get_pixel(col as u32, row as u32 - 1),
            self.light_mask.get_pixel(col as u32, row as u32 + 1),
            self.light_mask.get_pixel(col as u32 + 1, row as u32 - 1),
            self.light_mask.get_pixel(col as u32 + 1, row as u32),
            self.light_mask.get_pixel(col as u32 + 1, row as u32 + 1),
            self.grid[(u_row, u_col)].light_emission(),
        ];



        let mut color = self.grid[(u_row, u_col)].light_emission();

        for c in light_mask_surroundings {
            if c.a <= color.a {
                color = c;
            }
        }

        
        self.light_mask.set_pixel(
                col as u32,
                row as u32,
                Color {
                    a: (color.a
                        + 0.15)
                        .clamp(0.0, 1.0),
                        r: 0.0,
                        g: 0.0,
                        b: 0.0,
                },
            );
    
    }
    }
}
