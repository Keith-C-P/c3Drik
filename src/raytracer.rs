use crate::camera::Camera;
use crate::object::{Object, ObjectTrait};
use crate::ray::Ray;
use crate::vector::Vec3;

#[derive(Debug)]
pub struct Scene {}

impl Scene {
    fn ray_hit(r: &Ray, object: &Object) -> usize {
        if object.hit(r) {
            6
        } else {
            0
        }
    }

    pub fn draw_frame(camera: &Camera, object_list: &[Object], delta_time: f64) {
        if delta_time > 1.0 / 60.0 {
            return;
        }
        let brightness: [&str; 7] = [" ", ".", ",", "*", "!", "@", "#"];
        let mut frame: String = "".to_string();
        for j in 0..camera.image_height() {
            for i in 0..camera.image_width() {
                let pixel_center: Vec3 = camera.pixel00_loc()
                    + (i as f64 * camera.pixel_delta_u())
                    + (j as f64 * camera.pixel_delta_v());
                let ray_dir: Vec3 = pixel_center - camera.camera_pos();

                let r: Ray = Ray::new(camera.camera_pos(), ray_dir);
                // print!("{}", ray_hit(&r, &tri));
                let obj_iter = object_list.iter();
                let mut max: usize = 0;
                for object in obj_iter {
                    max = if Scene::ray_hit(&r, object) > max {
                        Scene::ray_hit(&r, object)
                    } else {
                        max
                    };
                }
                frame += brightness[max];
            }
            frame += "\n";
            // println!("");
        }
        print!("{}", frame);
    }
}
