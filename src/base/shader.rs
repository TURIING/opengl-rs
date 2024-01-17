use std::{fs, ptr, ffi::CString};

use gl::types::{GLuint, GLenum };

use crate::base::{error::ShaderError, utility::{self, CheckType}};

pub struct Shader {
    pub id: GLuint,
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.id); }
    }
}

impl Shader {
    pub unsafe fn new(source_path: &str, shader_type: GLenum) -> Result<Self, ShaderError> {
        let source = CString::new(fs::read_to_string(source_path)?)?;
        let shader = Self{ id: gl::CreateShader(shader_type) };

        gl::ShaderSource(shader.id, 1, &source.as_ptr(), ptr::null());
        gl::CompileShader(shader.id);

        utility::check_compile_error(shader.id, CheckType::Shader)?;

        Ok(shader)
    }

    
}