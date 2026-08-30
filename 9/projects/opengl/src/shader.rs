use gl::types::*;
use std::ffi::CString;
use std::ptr;
use std::str;

// Vertex Shader: Köşe noktalarının konumunu belirler
pub const VERTEX_SHADER_SRC: &str = r#"
#version 330 core
layout (location = 0) in vec3 aPos;

void main() {
    gl_Position = vec4(aPos.x, aPos.y, aPos.z, 1.0);
}
"#;

// Fragment Shader: Piksel/Yüzey rengini belirler (Turuncu)
pub const FRAGMENT_SHADER_SRC: &str = r#"
#version 330 core
out vec4 FragColor;

void main() {
    FragColor = vec4(1.0, 0.5, 0.2, 1.0);
}
"#;

pub struct Shader {
    pub id: u32,
}

impl Shader {
    /// Varsayılan 2D üçgen shader programını derler ve oluşturur
    pub fn default_triangle_shader() -> Result<Self, String> {
        Self::new(VERTEX_SHADER_SRC, FRAGMENT_SHADER_SRC)
    }
    /// Vertex ve Fragment shader kaynak kodlarını derler ve bağlar (Link)
    pub fn new(vertex_src: &str, fragment_src: &str) -> Result<Self, String> {
        unsafe {
            // 1. Vertex Shader Derle
            let vert_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let c_vert = CString::new(vertex_src.as_bytes()).unwrap();
            gl::ShaderSource(vert_shader, 1, &c_vert.as_ptr(), ptr::null());
            gl::CompileShader(vert_shader);
            Self::check_compile_errors(vert_shader, "VERTEX")?;

            // 2. Fragment Shader Derle
            let frag_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            let c_frag = CString::new(fragment_src.as_bytes()).unwrap();
            gl::ShaderSource(frag_shader, 1, &c_frag.as_ptr(), ptr::null());
            gl::CompileShader(frag_shader);
            Self::check_compile_errors(frag_shader, "FRAGMENT")?;

            // 3. Shader Programına Bağla (Link)
            let program_id = gl::CreateProgram();
            gl::AttachShader(program_id, vert_shader);
            gl::AttachShader(program_id, frag_shader);
            gl::LinkProgram(program_id);
            Self::check_link_errors(program_id)?;

            // Geçici shader objelerini sil
            gl::DeleteShader(vert_shader);
            gl::DeleteShader(frag_shader);

            Ok(Shader { id: program_id })
        }
    }

    /// Bu shader programını çizim için aktif hale getirir
    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    /// GPU üzerindeki program kaynağını temizler
    pub fn cleanup(&self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }

    unsafe fn check_compile_errors(shader: u32, shader_type: &str) -> Result<(), String> {
        let mut success = gl::FALSE as GLint;
        let mut info_log = vec![0u8; 512];
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
        if success == gl::FALSE as GLint {
            gl::GetShaderInfoLog(
                shader,
                512,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            let err = str::from_utf8(&info_log).unwrap_or("Bilinmeyen Hata");
            Err(format!("{} Shader Derleme Hatası: {}", shader_type, err))
        } else {
            Ok(())
        }
    }

    unsafe fn check_link_errors(program: u32) -> Result<(), String> {
        let mut success = gl::FALSE as GLint;
        let mut info_log = vec![0u8; 512];
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        if success == gl::FALSE as GLint {
            gl::GetProgramInfoLog(
                program,
                512,
                ptr::null_mut(),
                info_log.as_mut_ptr() as *mut GLchar,
            );
            let err = str::from_utf8(&info_log).unwrap_or("Bilinmeyen Hata");
            Err(format!("Shader Link Hatası: {}", err))
        } else {
            Ok(())
        }
    }
}
