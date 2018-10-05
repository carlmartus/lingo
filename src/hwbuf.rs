extern crate gl;

use gl::types::{GLenum, GLuint};
use std::vec::Vec;
use std::{mem, ptr};

pub enum Usage {
    Stream,
    Static,
    Dynamic,
}

pub struct HwBuf<T> {
    gl_vbo: u32,
    data: Vec<T>,
}

impl Usage {
    pub fn to_gl_enum(&self) -> GLenum {
        match *self {
            Usage::Stream => gl::STREAM_DRAW,
            Usage::Static => gl::STATIC_DRAW,
            Usage::Dynamic => gl::DYNAMIC_DRAW,
        }
    }
}

impl<T> HwBuf<T> {
    pub fn new(max_verts: usize, usage: Usage) -> Result<HwBuf<T>, String> {
        let mut gl_vbo: u32 = 0;

        unsafe {
            let max_size: isize = (max_verts * mem::size_of::<T>()) as isize;
            gl::GenBuffers(1, &mut gl_vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, gl_vbo);
            gl::BufferData(gl::ARRAY_BUFFER, max_size, ptr::null(), usage.to_gl_enum());
        };

        Ok(HwBuf {
            gl_vbo,
            data: Vec::with_capacity(max_verts),
        })
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.gl_vbo);
        }
    }

    pub fn get_gl_id(&self) -> GLuint {
        self.gl_vbo
    }

    pub fn push(&mut self, vert: T) {
        if self.data.len() <= self.data.capacity() {
            self.data.push(vert);
        }
    }

    pub fn rewind(&mut self) {
        self.data.clear();
    }

    pub fn prepear_graphics(&self) {
        if self.data.len() == 0 {
            return;
        }

        unsafe {
            let tot_size = (self.data.len() * mem::size_of::<T>()) as isize;
            gl::BindBuffer(gl::ARRAY_BUFFER, self.gl_vbo);
            gl::BufferSubData(gl::ARRAY_BUFFER, 0, tot_size, mem::transmute(&self.data[0]));
        }
    }

    pub fn vertex_count(&self) -> usize {
        self.data.len()
    }
}
