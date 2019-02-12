extern crate lingo;

use lingo::{draw, gl};
use std::mem;

include!("examples_shared.rs");

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

fn main() {
    if let Err(msg) = sample() {
        eprintln!("Example error: {}", msg);
    }
}

fn sample() -> Result<(), String> {
    let mut win = Window::new();

    let prog = draw::ProgramBuilder::new()?
        .vertex_shader(STRIDE_VERT.to_string())?
        .fragment_shader(STRIDE_FRAG.to_string())?
        .link()?
        .bind_attribute("at_loc".to_string(), 0)?
        .bind_attribute("at_color".to_string(), 1)?
        .build();

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
