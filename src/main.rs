pub mod font_loader;

use std::{collections::HashMap, num::NonZeroU32};

use font_loader::Glyph;
use simple_logger::SimpleLogger;
use winit::{
    event::{Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
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

    let glyphs = font_loader::get_glyph_table();

    let mut input_buffer = String::new();

    event_loop.run(move |event, _, control_flow| {
        // Run loop only when there are events happening
        control_flow.set_wait();

        match event {
            Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => control_flow.set_exit(),
                WindowEvent::KeyboardInput { event, .. } => {
                    if handle_key_press(&mut input_buffer, event) {
                        window.request_redraw();
                    };
                }

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

                draw_string(&glyphs, &mut buffer, &window, &mut input_buffer);

                buffer.present().unwrap();
            }
            _ => (),
        };
    })
}

fn draw_string(
    glyphs: &HashMap<char, Glyph>,
    output_buffer: &mut softbuffer::Buffer,
    window: &Window,
    input_buffer: &mut str,
) {
    let mut offset = 0;
    let max_glyph_height = font_loader::get_max_glyph_height(glyphs);

    for ch in input_buffer.chars() {
        if let Some(glyph) = glyphs.get(&ch) {
            draw_glyph(glyph, output_buffer, window, offset, max_glyph_height);
            offset += glyph.metrics.width;
        }
    }
}

fn draw_glyph(
    glyph: &Glyph,
    output_buffer: &mut softbuffer::Buffer,
    window: &Window,
    mut row_offset: usize,
    max_glyph_height: usize,
) {
    let glyph_width = glyph.metrics.width;
    let window_width = window.inner_size().width as usize;
    row_offset += window_width * (max_glyph_height - glyph.metrics.height);

    for (index, byte) in glyph.glyph_bytes.iter().enumerate() {
        if *byte > 0 {
            let buffer_index = index + row_offset;

            if buffer_index < output_buffer.len() {
                // Color format: 0000 0000 RRRR RRRR GGGG GGGG BBBB BBBB
                output_buffer[buffer_index] =
                    *byte as u32 | ((*byte as u32) << 8) | ((*byte as u32) << 16);
            }
        }

        // Update offset when reaching end of glyph row
        if index > 0 && index % glyph_width == 0 {
            row_offset += window_width - glyph_width;
        }
    }
}

// Returns true if key event should trigger a re-render
fn handle_key_press(input_buffer: &mut String, event: KeyEvent) -> bool {
    if event.state.is_pressed() {
        return match event.logical_key {
            winit::keyboard::Key::Alphanumeric => todo!(),
            winit::keyboard::Key::Alt => todo!(),
            winit::keyboard::Key::AltGraph => todo!(),
            winit::keyboard::Key::Control => todo!(),
            winit::keyboard::Key::Enter => {
                send_input(input_buffer);
                true
            }
            winit::keyboard::Key::Tab => todo!(),
            winit::keyboard::Key::ArrowDown => todo!(),
            winit::keyboard::Key::ArrowLeft => todo!(),
            winit::keyboard::Key::ArrowRight => todo!(),
            winit::keyboard::Key::ArrowUp => todo!(),
            winit::keyboard::Key::Backspace => {
                println!("popping");
                println!("{input_buffer}");
                input_buffer.pop();
                true
            }
            winit::keyboard::Key::Clear => todo!(),
            winit::keyboard::Key::Copy => todo!(),
            winit::keyboard::Key::Cut => todo!(),
            winit::keyboard::Key::Delete => todo!(),
            winit::keyboard::Key::Insert => todo!(),
            winit::keyboard::Key::Paste => todo!(),
            winit::keyboard::Key::Redo => todo!(),
            winit::keyboard::Key::Undo => todo!(),
            winit::keyboard::Key::Escape => todo!(),
            winit::keyboard::Key::Execute => todo!(),
            _ => {
                if let Some(text) = event.text {
                    println!("{}", text);
                    input_buffer.push_str(text.as_str());
                    true
                } else {
                    false
                }
            }
        };
    }

    false
}

fn send_input(_input: &mut str) {
    todo!()
}
