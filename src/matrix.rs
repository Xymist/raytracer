use crate::descartes::{Point3D, Vector3D};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct M2([[f64; 2]; 2]);

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

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct M3([[f64; 3]; 3]);

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

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct M4([[f64; 4]; 4]);

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

impl<T> std::ops::Mul<T> for M4
where
    T: Into<M1_4>,
{
    type Output = M1_4;

    fn mul(self, other: T) -> M1_4 {
        let mut result = M1_4([[0.0; 1]; 4]);
        let o = other.into();

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
}
