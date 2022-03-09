use std::ffi::{CString};
use std::path::Path;

use crate::glad_gl::gl;

pub struct Shader {
    pub id: u32
}

pub enum ShaderContent<'a> {
    Fragment (&'a Path, CString),
    Vertex (&'a Path, CString)
}

fn load_file_to_cstr(path: &Path) -> Result<CString, String> {
    let content = std::fs::read_to_string(path);
    let file_string = match content {
        Ok (s) => Ok(s),
        Err (e) => Err(e.to_string())
    }?;
    let cstr = CString::new(file_string.to_owned());
    match cstr {
        Ok (cs) => Ok (cs.clone()),
        Err (e) => Err (e.to_string())
    }
}

unsafe fn create_and_compile_shader(content: ShaderContent) -> Result<u32, String> {
    let (shader_handle, path, code, msg_part) = match content {
        ShaderContent::Vertex (p, c) => (gl::CreateShader(gl::VERTEX_SHADER), p, c, "Vertex"),
        ShaderContent::Fragment (p, c) => (gl::CreateShader(gl::FRAGMENT_SHADER), p, c, "Fragment")
    };
    gl::ShaderSource(
        shader_handle,
        1,
        &code.as_ptr(),
        std::ptr::null(),
    );
    gl::CompileShader(shader_handle);

    let mut success: i32 = 0;
    gl::GetShaderiv(shader_handle, gl::COMPILE_STATUS, &mut success);

    if success == 0 {
        let raw_log = CString::default().into_raw();
        let mut log_length: i32 = 0;
        gl::GetShaderInfoLog(shader_handle, 512, &mut log_length, raw_log);
        let log = CString::from_raw(raw_log);
        Err(format!("{} Shader [{}] failed compile: {}", msg_part, path.display(), log.into_string().unwrap()))
    } else {
        Ok(shader_handle)
    }
}

impl Shader {
    pub fn new<'a>(vertex_path: &'a Path, fragment_path: &'a Path) -> Result<Shader, String> {
        unsafe {
            let vertex = ShaderContent::Vertex (vertex_path, load_file_to_cstr(vertex_path)?);
            let vertex_handle = create_and_compile_shader(vertex)?;

            let fragment = ShaderContent::Fragment (fragment_path, load_file_to_cstr(fragment_path)?);
            let fragment_handle = create_and_compile_shader(fragment)?;

            let program_handle = gl::CreateProgram();
            gl::AttachShader(program_handle, vertex_handle);
            gl::AttachShader(program_handle, fragment_handle);
            gl::LinkProgram(program_handle);
    
            let mut success: i32 = 0;
            gl::GetProgramiv(program_handle, gl::LINK_STATUS, &mut success);
    
            gl::DeleteShader(vertex_handle);
            gl::DeleteShader(fragment_handle);
    
            if success == 0 {
                let raw_log = CString::default().into_raw();
                let mut log_length: i32 = 0;
                gl::GetProgramInfoLog(program_handle, 512, &mut log_length, raw_log);
                let log = CString::from_raw(raw_log);
                Err (format!("Program Link Failure: {}", log.into_string().unwrap()))
            } else {
                Ok (Shader {
                    id: program_handle
                })
            }
        }
    }

    pub fn activate(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}