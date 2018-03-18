extern crate gl;

use gl::types::{GLuint, GLint, GLenum, GLboolean};

pub enum PrimitiveType {
    Points,
    Lines,
    Triangles,
    TriangleFan,
    TriangleStrip,
    Quads,
}

pub enum DataType {
    U8,
    U16,
    U32,
    F32,
    F64,
}

struct Part {
    buffer_id: GLuint,
    element_count: GLint,
    data_type: GLenum,
    size: usize,
    normalize: GLboolean,
}

pub struct Attribute {
    draw_type: GLenum,
    parts: Vec<Part>,
    stride: GLint,
}

impl PrimitiveType {
    pub fn to_gl_enum(t: PrimitiveType) -> GLenum {
        match t {
            PrimitiveType::Points          => gl::POINTS,
            PrimitiveType::Lines           => gl::LINES,
            PrimitiveType::Triangles       => gl::TRIANGLES,
            PrimitiveType::TriangleFan     => gl::TRIANGLE_FAN,
            PrimitiveType::TriangleStrip   => gl::TRIANGLE_STRIP,
            PrimitiveType::Quads           => gl::QUADS,
        }
    }
}

impl DataType {
    pub fn to_gl_enum(&self) -> GLenum {
        match *self {
            DataType::U8    => gl::UNSIGNED_BYTE,
            DataType::U16   => gl::UNSIGNED_SHORT,
            DataType::U32   => gl::UNSIGNED_INT,
            DataType::F32   => gl::FLOAT,
            DataType::F64   => gl::DOUBLE,
        }
    }

    pub fn size(&self) -> usize {
        match *self {
            DataType::U8    => 1,
            DataType::U16   => 2,
            DataType::U32   => 4,
            DataType::F32   => 4,
            DataType::F64   => 8,
        }
    }
}

impl Attribute {
    pub fn new(stride: usize, primitive_type: PrimitiveType) -> Attribute {
        Attribute {
            draw_type: PrimitiveType::to_gl_enum(primitive_type),
            parts: Vec::new(),
            stride: stride as GLint,
        }
    }

    pub fn push_attribute(
        &mut self, buffer_id: usize, element_count: usize,
        data_type: DataType,
        normalize: bool) {

        let normalize = match normalize {
            true => gl::TRUE,
            false => gl::FALSE,
        };

        let size = data_type.size()*element_count;
        let data_type = data_type.to_gl_enum();

        self.parts.push(Part {
            buffer_id: buffer_id as GLuint,
            element_count: element_count as GLint,
            data_type,
            size,
            normalize,
        });
    }

    pub fn draw(&self, vertex_count: usize) {
        let mut offset = 0;

        for (i, p) in self.parts.iter().enumerate() {
            let i = i as u32;

            unsafe {
                gl::EnableVertexAttribArray(i);
                gl::BindBuffer(gl::ARRAY_BUFFER, p.buffer_id);

                gl::VertexAttribPointer(
                    i, p.element_count,
                    p.data_type, p.normalize,
                    self.stride,
                    offset as *const gl::types::GLvoid);

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
