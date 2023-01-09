use crate::{Color, Point, Vec3};

trait Texture {
    fn value(&self, u: f64, v: f64, p: Point) -> Color;
}

pub struct Solid {
    pub color: Color,
}

impl Solid {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Solid {
            color: Color::new(r, g, b),
        }
    }
    pub fn from_color(color: Color) -> Self {
        Solid { color }
    }
}

impl Texture for Solid {
    fn value(&self, _u: f64, _v: f64, _p: Point) -> Color {
        self.color
    }
}
