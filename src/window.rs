use pixels::{SurfaceTexture, PixelsBuilder};
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{EventLoop, ControlFlow};
#[cfg(windows)]
use winit::platform::windows::WindowBuilderExtWindows;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use crate::cpu::Cpu;
use crate::display;
use crate::keyboard;
use pixels::wgpu::PresentMode;

pub fn create_window(mut cpu: Cpu) {
    let ev_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let multiplier = 15;
        let size = LogicalSize::new(display::WIDTH as u32 * multiplier, display::HEIGHT as u32 * multiplier);
        let mut builder = WindowBuilder::new()
            .with_title("CHIP-8 Interpreter")
            .with_inner_size(size)
            .with_min_inner_size(size);

        #[cfg(windows)]
        {
            builder = builder.with_drag_and_drop(false);
        }

        builder
            .build(&ev_loop)
            .unwrap()
    };

    let mut pixels = {
        let inner = window.inner_size();
        let texture = SurfaceTexture::new(inner.width, inner.height, &window);

        PixelsBuilder::new(display::WIDTH as u32, display::HEIGHT as u32, texture)
            .present_mode(PresentMode::Mailbox)
            .build()
            .unwrap()
    };

    ev_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawRequested(_) | Event::MainEventsCleared => {
                // draw screen
                let frame = pixels.get_frame();
                let display = cpu.get_display();

                for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                    let new_pixel = if display.is_set(i) {
                        [255, 255, 255, 255]
                    } else {
                        [20, 20, 20, 255]
                    };

                    pixel.copy_from_slice(&new_pixel);
                }

                pixels.render().unwrap();
                window.request_redraw();
            }
            _ => {}
        }

        // handle input events
        if input.update(&event) {
            let keyboard = cpu.get_keyboard();

            // close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // window resizing
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }

            // other key inputs
            for key in 0x0..=0xF_u8 {
                if let Some(key_code) = keyboard::get_keycode_from_key(key) {
                    if input.key_pressed(key_code) {
                        keyboard.set_key_pressed(key, true);
                    } else if input.key_released(key_code) {
                        keyboard.set_key_pressed(key, false);
                    }
                }
            }
        }

        // interpreter tick
        cpu.tick();
    });
}