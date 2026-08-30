mod mesh;
mod shader;
mod window;

use glfw::Context;
use mesh::Mesh;
use shader::Shader;

fn main() {
    // 1. Pencereyi Oluştur (window modülü)
    let (mut glfw, mut window, events) = window::init_window(800, 600, "Modüler OpenGL Üçgeni");

    // 2. Shader'ları Derle ve Bağla (shader modülü içindeki sabitlerle)
    let shader = Shader::default_triangle_shader()
        .expect("Shader derleme hatası oluştu!");

    // 3. Üçgen Mesh'ini Hazırla (mesh modülü)
    let triangle = Mesh::new_triangle();

    println!("Modüler OpenGL uygulaması başlatıldı!");

    // 4. Ana Render Döngüsü
    while !window.should_close() {
        // Girdileri ve olayları işle
        glfw.poll_events();
        window::process_events(&mut window, &events);

        // Ekranı koyu griye temizle
        window::clear(0.1, 0.1, 0.12);

        // Shader'ı aktifleştir ve üçgeni çiz
        shader.bind();
        triangle.draw();

        // Çift tamponlamayı takas et (Swap Buffers)
        window.swap_buffers();
    }

    // 5. Kaynakları Temizle
    triangle.cleanup();
    shader.cleanup();
}
