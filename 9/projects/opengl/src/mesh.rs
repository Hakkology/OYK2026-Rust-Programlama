use gl::types::*;
use std::ptr;

pub struct Mesh {
    vao: u32,
    vbo: u32,
    vertex_count: i32,
}

impl Mesh {
    /// 2D varsayılan bir üçgen (Triangle) mesh'i oluşturur
    pub fn new_triangle() -> Self {
        let vertices: [f32; 9] = [
            -0.5, -0.5, 0.0, // Sol Alt
             0.5, -0.5, 0.0, // Sağ Alt
             0.0,  0.5, 0.0, // Üst Orta
        ];

        let (mut vao, mut vbo) = (0, 0);

        unsafe {
            // VAO ve VBO oluştur
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            // VAO ve VBO'yu GPU bağlamına bind et
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            // Köşe koordinat verisini GPU'ya aktar
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<GLfloat>()) as GLsizeiptr,
                vertices.as_ptr() as *const _,
                gl::STATIC_DRAW,
            );

            // Köşe verisinin nitelik (attribute) düzenini belirt (layout 0: vec3)
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (3 * std::mem::size_of::<GLfloat>()) as GLsizei,
                ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            // Bind işlemini sıfırla
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }

        Mesh {
            vao,
            vbo,
            vertex_count: 3,
        }
    }

    /// Mesh'i ekrana çizer
    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertex_count);
        }
    }

    /// GPU üzerindeki VAO ve VBO kaynaklarını temizler
    pub fn cleanup(&self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}
