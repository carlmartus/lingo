extern crate gl;

use crate::error::check_gl_error;
use crate::shader::{create_shader, Program};
use gl::types::{GLenum, GLuint};
use std::ffi::CString;

pub struct ProgramBuilder {
    program: GLuint,
    shaders: Vec<GLuint>,
}

impl ProgramBuilder {
    pub fn new() -> Result<Self, String> {
        let program = unsafe { gl::CreateProgram() };

        check_gl_error()?;

        Ok(ProgramBuilder {
            program,
            shaders: Vec::new(),
        })
    }

    pub fn fragment_shader(&mut self, source: String) -> Result<&mut Self, String> {
        self.add_shader(gl::FRAGMENT_SHADER, source)
    }

    pub fn vertex_shader(&mut self, source: String) -> Result<&mut Self, String> {
        self.add_shader(gl::VERTEX_SHADER, source)
    }

    pub fn geometry_shader(&mut self, source: String) -> Result<&mut Self, String> {
        self.add_shader(gl::GEOMETRY_SHADER, source)
    }

    pub fn compute_shader(&mut self, source: String) -> Result<&mut Self, String> {
        self.add_shader(gl::COMPUTE_SHADER, source)
    }

    pub fn tesslation_control_shader(&mut self, source: String) -> Result<&mut Self, String> {
        self.add_shader(gl::TESS_CONTROL_SHADER, source)
    }

    pub fn tesslation_evaluation_shader(&mut self, source: String) -> Result<&mut Self, String> {
        self.add_shader(gl::TESS_EVALUATION_SHADER, source)
    }

    pub fn bind_attribute(&mut self, name: String, bind_id: usize) -> Result<&mut Self, String> {
        let name_str = &name.as_str();
        unsafe {
            gl::BindAttribLocation(
                self.program,
                bind_id as GLuint,
                CString::new(*name_str).unwrap().as_ptr(),
            );
        }
        check_gl_error()?;

        Ok(self)
    }

    pub fn link(&mut self) -> Result<&mut Self, String> {
        unsafe {
            gl::LinkProgram(self.program);
            check_gl_error()?;
        }

        Ok(self)
    }

    pub fn build(&mut self) -> Program {
        for c in &self.shaders {
            unsafe {
                gl::DeleteShader(*c);
            }
        }

        Program {
            program: self.program,
        }
    }

    fn add_shader(&mut self, shader_type: GLenum, source: String) -> Result<&mut Self, String> {
        let shader = create_shader(shader_type, source)?;

        unsafe {
            gl::AttachShader(self.program, shader);
        }

        self.shaders.push(shader);
        Ok(self)
    }
}
