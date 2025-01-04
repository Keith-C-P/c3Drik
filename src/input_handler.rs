use input::event::keyboard::KeyboardEventTrait;
use input::{Libinput, LibinputInterface};
use libc::{O_RDONLY, O_RDWR, O_WRONLY};
use std::fs::{File, OpenOptions};
use std::os::unix::{fs::OpenOptionsExt, io::OwnedFd};
use std::path::Path;

struct Interface;
pub enum InputEvent {
    MouseMove(f64, f64),
    Keyboard(char),
    None,
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
    pub fn new() -> (InputEvent, Libinput) {
        let mut input = Libinput::new_with_udev(Interface);
        input.udev_assign_seat("seat0").unwrap();
        (InputEvent::None, input)
    }
    pub fn input_listener(mut input: Libinput) {
        loop {
            input.dispatch().unwrap();
            for event in &mut input {
                match event {
                    input::Event::Keyboard(ref key) => match key {
                        input::event::keyboard::KeyboardEvent::Key(ref k) => {
                            // println!("{}", k.key())
                        }
                        _ => {}
                    },
                    input::Event::Pointer(ref mouse) => match mouse {
                        input::event::PointerEvent::Motion(mouse_motion) => {
                            // println!("x:{}, y:{}", mouse_motion.dx(), mouse_motion.dy());
                        }
                        _ => {}
                    },
                    _ => {}
                }
                println!("Got event: {:?}", event);
            }
        }
    }
}
