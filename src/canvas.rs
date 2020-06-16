use crate::colour::Colour;

pub struct Canvas {
    width: usize,
    pixels: Vec<Colour>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            pixels: vec![Colour::black(); width * height],
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, colour: Colour) {
        self.pixels[(y * self.width) + x] = colour
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Colour {
        self.pixels[(y * self.width) + x]
    }

    fn u8_pixels(&self) -> Vec<u8> {
        let mut v = Vec::with_capacity(self.pixels.len());
        for pixel in &self.pixels {
            v.push(pixel.red_u8());
            v.push(pixel.green_u8());
            v.push(pixel.blue_u8());
        }
        v
    }

    pub fn u8_with_alpha(&self, alpha: u8) -> Vec<u8> {
        let mut v = Vec::with_capacity((self.pixels.len() / 3) * 4);
        for pixel in &self.pixels {
            v.push(pixel.red_u8());
            v.push(pixel.green_u8());
            v.push(pixel.blue_u8());
            v.push(alpha);
        }
        v
    }

    pub fn to_ppm(self, filename: &str) -> () {
        // TODO: in theory this should be capped at 70 characters per
        // line, including spaces. Most modern implementations don't
        // care, so ignoring this for now.
        use std::fs::File;
        use std::io::prelude::*;

        // This is stupidly inefficient; better done as bytes?
        let pixels = self
            .u8_pixels()
            .chunks(self.width)
            .map(|line| {
                line.iter()
                    .map(|px| px.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            })
            .collect::<Vec<String>>()
            .join("\n");

        // The header for this file type consists of an identifier, the width/height data,
        // and then space-separated integers for red, green and blue channels.
        let content = format!(
            "P3\n{} {}\n255\n{}\n",
            self.width,
            self.pixels.len() / self.width,
            pixels
        );

        let mut output_file = File::create(filename).unwrap();
        output_file.write_all(&content.as_bytes()).unwrap();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn read_writeable() {
        let mut c = Canvas::new(4, 6);
        let black = Colour::black();
        let white = Colour::new(1.0, 1.0, 1.0);
        assert_eq!(c.get_pixel(3, 5), black);
        c.write_pixel(3, 5, white);
        assert_eq!(c.get_pixel(3, 5), white)
    }
}
