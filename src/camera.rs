use crate::{rotation::Quaternion, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Stretch(pub f64, pub f64);
#[derive(Debug, Copy, Clone)]
pub struct Camera {
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

    viewport_upper_left: Vec3,
    pixel00_loc: Vec3,
    forward: Vec3,
    orientation: Quaternion,

    stretch: Option<Stretch>,
}

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
        let orientation: Quaternion = Quaternion::new(0.0, 0.0, 1.0, 0.0);

        let focal_length: f64 = 1.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);
        let camera_pos: Vec3 = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        };
        let view_dir = Vec3 {
            x: 2.0 * (orientation.x() * orientation.z() + orientation.w() * orientation.y()),
            y: 2.0 * (orientation.y() * orientation.z() - orientation.w() * orientation.x()),
            z: 1.0 - 2.0 * (orientation.x() * orientation.x() + orientation.y() * orientation.y()),
        };

        // let viewport_u: Vec3 = Vec3 {
        //     x: viewport_width,
        //     y: 0.0,
        //     z: 0.0,
        // };
        let right: Vec3 = Vec3 {
            x: 2.0 * (orientation.x() * orientation.y() - orientation.w() * orientation.z()),
            y: 1.0 - 2.0 * (orientation.x() * orientation.x() + orientation.z() * orientation.z()),
            z: 2.0 * (orientation.y() * orientation.z() + orientation.w() * orientation.x()),
        };
        let viewport_u = viewport_width * right;

        // let viewport_v: Vec3 = Vec3 {
        //     x: 0.0,
        //     y: -viewport_height,
        //     z: 0.0,
        // };

        let up = Vec3 {
            x: 2.0 * (orientation.x() * orientation.y() - orientation.w() * orientation.z()),
            y: 1.0 - 2.0 * (orientation.x() * orientation.x() + orientation.z() * orientation.z()),
            z: 2.0 * (orientation.y() * orientation.z() + orientation.w() * orientation.x()),
        };

        let viewport_v = viewport_height * up;

        let pixel_delta_u: Vec3 = viewport_u / image_width as f64;
        let pixel_delta_v: Vec3 = viewport_v / image_height as f64;

        let viewport_upper_left: Vec3 =
            camera_pos - (view_dir * focal_length) - (viewport_u / 2.0) - (viewport_v / 2.0);
        let pixel00_loc: Vec3 = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

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
            viewport_upper_left,
            pixel00_loc,
            forward: view_dir,
            orientation,
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

                self.viewport_upper_left = self.camera_pos
                    - (self.forward * self.focal_length)
                    - (self.viewport_u / 2.0)
                    - (self.viewport_v / 2.0);
            }
            Some(stretch) => {
                self.pixel_delta_u = (self.viewport_u / self.image_width as f64) * stretch.0;
                self.pixel_delta_v = (self.viewport_v / self.image_height as f64) * stretch.1;

                self.viewport_upper_left = self.camera_pos
                    - (self.forward * self.focal_length)
                    - (self.viewport_u * stretch.0 / 2.0)
                    - (self.viewport_v * stretch.1 / 2.0);
            }
        }
        self.pixel00_loc =
            self.viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    pub fn set_width(&mut self, width: i32) -> &mut Self {
        self.image_width = width;
        self.update();
        self
    }
    pub fn set_aspect_ratio(&mut self, aspect_ratio: f64) -> &mut Self {
        self.aspect_ratio = aspect_ratio;
        self.update();
        self
    }
    pub fn set_viewport_height(&mut self, viewport_height: f64) -> &mut Self {
        self.viewport_height = viewport_height;
        self.update();
        self
    }
    pub fn set_focal_length(&mut self, focal_length: f64) -> &mut Self {
        self.focal_length = focal_length;
        self.update();
        self
    }
    pub fn set_stretch(&mut self, stretch: Stretch) -> &mut Self {
        self.stretch = Some(stretch);
        self.update();
        self
    }

    pub fn image_height(&self) -> i32 {
        self.image_height
    }
    pub fn image_width(&self) -> i32 {
        self.image_width
    }
    pub fn pixel00_loc(&self) -> Vec3 {
        self.pixel00_loc
    }
    pub fn pixel_delta_u(&self) -> Vec3 {
        self.pixel_delta_u
    }
    pub fn pixel_delta_v(&self) -> Vec3 {
        self.pixel_delta_v
    }
    pub fn viewport_u(&self) -> Vec3 {
        self.viewport_u
    }
    pub fn viewport_v(&self) -> Vec3 {
        self.viewport_v
    }
    pub fn camera_pos(&self) -> Vec3 {
        self.camera_pos
    }
    pub fn view_dir(&self) -> Vec3 {
        self.forward
    }

    pub fn rotate_around_center(&mut self, euler: Vec3) {
        let quaternion: Quaternion = Quaternion::euler_to_quaternion(euler);
        self.viewport_u = self
            .viewport_u
            .rotate_around_point_local(&self.camera_pos, &quaternion);
        self.viewport_v = self
            .viewport_v
            .rotate_around_point_local(&self.camera_pos, &quaternion);
        self.viewport_upper_left = self
            .viewport_upper_left
            .rotate_around_point_local(&self.camera_pos, &quaternion);
        self.forward = self
            .forward
            .rotate_around_point_local(&self.camera_pos, &quaternion);

        match self.stretch {
            None => {
                self.pixel_delta_u = self.viewport_u / self.image_width as f64;
                self.pixel_delta_v = self.viewport_v / self.image_height as f64;
            }
            Some(stretch) => {
                self.pixel_delta_u = (self.viewport_u / self.image_width as f64) * stretch.0;
                self.pixel_delta_v = (self.viewport_v / self.image_height as f64) * stretch.1;
            }
        }
        self.pixel00_loc =
            self.viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    pub fn look(&mut self, dx: f64, dy: f64) {
        // let pitch = Quaternion::euler_to_quaternion(dy * self.viewport_u().normalise());
        // let yaw = Quaternion::euler_to_quaternion(dx * self.viewport_v().normalise());
        let pitch = Quaternion::euler_to_quaternion(Vec3 {
            x: dy,
            y: 0.0,
            z: 0.0,
        });
        let yaw = Quaternion::euler_to_quaternion(Vec3 {
            x: 0.0,
            y: -dx,
            z: 0.0,
        });

        self.orientation = yaw * self.orientation;
        self.orientation = self.orientation * pitch;
        self.orientation.normalize();
        // self.viewport_u = self
        //     .viewport_u
        //     .rotate_around_point_local(&self.camera_pos, &pitch);
        // self.viewport_v = self
        //     .viewport_v
        //     .rotate_around_point_local(&self.camera_pos, &pitch);
        // self.viewport_upper_left = self
        //     .viewport_upper_left
        //     .rotate_around_point_local(&self.camera_pos, &pitch);
        // self.forward = self
        //     .forward
        //     .rotate_around_point_local(&self.camera_pos, &pitch);

        // self.viewport_u = self
        //     .viewport_u
        //     .rotate_around_point_global(&self.camera_pos, &yaw);
        // self.viewport_v = self
        //     .viewport_v
        //     .rotate_around_point_global(&self.camera_pos, &yaw);
        // self.viewport_upper_left = self
        //     .viewport_upper_left
        //     .rotate_around_point_global(&self.camera_pos, &yaw);
        // self.forward = self
        //     .forward
        //     .rotate_around_point_global(&self.camera_pos, &yaw);

        match self.stretch {
            None => {
                self.pixel_delta_u = self.viewport_u / self.image_width as f64;
                self.pixel_delta_v = self.viewport_v / self.image_height as f64;
            }
            Some(stretch) => {
                self.pixel_delta_u = (self.viewport_u / self.image_width as f64) * stretch.0;
                self.pixel_delta_v = (self.viewport_v / self.image_height as f64) * stretch.1;
            }
        }
        self.pixel00_loc =
            self.viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    pub fn move_by(&mut self, dx: Vec3) {
        self.camera_pos += dx;
        match self.stretch {
            None => {
                self.pixel_delta_u = self.viewport_u / self.image_width as f64;
                self.pixel_delta_v = self.viewport_v / self.image_height as f64;

                self.viewport_upper_left = self.camera_pos
                    - (self.forward * self.focal_length)
                    - (self.viewport_u / 2.0)
                    - (self.viewport_v / 2.0);
            }
            Some(stretch) => {
                self.pixel_delta_u = (self.viewport_u / self.image_width as f64) * stretch.0;
                self.pixel_delta_v = (self.viewport_v / self.image_height as f64) * stretch.1;

                self.viewport_upper_left = self.camera_pos
                    - (self.forward * self.focal_length)
                    - (self.viewport_u * stretch.0 / 2.0)
                    - (self.viewport_v * stretch.1 / 2.0);
            }
        }
        self.pixel00_loc =
            self.viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    pub fn up(&self) {
        // right = np.array([
        //     1 - 2 * (y**2 + z**2),
        //     2 * (x*y + w*z),
        //     2 * (x*z - w*y)
        // ])

        // forward = np.array([
        //     2 * (x*z + w*y),
        //     2 * (y*z - w*x),
        //     1 - 2 * (x**2 + y**2)
        // ])

        let Quaternion { w, x, y, z } = self.orientation;
        Vec3::new(
            2.0 * (x * y - w * z),
            1.0 - 2.0 * (x * 2.0 + z * 2.0),
            2.0 * (y * z + w * x),
        );
    }
}
// TODO camera
// [ ] Convert to builder philosophy
