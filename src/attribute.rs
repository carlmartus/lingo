extern crate gl;

use gl::types::{GLboolean, GLenum, GLint, GLuint};
use hwbuf::HwBufReference;

pub enum PrimitiveType {
    Points,
    Lines,
    Triangles,
    TriangleFan,
    TriangleStrip,
    Quads,
}

pub enum DataType {
    // Unsigned
    U8,
    U16,
    U32,
    // Signed
    I8,
    I16,
    I32,
    // Floating point
    F32,
    F64,
}

struct Part {
    buffer_id: usize,
    element_count: GLint,
    data_type: GLenum,
    size: usize,
    normalize: GLboolean,
}

struct BufferReference {
    gl_buffer: GLuint,
    stride: usize,
}

pub struct Pipeline {
    draw_type: GLenum,
    parts: Vec<Part>,
    buffers: Vec<BufferReference>,
}

impl PrimitiveType {
    pub fn to_gl_enum(t: PrimitiveType) -> GLenum {
        match t {
            PrimitiveType::Points => gl::POINTS,
            PrimitiveType::Lines => gl::LINES,
            PrimitiveType::Triangles => gl::TRIANGLES,
            PrimitiveType::TriangleFan => gl::TRIANGLE_FAN,
            PrimitiveType::TriangleStrip => gl::TRIANGLE_STRIP,
            PrimitiveType::Quads => gl::QUADS,
        }
    }
}

impl DataType {
    pub fn to_gl_enum(&self) -> GLenum {
        match *self {
            DataType::U8 => gl::UNSIGNED_BYTE,
            DataType::U16 => gl::UNSIGNED_SHORT,
            DataType::U32 => gl::UNSIGNED_INT,
            DataType::I8 => gl::BYTE,
            DataType::I16 => gl::SHORT,
            DataType::I32 => gl::INT,
            DataType::F32 => gl::FLOAT,
            DataType::F64 => gl::DOUBLE,
        }
    }

    pub fn size(&self) -> usize {
        match *self {
            DataType::U8 => 1,
            DataType::U16 => 2,
            DataType::U32 => 4,
            DataType::I8 => 1,
            DataType::I16 => 2,
            DataType::I32 => 4,
            DataType::F32 => 4,
            DataType::F64 => 8,
        }
    }
}

impl Pipeline {
    pub fn new(primitive_type: PrimitiveType) -> Result<Pipeline, String> {
        Ok(Pipeline {
            draw_type: PrimitiveType::to_gl_enum(primitive_type),
            parts: Vec::new(),
            buffers: Vec::new(),
        })
    }

    pub fn push_attribute(
        &mut self,
        buffer_id: usize,
        element_count: usize,
        data_type: DataType,
        normalize: bool,
    ) {
        let normalize = match normalize {
            true => gl::TRUE,
            false => gl::FALSE,
        };

        let size = data_type.size() * element_count;
        let data_type = data_type.to_gl_enum();

        self.parts.push(Part {
            buffer_id,
            element_count: element_count as GLint,
            data_type,
            size,
            normalize,
        });
    }

    pub fn push_buffer(&mut self, hw: &HwBufReference, stride: usize) -> usize {
        let index = self.buffers.len();
        self.buffers.push(BufferReference {
            gl_buffer: hw.get_gl_buffer(),
            stride,
        });
        index
    }

    pub fn draw(&self, vertex_count: usize) {
        let mut offset = 0;

        for (i, p) in self.parts.iter().enumerate() {
            let i = i as u32;

            unsafe {
                gl::EnableVertexAttribArray(i);

                let buffer_id = self.buffers[p.buffer_id].gl_buffer;
                let buffer_stride = self.buffers[p.buffer_id].stride;
                gl::BindBuffer(gl::ARRAY_BUFFER, buffer_id);

                gl::VertexAttribPointer(
                    i as GLuint,
                    p.element_count,
                    p.data_type,
                    p.normalize,
                    buffer_stride as i32,
                    offset as *const gl::types::GLvoid,
                );
                offset += p.size;
            }
        }

        unsafe {
            gl::DrawArrays(self.draw_type, 0, vertex_count as GLint);
        }

        for i in 0..self.parts.len() {
            unsafe {
                gl::DisableVertexAttribArray(i as u32);
            }
        }
    }
}
