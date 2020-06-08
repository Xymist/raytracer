use crate::descartes::{Point3D, Vector3D};
use crate::matrix::M4;

#[derive(Debug, Copy, Clone)]
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
        self.0 * rhs
    }
}

impl std::ops::Mul<Vector3D<f64>> for Translation {
    type Output = Vector3D<f64>;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        rhs
    }
}

#[derive(Debug, Copy, Clone)]
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
        self.0 * rhs
    }
}

impl std::ops::Mul<Vector3D<f64>> for Scaling {
    type Output = Vector3D<f64>;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        self.0 * rhs
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Rotation(M4);

impl Rotation {
    pub fn x(r: f64) -> Self {
        Rotation(M4::x_rotation(r))
    }

    pub fn y(r: f64) -> Self {
        Rotation(M4::y_rotation(r))
    }

    pub fn z(r: f64) -> Self {
        Rotation(M4::z_rotation(r))
    }

    pub fn inverse(self) -> Option<Self> {
        self.0.inverse().map(|m| Rotation(m))
    }
}

impl std::ops::Mul<Point3D<f64>> for Rotation {
    type Output = Point3D<f64>;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        self.0 * rhs
    }
}

impl std::ops::Mul<Vector3D<f64>> for Rotation {
    type Output = Vector3D<f64>;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        self.0 * rhs
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Shear(M4);

impl Shear {
    pub fn new(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        Shear(M4::shear(xy, xz, yx, yz, zx, zy))
    }
}

impl std::ops::Mul<Point3D<f64>> for Shear {
    type Output = Point3D<f64>;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        self.0 * rhs
    }
}

impl std::ops::Mul<Vector3D<f64>> for Shear {
    type Output = Vector3D<f64>;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        self.0 * rhs
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::f64::consts::PI;

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

    #[test]
    fn x_rotate() {
        let p = Point3D::new(0.0, 1.0, 0.0);
        let eighth_turn = Rotation::x(PI / 4.0);
        let quarter_turn = Rotation::x(PI / 2.0);

        assert_eq!(
            eighth_turn * p.clone(),
            Point3D::new(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
        );

        assert_eq!(quarter_turn * p.clone(), Point3D::new(0.0, 0.0, 1.0))
    }

    #[test]
    fn x_rotate_inverse() {
        let p = Point3D::new(0.0, 1.0, 0.0);
        let eighth_turn = Rotation::x(PI / 4.0).inverse().unwrap();

        assert_eq!(
            eighth_turn * p.clone(),
            Point3D::new(0.0, 2.0_f64.sqrt() / 2.0, -(2.0_f64.sqrt() / 2.0))
        );
    }

    #[test]
    fn y_rotate() {
        let p = Point3D::new(0.0, 0.0, 1.0);
        let eighth_turn = Rotation::y(PI / 4.0);
        let quarter_turn = Rotation::y(PI / 2.0);

        assert_eq!(
            eighth_turn * p.clone(),
            Point3D::new(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0)
        );

        assert_eq!(quarter_turn * p.clone(), Point3D::new(1.0, 0.0, 0.0))
    }

    #[test]
    fn z_rotate() {
        let p = Point3D::new(0.0, 1.0, 0.0);
        let eighth_turn = Rotation::z(PI / 4.0);
        let quarter_turn = Rotation::z(PI / 2.0);

        assert_eq!(
            eighth_turn * p.clone(),
            Point3D::new(-2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0)
        );

        assert_eq!(quarter_turn * p.clone(), Point3D::new(-1.0, 0.0, 0.0))
    }

    #[test]
    fn shear_xy() {
        let shear = Shear::new(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let point = Point3D::new(2.0, 3.0, 4.0);

        assert_eq!(shear * point, Point3D::new(5.0, 3.0, 4.0))
    }

    #[test]
    fn shear_xz() {
        let shear = Shear::new(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let point = Point3D::new(2.0, 3.0, 4.0);

        assert_eq!(shear * point, Point3D::new(6.0, 3.0, 4.0))
    }

    #[test]
    fn shear_yx() {
        let shear = Shear::new(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let point = Point3D::new(2.0, 3.0, 4.0);

        assert_eq!(shear * point, Point3D::new(2.0, 5.0, 4.0))
    }

    #[test]
    fn shear_yz() {
        let shear = Shear::new(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let point = Point3D::new(2.0, 3.0, 4.0);

        assert_eq!(shear * point, Point3D::new(2.0, 7.0, 4.0))
    }

    #[test]
    fn shear_zx() {
        let shear = Shear::new(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let point = Point3D::new(2.0, 3.0, 4.0);

        assert_eq!(shear * point, Point3D::new(2.0, 3.0, 6.0))
    }

    #[test]
    fn shear_zy() {
        let shear = Shear::new(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let point = Point3D::new(2.0, 3.0, 4.0);

        assert_eq!(shear * point, Point3D::new(2.0, 3.0, 7.0))
    }
}
