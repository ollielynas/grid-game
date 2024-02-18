#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod entity;
mod game_ui;
mod map;
mod player;
mod settings;
mod update;
mod egui_style;
mod physics;

use egui_macroquad::{egui::{self, epaint::text::cursor, FontData, FontDefinitions, FontFamily}, macroquad::{self, miniquad::Pipeline, prelude::*}};
use egui_style::robot_style;
// mod profiling;
mod craft;
use crate::craft::craft;

use game_ui::home;
use savefile::prelude::*;
use settings::Settings;

// use console_error_panic_hook;
use std::{collections::HashSet, panic};

use egui_macroquad::macroquad::{
    miniquad::{BlendFactor, BlendState, BlendValue, Equation},
    prelude::*,
    ui::{root_ui, Skin, Style},
};
use map::{Map, Pixel};
use player::{Item, Player};

/// size of map
const SIZE: usize = 301;

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
    // console_error_panic_hook::set_once();


    let light_material = if cfg!(target_family = "wasm") {
        None
    } else {
        Some(
            load_material(
                include_str!("./shader/vertex.glsl"),
                include_str!("./shader/light_frag.glsl"),
                MaterialParams {
                    pipeline_params: PipelineParams {
                        color_blend: Some(BlendState::new(
                            Equation::Add,
                            BlendFactor::Value(BlendValue::SourceAlpha),
                            BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                        )),
                        ..Default::default()
                    },
                    uniforms: vec![
                        ("textureSize".to_owned(), UniformType::Float2),
                        ("canvasSize".to_owned(), UniformType::Float2),
                    ],
                    ..Default::default()
                },
            )
            .unwrap(),
        )
    };
    let world_material = if cfg!(target_family = "wasm") {
        None
    } else {
        Some(
            load_material(

                    include_str!("./shader/vertex.glsl"),
                    include_str!("./shader/world_frag.glsl"),
                
                MaterialParams {
                    pipeline_params: PipelineParams {
                        color_blend: Some(BlendState::new(
                            Equation::Add,
                            BlendFactor::Value(BlendValue::SourceAlpha),
                            BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                        )),
                        ..Default::default()
                    },
                    uniforms: vec![
                        ("textureSize".to_owned(), UniformType::Float2),
                        ("canvasSize".to_owned(), UniformType::Float2),
                    ],
                    ..Default::default()
                },
            )
            .unwrap(),
        )
    };

    let overlay_material = if cfg!(target_family = "wasm") {
        None
    } else {
        Some(
            load_material(

                    include_str!("./shader/vertex.glsl"),
                    include_str!("./shader/damage_frag.glsl"),

                MaterialParams {
                    pipeline_params: PipelineParams {
                        color_blend: Some(BlendState::new(
                            Equation::Add,
                            BlendFactor::Value(BlendValue::SourceAlpha),
                            BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                        )),
                        ..Default::default()
                    },
                    uniforms: vec![
                        ("ScreenSize".to_owned(), UniformType::Float2),
                        ("Damage".to_owned(), UniformType::Float1),
                    ],
                    ..Default::default()
                },
            )
            .unwrap(),
        )
    };

    let post_process_material = if cfg!(target_family = "wasm") {
        None 
    } else {
        Some(
            load_material(
                include_str!("./shader/vertex.glsl"),
                include_str!("./shader/post_process.glsl"),

                MaterialParams {
                    pipeline_params: PipelineParams {
                        color_blend: Some(BlendState::new(
                            Equation::Add,
                            BlendFactor::Value(BlendValue::SourceAlpha),
                            BlendFactor::OneMinusValue(BlendValue::SourceAlpha),
                        )),
                        ..Default::default()
                    },
                    uniforms: vec![
                        ("ScreenSize".to_owned(), UniformType::Float2)
                    ],
                    ..Default::default()
                },
            )
            .unwrap(),
        )
    };

    let (mut map, mut player) = home().await;

    let mut texture: Texture2D = Texture2D::from_image(&map.image);
    let mut light_texture: Texture2D = Texture2D::from_image(&map.light_mask);

    let settings = Settings::default();

    texture.set_filter(FilterMode::Nearest);

    //light_texture.set_filter(FilterMode::Nearest);

    // map.make_square(map::Pixel::Water);
    // map.make_log();

    let paused = false;
    show_mouse(false);

    // let texture_heatmap: Texture2D = Texture2D::from_image(&map.heatmap);

    let mut hover: Option<Pixel>;

    let mut average_damage = vec![0.0, 0.0, 0.0];

    // root_ui().push_skin(&skin);
    // root_ui().pop_skin();

    egui_macroquad::ui(|egui_ctx: &egui::Context| {
        egui_ctx.set_style(robot_style());
        let mut fonts = FontDefinitions::default();

        // Install my own font (maybe supporting non-latin characters):
        fonts.font_data.insert("my_font".to_owned(),
        FontData::from_static(include_bytes!("./font/FSEX300.ttf"))); // .ttf and .otf supported

        // Put my font first (highest priority):
        fonts.families.get_mut(&FontFamily::Proportional).unwrap()
            .insert(0, "my_font".to_owned());

        // Put my font as last fallback for monospace:
        fonts.families.get_mut(&FontFamily::Monospace).unwrap()
            .push("my_font".to_owned());

        egui_ctx.set_fonts(fonts);
    });

    let mut curr_window_size = (screen_width() as u32, screen_height() as u32);
    let mut rt = render_target(curr_window_size.0, curr_window_size.1);

    loop {
        let new_window_size = (screen_width() as u32, screen_height() as u32);
        if new_window_size != curr_window_size {
            curr_window_size = new_window_size;
            rt = render_target(curr_window_size.0, curr_window_size.1);
        }
        // Draw things before egui


        let delta = get_frame_time();

        let mut player_damage_taken = player.health;
        player.update(&map, &settings);
        player_damage_taken -= player.health;

        player_damage_taken /= delta;

        average_damage.pop();
        average_damage.insert(0, player_damage_taken);

        let mut cam = player.cam();
        cam.render_target = Some(rt);
        set_camera(&cam);

        // clear_background(Color { r: 0.8, g: 0.8, b: 0.8, a: 1.0 });
        clear_background(WHITE);

    
        if !paused {
            map.update_state(&player);
            map.entities.retain_mut(|x| x.update(&(map.grid)));
        }

        light_texture.update(&map.light_mask);

        match get_char_pressed() {
            Some('i') => {
                player.inventory.open = !player.inventory.open;
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

        if (((is_mouse_button_pressed(MouseButton::Left)
            || is_mouse_button_released(MouseButton::Left))
            && matches!(player.item_in_hand, Item::Crafter { start }))
            || (is_mouse_button_down(MouseButton::Left)
                && !matches!(player.item_in_hand, Item::Crafter { start })))
            && distance < 25.0
            && !player.hover_ui
        {
            map.update_texture_px.insert((mouse_row, mouse_col));
            player.use_item(&mut map, mouse_row, mouse_col);
        }

        if !map.update_texture_px.is_empty() {
            map.update_image();
            texture.update(&map.image);
            map.update_texture_px = HashSet::new();
        }

        draw_rectangle(player.x, player.y, 2.0, 3.0, ORANGE);

        for e in &map.entities {
            draw_texture_ex(
                e.texture,
                e.x,
                e.y - e.height + 1.0,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::new(e.width, e.height)),
                    ..Default::default()
                },
            );
        }
        if let Some(ref world_material) = world_material {
            gl_use_material(*world_material);
            world_material.set_uniform("textureSize", (map.size as f32, map.size as f32));
        }
        draw_texture_ex(
            texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                ..Default::default()
            },
        );
        if let Some(ref light_material) = light_material {
            if cfg!(not(target_family = "wasm")) {
                gl_use_material(*light_material);
                light_material.set_uniform("textureSize", (map.size as f32, map.size as f32));
            }
        }
        draw_texture_ex(
            light_texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                ..Default::default()
            },
        );

        let v_port = player.view_port_cache;

        if let Some(ref overlay_material) = overlay_material {
            if cfg!(not(target_family = "wasm")) {
                gl_use_material(*overlay_material);
                overlay_material.set_uniform("ScreenSize", (screen_width(), screen_height()));
                overlay_material.set_uniform(
                    "Damage",
                    average_damage.iter().sum::<f32>() / average_damage.len() as f32,
                );
                draw_rectangle(v_port.x, v_port.y, v_port.w, v_port.h, WHITE);
            }
        }

        

        gl_use_default_material();

        //let hit = player.make_map_box(&map, player.view_port_cache, false);
        //let hit = player.make_map_box(&map, Rect::new(player.x - 20.0, player.y - 20.0, 40.0, 40.0), true);
        let hit = physics::make_map_box(&map.grid, player.view_port_cache, false, 0.0, 0.0);
        hit.render();

        if player.render_ui() {
            (map, player) = home().await;

            texture = Texture2D::from_image(&map.image);
            light_texture = Texture2D::from_image(&map.light_mask);
            texture.set_filter(FilterMode::Nearest);
            continue;
        };

        player.get_player_box(0.0, 0.0).render();

        // crafting

        let wand_rect = player
            .craft_rect(map.size.clone() as usize)
            .unwrap_or_default();
        let craft_result = craft(map.get_region(wand_rect));

        if let Some(wand_rect) = player.craft_rect(map.size as usize) {
            draw_rectangle_lines(
                wand_rect.x,
                wand_rect.y,
                wand_rect.w,
                wand_rect.h,
                0.8,
                Color { r: 0.6, g: 0.7, b: 1.0, a: 0.8 },
            );
            if distance >= 25.0 {
                draw_rectangle_lines(wand_rect.x, wand_rect.y, wand_rect.w, wand_rect.h, 0.3, RED);
            } else if craft_result.0 {
                draw_rectangle_lines(
                    wand_rect.x,
                    wand_rect.y,
                    wand_rect.w,
                    wand_rect.h,
                    0.3,
                    GREEN,
                );
            } else {
                draw_rectangle_lines(
                    wand_rect.x,
                    wand_rect.y,
                    wand_rect.w,
                    wand_rect.h,
                    0.3,
                    WHITE,
                );
            }
        }

        
        // if hover != Some(Pixel::Air) {
        //     root_ui().label(None, &format!("{:?}", hover));
        // }

        if distance >= 25.0 {
            draw_circle(pt.x, pt.y, 0.5, LIGHTGRAY);
            draw_circle_lines(pt.x, pt.y, 0.4, 0.3, BLACK);
        } else {
            match player.item_in_hand {
                Item::Pickaxe if hover != Some(Pixel::Air) => {
                    draw_rectangle_lines(pt.x.floor(), pt.y.floor(), 1.0, 1.0, 0.5, RED);
                }
                Item::PlacePixel { pixel: _, count } => {
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

        set_default_camera();
        if let Some(mat) = post_process_material {
            gl_use_material(mat);
            mat.set_uniform("ScreenSize", (screen_width(), screen_height()))
        }
        draw_texture_ex(rt.texture, 0.0, 0.0, WHITE, DrawTextureParams {
            dest_size: Some(vec2(screen_width(), screen_height())),
            ..Default::default()
        });
        gl_use_default_material();

        // get_internal_gl().quad_context.apply_pipeline(&Pipeline::new(ctx, buffer_layout, attributes, shader));
        // egui::end_frame();

        egui_macroquad::draw();

        next_frame().await
    }
}
