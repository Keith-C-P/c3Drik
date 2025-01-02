use crate::vector::Vec3;
use libm::{cos, sin};
use std::ops;
#[derive(Debug, Copy, Clone)]
pub struct Quaternion {
    w: f64,
    x: f64,
    y: f64,
    z: f64,
}

impl Quaternion {
    pub fn new(w: f64, x: f64, y: f64, z: f64) -> Quaternion {
        Quaternion { w, x, y, z }
    }
    pub fn new_from_point(point: Vec3) -> Quaternion {
        Quaternion {
            w: 0.0,
            x: point.x,
            y: point.y,
            z: point.z,
        }
    }
    pub fn conjugate(&self) -> Quaternion {
        Quaternion {
            w: self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
    pub fn rotate(&self, point: &Vec3) -> Vec3 {
        let point_quat = Quaternion::new_from_point(*point);
        let result = *self * point_quat * self.conjugate();
        Vec3 {
            x: result.x,
            y: result.y,
            z: result.z,
        }
    }
    pub fn euler_to_quaternion(euler: Vec3) -> Quaternion {
        // x = p
        // y = h
        // z = b
        let h = euler.y;
        let p = euler.x;
        let b = euler.z;
        Quaternion {
            w: cos(h / 2.0) * cos(p / 2.0) * cos(b / 2.0)
                + sin(h / 2.0) * sin(p / 2.0) * sin(b / 2.0),
            x: -cos(h / 2.0) * sin(p / 2.0) * cos(b / 2.0)
                - sin(h / 2.0) * cos(p / 2.0) * sin(b / 2.0),
            y: cos(h / 2.0) * sin(p / 2.0) * sin(b / 2.0)
                - sin(h / 2.0) * cos(p / 2.0) * cos(b / 2.0),
            z: sin(h / 2.0) * sin(p / 2.0) * cos(b / 2.0)
                - cos(h / 2.0) * cos(p / 2.0) * sin(b / 2.0),
        }
    }
}

impl ops::Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, rhs: Self) -> Self {
        Quaternion {
            w: self.w * rhs.w - self.x * rhs.x - self.y * rhs.y - self.z * rhs.z,
            x: self.w * rhs.x + self.x * rhs.w + self.y * rhs.z - self.z * rhs.y,
            y: self.w * rhs.y - self.x * rhs.z + self.y * rhs.w + self.z * rhs.x,
            z: self.w * rhs.z + self.x * rhs.y - self.y * rhs.x + self.z * rhs.w,
        }
    }
}
