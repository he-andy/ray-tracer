use crate::{Color, Perlin, Point};

pub trait Texture: Sync {
    fn value(&self, u: f64, v: f64, p: &Point) -> Color;
}

#[derive(Clone)]
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
    fn value(&self, _u: f64, _v: f64, _p: &Point) -> Color {
        self.color
    }
}

#[derive(Clone)]
pub struct Checkered<T: Texture, U: Texture> {
    odd: T,
    even: U,
}

impl<T: Texture, U: Texture> Checkered<T, U> {
    pub fn new(odd: T, even: U) -> Self {
        Checkered { odd, even }
    }
}

impl<T: Texture, U: Texture> Texture for Checkered<T, U> {
    fn value(&self, u: f64, v: f64, p: &Point) -> Color {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

#[derive(Clone)]
pub struct Noisy {
    noise: Perlin,
}

impl Noisy {
    pub fn new() -> Self {
        Self {
            noise: Perlin::new(),
        }
    }
}

impl Texture for Noisy {
    fn value(&self, _u: f64, _v: f64, p: &Point) -> Color {
        Color::new(1.0, 1.0, 1.0) * self.noise.noise(p)
    }
}
