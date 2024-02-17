use egui_macroquad::{egui::{self, epaint::Shadow, style::{WidgetVisuals, Widgets}, Color32, FontFamily, FontId, Rounding, Stroke, Style, TextStyle, Visuals}, macroquad::color::BLACK};
use crate::egui_style::TextStyle::*;
use crate::egui_style::FontFamily::Proportional;

const STROKE_SIZE: f32 = 3.0;
const EXPANSION:f32 =  0.0;

pub fn robot_style() -> Style {
    
    let outline: Stroke = Stroke{width: STROKE_SIZE, color:  Color32::from_rgba_unmultiplied(215, 215, 255, 0)};
    // let text: Stroke = Stroke{width: , color:  Color32::from_rgba_unmultiplied(215, 215, 255, 0)};
    let background_color: Color32 = Color32::from_rgba_unmultiplied(255, 255, 255, 0);
    let background_color_button: Color32 = Color32::from_rgba_unmultiplied(255, 255, 255, 0);
    let background_color_focus: Color32 = Color32::from_rgba_unmultiplied(255, 255, 255, 0);
    let background_color_click: Color32 = Color32::from_rgba_unmultiplied(255, 255, 255, 0);


    Style { 
        text_styles: [
            (Heading, FontId::new(30.0, Proportional)),
            (Name("Heading2".into()), FontId::new(25.0, Proportional)),
            (Name("Context".into()), FontId::new(23.0, Proportional)),
            (Body, FontId::new(18.0, Proportional)),
            (Monospace, FontId::new(14.0, Proportional)),
            (Button, FontId::new(20.0, Proportional)),
            (Small, FontId::new(10.0, Proportional)),
        ].into(),
        
        
        visuals: Visuals {
        
            popup_shadow: Shadow::small_light(),
            window_shadow: Shadow::NONE,
            dark_mode: false,
            // override_text_color: Some(Color32::WHITE),
            
            window_stroke: outline,
            window_fill: background_color,
            widgets: Widgets {
                noninteractive: WidgetVisuals {
                    bg_fill: background_color,
                    weak_bg_fill: background_color_button,
                    bg_stroke: outline,
                    rounding: Rounding::same(STROKE_SIZE),
                    fg_stroke: Stroke { width: 3.0, color: Color32::WHITE },
                    expansion: EXPANSION,
                },
                inactive: WidgetVisuals {
                    bg_fill: Color32::from_rgba_unmultiplied(255, 255, 255, 255),
                    weak_bg_fill: background_color_button,
                    bg_stroke: outline,
                    rounding: Rounding::same(STROKE_SIZE),
                    fg_stroke:Stroke { width: 3.0, color: Color32::WHITE },
                    expansion: EXPANSION,
                },
                hovered: WidgetVisuals {
                    bg_fill: background_color_focus,
                    weak_bg_fill: background_color_focus,
                    bg_stroke: outline,
                    rounding: Rounding::same(STROKE_SIZE),
                    fg_stroke: Stroke::new(1.0, Color32::from_rgb(100, 200, 100)),
                    expansion: EXPANSION,
                },
                active: WidgetVisuals {
                    bg_fill: background_color_click,
                    weak_bg_fill: background_color_click,
                    bg_stroke: outline,
                    rounding: Rounding::same(STROKE_SIZE),
                    fg_stroke:Stroke { width: 3.0, color: Color32::WHITE },
                    expansion: EXPANSION,
                },
                open: WidgetVisuals {
                    bg_fill: Color32::from_rgba_unmultiplied(255, 255, 255, 255),
                    weak_bg_fill: Color32::from_rgba_unmultiplied(215, 215, 255, 225),
                    bg_stroke: outline,
                    rounding: Rounding::same(STROKE_SIZE),
                    fg_stroke:Stroke { width: 3.0, color: Color32::WHITE },
                    expansion: EXPANSION,
                },
            },
            ..Default::default()
        },
        ..Default::default()
     }
}

