#![allow(non_snake_case)]
#![allow(dead_code)]

use std::{rc::Rc, cell::RefCell};
use nalgebra_glm as glm;

use crate::{RendererInterface, base::{program::ShaderProgram, buffer::Buffer, vertex_array::VertexArray, camera::Camera}, set_attribute};

const VERTEX_SOURCE_FILE: &str = "glsl/sphere/vertex.glsl";
const FRAGMENT_SOURCE_FILE: &str = "glsl/sphere/fragment.glsl";

type Pos = [f32; 3];

#[derive(Clone, Copy)]
#[repr(C, packed)]
struct Vertex(Pos);

pub struct Sphere{
    program: ShaderProgram,             // 着色器程序
    vertex_buffer: Buffer,              // 顶点缓冲对象（VBO）
    vertex_array: VertexArray,          // 顶点数组对象（VAO）

    camera: Rc<RefCell<Camera>>,

    win_size: (u32, u32),               // 窗口宽高
}

impl Sphere {
    pub unsafe fn new(size: (u32, u32), camera: Rc<RefCell<Camera>>) -> Result<Self, crate::base::error::GLError> {
        let latitude = 60;
        let longitude = 120;
        let vertex_data = Self::makeSphere(longitude, latitude);

        // VAO
        let vertex_array = VertexArray::new();

        // VBO
        let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER, vertex_data.as_slice(), gl::STATIC_DRAW);

        // 着色器程序
        let program = ShaderProgram::new(VERTEX_SOURCE_FILE, FRAGMENT_SOURCE_FILE)?;

        let pos_attr = program.get_attr_location("aPos")?;
        set_attribute!(vertex_array, pos_attr, Vertex::0);

        let renderer = Self {
            program,
            vertex_buffer,
            vertex_array,
            camera,
            win_size: size,
        };

        Ok(renderer)
    }

    // 生成顶点坐标
    fn getPoint(u: f32, v: f32) -> Vertex {
        let r: f32 = 0.9;
        let pi = glm::pi::<f32>();

        let u = pi * u;
        let v = 2_f32 * pi * v;

        let z = r * u.cos();
        let x = r * u.sin() * v.cos();
        let y = r * u.sin() * v.sin();

        Vertex([x, y, z])
    }

    // 遍历生成球体的所有顶点坐标
    fn makeSphere(longitude: u32, latitude: u32) -> Vec<Vertex> {
        let lon_unit = 1.0 / longitude as f32;
        let lat_unit = 1.0 / latitude as f32;

        let mut ret: Vec<Vertex> = Vec::new();
        for lat in 0..latitude {
            for lon in 0..longitude {
                let point1 = Self::getPoint(lat as f32 * lat_unit, lon as f32 * lon_unit);
                let point2 = Self::getPoint((lat + 1) as f32 * lat_unit, lon as f32 * lon_unit);
                let point3 = Self::getPoint((lat + 1) as f32 * lat_unit, (lon + 1) as f32 * lon_unit);
                let point4 = Self::getPoint(lat as f32 * lat_unit, (lon + 1) as f32 * lon_unit);

                ret.push(point1);
                ret.push(point4);
                ret.push(point3);

                ret.push(point1);
                ret.push(point3);
                ret.push(point2);
            }
        }

        ret
    }
}

impl RendererInterface for Sphere {
    unsafe fn draw(&self) -> Result<(), crate::base::error::GLError> {
        self.clear();

        self.program.apply();
        self.vertex_array.bind();

        let win_radio = (self.win_size.0 / self.win_size.1) as f32;

        let view = self.camera.borrow().get_view_matrix();

        let projection = glm::perspective(win_radio, f32::to_radians(self.camera.borrow().get_fov()), 0.1, 100.0);

        self.program.set_mat4("view", glm::value_ptr::<f32, 4, 4>(&view))?;
        self.program.set_mat4("projection", glm::value_ptr::<f32, 4, 4>(&projection))?;

        let model = glm::identity::<f32, 4>();
        self.program.set_mat4("model", glm::value_ptr::<f32, 4, 4>(&model))?;
        
        gl::DrawArrays(gl::LINE_LOOP, 0, 6 * 60 * 120); 

        Ok(())
    }
}