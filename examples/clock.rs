use raytracer::canvas::Canvas;
use raytracer::colour::Colour;
use raytracer::descartes::{Point3D, Vector3D};
use raytracer::matrix::M4;
use raytracer::transformations::{Rotation, Scaling, Shear, Translation};

fn main() {
    let width = 640;
    let height = 480;
    let mut canvas = Canvas::new(width, height);
    let blue = Colour::new(0.0, 0.0, 1.0);
    let red = Colour::new(1.0, 0.0, 0.0);
    let origin = Point3D::new(width as f64 / 2.0, height as f64 / 2.0, 0.0);
    canvas.write_pixel(
        origin.x().trunc() as usize,
        origin.y().trunc() as usize,
        red,
    );

    let translation = Translation::new(width as f64 / 2.0, height as f64 / 2.0, 0.0);

    for i in 0..12 {
        let point = Point3D::new(0.0, 200.0, 0.0);
        let rotation = Rotation::z(((2.0 * std::f64::consts::PI) / 12.0) * i as f64);
        let num = translation * (rotation * point);
        write_blot(num, &mut canvas, blue)
    }

    canvas.to_ppm("clock.ppm");
}

fn write_blot(num: Point3D<f64>, canvas: &mut Canvas, colour: Colour) {
    let x = num.x().trunc() as usize;
    let y = num.y().trunc() as usize;
    for i in x - 2..x + 3 {
        for j in y - 2..y + 3 {
            canvas.write_pixel(i, j, colour);
        }
    }
}
