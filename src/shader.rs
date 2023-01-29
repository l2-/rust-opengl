const VERT_SHADER: &str = r#"#version 330 core
  layout (location = 0) in vec3 pos;
  void main() {
    gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
  }
"#;

const FRAG_SHADER: &str = r#"#version 330 core
  out vec4 frag;
  void main() {
    frag = vec4(1.0, 0.5, 0.2, 1.0);
  }
"#;

#[derive(Debug)]
pub struct Shader {
    pub shader_program: u32
}

impl Shader {
    pub fn create(v: &str, f: &str) -> Shader {
        unsafe {
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            assert_ne!(vertex_shader, 0);
            gl::ShaderSource(vertex_shader, 1, &(v.as_bytes().as_ptr().cast()), &(v.len().try_into().unwrap()));
            gl::CompileShader(vertex_shader);
            let mut success = 0;
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetShaderInfoLog(vertex_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
            }
    
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            assert_ne!(fragment_shader, 0);
            gl::ShaderSource(fragment_shader, 1, &(f.as_bytes().as_ptr().cast()), &(f.len().try_into().unwrap()));
            gl::CompileShader(fragment_shader);
            let mut success = 0;
            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetShaderInfoLog(fragment_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
            }
    
            let shader_program = gl::CreateProgram();
            assert_ne!(shader_program, 0);
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);
            let mut success = 0;
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetProgramInfoLog(shader_program, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("Program Link Error: {}", String::from_utf8_lossy(&v));
            }
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
    
            gl::UseProgram(shader_program);
            return Shader { shader_program: shader_program };
        }
    }

    pub fn _create() -> Shader {
        return Shader::create(VERT_SHADER,  FRAG_SHADER);
    }
}