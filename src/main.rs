
use grid::*;

mod map;
mod update;
mod entity;
mod player;



use macroquad::{prelude::*, ui::root_ui};
use map::{Map, Pixel};
use player::Player;

/// size of map
const SIZE: usize = 101;

fn window_conf() -> Conf {
        Conf {
            window_title: "pixel game".to_owned(),
            window_width: 1200,
            window_height: 800,
            high_dpi: true,
            ..Default::default()
        }
    }

#[macroquad::main(window_conf)]
async fn main() {
    
    let mut player = Player::default();
    
    
    let mut map = Map::new_square(SIZE);
    map.update_image();

    
    let texture: Texture2D = Texture2D::from_image(&map.image);
    let light_texture: Texture2D = Texture2D::from_image(&map.light_mask);
    
    texture.set_filter(FilterMode::Nearest);
    // light_texture.set_filter(FilterMode::Nearest);

    // map.make_square(map::Pixel::Water);
    // map.make_log();

    let paused = false;
    
    // let texture_heatmap: Texture2D = Texture2D::from_image(&map.heatmap);
    
    let mut draw: Pixel = Pixel::Air;
    let mut hover: Option<Pixel>;
    
    loop {
        player.update(&map);
        
        set_camera(&player.cam());
        // clear_background(Color { r: 0.8, g: 0.8, b: 0.8, a: 1.0 });
        clear_background(WHITE);
        
        if !paused {
            map.update_state(&player);
            map.entities.retain_mut(|x| x.update(&(map.grid)));
        }

        light_texture.update(&map.light_mask);

        match get_char_pressed() {
            Some('c') => {map.make_square(map::Pixel::Air);},
            Some('t') => {map.update_state(&player);},
            Some('f') => {draw = draw.cycle()},
            Some('g') => {map.gen_terrain()},
            _ => {}
        }
        let mouse = mouse_position();

        // real point point in world
        let pt = player.cam().screen_to_world(Vec2::new(mouse.0, mouse.1));

        let mouse_row = (pt.y as usize).clamp(2 , map.size as usize -2);
        let mouse_col = (pt.x as usize).clamp(2 , map.size as usize -2);
        
            hover = Some(map.grid[(mouse_row, mouse_col)]);
            if is_mouse_button_down(MouseButton::Left) {
                for x in 0..3 {
                    for y in 0..3 {
                    map.grid[(mouse_row +y -1, mouse_col + x -1)] = draw;
                map.update_texture_px.push((mouse_row +y -1, mouse_col + x -1));
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

        draw_texture_ex(&light_texture, 0.0, 0.0, WHITE,  DrawTextureParams { 
            ..Default::default()});
        

        let hit = player.make_map_box(&map, player.get_view_port(), false);
        hit.render();

        player.get_player_box().render();
        
        root_ui().label(None, &format!("fps: {}", get_fps()));
        root_ui().label(None, &format!("Using {:?}", draw));
        if hover != Some(Pixel::Air) {
            root_ui().label(None, &format!("{:?}", hover));
        }


        next_frame().await

    }
}