
use macroquad::prelude::*;

use macroquad::ui::{hash, root_ui, widgets, Skin};


pub fn get_skins() -> Vec<Skin> {
    let skin1 = {
        let label_style = root_ui()
            .style_builder()
            .font(include_bytes!("ui/HTOWERT.TTF"))
            .unwrap()
            .text_color(Color::from_rgba(180, 180, 120, 255))
            .font_size(30)
            .build();

        let window_style = root_ui()
            .style_builder()
            .background(
                Image::from_file_with_format(
                    include_bytes!("ui/window_background.png"),
                    None,
                )
                .unwrap(),
            )
            .background_margin(RectOffset::new(20.0, 20.0, 10.0, 20.0))
            .margin(RectOffset::new(-20.0, -30.0, 0.0, 0.0))
            .build();

        let button_style = root_ui()
            .style_builder()
            .background(
                Image::from_file_with_format(
                    include_bytes!("ui/button_background.png"),
                    None,
                )
                .unwrap(),
            )
            .background_margin(RectOffset::new(37.0, 37.0, 5.0, 5.0))
            .margin(RectOffset::new(10.0, 10.0, 0.0, 0.0))
            .background_hovered(
                Image::from_file_with_format(
                    include_bytes!("ui/button_hovered_background.png"),
                    None,
                )
                .unwrap(),
            )
            .background_clicked(
                Image::from_file_with_format(
                    include_bytes!("ui/button_clicked_background.png"),
                    None,
                )
                .unwrap(),
            )
            .font(include_bytes!("ui/HTOWERT.TTF"))
            .unwrap()
            .text_color(Color::from_rgba(180, 180, 100, 255))
            .font_size(40)
            .build();

        let editbox_style = root_ui()
            .style_builder()
            .background_margin(RectOffset::new(0., 0., 0., 0.))
            .font(include_bytes!("ui/HTOWERT.TTF"))
            .unwrap()
            .text_color(Color::from_rgba(120, 120, 120, 255))
            .color_selected(Color::from_rgba(190, 190, 190, 255))
            .font_size(50)
            .build();

        Skin {
            editbox_style,
            window_style,
            button_style,
            label_style,
            ..root_ui().default_skin()
        }
    };

    let skin2 = {
        let label_style = root_ui()
            .style_builder()
            .font(include_bytes!("ui/MinimalPixel v2.ttf"))
            .unwrap()
            .text_color(Color::from_rgba(120, 120, 120, 255))
            .font_size(25)
            .build();

        let window_style = root_ui()
            .style_builder()
            .background(
                Image::from_file_with_format(
                    include_bytes!("ui/window_background_2.png"),
                    None,
                )
                .unwrap(),
            )
            .background_margin(RectOffset::new(52.0, 52.0, 52.0, 52.0))
            .margin(RectOffset::new(-30.0, 0.0, -30.0, 20.0))
            .build();

        let button_style = root_ui()
            .style_builder()
            .background(
                Image::from_file_with_format(
                    include_bytes!("ui/button_background_2.png"),
                    None,
                )
                .unwrap(),
            )
            .background_margin(RectOffset::new(8.0, 8.0, 8.0, 8.0))
            .background_hovered(
                Image::from_file_with_format(
                    include_bytes!("ui/button_hovered_background_2.png"),
                    None,
                )
                .unwrap(),
            )
            .background_clicked(
                Image::from_file_with_format(
                    include_bytes!("ui/button_clicked_background_2.png"),
                    None,
                )
                .unwrap(),
            )
            .font(include_bytes!("ui/MinimalPixel v2.ttf"))
            .unwrap()
            .text_color(Color::from_rgba(180, 180, 100, 255))
            .font_size(40)
            .build();

        let checkbox_style = root_ui()
            .style_builder()
            .background(
                Image::from_file_with_format(
                    include_bytes!("ui/checkbox_background.png"),
                    None,
                )
                .unwrap(),
            )
            .background_hovered(
                Image::from_file_with_format(
                    include_bytes!("ui/checkbox_hovered_background.png"),
                    None,
                )
                .unwrap(),
            )
            .background_clicked(
                Image::from_file_with_format(
                    include_bytes!("ui/checkbox_clicked_background.png"),
                    None,
                )
                .unwrap(),
            )
            .build();

        let editbox_style = root_ui()
            .style_builder()
            .background(
                Image::from_file_with_format(
                    include_bytes!("ui/editbox_background.png"),
                    None,
                )
                .unwrap(),
            )
            .background_margin(RectOffset::new(2., 2., 2., 2.))
            .font(include_bytes!("ui/MinimalPixel v2.ttf"))
            .unwrap()
            .text_color(Color::from_rgba(120, 120, 120, 255))
            .font_size(25)
            .build();

        let combobox_style = root_ui()
            .style_builder()
            .background(
                Image::from_file_with_format(
                    include_bytes!("ui/combobox_background.png"),
                    None,
                )
                .unwrap(),
            )
            .background_margin(RectOffset::new(4., 25., 6., 6.))
            .font(include_bytes!("ui/MinimalPixel v2.ttf"))
            .unwrap()
            .text_color(Color::from_rgba(120, 120, 120, 255))
            .color(Color::from_rgba(210, 210, 210, 255))
            .font_size(25)
            .build();


            let scrollbar_style = root_ui().default_skin().scrollbar_style;

        Skin {
            window_style,
            button_style,
            label_style,
            checkbox_style,
            editbox_style,
            combobox_style,
            scrollbar_style,
            ..root_ui().default_skin()
        }
    };
    let default_skin = root_ui().default_skin().clone();

    let mut window1_skin = skin1.clone();
    let mut window2_skin = skin2.clone();

    return vec![window1_skin, window2_skin];

}