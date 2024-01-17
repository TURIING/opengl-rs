#![allow(non_snake_case)]
#![allow(dead_code)]

use std::path::Path;
use gl::types::GLuint;
use image::{ImageError, DynamicImage};

use crate::base::error::ModelError;

#[derive(Clone)]
pub struct Texture{
    id: GLuint,
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe { gl::DeleteTextures(1, [self.id].as_ptr()); }
    }
}

impl Texture {
    /**
     * path:         图像路径
     * mode_wrap:    环绕方式
     * mode_filter： 过滤方式
     */
    pub unsafe fn new(path: &str, mode_wrap: GLuint, mode_filter: GLuint) -> Result<Self, ModelError> {
        let path = Path::new(path);

        let mut id = 0;
        gl::GenTextures(1, &mut id);

        let ret = Self { id };

        ret.set_wrapping_filtering(mode_wrap, mode_filter);

        ret.load(path)?;

        Ok(ret)
    }

    pub fn get_id(&self) -> GLuint { self.id }

    pub unsafe fn bind(&self) { 
        gl::BindTexture(gl::TEXTURE_2D, self.id); 
    }

    // 加载图像
    unsafe fn load(&self, path: &Path) -> Result<(), ImageError> {
        self.bind();

        let img = image::open(path)?.flipv();

        let format = match img {
            DynamicImage::ImageLuma8(_) => gl::RED,
            DynamicImage::ImageLumaA8(_) => gl::RG,
            DynamicImage::ImageRgb8(_) => gl::RGB,
            DynamicImage::ImageRgba8(_) => gl::RGBA,
            _ => todo!(),
        };

        // todo: 改变图像模式
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

    // 设置环绕方式、过滤方式
    unsafe fn set_wrapping_filtering(&self, mode_wrap: GLuint, mode_filter: GLuint) {
        self.bind();
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, mode_wrap as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, mode_wrap as i32);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, mode_filter as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, mode_filter as i32);
    }

    pub unsafe fn activate(&self, unit: GLuint) {
        gl::ActiveTexture(unit);
        self.bind();
    }
}

