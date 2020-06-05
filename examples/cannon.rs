use raytracer::descartes::{Point3D, Vector3D};

const MUZZLE_VELOCITY: f64 = 32.0;

fn main() {
    let mut timer = 0;

    let mut p = Projectile {
        position: Point3D::new(10.0, 10.0, 10.0),
        velocity: Vector3D::new(12.0, 1.4, 5.3).normalize() * MUZZLE_VELOCITY,
    };

    let e = Environment {
        gravity: Vector3D::new(0.0, -0.2, 0.0),
        wind: Vector3D::new(5.1, 0.0, 0.0),
    };

    while p.flying() {
        timer += 1;

        p = tick(p, &e)
    }

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
