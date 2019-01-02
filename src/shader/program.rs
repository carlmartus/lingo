extern crate gl;

use crate::shader::create_shader;
use crate::shader::{Program, UniformLocation};
use gl::types::{GLint, GLuint};
use std::ffi::CString;

impl Program {
    pub fn from_static(
        vert_src: &'static str,
        frag_src: &'static str,
        attribute_binds: &[&'static str],
    ) -> Result<Self, String> {
        let id_vert = create_shader(gl::VERTEX_SHADER, vert_src.to_string())?;
        let id_frag = create_shader(gl::FRAGMENT_SHADER, frag_src.to_string())?;

        let program;
        unsafe {
            program = gl::CreateProgram();
            gl::AttachShader(program, id_vert);
            gl::AttachShader(program, id_frag);
            gl::LinkProgram(program);

            for (i, attribute) in attribute_binds.iter().enumerate() {
                gl::BindAttribLocation(
                    program,
                    i as GLuint,
                    CString::new(*attribute).unwrap().as_ptr(),
                );
            }

            gl::DeleteShader(id_vert);
            gl::DeleteShader(id_frag);
        }

        Ok(Self { program })
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }

    pub fn unuse_program() {
        unsafe {
            gl::UseProgram(0);
        }
    }

    pub fn get_uniform_location(&self, name: &'static str) -> UniformLocation {
        let location =
            unsafe { gl::GetUniformLocation(self.program, CString::new(name).unwrap().as_ptr()) };

        UniformLocation(location)
    }

    pub fn set_uniform<F>(&self, location: &UniformLocation, cb: F)
    where
        F: FnOnce(GLint),
    {
        self.use_program();
        cb(location.0);
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
        }
    }
}

/*
fn create_shader(type_enum: GLenum, src: String) -> Result<GLuint, String> {
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
*/
