use grid::*;

mod map;
mod update;


use macroquad::prelude::*;
use map::{Map, Pixel};

#[macroquad::main("BasicShapes")]
async fn main() {

    let mut map = Map::new_square(100);
    map.update_image();
    let texture: Texture2D = Texture2D::from_image(&map.image);
    texture.set_filter(FilterMode::Nearest);
    map.make_square(map::Pixel::Water);
    // map.make_log();

    let mut paused = true;
    
    // let texture_heatmap: Texture2D = Texture2D::from_image(&map.heatmap);
    
    let mut draw = Pixel::Air;
    let mut hover = None;

    loop {
        clear_background(WHITE);

        if !paused {
            map.update_state();
        }

        match get_char_pressed() {
            Some(' ') => {paused = !paused},
            Some('c') => {map.make_square(map::Pixel::Air);},
            Some('s') => {map.update_state();},
            Some('d') => {draw = draw.cycle()},
            _ => {}
        }
        
        let (mut x,mut y) = mouse_position();
        x=x/600.0;
        y=y/600.0;
        let row = ((map.size as f32 * y as f32) as usize).clamp(2 , map.size as usize -2);
        let col = ((map.size as f32 * x ) as usize).clamp(2 , map.size as usize -2);
        if x<=1.0 && y<=1.0 {
            hover = Some(map.grid[(row, col)]);
            if is_mouse_button_down(MouseButton::Left) {
                map.grid[(row, col)] = draw;
                for x in 0..3 {
                for y in 0..3 {
                map.update_texture_px.push((row +y -1, col + x -1));
                }
                }
                
            }
        }

        if !map.update_texture_px.is_empty() {
            map.update_image();
            // texture.update_part(&image, x_offset, y_offset, width, height);
            texture.update(&map.image);
            map.update_texture_px = vec![];
            // texture_heatmap.update(&map.heatmap);
        }




        draw_texture_ex(&texture, 0.0, 0.0, WHITE,  DrawTextureParams { 
            dest_size: Some(Vec2::new(600.0, 600.0)),
            ..Default::default()});
        // draw_texture_ex(&texture_heatmap, 0.0, 0.0, Color { r: 1.0, g: 1.0, b: 1.0, a: 0.3 },  DrawTextureParams { 
        //     dest_size: Some(Vec2::new(512.0, 512.0)),
        //     ..Default::default()});

        draw_text(&format!("fps: {}", get_fps()), 10.0, 20.0, 20.0, BLACK);
        draw_text(&format!("{:?}", hover), 10.0, 50.0, 20.0, BLACK);
        draw_text(&format!("Using {:?}", draw), 10.0, 80.0, 20.0, BLACK);
        next_frame().await
    }
}