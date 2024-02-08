mod entity;
mod game_ui;
mod map;
mod player;
mod skin_style;
mod update;

use game_ui::home;
use savefile::prelude::*;



use macroquad::{
    prelude::*,
    ui::{root_ui, Skin, Style},
};
use map::{Map, Pixel};
use player::{Item, Player};

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
    
    let skins = skin_style::get_skins();
    
    root_ui().push_skin(&skins[1]);

    let (mut map, mut player) = home().await;




    let texture: Texture2D = Texture2D::from_image(&map.image);
    let light_texture: Texture2D = Texture2D::from_image(&map.light_mask);

    texture.set_filter(FilterMode::Nearest);
    // light_texture.set_filter(FilterMode::Nearest);

    // map.make_square(map::Pixel::Water);
    // map.make_log();
    


    let paused = false;
    show_mouse(false);

    // let texture_heatmap: Texture2D = Texture2D::from_image(&map.heatmap);

    let mut hover: Option<Pixel>;


    // root_ui().pop_skin();
    loop {
        player.update(&map);

        set_camera(&player.cam());
        // clear_background(Color { r: 0.8, g: 0.8, b: 0.8, a: 1.0 });
        clear_background(WHITE);

        player.render();

        if !paused {
            map.update_state(&player);
            map.entities.retain_mut(|x| x.update(&(map.grid)));
        }

        light_texture.update(&map.light_mask);

        match get_char_pressed() {
            Some('c') => {
                map.make_square(map::Pixel::Air);
            }
            Some('t') => {
                map.update_state(&player);
            }
            Some('g') => map.gen_terrain(),
            Some('i') => {
                player.inventory.open = !player.inventory.open;
                show_mouse(player.inventory.open);
            }
            _ => {}
        }
        let mouse = mouse_position();

        // real point point in world
        let pt = player.cam().screen_to_world(Vec2::new(mouse.0, mouse.1));
        let distance = (player.x.max(pt.x) - player.x.min(pt.x))
            .hypot(player.y.max(pt.y) - player.y.min(pt.y));

        let mouse_row = (pt.y as usize).clamp(2, map.size as usize - 2);
        let mouse_col = (pt.x as usize).clamp(2, map.size as usize - 2);
        hover = Some(map.grid[(mouse_row, mouse_col)]);
        if is_mouse_button_down(MouseButton::Left) && distance < 25.0 && !player.inventory.open {
            map.update_texture_px.push((mouse_row, mouse_col));

            player.use_item(&mut map, mouse_row, mouse_col)
        }

        if !map.update_texture_px.is_empty() {
            map.update_image();
            texture.update(&map.image);
            map.update_texture_px = vec![];
        }

        draw_rectangle(player.x, player.y, 2.0, 3.0, ORANGE);

        for e in &map.entities {
            draw_texture_ex(
                &e.texture,
                e.x,
                e.y - e.height + 1.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(e.width, e.height)),
                    ..Default::default()
                },
            );
        }

        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                ..Default::default()
            },
        );

        draw_texture_ex(
            &light_texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                ..Default::default()
            },
        );

        let hit = player.make_map_box(&map, player.get_view_port(), false);
        hit.render();

        player.get_player_box(0.0, 0.0).render();

        root_ui().label(None, &format!("fps: {}", get_fps()));
        if hover != Some(Pixel::Air) {
            root_ui().label(None, &format!("{:?}", hover));
        }

        if distance >= 25.0 {
            draw_circle(pt.x, pt.y, 0.5, LIGHTGRAY);
            draw_circle_lines(pt.x, pt.y, 0.4, 0.3, BLACK);
        } else {
            match player.item_in_hand {
                Item::Pickaxe if hover != Some(Pixel::Air) => {
                    draw_rectangle_lines(pt.x.floor(), pt.y.floor(), 1.0, 1.0, 0.5, RED);
                }
                Item::PlacePixel { pixel, count } => {
                    draw_rectangle_lines(pt.x.floor(), pt.y.floor(), 1.0, 1.0, 0.5, LIGHTGRAY);
                    draw_text_ex(
                        &format!("{count}"),
                        pt.x.ceil(),
                        pt.y.ceil(),
                        
                        TextParams {
                            font_size: 20,
                            font_scale: 0.1,
                            ..Default::default()
                        },
                    )
                }
                _ => {
                    draw_circle(pt.x, pt.y, 0.5, LIGHTGRAY);
                    draw_circle_lines(pt.x, pt.y, 0.4, 0.3, BLACK);
                }
            }
        }

        next_frame().await
    }
}
