pub extern crate gl;
extern crate glutin;

pub mod window;
pub mod hwbuf;
pub mod shader;

pub fn print_version() {
    print!("Hello work");
}

#[cfg(test)]
mod tests {
    #[test]
    fn version() {
        Lingo::print_version();
        assert_eq!(2 + 2, 4);
    }
}
