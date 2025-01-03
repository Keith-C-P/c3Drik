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
    let mut frame: String = "".to_string();
    for j in 0..camera.image_height() {
        for i in 0..camera.image_width() {
            let pixel_center: Vec3 = camera.pixel00_loc()
                + (i as f64 * camera.pixel_delta_u())
                + (j as f64 * camera.pixel_delta_v());
            let ray_dir: Vec3 = pixel_center - camera.camera_pos();

            let r: ray::Ray = ray::Ray::new(camera.camera_pos(), ray_dir);
            // print!("{}", ray_hit(&r, &tri));
            frame += ray_hit(&r, &tri);
        }
        // println!("");
        frame += "\n";
    }
    println!("{}", frame);
}

fn main() {
    let terminal = terminal::Terminal::new();
    let mut camera: camera::Camera = camera::Camera::new();
    camera.set_width(terminal.columns() as i32);
    camera.set_aspect_ratio(terminal.columns() / (terminal.lines() - 2.0)); //FIXME remove - 2.0
    camera.set_focal_length(1.0 / 2.0);
    camera.set_stretch(camera::Stretch(0.5, 1.0));
    println!("{:?}", camera);

    let euler = Vec3 {
        x: 0.08,
        y: 0.0,
        z: 0.0,
    };

    let a = Vec3 {
        x: 1.,
        y: 0.0,
        z: -2.0,
    };
    let b = Vec3 {
        x: -1.0,
        y: 0.0,
        z: -2.0,
    };
    let c = Vec3 {
        x: 0.0,
        y: 1.0,
        z: -2.0,
    };
    let mut tri = object::Triangle::new(a, b, c);
    loop {
        draw_frame(camera, tri);

        // tri = tri.rotate_around_point(
        //     euler,
        //     Vec3 {
        //         x: 0.0,
        //         y: 0.0,
        //         z: -2.0,
        //     },
        // );
        camera.rotate_around_center(euler);
        println!(
            "viewport_u: {:?}\nviewport_v: {:?}",
            camera.viewport_u, camera.viewport_v
        );
        println!("triangle.c: {:?}", tri.c);
        thread::sleep(Duration::from_millis(50));
        // print!("\x1Bc");
    }
}
