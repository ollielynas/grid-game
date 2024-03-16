use core::fmt;
use savefile::{load_file, save_file};
use savefile_derive::Savefile;
use std::{fmt::Display, fs::create_dir_all};
use strum::IntoEnumIterator;

use egui_macroquad::macroquad::{
    camera::Camera2D,
    input::{is_key_down, mouse_position},
    math::{Rect, Vec2},
    miniquad::KeyCode,
    time::get_frame_time,
    ui::root_ui,
    window::{screen_height, screen_width},
};

use crate::{map::Map, physics::{self, CollisionDirection, HitLineSet}, settings::Settings, SAVEFILE_VERSION};
use crate::{craft::craft, map::Pixel};

#[derive(PartialEq, Debug, Clone, Savefile)]
pub enum Item {
    Hand,
    Crafter { start: Option<(usize, usize)> },
    Pickaxe,
    PlacePixel { pixel: Pixel, count: i32 },
}

impl Default for Item {
    fn default() -> Self {
        Item::Hand
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Item::Hand => "Empty Hand".to_owned(),
            Item::Crafter { start:_ } => "Crafting Wand".to_owned(),
            Item::Pickaxe => "Pickaxe".to_owned(),
            // Item::SpawnEntity { entity, count } => format!(
            //     "{}x{}",
            //     match entity.entity_type {
            //         crate::entity::EntityType::Tree => "Tree".to_owned(),
            //         crate::entity::EntityType::Soul => "Soul".to_owned(),
            //         crate::entity::EntityType::Fish { air } => "Fish".to_owned(),
            //     },
            //     count
            // ),
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
                    Pixel::Lamp => "Lamp",
                },
                count
            ),
        };
        write!(f, "{s}")
    }
}

#[derive(Savefile)]
pub struct Inventory {
    pub items: Vec<Item>,
    pub open: bool,

    #[savefile_ignore]
    pub animation: f32,
}

