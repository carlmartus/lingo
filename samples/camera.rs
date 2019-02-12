extern crate lingo;

use lingo::{draw, gl};

include!("examples_shared.rs");

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

fn main() {
    if let Err(msg) = sample() {
        eprintln!("Example error: {}", msg);
    }
}

fn sample() -> Result<(), String> {
    // Create environment
    // Se hello_triangle.rs for description of this part

    let mut win = Window::new();

    let prog = draw::ProgramBuilder::new()?
        .vertex_shader(CAMERA_VERT.to_string())?
        .fragment_shader(CAMERA_FRAG.to_string())?
        .link()?
        .bind_attribute("at_loc".to_string(), 0)?
        .build();

    let mut verts = draw::HwBuf::new(5, draw::Usage::Static)?;
    verts.push(Vertex(0, 0));
    verts.push(Vertex(1, 0));
    verts.push(Vertex(0, 1));
    verts.prepear_graphics();

    let mut pipeline = draw::Pipeline::new(draw::PrimitiveType::Triangles)?;
    let buf_id = pipeline.push_buffer(&verts, 0);
    pipeline.push_attribute(buf_id, 2, draw::DataType::I16, false);

    let location_mvp = prog.get_uniform_location("un_mvp");

    draw::print_gl_error()?;

    unsafe {
        gl::ClearColor(0.3, 0.4, 0.5, 1.0);
    }

    loop {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        let mut mat = draw::Matrix4x4::new();

        // 2D Orthogonal mode
        //mat.ortho(-4f32, -3f32, 4f32, 3f32);

        // 3D camera mode
        mat.camera_3d(
            1.3f32,
            1.3333f32,
            0.1f32,
            20f32,
            draw::Vec3(2f32, 1f32, 1f32), // Eye
            draw::Vec3(0f32, 0f32, 0f32), // At
            draw::Vec3(0f32, 0f32, 1f32),
        ); // Center

        prog.use_program();
        prog.set_uniform(&location_mvp, |loc| unsafe {
            gl::UniformMatrix4fv(loc, 1, gl::FALSE, mat.values.as_ptr());
        });

        verts.bind();
        pipeline.draw(3);
        draw::print_gl_error().unwrap();

        if win.next() {
            break;
        }
    }

    Ok(())
}
