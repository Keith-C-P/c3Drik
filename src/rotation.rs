use crate::vector::Vec3;
use libm::{cos, sin};
use std::ops;
#[derive(Debug, Copy, Clone)]
pub struct Quaternion {
    pub w: f64,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Quaternion {
    pub fn new(w: f64, x: f64, y: f64, z: f64) -> Quaternion {
        Quaternion { w, x, y, z }
    }
    pub fn new_from_vec3(point: Vec3) -> Quaternion {
        Quaternion {
            w: 0.0,
            x: point.x,
            y: point.y,
            z: point.z,
        }
    }

    pub fn to_vec3(&self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.z,
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

    pub fn rotate_vector(&self, v: Vec3) -> Vec3 {
        let q_vec = Quaternion::new(0.0, v.x, v.y, v.z);
        let rotated = *self * q_vec * self.conjugate();
        Vec3::new(rotated.x, rotated.y, rotated.z)
    }

    pub fn rotate_local(&self, quat: &Quaternion) -> Quaternion {
        *self * *quat * self.conjugate()
    }
    pub fn rotate_global(&self, quat: &Quaternion) -> Quaternion {
        self.conjugate() * *quat * *self
    }
    pub fn rotate_point_local(&self, point: &Vec3) -> Vec3 {
        let point_quat: Quaternion = Quaternion::new_from_vec3(*point);
        let result: Quaternion = self.rotate_local(&point_quat);
        Vec3 {
            x: result.x,
            y: result.y,
            z: result.z,
        }
    }
    pub fn rotate_point_global(&self, point: &Vec3) -> Vec3 {
        let point_quat: Quaternion = Quaternion::new_from_vec3(*point);
        let result: Quaternion = self.rotate_global(&point_quat);
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
        let hc = cos(euler.y / 2.0);
        let pc = cos(euler.x / 2.0);
        let bc = cos(euler.z / 2.0);
        let hs = sin(euler.y / 2.0);
        let ps = sin(euler.x / 2.0);
        let bs = sin(euler.z / 2.0);
        Quaternion {
            w: hc * pc * bc + hs * ps * bs,
            x: -hc * ps * bc - hs * pc * bs,
            y: hc * ps * bs - hs * pc * bc,
            z: hs * ps * bc - hc * pc * bs,
        }
    }
    pub fn w(&self) -> f64 {
        self.w
    }
    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }
    pub fn normalize(&self) {
        let mag = (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt();
        if mag > 0.0 {
            Quaternion::new(self.x / mag, self.y / mag, self.z / mag, self.w / mag);
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
