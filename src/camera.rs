use crate::Vec3;

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

    vieport_upper_left: Vec3,
    pixel00_loc: Vec3,

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
    pub fn camera_pos(&self) -> Vec3 {
        self.camera_pos
    }
}
// TODO camera
// [ ] Convert to builder philosophy
