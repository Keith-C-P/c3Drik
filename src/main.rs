// #![allow(dead_code)]
use std::thread;
use std::time::Duration;

mod camera;
mod object;
mod ray;
mod rotation;
mod terminal;
mod vector;

use object::{Object, Triangle};
use vector::Vec3;

fn ray_hit<'a>(r: &'a ray::Ray, tri: &'a object::Triangle) -> &'a str {
    if tri.hit(r) {
        "#"
    } else {
        "."
    }
}

fn draw_frame(camera: camera::Camera, tri: Triangle) {
    for j in 0..camera.image_height() {
        for i in 0..camera.image_width() {
            let pixel_center: Vec3 = camera.pixel00_loc()
                + (i as f64 * camera.pixel_delta_u())
                + (j as f64 * camera.pixel_delta_v());
            let ray_dir: Vec3 = pixel_center - camera.camera_pos();

            let r: ray::Ray = ray::Ray::new(camera.camera_pos(), ray_dir);
            print!("{}", ray_hit(&r, &tri));
        }
        println!("");
    }
}

fn main() {
    let terminal = terminal::Terminal::new();
    let mut camera: camera::Camera = camera::Camera::new();
    camera.set_width(terminal.columns() as i32);
    camera.set_aspect_ratio(terminal.columns() / terminal.lines());
    camera.set_focal_length(1.5);
    camera.set_stretch(camera::Stretch(0.5, 1.0));
    println!("{:?}", camera);

    let euler = Vec3 {
        x: 0.03,
        y: 0.08,
        z: 0.13,
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
    let mut tri = object::Triangle::new(a, b, c);
    loop {
        draw_frame(camera, tri);

        tri = tri.rotate_around_center(euler);
        thread::sleep(Duration::from_millis(100));
        print!("\x1Bc");
    }
}
