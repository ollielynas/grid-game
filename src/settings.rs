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
}

impl Default for Settings {


    fn default() -> Self {
        
        let mut settings:Settings = load_file("saves/user_settings.bin", SAVEFILE_VERSION).unwrap_or(
            Settings {
                mobile: false,
                sim_distance: 10000,
                min_fps: 25,
                dynamic_simulation_distance: true,
            }
        );

        settings.sim_distance = settings.sim_distance.max(MIN_SIM_DISTANCE);

        return  settings;
    }
}

impl Settings {
    pub fn save(&self) {
        savefile::save_file("saves/user_settings.bin", SAVEFILE_VERSION, self);
    }
}