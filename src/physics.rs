use egui_macroquad::macroquad::{color::BLACK, math::{Vec2, Rect}, shapes::draw_line};
use grid::Grid;

use crate::map::{Map, Pixel};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CollisionDirection {
    Right,
    Left,
    Down,
    Up,
}

#[derive(Clone, Copy, Debug)]
pub struct Collision {
    pub time: f32,
    pub dir: CollisionDirection,
}

#[derive(Clone, Copy, Debug)]
pub struct VerticalLine {
    pub x: f32,
    pub y: f32,
    pub height: f32,
    pub left_collide: bool,
}

impl VerticalLine {
    pub fn new<A: Into<f32>, B: Into<f32>, C: Into<f32>>(
        x: A,
        y: B,
        height: C,
        left_collide: bool,
    ) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            height: height.into(),

            left_collide,
        }
    }

    pub fn get_collision_with(&self, other: &VerticalLine, v: Vec2) -> Option<Collision> {
        if v.x.abs() < 0.0000000000001 {
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
            return Some(Collision {
                time: collision_time,
                dir: CollisionDirection::Right,
            });
        } else {
            return Some(Collision {
                time: collision_time,
                dir: CollisionDirection::Left,
            });
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct HorizontalLine {
    pub x: f32,
    pub y: f32,
    pub length: f32,

    pub top_collide: bool,
}

impl HorizontalLine {
    pub fn new<A: Into<f32>, B: Into<f32>, C: Into<f32>>(
        x: A,
        y: B,
        length: C,
        top_collide: bool,
    ) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            length: length.into(),
            top_collide,
        }
    }

    pub fn get_collision_with(&self, other: &HorizontalLine, v: Vec2) -> Option<Collision> {
        if v.y.abs() < 0.0000000001 {
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
            return Some(Collision {
                time: collision_time,
                dir: CollisionDirection::Down,
            });
        } else {
            return Some(Collision {
                time: collision_time,
                dir: CollisionDirection::Up,
            });
        }
    }
}

#[derive(Clone, Debug)]
pub struct HitLineSet {
    pub vertical: Vec<VerticalLine>,
    pub horizontal: Vec<HorizontalLine>,
}

impl HitLineSet {
    pub fn render(&self) {
        for line in &self.horizontal {
            let p1 = Vec2::new(line.x, line.y);
            let p2 = Vec2::new(line.x + line.length, line.y);

            draw_line(p1.x - 0.1, p1.y, p2.x + 0.1, p2.y, 0.2, BLACK);
        }

        // let points = self.vertical.par_iter().map(|line| {
        //     (Vec2::new(line.x, line.y - 0.1),
        //     Vec2::new(line.x, line.y + line.height + 0.1))
        // }).chain(self.horizontal.par_iter().map(|line| {
        //     (Vec2::new(line.x - 0.1, line.y),
        //     Vec2::new(line.x + 0.1 + line.length, line.y))
        // })).collect::<Vec<(Vec2, Vec2)>>();

        // for (p1,p2) in points {
        //     draw_line(p1.x, p1.y, p2.x, p2.y, 0.2, BLACK);
        // }

        for line in &self.vertical {
            let p1 = Vec2::new(line.x, line.y);
            let p2 = Vec2::new(line.x, line.y + line.height);

            draw_line(p1.x, p1.y - 0.1, p2.x, p2.y + 0.1, 0.2, BLACK);
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

pub fn make_map_box(grid: &Grid<Pixel>, view: Rect, waffle: bool, x: f32, y: f32) -> HitLineSet {
    let mut res = HitLineSet {
        vertical: vec![],
        horizontal: vec![],
    };

    let size = grid.cols();

    res.horizontal
        .push(HorizontalLine::new(0.0, 0.0, size as f32, false));
    res.horizontal.push(HorizontalLine::new(
        0.0,
        size as f32,
        size as f32,
        true,
    ));

    res.vertical
        .push(VerticalLine::new(0.0, 0.0, size as f32, false));
    res.vertical.push(VerticalLine::new(
        size as f32,
        0.0,
        size as f32,
        true,
    ));

    for row in 0.max((view.y - 2.0) as i32) as usize
        ..(size as u32).min((view.y + view.h + 2.0) as u32) as usize
    {
        for col in 0.max((view.x - 2.0) as i32) as usize
            ..(size as u32).min((view.x + view.w + 2.0) as u32) as usize
        {
            if !grid[(row, col)].can_hit() {
                continue;
            }

            if row == 0 || !grid[(row - 1, col)].can_hit() {
                res.horizontal
                    .push(HorizontalLine::new(col as f32, row as f32, 1.0, true));
            } else if waffle && grid[(row - 1, col)].can_hit() {
                let colf = col as f32;
                let rowf = row as f32;

                if rowf > y - 1.5
                    && rowf < y + 5.0
                    && colf > x - 2.0
                    && colf < x + 3.0
                {
                    res.horizontal
                        .push(HorizontalLine::new(col as f32, row as f32, 1.0, true));
                }
            }

            if col == 0 || !grid[(row, col - 1)].can_hit() {
                res.vertical
                    .push(VerticalLine::new(col as f32, row as f32, 1.0, true));
            } else if waffle && grid[(row, col - 1)].can_hit() {
                let colf = col as f32;
                let rowf = row as f32;

                if colf < x + 0.1
                    && colf > x - 2.0
                    && rowf > y - 2.0
                    && rowf < y + 5.0
                {
                    res.vertical
                        .push(VerticalLine::new(col as f32, row as f32, 1.0, false));
                }
            }

            if row == size - 1 || !grid[(row + 1, col)].can_hit() {
                res.horizontal.push(HorizontalLine::new(
                    col as f32,
                    row as f32 + 1.0,
                    1.0,
                    false,
                ));
            }

            if col == size - 1 || !grid[(row, col + 1)].can_hit() {
                res.vertical
                    .push(VerticalLine::new(col as f32 + 1.0, row as f32, 1.0, false));
            } else if waffle && grid[(row, col + 1)].can_hit() {
                let colf = col as f32;
                let rowf = row as f32;

                if colf > x + 1.9
                    && colf < x + 4.0
                    && rowf > y - 2.0
                    && rowf < y + 5.0
                {
                    res.vertical
                        .push(VerticalLine::new(col as f32, row as f32, 1.0, true));
                }
            }
        }
    }

    res
}

pub fn make_bounding_box(rect: Rect) -> HitLineSet {
    HitLineSet {
        vertical: vec![
            VerticalLine::new(rect.x, rect.y, rect.h, true),
            VerticalLine::new(rect.x + rect.w, rect.y, rect.h, false),
        ],
        horizontal: vec![
            HorizontalLine::new(rect.x, rect.y, rect.w, true),
            HorizontalLine::new(rect.x, rect.y + rect.h, rect.w, false),
        ],
    }
}