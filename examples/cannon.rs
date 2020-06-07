use raytracer::canvas::Canvas;
use raytracer::colour::Colour;
use raytracer::descartes::{Point3D, Vector3D};

const MUZZLE_VELOCITY: f64 = 12.0;

fn main() {
    let width = 640;
    let height = 480;
    let mut canvas = Canvas::new(width, height);
    let red = Colour::new(1.0, 0.0, 0.0);

    let mut timer = 0;

    let mut p = Projectile {
        // Bottom left corner
        position: Point3D::new(0.0, 1.0, 0.0),
        // In the plane, at MUZZLE_VELOCITY units per tick
        velocity: Vector3D::new(1.0, 2.0, 0.0).normalize() * MUZZLE_VELOCITY,
    };

    let e = Environment {
        // Downwards at 0.2 units per tick
        gravity: Vector3D::new(0.0, -0.2, 0.0),
        // From right to left, in the plane, at -0.01 units per tick
        wind: Vector3D::new(-0.01, 0.0, 0.0),
    };

    while p.flying() {
        timer += 1;
        let px = p.position.x().trunc() as usize;
        let py = p.position.y().trunc() as usize;
        if py > height || px > width {
            break;
        }

        canvas.write_pixel(px, height - py, red);

        p = tick(p, &e)
    }

    canvas.to_ppm("cannon.ppm");

    println!("Took {} ticks to land", timer)
}
struct Projectile {
    position: Point3D<f64>,
    velocity: Vector3D<f64>,
}

impl Projectile {
    pub fn flying(&self) -> bool {
        self.position.y() > 0.0
    }
}

struct Environment {
    gravity: Vector3D<f64>,
    wind: Vector3D<f64>,
}

fn tick(proj: Projectile, env: &Environment) -> Projectile {
    Projectile {
        position: proj.position + proj.velocity,
        velocity: proj.velocity + env.gravity + env.wind,
    }
}
