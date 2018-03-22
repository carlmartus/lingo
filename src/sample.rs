extern crate lingo;

use lingo::window::{Window, Command, Peripheral};
use lingo::shader::Program;
use lingo::hwbuf::{HwBuf, Usage};
use lingo::attribute::{Attribute, PrimitiveType, DataType};
use lingo::{gl, error};

const RED_VERT: &'static str = r#"
#version 100
precision mediump float;

attribute vec2 at_loc;

void main() {
    gl_Position = vec4(at_loc, 0, 1);
}
"#;

const RED_FRAG: &'static str = r#"
#version 100
precision mediump float;

void main() {
    gl_FragColor = vec4(1, 0, 0, 1);
}
"#;

struct Vertex(f32, f32);

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
        error::print_gl_error();

        let binds = ["at_loc"];
        let prog = Program::from_static(RED_VERT, RED_FRAG, &binds)?;
        error::print_gl_error();
        let mut verts = HwBuf::new(10, Usage::Static)?;
        error::print_gl_error();
        let mut attribs = Attribute::new(8, PrimitiveType::Triangles)?;
        error::print_gl_error();
        attribs.push_buffer(verts.get_gl_id());
        attribs.push_attribute(0, 2, DataType::F32, false);
        error::print_gl_error();

        unsafe {
            gl::ClearColor(0.3, 0.4, 0.5, 1.0);
        }

        verts.push(Vertex(0.0, 0.0));
        verts.push(Vertex(1.0, 0.0));
        verts.push(Vertex(0.0, 1.0));
        verts.prepear_graphics();

        error::print_gl_error();

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

            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            self.prog.use_program();
            error::print_gl_error();
            self.verts.bind();
            error::print_gl_error();
            self.attribs.draw(3);
            error::print_gl_error();

            self.win.swap_buffers();
        }
    }
}
