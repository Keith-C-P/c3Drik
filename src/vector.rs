use libm::sqrt;

use crate::rotation;
use std::ops;
#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn rotate_local(self, rotation: &rotation::Quaternion) -> Vec3 {
        let rotated =
            rotation.rotate_local(&rotation::Quaternion::new(0.0, self.x, self.y, self.z));
        Vec3 {
            x: rotated.x,
            y: rotated.y,
            z: rotated.z,
        }
    }
    pub fn rotate_global(self, rotation: &rotation::Quaternion) -> Vec3 {
        let rotated =
            rotation.rotate_global(&rotation::Quaternion::new(0.0, self.x, self.y, self.z));
        Vec3 {
            x: rotated.x,
            y: rotated.y,
            z: rotated.z,
        }
    }

    pub fn rotate_around_point_local(self, center: &Vec3, rotation: &rotation::Quaternion) -> Vec3 {
        let p_normalised = Vec3 {
            x: self.x - center.x,
            y: self.y - center.y,
            z: self.z - center.z,
        };

        let rotated_p_normalised = rotation.rotate_point_local(&p_normalised);

        Vec3 {
            x: rotated_p_normalised.x + center.x,
            y: rotated_p_normalised.y + center.y,
            z: rotated_p_normalised.z + center.z,
        }
    }
    pub fn rotate_around_point_global(
        self,
        center: &Vec3,
        rotation: &rotation::Quaternion,
    ) -> Vec3 {
        let p_normalised = Vec3 {
            x: self.x - center.x,
            y: self.y - center.y,
            z: self.z - center.z,
        };

        let rotated_p_normalised = rotation.rotate_point_global(&p_normalised);

        Vec3 {
            x: rotated_p_normalised.x + center.x,
            y: rotated_p_normalised.y + center.y,
            z: rotated_p_normalised.z + center.z,
        }
    }

    pub fn normalise(self) -> Vec3 {
        let inv_sqrt = 1.0 / sqrt(self.x * self.x + self.y * self.y + self.z * self.z);
        Vec3 {
            x: self.x * inv_sqrt,
            y: self.y * inv_sqrt,
            z: self.z * inv_sqrt,
        }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z }
    }
}

macro_rules! impl_vec3_operations {
    ($T:ident) => {
        // Vec3 + Vec3
        impl ops::Add<$T> for $T {
            type Output = Self;

            fn add(self, rhs: Self) -> Self {
                $T {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                    z: self.z + rhs.z,
                }
            }
        }

        // Vec3 - Vec3
        impl ops::Sub<$T> for $T {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self {
                $T {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                    z: self.z - rhs.z,
                }
            }
        }

        // Vec3 * Vec3
        impl ops::Mul<$T> for $T {
            type Output = Self;

            fn mul(self, rhs: $T) -> Self {
                $T {
                    x: self.x * rhs.x,
                    y: self.y * rhs.y,
                    z: self.z * rhs.z,
                }
            }
        }

        // Vec3 * s (s => scalar)
        impl ops::Mul<f64> for $T {
            type Output = Self;

            fn mul(self, rhs: f64) -> Self {
                return (Self {
                    x: self.x * rhs,
                    y: self.y * rhs,
                    z: self.z * rhs,
                });
            }
        }

        // s * Vec3
        impl ops::Mul<$T> for f64 {
            type Output = $T;

            fn mul(self, rhs: $T) -> $T {
                return (rhs * self);
            }
        }

        //Vec3 / s
        impl ops::Div<f64> for $T {
            type Output = Self;

            fn div(self, rhs: f64) -> Self {
                return ((1.0 / rhs) * self);
            }
        }

        // Vec3 += Vec3
        impl ops::AddAssign<$T> for $T {
            fn add_assign(&mut self, rhs: Self) {
                self.x += rhs.x;
                self.y += rhs.y;
                self.z += rhs.z;
            }
        }

        // Vec3 -= Vec3
        impl ops::SubAssign<$T> for $T {
            fn sub_assign(&mut self, rhs: Self) {
                self.x -= rhs.x;
                self.y -= rhs.y;
                self.z -= rhs.z;
            }
        }

        // Vec3 *= t
        impl ops::MulAssign<f64> for $T {
            fn mul_assign(&mut self, rhs: f64) {
                self.x *= rhs;
                self.y *= rhs;
                self.z *= rhs;
            }
        }

        impl ops::Neg for $T {
            type Output = Self;

            fn neg(self) -> Self {
                self * (-1.0)
            }
        }

        impl $T {
            pub fn dot(u: &$T, v: &$T) -> f64 {
                u.x * v.x + u.y * v.y + u.z * v.z
            }
            pub fn cross(u: &$T, v: &$T) -> Vec3 {
                Vec3 {
                    x: u.y * v.z - u.z * v.y,
                    y: u.z * v.x - u.x * v.z,
                    z: u.x * v.y - u.y * v.x,
                }
            }
        }
    };
}

impl_vec3_operations!(Vec3);
