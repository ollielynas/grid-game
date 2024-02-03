

struct Cam {
    x: f32,
    y: f32,
    zoom: f32,
}


impl Default for Cam {
    fn default() -> Self {
        Cam {
            x:0.0,
            y: 0.0,
            zoom: 10.0,
        }
    }
}