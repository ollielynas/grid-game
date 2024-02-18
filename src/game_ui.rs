use std::{default, fmt::format, fs::{self, create_dir_all}};

// use egui::util::hash;
use crate::{
    map::Map,
    player::{self, Inventory, Item, Player},
};
use egui_macroquad::{egui::util::hash, macroquad::prelude::*};
use egui_macroquad::{
    egui::{self, Align2, Color32, Id, RichText},
    macroquad::{
        experimental::animation,
        math::{vec2, Vec2},
        miniquad::Context,
        time::get_frame_time,
        ui::{
            hash, root_ui,
            widgets::{self, Group, Popup},
        },
        window::{screen_height, screen_width},
        Window,
    },
};

impl Player {
    pub fn render_ui(&mut self) -> bool {
        let delta = get_frame_time();
        // let hand_item = self.item_in_hand;
        let mut equip_item: Option<crate::player::Item> = None;
        self.inventory.animation = (self.inventory.animation + delta) % 10.0;
        let mut return_value = false;

        let vb = self.get_view_port();

        // draw_rectangle(1.0+vb.x, vb.y + 2.0, 20.0* 0.8, 2.0, GRAY);
        // draw_rectangle_lines(vb.x+1.0, vb.y + 2.0, 20.0* 0.8, 2.0, 0.4,BLACK);

        // draw_rectangle(1.0+vb.x, vb.y + 2.0, self.health* 0.8, 2.0, RED);
        // draw_rectangle_lines(vb.x+1.0, vb.y + 2.0, self.health* 0.8, 2.0, 0.4,BLACK);

        egui_macroquad::ui(|egui_ctx| {
            egui::Area::new("info")
                .anchor(Align2::LEFT_TOP, [5.0, 5.0])
                .show(egui_ctx, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(&format!("INTEGRITY: {}%", self.health / 2.0 * 10.0));
                        if self.inventory.animation % 1.0 > 0.5 && self.health < 5.0 {
                            ui.label("WARNING! LOW INTEGRITY");
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label(&format!("Battery: {}%", self.battery.round()));
                        if self.inventory.animation % 1.0 > 0.5 && self.battery < 25.0 {
                            ui.label("WARNING! LOW BATTERY");
                        } else if self.charging {
                            ui.label("*");
                        }
                    });
                    self.hover_ui = egui_ctx.is_pointer_over_area();
                });
            egui::Window::new("")
                .id(Id::new("bottom info"))
                .anchor(Align2::LEFT_BOTTOM, [0.0, 0.0])
                .show(egui_ctx, |ui| {
                    ui.label(&format!("FPS: {}", get_fps()));
                    ui.label(&format!("X / Y: {} {}", self.x, self.y));

                    self.hover_ui = egui_ctx.is_pointer_over_area();
                });
            egui::Window::new("")
                .id(Id::new("bottom center buttons"))
                .anchor(Align2::CENTER_BOTTOM, [0.0, 0.0])
                .show(egui_ctx, |ui| {
                    if ui.button("LEAVE MISION").clicked() {
                        // self.save();
                        
                        return_value = true;
                    }
                    if ui
                        .button(RichText::new("SELF DESTRUCT").color(Color32::RED))
                        .clicked()
                    {
                        self.health = -0.1;
                    }
                    self.hover_ui = egui_ctx.is_pointer_over_area();
                });
            egui::Area::new("")
                .id(Id::new("LIVE"))
                .anchor(Align2::CENTER_TOP, [0.0, 5.0])
                .show(egui_ctx, |ui| {
                    ui.colored_label(Color32::from_rgb(255, 0, 0), "* LIVE");

                    self.hover_ui = egui_ctx.is_pointer_over_area();
                });

            egui::Window::new("")
                .id(Id::new("option"))
                .anchor(Align2::LEFT_CENTER, [10.0, 0.0])
                .show(egui_ctx, |ui| {
                    ui.label(format!("CURRENTLY HOLDING: {}", self.item_in_hand));
                    ui.label("");

                    if ui
                        .button(
                            " > Craft".to_owned()
                                + if matches!(self.item_in_hand, Item::Crafter { start: _ }) {
                                    "*"
                                } else {
                                    ""
                                },
                        )
                        .clicked()
                    {
                        equip_item = Some(Item::Crafter { start: None })
                    }
                    if ui
                        .button(
                            " > Mine".to_owned()
                                + if matches!(self.item_in_hand, Item::Pickaxe) {
                                    "*"
                                } else {
                                    ""
                                },
                        )
                        .clicked()
                    {
                        equip_item = Some(Item::Pickaxe)
                    }
                    if ui
                        .button(
                            " > Place".to_owned()
                                + if matches!(
                                    self.item_in_hand,
                                    Item::PlacePixel { pixel: _, count: _ }
                                ) {
                                    "*"
                                } else {
                                    ""
                                },
                        )
                        .clicked()
                    {
                        equip_item =
                            Some(self.inventory.items.get(0).unwrap_or(&Item::Hand).clone());
                    }

                    if matches!(self.item_in_hand, Item::PlacePixel { pixel: _, count: _ })
                        && ui.button("  > Select Item").clicked()
                    {
                        self.inventory.open = !self.inventory.open;
                    }
                });

            egui::Window::new("Inventory")
                .vscroll(true)
                .anchor(Align2::RIGHT_TOP, [0.0, 0.0])
                .open(&mut self.inventory.open)
                .show(egui_ctx, |ui| {
                    for item in &self.inventory.items {
                        if matches!(item, Item::PlacePixel { pixel: _, count: _ })
                            && ui.button(&format!("> {item}")).clicked()
                        {
                            equip_item = Some(item.clone());
                        }
                    }
                });
        });

        // equip_item = Some(item.clone());

        if let Some(item) = equip_item {
            self.gain_item(self.item_in_hand.clone());
            self.item_in_hand = item;
            self.inventory.items.retain(|x| {
                x != &self.item_in_hand && matches!(x, Item::PlacePixel { pixel: _, count: _ })
            });
        }

        return return_value;
    }
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
    }
}

