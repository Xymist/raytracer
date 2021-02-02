use rand::{distributions::Uniform, Rng};
use raytracer::canvas::Canvas;
use raytracer::colour::Colour;
use raytracer::descartes::Point3D;
use raytracer::transformations::Transformation;
use std::f64::consts::PI;

fn main() {
    let width = 1200;
    let height = 1200;
    let mut rng = rand::thread_rng();
    let dist = Uniform::new(0, 3);
    let mut canvas = Canvas::new(width, height);

    let centraliser = Transformation::translation(600.0, 600.0, 0.0);
    let flipper = Transformation::rotation_z(PI);
    let center = Point3D::new(0.0, 500.0, 0.0);
    let mut vertices = Vec::new();
    let mut p: Point3D<f64> =
        Point3D::new(rng.gen_range(0.0, 1200.0), rng.gen_range(0.0, 1200.0), 0.0);

    // Vertices
    for i in 0..3 {
        let rotation = flipper.clone() * Transformation::rotation_z(((2.0 * PI) / 3.0) * i as f64);
        let point = centraliser.clone() * (rotation * center.clone());

        vertices.push(point)
    }

    for _ in 0..10000000 {
        let n = rng.sample(dist);
        let vn = vertices[n];

        let colour = Colour::new(
            ((((2.0 * p.x()) / (vertices[0].x() + p.x()))
                * ((2.0 * p.y()) / (vertices[0].y() + p.y())))
                - 1.0)
                .abs(),
            ((((2.0 * p.x()) / (vertices[1].x() + p.x()))
                * ((2.0 * p.y()) / (vertices[1].y() + p.y())))
                - 1.0)
                .abs(),
            ((((2.0 * p.x()) / (vertices[2].x() + p.x()))
                * ((2.0 * p.y()) / (vertices[2].y() + p.y())))
                - 1.0)
                .abs(),
        );

        canvas.write_pixel(p.x().trunc() as usize, p.y().trunc() as usize, colour);
        p = Point3D::new((p.x() + vn.x()) / 2.0, (p.y() + vn.y()) / 2.0, 0.0)
    }

    canvas.to_ppm("sierpinski.ppm")
}
