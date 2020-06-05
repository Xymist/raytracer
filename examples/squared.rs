use raytracer::{canvas::Canvas, colour::Colour};

fn main() {
    let width = 640;
    let height = 480;
    let mut canvas = Canvas::new(width, height);
    let red = Colour::new(1.0, 0.0, 0.0);

    for i in 0..width {
        for j in 0..height {
            if i.pow(2) == j {
                canvas.write_pixel(i, j, red)
            }
        }
    }

    canvas.to_ppm();
}
