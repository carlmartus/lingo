extern crate lingo;

use lingo::window::{Window, Command, Peripheral};
use lingo::shader::Program;
use lingo::gl;

const RED_VERT: &'static str = r#"
#version 130

attribute vec2 at_loc;

void main() {
    gl_Position = vec4(at_loc, 0, 1);
}
"#;

const RED_FRAG: &'static str = r#"
#version 130

void main() {
    gl_FragColor = vec4(1, 0, 0, 1);
}
"#;

struct Sample {
    win: Window,
    prog: Program,
}

fn main() {
    match Sample::new() {
        Ok(mut s) => s.run(),
        Err(msg) => eprintln!("Error at start: {}", msg),
    }
}

impl Sample {
    pub fn new() -> Result<Sample, String> {
        let win = Window::new("dialog");

        unsafe {
            gl::ClearColor(0.3, 0.4, 0.5, 1.0);
        }

        let prog = Program::from_static(RED_VERT, RED_FRAG)?;

        unsafe {
            gl::ClearColor(0.3, 0.4, 0.5, 1.0);
        }

        Ok(Sample {
            win, prog,
        })
    }

    pub fn run(&mut self) {
        'gameloop: loop {
            self.win.poll_events();

            // Command events
            while let Some(c) = self.win.next_command() {
                match c {
                    Command::Quit =>
                        break 'gameloop,
                    _ => (),
                }
            }

            // Peripheral events
            while let Some(p) = self.win.next_peripheral() {
                match p {
                    Peripheral::MousePosition(_, _) => (),
                    //_ => (),
                }
            }

            self.prog.use_program();

            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
            self.win.swap_buffers();
        }
    }
}
