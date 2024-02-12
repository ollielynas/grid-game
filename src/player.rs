use core::fmt;
use grid::Grid;
use rayon::prelude::*;
use savefile_derive::Savefile;
use std::{default, fmt::Display};
use strum::IntoEnumIterator;

use macroquad::{
    camera::Camera2D,
    color::BLACK,
    input::{is_key_down, is_mouse_button_down, mouse_position},
    math::{Rect, Vec2},
    miniquad::{KeyCode, MouseButton},
    shapes::draw_line,
    time::{get_fps, get_frame_time},
    window::{screen_height, screen_width},
};

use crate::map::Map;
use crate::{craft::craft, entity::Entity, map::Pixel};

#[derive(PartialEq, Debug, Clone)]
// #[derive(PartialEq, Debug, Clone, Savefile)]
pub enum Item {
    Hand,
    Crafter { start: Option<(usize, usize)> },
    Pickaxe,
    SpawnEntity { entity: Entity, count: i32 },
    PlacePixel { pixel: Pixel, count: i32 },
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Item::Hand => "Empty Hand".to_owned(),
            Item::Crafter { start } => "Crafting Wand".to_owned(),
            Item::Pickaxe => "Pickaxe".to_owned(),
            Item::SpawnEntity { entity, count } => format!(
                "{}x{}",
                match entity.entity_type {
                    crate::entity::EntityType::Tree => "Tree".to_owned(),
                    crate::entity::EntityType::Soul => "Soul".to_owned(),
                    crate::entity::EntityType::Fish { air } => "Fish".to_owned(),
                },
                count
            ),
            Item::PlacePixel { pixel, count } => format!(
                "{}x{}",
                match pixel {
                    Pixel::Air => "Air",
                    Pixel::Sand => "Sand",
                    Pixel::Dirt => "Dirt",
                    Pixel::Stone => "Stone",
                    Pixel::Water => "Water",
                    Pixel::Candle => "Candle",
                    Pixel::Fire => "Fire",
                    Pixel::Grass => "Grass",
                    Pixel::Wood => "Wood",
                    Pixel::Bedrock => "Bedrock",
                    Pixel::Smoke => "Smoke",
                    Pixel::Steam => "Steam",
                    Pixel::Gold => "Gold",
                    Pixel::Oil => "Oil",
                    Pixel::Glass => "Glass",
                    Pixel::Lava => "Lava",
                    Pixel::Explosive => "Explosive",
                    Pixel::LiveWood => "Living Wood",
                    Pixel::Seed => "Seed",
                    Pixel::Leaf => "Leaf",
                    Pixel::Loot => "Loot Box",
                },
                count
            ),
        };
        write!(f, "{s}")
    }
}

pub struct Inventory {
    pub items: Vec<Item>,
    pub open: bool,
    pub animation: f32,
}

impl Default for Inventory {
    fn default() -> Self {
        Inventory {
            items: vec![],
            open: false,
            animation: 1.0,
        }
    }
}

impl Inventory {
    pub fn creative() -> Self {
        let mut items: Vec<Item> = Pixel::iter()
            .map(|x| Item::PlacePixel {
                pixel: x,
                count: 9999999,
            })
            .collect();
        items.push(Item::Crafter { start: None });
        Inventory {
            items,
            open: false,
            animation: 1.0,
        }
    }
}

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub zoom: f32,
    pub health: f32,
    pub inventory: Inventory,
    pub item_in_hand: Item,
    pub name: String,
    pub respawn_pos: Vec2,
    jump_height_timer: f32,
    craft_timer: f32,
    pub view_port_cache: Rect,
}

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

impl Default for Player {
    fn default() -> Self {
        Player {
            x: 50.0,
            y: 50.0,
            vx: 0.0,
            vy: 0.0,
            health: 20.0,
            zoom: 30.0,
            inventory: Inventory::default(),
            item_in_hand: Item::Pickaxe,
            name: "Herobrine".to_string(),
            respawn_pos: Vec2 { x: 50.0, y: 50.0 },
            jump_height_timer: 0.0,
            craft_timer: 0.0,
            view_port_cache: Rect::default(),
        }
    }
}
/// TODO get rid of that clone and generally speed up this function

