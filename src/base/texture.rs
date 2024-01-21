#![allow(non_snake_case)]
#![allow(dead_code)]

use std::path::Path;
use gl::types::GLuint;
use image::DynamicImage;

use super::error::ModelError;

#[derive(Clone)]
pub struct Texture{
    id: GLuint,

    path: String,

    wrap_s_mode: GLuint, 

    wrap_t_mode: GLuint, 

    min_filter_mode: GLuint,

    mag_filter_mode: GLuint
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, [self.id].as_ptr()); }
    }
}

impl Texture {
    pub fn new<T: Into<String>>(path: T, wrap_s_mode: GLuint, wrap_t_mode: GLuint, min_filter_mode: GLuint, mag_filter_mode: GLuint) -> Result<Self, ModelError> {
        unsafe {
            let mut ret = Self { id: 0, path: path.into(), wrap_s_mode, wrap_t_mode, min_filter_mode, mag_filter_mode };

            gl::GenTextures(1, &mut ret.id);
    
            ret.set_wrapping_filtering();

            ret.load()?;

            Ok(ret)
        }
    }

    pub fn id(&self) -> GLuint { self.id }

    pub fn bind(&self) { 
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id); 
        }
    }

    // 加载图像
    pub fn load(&mut self) -> Result<(), ModelError> {
        unsafe {
            let path = Path::new(&self.path);

            let img = image::open(path)?.flipv();

            let format = match img {
                DynamicImage::ImageLuma8(_) => gl::RED,
                DynamicImage::ImageLumaA8(_) => gl::RG,
                DynamicImage::ImageRgb8(_) => gl::RGB,
                DynamicImage::ImageRgba8(_) => gl::RGBA,
                _ => todo!(),
            };

            gl::TexImage2D(
                gl::TEXTURE_2D, 
                0, 
                format as i32, 
                img.width() as i32, 
                img.height() as i32, 
                0, 
                format, 
                gl::UNSIGNED_BYTE, 
                img.as_bytes().as_ptr() as *const _
            );

            gl::GenerateMipmap(gl::TEXTURE_2D);
            Ok(())
        }
    }

    // 设置环绕方式、过滤方式
    unsafe fn set_wrapping_filtering(&self) {
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, self.wrap_s_mode as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, self.wrap_t_mode as i32);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, self.min_filter_mode as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, self.mag_filter_mode as i32);
    }

    pub fn activate(&self, unit: GLuint) {
        unsafe {
            gl::ActiveTexture(unit);
            self.bind();
        }
    }
}