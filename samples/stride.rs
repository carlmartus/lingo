extern crate lingo;

use lingo::{draw, gl, window};
use std::mem;

const STRIDE_VERT: &'static str = r#"
#version 100
precision mediump float;

attribute vec2 at_loc;
attribute vec4 at_color;

varying vec4 va_color;

void main() {
    va_color = at_color;
    gl_Position = vec4(at_loc, 0, 1);
}
"#;

const STRIDE_FRAG: &'static str = r#"
#version 100
precision mediump float;

varying vec4 va_color;

void main() {
    gl_FragColor = vec4(va_color);
}
"#;

#[repr(C, packed)]
struct Vertex(f32, f32, u8, u8, u8, u8);

struct Sample {
    win: window::Window,
    prog: draw::Program,
    verts: draw::HwBuf<Vertex>,
    pipeline: draw::Pipeline,
}

fn main() {
    match Sample::new() {
        Ok(mut s) => s.run(),
        Err(msg) => eprintln!("Error at start: {}", msg),
    }
}

impl Sample {
    pub fn new() -> Result<Sample, String> {
        let win = window::WindowBuilder::new()
            .with_title("Lingo stride example".to_string())
            .build()?;

        let prog = draw::Program::from_static(STRIDE_VERT, STRIDE_FRAG, &["at_loc", "at_color"])?;

        let mut verts = draw::HwBuf::new(3, draw::Usage::Static)?;
        verts.push(Vertex(0.0, 0.0, 1, 0, 0, 1));
        verts.push(Vertex(1.0, 0.0, 0, 1, 0, 1));
        verts.push(Vertex(0.0, 1.0, 0, 0, 1, 1));
        verts.prepear_graphics();

        let mut pipeline = draw::Pipeline::new(draw::PrimitiveType::Triangles)?;
        let buf_id = pipeline.push_buffer(&verts, mem::size_of::<Vertex>());

        pipeline.push_attribute(buf_id, 2, draw::DataType::F32, false);
        pipeline.push_attribute(buf_id, 4, draw::DataType::U8, false);

        draw::print_gl_error()?;

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
                    window::Command::Quit => break 'gameloop,
                    _ => (),
                }
            }

            // Peripheral events
            while let Some(p) = self.win.next_peripheral() {
                match p {
                    window::Peripheral::MousePosition(_, _) => (),
                    //_ => (),
                }
            }

            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            self.prog.use_program();
            self.verts.bind();
            self.pipeline.draw(3);

            draw::print_gl_error().unwrap();

            self.win.swap_buffers();
        }
    }
}