impl Player {
    /// don't forget to set spawn point once twh world has been decided on!
    pub fn new(name: String) -> Player {
        let player = Player {
            name,
            ..Default::default()
        };
        return player;
    }

    pub fn gain_item(&mut self, item: Item) {
        match item {
            Item::Hand => {}
            Item::Crafter { start } => {
                if !self
                    .inventory
                    .items
                    .contains(&Item::Crafter { start: None })
                {
                    self.inventory
                        .items
                        .insert(0, Item::Crafter { start: None })
                }
            }
            Item::Pickaxe => {
                if !self.inventory.items.contains(&Item::Pickaxe) {
                    self.inventory.items.insert(0, Item::Pickaxe)
                }
            }
            Item::SpawnEntity { entity, count } => {
                let mut added_count = false;
                for i in self.inventory.items.iter_mut() {
                    if let Item::SpawnEntity {
                        entity: entity2,
                        count: count2,
                    } = i
                    {
                        if entity == *entity2 {
                            *count2 += count;
                            added_count = true;
                        }
                    }
                }
                if !added_count {
                    self.inventory
                        .items
                        .insert(0, Item::SpawnEntity { entity, count })
                }
            }

            Item::PlacePixel {
                mut pixel,
                mut count,
            } => {
                match pixel {
                    Pixel::LiveWood => pixel = Pixel::Wood,
                    Pixel::Loot => {
                        pixel = fastrand::choice(Pixel::iter().collect::<Vec<Pixel>>())
                            .unwrap_or(Pixel::Gold);
                        count = fastrand::i32(10..200);
                    }
                    _ => {}
                }
                let mut added_count = false;
                for i in self.inventory.items.iter_mut() {
                    if let Item::PlacePixel {
                        pixel: pixel2,
                        count: count2,
                    } = i
                    {
                        if pixel == *pixel2 {
                            *count2 += count;
                            added_count = true;
                        }
                    }
                }
                if !added_count {
                    self.inventory
                        .items
                        .insert(0, Item::PlacePixel { pixel, count })
                }
            }
        }
    }

    pub fn craft_rect(&self, size: usize) -> Option<Rect> {
        match self.item_in_hand {
            Item::Crafter { start: Some(start) } => {
                let mouse = mouse_position();
                let pt = self.cam().screen_to_world(Vec2::new(mouse.0, mouse.1));
                let distance = (self.x.max(pt.x) - self.x.min(pt.x))
                    .hypot(self.y.max(pt.y) - self.y.min(pt.y));

                let row = (pt.y as usize).clamp(2, size - 2);
                let col = (pt.x as usize).clamp(2, size - 2);

                let min_x = start.1.min(col);
                let max_x = start.1.max(col);
                let min_y = start.0.min(row);
                let max_y = start.0.max(row);

                Some(Rect {
                    x: min_x as f32,
                    y: min_y as f32,
                    h: (max_y - min_y + 1) as f32,
                    w: (max_x - min_x + 1) as f32,
                })
            }
            _ => None,
        }
    }

    pub fn use_item(&mut self, map: &mut Map, row: usize, col: usize) {
        if matches!(self.item_in_hand, Item::Crafter { start: Some(_) }) {
            let wand_rect = self
                .craft_rect(map.size.clone() as usize)
                .unwrap_or_default();
            let result = craft(map.get_region(wand_rect));
            if result.0 {
                for ((row, col), i) in result.1.indexed_iter() {
                    let px = (row + wand_rect.y as usize, col + wand_rect.x as usize);
                    map.grid[px] = *i;
                    map.update_texture_px.push(px);
                }
            }
        }

        let pos = (row, col);
        match &mut self.item_in_hand {
            Item::Hand => {}
            Item::Crafter { start: Some(_) } => {
                if self.craft_timer != 0.0 {
                    return;
                }
                self.craft_timer = 0.2;
                self.item_in_hand = Item::Crafter { start: None }
            }
            Item::Crafter { start: None } => {
                if self.craft_timer != 0.0 {
                    return;
                }
                let mouse = mouse_position();
                let pt = self.cam().screen_to_world(Vec2::new(mouse.0, mouse.1));

                let row = (pt.y as usize).clamp(2, map.size as usize - 2);
                let col = (pt.x as usize).clamp(2, map.size as usize - 2);

                self.item_in_hand = Item::Crafter {
                    start: Some((row, col)),
                };
                self.craft_timer = 0.2;
            }
            Item::Pickaxe => {
                if map.grid[pos] != Pixel::Air {
                    self.gain_item(Item::PlacePixel {
                        pixel: map.grid[pos],
                        count: 1,
                    });
                    map.grid[pos] = Pixel::Air;
                }
            }
            Item::SpawnEntity { entity, count } => {
                *count -= 1;
                // real point point in world
                map.spawn_entity(entity.entity_type.clone(), col as f32, row as f32);
                if *count == 0 {
                    self.item_in_hand = Item::Hand;
                }
            }
            Item::PlacePixel { pixel, count } => {
                if map.grid[pos] != *pixel {
                    *count -= 1;
                    map.grid[pos] = *pixel
                }
                if *count == 0 {
                    self.item_in_hand = Item::Hand;
                }
            }
        }
    }

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

