extern crate lingo;

use lingo::attribute::{DataType, Pipeline, PrimitiveType};
use lingo::hwbuf::{HwBuf, Usage};
use lingo::shader::Program;
use lingo::window::{Command, Peripheral, Window, WindowBuilder};
use lingo::{error, gl};

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
    pipeline: Pipeline,
}

fn main() {
    match Sample::new() {
        Ok(mut s) => s.run(),
        Err(msg) => eprintln!("Error at start: {}", msg),
    }
}

impl Sample {
    pub fn new() -> Result<Sample, String> {
        // Create window
        let win = WindowBuilder::new()
            .with_title("dialog".to_string())
            .build()?;

        // Create shader program from source
        let prog = Program::from_static(RED_VERT, RED_FRAG, &["at_loc"])?;

        // Create buffer for geometry
        let mut verts = HwBuf::new(10, Usage::Static)?;

        // Fill buffer with a triangle
        verts.push(Vertex(0.0, 0.0));
        verts.push(Vertex(1.0, 0.0));
        verts.push(Vertex(0.0, 1.0));
        verts.prepear_graphics();

        // Describe rendering attributes
        let mut pipeline = Pipeline::new(PrimitiveType::Triangles)?;
        let buf_id = pipeline.push_buffer(&verts, 0);
        pipeline.push_attribute(buf_id, 2, DataType::F32, false);

        // Did any error occur?
        error::print_gl_error()?;

        unsafe {
            gl::ClearColor(0.3, 0.4, 0.5, 1.0);
        }

        Ok(Sample {
            win,
            prog,
            verts,
            pipeline,
        })
    }

    pub fn run(&mut self) {
        'gameloop: loop {
            self.win.poll_events();

            // Command events
            while let Some(c) = self.win.next_command() {
                match c {
                    Command::Quit => break 'gameloop,
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
            self.verts.bind();
            self.pipeline.draw(3);

            error::print_gl_error().unwrap();

            self.win.swap_buffers();
        }
    }
}
