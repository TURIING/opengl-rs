#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]

use std::cell::RefCell;
use std::rc::Rc;

use opengl_rs::RendererInterface;
use opengl_rs::base::camera::Camera;
use opengl_rs::base::engine::Engine;
use opengl_rs::advance::sphere::Sphere;
use opengl_rs::base::error::GLError;

use nalgebra_glm as glm;

const WINDOW_TITLE: &str = "model";
const WINDOW_SIZE: (u32, u32) = (1200, 1200);

pub struct Spheres {
    sphere: Sphere,
    camera: Rc<RefCell<Camera>>,

    is_enable_deep_test: bool,          // 是否开启深度测试
}

impl Spheres {
    pub unsafe fn new() -> Result<Self, GLError> {
        let camera = Rc::new(RefCell::new(Camera::new(glm::vec3(0.0, 0.0, 4.0))));
        let sphere = Sphere::new(WINDOW_SIZE, Rc::clone(&camera))?;

        let mut ret = Self {sphere, is_enable_deep_test: true, camera};
        ret.enable_deep_test();
        
        Ok(ret)
    }
}

impl RendererInterface for Spheres {
    // 绘制
    unsafe fn draw(&self) -> Result<(), GLError> {
        self.clear();

        self.sphere.draw()?;

        Ok(())
    }

    // 开启深度测试
    fn enable_deep_test(&mut self) {
        self.is_enable_deep_test = true;
        unsafe { gl::Enable(gl::DEPTH_TEST) };
    }

    // 清屏
    unsafe fn clear(&self) {
        gl::ClearColor(0.5, 0.5, 0.5, 1.0);

        if self.is_enable_deep_test {
            gl::Clear(gl::DEPTH_BUFFER_BIT);
        }
        gl::Clear(gl::COLOR_BUFFER_BIT);
     }

    // 获取摄像机
    fn getCamera(&self) -> Option<Rc<RefCell<Camera>>> { Some(Rc::clone(&self.camera)) }

    unsafe fn pre_draw(&self) -> Result<(), GLError> {
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let act = || -> Result<Spheres, GLError> {unsafe{ Spheres::new() }};

    let mut engine = Engine::<Spheres>::new(WINDOW_TITLE, WINDOW_SIZE, act)?;
    engine.execute()?;
    
    Ok(())
}