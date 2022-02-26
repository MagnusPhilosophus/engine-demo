use std::error::Error;

pub struct Program{
    pub id: gl::types::GLuint
}

impl Program {
    pub fn from_shaders(vss: &str, fss: &str) -> Result<Self, Box<dyn Error>>{ 
        let vs = shader_from_source(vss, gl::VERTEX_SHADER)?;
        let fs = shader_from_source(fss, gl::FRAGMENT_SHADER)?;
        let id = unsafe{ gl::CreateProgram() };
        unsafe {
            gl::AttachShader(id, vs);
            gl::AttachShader(id, fs);
            gl::LinkProgram(id);
            gl::DeleteShader(vs);
            gl::DeleteShader(fs);
        }
        Ok(Program { id })
    }
}

fn shader_from_source(source: &str, kind: gl::types::GLenum) -> Result<gl::types::GLuint, Box<dyn Error>> {
    let mut file = std::fs::File::open(env!("CARGO_MANIFEST_DIR").to_owned() + source)?;
    let mut buffer = String::new();
    use std::io::Read;
    file.read_to_string(&mut buffer)?;
    let source = std::ffi::CString::new(buffer)?;
    let id = unsafe{ gl::CreateShader(kind) };
    unsafe {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }
    Ok(id)
}
