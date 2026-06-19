use std::collections::HashMap;
use std::f64::consts::PI;
use std::sync::{Arc, RwLock};
use std::thread;
use std::time::Duration;
use std::time::Instant;

mod camera;
mod input_handler;
mod object;
mod ray;
mod raytracer;
mod rotation;
mod terminal;
mod vector;

use camera::Camera;
// use input::event;
use input_handler::{InputEvent, InputState};
use object::{Object, ObjectTrait};
use raytracer::Scene;
use terminal::Terminal;
use vector::Vec3;

const TARGET_FPS: f64 = 60.0;
const FIXED_UPDATE_TIME: f64 = 1.0 / TARGET_FPS; // 16.67ms per update
const MAX_FRAME_SKIP: usize = 5;

fn noclip_movement_controller(
    camera: &mut Camera,
    input: &mut InputState,
    delta_time: f64,
) -> Camera {
    let speed: f64 = 0.08;
    let sensitivity: (f64, f64) = (0.001, 0.001);
    let mut buttonmap: HashMap<&str, u32> = HashMap::new();
    //17 30 31 32
    buttonmap.insert("Forward", 17);
    buttonmap.insert("Left", 30);
    buttonmap.insert("Back", 31);
    buttonmap.insert("Right", 32);
    buttonmap.insert("Up", 57);
    buttonmap.insert("Down", 42);
    let mut move_by_amount: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    if !input.keychord.is_empty() {
        if input.keychord.contains(
            buttonmap
                .get("Forward")
                .expect("expected Forward to be mapped"),
        ) {
            move_by_amount += -camera.view_dir();
        }
        if input
            .keychord
            .contains(buttonmap.get("Back").expect("expected Back to be mapped"))
        {
            move_by_amount += camera.view_dir();
        }
        if input
            .keychord
            .contains(buttonmap.get("Right").expect("expected Right to be mapped"))
        {
            move_by_amount += camera.viewport_u().normalise();
        }
        if input
            .keychord
            .contains(buttonmap.get("Left").expect("expected Left to be mapped"))
        {
            move_by_amount += -camera.viewport_u().normalise();
        }
        if input
            .keychord
            .contains(buttonmap.get("Up").expect("expected Up to be mapped"))
        {
            move_by_amount += -camera.viewport_v().normalise();
        }
        if input
            .keychord
            .contains(buttonmap.get("Down").expect("expected Down to be mapped"))
        {
            move_by_amount += camera.viewport_v().normalise();
        }
        if !(move_by_amount.x == 0.0 && move_by_amount.y == 0.0 && move_by_amount.z == 0.0) {
            move_by_amount = move_by_amount.normalise() * speed;
            // println!("{:?}", move_by_amount);
            camera.move_by(move_by_amount);
        }
    }
    if !(input.mouse_motion.0 == 0.0 && input.mouse_motion.1 == 0.0) {
        let pitch =
            -camera.viewport_u().normalise() * (sensitivity.1 * input.mouse_motion.1 * delta_time);
        let yaw =
            camera.viewport_v().normalise() * (sensitivity.0 * input.mouse_motion.0 * delta_time);
        camera.rotate_around_center(yaw);
        camera.rotate_around_center(pitch);
        camera.look(
            sensitivity.0 * -input.mouse_motion.0,
            sensitivity.1 * -input.mouse_motion.1,
        );
        input.mouse_motion = (0.0, 0.0);
    }
    *camera
}

fn main() {
    // env::set_var("RUST_BACKTRACE", "1");
    let inputs = Arc::new(RwLock::new(InputState::new()));

    let inputs_clone = Arc::clone(&inputs);
    thread::spawn(move || {
        let libinput_context = InputEvent::new_libinput_context();
        let mut pressed_keys: Vec<u32> = vec![];
        let mut event: InputEvent;
        loop {
            event = InputEvent::input_listener(libinput_context.clone(), &mut pressed_keys);
            // println!("{:?}", event);
            if let InputEvent::KeyEvent { keychord } = event.clone() {
                pressed_keys = keychord;
            }
            if let Ok(mut w) = inputs_clone.try_write() {
                match event {
                    InputEvent::KeyEvent { keychord } => {
                        w.keychord = keychord;
                    }
                    InputEvent::MouseEvent { dx, dy } => {
                        w.previous_mouse_motion = w.mouse_motion;
                        w.mouse_motion = (w.mouse_motion.0 + dx, w.mouse_motion.1 + dy);
                    }
                    InputEvent::None => {
                        w.mouse_motion = (0.0, 0.0);
                    }
                }
                // println!("Updated input state: {:?}", *w);
            } else {
                println!("Input Thread: Unable to access RwLock");
            }
            thread::sleep(Duration::from_millis(1));
        }
    });

    let inputs_clone = Arc::clone(&inputs);
    thread::spawn(move || {
        let euler = Vec3 {
            x: 0.0,
            y: 0.0,
            z: PI,
        };
        let terminal = Terminal::new();
        let mut object_list: Vec<Object> = vec![];
        let mut camera_binding = camera::Camera::new();
        let camera = camera_binding
            .set_width(terminal.columns() as i32)
            .set_aspect_ratio(terminal.columns() / (terminal.lines())) //FIXME remove - 2.0
            .set_focal_length(1.5)
            .set_stretch(camera::Stretch(0.4, 1.0));
        let tri_a = Vec3 {
            x: 0.5,
            y: 0.0,
            z: -2.0,
        };
        let tri_b = Vec3 {
            x: -0.5,
            y: 0.0,
            z: -2.0,
        };
        let tri_c = Vec3 {
            x: 0.0,
            y: 1.0,
            z: -2.0,
        };
        let tri = Object::new_triangle(&tri_a, &tri_b, &tri_c);
        object_list.push(tri);
        object_list.push(tri.rotate_around_center(euler).rotate_around_point(
            Vec3 {
                x: 0.0,
                y: PI,
                z: 0.0,
            },
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        ));
        let mut last_time = Instant::now();
        let mut accumulator = 0.0;
        let mut frame_skips = 0;
        loop {
            let now = Instant::now();
            let delta_time = now.duration_since(last_time).as_secs_f64();
            last_time = now;

            accumulator += delta_time;
            frame_skips = 0;

            while accumulator >= FIXED_UPDATE_TIME && frame_skips < MAX_FRAME_SKIP {
                if let Ok(mut w) = inputs_clone.try_write() {
                    noclip_movement_controller(camera, &mut w, FIXED_UPDATE_TIME);
                    w.mouse_motion = (0.0, 0.0);
                }
                accumulator -= FIXED_UPDATE_TIME;
                frame_skips += 1;
            }

            if frame_skips < MAX_FRAME_SKIP || accumulator < FIXED_UPDATE_TIME {
                Scene::draw_frame(camera, &object_list, delta_time);
            }

            // object_list[0] = object_list[0].rotate_around_center(euler);
            thread::sleep(Duration::from_millis((1.0 / TARGET_FPS * 1000.0) as u64));
            // thread::sleep(Duration::from_millis(10));
        }
    });

    loop {}
}
//TODO:
//[ ] button maps
