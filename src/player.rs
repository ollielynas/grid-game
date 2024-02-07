use macroquad::{
    camera::Camera2D, color::{BLACK, RED}, input::{get_char_pressed, is_key_down}, math::{Rect, Vec2}, miniquad::KeyCode, shapes::draw_line, time::{get_fps, get_frame_time}, window::{screen_height, screen_width}
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CollisionDirection {
    IntoRight, IntoLeft, IntoDown, IntoUp
}

#[derive(Clone, Copy, Debug)]
pub struct Collision {
    pub time: f32,
    pub dir: CollisionDirection
}

#[derive(Clone, Copy, Debug)]
pub struct VerticalLine {
    pub x: f32,
    pub y: f32,
    pub height: f32,

    pub left_collide: bool
}

impl VerticalLine {
    pub fn new<A: Into<f32>, B: Into<f32>, C: Into<f32>>(x: A, y: B, height: C, left_collide: bool) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            height: height.into(),

            left_collide
        }
    }

    pub fn get_collision_with(&self, other: &VerticalLine, v: Vec2) -> Option<Collision> {
        if v.x.abs() < 0.0001 {
            return None;
        }

        if v.x > 0.0 && (!other.left_collide || self.left_collide) {
            return None;
        }

        if v.x < 0.0 && (other.left_collide || !self.left_collide) {
            return None;
        }

        let dx = other.x - self.x;
        let collision_time = dx / v.x;

        if collision_time < 0.0 || collision_time > 1.0 {
            return None;
        }

        let y_shift = v.y * collision_time;

        let top_y = other.y - y_shift;
        let bottom_y = top_y + other.height;

        if bottom_y <= self.y || top_y >= self.y + self.height {
            return None;
        }

        if v.x > 0.0 {
            return Some(Collision { time: collision_time, dir: CollisionDirection::IntoRight });
        } else {
            return Some(Collision { time: collision_time, dir: CollisionDirection::IntoLeft });
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct HorizontalLine {
    pub x: f32,
    pub y: f32,
    pub length: f32,

    pub top_collide: bool
}

impl HorizontalLine {
    pub fn new<A: Into<f32>, B: Into<f32>, C: Into<f32>>(x: A, y: B, length: C, top_collide: bool) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            length: length.into(),
            top_collide
        }
    }

    pub fn get_collision_with(&self, other: &HorizontalLine, v: Vec2) -> Option<Collision> {
        if v.y.abs() < 0.0001 {
            return None;
        }

        if v.y > 0.0 && (!other.top_collide || self.top_collide) {
            return None;
        }

        if v.y < 0.0 && (other.top_collide || !self.top_collide) {
            return None;
        }

        let dy = other.y - self.y;
        let collision_time = dy / v.y;

        if collision_time < 0.0 || collision_time > 1.0 {
            return None;
        }

        let x_shift = v.x * collision_time;

        let left_x = other.x - x_shift;
        let right_x = left_x + other.length;

        if right_x <= self.x || left_x >= self.x + self.length {
            return None;
        }

        if v.y > 0.0 {
            return Some(Collision { time: collision_time, dir: CollisionDirection::IntoDown });
        } else {
            return Some(Collision { time: collision_time, dir: CollisionDirection::IntoUp });
        }
    }
}

#[derive(Clone, Debug)]
pub struct HitLineSet {
    pub vertical: Vec<VerticalLine>,
    pub horizontal: Vec<HorizontalLine>
}

impl HitLineSet {
    pub fn render(&self) {
        for line in &self.horizontal {
            let p1 = Vec2::new(line.x, line.y);
            let p2 = Vec2::new(line.x + line.length, line.y);

            //println!("{:?} {:?}", p1, p2);
    
            draw_line(p1.x, p1.y, p2.x, p2.y, 0.2, BLACK);
        }

        for line in &self.vertical {
            let p1 = Vec2::new(line.x, line.y);
            let p2 = Vec2::new(line.x, line.y + line.height);

            //println!("{:?} {:?}", p1, p2);
    
            draw_line(p1.x, p1.y, p2.x, p2.y, 0.2, BLACK);
        }
    }

    pub fn get_collision_with(&self, other: &HitLineSet, v: Vec2) -> Option<Collision> {
        let mut res: Option<Collision> = None;

        for v1 in &self.vertical {
            for v2 in &other.vertical {
                if let Some(collision) = v1.get_collision_with(v2, v) {
                    if res.is_none() || res.as_ref().unwrap().time > collision.time {
                        res = Some(collision);
                    } 
                }
            }
        }

        for h1 in &self.horizontal {
            for h2 in &other.horizontal {
                if let Some(collision) = h1.get_collision_with(h2, v) {
                    if res.is_none() || res.as_ref().unwrap().time > collision.time {
                        res = Some(collision);
                    } 
                }
            }
        }

        res
    }
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

