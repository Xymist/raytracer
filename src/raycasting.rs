use crate::descartes::{Point3D, Vector3D};
use typed_arena::Arena;

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
pub struct Sphere {
    id: usize,
}

impl Sphere {
    pub fn new(id: usize) -> Self {
        Self { id }
    }

    pub fn id(&self) -> usize {
        self.id
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Intersection {
    t: f64,
    object: usize,
}

impl Intersection {
    pub fn new(t: f64, object: usize) -> Self {
        Self { t, object }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Interaction {
    Collision(Vec<Intersection>),
    Miss,
}

impl Interaction {
    pub fn new(sphere: &mut Sphere, ray: Ray) -> Self {
        let ro = ray.origin - Point3D::new(0.0, 0.0, 0.0);
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(ro);
        let c = ro.dot(ro) - 1.0;
        let d = b.powi(2) - (4.0 * a * c);

        if d < 0.0 {
            return Interaction::Miss;
        }

        Interaction::Collision(vec![
            Intersection::new((-b - d.sqrt()) / (2.0 * a), sphere.id()),
            Intersection::new((-b + d.sqrt()) / (2.0 * a), sphere.id()),
        ])
    }
}

pub struct Scene {
    objects: Arena<Sphere>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            objects: Arena::new(),
        }
    }

    pub fn sphere(&self) -> &mut Sphere {
        let new_id = self.objects.len();
        let s = self.objects.alloc(Sphere::new(new_id));
        s
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
    fn equatorial_interaction() {
        let scene = Scene::new();
        let p = Point3D::new(0.0, 0.0, -5.0);
        let v = Vector3D::new(0.0, 0.0, 1.0);
        let r = Ray::new(p, v);
        let s = scene.sphere();
        let i = Interaction::new(s, r);

        assert_eq!(
            i,
            Interaction::Collision(vec![
                Intersection::new(4.0, s.id()),
                Intersection::new(6.0, s.id())
            ])
        )
    }

    #[test]
    fn tangential_interaction() {
        let scene = Scene::new();
        let p = Point3D::new(0.0, 1.0, -5.0);
        let v = Vector3D::new(0.0, 0.0, 1.0);
        let r = Ray::new(p, v);
        let s = scene.sphere();
        let i = Interaction::new(s, r);

        assert_eq!(
            i,
            Interaction::Collision(vec![
                Intersection::new(5.0, s.id()),
                Intersection::new(5.0, s.id())
            ])
        )
    }

    #[test]
    fn missed_interaction() {
        let scene = Scene::new();
        let p = Point3D::new(0.0, 2.0, -5.0);
        let v = Vector3D::new(0.0, 0.0, 1.0);
        let r = Ray::new(p, v);
        let s = scene.sphere();
        let i = Interaction::new(s, r);

        assert_eq!(i, Interaction::Miss)
    }

    #[test]
    fn internal_origin() {
        let scene = Scene::new();
        let p = Point3D::new(0.0, 0.0, 0.0);
        let v = Vector3D::new(0.0, 0.0, 1.0);
        let r = Ray::new(p, v);
        let s = scene.sphere();
        let i = Interaction::new(s, r);

        assert_eq!(
            i,
            Interaction::Collision(vec![
                Intersection::new(-1.0, s.id()),
                Intersection::new(1.0, s.id())
            ])
        )
    }

    #[test]
    fn post_origin() {
        let scene = Scene::new();
        let p = Point3D::new(0.0, 0.0, 5.0);
        let v = Vector3D::new(0.0, 0.0, 1.0);
        let r = Ray::new(p, v);
        let s = scene.sphere();
        let i = Interaction::new(s, r);

        assert_eq!(
            i,
            Interaction::Collision(vec![
                Intersection::new(-6.0, s.id()),
                Intersection::new(-4.0, s.id())
            ])
        )
    }

    #[test]
    fn different_spheres() {
        let scene = Scene::new();
        let s1 = scene.sphere();
        let s2 = scene.sphere();
        let s3 = scene.sphere();

        assert_ne!(s1, s2);
        assert_ne!(s2, s3);
        assert_ne!(s1, s3);
        assert_eq!(scene.objects.len(), 3);
    }
}
