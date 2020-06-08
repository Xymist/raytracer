use crate::descartes::{Point3D, Vector3D};
use crate::matrix::M4;

#[derive(Debug, Clone)]
pub enum Transformation {
    Translation(Box<Transformation>),
    Rotation(Box<Transformation>),
    Scaling(Box<Transformation>),
    Shear(Box<Transformation>),
    Matrix(M4),
}

impl Transformation {
    pub fn inverse(self) -> Option<Self> {
        match self {
            Transformation::Translation(mtx) => mtx
                .inverse()
                .map(|m| Transformation::Translation(Box::new(m))),
            Transformation::Rotation(mtx) => {
                mtx.inverse().map(|m| Transformation::Rotation(Box::new(m)))
            }
            Transformation::Scaling(mtx) => {
                mtx.inverse().map(|m| Transformation::Scaling(Box::new(m)))
            }
            Transformation::Shear(mtx) => mtx.inverse().map(|m| Transformation::Shear(Box::new(m))),
            Transformation::Matrix(mtx) => mtx.inverse().map(|m| Transformation::Matrix(m)),
        }
    }

    pub fn translation(x: f64, y: f64, z: f64) -> Transformation {
        Transformation::Translation(Box::new(Transformation::Matrix(M4::translation(x, y, z))))
    }

    pub fn rotation_x(r: f64) -> Transformation {
        Transformation::Rotation(Box::new(Transformation::Matrix(M4::x_rotation(r))))
    }

    pub fn rotation_y(r: f64) -> Transformation {
        Transformation::Rotation(Box::new(Transformation::Matrix(M4::y_rotation(r))))
    }

    pub fn rotation_z(r: f64) -> Transformation {
        Transformation::Rotation(Box::new(Transformation::Matrix(M4::z_rotation(r))))
    }

    pub fn scaling(x: f64, y: f64, z: f64) -> Transformation {
        Transformation::Scaling(Box::new(Transformation::Matrix(M4::scaling(x, y, z))))
    }

    pub fn shear(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        Transformation::Shear(Box::new(Transformation::Matrix(M4::shear(
            xy, xz, yx, yz, zx, zy,
        ))))
    }
}

impl std::ops::Mul<Point3D<f64>> for Transformation {
    type Output = Point3D<f64>;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        match self {
            Transformation::Translation(mtx) => *mtx * rhs,
            Transformation::Rotation(mtx) => *mtx * rhs,
            Transformation::Scaling(mtx) => *mtx * rhs,
            Transformation::Shear(mtx) => *mtx * rhs,
            Transformation::Matrix(mtx) => mtx * rhs,
        }
    }
}

impl std::ops::Mul<Vector3D<f64>> for Transformation {
    type Output = Vector3D<f64>;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        match self {
            // Translations do not affect matrices
            Transformation::Translation(_) => rhs,
            Transformation::Rotation(mtx) => *mtx * rhs,
            Transformation::Scaling(mtx) => *mtx * rhs,
            Transformation::Shear(mtx) => *mtx * rhs,
            Transformation::Matrix(mtx) => mtx * rhs,
        }
    }
}

impl std::ops::Mul<Transformation> for Transformation {
    type Output = Transformation;

