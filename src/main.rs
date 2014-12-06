#![feature(phase)]
#![crate_name = "shiny"]

extern crate shader_version;
extern crate current;
extern crate event;
extern crate gfx;
#[phase(plugin)]
extern crate gfx_macros;
extern crate sdl2;
extern crate sdl2_window;

use current::{ Set };
use std::cell::RefCell;
use sdl2_window::Sdl2Window;
use gfx::{ Device, DeviceHelper, ToSlice };
use event::{ Events, WindowSettings };
use event::window::{ CaptureCursor };

fn main() {
    let (win_width, win_height) = (640, 480);
    let mut window = Sdl2Window::new(
        shader_version::opengl::OpenGL::OpenGL_3_2,
        WindowSettings {
            title: "Shiny".to_string(),
            size: [win_width, win_height],
            fullscreen: false,
            exit_on_esc: true,
            samples: 4,
        }
    );

    window.set_mut(CaptureCursor(true));

    let mut device = gfx::GlDevice::new(|s| unsafe {
        std::mem::transmute(sdl2::video::gl_get_proc_address(s))
    });
    let frame = gfx::Frame::new(win_width as u16, win_height as u16);

    let mut graphics = gfx::Graphics::new(device);

    let window = RefCell::new(window);
    for e in Events::new(&window) {
        use event::RenderEvent;

        e.render(|args| {
            graphics.clear(
                gfx::ClearData {
                    color: [0.3, 0.3, 0.3, 1.0],
                    depth: 1.0,
                    stencil: 0,
                },
                gfx::COLOR | gfx::DEPTH,
                &frame
            );

            graphics.end_frame();
        });
    }
}