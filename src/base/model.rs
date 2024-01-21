#![allow(non_snake_case)]
#![allow(dead_code)]

use std::path::Path;
use nalgebra_glm as glm;

use crate::base::{mesh::{Mesh, MeshTexture, MeshVertex}, error::{ModelError, GLError}, texture::Texture, program::ShaderProgram};

#[derive(Debug, PartialEq)]
pub enum MaterialType {
    Diffuse,
    Normal,
    Specular,
}

#[derive(Default)]
pub struct Model {
    pub meshes: Vec<Mesh>,
    pub textures_loaded: Vec<MeshTexture>,

    directory: String,          // 该文件所在的文件夹
}

impl Model {
    /**
     * load_field: 需要加载的纹理项
     */
    pub fn new(path: &str, load_field: Option<&[MaterialType]>) -> Result<Self, ModelError> {
        let mut model = Model::default();
        model.loadModel(path, load_field)?;
        Ok(model)
    }

    pub fn draw(&self, program: &ShaderProgram) -> Result<(), GLError> {
        for mesh in &self.meshes {
            unsafe { mesh.draw(program)?; }
        }
        Ok(())
    }

    fn loadModel(&mut self, path: &str, load_field: Option<&[MaterialType]>) -> Result<(), ModelError> {
        let path = Path::new(path);

        self.directory = path.parent().unwrap_or_else(|| Path::new("")).to_str().unwrap().into();

        let (models, materials) = tobj::load_obj(path, &tobj::GPU_LOAD_OPTIONS)?;
        for model in &models {
            let mesh = &model.mesh;
            let vertices_count = mesh.positions.len() / 3;

            let mut vertices: Vec<MeshVertex> = Vec::with_capacity(vertices_count);
            let indices: Vec<u32> = mesh.indices.clone();

            let (pos, norm, tex) = (&mesh.positions, &mesh.normals, &mesh.texcoords);
            for i in 0..vertices_count {
                vertices.push(
                    MeshVertex {
                        position: glm::vec3(pos[i*3], pos[i*3+1], pos[i*3+2]),
                        normal: glm::vec3(norm[i*3], norm[i*3+1], norm[i*3+2]),
                        texCoords: glm::vec2(tex[i*2], tex[i*2+1]),
                    }
                )
            }

            let materials = materials.clone()?;
            let mut textures: Vec<MeshTexture> = Vec::new();
            if let Some(id) = mesh.material_id {
                let material = &materials[id];

                if let Some(load_field) = load_field {
                    // diffuse map
                    if load_field.contains(&MaterialType::Diffuse) && material.diffuse_texture.is_some() {
                        let path = format!("{}/{}", self.directory, material.diffuse_texture.clone().unwrap());
                        let texture = self.loadMaterialTexture(path, "texture_diffuse")?;
                        textures.push(texture);
                    }

                    // specular map
                    if load_field.contains(&MaterialType::Specular) && material.specular_texture.is_some() {
                        let path = format!("{}/{}", self.directory, material.specular_texture.clone().unwrap());
                        let texture = self.loadMaterialTexture(path, "texture_specular")?;
                        textures.push(texture);
                    }

                    // normal map
                    if load_field.contains(&MaterialType::Normal) && material.normal_texture.is_some() {
                        let path = format!("{}/{}", self.directory, material.normal_texture.clone().unwrap());
                        let texture = self.loadMaterialTexture(path, "texture_normal")?;
                        textures.push(texture);
                    }
                }
            }

            unsafe { self.meshes.push(Mesh::new(vertices, indices, textures)); }
        }

        Ok(())
    }

    fn loadMaterialTexture(&mut self, path: String, typeName: &str) -> Result<MeshTexture, ModelError> {
        let texture = self.textures_loaded.iter().find(|t| t.path == path);
        if let Some(tex) = texture {
            return Ok(tex.clone());
        }

        let texture = MeshTexture{
            tex: Box::new(Texture::new(&path, gl::REPEAT, gl::REPEAT, gl::LINEAR, gl::LINEAR)?),
            type_: typeName.into(),
            path, 
        };

        self.textures_loaded.push(texture.clone());
        Ok(texture)
    }
}