    fn mul(self, rhs: Self::Output) -> Self::Output {
        use Transformation::*;
        match self {
            // Translations do not affect matrices
            Translation(mtx) => Translation(Box::new(*mtx * rhs)),
            Rotation(mtx) => Rotation(Box::new(*mtx * rhs)),
            Scaling(mtx) => Scaling(Box::new(*mtx * rhs)),
            Shear(mtx) => Shear(Box::new(*mtx * rhs)),
            Matrix(mtx) => match rhs {
                Translation(mtx2) => Translation(Box::new(self * *mtx2)),
                Rotation(mtx2) => Rotation(Box::new(self * *mtx2)),
                Scaling(mtx2) => Scaling(Box::new(self * *mtx2)),
                Shear(mtx2) => Shear(Box::new(self * *mtx2)),
                Matrix(mtx2) => Matrix(mtx * mtx2),
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn translate_point() {
        let t = Transformation::translation(5.0, -3.0, 2.0);
        let p = Point3D::new(-3.0, 4.0, 5.0);

        assert_eq!(t * p, Point3D::new(2.0, 1.0, 7.0))
    }

    #[test]
    fn translate_point_inverse() {
        let t = Transformation::translation(5.0, -3.0, 2.0);
        let p = Point3D::new(-3.0, 4.0, 5.0);

        assert_eq!(t.inverse().unwrap() * p, Point3D::new(-8.0, 7.0, 3.0))
    }

    #[test]
    fn translate_vec() {
        let t = Transformation::translation(5.0, -3.0, 2.0);
        let v = Vector3D::new(-3.0, 4.0, 5.0);

        assert_eq!(t * v, v)
    }

    #[test]
    fn translate_vec_inverse() {
        let t = Transformation::translation(5.0, -3.0, 2.0);
        let v = Vector3D::new(-3.0, 4.0, 5.0);

        assert_eq!(t.inverse().unwrap() * v, v)
    }

    #[test]
    fn scale_point() {
        let scale = Transformation::scaling(2.0, 3.0, 4.0);
        let scaleable = Point3D::new(-4.0, 6.0, 8.0);

        assert_eq!(scale * scaleable, Point3D::new(-8.0, 18.0, 32.0))
    }

    #[test]
    fn scale_vec() {
        let scale = Transformation::scaling(2.0, 3.0, 4.0);
        let scaleable = Vector3D::new(-4.0, 6.0, 8.0);

        assert_eq!(scale * scaleable, Vector3D::new(-8.0, 18.0, 32.0))
    }

    #[test]
    fn scale_point_inverse() {
        let scale = Transformation::scaling(2.0, 3.0, 4.0).inverse().unwrap();
        let scaleable = Point3D::new(-4.0, 6.0, 8.0);

        assert_eq!(scale * scaleable, Point3D::new(-2.0, 2.0, 2.0))
    }

    #[test]
    fn scale_vec_inverse() {
        let scale = Transformation::scaling(2.0, 3.0, 4.0).inverse().unwrap();
        let scaleable = Vector3D::new(-4.0, 6.0, 8.0);

        assert_eq!(scale * scaleable, Vector3D::new(-2.0, 2.0, 2.0))
    }

    #[test]
    fn reflect() {
        let scale = Transformation::scaling(-1.0, 1.0, 1.0);
        let scaleable = Point3D::new(2.0, 3.0, 4.0);

        assert_eq!(scale * scaleable, Point3D::new(-2.0, 3.0, 4.0));
    }

    #[test]
    fn x_rotate() {
        let p = Point3D::new(0.0, 1.0, 0.0);
        let eighth_turn = Transformation::rotation_x(PI / 4.0);
        let quarter_turn = Transformation::rotation_x(PI / 2.0);

        assert_eq!(
            eighth_turn * p.clone(),
            Point3D::new(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
        );

        assert_eq!(quarter_turn * p.clone(), Point3D::new(0.0, 0.0, 1.0))
    }

    #[test]
    fn x_rotate_inverse() {
        let p = Point3D::new(0.0, 1.0, 0.0);
        let eighth_turn = Transformation::rotation_x(PI / 4.0).inverse().unwrap();

        assert_eq!(
            eighth_turn * p.clone(),
            Point3D::new(0.0, 2.0_f64.sqrt() / 2.0, -(2.0_f64.sqrt() / 2.0))
        );
    }

    #[test]
    fn y_rotate() {
        let p = Point3D::new(0.0, 0.0, 1.0);
        let eighth_turn = Transformation::rotation_y(PI / 4.0);
        let quarter_turn = Transformation::rotation_y(PI / 2.0);

        assert_eq!(
            eighth_turn * p.clone(),
            Point3D::new(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0)
        );

        assert_eq!(quarter_turn * p.clone(), Point3D::new(1.0, 0.0, 0.0))
    }

    #[test]
    fn z_rotate() {
        let p = Point3D::new(0.0, 1.0, 0.0);
        let eighth_turn = Transformation::rotation_z(PI / 4.0);
        let quarter_turn = Transformation::rotation_z(PI / 2.0);

        assert_eq!(
            eighth_turn * p.clone(),
            Point3D::new(-2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0)
        );

        assert_eq!(quarter_turn * p.clone(), Point3D::new(-1.0, 0.0, 0.0))
    }

    #[test]
    fn shear_xy() {
        let shear = Transformation::shear(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let point = Point3D::new(2.0, 3.0, 4.0);

        assert_eq!(shear * point, Point3D::new(5.0, 3.0, 4.0))
    }

    #[test]
    fn shear_xz() {
        let shear = Transformation::shear(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let point = Point3D::new(2.0, 3.0, 4.0);

        assert_eq!(shear * point, Point3D::new(6.0, 3.0, 4.0))
    }

    #[test]
    fn shear_yx() {
        let shear = Transformation::shear(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let point = Point3D::new(2.0, 3.0, 4.0);

        assert_eq!(shear * point, Point3D::new(2.0, 5.0, 4.0))
    }

    #[test]
    fn shear_yz() {
        let shear = Transformation::shear(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let point = Point3D::new(2.0, 3.0, 4.0);

        assert_eq!(shear * point, Point3D::new(2.0, 7.0, 4.0))
    }

    #[test]
    fn shear_zx() {
        let shear = Transformation::shear(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let point = Point3D::new(2.0, 3.0, 4.0);

        assert_eq!(shear * point, Point3D::new(2.0, 3.0, 6.0))
    }

    #[test]
    fn shear_zy() {
        let shear = Transformation::shear(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let point = Point3D::new(2.0, 3.0, 4.0);

        assert_eq!(shear * point, Point3D::new(2.0, 3.0, 7.0))
    }

    #[test]
    fn sequential_transformations() {
        let p = Point3D::new(1.0, 0.0, 1.0);
        let a = Transformation::rotation_x(PI / 2.0);
        let b = Transformation::scaling(5.0, 5.0, 5.0);
        let c = Transformation::translation(10.0, 5.0, 7.0);

        let ap = a * p;
        assert_eq!(ap, Point3D::new(1.0, -1.0, 0.0));

        let bap = b * ap;
        assert_eq!(bap, Point3D::new(5.0, -5.0, 0.0));

        let cbap = c * bap;
        assert_eq!(cbap, Point3D::new(15.0, 0.0, 7.0))
    }

    #[test]
    fn chained_transformations() {
        let p = Point3D::new(1.0, 0.0, 1.0);
        let a = Transformation::rotation_x(PI / 2.0);
        let b = Transformation::scaling(5.0, 5.0, 5.0);
        let c = Transformation::translation(10.0, 5.0, 7.0);

        let cba = c * b * a;

        let cbap = cba * p;
        assert_eq!(cbap, Point3D::new(15.0, 0.0, 7.0))
    }
}
