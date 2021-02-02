use crate::TAU;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Colour {
    red: f64,
    green: f64,
    blue: f64,
}

impl Colour {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Colour { red, green, blue }
    }

    pub fn black() -> Self {
        Colour::new(0.0, 0.0, 0.0)
    }

    pub fn white() -> Self {
        Colour::new(1.0, 1.0, 1.0)
    }

    pub fn red_u8(&self) -> u8 {
        (clamp(self.red, 0.0, 1.0) * 255.0).trunc() as u8
    }

    pub fn green_u8(&self) -> u8 {
        (clamp(self.green, 0.0, 1.0) * 255.0).trunc() as u8
    }

    pub fn blue_u8(&self) -> u8 {
        (clamp(self.blue, 0.0, 1.0) * 255.0).trunc() as u8
    }

    pub fn hsv_to_rgb(hue: f64, lightness: f64, saturation: f64) -> Colour {
        Colour::new(
            hsv_to_clr(0.0, hue, lightness, saturation),
            hsv_to_clr(8.0, hue, lightness, saturation),
            hsv_to_clr(4.0, hue, lightness, saturation),
        )
    }
}

#[inline]
fn clamp(test: f64, min: f64, max: f64) -> f64 {
    assert!(min <= max);
    let mut x = test;
    if x < min {
        x = min;
    }
    if x > max {
        x = max;
    }
    x
}

#[inline]
fn arr_min(arr: &[f64]) -> f64 {
    let init = arr[0];
    arr.into_iter()
        .fold(init, |res, val| if res < *val { res } else { *val })
}

#[inline]
fn arr_max(arr: &[f64]) -> f64 {
    let init = arr[0];
    arr.into_iter()
        .fold(init, |res, val| if res > *val { res } else { *val })
}

fn hsv_to_clr(n: f64, hue: f64, lightness: f64, saturation: f64) -> f64 {
    let k = (n + (hue / (TAU / 12.0))) % 12.0;

    lightness
        - (saturation
            * arr_min(&[lightness, 1.0 - lightness])
            * arr_max(&[arr_min(&[1.0, k - 3.0, 9.0 - k]), -1.0]))
}

impl std::ops::Add for Colour {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Colour::new(
            self.red + other.red,
            self.green + other.green,
            self.blue + other.blue,
        )
    }
}

impl std::ops::Sub for Colour {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Colour::new(
            self.red - other.red,
            self.green - other.green,
            self.blue - other.blue,
        )
    }
}

impl std::ops::Mul<f64> for Colour {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Colour::new(self.red * scalar, self.green * scalar, self.blue * scalar)
    }
}

impl std::ops::Mul for Colour {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Colour::new(
            self.red * other.red,
            self.green * other.green,
            self.blue * other.blue,
        )
    }
}