    pub fn make_map_box(&self, map: &Map, view: Rect, waffle: bool) -> HitLineSet {
        let mut res = HitLineSet {
            vertical: vec![],
            horizontal: vec![],
        };

        res.horizontal
            .push(HorizontalLine::new(0.0, 0.0, map.size as f32, false));
        res.horizontal.push(HorizontalLine::new(
            0.0,
            map.size as f32,
            map.size as f32,
            true,
        ));

        res.vertical
            .push(VerticalLine::new(0.0, 0.0, map.size as f32, false));
        res.vertical.push(VerticalLine::new(
            map.size as f32,
            0.0,
            map.size as f32,
            true,
        ));

        for row in 0.max((view.y - 2.0) as i32) as usize
            ..map.size.min((view.y + view.h + 2.0) as u32) as usize
        {
            for col in 0.max((view.x - 2.0) as i32) as usize
                ..map.size.min((view.x + view.w + 2.0) as u32) as usize
            {
                if !map.grid[(row, col)].can_hit() {
                    continue;
                }

                if row == 0 || !map.grid[(row - 1, col)].can_hit() {
                    res.horizontal
                        .push(HorizontalLine::new(col as f32, row as f32, 1.0, true));
                } else if waffle && map.grid[(row - 1, col)].can_hit() {
                    let colf = col as f32;
                    let rowf = row as f32;

                    if rowf > self.y - 1.5
                        && rowf < self.y + 5.0
                        && colf > self.x - 2.0
                        && colf < self.x + 3.0
                    {
                        res.horizontal
                            .push(HorizontalLine::new(col as f32, row as f32, 1.0, true));
                    }
                }

                if col == 0 || !map.grid[(row, col - 1)].can_hit() {
                    res.vertical
                        .push(VerticalLine::new(col as f32, row as f32, 1.0, true));
                } else if waffle && map.grid[(row, col - 1)].can_hit() {
                    let colf = col as f32;
                    let rowf = row as f32;

                    if colf < self.x + 0.1
                        && colf > self.x - 2.0
                        && rowf > self.y - 2.0
                        && rowf < self.y + 5.0
                    {
                        res.vertical
                            .push(VerticalLine::new(col as f32, row as f32, 1.0, false));
                    }
                }

                if row == map.size as usize - 1 || !map.grid[(row + 1, col)].can_hit() {
                    res.horizontal.push(HorizontalLine::new(
                        col as f32,
                        row as f32 + 1.0,
                        1.0,
                        false,
                    ));
                }

                if col == map.size as usize - 1 || !map.grid[(row, col + 1)].can_hit() {
                    res.vertical
                        .push(VerticalLine::new(col as f32 + 1.0, row as f32, 1.0, false));
                } else if waffle && map.grid[(row, col + 1)].can_hit() {
                    let colf = col as f32;
                    let rowf = row as f32;

                    if colf > self.x + 1.9
                        && colf < self.x + 4.0
                        && rowf > self.y - 2.0
                        && rowf < self.y + 5.0
                    {
                        res.vertical
                            .push(VerticalLine::new(col as f32, row as f32, 1.0, true));
                    }
                }
            }
        }

        res
    }

    fn respawn(&mut self) {
        self.health = 20.0;
        self.x = self.respawn_pos.x;
        self.y = self.respawn_pos.y;
    }

