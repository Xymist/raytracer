#[derive(Debug, Copy, Clone)]
pub struct Point3D<T> {
    x: T,
    y: T,
    z: T,
}

impl PartialEq for Point3D<f64> {
    fn eq(&self, other: &Point3D<f64>) -> bool {
        let lx = (self.x * 100000.0).round();
        let rx = (other.x * 100000.0).round();
        let ly = (self.y * 100000.0).round();
        let ry = (other.y * 100000.0).round();
        let lz = (self.z * 100000.0).round();
        let rz = (other.z * 100000.0).round();
        if lx != rx || ly != ry || lz != rz {
            return false;
        }
        true
    }
}

impl<T: Clone> Point3D<T> {
    pub fn x(&self) -> T {
        self.x.clone()
    }

    pub fn y(&self) -> T {
        self.y.clone()
    }

    pub fn z(&self) -> T {
        self.z.clone()
    }
}

impl<T> Point3D<T> {
    pub fn new(x: T, y: T, z: T) -> Point3D<T> {
        Point3D { x, y, z }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Vector3D<T> {
    x: T,
    y: T,
    z: T,
}

impl PartialEq for Vector3D<f64> {
    fn eq(&self, other: &Vector3D<f64>) -> bool {
        let lx = (self.x * 100000.0).round();
        let rx = (other.x * 100000.0).round();
        let ly = (self.y * 100000.0).round();
        let ry = (other.y * 100000.0).round();
        let lz = (self.z * 100000.0).round();
        let rz = (other.z * 100000.0).round();
        if lx != rx || ly != ry || lz != rz {
            return false;
        }
        true
    }
}

impl<T: Clone> Vector3D<T> {
    pub fn x(&self) -> T {
        self.x.clone()
    }

    pub fn y(&self) -> T {
        self.y.clone()
    }

    pub fn z(&self) -> T {
        self.z.clone()
    }
}

impl<T> Vector3D<T> {
    pub fn new(x: T, y: T, z: T) -> Vector3D<T> {
        Vector3D { x, y, z }
    }
}

impl<T: Into<f64> + Copy> Vector3D<T> {
    pub fn mag(&self) -> f64 {
        (self.x.into().powi(2) + self.y.into().powi(2) + self.z.into().powi(2)).sqrt()
    }

    pub fn normalize(self) -> Vector3D<f64> {
        let mag = self.mag();

        Vector3D::new(
            self.x.into() / mag,
            self.y.into() / mag,
            self.z.into() / mag,
        )
    }
}

impl Vector3D<i8> {
    pub fn flt(self) -> Vector3D<f64> {
        Vector3D::new(self.x as f64, self.y as f64, self.z as f64)
    }
}

impl Vector3D<i16> {
    pub fn flt(self) -> Vector3D<f64> {
        Vector3D::new(self.x as f64, self.y as f64, self.z as f64)
    }
}

impl Vector3D<i32> {
    pub fn flt(self) -> Vector3D<f64> {
        Vector3D::new(self.x as f64, self.y as f64, self.z as f64)
    }
}

impl Vector3D<f64> {
    pub fn dot(self, other: Vector3D<f64>) -> f64 {
        let x = self.x * other.x;
        let y = self.y * other.y;
        let z = self.z * other.z;

        x + y + z
    }

    pub fn cross(self, other: Self) -> Self {
        let x = (self.y * other.z) - (self.z * other.y);
        let y = (self.z * other.x) - (self.x * other.z);
        let z = (self.x * other.y) - (self.y * other.x);

        Vector3D::new(x, y, z)
    }
}

impl<T: std::ops::Sub<Output = T>> std::ops::Sub<Point3D<T>> for Point3D<T> {
    type Output = Vector3D<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector3D::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: std::ops::Sub<Output = T>> std::ops::Sub<Vector3D<T>> for Point3D<T> {
    type Output = Self;

    fn sub(self, rhs: Vector3D<T>) -> Self::Output {
        Point3D::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: std::ops::Sub<Output = T>> std::ops::Sub<Vector3D<T>> for Vector3D<T> {
    type Output = Self;

    fn sub(self, rhs: Vector3D<T>) -> Self::Output {
        Vector3D::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl<T: std::ops::Neg<Output = T>> std::ops::Neg for Vector3D<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vector3D::new(-self.x, -self.y, -self.z)
    }
}

impl<T: std::ops::Add<Output = T>> std::ops::Add<Vector3D<T>> for Point3D<T> {
    type Output = Self;

    fn add(self, other: Vector3D<T>) -> Self::Output {
        Point3D::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<T: std::ops::Add<Output = T>> std::ops::Add<Point3D<T>> for Vector3D<T> {
    type Output = Point3D<T>;

    fn add(self, other: Point3D<T>) -> Self::Output {
        Point3D::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<T: std::ops::Add<Output = T>> std::ops::Add<Vector3D<T>> for Vector3D<T> {
    type Output = Self;

    fn add(self, other: Vector3D<T>) -> Self::Output {
        Vector3D::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl<T: std::ops::Mul<Output = T> + Copy> std::ops::Mul<T> for Vector3D<T> {
    type Output = Self;

    fn mul(self, scalar: T) -> Self::Output {
        Vector3D::new(self.x * scalar, self.y * scalar, self.z * scalar)
    }
}

impl<T: std::ops::Div<Output = T> + Copy> std::ops::Div<T> for Vector3D<T> {
    type Output = Self;

    fn div(self, scalar: T) -> Self::Output {
        Vector3D::new(self.x / scalar, self.y / scalar, self.z / scalar)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_unit_norm() {
        let v = Vector3D::new(1, 2, 3);
        let norm = v.normalize();

        assert_eq!(norm.mag(), 1.0)
    }

    #[test]
    fn check_dot_product() {
        let v1: Vector3D<i8> = Vector3D::new(1, 2, 3);
        let v2 = Vector3D::new(2.0, 3.0, 4.0);
        let dot = v1.flt().dot(v2);

        assert_eq!(dot, 20.0)
    }

    #[test]
    fn check_cross_product() {
        let v1: Vector3D<i8> = Vector3D::new(1, 2, 3);
        let v2 = Vector3D::new(2.0, 3.0, 4.0);

        assert_eq!(v1.flt().cross(v2), Vector3D::new(-1.0, 2.0, -1.0));
        assert_eq!(v2.cross(v1.flt()), Vector3D::new(1.0, -2.0, 1.0));
    }
}
