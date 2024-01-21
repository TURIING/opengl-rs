#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(clippy::missing_safety_doc)]

use std::cell::RefCell;
use std::rc::Rc;

use opengl_rs::IRenderer;
use opengl_rs::base::camera::Camera;
use opengl_rs::base::engine::Engine;
use opengl_rs::base::error::GLError;

use nalgebra_glm as glm;
use opengl_rs::base::model::Model;
use opengl_rs::base::program::ShaderProgram;

const WINDOW_TITLE: &str = "phone_light";
const WINDOW_SIZE: (u32, u32) = (1200, 1200);

const LIGHT_VERTEX_SOURCE_FILE: &str = "glsl/phone_light/light.vs";
const LIGHT_FRAGMENT_SOURCE_FILE: &str = "glsl/phone_light/light.fs";
const OBJECT_VERTEX_SOURCE_FILE: &str = "glsl/phone_light/object.vs";
const OBJECT_FRAGMENT_SOURCE_FILE: &str = "glsl/phone_light/object.fs";

const OBJECT_MODEL_FILE: &str = "assets/model/cube/cueb.obj";
const LIGHT_MODEL_FILE: &str = "assets/model/sphere/sphere.obj";

pub struct PhoneLight {
    camera: Rc<RefCell<Camera>>,

    light_program: ShaderProgram,             
    object_program: ShaderProgram,         

    object_model: Model,    
    light_model: Model,    

    is_enable_deep_test: bool,          // 是否开启深度测试
}

impl PhoneLight {
    pub unsafe fn new() -> Result<Self, GLError> {
        let camera = Rc::new(RefCell::new(Camera::new(glm::vec3(0.0, 0.0, 4.0))));

        let light_program = ShaderProgram::new(LIGHT_VERTEX_SOURCE_FILE, LIGHT_FRAGMENT_SOURCE_FILE)?;
        let object_program = ShaderProgram::new(OBJECT_VERTEX_SOURCE_FILE, OBJECT_FRAGMENT_SOURCE_FILE)?;

        let object_model = Model::new(OBJECT_MODEL_FILE, None)?;
        let light_model = Model::new(LIGHT_MODEL_FILE, None)?;

        let mut ret = Self {is_enable_deep_test: true, camera, light_program, object_program, object_model, light_model};

        ret.enable_deep_test();

        Ok(ret)
    }
}

impl IRenderer for PhoneLight {
    // 绘制
    unsafe fn draw(&self) -> Result<(), GLError> {
        self.clear();

        let win_radio = (WINDOW_SIZE.0 / WINDOW_SIZE.1) as f32;
        const LIGHT_POS: &[f32] = &[1.2, 1.0, 2.0];

        self.object_program.set_vec3("objectColor", &[1.0, 0.5, 0.31])?;
        self.object_program.set_vec3("lightColor", &[1.0, 1.0, 1.0])?;
        self.object_program.set_vec3("lightPos", LIGHT_POS)?;
        self.object_program.set_vec3("viewPos", glm::value_ptr(&self.camera.borrow().get_pos()))?;

        let projection = glm::perspective(win_radio, f32::to_radians(self.camera.borrow().get_fov()), 0.1, 100.0);
        self.object_program.set_mat4("projection", glm::value_ptr(&projection))?;

        let view = self.camera.borrow().get_view_matrix();
        self.object_program.set_mat4("view", glm::value_ptr(&view))?;

        let mut model = glm::Mat4::identity();
        model = glm::scale(&model, &glm::Vec3::new(0.6, 0.6, 0.6));
        self.object_program.set_mat4("model", glm::value_ptr(&model))?;

        self.object_model.draw(&self.object_program)?;

        self.light_program.set_mat4("projection", glm::value_ptr(&projection))?;
        self.light_program.set_mat4("view", glm::value_ptr(&view))?;
        let mut model = glm::Mat4::identity();
        model = glm::translate(&model, &glm::Vec3::from_row_slice(LIGHT_POS));
        model = glm::scale(&model, &glm::Vec3::new(0.2, 0.2, 0.2));
        self.light_program.set_mat4("model", glm::value_ptr(&model))?;
        
        self.light_model.draw(&self.light_program)?;

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
    let act = || -> Result<PhoneLight, GLError> {unsafe{ PhoneLight::new() }};

    let mut engine = Engine::<PhoneLight>::new(WINDOW_TITLE, WINDOW_SIZE, act)?;
    engine.execute()?;
    
    Ok(())
}