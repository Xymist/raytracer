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

    pub fn red_u8(&self) -> u8 {
        if self.red < 0.0 {
            return 0;
        }

        if self.red > 1.0 {
            return 255;
        }

        (self.red * 255.0).trunc() as u8
    }

    pub fn green_u8(&self) -> u8 {
        if self.green < 0.0 {
            return 0;
        }

        if self.green > 1.0 {
            return 255;
        }

        (self.green * 255.0).trunc() as u8
    }

    pub fn blue_u8(&self) -> u8 {
        if self.blue < 0.0 {
            return 0;
        }

        if self.blue > 1.0 {
            return 255;
        }

        (self.blue * 255.0).trunc() as u8
    }
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
