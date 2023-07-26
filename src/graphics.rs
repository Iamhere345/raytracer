use std::ops::{Add, Sub, Mul, Div};

pub const CANVAS_WIDTH: i32 = 600;
pub const CANVAS_HEIGHT: i32 = 600;

pub const SCREEN_WIDTH: usize = CANVAS_WIDTH as usize;
pub const SCREEN_HEIGHT: usize = CANVAS_HEIGHT as usize;

pub type PixelColour = [u8; 3];
pub type PixelBuf = Vec<PixelColour>;

#[derive(Clone, Copy, Debug)]
pub struct CanvasColour {
    r: u8,
    g: u8,
    b: u8
}

impl CanvasColour {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            r: r,
            g: g,
            b: b
        }
    }

    const fn as_pixels(self) -> PixelColour {
        [self.r, self.g, self.b]
    }

    pub const WHITE: CanvasColour = CanvasColour::new(255, 255, 255);
}

impl Mul<f32> for CanvasColour {

    type Output = Self;

    fn mul(self, mul: f32) -> Self::Output {
        Self {
            r: (self.r as f32 * mul) as u8,
            g: (self.g as f32 * mul) as u8,
            b: (self.b as f32 * mul) as u8,
        }
    }

}

impl From<(u8, u8, u8)> for CanvasColour {
    fn from(colour: (u8, u8, u8)) -> Self {
        Self::new(colour.0, colour.1, colour.2)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point<T> {
    x: T,
    y: T,
    z: T
}

impl<T: Mul<Output = T> + Add<Output = T> + Div<Output = T> + Copy> Point<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {
            x: x,
            y: y,
            z: z
        }
    }

    pub fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn mul_scalar(self, mul: &T) -> Self {
        Point {
            // i hate this
            x: self.x * *mul,
            y: self.y * *mul,
            z: self.z * *mul
        }
    }

}

impl Point<f32> {
    pub fn len(self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
}

impl<T: Div<Output = T> + Copy> Div<T> for Point<T> {

    type Output = Self;

    fn div(self, other: T) -> Self::Output {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other
        }
    }

}

// TODO: impl Sub<T>, Add<T>, Mul<T> and Div<T> for Point<T>

impl<T: Sub<Output = T>> Sub for Point<T> {

    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: Add<Output = T>> Add for Point<T> {

    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3<T> {
    x: T,
    y: T,
    z: T
}

impl<T: Mul<Output = T> + Add<Output = T>> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self {
            x: x,
            y: y,
            z: z
        }
    }

    pub fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl<T: Sub<Output = T>> Sub for Vec3<T> {

    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl<T: Add<Output = T>> Add for Vec3<T> {

    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

pub fn put_pixel(framebuffer: &mut PixelBuf, x: i32, y: i32, colour: CanvasColour) {

    //println!("draw at ({x}, {y})");

    let screen_x: i32 = ((CANVAS_WIDTH / 2) - x) as i32;
    let screen_y: i32 = ((CANVAS_HEIGHT / 2) - y) as i32;

    //println!("draw at ({screen_x}, {screen_y}) with {:?}", colour);

    //framebuffer[1919 + 1089 * SCREEN_WIDTH] = colour.to_pixel_colour();

    framebuffer[((screen_x - 1) + (screen_y - 1) * SCREEN_WIDTH as i32) as usize] = colour.as_pixels();

    //println!("draw colour {:?}", colour);

    //canvas.set_draw_color(colour);
    //canvas.draw_point((screen_x, screen_y)).unwrap();

}