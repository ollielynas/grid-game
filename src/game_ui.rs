use macroquad::{experimental::animation, math::{vec2, Vec2}, miniquad::Context, time::get_frame_time, ui::{root_ui, widgets::{self, Group, Popup}}, window::{screen_height, screen_width}, Window};
use crate::player::{Inventory, Player};
use macroquad::prelude::*;

impl Player {
    pub fn render(&mut self) {
        let delta = get_frame_time();
        // let hand_item = self.item_in_hand;
        let mut equip_item: Option<crate::player::Item> = None;
        self.inventory.animation -= if self.inventory.open {1.0} else {-1.0} * 4.0 * delta;
        self.inventory.animation = self.inventory.animation.clamp(0.0, 1.0);
        
        if self.inventory.animation != 1.0 {
        
                widgets::Window::new(128, vec2(100., 100.0 ), vec2(screen_width() - 200.0, screen_height() - 200.0))
                    .label("inventory")
                    .titlebar(true)
                    .ui(&mut *root_ui(), |ui| {
                        Group::new(9999 as u64+99, Vec2::new(screen_width() - 200.0, 80.)).ui(ui, |ui| {
                            if ui.button(None, "Holding: ") {
                                
                            }
                            ui.label(None, &format!("{:?}", self.item_in_hand));
                        });
                        for (i,item) in self.inventory.items.iter().enumerate() {
                            Group::new(i as u64+99, Vec2::new(screen_width() - 200.0, 80.)).ui(ui, |ui| {
                                if ui.button(None, "Equip") {
                                    equip_item = Some(item.clone());
                                }
                                ui.label(None, &format!("{item:?}"));
                            });
                        }
                    });
                
            // egui_macroquad::ui(|egui_ctx| {
            //     egui::Window::new("egui ❤ macroquad")

            //         .show(egui_ctx, |ui| {
            //             ui.label("Test");
            //         });
            // });

            // Draw things before egui
    
            // egui_macroquad::draw();
        }

        // equip_item = Some(item.clone());

        if let Some(item) =  equip_item {
            self.gain_item(self.item_in_hand.clone());
            self.item_in_hand = item;
            self.inventory.items.retain(|x| x != &self.item_in_hand);
        }

    
}
}