fn color_command(s: &str) -> Color32 {
    match s {
        "HELP" => Color32::LIGHT_BLUE,
        "DEBUG" => Color32::LIGHT_BLUE,
        _ => Color32::WHITE,
    }
}

/// note to self: all terminal commands should be full caps
pub async fn terminal() -> (Map, Player) {
    let mut player = None;
    let mut map = None;

    let mut name = "todo make world gen".to_owned();
    let mut size = "300".to_string();
    let mut size_int = 300;
    let mut creative = false;
    let mut advanced_water = true;
    let mut blank = false;
    let mut seed = fastrand::u64(10000..99999).to_string();

    let mut loadable_names: Vec<String> = vec![];

    if !cfg!(target_family = "wasm") {
        if let Err(error) = create_dir_all("saves/maps/") {
            println!("error {error}");
        }
    for path in fs::read_dir("./saves/maps/").unwrap() {
        loadable_names.push(
            path.unwrap()
                .path()
                .file_name()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
                .to_owned()
                .replace(".map_save", ""),
        );
    }
    }

    let mut process_state = 0;

    loop {
        clear_background(BLACK);
        egui_macroquad::ui(|egui_ctx| {
            egui::Area::new("terminal")
                .anchor(Align2::LEFT_TOP, [10.0, 10.0])
                .show(egui_ctx, |ui| {
                    match process_state {
                        0 => {
                            if ui.button("> Start Game!").clicked() {
                                process_state = 1;
                            }
                        }
                        1 => {
                            if ui.button("> New World").clicked() {
                                process_state = 2;
                            }
                            if !cfg!(target_family = "wasm") {
                            if ui.button("> Load World").clicked() {
                                process_state = 3;
                            }
                        }
                            if ui.button("> Debug World").clicked() {
                                let mut final_player = Player::new("debug".to_owned());
                                let mut final_map = Map::new_square(200, "debug".to_owned());
                                final_map.gen_terrain();
                                final_map.update_image();
                                final_player.inventory = Inventory::creative();
                                let respawn_point =
                                    Vec2::new(final_map.size as f32 / 2.0 - 1.0, 4.0);

                                final_player.x = respawn_point.x;
                                final_player.y = respawn_point.y;

                                final_player.respawn_pos = respawn_point;
                                for ((row, col), _) in final_map.grid.indexed_iter() {
                                    final_map.update_texture_px.insert((row, col));
                                }
                                final_map.update_image();
                                map = Some(final_map);
                                player = Some(final_player);
                                // return (final_map, final_player);
                            }
                        }
                        2 => {
                            ui.label(RichText::new("Start New Mission").size(25.0));
                            ui.horizontal(|ui| {
                                ui.label("Name: [");
                                ui.text_edit_singleline(&mut name);
                                ui.label("]")
                            });

                            ui.horizontal(|ui| {
                                ui.label("Size: [");
                                if ui.text_edit_singleline(&mut size).lost_focus() {
                                    if let Ok(num) = size.parse::<usize>() {
                                        size_int = num;
                                    }
                                    size = size_int.to_string();
                                };
                                ui.label("]")
                            });
                            ui.horizontal(|ui| {
                                ui.label("Seed: [");
                                ui.text_edit_singleline(&mut seed);
                                ui.label("]")
                            });
                            if ui
                                .button(format!(
                                    "[{}] Advanced Water",
                                    if advanced_water { "x" } else { " " }
                                ))
                                .clicked()
                            {
                                advanced_water = !advanced_water
                            };
                            if ui
                                .button(format!("[{}] Creative", if creative { "x" } else { " " }))
                                .clicked()
                            {
                                creative = !creative
                            };
                            if ui
                                .button(format!("[{}] Blank", if blank { "x" } else { " " }))
                                .clicked()
                            {
                                blank = !blank
                            };
                            ui.label(" ");
                            if ui.button("> Launch").clicked() {
                                fastrand::seed(hash(seed.clone()));
                                let mut final_player = Player::new(name.clone());
                                let mut final_map = Map::new_square(size_int, name.clone());
                                final_map.gen_terrain();
                                final_player.inventory = if creative {
                                    Inventory::creative()
                                } else {
                                    Inventory::default()
                                };
                                let respawn_point =
                                    Vec2::new(final_map.size as f32 / 2.0 - 1.0, 4.0);

                                final_player.x = respawn_point.x;
                                final_player.y = respawn_point.y;

                                final_player.respawn_pos = respawn_point;
                                for ((row, col), _) in final_map.grid.indexed_iter() {
                                    final_map.update_texture_px.insert((row, col));
                                }
                                final_map.update_image();
                                map = Some(final_map);
                                player = Some(final_player);
                            }
                        }
                        3 => {
                            ui.label(RichText::new("Continue Previous Mission").size(25.0));

                            for save in &loadable_names {
                                if ui.button(format!("> {save}")).clicked() {
                                    let mut final_player = Player::load(&save);
                                    let mut final_map = Map::load(&save);
                                    let respawn_point =
                                    Vec2::new(final_map.size as f32 / 2.0 - 1.0, 4.0);

                                final_player.x = respawn_point.x;
                                final_player.y = respawn_point.y;

                                final_player.respawn_pos = respawn_point;
                                for ((row, col), _) in final_map.grid.indexed_iter() {
                                    final_map.update_texture_px.insert((row, col));
                                }
                                    final_map.update_image();
                                    map = Some(final_map);
                                    player = Some(final_player);
                                }
                            }
                            if ui.button("> Back").clicked() {
                                process_state = 1;
                            }
                        }
                        _ => {
                            ui.label("an error has occurred");
                        }
                    }
                });
        });

        egui_macroquad::draw();

        next_frame().await;

        if map.is_some() && player.is_some() {
            break;
        }
    }

    return (
        map.unwrap_or_else(|| Map::new_square(150, "ERROR I SHOULD NOT EXIST".to_owned())),
        player.unwrap_or_default(),
    );
}
