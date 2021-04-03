use winit::dpi::LogicalSize;
use winit::window::WindowBuilder;
use winit::event_loop::{EventLoop, ControlFlow};
use winit_input_helper::WinitInputHelper;
use pixels::{SurfaceTexture, Pixels};
use winit::event::{Event, VirtualKeyCode};
use crate::cpu::Cpu;

const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;

pub fn create_window(mut cpu: Cpu) {
    let ev_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(WIDTH, HEIGHT);
        WindowBuilder::new()
            .with_title("CHIP-8 Interpreter")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&ev_loop)
            .unwrap()
    };

    let mut pixels = {
        let inner = window.inner_size();
        let texture = SurfaceTexture::new(inner.width, inner.height, &window);
        Pixels::new(WIDTH, HEIGHT, texture).unwrap()
    };

    ev_loop.run(move |event, _, control_flow| {
        // draw frame
        if let Event::RedrawRequested(_) = event {
            let frame = pixels.get_frame();

            for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                pixel[0] = 255;
                pixel[1] = 255;
                pixel[2] = 0;
                pixel[3] = 255;
            }

            pixels.render().unwrap();
        }

        // handle input events
        if input.update(&event) {
            // close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            // window resizing
            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }

            window.request_redraw();
        }

        // interpreter tick
        cpu.tick();
    });
}