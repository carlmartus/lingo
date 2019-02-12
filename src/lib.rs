pub extern crate gl;

mod attribute;
mod error;
mod hwbuf;
mod projection;
mod shader;

#[cfg(test)]
mod test;

pub mod draw {
    pub use crate::attribute::{DataType, Pipeline, PrimitiveType};
    pub use crate::error::print_gl_error;
    pub use crate::hwbuf::HwBuf;
    pub use crate::hwbuf::Usage;
    pub use crate::projection::{Matrix4x4, Vec3};
    pub use crate::shader::{Program, ProgramBuilder, UniformLocation};
}
