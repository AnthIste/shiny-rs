#![feature(phase)]
#![crate_name = "shiny"]

extern crate gfx;
#[phase(plugin)]
extern crate gfx_macros;
extern crate glfw;
extern crate native;

use glfw::Context;
use scene::Scene;
use simulation::MySimulation;

mod util;
mod simulation;
mod scene;

// We need to run on the main thread for GLFW, so ensure we are using the `native` runtime. This is
// technically not needed, since this is the default, but it's not guaranteed.
#[start]
fn start(argc: int, argv: *const *const u8) -> int {
     native::start(argc, argv, main)
}

fn main() {
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::ContextVersion(3, 2));
    glfw.window_hint(glfw::OpenglForwardCompat(true));
    glfw.window_hint(glfw::OpenglProfile(glfw::OpenGlCoreProfile));

    let (window, events) = glfw
        .create_window(640, 480, "Triangle example.", glfw::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    glfw.set_error_callback(glfw::FAIL_ON_ERRORS);
    window.set_key_polling(true);

    let (w, h) = window.get_framebuffer_size();
    let frame = gfx::Frame::new(w as u16, h as u16);

    let device = gfx::GlDevice::new(|s| glfw.get_proc_address(s));
    let mut graphics = gfx::Graphics::new(device);

    let clear_data = gfx::ClearData {
        color: Some([0.3, 0.3, 0.3, 1.0]),
        depth: None,
        stencil: None,
    };

    // The meat and potatos: what we are drawing (simulation) and how we draw it (scene)
    let (mut scene, mut simulation) = init();

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                // Escape to close
                glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _) => {
                    window.set_should_close(true);
                },

                // Space to spawn a new particle
                glfw::KeyEvent(glfw::KeySpace, _, glfw::Press, _) => {
                    simulation.spawn();
                },
                _ => {}
            }
        }

        simulation.update(0.1f32);

        graphics.clear(clear_data, &frame);
        scene.render(&mut graphics, &frame, &simulation);
        graphics.end_frame();

        window.swap_buffers();
    }
}

fn init<D: gfx::Device<C>, C: gfx::CommandBuffer>() -> (Scene<D, C>, MySimulation) {
    let scene = Scene::new();
    let simulation = MySimulation::new();

    (scene, simulation)
}