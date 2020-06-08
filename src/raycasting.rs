use crate::descartes::{Point3D, Vector3D};

#[derive(Debug, Clone, PartialEq)]
pub struct Ray {
    origin: Point3D<f64>,
    direction: Vector3D<f64>,
}

impl Ray {
    pub fn new(origin: Point3D<f64>, direction: Vector3D<f64>) -> Self {
        Ray { origin, direction }
    }

    pub fn position(&self, t: f64) -> Point3D<f64> {
        self.origin + (self.direction * t)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {}

impl Sphere {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Intersection {
    Intersection(Vec<f64>),
    Miss,
}

impl Intersection {
    fn new(sphere: Sphere, ray: Ray) -> Self {
        let ro = ray.origin - Point3D::new(0.0, 0.0, 0.0);
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(ro);
        let c = ro.dot(ro) - 1.0;
        let d = b.powi(2) - (4.0 * a * c);

        if d < 0.0 {
            return Intersection::Miss;
        }

        Intersection::Intersection(vec![
            (-b - d.sqrt()) / (2.0 * a),
            (-b + d.sqrt()) / (2.0 * a),
        ])
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn ray_position() {
        let r = Ray::new(Point3D::new(2.0, 3.0, 4.0), Vector3D::new(1.0, 0.0, 0.0));

        assert_eq!(r.position(0.0), Point3D::new(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), Point3D::new(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), Point3D::new(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), Point3D::new(4.5, 3.0, 4.0));
    }

    #[test]
    fn equatorial_intersection() {
        let p = Point3D::new(0.0, 0.0, -5.0);
        let v = Vector3D::new(0.0, 0.0, 1.0);
        let r = Ray::new(p, v);
        let s = Sphere::new();
        let i = Intersection::new(s, r);

        assert_eq!(i, Intersection::Intersection(vec![4.0, 6.0]))
    }

    #[test]
    fn tangential_intersection() {
        let p = Point3D::new(0.0, 1.0, -5.0);
        let v = Vector3D::new(0.0, 0.0, 1.0);
        let r = Ray::new(p, v);
        let s = Sphere::new();
        let i = Intersection::new(s, r);

        assert_eq!(i, Intersection::Intersection(vec![5.0, 5.0]))
    }

    #[test]
    fn missed_intersection() {
        let p = Point3D::new(0.0, 2.0, -5.0);
        let v = Vector3D::new(0.0, 0.0, 1.0);
        let r = Ray::new(p, v);
        let s = Sphere::new();
        let i = Intersection::new(s, r);

        assert_eq!(i, Intersection::Miss)
    }

    #[test]
    fn internal_origin() {
        let p = Point3D::new(0.0, 0.0, 0.0);
        let v = Vector3D::new(0.0, 0.0, 1.0);
        let r = Ray::new(p, v);
        let s = Sphere::new();
        let i = Intersection::new(s, r);

        assert_eq!(i, Intersection::Intersection(vec![-1.0, 1.0]))
    }

    #[test]
    fn post_origin() {
        let p = Point3D::new(0.0, 0.0, 5.0);
        let v = Vector3D::new(0.0, 0.0, 1.0);
        let r = Ray::new(p, v);
        let s = Sphere::new();
        let i = Intersection::new(s, r);

        assert_eq!(i, Intersection::Intersection(vec![-6.0, -4.0]))
    }
}
