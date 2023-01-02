use winit::{
    event::{ElementState, Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

pub struct Window {
    event_loop: EventLoop<()>,
    window: winit::window::Window,
}

impl Window {
    pub fn new() -> Self {
        let event_loop = EventLoop::new();

        let window = WindowBuilder::new()
            .with_title("A fantastic window!")
            .build(&event_loop)
            .unwrap();

        Window { event_loop, window }
    }

    pub fn run(self) {
        self.event_loop.run(move |event, _, control_flow| {
            control_flow.set_wait();

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => control_flow.set_exit(),
                    WindowEvent::MouseInput {
                        state: ElementState::Released,
                        ..
                    } => {
                        self.window.request_redraw();
                    }
                    _ => (),
                },
                Event::RedrawRequested(_) => {
                    println!("\nredrawing!\n");
                }
                _ => (),
            }
        });
    }
}
