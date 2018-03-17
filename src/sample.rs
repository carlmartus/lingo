extern crate lingo;

use lingo::window::{Window, Command, Peripheral};

fn main() {
    let mut w = Window::new("dialog");

    'gameloop: loop {

        w.poll_events();

        // Command events
        while let Some(c) = w.next_command() {
            match c {
                Command::Quit =>
                    break 'gameloop,
                _ => (),
            }
        }

        // Peripheral events
        while let Some(p) = w.next_peripheral() {
            match p {
                Peripheral::MousePosition(x, y) =>
                    println!("Mouse position {}, {}", x, y),
                //_ => (),
            }
        }

        w.draw();
    }
}
