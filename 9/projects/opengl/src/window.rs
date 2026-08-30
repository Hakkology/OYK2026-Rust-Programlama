use glfw::{Action, Context, GlfwReceiver, Key, PWindow, WindowEvent};

/// GLFW ve OpenGL 3.3 Core profilinde pencere başlatır
pub fn init_window(
    width: u32,
    height: u32,
    title: &str,
) -> (glfw::Glfw, PWindow, GlfwReceiver<(f64, WindowEvent)>) {
    let mut glfw = glfw::init(glfw::fail_on_errors).expect("GLFW ilklendirilemedi!");

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    let (mut window, events) = glfw
        .create_window(width, height, title, glfw::WindowMode::Windowed)
        .expect("Pencere oluşturulamadı!");

    window.make_current();
    window.set_key_polling(true);
    window.set_framebuffer_size_polling(true);

    // OpenGL fonksiyon göstergelerini yükle
    gl::load_with(|s| window.get_proc_address(s) as *const _);

    unsafe {
        gl::Viewport(0, 0, width as i32, height as i32);
    }

    (glfw, window, events)
}

/// Klavye ve pencere boyutlandırma olaylarını işler
pub fn process_events(
    window: &mut PWindow,
    events: &GlfwReceiver<(f64, WindowEvent)>,
) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true);
            }
            WindowEvent::FramebufferSize(width, height) => unsafe {
                gl::Viewport(0, 0, width, height);
            },
            _ => {}
        }
    }
}

/// Ekran arka plan rengini temizler
pub fn clear(r: f32, g: f32, b: f32) {
    unsafe {
        gl::ClearColor(r, g, b, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}
