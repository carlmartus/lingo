extern crate gl;

mod build;
mod program;

pub use crate::shader::build::ProgramBuilder;
use gl::types::{GLchar, GLenum, GLint, GLuint};
use std::{ffi, ptr, str};

pub struct Program {
    program: GLuint,
}

pub struct UniformLocation(GLint);

pub fn create_shader(type_enum: GLenum, src: String) -> Result<GLuint, String> {
    unsafe {
        let id = gl::CreateShader(type_enum);

        let c_str = ffi::CString::new(src.as_bytes()).unwrap();
        gl::ShaderSource(id, 1, &c_str.as_ptr(), ptr::null());
        gl::CompileShader(id);

        let mut status = gl::FALSE as GLint;
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut status);

        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = Vec::with_capacity(len as usize);
            buf.set_len((len as usize) - 1);
            gl::GetShaderInfoLog(id, len, ptr::null_mut(), buf.as_mut_ptr() as *mut GLchar);

            Err(str::from_utf8(&buf).unwrap().to_string())
        } else {
            Ok(id)
        }
    }
}
