use std::ffi::CString;

use gl::types::{GLuint, GLint};

use crate::base::{shader::Shader, error::ShaderError, utility::{self, CheckType}};

pub struct ShaderProgram {
    pub id: GLuint,
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id); }
    }
}

#[allow(dead_code)]
impl ShaderProgram {
    pub unsafe fn new(file_vs: &str, file_fs: &str) -> Result<Self, ShaderError> {
        let program = Self { id: gl::CreateProgram() };

        let vertex_shader = Shader::new(file_vs, gl::VERTEX_SHADER)?;
        let fragment_shader = Shader::new(file_fs, gl::FRAGMENT_SHADER)?;

        gl::AttachShader(program.id, vertex_shader.id);
        gl::AttachShader(program.id, fragment_shader.id);
        
        gl::LinkProgram(program.id);

        utility::check_compile_error(program.id, CheckType::Program)?;

        program.apply();
        
        Ok(program)
    }

    pub unsafe fn apply(&self) { gl::UseProgram(self.id); }

    // 获取属性对应的location ID
    pub unsafe fn get_attr_location(&self, attr: &str) -> Result<GLuint, ShaderError> {
        let cs_attr = CString::new(attr)?;
        let ret = gl::GetAttribLocation(self.id, cs_attr.as_ptr());
        match ret {
            -1 => Err(ShaderError::AttributeLocationParseError(attr.into())),
            value => Ok(value as GLuint)
        }
    }

    // 获取uniform变量的location ID
    pub unsafe fn get_uniform_location(&self, name: &str) -> Result<GLint, ShaderError> {
        let cs_name = CString::new(name)?;
        let ret = gl::GetUniformLocation(self.id, cs_name.as_ptr());
        match ret {
            -1 => Err(ShaderError::UniformLocationParseError(name.into())),
            value => Ok(value)
        }
    }

    pub unsafe fn set_int(&self, name: &str, value: i32) -> Result<(), ShaderError> {
        self.apply();
        gl::Uniform1i(self.get_uniform_location(name)?, value);
        Ok(())
    }

    pub unsafe fn set_float(&self, name: &str, value: f32) -> Result<(), ShaderError> {
        self.apply();
        gl::Uniform1f(self.get_uniform_location(name)?, value);
        Ok(())
    }

    pub unsafe fn set_mat4(&self, name: &str, value: &[f32]) -> Result<(), ShaderError> {
        self.apply();
        gl::UniformMatrix4fv(self.get_uniform_location(name)?, 1, gl::FALSE, &value[0]);
        Ok(())
    }

    pub unsafe fn set_vec3(&self, name: &str, value: &[f32]) -> Result<(), ShaderError> {
        self.apply();
        gl::Uniform3fv(self.get_uniform_location(name)?, 1, &value[0]);
        Ok(())
    }

    pub unsafe fn set_vec4(&self, name: &str, value: &[f32]) -> Result<(), ShaderError> {
        self.apply();
        gl::Uniform4fv(self.get_uniform_location(name)?, 1, &value[0]);
        Ok(())
    }
}