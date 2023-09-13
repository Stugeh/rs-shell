pub mod font_loader;

use std::{num::NonZeroU32, collections::HashMap};

use font_loader::font_loader::Glyph;
use simple_logger::SimpleLogger;
use winit::{
    event::{Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    window::{WindowBuilder, Window},
};

//***TODO FIX draw glyph math. Dont redraw everything every time theres an event, add positioning to glyphs***//
fn main() -> Result<(), impl std::error::Error> {
    SimpleLogger::new().init().unwrap();

    let event_loop = EventLoop::new().unwrap();

    let window = WindowBuilder::new()
        .with_title("tterm")
        .build(&event_loop)
        .unwrap();

    let context = unsafe { softbuffer::Context::new(&window) }.unwrap();
    let mut surface = unsafe { softbuffer::Surface::new(&context, &window) }.unwrap();

    let glyphs = font_loader::font_loader::get_glyph_table();

    let mut input_buffer = String::new();

    event_loop.run(move |event, _, control_flow| {
        // Run loop only when there are events happening
        control_flow.set_wait();

        match event {
            Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => control_flow.set_exit(),
                WindowEvent::KeyboardInput { event, .. } => {
                    handle_key_press(&mut input_buffer, event)
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

                draw_string(&glyphs, &mut buffer, &window);
                buffer.present().unwrap();
                println!("redrawing");
            }
            _ => (),
        };
    })
}

fn draw_string(glyphs: &HashMap<char, Glyph>, output_buffer: &mut softbuffer::Buffer, window: &Window){
    let hello = String::from("Hello World!");
    let mut offset = 0;

    for ch in hello.chars(){
        let current_glyph = glyphs.get(&ch).expect("invalid char");

        draw_glyph(current_glyph, output_buffer, window, offset );
        offset += current_glyph.metrics.width;
    }
}

fn draw_glyph(glyph: &Glyph, output_buffer: &mut softbuffer::Buffer, window: &Window, mut row_offset: usize){
    println!("{}", glyph.metrics.width);

    let glyph_width = glyph.metrics.width as usize;
    let window_width = window.inner_size().width as usize;

    for (index,byte) in glyph.glyph_bytes.iter().enumerate() {
        if *byte > 0{
            let buffer_index = index + row_offset;

            if buffer_index < output_buffer.len() {
                // Color format: 0000 0000 RRRR RRRR GGGG GGGG BBBB BBBB
                output_buffer[buffer_index] = *byte as u32| (( *byte as u32 ) << 8)| (( *byte as u32 ) << 16);
            }
        }

        // Update offset when reaching end of glyph row
        if index>0 && index % glyph_width == 0 {
            row_offset += window_width - glyph_width;
        }
    }
}


fn handle_key_press(input_buffer: &mut String, event: KeyEvent) {
    if event.state.is_pressed() {
        if let Some(text) = event.text {
            input_buffer.push_str(text.as_str());
            println!("pressed: {}", text);
            println!("input buffer: {}", input_buffer);
        }
    }
}
