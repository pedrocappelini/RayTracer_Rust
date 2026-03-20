//color.rs

pub struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
}

pub fn write_color(pixel_color: Color) {
    let rbyte = (255.999 * pixel_color.r) as i32;
    let gbyte = (255.999 * pixel_color.g) as i32;
    let bbyte = (255.999 * pixel_color.b) as i32;
    println!("{rbyte} {gbyte} {bbyte}")
}
