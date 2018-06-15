extern crate glutin;

mod support;

use glutin::GlContext;

fn main() {
    // Create a new headless context
    let context_builder = glutin::ContextBuilder::new();
    let headless_context = glutin::Context::new(context_builder, true).unwrap();

    // Create a windowed context sharing objects with the headless context
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new().with_title("Windowed Context");
    let context = glutin::ContextBuilder::new().with_shared_lists(&headless_context);
    let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();

    // Load something with the headless context
    let _ = unsafe { headless_context.make_current() };
    let gl = support::load(&headless_context);

    // Switch to the windowed context
    let _ = unsafe { gl_window.context().make_current() };

    gl.reinit();

    events_loop.run_forever(|event| {
        match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::CloseRequested => return glutin::ControlFlow::Break,
                glutin::WindowEvent::Resized(w, h) => gl_window.resize(w, h),
                _ => (),
            },
            _ => ()
        }

        gl.draw_frame([0.0, 1.0, 0.0, 1.0]);
        let _ = gl_window.swap_buffers();
        glutin::ControlFlow::Continue
    });
}