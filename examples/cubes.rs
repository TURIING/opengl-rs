#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]

use std::cell::RefCell;
use std::rc::Rc;

use opengl_rs::IRenderer;
use opengl_rs::base::camera::Camera;
use opengl_rs::base::engine::Engine;
use opengl_rs::advance::cube::Cube;
use opengl_rs::base::error::GLError;

use nalgebra_glm as glm;

const WINDOW_TITLE: &str = "model";
const WINDOW_SIZE: (u32, u32) = (1200, 1200);

const TEXTURE_0_FILE: &str = "assets/image/awesomeface.png";
const TEXTURE_1_FILE: &str = "assets/image/container.jpg";

const POS: [[f32; 3]; 10] = [
    [ 2.0,  2.0,  2.0],
    [ 2.0,  5.0, -15.0],
    [-1.5, -2.2, -2.5],
    [-3.8, -2.0, -12.3],
    [ 2.4, -0.4, -3.5],
    [-1.7,  3.0, -7.5],
    [ 1.3, -2.0, -2.5],
    [ 1.5,  2.0, -2.5],
    [ 1.5,  0.2, -1.5],
    [-1.3,  1.0, -1.5],
];

pub struct Cubes {
    cube: Cube,
    camera: Rc<RefCell<Camera>>,

    is_enable_deep_test: bool,          // 是否开启深度测试
}

impl Cubes {
    pub unsafe fn new() -> Result<Self, GLError> {
        let camera = Rc::new(RefCell::new(Camera::new(glm::vec3(0.0, 0.0, 4.0))));
        let cube = Cube::new(WINDOW_SIZE, (TEXTURE_0_FILE, TEXTURE_1_FILE), POS.to_vec(), Rc::clone(&camera))?;

        let mut ret = Self {cube, is_enable_deep_test: true, camera};
        ret.enable_deep_test();
        Ok(ret)
    }
}

impl IRenderer for Cubes {
    // 绘制
    unsafe fn draw(&self) -> Result<(), GLError> {
        self.clear();

        self.cube.draw()?;

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
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let act = || -> Result<Cubes, GLError> {unsafe{ Cubes::new() }};

    let mut engine = Engine::<Cubes>::new(WINDOW_TITLE, WINDOW_SIZE, act)?;
    engine.execute()?;
    
    Ok(())
}