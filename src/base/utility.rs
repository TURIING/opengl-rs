#![allow(non_snake_case)]
#![allow(dead_code)]

use gl::types::{GLuint, GLint};

use crate::base::error::ShaderError;

#[derive(PartialEq)]
pub enum CheckType { Program, Shader }

pub unsafe fn check_compile_error(id: GLuint, _type: CheckType) -> Result<(), ShaderError> {
    let mut success: GLint = 0;
    let mut err_log_size: GLint = 0;

    if _type == CheckType::Shader {
        // 获取编译状态
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
            
        if success == 1 { return Ok(()); }

        // 获取失败日志大小及内容
        gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut err_log_size);

        let mut err_log: Vec<u8> = Vec::with_capacity(err_log_size as usize);

        gl::GetShaderInfoLog(id, err_log_size, &mut err_log_size, err_log.as_mut_ptr() as *mut _);

        err_log.set_len(err_log_size as usize);
        let log = String::from_utf8(err_log)?;

        Err(ShaderError::CompilationError(log))
    } else {
        // 获取编译状态
        gl::GetProgramiv(id, gl::LINK_STATUS, &mut success);
    
        if success == 1 { return Ok(()); }

        // 获取失败日志大小及内容
        gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut err_log_size);

        let mut err_log: Vec<u8> = Vec::with_capacity(err_log_size as usize);

        gl::GetProgramInfoLog(id, err_log_size, &mut err_log_size, err_log.as_mut_ptr() as *mut _);

        err_log.set_len(err_log_size as usize);
        let log = String::from_utf8(err_log)?;

        Err(ShaderError::LinkingError(log))
    }
}