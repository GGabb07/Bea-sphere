use glfw::*;

fn main() {
    let (mut glfw, mut window, events) = create_window_context();

    window.show();

    'main: loop {
        glfw.poll_events();
        for (_, event) in flush_messages(&events) {
            match event {
                WindowEvent::Close => break 'main,
                _ => println!("WHATHELL?!?!?!?!?!"),
            }
        }
        window.swap_buffers();
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
