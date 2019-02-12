extern crate glfw;

use std::sync::mpsc::Receiver;
use glfw::Context;

struct Window {
    win: glfw::Window,
    glfw_ctx: glfw::Glfw,
    events: Receiver<(f64, glfw::WindowEvent)>,
}

impl Window {
    pub fn new() -> Self {
        let glfw_ctx = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        let (mut win, events) = glfw_ctx.create_window(300, 300, "Hello this is window", glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");


        win.set_key_polling(true);
        win.set_size_polling(true);
        win.make_current();

        gl::load_with(|s| win.get_proc_address(s) as *const _);

        Self { win, events, glfw_ctx }
    }

    pub fn next(&mut self) -> bool {
        self.glfw_ctx.wait_events();

        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::Size(w, h) => {
                    unsafe {
                        gl::Viewport(0, 0, w, h);
                    }
                },
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    self.win.set_should_close(true)
                },
                _ => (),
            }
        }

        self.win.swap_buffers();
        self.win.should_close()
    }
}
