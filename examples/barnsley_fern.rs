use rand::{rngs::ThreadRng, Rng};
use raytracer::canvas::Canvas;
use raytracer::colour::Colour;
use raytracer::descartes::Point3D;

fn main() {
    let width = 4000;
    let height = 4800;
    let mut rng = rand::thread_rng();
    let mut canvas = Canvas::new(width, height);
    let green = Colour::new(0.0, 1.0, 0.0);

    let mut pt: Point3D<f64> = Point3D::new(0.0, 0.0, 0.0);

    for _ in 0..10000000 {
        canvas.write_pixel(
            ((pt.x() * 450.0).trunc() + 2000.0) as usize,
            (height as f64 - ((pt.y() * 450.0).trunc() + 100.0)) as usize,
            green,
        );
        pt = next_point(&mut rng, pt);
    }

    canvas.to_ppm("fern.ppm");
}

fn next_point(rng: &mut ThreadRng, point: Point3D<f64>) -> Point3D<f64> {
    let rd = rng.gen_range(0, 100);
    match rd {
        0 => Point3D::new(0.0, 0.16 * point.y(), 0.0),
        1..=85 => Point3D::new(
            (0.85 * point.x()) + (0.04 * point.y()),
            (-0.04 * point.x()) + (0.85 * point.y()) + 1.6,
            0.0,
        ),
        86..=92 => Point3D::new(
            (0.2 * point.x()) - (0.26 * point.y()),
            (0.23 * point.x()) + (0.22 * point.y()) + 1.6,
            0.0,
        ),
        93..=99 => Point3D::new(
            (-0.15 * point.x()) + (0.28 * point.y()),
            (0.26 * point.x()) + (0.24 * point.y()) + 0.44,
            0.0,
        ),
        _ => unreachable!(),
    }
}
