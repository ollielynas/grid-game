use std::borrow::BorrowMut;

use entity::EntityType;
use grid::*;

mod map;
mod update;
mod entity;
mod player;
mod cam;



use macroquad::{prelude::*, ui::root_ui};
use map::{Map, Pixel};
use player::Player;

#[macroquad::main("BasicShapes")]
async fn main() {
    

    
    let mut player = Player::default();
    
    
    let mut map = Map::new_square(101);
    map.update_image();
    let texture: Texture2D = Texture2D::from_image(&map.image);
    texture.set_filter(FilterMode::Nearest);
    map.make_square(map::Pixel::Water);
    // map.make_log();
    map.spawn_entity(EntityType::Tree, 50.0, 10.0);
    map.spawn_entity(EntityType::Fish{air:20.0}, 100.0, 100.0);
    
    let mut paused = false;
    
    // let texture_heatmap: Texture2D = Texture2D::from_image(&map.heatmap);
    
    let mut draw = Pixel::Air;
    let mut hover = None;
    
    loop {
        player.update(&map);
        set_camera(&player.cam());
        clear_background(WHITE);
        
        if !paused {
            map.update_state();
            map.entities.retain_mut(|x| x.update(&(map.grid)));
        }

        match get_char_pressed() {
            Some('c') => {map.make_square(map::Pixel::Air);},
            Some('t') => {map.update_state();},
            Some('f') => {draw = draw.cycle()},
            Some('g') => {map.gen_terrain()},
            _ => {}
        }
        let mouse = mouse_position();

        // real point point in world
        let pt = player.cam().screen_to_world(Vec2::new(mouse.0, mouse.1));

        let row = (pt.y as usize).clamp(2 , map.size as usize -2);
        let col = (pt.x as usize).clamp(2 , map.size as usize -2);
        if pt.y as usize == row && pt.x as usize == col || true {
            hover = Some(map.grid[(row, col)]);
            if is_mouse_button_down(MouseButton::Left) {
                for x in 0..3 {
                    for y in 0..3 {
                    map.grid[(row +y -1, col + x -1)] = draw;
                map.update_texture_px.push((row +y -1, col + x -1));
                }
                }
            }
        }

        if !map.update_texture_px.is_empty() {
            map.update_image();
            texture.update(&map.image);
            map.update_texture_px = vec![];
        }
        
        
        draw_rectangle(player.x, player.y, 2.0, 3.0, ORANGE);
        
        for e in &map.entities {
            draw_texture_ex(&e.texture, e.x, e.y - e.height +1.0, WHITE,  DrawTextureParams { 
                dest_size: Some(Vec2::new(e.width, e.height)),
                ..Default::default()});
        }
        
        draw_texture_ex(&texture, 0.0, 0.0, WHITE,  DrawTextureParams { 
            ..Default::default()});
        
        
        

        
        root_ui().label(None, &format!("fps: {}", get_fps()));
        root_ui().label(None, &format!("Using {:?}", draw));
        // root_ui().slider(0, "health", 0.0..20.0, &mut player.health);


        next_frame().await

    }
}