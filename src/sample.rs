extern crate lingo;

use lingo::window::{Window, Command, Peripheral};
use lingo::shader::Program;
use lingo::hwbuf::HwBuf;
use lingo::attribute::{Attribute, PrimitiveType, DataType};
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

struct Vertex {
    x: f32,
    y: f32,
}

struct Sample {
    win: Window,
    prog: Program,
    verts: HwBuf<Vertex>,
    attribs: Attribute,
}

fn main() {
    match Sample::new() {
        Ok(mut s) => s.run(),
        Err(msg) => eprintln!("Error at start: {}", msg),
    }
}

impl Sample {
    pub fn new() -> Result<Sample, String> {
        let win = Window::new("dialog")?;
        let prog = Program::from_static(RED_VERT, RED_FRAG)?;
        let mut verts = HwBuf::new(10)?;
        let mut attribs = Attribute::new(8, PrimitiveType::Triangles)?;
        attribs.push_attribute(0, 2, DataType::F32, false);

        unsafe {
            gl::ClearColor(0.3, 0.4, 0.5, 1.0);
        }

        verts.push(Vertex { x: 0.0, y: 0.0 });
        verts.push(Vertex { x: 1.0, y: 0.0 });
        verts.push(Vertex { x: 0.0, y: 1.0 });
        verts.prepear_graphics();

        Ok(Sample {
            win, prog, verts, attribs,
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
            self.verts.bind();
            self.attribs.draw(3);

            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
            self.win.swap_buffers();
        }
    }
}
