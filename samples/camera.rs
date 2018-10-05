extern crate lingo;

use lingo::attribute::{Attribute, DataType, PrimitiveType};
use lingo::hwbuf::{HwBuf, Usage};
use lingo::projection::{Matrix4x4, Vec3};
use lingo::shader::{Program, UniformLocation};
use lingo::window::{Command, Window, WindowBuilder};
use lingo::{error, gl};

const CAMERA_VERT: &'static str = r#"
#version 100
precision mediump float;

attribute vec2 at_loc;
uniform mat4 un_mvp;

void main() {
    gl_Position = un_mvp * vec4(at_loc, 0, 1);
}
"#;

const CAMERA_FRAG: &'static str = r#"
#version 100
precision mediump float;

void main() {
    gl_FragColor = vec4(1, 0, 0, 1);
}
"#;

struct Vertex(i16, i16);

struct Camera {
    win: Window,
    prog: Program,
    verts: HwBuf<Vertex>,
    attribs: Attribute,
    location_mvp: UniformLocation,
}

fn main() {
    match Camera::new() {
        Ok(mut s) => s.run(),
        Err(msg) => eprintln!("Error at start: {}", msg),
    }
}

impl Camera {
    pub fn new() -> Result<Camera, String> {
        // Create environment
        // Se hello_triangle.rs for description of this part

        let win = WindowBuilder::new()
            .with_title("dialog".to_string())
            .build()?;

        let prog = Program::from_static(CAMERA_VERT, CAMERA_FRAG, &["at_loc"])?;
        let mut verts = HwBuf::new(5, Usage::Static)?;
        verts.push(Vertex(0, 0));
        verts.push(Vertex(1, 0));
        verts.push(Vertex(0, 1));
        verts.prepear_graphics();

        let mut attribs = Attribute::new(4, PrimitiveType::Triangles)?;
        attribs.push_buffer(verts.get_gl_id());
        attribs.push_attribute(0, 2, DataType::I16, false);

        let location_mvp = prog.get_uniform_location("un_mvp");

        error::print_gl_error()?;

        unsafe {
            gl::ClearColor(0.3, 0.4, 0.5, 1.0);
        }

        Ok(Camera {
            win,
            prog,
            verts,
            attribs,
            location_mvp,
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

            unsafe {
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }

            let mut mat = Matrix4x4::new();

            // 2D Orthogonal mode
            //mat.ortho(-4f32, -3f32, 4f32, 3f32);

            // 3D camera mode
            mat.camera_3d(
                1.3f32,
                1.3333f32,
                0.1f32,
                20f32,
                Vec3(2f32, 1f32, 1f32), // Eye
                Vec3(0f32, 0f32, 0f32), // At
                Vec3(0f32, 0f32, 1f32),
            ); // Center

            self.prog.use_program();
            self.prog.set_uniform(&self.location_mvp, |loc| unsafe {
                gl::UniformMatrix4fv(loc, 1, gl::FALSE, mat.values.as_ptr());
            });

            self.verts.bind();
            self.attribs.draw(3);
            self.win.swap_buffers();
            error::print_gl_error().unwrap();
        }
    }
}
