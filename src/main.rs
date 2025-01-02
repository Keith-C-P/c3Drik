// #![allow(dead_code)]

use std::ops;
use term_size;
// use std::f64::consts::PI;
use libm::cos;
use libm::sin;
use std::thread;
use std::time::Duration;

#[derive(Debug, Copy, Clone)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn rotate_around_point(self: Vec3, center: Vec3, rotation: Quaternion) -> Vec3 {
        let p_normalised = Vec3 {
            x: self.x - center.x,
            y: self.y - center.y,
            z: self.z - center.z,
        };

        let rotated_p_normalised = rotation.rotate(&p_normalised);

        Vec3 {
            x: rotated_p_normalised.x + center.x,
            y: rotated_p_normalised.y + center.y,
            z: rotated_p_normalised.z + center.z,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Quaternion {
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

#[derive(Debug, Copy, Clone)]
struct Ray {
    origin: Vec3,
    dir: Vec3,
}
// struct Color {red: f32, green: f32, blue: f32}
trait Hittable {
    fn hit(&self, r: &Ray) -> bool;
}
#[derive(Debug, Copy, Clone)]
struct Triangle {
    a: Vec3,
    b: Vec3,
    c: Vec3,
    center: Vec3,
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Triangle {
        let center: Vec3 = Vec3 {
            x: (a.x + b.x + c.x) / 3.0,
            y: (a.y + b.y + c.y) / 3.0,
            z: (a.z + b.z + c.z) / 3.0,
        };
        Triangle { a, b, c, center }
    }
    pub fn rotate_around_center(&mut self, euler: Vec3) -> Triangle {
        let quaternion = Quaternion::euler_to_quaternion(euler);
        let a: Vec3 = self.a.rotate_around_point(self.center, quaternion);
        let b: Vec3 = self.b.rotate_around_point(self.center, quaternion);
        let c: Vec3 = self.c.rotate_around_point(self.center, quaternion);
        Triangle::new(a, b, c)
    }
}

impl Hittable for Triangle {
    fn hit(&self, r: &Ray) -> bool {
        let e1: Vec3 = self.b - self.a;
        let e2: Vec3 = self.c - self.a;

        let ray_cross_e2: Vec3 = Vec3::cross(&r.direction(), &e2);
        let det: f64 = Vec3::dot(&e1, &ray_cross_e2);

        if det > -f64::EPSILON && det < f64::EPSILON {
            return false; // This ray is parallel to this triangle.
        }

        let inv_det: f64 = 1.0 / det;
        let s: Vec3 = r.origin() - self.a;
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
}

#[derive(Debug, Copy, Clone)]
struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray) -> bool {
        let oc: Vec3 = self.center - r.origin();
        let a: f64 = Vec3::dot(&r.direction(), &r.direction());
        let b: f64 = -2.0 * Vec3::dot(&r.direction(), &oc);
        let c: f64 = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let discriminant: f64 = b * b - 4.0 * a * c;
        return discriminant >= 0.0;
    }
}

#[derive(Debug, Copy, Clone)]
struct Stretch(f64, f64);
#[derive(Debug, Copy, Clone)]
struct Camera {
    aspect_ratio: f64,
    image_width: i32,
    image_height: i32,

    focal_length: f64,
    viewport_height: f64,
    viewport_width: f64,
    camera_pos: Vec3,

    viewport_u: Vec3,
    viewport_v: Vec3,

    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,

    vieport_upper_left: Vec3,
    pixel00_loc: Vec3,

    stretch: Option<Stretch>,
}

struct Terminal {
    columns: f64, // x-axis
    lines: f64,   // y-axis
}

impl Terminal {
    pub fn new() -> Terminal {
        let dims = term_size::dimensions();
        match dims {
            Some((width, height)) => {
                println!("{:?}", dims);
                Terminal {
                    columns: width as f64,
                    lines: height as f64,
                }
            }
            None => panic!("Terminal Size Not Found"),
        }
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

        impl $T {
            fn dot(u: &$T, v: &$T) -> f64 {
                u.x * v.x + u.y * v.y + u.z * v.z
            }
            fn cross(u: &$T, v: &$T) -> Vec3 {
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

impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio: f64 = 16.0 / 9.0;
        let image_width: i32 = 400;

        let image_height_calc: i32 = (image_width as f64 / aspect_ratio) as i32;
        let image_height: i32 = if image_height_calc < 1 {
            1
        } else {
            image_height_calc
        };

        let focal_length: f64 = 1.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);
        let camera_pos: Vec3 = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };

        let viewport_u: Vec3 = Vec3 {
            x: viewport_width,
            y: 0.0,
            z: 0.0,
        };
        let viewport_v: Vec3 = Vec3 {
            x: 0.0,
            y: -viewport_height,
            z: 0.0,
        };

        let pixel_delta_u: Vec3 = viewport_u / image_width as f64;
        let pixel_delta_v: Vec3 = viewport_v / image_height as f64;

        let vieport_upper_left: Vec3 = camera_pos
            - Vec3 {
                x: 0.0,
                y: 0.0,
                z: focal_length,
            }
            - (viewport_u / 2.0)
            - (viewport_v / 2.0);
        let pixel00_loc: Vec3 = vieport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Camera {
            aspect_ratio,
            image_width,
            image_height,
            focal_length,
            viewport_height,
            viewport_width,
            viewport_u,
            viewport_v,
            camera_pos,
            pixel_delta_u,
            pixel_delta_v,
            vieport_upper_left,
            pixel00_loc,
            stretch: Option::None,
        }
    }

    fn update(&mut self) {
        let image_height_calc: i32 = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if image_height_calc < 1 {
            1
        } else {
            image_height_calc
        };

        self.viewport_width =
            self.viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.viewport_u = Vec3 {
            x: self.viewport_width,
            y: 0.0,
            z: 0.0,
        };

        self.viewport_v = Vec3 {
            x: 0.0,
            y: -self.viewport_height,
            z: 0.0,
        };

        match self.stretch {
            None => {
                self.pixel_delta_u = self.viewport_u / self.image_width as f64;
                self.pixel_delta_v = self.viewport_v / self.image_height as f64;
            }
            Some(stretch) => {
                self.pixel_delta_u = (self.viewport_u / self.image_width as f64) * stretch.0;
                self.pixel_delta_v = (self.viewport_v / self.image_height as f64) * stretch.1;

                self.vieport_upper_left = self.camera_pos
                    - Vec3 {
                        x: 0.0,
                        y: 0.0,
                        z: self.focal_length,
                    }
                    - (self.viewport_u * stretch.0 / 2.0)
                    - (self.viewport_v * stretch.1 / 2.0);
            }
        }
        self.pixel00_loc =
            self.vieport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    pub fn set_width(&mut self, width: i32) {
        self.image_width = width;
        self.update();
    }
    pub fn set_aspect_ratio(&mut self, aspect_ratio: f64) {
        self.aspect_ratio = aspect_ratio;
        self.update();
    }
    pub fn set_viewport_height(&mut self, viewport_height: f64) {
        self.viewport_height = viewport_height;
        self.update();
    }
    pub fn set_focal_length(&mut self, focal_length: f64) {
        self.focal_length = focal_length;
        self.update();
    }
    pub fn set_stretch(&mut self, stretch: Stretch) {
        self.stretch = Some(stretch);
        self.update();
    }
}
// TODO camera
// [ ] Convert to builder philosophy

impl Ray {
    fn origin(self) -> Vec3 {
        self.origin
    }
    fn direction(self) -> Vec3 {
        self.dir
    }
}

fn color_ray<'a>(r: &'a Ray, tri: &'a Triangle) -> &'a str {
    if tri.hit(r) {
        "#"
    } else {
        "."
    }
}

fn main() {
    // let b = cos(PI);
    let terminal = Terminal::new();
    let mut camera: Camera = Camera::new();
    camera.set_width(terminal.columns as i32);
    camera.set_aspect_ratio(terminal.columns / terminal.lines);
    camera.set_focal_length(1.5);
    camera.set_stretch(Stretch(0.5, 1.0));
    println!("{:?}", camera);

    let X_ROTATE_SPEED = 0.03 / 2.0;
    let Y_ROTATE_SPEED = 0.03 / 2.0;
    let Z_ROTATE_SPEED = 0.03 / 2.0;

    let mut euler = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };

    let a = Vec3 {
        x: 0.0,
        y: 0.5,
        z: -1.0,
    };
    let b = Vec3 {
        x: -0.5,
        y: -0.5,
        z: -1.0,
    };
    let c = Vec3 {
        x: 0.5,
        y: -0.5,
        z: -1.0,
    };
    let mut tri = Triangle::new(a, b, c);

    // let t: f64 =
    loop {
        for j in 0..camera.image_height {
            for i in 0..camera.image_width {
                let pixel_center: Vec3 = camera.pixel00_loc
                    + (i as f64 * camera.pixel_delta_u)
                    + (j as f64 * camera.pixel_delta_v);
                let ray_dir: Vec3 = pixel_center - camera.camera_pos;

                let r: Ray = Ray {
                    origin: camera.camera_pos,
                    dir: ray_dir,
                };
                print!("{}", color_ray(&r, &tri));
            }
            println!("");
        }
        euler.x += X_ROTATE_SPEED;
        euler.y += Y_ROTATE_SPEED;
        euler.z += Z_ROTATE_SPEED;
        tri = tri.rotate_around_center(euler);
        thread::sleep(Duration::from_millis(100));
    }
}

// TODO main
// [x] Add triangle
// [ ] Add Rotations
// [ ] Add camera rotations
// [ ] Add moveable objects
// [ ] Move stuff to different files
// [ ] Add mouse functionality with 'input' (rust package bind to libinput)
//      [ ] Add dynamic terminal sizing once threading is implemented
// [ ] Add Collision
// [ ] Add Gravity
