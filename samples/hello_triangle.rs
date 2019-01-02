extern crate lingo;

use lingo::{draw, gl, window};

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
        // Create window
        let win = window::WindowBuilder::new()
            .with_title("dialog".to_string())
            .build()?;

        /*
        let prog = draw::ProgramBuilder::new()
            .fragment_shader(RED_FRAG.to_string())?
            .vertex_shader(RED_VERT.to_string())?
            .bind_attribute("at_loc".to_string(), 0)
            .build();*/

        // Create shader program from source
        let prog = draw::Program::from_static(RED_VERT, RED_FRAG, &["at_loc"])?;

        // Create buffer for geometry
        let mut verts = draw::HwBuf::new(10, draw::Usage::Static)?;

        // Fill buffer with a triangle
        verts.push(Vertex(0.0, 0.0));
        verts.push(Vertex(1.0, 0.0));
        verts.push(Vertex(0.0, 1.0));
        verts.prepear_graphics();

        // Describe rendering attributes
        let mut pipeline = draw::Pipeline::new(draw::PrimitiveType::Triangles)?;
        let buf_id = pipeline.push_buffer(&verts, 0);
        pipeline.push_attribute(buf_id, 2, draw::DataType::F32, false);

        // Did any error occur?
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
