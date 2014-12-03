#![feature(phase)]
#![crate_name = "shiny"]

extern crate gfx;
#[phase(plugin)]
extern crate gfx_macros;
extern crate glfw;
// extern crate native;

use glfw::Context;

use self::util::time::{FixedTimestep, FpsCounter, ToNanoSeconds, ToSeconds};
use self::scene::Scene;
use self::simulation::MySimulation;

mod util;
mod simulation;
mod scene;

// We need to run on the main thread for GLFW, so ensure we are using the `native` runtime. This is
// technically not needed, since this is the default, but it's not guaranteed.
// #[start]
// fn start(argc: int, argv: *const *const u8) -> int {
//      native::start(argc, argv, main)
// }

fn main() {
    let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 2));
    glfw.window_hint(glfw::WindowHint::OpenglForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenglProfile(glfw::OpenGlProfileHint::Core));

    let (window, events) = glfw
        .create_window(640, 480, "Triangle example.", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    window.make_current();
    glfw.set_swap_interval(1); // vsync = on
    glfw.set_error_callback(glfw::FAIL_ON_ERRORS);
    window.set_key_polling(true);

    let (w, h) = window.get_framebuffer_size();
    let frame = gfx::Frame::new(w as u16, h as u16);

    let device = gfx::GlDevice::new(|s| glfw.get_proc_address(s));
    let mut graphics = gfx::Graphics::new(device);

    let clear_data = gfx::ClearData {
        color: [0.3f32, 0.3, 0.3, 1.0],
        depth: 0.0f32,
        stencil: 0,
    };

    // The meat and potatos: what we are drawing (simulation) and how we draw it (scene)
    let mut scene = Scene::new(&mut graphics);
    let mut simulation = MySimulation::new();

    // Time progression for simulation
    let updates_hz = 30.0f32;
    let update_time_s = 1.0f32 / updates_hz;

    let mut timestep = FixedTimestep::new(update_time_s.to_nanoseconds());
    let mut timestep_fps = FixedTimestep::new(1_000_000_000u64);
    let mut fps_counter = FpsCounter::new();

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                // Escape to close
                glfw::WindowEvent::Key(glfw::Key::Escape, _, glfw::Action::Press, _) => {
                    window.set_should_close(true);
                },

                // Space to spawn a new particle
                glfw::WindowEvent::Key(glfw::Key::Space, _, glfw::Action::Press, _) => {
                    simulation.emit_triangles();
                },

                _ => { }
            }
        }

        timestep.tick(|_t, dt| {
            simulation.update(dt.to_seconds());
        });

        timestep_fps.tick(|_t, _dt| {
            let fps = fps_counter.fps();
            println!("FPS: {0}, Entities: {1}", fps, simulation.triangles().len());
        });

        graphics.clear(clear_data, gfx::Mask::empty(), &frame);
        scene.render(&mut graphics, &frame, &simulation);
        graphics.end_frame();

        fps_counter.frame();
        window.swap_buffers();
    }
}