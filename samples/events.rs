extern crate lingo;

use lingo::window;

fn main() {
    let mut win = window::WindowBuilder::new()
        .with_title("Event test".to_string())
        .build()
        .unwrap();

    'gameloop: loop {
        win.poll_events();

        // Peripherals
        while let Some(p) = win.next_peripheral() {
            // print!("Peripheral({}) ", p.device_id);

            match p.event {
                window::PeripheralEvent::MousePosition(x, y) => println!("Mouse move {}, {}", x, y),
                window::PeripheralEvent::Button(id, press) => {
                    print!("Button press {} ", press);
                    match id {
                        window::ButtonId::Keyboard {
                            scancode, vcode, ..
                        } => {
                            print!("keyboard: Has Virtual? {}, ", vcode.is_some());
                            println!("Scancode is {}", scancode);
                        }
                        window::ButtonId::Mouse(button) => {
                            print!("mouse ");
                            match button {
                                window::MouseButton::Left => println!("left"),
                                window::MouseButton::Right => println!("right"),
                                window::MouseButton::Middle => println!("middle"),
                                window::MouseButton::Other(n) => println!("other {}", n),
                            };
                        }
                    }
                }
            }
        }

        let mut close = false;

        // Command events
        while let Some(c) = win.next_command() {
            match c {
                window::Command::Quit => {
                    println!("Command Quit");
                    close = true;
                }
                window::Command::WinResize(w, h) => {
                    println!("Window resized to {}x{}", w, h);
                }
                window::Command::WinFocus(focused) => println!("Window focused? {}", focused),
            }
        }

        if close {
            break 'gameloop;
        }
    }
}
