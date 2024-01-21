#![allow(non_snake_case)]
#![allow(dead_code)]

use std::ptr;
use nalgebra_glm as glm;

use crate::base::error::{GLError, ModelError};
use crate::set_attribute;
use crate::base::program::ShaderProgram;
use crate::base::buffer::Buffer;
use crate::base::texture::Texture;
use crate::base::vertex_array::VertexArray;

#[repr(C, packed)]
pub struct MeshVertex {
    pub position: glm::Vec3,                    // 位置向量
    pub normal: glm::Vec3,                      // 法线向量
    pub texCoords: glm::Vec2,                   // 纹理
}

impl Default for MeshVertex {
    fn default() -> Self {
        Self { 
            position: glm::Vec3::zeros(), 
            normal: glm::Vec3::zeros(), 
            texCoords: glm::Vec2::zeros()
        }
    }
}

#[derive(Clone)]
pub struct MeshTexture {
    pub tex: Box<Texture>,
    pub type_: String,
    pub path: String,
}

pub struct Mesh {
    pub vertices: Vec<MeshVertex>,
    pub indices: Vec<u32>,
    pub textures: Vec<MeshTexture>,
    pub vao: VertexArray,

    vbo: Buffer,
    ebo: Buffer,
}

impl Mesh {
    pub unsafe fn new(vertices: Vec<MeshVertex>, indices: Vec<u32>, textures: Vec<MeshTexture>) -> Self {

        let vao = VertexArray::new();
        let vbo = Buffer::new(gl::ARRAY_BUFFER, vertices.as_slice(), gl::STATIC_DRAW);
        let ebo = Buffer::new(gl::ELEMENT_ARRAY_BUFFER, indices.as_slice(), gl::STATIC_DRAW);

        set_attribute!(vao, 0, MeshVertex::position);
        set_attribute!(vao, 1, MeshVertex::normal);
        set_attribute!(vao, 2, MeshVertex::texCoords);
        vao.unbind();

        Mesh { vertices, indices, textures, vao, vbo, ebo }
    }

    pub unsafe fn draw(&self, program: &ShaderProgram) -> Result<(), GLError> {
        let mut diffuseNr = 0;
        let mut specularNr = 0;
        let mut normalNr = 0;
        let mut heightNr = 0;

        for (i, texture) in self.textures.iter().enumerate() {
            gl::ActiveTexture(gl::TEXTURE0 + i as u32);
            let name = &texture.type_;
            let number = match name.as_str() {
                "texture_diffuse" => {
                    diffuseNr += 1;
                    diffuseNr
                },
                "texture_specular" => {
                    specularNr += 1;
                    specularNr
                },
                "texture_normal" => {
                    normalNr += 1;
                    normalNr
                },
                "texture_height" => {
                    heightNr += 1;
                    heightNr
                },
                _ => return Err(GLError::ModelError(ModelError::UnkownTextureType(texture.type_.clone())))
            };

            let sampler = format!("{}{}", name, number);
            program.set_int(&sampler, i as i32)?;
            
            gl::BindTexture(gl::TEXTURE_2D, texture.tex.id());
        }

        self.vao.bind();
        gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32, gl::UNSIGNED_INT, ptr::null());
        self.vao.unbind();
        gl::ActiveTexture(gl::TEXTURE0);        

        Ok(())
    }
}