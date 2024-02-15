use std::default;

// use egui::util::hash;
use egui_macroquad::{egui::{self, Align2}, macroquad::{experimental::animation, math::{vec2, Vec2}, miniquad::Context, time::get_frame_time, ui::{hash, root_ui, widgets::{self, Group, Popup}}, window::{screen_height, screen_width}, Window}};
use crate::{map::Map, player::{Inventory, Player}};
use egui_macroquad::macroquad::prelude::*;

impl Player {
    pub fn render_ui(&mut self) -> bool {
        let delta = get_frame_time();
        // let hand_item = self.item_in_hand;
        let mut equip_item: Option<crate::player::Item> = None;
        self.inventory.animation -= if self.inventory.open {1.0} else {-1.0} * 4.0 * delta;
        self.inventory.animation = self.inventory.animation.clamp(0.0, 1.0);

        
        let vb = self.get_view_port();
        
        draw_rectangle(1.0+vb.x, vb.y+(self.inventory.animation) * 10.0 - 9.0 , 20.0* 0.8, 2.0, GRAY);
        draw_rectangle_lines(vb.x+1.0, vb.y+(self.inventory.animation) * 10.0 -9.0 , 20.0* 0.8, 2.0, 0.4,BLACK);
        
        draw_rectangle(1.0+vb.x, vb.y+(self.inventory.animation) * 10.0 - 9.0 , self.health* 0.8, 2.0, RED);
        draw_rectangle_lines(vb.x+1.0, vb.y+(self.inventory.animation) * 10.0 -9.0 , self.health* 0.8, 2.0, 0.4,BLACK);

        let offset = (1.0-self.inventory.animation) * 100.0;
        
        egui_macroquad::ui(|egui_ctx| {
            
            
            
            if self.inventory.animation != 1.0 {
                egui::Window::new("Inventory")
                .anchor(Align2::CENTER_CENTER, [0.0,0.0])
                .show(egui_ctx, |ui| {
                    ui.label("Test");
                });
            } else {
                egui::Area::new("settings")
                
                .show(egui_ctx, |ui| {
                    if ui.button("Inventory").clicked() {
                        self.inventory.open = true;
                    }
                });
            }
        });

        // equip_item = Some(item.clone());

        if let Some(item) =  equip_item {
            self.gain_item(self.item_in_hand.clone());
            self.item_in_hand = item;
            self.inventory.items.retain(|x| x != &self.item_in_hand);
        }

        return false

    
}
}


pub async fn home() -> (Map, Player) {

    let mut map: Option<Map> = None;
    let mut player: Option<Player> =  None;
    let mut opt_test = false;

    loop {
        clear_background(WHITE);
        
        root_ui().label(None, "Game Title");
        root_ui().separator();
        root_ui().separator();
        
        root_ui().label(None, " ");
        if let Some(ref player2) = player {
            root_ui().label(None, &format!("Loaded Player: {}", player2.name))
        }else {
            root_ui().label(None, "to start playing you need to create or load a player")
        }
        if root_ui().button(None, "New Player") {
            player = Some(player_gen().await)
        }
        if root_ui().button(None, "Load Player") {
        }
        root_ui().separator();
        
        
        root_ui().label(None, " ");
        if let Some(ref map2) = map {
            root_ui().label(None, &format!("Loaded Map: {}", map2.name))
        }else {
            root_ui().label(None, "to start playing you need to create or load a map")
        }
        if root_ui().button(None, "New Map") {
            map = Some(map_gen().await)
        }
        if root_ui().button(None, "load map") {
        }
        
        
        if map.is_some() && player.is_some() {
            root_ui().label(None, " ");
            if root_ui().button(None, "Start Playing!") {
                let mut final_player = player.unwrap();
                let mut final_map = map.unwrap();
                final_map.update_image();
                let respawn_point = Vec2::new(final_map.size as f32 / 2.0 - 1.0, 4.0);
                
                final_player.x = respawn_point.x;
                final_player.y = respawn_point.y;
                
                final_player.respawn_pos = respawn_point;
                
                return (final_map, final_player);
            }
        }
        root_ui().label(None, " ");
        root_ui().label(None, "or create a debug world");
        root_ui().checkbox(6234, "opt test", &mut opt_test);

        if root_ui().button(None, "Debug") {
            player = Some(Player::new("debug".to_owned()));
            map = Some(Map::new_square(if opt_test {800} else {200}, "debug".to_owned()));
            let mut final_player = player.unwrap();
                let mut final_map = map.unwrap();
                final_map.gen_terrain();
                final_map.update_image();
                final_player.inventory = Inventory::creative();
                let respawn_point = Vec2::new(final_map.size as f32 / 2.0 - 1.0, 4.0);

                final_player.x = respawn_point.x;
                final_player.y = respawn_point.y;

                final_player.respawn_pos = respawn_point;
                for ((row, col),_) in final_map.grid.indexed_iter() {
                    final_map.update_texture_px.insert((row, col));
                }
                final_map.update_image();
                return (final_map, final_player);
        }



        root_ui().label(None, " ");
        root_ui().label(None, " tip: press i to open inventory");


        next_frame().await
    };
}


pub async fn player_gen() -> Player {

    let mut name = "Player Name Here".to_owned();
    let mut player: Player;

    let mut creative_player = false;

    loop {
        clear_background(WHITE);

        root_ui().label(None, "New Player");
        root_ui().input_text(2, "Name", &mut name);
        root_ui().checkbox(432, "creative Player", &mut creative_player);
        if root_ui().button(None, "Create") {
            player = Player::new(name);
            if creative_player {
                player.inventory = Inventory::creative();
            }
            return player;
        }

        next_frame().await
    };
}


pub async fn map_gen() -> Map {

    let mut map_size: i32 = 500;
    let mut map_size_string: String = "500".to_owned();
    let mut seed = fastrand::i32(1000000000..2147483647).to_string();
    let mut name = "Map Name Here".to_owned();
    let mut map: Map;
    let mut blank = false;
    let mut realistic_fluid = true;
    loop {
        clear_background(WHITE);

        root_ui().label(None, "New World");

        root_ui().input_text(2, "Name", &mut name);
        root_ui().input_text(3, "Seed", &mut seed);
        root_ui().input_text(0, "World Size", &mut map_size_string);
        if let Ok(num) = map_size_string.parse::<i32>() {
            map_size = num;
        }else {
            root_ui().label(None, "invalid world size");
        }
        
        root_ui().checkbox(432, "Blank World", &mut blank);
        root_ui().checkbox(432, "Realistic Fluid", &mut realistic_fluid);

        if root_ui().button(None, "Create") {
            fastrand::seed(hash!(seed.clone()));
            map = Map::new_square(map_size as usize, name.clone());
            if !blank {
                map.gen_terrain();
            }
            map.realistic_fluid = realistic_fluid;
            return map;
        }

        next_frame().await
    };
}