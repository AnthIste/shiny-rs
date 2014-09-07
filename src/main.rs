#![feature(phase)]
#![crate_name = "shiny"]

extern crate gfx;
#[phase(plugin)]
extern crate gfx_macros;
extern crate glfw;
extern crate native;

use glfw::Context;

use self::util::time::{FixedTimestep, FpsCounter};
use self::scene::Scene;
use self::simulation::MySimulation;

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
    let mut scene = Scene::new(&mut graphics);
    let mut simulation = MySimulation::new();

    // Time progression for simulation
    let updates_hz = 30.0f32;
    let update_time_s = 1.0f32 / updates_hz;
    let update_time_ns = update_time_s * 1000000000f32;

    let mut timestep = FixedTimestep::new(update_time_ns as u64);
    let mut timestep_fps = FixedTimestep::new(1000000000u64);
    let mut fps_counter = FpsCounter::new();

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
                    simulation.emit_triangles();
                },
                _ => {}
            }
        }

        timestep.tick(|_t: u64, dt: u64| {
            let dt_s = dt as f32 / 1000000000f32;
            simulation.update(dt_s);
        });

        timestep_fps.tick(|_t: u64, _dt: u64| {
            let fps = fps_counter.fps();
            println!("FPS: {0}, Entities: {1}", fps, simulation.triangles().len());
        });

        graphics.clear(clear_data, &frame);
        scene.render(&mut graphics, &frame, &simulation);
        graphics.end_frame();

        fps_counter.frame();
        window.swap_buffers();
    }
}