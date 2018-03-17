extern crate gl;
extern crate glutin;
use glutin::{EventsLoop, GlWindow};
use glutin::GlContext;
use std::collections::vec_deque::VecDeque;

const QUEUE_LEN: usize = 20;

pub enum Peripheral {
    MousePosition(f32, f32),
}

pub enum Command {
    Quit,
    WinResize(u32, u32),
}

enum GlutinEvent {
    Peripheral(Peripheral),
    Command(Command),
    None,
}

pub struct Window {
    // Glutin releated
    events_loop: EventsLoop,
    gl_window: GlWindow,

    // Command queue
    queue_peripheral: VecDeque<Peripheral>,
    queue_command: VecDeque<Command>,
}

impl Window {
    pub fn new(title: &'static str) -> Window {
        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(400, 300);
        let context = glutin::ContextBuilder::new()
            .with_vsync(true);
        let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

        unsafe {
            gl_window.make_current().unwrap();
            gl::load_with(|symbol| {
                gl_window.get_proc_address(symbol) as *const _
            });
            //gl::ClearColor(0.0, 1.0, 0.0, 1.0);
        }

        Window {
            events_loop, gl_window,
            queue_peripheral: VecDeque::with_capacity(QUEUE_LEN),
            queue_command: VecDeque::with_capacity(QUEUE_LEN),
        }
    }

    pub fn swap_buffers(&self) {
        self.gl_window.swap_buffers().unwrap();
    }

    pub fn poll_events(&mut self) {
        self.queue_peripheral.clear();
        self.queue_command.clear();

        let el = &mut self.events_loop;
        let qc = &mut self.queue_command;
        let qp = &mut self.queue_peripheral;

        //self.events_loop.poll_events(|event| {
        el.poll_events(|event| {
            match match event {
                glutin::Event::WindowEvent{ event, .. } => match event {
                    glutin::WindowEvent::Closed =>
                        GlutinEvent::Command(Command::Quit),
                    glutin::WindowEvent::Resized(w, h) =>
                        GlutinEvent::Command(Command::WinResize(w, h)),
                    glutin::WindowEvent::CursorMoved{ position, .. } =>
                        GlutinEvent::Peripheral(Peripheral::MousePosition(
                                position.0 as f32, position.1 as f32)),
                    _ => GlutinEvent::None,
                },
                _ => GlutinEvent::None,
            } {
                GlutinEvent::Command(c) =>
                    if qc.len() < QUEUE_LEN {
                        qc.push_back(c);
                    },
                GlutinEvent::Peripheral(p) =>
                    if qp.len() < QUEUE_LEN {
                        qp.push_back(p);
                    },
                _ => (),
            }
        });
    }

    pub fn next_peripheral(&mut self) -> Option<Peripheral> {
        self.queue_peripheral.pop_front()
    }

    pub fn next_command(&mut self) -> Option<Command> {
        self.queue_command.pop_front()
    }
}
