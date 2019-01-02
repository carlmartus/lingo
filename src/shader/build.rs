extern crate gl;

use crate::shader::{create_shader, Program};
use gl::types::GLuint;
use std::ffi::CString;

enum Components {
    Shader(GLuint),
    Attribute(String, usize),
}

pub struct ProgramBuilder {
    comps: Vec<Components>,
}

impl ProgramBuilder {
    pub fn new() -> Self {
        ProgramBuilder { comps: Vec::new() }
    }

    pub fn fragment_shader(&mut self, source: String) -> Result<&mut Self, String> {
        self.comps.push(Components::Shader(create_shader(
            gl::VERTEX_SHADER,
            source,
        )?));
        Ok(self)
    }

    pub fn vertex_shader(&mut self, source: String) -> Result<&mut Self, String> {
        self.comps.push(Components::Shader(create_shader(
            gl::VERTEX_SHADER,
            source,
        )?));
        Ok(self)
    }

    pub fn bind_attribute(&mut self, name: String, bind_id: usize) -> &mut Self {
        self.comps.push(Components::Attribute(name, bind_id));
        self
    }

    pub fn build(&mut self) -> Result<Program, String> {
        let program;
        unsafe {
            program = gl::CreateProgram();

            for c in &self.comps {
                match c {
                    Components::Shader(id) => gl::AttachShader(program, *id),
                    _ => (),
                }
            }

            gl::LinkProgram(program);

            for c in &self.comps {
                match c {
                    Components::Shader(id) => gl::DeleteShader(*id),
                    Components::Attribute(name, id) => {
                        let name_str = &name.as_str();
                        gl::BindAttribLocation(
                            program,
                            *id as GLuint,
                            CString::new(*name_str).unwrap().as_ptr(),
                            );

                    }
                }
            }
        }

        Ok(Program { program })
    }
}
