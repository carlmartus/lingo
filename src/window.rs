extern crate gl;
extern crate glutin;
use glutin::{dpi, GlContext};
use glutin::{EventsLoop, GlWindow};
use std::collections::vec_deque::VecDeque;

pub use glutin::MouseButton;
pub use glutin::VirtualKeyCode;

const QUEUE_LEN: usize = 20;

type PeripheralQueue = VecDeque<Peripheral>;
type CommandQueue = VecDeque<Command>;

pub enum ButtonId {
    Keyboard {
        vcode: VirtualKeyCode,
        scancode: u32,
    },
    Mouse {
        vcode: MouseButton,
    },
}

pub enum Peripheral {
    MousePosition(f32, f32),
    Button(ButtonId, bool),
}

pub enum Command {
    Quit,
    WinResize(u32, u32),
}

pub struct Window {
    // Glutin releated
    pub events_loop: EventsLoop,
    pub gl_window: GlWindow,

    // Command queue
    queue_peripheral: PeripheralQueue,
    queue_command: CommandQueue,
}

pub struct WindowBuilder {
    title: Option<String>,
    w: u32,
    h: u32,
}

impl Window {
    pub fn new(title: String, w: u32, h: u32) -> Result<Window, String> {
        let events_loop = glutin::EventsLoop::new();
        let window = glutin::WindowBuilder::new()
            .with_title(title)
            .with_dimensions(dpi::LogicalSize::new(w as f64, h as f64));
        let context = glutin::ContextBuilder::new()
            .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGlEs, (2, 0)))
            .with_vsync(true);
        let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

        unsafe {
            gl_window.make_current().unwrap();
            gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        }

        Ok(Window {
            events_loop,
            gl_window,
            queue_peripheral: VecDeque::with_capacity(QUEUE_LEN),
            queue_command: VecDeque::with_capacity(QUEUE_LEN),
        })
    }

    pub fn swap_buffers(&self) {
        self.gl_window.swap_buffers().unwrap();
    }

    fn translate_glutin_window_event(
        event: &glutin::WindowEvent,
        peripherals: &mut PeripheralQueue,
        commands: &mut CommandQueue,
    ) {
        match event {
            glutin::WindowEvent::CloseRequested => commands.push_back(Command::Quit),
            glutin::WindowEvent::Resized(size) => {
                commands.push_back(Command::WinResize(size.width as u32, size.height as u32))
            }
            glutin::WindowEvent::CursorMoved { position, .. } => peripherals.push_back(
                Peripheral::MousePosition(position.x as f32, position.y as f32),
            ),
            _ => (),
        }
    }

    pub fn poll_events(&mut self) {
        self.queue_peripheral.clear();
        self.queue_command.clear();

        let el = &mut self.events_loop;
        let mut qc = &mut self.queue_command;
        let mut qp = &mut self.queue_peripheral;

        el.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => {
                Self::translate_glutin_window_event(&event, &mut qp, &mut qc)
            }
            _ => (),
        });
    }

    pub fn next_peripheral(&mut self) -> Option<Peripheral> {
        self.queue_peripheral.pop_front()
    }

    pub fn next_command(&mut self) -> Option<Command> {
        self.queue_command.pop_front()
    }
}

impl WindowBuilder {
    pub fn new() -> WindowBuilder {
        WindowBuilder {
            title: None,
            w: 640,
            h: 480,
        }
    }

    pub fn with_title(mut self, title: String) -> WindowBuilder {
        self.title = Some(title);
        self
    }

    pub fn with_dimensions(mut self, w: u32, h: u32) -> WindowBuilder {
        self.w = w;
        self.h = h;
        self
    }

    pub fn build(self) -> Result<Window, String> {
        let title_str = match self.title {
            Some(s) => s,
            None => "Lingo window".to_string(),
        };

        Window::new(title_str, self.w, self.h)
    }
}
