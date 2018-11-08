pub extern crate gl;
pub extern crate glutin;

mod attribute;
mod error;
mod hwbuf;
mod projection;
mod shader;
pub mod window;

#[cfg(test)]
mod test;

pub mod draw {
    pub use attribute::{DataType, Pipeline, PrimitiveType};
    pub use error::print_gl_error;
    pub use hwbuf::HwBuf;
    pub use hwbuf::Usage;
    pub use projection::{Matrix4x4, Vec3};
    pub use shader::{Program, UniformLocation};
}