impl Default for Inventory {
    fn default() -> Self {
        Inventory {
            items: vec![Item::Crafter { start: None }],
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

#[derive(Savefile)]
pub struct Player {
    #[savefile_ignore]
    pub x: f32,
    #[savefile_ignore]
    pub y: f32,
    #[savefile_ignore]
    pub vx: f32,
    #[savefile_ignore]
    pub vy: f32,
    #[savefile_ignore]
    pub zoom: f32,
    #[savefile_ignore]
    #[savefile_default_val="20.0"]
    pub health: f32,
    
    pub inventory: Inventory,
    
    // #[savefile_ignore]
    pub item_in_hand: Item,
    
    pub name: String,

    #[savefile_ignore]
    #[savefile_introspect_ignore]
    pub respawn_pos: Vec2,

    #[savefile_ignore]
    jump_height_timer: f32,
    #[savefile_ignore]
    craft_timer: f32,

    #[savefile_introspect_ignore]
    #[savefile_ignore]
    pub view_port_cache: Rect,

    pub hover_ui: bool,
    #[savefile_ignore]
    #[savefile_default_val="100.0"]
    pub battery: f32,
    pub charging: bool,
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
            hover_ui: true,
            view_port_cache: Rect::default(),
            battery: 100.0,
            charging: false,
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
            Item::Crafter { start:_ } => {
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
            // Item::SpawnEntity { entity, count } => {
            //     let mut added_count = false;
            //     for i in self.inventory.items.iter_mut() {
            //         if let Item::SpawnEntity {
            //             entity: entity2,
            //             count: count2,
            //         } = i
            //         {
            //             if entity == *entity2 {
            //                 *count2 += count;
            //                 added_count = true;
            //             }
            //         }
            //     }
            //     if !added_count {
            //         self.inventory
            //             .items
            //             .insert(0, Item::SpawnEntity { entity, count })
            //     }
            // }
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
                // let distance = (self.x.max(pt.x) - self.x.min(pt.x))
                    // .hypot(self.y.max(pt.y) - self.y.min(pt.y));

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
                for ((row, col), i) in result.2.indexed_iter() {
                    let px = (row + wand_rect.y as usize, col + wand_rect.x as usize);
                    map.grid[px] = *i;
                    map.update_texture_px.insert(px);
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
            // Item::SpawnEntity { entity, count } => {
            //     *count -= 1;
            //     // real point point in world
            //     map.spawn_entity(entity.entity_type.clone(), col as f32, row as f32);
            //     if *count == 0 {
            //         self.item_in_hand = Item::Hand;
            //     }
            // }
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
            x: self.x - screen_width() * scale / 2.0 * if cfg!(target = "wasm") {-1.0} else {1.0},
            y: self.y - screen_height() * scale / 2.0,
            w: screen_width() * scale,
            h: screen_height() * scale * if cfg!(target = "wasm") {-1.0} else {1.0},
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

    fn respawn(&mut self) {
        self.health = 20.0;
        self.x = self.respawn_pos.x;
        self.y = self.respawn_pos.y;
        self.battery = 100.0;
    }

    pub fn get_player_box(&self, offset_x: f32, offset_y: f32) -> HitLineSet {
        physics::make_bounding_box(Rect::new(self.x + offset_x, self.y + offset_y, 1.95, 2.95))
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.x, self.y, 1.95, 2.95)
    }

    pub fn update(&mut self, map: &Map, settings: &Settings) {
        let delta = if is_key_down(KeyCode::K) {
            get_frame_time() * 10.0
        } else {
            get_frame_time()
        };

        let mut remaining = delta;

        let move_left_pressed = (is_key_down(KeyCode::A)
            || (settings.mobile && root_ui().button(Vec2::new(0.0, screen_height() - 100.0), "<"))) 
            && self.battery != 0.0;
        let move_right_pressed = (is_key_down(KeyCode::D)
            || (settings.mobile && root_ui().button(Vec2::new(50.0, screen_height() - 100.0), ">")))
            && self.battery != 0.0;
        let jump_pressed = (is_key_down(KeyCode::Space)
            || (settings.mobile && root_ui().button(Vec2::new(50.0, screen_height() - 100.0), "^")))
            && self.battery != 0.0;

        

        let mut damage: f32 = 0.0;

        for pixel in map.get_region(self.rect()).iter() {
            damage = damage.max(pixel.player_damage());
        }

        self.health -= damage * delta * 2.0;

        if self.health < 0.0 {
            self.respawn()
        }

        let terrain_hit = physics::make_map_box(
            &map.grid, 
            Rect::new(self.x - 20.0, self.y - 20.0, 40.0, 40.0), 
            true, 
            self.x, 
            self.y
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
                            //self.x = 50.0;
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
                            //self.y -= self.vy.signum() * 0.01;
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

        if map.sky_light[self.x as usize] >= self.y as usize
            || map.sky_light[self.x as usize + 1] >= self.y as usize
        {
            self.battery += delta;
            self.charging = true;
        } else {
            self.charging = false;
        }

        self.jump_height_timer -= delta;
        self.jump_height_timer = self.jump_height_timer.clamp(0.0, 1.0);
        self.craft_timer -= delta;
        self.craft_timer = self.craft_timer.clamp(0.0, 1.0);

        if (on_ground | in_water) && is_key_down(KeyCode::Space) && self.vy > -100.0 {
            self.vy -= if in_water { 10.0 } else { 50.0 };
            self.jump_height_timer = 0.2;
        }

        if jump_pressed && self.vy > -200.0 && self.jump_height_timer > 0.0 {
            self.vy -= 500.0 * delta;
        }
        self.battery -= delta * 0.1;

        self.battery = self.battery.clamp(0.0, 100.0);

        self.vx *= 0.75_f32;

        if on_ground {
            self.vx *= 0.8_f32;
        }

        if in_water {
            self.vx *= 0.7_f32;
            self.vy *= 0.7f32;
        }

        if move_left_pressed && self.vx > -500.0 {
            self.vx -= 8.0;
        }
        // if is_key_down(KeyCode::A) && self.vx > -500.0 && on_ground {
        //     self.vx -= 4.0;
        // }

        if move_right_pressed && self.vx < 500.0 {
            self.vx += 8.0;
        }
        // if is_key_down(KeyCode::D) && self.vx < 500.0 && on_ground {
        //     self.vx +=8.0;
        // }

        self.view_port_cache = self.get_view_port();
    }

    pub fn save(&self) {
        if cfg!(target_family = "wasm") {return}

        if let Err(error) = create_dir_all("saves/players/") {
            println!("error {error}");
        }

        if let Err(error) = save_file(format!("saves/players/{}.player_save", self.name), SAVEFILE_VERSION, self) {
            println!("error {error}");
        }
    }
    pub fn load(name: &str) -> Player {
        load_file(format!("saves/players/{}.player_save", name), SAVEFILE_VERSION).unwrap_or_default()
    }

}

