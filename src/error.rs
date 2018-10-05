extern crate gl;

pub fn check_gl_error() -> Result<(), String> {
    match unsafe { gl::GetError() } {
        gl::INVALID_ENUM => Err("Invalid enum".to_string()),
        gl::INVALID_VALUE => Err("Invalid value".to_string()),
        gl::INVALID_OPERATION => Err("Invalid operation".to_string()),
        gl::STACK_OVERFLOW => Err("Stack overflow".to_string()),
        gl::STACK_UNDERFLOW => Err("Stack underflow".to_string()),
        gl::OUT_OF_MEMORY => Err("Out of memmory".to_string()),
        _ => Ok(()),
    }
}

pub fn print_gl_error() -> Result<(), String> {
    match check_gl_error() {
        Err(msg) => {
            eprintln!("GL error: {}", msg);
            Err(msg)
        }
        _ => Ok(()),
    }
}
