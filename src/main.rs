use std::num::NonZeroU32;

use simple_logger::SimpleLogger;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() -> Result<(), impl std::error::Error> {
    SimpleLogger::new().init().unwrap();

    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new()
        .with_title("tterm")
        .build(&event_loop)
        .unwrap();

    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();

    // elwt == event loop window target
    event_loop.run(move |event, _, control_flow| {
        // Poll continuously
        control_flow.set_wait();
        println!("{event:?}");

        match event {
            Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => control_flow.set_exit(),
                WindowEvent::KeyboardInput {
                    device_id,
                    event,
                    is_synthetic,
                } => window.request_redraw(),
                _ => (),
            },

            Event::RedrawRequested(_) => {
                //notify windowing system that we'll be presenting to the window
                window.pre_present_notify();

                let (width, height) = {
                    let size = window.inner_size();
                    (size.width, size.height)
                };

                surface
                    .resize(
                        NonZeroU32::new(width).unwrap(),
                        NonZeroU32::new(height).unwrap(),
                    )
                    .unwrap();
                let mut buffer = surface.buffer_mut().unwrap();
                buffer.fill(0x00181818);

                buffer.present().unwrap();
                println!("redrawing");
            }
            _ => (),
        };
    })
}
