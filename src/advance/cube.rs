#![allow(dead_code)]
#![allow(non_snake_case)]

use std::{rc::Rc, cell::RefCell};
use nalgebra_glm as glm;

use crate::{IRenderer, base::{program::ShaderProgram, buffer::Buffer, vertex_array::VertexArray, texture::Texture, camera::Camera}, set_attribute};

const VERTEX_SOURCE_FILE: &str = "glsl/cube/vertex.glsl";
const FRAGMENT_SOURCE_FILE: &str = "glsl/cube/fragment.glsl";

type Pos = [f32; 3];
type TextureCoords = [f32; 2];

#[repr(C, packed)]
struct Vertex(Pos, TextureCoords);

const VERTICES: [Vertex; 36] = [
    Vertex([-0.5, -0.5, -0.5], [ 0.0, 0.0]),
    Vertex([0.5, -0.5, -0.5,], [1.0, 0.0]),
    Vertex([0.5,  0.5, -0.5,], [1.0, 1.0]),
    Vertex([0.5,  0.5, -0.5,], [1.0, 1.0]),
    Vertex([-0.5,  0.5, -0.5], [ 0.0, 1.0]),
    Vertex([-0.5, -0.5, -0.5], [ 0.0, 0.0]),
    Vertex([-0.5, -0.5,  0.5], [ 0.0, 0.0]),
    Vertex([0.5, -0.5,  0.5,], [1.0, 0.0]),
    Vertex([0.5,  0.5,  0.5,], [1.0, 1.0]),
    Vertex([0.5,  0.5,  0.5,], [1.0, 1.0]),
    Vertex([-0.5,  0.5,  0.5], [ 0.0, 1.0]),
    Vertex([-0.5, -0.5,  0.5], [ 0.0, 0.0]),
    Vertex([-0.5,  0.5,  0.5], [ 1.0, 0.0]),
    Vertex([-0.5,  0.5, -0.5], [ 1.0, 1.0]),
    Vertex([-0.5, -0.5, -0.5], [ 0.0, 1.0]),
    Vertex([-0.5, -0.5, -0.5], [ 0.0, 1.0]),
    Vertex([-0.5, -0.5,  0.5], [ 0.0, 0.0]),
    Vertex([-0.5,  0.5,  0.5], [ 1.0, 0.0]),
    Vertex([0.5,  0.5,  0.5,], [1.0, 0.0]),
    Vertex([0.5,  0.5, -0.5,], [1.0, 1.0]),
    Vertex([0.5, -0.5, -0.5,], [0.0, 1.0]),
    Vertex([0.5, -0.5, -0.5,], [0.0, 1.0]),
    Vertex([0.5, -0.5,  0.5,], [0.0, 0.0]),
    Vertex([0.5,  0.5,  0.5,], [1.0, 0.0]),
    Vertex([-0.5, -0.5, -0.5], [ 0.0, 1.0]),
    Vertex([0.5, -0.5, -0.5,], [1.0, 1.0]),
    Vertex([0.5, -0.5,  0.5,], [1.0, 0.0]),
    Vertex([0.5, -0.5,  0.5,], [1.0, 0.0]),
    Vertex([-0.5, -0.5,  0.5], [ 0.0, 0.0]),
    Vertex([-0.5, -0.5, -0.5], [ 0.0, 1.0]),
    Vertex([-0.5,  0.5, -0.5], [ 0.0, 1.0]),
    Vertex([0.5,  0.5, -0.5,], [1.0, 1.0]),
    Vertex([0.5,  0.5,  0.5,], [1.0, 0.0]),
    Vertex([0.5,  0.5,  0.5,], [1.0, 0.0]),
    Vertex([-0.5,  0.5,  0.5], [ 0.0, 0.0]),
    Vertex([-0.5,  0.5, -0.5], [ 0.0, 1.0]),
];

pub struct Cube {
    program: ShaderProgram,             // 着色器程序
    vertex_buffer: Buffer,              // 顶点缓冲对象（VBO）
    vertex_array: VertexArray,          // 顶点数组对象（VAO）
    texture_0: Texture,
    texture_1: Texture,

    camera: Rc<RefCell<Camera>>,

    pos: Vec<[f32; 3]>,                 // 正方体的位置
    win_size: (u32, u32),               // 窗口宽高
}

impl Cube {

    pub unsafe fn new(size: (u32, u32), image: (&str, &str), pos: Vec<[f32; 3]>, camera: Rc<RefCell<Camera>>) -> Result<Self, crate::base::error::GLError> {
        // VAO
        let vertex_array = VertexArray::new();

        // VBO
        let vertex_buffer = Buffer::new(gl::ARRAY_BUFFER, &VERTICES, gl::STATIC_DRAW);

        // 着色器程序
        let program = ShaderProgram::new(VERTEX_SOURCE_FILE, FRAGMENT_SOURCE_FILE)?;

        let pos_attr = program.get_attr_location("aPos")?;
        set_attribute!(vertex_array, pos_attr, Vertex::0);

        let tex_attr = program.get_attr_location("aTexCoord")?;
        set_attribute!(vertex_array, tex_attr, Vertex::1);

        // 纹理
        let texture_0 = Texture::new(image.0, gl::REPEAT, gl::REPEAT, gl::LINEAR, gl::LINEAR)?;
        program.set_int("texture1", 0)?;

        let texture_1 = Texture::new(image.1, gl::REPEAT, gl::REPEAT, gl::LINEAR, gl::LINEAR)?;
        program.set_int("texture2", 1)?;

        let renderer = Self {
            program,
            vertex_buffer,
            vertex_array,
            texture_0,
            texture_1,
            pos,
            camera,
            win_size: size,
        };

        Ok(renderer)
    }
}

impl IRenderer for Cube {
    unsafe fn draw(&self) -> Result<(), crate::base::error::GLError> {
        self.clear();
        self.texture_0.activate(gl::TEXTURE0);
        self.texture_1.activate(gl::TEXTURE1);

        self.program.apply();
        self.vertex_array.bind();

        let win_radio = (self.win_size.0 / self.win_size.1) as f32;

        let view = self.camera.borrow().get_view_matrix();

        let projection = glm::perspective(win_radio, f32::to_radians(self.camera.borrow().get_fov()), 0.1, 100.0);

        self.program.set_mat4("view", glm::value_ptr::<f32, 4, 4>(&view))?;
        self.program.set_mat4("projection", glm::value_ptr::<f32, 4, 4>(&projection))?;

        for _pos in &self.pos {
            let mut model = glm::identity::<f32, 4>();
            model = glm::translate(&model, &glm::Vec3::from_row_slice(_pos));

            self.program.set_mat4("model", glm::value_ptr::<f32, 4, 4>(&model))?;

            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }        

        Ok(())
    }
}
