use input::event::keyboard::{KeyState, KeyboardEventTrait};
use input::{Libinput, LibinputInterface};
use libc::{O_RDONLY, O_RDWR, O_WRONLY};
use std::fs::{File, OpenOptions};
use std::os::unix::{fs::OpenOptionsExt, io::OwnedFd};
use std::path::Path;
struct Interface;
#[derive(Debug, Clone)]
pub enum InputEvent {
    MouseEvent { dx: f64, dy: f64 },
    KeyEvent { keychord: Vec<u32> },
    None,
}

#[derive(Debug)]
pub struct InputState {
    pub mouse_motion: (f64, f64),
    pub previous_mouse_motion: (f64, f64), // previous motion
    pub keychord: Vec<u32>,
}

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read((flags & O_RDONLY != 0) | (flags & O_RDWR != 0))
            .write((flags & O_WRONLY != 0) | (flags & O_RDWR != 0))
            .open(path)
            .map(|file| file.into())
            .map_err(|err| err.raw_os_error().unwrap())
    }
    fn close_restricted(&mut self, fd: OwnedFd) {
        drop(File::from(fd));
    }
}

impl InputEvent {
    pub fn new_libinput_context() -> Libinput {
        let mut input = Libinput::new_with_udev(Interface);
        input.udev_assign_seat("seat0").unwrap();
        input
    }

    pub fn input_listener(mut input: Libinput, pressed_keys: &mut Vec<u32>) -> InputEvent {
        input.dispatch().unwrap();
        for event in &mut input {
            // println!("Got event: {:?}", event);
            match event {
                input::Event::Keyboard(ref key) => match key {
                    input::event::keyboard::KeyboardEvent::Key(ref k) => {
                        let keystate = k.key_state();
                        let key = k.key();
                        // println!("{} {:?}", key, keystate);
                        match keystate {
                            KeyState::Pressed => {
                                pressed_keys.push(key);
                                return InputEvent::KeyEvent {
                                    keychord: pressed_keys.clone(),
                                };
                            }
                            KeyState::Released => {
                                pressed_keys.retain(|&x| x != key);
                                return InputEvent::KeyEvent {
                                    keychord: pressed_keys.clone(),
                                };
                            }
                        }
                    }
                    _ => {}
                },
                input::Event::Pointer(ref mouse) => match mouse {
                    input::event::PointerEvent::Motion(mouse_motion) => {
                        // println!("x:{}, y:{}", mouse_motion.dx(), mouse_motion.dy());
                        return InputEvent::MouseEvent {
                            dx: mouse_motion.dx(),
                            dy: mouse_motion.dy(),
                        };
                    }
                    _ => {}
                },
                _ => {}
            }
        }
        InputEvent::None
    }
}

impl InputState {
    pub fn new() -> Self {
        Self {
            mouse_motion: (0.0, 0.0),
            previous_mouse_motion: (0.0, 0.0),
            keychord: vec![],
        }
    }
}
//FIXME add debug statements
