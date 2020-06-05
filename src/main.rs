mod error {
    pub use anyhow::{Context, Result};
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum Error {
        #[error("It is meaningless to add two Point3Ds")]
        NonsensicalAddition,

        #[error(transparent)]
        Other(#[from] anyhow::Error),
    }
}

use error::Result;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Spatial<T> {
    Point3D { x: T, y: T, z: T },
    Vector3D { x: T, y: T, z: T },
}

impl<T> Spatial<T> {
    pub fn point(x: T, y: T, z: T) -> Self {
        Spatial::Point3D { x, y, z }
    }

    pub fn vector(x: T, y: T, z: T) -> Self {
        Spatial::Vector3D { x, y, z }
    }
}

// impl<T: std::ops::Sub<Output = T>> std::ops::Sub for Spatial<T> {
//     type Output = Result<Self>;
// }

impl<T: std::ops::Add<Output = T>> std::ops::Add for Spatial<T> {
    type Output = Result<Self>;

    fn add(self, other: Spatial<T>) -> Self::Output {
        match self {
            Spatial::Point3D { x, y, z } => {
                let a = x;
                let b = y;
                let c = z;

                match other {
                    Spatial::Vector3D { x, y, z } => Ok(Spatial::Point3D {
                        x: a + x,
                        y: b + y,
                        z: c + z,
                    }),
                    _ => Err(error::Error::NonsensicalAddition)?,
                }
            }
            Spatial::Vector3D { x, y, z } => {
                let a = x;
                let b = y;
                let c = z;

                match other {
                    Spatial::Point3D { x, y, z } => Ok(Spatial::Point3D {
                        x: a + x,
                        y: b + y,
                        z: c + z,
                    }),
                    Spatial::Vector3D { x, y, z } => Ok(Spatial::Vector3D {
                        x: a + x,
                        y: b + y,
                        z: c + z,
                    }),
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn point_addition() {
        let p1 = Spatial::point(0.0, 0.1, 0.2);
        let p2 = Spatial::point(0.5, 0.4, 0.3);
        let res = p1 + p2;

        assert!(res.is_err())
    }

    #[test]
    fn point_vector_addition() {
        let p1 = Spatial::point(0.0, 0.1, 0.2);
        let p2 = Spatial::vector(0.5, 0.4, 0.3);
        let res = p1 + p2;

        assert_eq!(
            res.unwrap(),
            Spatial::Point3D {
                x: 0.5,
                y: 0.5,
                z: 0.5
            }
        )
    }

    #[test]
    fn vector_point_addition() {
        let p1 = Spatial::point(0.0, 0.1, 0.2);
        let p2 = Spatial::vector(0.5, 0.4, 0.3);
        let res = p2 + p1;

        assert_eq!(
            res.unwrap(),
            Spatial::Point3D {
                x: 0.5,
                y: 0.5,
                z: 0.5
            }
        )
    }

    #[test]
    fn vector_vector_addition() {
        let p1 = Spatial::vector(0.0, 0.1, 0.2);
        let p2 = Spatial::vector(0.5, 0.4, 0.3);
        let res = p1 + p2;

        assert_eq!(
            res.unwrap(),
            Spatial::Vector3D {
                x: 0.5,
                y: 0.5,
                z: 0.5
            }
        )
    }
}