    pub fn make_map_box(&self, map: &Map) -> HitLineSet {
        let mut res = HitLineSet {
            vertical: vec![],
            horizontal: vec![]
        };

        res.horizontal.push(HorizontalLine::new(0.0, 0.0, map.size as f32, false));
        res.horizontal.push(HorizontalLine::new(0.0, map.size as f32, map.size as f32, true));

        res.vertical.push(VerticalLine::new(0.0, 0.0, map.size as f32, false));
        res.vertical.push(VerticalLine::new(map.size as f32, 0.0, map.size as f32, true));

        for row in 0.max(self.y as i32 - 20) as usize..map.size.min(self.y as u32 + 20) as usize {
            for col in 0.max(self.x as i32 - 20) as usize..map.size.min(self.x as u32 + 20) as usize  {
                
                if !map.grid[(row, col)].can_hit() {
                    continue;
                }

                if row == 0 || !map.grid[(row - 1, col)].can_hit() {
                    res.horizontal.push(HorizontalLine::new(col as f32, row as f32, 1.0, true));
                } else if map.grid[(row - 1, col)].can_hit() {
                    let colf = col as f32;
                    let rowf = row as f32;

                    if rowf > self.y - 1.5 && rowf < self.y + 5.0 && colf > self.x - 2.0 && colf < self.x + 3.0 {
                        res.horizontal.push(HorizontalLine::new(col as f32, row as f32, 1.0, true));
                    }
                }

                if col == 0 || !map.grid[(row, col - 1)].can_hit() {
                    res.vertical.push(VerticalLine::new(col as f32, row as f32, 1.0, true));
                } else if map.grid[(row, col - 1)].can_hit() {
                    let colf = col as f32;
                    let rowf = row as f32;

                    if colf < self.x + 0.1 && colf > self.x - 2.0 && rowf > self.y - 2.0 && rowf < self.y + 5.0 {
                        res.vertical.push(VerticalLine::new(col as f32, row as f32, 1.0, false));
                    }
                }

                if row == map.size as usize - 1 || !map.grid[(row + 1, col)].can_hit() {
                    res.horizontal.push(HorizontalLine::new(col as f32, row as f32 + 1.0, 1.0, false));
                }

                if col == map.size as usize - 1 || !map.grid[(row, col + 1)].can_hit() {
                    res.vertical.push(VerticalLine::new(col as f32 + 1.0, row as f32, 1.0, false));
                } else if map.grid[(row, col + 1)].can_hit() {
                    let colf = col as f32;
                    let rowf = row as f32;

                    if colf > self.x + 1.9 && colf < self.x + 4.0 && rowf > self.y - 2.0 && rowf < self.y + 5.0 {
                        res.vertical.push(VerticalLine::new(col as f32, row as f32, 1.0, true));
                    }
                }
            }
        }

        res
    }

    pub fn get_player_box(&self) -> HitLineSet {
        let mut res = HitLineSet {
            vertical: vec![],
            horizontal: vec![]
        };

        res.horizontal.push(HorizontalLine::new(self.x, self.y, 1.95, true));
        res.horizontal.push(HorizontalLine::new(self.x, self.y + 2.95, 1.95, false));

        res.vertical.push(VerticalLine::new(self.x, self.y, 2.95, true));
        res.vertical.push(VerticalLine::new(self.x + 1.95, self.y, 2.95, false));

        res
    }

    pub fn update(&mut self, map: &Map) {
        let delta = get_frame_time();
        let mut remaining = delta;

        let terain_hit = self.make_map_box(map);

        let mut on_ground = false;

        while remaining > 0.0 {
            let dp = Vec2::new(self.vx, self.vy) * remaining;

            let collision = self.get_player_box().get_collision_with(&terain_hit, dp);

            match collision {
                None => {
                    self.x += self.vx * remaining;
                    self.y += self.vy * remaining;

                    remaining = 0.0;
                },

                Some(collision) => {
                    //println!("Collision! {:?}", collision);

                    self.x += self.vx * collision.time * delta;
                    self.y += self.vy * collision.time * delta;

                    match collision.dir {
                        CollisionDirection::IntoLeft | CollisionDirection::IntoRight => self.vx = 0.0,

                        CollisionDirection::IntoDown | CollisionDirection::IntoUp => {
                            self.vy = 0.0;
                            if collision.dir == CollisionDirection::IntoDown {
                                on_ground = true;
                            }
                        }
                    }

                    remaining -= collision.time;
                }
            }
        }

        self.vy += if self.vy > 50.0 {
            0.0
        } else {
            10.0
        };

        let in_water = false;

        if on_ground && is_key_down(KeyCode::Space) && self.vy > -100.0 {
            if in_water {
                self.vy -= 10.0
            } else {
                self.vy -= 200.0;
            }
        }

        self.vx *= 0.75;

        if on_ground {
            self.vx *= 0.7;
        }

        if is_key_down(KeyCode::A) && self.vx > -100.0 {
            self.vx -= 10.0;
        }

        if is_key_down(KeyCode::D) && self.vx < 100.0 {
            self.vx += 10.0;
        }

        /*if on_ground && self.vy >= 0.0 {
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
        */

        // println!("{points:?}");
    }
}
