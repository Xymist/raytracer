use crate::descartes::{Point3D, Vector3D};

#[derive(Debug, Copy, Clone)]
pub struct M2([[f64; 2]; 2]);

impl PartialEq for M2 {
    fn eq(&self, other: &M2) -> bool {
        for r in 0..2 {
            for c in 0..2 {
                let lhs = (self.0[r][c] * 100000.0).round();
                let rhs = (other.0[r][c] * 100000.0).round();
                if lhs != rhs {
                    return false;
                }
            }
        }
        true
    }
}

impl M2 {
    pub fn new() -> Self {
        M2([[0.0; 2]; 2])
    }

    pub fn idx(&self, r: usize, c: usize) -> f64 {
        self.0[r][c]
    }

    pub fn write_idx(&mut self, r: usize, c: usize, v: f64) {
        self.0[r][c] = v
    }

    pub fn identity() -> Self {
        M2([[1.0, 0.0], [0.0, 1.0]])
    }

    pub fn transpose(self) -> Self {
        let mut result = Self::new();
        for r in 0..2 {
            for c in 0..2 {
                result.0[c][r] = self.0[r][c]
            }
        }
        result
    }

    pub fn determinant(self) -> f64 {
        (self.idx(0, 0) * self.idx(1, 1)) - (self.idx(0, 1) * self.idx(1, 0))
    }
}

#[derive(Debug, Copy, Clone)]
pub struct M3([[f64; 3]; 3]);

impl PartialEq for M3 {
    fn eq(&self, other: &M3) -> bool {
        for r in 0..3 {
            for c in 0..3 {
                let lhs = (self.0[r][c] * 100000.0).round();
                let rhs = (other.0[r][c] * 100000.0).round();
                if lhs != rhs {
                    return false;
                }
            }
        }
        true
    }
}

impl M3 {
    pub fn new() -> Self {
        M3([[0.0; 3]; 3])
    }

    pub fn idx(&self, r: usize, c: usize) -> f64 {
        self.0[r][c]
    }

    pub fn write_idx(&mut self, r: usize, c: usize, v: f64) {
        self.0[r][c] = v
    }

    pub fn submatrix(&self, r: usize, c: usize) -> M2 {
        let mut result = M2::new();

        for row in 0..3 {
            let nrow;
            // Skip the row we're eliminating
            if row == r {
                continue;
            }
            // Rows greater than the removed one have their index
            // lowered by one.
            if row > r {
                nrow = row - 1
            } else {
                nrow = row
            }
            for col in 0..3 {
                let ncol;
                // Skip the column we're eliminating
                if col == c {
                    continue;
                }
                // Columns greater than the removed one have their
                // index lowered by one.
                if col > c {
                    ncol = col - 1
                } else {
                    ncol = col
                }
                result.write_idx(nrow, ncol, self.idx(row, col))
            }
        }
        result
    }

