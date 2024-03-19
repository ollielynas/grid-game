use egui_macroquad::{egui::{self, Align2, Button, RichText}, macroquad::prelude::*};
use savefile::load_file;
use savefile_derive::Savefile;

use crate::SAVEFILE_VERSION;




pub const MIN_SIM_DISTANCE:i32 = 100;
pub const FPS_BUFFER:i32 = 5;


#[derive(Savefile)]
pub struct Settings {
    pub mobile: bool,
    pub sim_distance: i32,
    pub min_fps: i32,
    pub dynamic_simulation_distance: bool,
    pub open: bool,
}

impl Default for Settings {


    fn default() -> Self {

        let mut settings =  Settings {
            mobile: false,
            sim_distance: MIN_SIM_DISTANCE +1,
            min_fps: 25,
            dynamic_simulation_distance: true,
            open: false,
        };
        
        if cfg!(not(target_family="wasm")) {
        if let Ok(file) = load_file("saves/user_settings.bin", SAVEFILE_VERSION) {
            settings = file;
        }
        }

        settings.sim_distance = settings.sim_distance.max(MIN_SIM_DISTANCE);

        return  settings;
    }
}

impl Settings {
    pub fn save(&self) {
        savefile::save_file("saves/user_settings.bin", SAVEFILE_VERSION, self);
    }




    pub async fn settings_ui(&mut self, blur_material:  Option<Material>) {

        let mut sim_distance_string: String = self.sim_distance.to_string();
        let mut min_fps_string: String = self.min_fps.to_string();

        // let mut text_edit_number = 0;

        let background = get_screen_data();

        // background.bytes = gaussian_blur::blur(background.height(), background.width(), [1, 14, 62, 102, 62, 14, 1], background.bytes).0;

        let background_texture = Texture2D::from_image(&background);

        while self.open{
            clear_background(LIGHTGRAY);

            if let Some(mat) = blur_material {
            gl_use_material(mat);
            mat.set_uniform("textureSize", (screen_width(), screen_height()));
            draw_texture_ex(background_texture, 0.0, 0.0, WHITE, DrawTextureParams {
                dest_size: Some(Vec2{y:screen_height(), x:screen_width()}),
                source: None,
                rotation: 0.0,
                flip_x: false,
                flip_y: true,
                pivot: None,
            });
            
            }
    
            egui_macroquad::ui(|egui_ctx| {
                egui::Window::new("Settings")
                    .anchor(Align2::LEFT_TOP, [0.0, 0.0])
                    .scroll2([false,true])
                    .collapsible(false)
                    .fixed_size([screen_width(),screen_height()])
                    .open(&mut self.open)
                    .show(egui_ctx, |ui| {
                        // ui.label();
                        
                        ui.vertical(|ui| {
                        
                        
                        
                        if ui
                                .button(format!(
                                    "[{}] Dynamic Simulation Distance",
                                    if self.dynamic_simulation_distance { "x" } else { " " }
                                ))
                                .clicked()
                            {
                                self.dynamic_simulation_distance = !self.dynamic_simulation_distance
                            };

                        // edit block
                        ui.horizontal(|ui| {
                            ui.label("Dynamic simulation distance: [");
                            if ui.text_edit_singleline(&mut sim_distance_string).lost_focus() {
                                if let Ok(num) = sim_distance_string.parse::<i32>() {
                                    self.sim_distance = num;
                                }
                                sim_distance_string = self.sim_distance.to_string();
                            };
                            ui.label("]")
                        });
                        
                        ui.horizontal(|ui| {
                            ui.label("Minimum FPS: [");
                            if ui.text_edit_singleline(&mut min_fps_string).lost_focus() {
                                if let Ok(num) = min_fps_string.parse::<i32>() {
                                    self.min_fps = num;
                                }
                                min_fps_string = self.min_fps.to_string();
                            };
                            ui.label("]")
                        });
                    });

                    });
                });

            egui_macroquad::draw();

            next_frame().await;
        }
    }
}