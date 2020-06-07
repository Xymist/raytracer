use crate::descartes::{Point3D, Vector3D};
use crate::matrix::M4;

pub struct Translation(M4);

impl Translation {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Translation(M4::translation(x, y, z))
    }

    pub fn inverse(self) -> Option<Self> {
        self.0.inverse().map(|m| Translation(m))
    }
}

impl std::ops::Mul<Point3D<f64>> for Translation {
    type Output = Point3D<f64>;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        let mtx = self.0 * rhs;
        Point3D::new(mtx.idx(0), mtx.idx(1), mtx.idx(2))
    }
}

impl std::ops::Mul<Vector3D<f64>> for Translation {
    type Output = Vector3D<f64>;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        rhs
    }
}

pub struct Scaling(M4);

impl Scaling {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Scaling(M4::scaling(x, y, z))
    }

    pub fn inverse(self) -> Option<Self> {
        self.0.inverse().map(|m| Scaling(m))
    }
}

impl std::ops::Mul<Point3D<f64>> for Scaling {
    type Output = Point3D<f64>;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        let mtx = self.0 * rhs;
        Point3D::new(mtx.idx(0), mtx.idx(1), mtx.idx(2))
    }
}

impl std::ops::Mul<Vector3D<f64>> for Scaling {
    type Output = Vector3D<f64>;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        let mtx = self.0 * rhs;
        Vector3D::new(mtx.idx(0), mtx.idx(1), mtx.idx(2))
    }
}

impl M4 {
    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        let mut v = Self::identity();
        v.write_idx(0, 3, x);
        v.write_idx(1, 3, y);
        v.write_idx(2, 3, z);
        v
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Self {
        let mut v = Self::identity();
        v.write_idx(0, 0, x);
        v.write_idx(1, 1, y);
        v.write_idx(2, 2, z);
        v
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn translate_point() {
        let t = Translation::new(5.0, -3.0, 2.0);
        let p = Point3D::new(-3.0, 4.0, 5.0);

        assert_eq!(t * p, Point3D::new(2.0, 1.0, 7.0))
    }

    #[test]
    fn translate_point_inverse() {
        let t = Translation::new(5.0, -3.0, 2.0);
        let p = Point3D::new(-3.0, 4.0, 5.0);

        assert_eq!(t.inverse().unwrap() * p, Point3D::new(-8.0, 7.0, 3.0))
    }

    #[test]
    fn translate_vec() {
        let t = Translation::new(5.0, -3.0, 2.0);
        let v = Vector3D::new(-3.0, 4.0, 5.0);

        assert_eq!(t * v, v)
    }

    #[test]
    fn translate_vec_inverse() {
        let t = Translation::new(5.0, -3.0, 2.0);
        let v = Vector3D::new(-3.0, 4.0, 5.0);

        assert_eq!(t.inverse().unwrap() * v, v)
    }

    #[test]
    fn scale_point() {
        let scale = Scaling::new(2.0, 3.0, 4.0);
        let scaleable = Point3D::new(-4.0, 6.0, 8.0);

        assert_eq!(scale * scaleable, Point3D::new(-8.0, 18.0, 32.0))
    }

    #[test]
    fn scale_vec() {
        let scale = Scaling::new(2.0, 3.0, 4.0);
        let scaleable = Vector3D::new(-4.0, 6.0, 8.0);

        assert_eq!(scale * scaleable, Vector3D::new(-8.0, 18.0, 32.0))
    }

    #[test]
    fn scale_point_inverse() {
        let scale = Scaling::new(2.0, 3.0, 4.0).inverse().unwrap();
        let scaleable = Point3D::new(-4.0, 6.0, 8.0);

        assert_eq!(scale * scaleable, Point3D::new(-2.0, 2.0, 2.0))
    }

    #[test]
    fn scale_vec_inverse() {
        let scale = Scaling::new(2.0, 3.0, 4.0).inverse().unwrap();
        let scaleable = Vector3D::new(-4.0, 6.0, 8.0);

        assert_eq!(scale * scaleable, Vector3D::new(-2.0, 2.0, 2.0))
    }

    #[test]
    fn reflect() {
        let scale = Scaling::new(-1.0, 1.0, 1.0);
        let scaleable = Point3D::new(2.0, 3.0, 4.0);

        assert_eq!(scale * scaleable, Point3D::new(-2.0, 3.0, 4.0));
    }
}
