use macroquad::{
    camera::Camera2D,
    input::{get_char_pressed, is_key_down},
    math::Rect,
    miniquad::KeyCode,
    time::{get_fps, get_frame_time},
    window::{screen_height, screen_width},
};

use crate::grid;
use crate::map::Map;

const SPEED: f32 = 100.0;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub zoom: f32,
    pub health: f32,
}

impl Default for Player {
    fn default() -> Self {
        Player {
            x: 10.0,
            y: 10.0,
            vx: 0.0,
            vy: 0.0,
            health: 20.0,
            zoom: 30.0,
        }
    }
}

impl Player {
    pub fn cam(&self) -> Camera2D {
        let scale = 100.0 / screen_width();
        Camera2D::from_display_rect(Rect {
            x: self.x - screen_width() * scale / 2.0,
            y: self.y + screen_height() * scale / 2.0,
            w: screen_width() * scale,
            h: -screen_height() * scale,
        })
    }

    pub fn get_view_port(&self) -> Rect {
        let scale = 100.0 / screen_width();
        Rect {
            x: self.x - (screen_width() * scale) / 2.0,
            y: self.y - (screen_height() * scale) / 2.0,
            w: screen_width() * scale,
            h: screen_height() * scale,
        }
    }

    pub fn update(&mut self, map: &Map) {
        let delta = get_frame_time();

        let points = grid![
            [map.grid[(self.y as usize, self.x as usize)],map.grid[(self.y as usize, self.x as usize + 1)],map.grid[(self.y as usize, (self.x - 0.01) as usize + 2)]]
            [map.grid[(self.y as usize +1, self.x as usize)],map.grid[(self.y as usize + 1, self.x as usize + 1)],map.grid[(self.y as usize + 1, (self.x - 0.01) as usize + 2)]]
            [map.grid[(self.y as usize + 2, self.x as usize)],map.grid[(self.y as usize + 2, self.x as usize + 1)],map.grid[(self.y as usize + 2, (self.x -0.01)as usize + 2)]]
            ];

        let floor_detector = [
            map.grid[((self.y + 3.0 + self.vy * delta) as usize, self.x as usize)],
            map.grid[(
                (self.y + 3.0 + self.vy * delta) as usize,
                self.x as usize + 1,
            )],
            map.grid[(
                (self.y + 3.0 + self.vy * delta) as usize,
                (self.x -0.01)as usize + 2,
            )],
        ];
        let left_wall_detector = [
            map.grid[((self.y) as usize, (self.x + -1.0 +  self.vx * delta) as usize)],
            map.grid[(
                (self.y +1.0) as usize,
                (self.x + -1.0 +  self.vx * delta) as usize,
            )],
        ];
        let right_wall_detector = [
            map.grid[((self.y) as usize, (self.x + 3.0 +  self.vx * delta) as usize)],
            map.grid[(
                (self.y +1.0) as usize,
                (self.x + -1.0 +  self.vx * delta) as usize,
            )],
        ];

        let on_ground1 =
            !points[(0, 1)].is_airy() || !points[(0, 0)].is_airy() || !points[(0, 2)].is_airy();
        let on_ground2 =
            !points[(1, 1)].is_airy() || !points[(1, 0)].is_airy() || !points[(1, 2)].is_airy();
        let on_ground3 =
            !points[(2, 1)].is_airy() || !points[(2, 0)].is_airy() || !points[(2, 2)].is_airy();

        let in_water = floor_detector[0].fluid_density().unwrap_or(99) < 20
            && floor_detector[1].fluid_density().unwrap_or(99) < 20
            && floor_detector[2].fluid_density().unwrap_or(99) < 20;

        if on_ground3 && !in_water {
            self.y -= 0.1;
        }

        let mut left_wall = false;
        let mut right_wall = false;

        left_wall = !left_wall_detector[0].is_airy();
        right_wall = !points[(1,2)].is_airy();

        if left_wall {
            self.x = self.x.ceil()
        }

        if left_wall {
            self.x = self.x.floor()
        }

        let on_floor = !floor_detector[0].is_airy()
            || !floor_detector[1].is_airy()
            || !floor_detector[2].is_airy();

        self.vy += if self.vy > 50.0 || on_floor {
            0.0
        } else {
            10.0
        };

        if on_floor && self.vy > 0.0 {
            self.y = self.y.ceil();
        }

        if on_floor && self.vy >= 0.0 {
            if in_water {
                self.vy *= 0.9
            } else {
                self.vy = 0.0;
                self.y = (self.y).round() + 0.01;
            }
        }

        if on_floor && is_key_down(KeyCode::Space) && self.vy > -100.0 {
            if in_water {
                self.vy -= 10.0
            } else {
                self.vy -= 100.0;
            }
        }

        if in_water && self.vy < 100.0 && is_key_down(KeyCode::S) {
            self.vy += 10.0
        }

        self.vx *= 0.75;

        if on_floor {
            self.vx *= 0.7;
        }

        if !left_wall && is_key_down(KeyCode::A) && self.vx > -100.0 {
            self.vx -= 10.0;
        }
        if self.vx != 0.0 {
            if on_ground3 && !in_water && !on_ground2 {
                self.y -= 1.0;
            }
        }

        if !right_wall && is_key_down(KeyCode::D) && self.vx < 100.0 {
            self.vx += 10.0;
        }

        self.x += self.vx * delta;
        self.y += self.vy * delta;

        self.x = self.x.clamp(3.0, map.size as f32 - 3.0);
        self.y = self.y.clamp(3.0, map.size as f32 - 6.0);

        // println!("{points:?}");
    }
}
