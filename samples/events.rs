extern crate lingo;

use lingo::window;

fn main() {
    let mut win = window::WindowBuilder::new()
        .with_title("Event test".to_string())
        .build().unwrap();

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
                        window::ButtonId::Keyboard { scancode, .. } => {
                            println!("keyboard {}", scancode);
                        },
                        window::ButtonId::Mouse(button) => {
                            print!("mouse ");
                            match button {
                                window::MouseButton::Left => println!("left"),
                                window::MouseButton::Right => println!("right"),
                                window::MouseButton::Middle => println!("middle"),
                                window::MouseButton::Other(n) => println!("other {}", n),
                            };
                        },
                    }
                },
            }
        }

        let mut close = false;

        // Command events
        while let Some(c) = win.next_command() {
            match c {
                window::Command::Quit => {
                    println!("Command Quit");
                    close = true;
                },
                _ => (),
            }
        }

        if close {
            break'gameloop;
        }
    }
}
