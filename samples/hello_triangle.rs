extern crate lingo;

use lingo::{draw, gl};

include!("examples_shared.rs");

const RED_VERT: &'static str = r#"#version 300 es
precision mediump float;

layout(location=0) in vec2 at_loc;

void main() {
    gl_Position = vec4(at_loc, 0, 1);
}
"#;

const RED_FRAG: &'static str = r#"#version 300 es
precision mediump float;

out vec4 out_color;

void main() {
    out_color = vec4(1, 0, 0, 1);
}
"#;

struct Vertex(f32, f32);

fn main() {
    if let Err(msg) = sample() {
        eprintln!("Example error: {}", msg);
    }
}

fn sample() -> Result<(), String> {
    let mut win = Window::new();

    // Create shader program from source
    let prog = draw::ProgramBuilder::new()?
        .vertex_shader(RED_VERT.to_string())?
        .fragment_shader(RED_FRAG.to_string())?
        .link()?
        .build();

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

    loop {
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        prog.use_program();
        verts.bind();
        pipeline.draw(3);

        draw::print_gl_error().unwrap();

        if win.next() {
            break;
        }
    }

    Ok(())
}
