use glfw::*;

fn main() {
    let (mut glfw, mut window, events) = create_window_context();

    const TARGET_FPS: f64 = 60.;
    const OPTIMAL_TIME: f64 = 1. / TARGET_FPS;
    let mut start = glfw.get_time();
    let mut end;
    let mut delta = 0.;

    let mut timer = start;
    let mut fps: u8 = 0;

    window.show();

    'main: loop {
        end = start;
        start = glfw.get_time();
        delta += start - end;

        if delta >= OPTIMAL_TIME {
            glfw.poll_events();
            for (_, event) in flush_messages(&events) {
                match event {
                    WindowEvent::Close => break 'main,
                    _ => println!("WHATHELL?!?!?!?!?!"),
                }
            }
            window.swap_buffers();

            delta -= OPTIMAL_TIME;
            fps += 1;
        }

        if timer - start >= 1. {
            println!("FPS: {fps}");
            timer += 1.;
            fps = 0;
        }
    }
}

fn create_window_context() -> (Glfw, PWindow, GlfwReceiver<(f64, WindowEvent)>) {
    let mut glfw = init(fail_on_errors!()).expect("Could not initialize GLFW");

    glfw.default_window_hints();
    glfw.window_hint(WindowHint::Visible(false));
    glfw.window_hint(WindowHint::ContextVersion(4, 3));
    glfw.window_hint(WindowHint::Resizable(false));

    let (mut window, events) = glfw
        .create_window(1280, 720, "Beaaaa", WindowMode::Windowed)
        .expect("Could not create the GLFW Window");

    window.set_close_polling(true);

    gl::load_with(|fn_name| window.get_proc_address(fn_name));

    (glfw, window, events)
}