    pub fn identity() -> Self {
        M3([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
    }

    pub fn minor(&self, r: usize, c: usize) -> f64 {
        self.submatrix(r, c).determinant()
    }

    pub fn cofactor(&self, r: usize, c: usize) -> f64 {
        let m = self.minor(r, c);
        if (r + c) % 2 == 0 {
            return m;
        }
        -m
    }

    pub fn determinant(&self) -> f64 {
        let mut det = 0.0;
        for idx in 0..3 {
            let cf = self.cofactor(0, idx);
            det += cf * self.0[0][idx]
        }
        det
    }

    pub fn transpose(self) -> Self {
        let mut result = Self::new();
        for r in 0..3 {
            for c in 0..3 {
                result.0[c][r] = self.0[r][c]
            }
        }
        result
    }
}

#[derive(Debug, Copy, Clone)]
pub struct M4([[f64; 4]; 4]);

impl PartialEq for M4 {
    fn eq(&self, other: &M4) -> bool {
        for r in 0..4 {
            for c in 0..4 {
                let lhs = (self.0[r][c] * 100000.0).round();
                let rhs = (other.0[r][c] * 100000.0).round();
                if lhs != rhs {
                    return false;
                }
            }
        }
        true
    }
}

impl M4 {
    pub fn new() -> Self {
        M4([[0.0; 4]; 4])
    }

    pub fn idx(&self, r: usize, c: usize) -> f64 {
        self.0[r][c]
    }

    pub fn write_idx(&mut self, r: usize, c: usize, v: f64) {
        self.0[r][c] = v
    }

    pub fn submatrix(&self, r: usize, c: usize) -> M3 {
        let mut result = M3::new();

        for row in 0..4 {
            let nrow;
            // Skip the row we're eliminating
            if row == r {
                continue;
            }
            // Rows greater than the removed one have their index
            // lowered by one.
            if row > r {
                nrow = row - 1
            } else {
                nrow = row
            }
            for col in 0..4 {
                let ncol;
                // Skip the column we're eliminating
                if col == c {
                    continue;
                }
                // Columns greater than the removed one have their
                // index lowered by one.
                if col > c {
                    ncol = col - 1
                } else {
                    ncol = col
                }
                result.write_idx(nrow, ncol, self.idx(row, col))
            }
        }
        result
    }

    pub fn minor(&self, r: usize, c: usize) -> f64 {
        self.submatrix(r, c).determinant()
    }

    pub fn cofactor(&self, r: usize, c: usize) -> f64 {
        let m = self.minor(r, c);
        if (r + c) % 2 == 0 {
            return m;
        }
        -m
    }

    pub fn determinant(&self) -> f64 {
        let mut det = 0.0;
        for idx in 0..4 {
            let cf = self.cofactor(0, idx);
            det += cf * self.0[0][idx]
        }
        det
    }

    pub fn identity() -> Self {
        M4([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn transpose(self) -> Self {
        let mut result = Self::new();
        for r in 0..4 {
            for c in 0..4 {
                result.0[c][r] = self.0[r][c]
            }
        }
        result
    }

    pub fn inverse(&self) -> Option<Self> {
        let det = self.determinant();
        if det == 0.0 {
            return None;
        }
        let mut result = Self::new();

        for r in 0..4 {
            for c in 0..4 {
                result.write_idx(c, r, self.cofactor(r, c) / self.determinant())
            }
        }

        Some(result)
    }

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

    pub fn x_rotation(r: f64) -> Self {
        M4([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, r.cos(), -r.sin(), 0.0],
            [0.0, r.sin(), r.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn y_rotation(r: f64) -> Self {
        M4([
            [r.cos(), 0.0, r.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-r.sin(), 0.0, r.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn z_rotation(r: f64) -> Self {
        M4([
            [r.cos(), -r.sin(), 0.0, 0.0],
            [r.sin(), r.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    pub fn shear(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        M4([
            [1.0, xy, xz, 0.0],
            [yx, 1.0, yz, 0.0],
            [zx, zy, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
}

impl std::ops::Mul<M4> for M4 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut result = M4([[0.0; 4]; 4]);

        for r in 0..4 {
            for c in 0..4 {
                result.write_idx(
                    r,
                    c,
                    (self.0[r][0] * other.0[0][c])
                        + (self.0[r][1] * other.0[1][c])
                        + (self.0[r][2] * other.0[2][c])
                        + (self.0[r][3] * other.0[3][c]),
                )
            }
        }

        result
    }
}

impl std::ops::Mul<M1_4> for M4 {
    type Output = M1_4;

    fn mul(self, other: M1_4) -> M1_4 {
        let mut result = M1_4([[0.0; 1]; 4]);

        for r in 0..4 {
            result.write_idx(
                r,
                (self.0[r][0] * other.0[0][0])
                    + (self.0[r][1] * other.0[1][0])
                    + (self.0[r][2] * other.0[2][0])
                    + (self.0[r][3] * other.0[3][0]),
            )
        }

        result
    }
}

impl std::ops::Mul<Vector3D<f64>> for M4 {
    type Output = M1_4;

    fn mul(self, other: Vector3D<f64>) -> M1_4 {
        let mut result = M1_4([[0.0; 1]; 4]);
        let o: M1_4 = other.into();

        for r in 0..4 {
            result.write_idx(
                r,
                (self.0[r][0] * o.0[0][0])
                    + (self.0[r][1] * o.0[1][0])
                    + (self.0[r][2] * o.0[2][0])
                    + (self.0[r][3] * o.0[3][0]),
            )
        }

        result
    }
}

impl std::ops::Mul<Point3D<f64>> for M4 {
    type Output = M1_4;

    fn mul(self, other: Point3D<f64>) -> M1_4 {
        let mut result = M1_4([[0.0; 1]; 4]);
        let o: M1_4 = other.into();

        for r in 0..4 {
            result.write_idx(
                r,
                (self.0[r][0] * o.0[0][0])
                    + (self.0[r][1] * o.0[1][0])
                    + (self.0[r][2] * o.0[2][0])
                    + (self.0[r][3] * o.0[3][0]),
            )
        }

        result
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct M1_4([[f64; 1]; 4]);

impl M1_4 {
    pub fn idx(&self, r: usize) -> f64 {
        self.0[r][0]
    }

    pub fn write_idx(&mut self, r: usize, v: f64) {
        self.0[r][0] = v
    }
}

impl From<Vector3D<f64>> for M1_4 {
    fn from(vector: Vector3D<f64>) -> Self {
        M1_4([[vector.x()], [vector.y()], [vector.z()], [0.0]])
    }
}

impl From<Point3D<f64>> for M1_4 {
    fn from(point: Point3D<f64>) -> Self {
        M1_4([[point.x()], [point.y()], [point.z()], [1.0]])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn transpose_identity() {
        let identity = M4::identity();
        let transposed = identity.clone().transpose();

        assert_eq!(identity, transposed);
    }

    #[test]
    fn select_m3_submatrix() {
        let m3 = M3([[1.0, 5.0, 0.0], [-3.0, 2.0, 7.0], [0.0, 6.0, -3.0]]);
        let m2 = M2([[-3.0, 2.0], [0.0, 6.0]]);
        let m2_2 = M2([[2.0, 7.0], [6.0, -3.0]]);
        assert_eq!(m3.submatrix(0, 0), m2_2);
        assert_eq!(m3.submatrix(0, 2), m2);
    }

    #[test]
    fn select_m4_submatrix() {
        let m4 = M4([
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ]);
        let m3 = M3([[-6.0, 1.0, 6.0], [-8.0, 8.0, 6.0], [-7.0, -1.0, 1.0]]);
        assert_eq!(m4.submatrix(2, 1), m3);
    }

    #[test]
    fn m3_minor() {
        let m3 = M3([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        assert_eq!(m3.minor(0, 0), -12.0);
        assert_eq!(m3.minor(1, 0), 25.0)
    }

    #[test]
    fn m3_cofactor() {
        let m3 = M3([[3.0, 5.0, 0.0], [2.0, -1.0, -7.0], [6.0, -1.0, 5.0]]);
        assert_eq!(m3.cofactor(0, 0), -12.0);
        assert_eq!(m3.cofactor(1, 0), -25.0);
    }

    #[test]
    fn m2_determinant() {
        let m2 = M2([[1.0, 5.0], [-3.0, 2.0]]);
        assert_eq!(m2.determinant(), 17.0);
    }

    #[test]
    fn m3_determinant() {
        let m3 = M3([[1.0, 2.0, 6.0], [-5.0, 8.0, -4.0], [2.0, 6.0, 4.0]]);
        assert_eq!(m3.cofactor(0, 0), 56.0);
        assert_eq!(m3.cofactor(0, 1), 12.0);
        assert_eq!(m3.cofactor(0, 2), -46.0);
        assert_eq!(m3.determinant(), -196.0)
    }

    #[test]
    fn m4_determinant() {
        let m4 = M4([
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ]);
        assert_eq!(m4.cofactor(0, 0), 690.0);
        assert_eq!(m4.cofactor(0, 1), 447.0);
        assert_eq!(m4.cofactor(0, 2), 210.0);
        assert_eq!(m4.cofactor(0, 3), 51.0);
        assert_eq!(m4.determinant(), -4071.0);
    }

    #[test]
    fn m4_invertibility_check() {
        let m4_invertable = M4([
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ]);
        let m4_uninvertable = M4([
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ]);
        assert_eq!(m4_uninvertable.inverse(), None);
        assert_ne!(m4_invertable.inverse(), None);
    }

    #[test]
    fn m4_inverse() {
        let m4_1 = M4([
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]);
        let m4_1_inverted = M4([
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308],
        ]);
        let m4_2 = M4([
            [9.0, 3.0, 0.0, 9.0],
            [-5.0, -2.0, -6.0, -3.0],
            [-4.0, 9.0, 6.0, 4.0],
            [-7.0, 6.0, 6.0, 2.0],
        ]);
        let m4_2_inverted = M4([
            [-0.04074, -0.07778, 0.14444, -0.22222],
            [-0.07778, 0.03333, 0.36667, -0.33333],
            [-0.02901, -0.14630, -0.10926, 0.12963],
            [0.17778, 0.06667, -0.26667, 0.33333],
        ]);

        assert_eq!(m4_1.inverse().unwrap(), m4_1_inverted);
        assert_eq!(m4_2.inverse().unwrap(), m4_2_inverted);
    }

    #[test]
    fn m4_inverse_detailed() {
        let m4 = M4([
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ]);

        let m4_inverted = m4.inverse().unwrap();

        assert_eq!(m4.determinant(), 532.0);
        assert_eq!(m4.cofactor(2, 3), -160.0);
        assert_eq!(m4_inverted.idx(3, 2), -160.0 / 532.0);
        assert_eq!(m4.cofactor(3, 2), 105.0);
        assert_eq!(m4_inverted.idx(2, 3), 105.0 / 532.0);
        assert_eq!(
            m4_inverted,
            M4([
                [0.21805, 0.45113, 0.24060, -0.04511],
                [-0.80827, -1.45677, -0.44361, 0.52068],
                [-0.07895, -0.22368, -0.05263, 0.19737],
                [-0.52256, -0.81391, -0.30075, 0.30639]
            ])
        )
    }

    #[test]
    fn m4_matrix_inverse_multiplication() {
        let m4_1 = M4([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);
        let m4_2 = M4([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        ]);

        let mult = m4_1 * m4_2;
        assert_eq!(mult * m4_2.inverse().unwrap(), m4_1)
    }
}