    pub fn get_player_box(&self, offset_x: f32, offset_y: f32) -> HitLineSet {
        HitLineSet {
            vertical: vec![
                VerticalLine::new(self.x + offset_x, self.y + offset_y, 2.95, true),
                VerticalLine::new(self.x + offset_x + 1.95, self.y + offset_y, 2.95, false),
            ],
            horizontal: vec![
                HorizontalLine::new(self.x + offset_x, self.y + offset_y, 1.95, true),
                HorizontalLine::new(self.x + offset_x, self.y + offset_y + 2.95, 1.95, false),
            ],
        }
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.x, self.y, 1.95, 2.95)
    }

    pub fn update(&mut self, map: &Map) {
        let delta = get_frame_time();
        let mut remaining = delta;

        let mut damage: f32 = 0.0;

        for pixel in map.get_region(self.rect()).iter() {
            damage = damage.max(pixel.player_damage());
        }

        self.health -= damage * delta * 2.0;

        if self.health < 0.0 {
            self.respawn()
        }

        let terrain_hit = self.make_map_box(
            map,
            Rect::new(self.x - 20.0, self.y - 20.0, 40.0, 40.0),
            true,
        );

        let mut on_ground = false;

        while remaining > 0.0 {
            let dp = Vec2::new(self.vx, self.vy) * remaining;

            let collision = self
                .get_player_box(0.0, 0.0)
                .get_collision_with(&terrain_hit, dp);

            match collision {
                None => {
                    self.x += self.vx * remaining;
                    self.y += self.vy * remaining;

                    remaining = 0.0;
                }

                Some(collision) => {
                    self.x += self.vx * collision.time * delta;
                    self.y += self.vy * collision.time * delta;

                    match collision.dir {
                        CollisionDirection::Left | CollisionDirection::Right => {
                            let direction = self.vx.signum() * 0.04;
                            if self
                                .get_player_box(0.0, 0.0)
                                .get_collision_with(&terrain_hit, Vec2::new(0.0, -1.04))
                                .is_none()
                                && self
                                    .get_player_box(0.0, -1.04)
                                    .get_collision_with(&terrain_hit, Vec2::new(direction, 0.0))
                                    .is_none()
                            {
                                self.y -= 1.04;
                                // self.vy += direction * 10.1;
                            } else {
                                self.vx = 0.0;
                            }
                        }

                        CollisionDirection::Down | CollisionDirection::Up => {
                            self.vy = 0.0;
                            if collision.dir == CollisionDirection::Down {
                                on_ground = true;
                            } else {
                                self.jump_height_timer = 0.0;
                            }
                        }
                    }

                    remaining -= collision.time;
                }
            }
        }

        let region = map.get_region(self.rect());
        let mut in_water = false;

        for pixel in region.iter() {
            if pixel.fluid() {
                in_water = true;
            }
        }

        let max_falling_speed = if in_water { 10.0 } else { 40.0 };

        self.vy += if self.vy > max_falling_speed {
            0.0
        } else {
            max_falling_speed * delta * 12.0
        };

        self.jump_height_timer -= delta;
        self.jump_height_timer = self.jump_height_timer.clamp(0.0, 1.0);
        self.craft_timer -= delta;
        self.craft_timer = self.craft_timer.clamp(0.0, 1.0);

        if (on_ground | in_water) && is_key_down(KeyCode::Space) && self.vy > -100.0 {
            self.vy -= if in_water { 10.0 } else { 50.0 };
            self.jump_height_timer = 0.2;
        }

        if is_key_down(KeyCode::Space) && self.vy > -200.0 && self.jump_height_timer > 0.0 {
            self.vy -= 500.0 * delta;
        }

        self.vx *= 0.75_f32;

        if on_ground {
            self.vx *= 0.8_f32;
        }

        if in_water {
            self.vx *= 0.7_f32;
            self.vy *= 0.7f32;
        }

        if is_key_down(KeyCode::A) && self.vx > -500.0 {
            self.vx -= 8.0;
        }
        // if is_key_down(KeyCode::A) && self.vx > -500.0 && on_ground {
        //     self.vx -= 4.0;
        // }

        if is_key_down(KeyCode::D) && self.vx < 500.0 {
            self.vx += 8.0;
        }
        // if is_key_down(KeyCode::D) && self.vx < 500.0 && on_ground {
        //     self.vx +=8.0;
        // }

        self.view_port_cache = self.get_view_port();
    }
}
