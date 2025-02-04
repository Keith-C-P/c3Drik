use crate::ray::Ray;
use crate::rotation::Quaternion;
use crate::vector::Vec3;

#[derive(Debug, Copy, Clone)]
pub enum Object {
    Triangle {
        a: Vec3,
        b: Vec3,
        c: Vec3,
        center: Vec3,
    },
    Sphere {
        center: Vec3,
        radius: f64,
    },
}

pub trait ObjectTrait {
    fn hit(&self, r: &Ray) -> bool;
    fn move_to(&self, v: Vec3) -> Self;
    fn rotate_around_center(&self, euler: Vec3) -> Self;
    fn rotate_around_point(&self, euler: Vec3, point: Vec3) -> Self;
}

impl Object {
    pub fn new_triangle(a: &Vec3, b: &Vec3, c: &Vec3) -> Self {
        let center: Vec3 = Vec3 {
            x: (a.x + b.x + c.x) / 3.0,
            y: (a.y + b.y + c.y) / 3.0,
            z: (a.z + b.z + c.z) / 3.0,
        };
        Object::Triangle {
            a: *a,
            b: *b,
            c: *c,
            center,
        }
    }
    pub fn new_sphere(center: &Vec3, radius: &f64) -> Self {
        Object::Sphere {
            center: *center,
            radius: *radius,
        }
    }
}
impl ObjectTrait for Object {
    fn hit(&self, r: &Ray) -> bool {
        match &self {
            Self::Triangle { a, b, c, center: _ } => {
                let e1: Vec3 = *b - *a;
                let e2: Vec3 = *c - *a;

                let ray_cross_e2: Vec3 = Vec3::cross(&r.direction(), &e2);
                let det: f64 = Vec3::dot(&e1, &ray_cross_e2);

                if det > -f64::EPSILON && det < f64::EPSILON {
                    return false; // This ray is parallel to this triangle.
                }

                let inv_det: f64 = 1.0 / det;
                let s: Vec3 = r.origin() - *a;
                let u: f64 = inv_det * Vec3::dot(&s, &ray_cross_e2);
                if u < 0.0 || u > 1.0 {
                    return false;
                }

                let s_cross_e1: Vec3 = Vec3::cross(&s, &e1);
                let v: f64 = inv_det * Vec3::dot(&r.direction(), &s_cross_e1);
                if v < 0.0 || u + v > 1.0 {
                    return false;
                }
                // At this stage we can compute t to find out where the intersection point is on the line.
                let t = inv_det * Vec3::dot(&e2, &s_cross_e1);

                if t > f64::EPSILON {
                    // ray intersection
                    let intersection_point = r.origin() + r.direction() * t;
                    return true;
                } else {
                    // This means that there is a line intersection but not a ray intersection.
                    return false;
                }
            }
            Self::Sphere { center, radius } => {
                let oc: Vec3 = *center - r.origin();
                let a: f64 = Vec3::dot(&r.direction(), &r.direction());
                let b: f64 = -2.0 * Vec3::dot(&r.direction(), &oc);
                let c: f64 = Vec3::dot(&oc, &oc) - radius * radius;
                let discriminant: f64 = b * b - 4.0 * a * c;
                return discriminant >= 0.0;
            }
        }
    }
    fn move_to(&self, v: Vec3) -> Self {
        match self {
            Self::Triangle { a, b, c, center: _ } => {
                Self::new_triangle(&(*a + v), &(*b + v), &(*c + v))
            }
            Self::Sphere { center, radius } => Self::new_sphere(&(*center + v), radius),
        }
    }
    fn rotate_around_center(&self, euler: Vec3) -> Self {
        match self {
            Self::Triangle { a, b, c, center } => {
                let quaternion = Quaternion::euler_to_quaternion(euler);
                let a: Vec3 = a.rotate_around_point(center, &quaternion);
                let b: Vec3 = b.rotate_around_point(center, &quaternion);
                let c: Vec3 = c.rotate_around_point(center, &quaternion);
                Object::new_triangle(&a, &b, &c)
            }
            Self::Sphere {
                center: _,
                radius: _,
            } => *self,
        }
    }
    fn rotate_around_point(&self, euler: Vec3, point: Vec3) -> Self {
        match self {
            Self::Triangle { a, b, c, center: _ } => {
                let quaternion = Quaternion::euler_to_quaternion(euler);
                let a: Vec3 = a.rotate_around_point(&point, &quaternion);
                let b: Vec3 = b.rotate_around_point(&point, &quaternion);
                let c: Vec3 = c.rotate_around_point(&point, &quaternion);
                Object::new_triangle(&a, &b, &c)
            }
            Self::Sphere { center, radius } => {
                let quaternion = Quaternion::euler_to_quaternion(euler);
                let center: Vec3 = center.rotate_around_point(&point, &quaternion);
                Object::new_sphere(&center, radius)
            }
        }
    }
}
