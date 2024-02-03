use macroquad::{camera::Camera2D, input::is_key_down, math::Rect, miniquad::KeyCode, time::{get_fps, get_frame_time}, window::{screen_height, screen_width}};

const SPEED: f32 = 100.0;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub zoom: f32,
    pub health: f32,
}


impl Default for Player {
    fn default() -> Self {
        Player {
            x:100.0,
            y: 100.0,
            health: 20.0,
            zoom: 30.0,
        }
    }
}

impl Player {
    pub fn cam(&self) -> Camera2D {
        let scale = 100.0/screen_width();
        Camera2D::from_display_rect(Rect {
            x: self.x,
            y: self.y,
            w: screen_width()*scale,
            h: -screen_height()*scale,
        })
    }

    pub fn update(&mut self) {
        let delta = get_frame_time();
        if is_key_down(KeyCode::W) {
            self.y -= SPEED * delta;
        }
        if is_key_down(KeyCode::S) {
            self.y += SPEED * delta;
        }
        if is_key_down(KeyCode::D) {
            self.x  += SPEED * delta;
        }
        if is_key_down(KeyCode::A) {
            self.x  -= SPEED * delta;
        }
    }
}