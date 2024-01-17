#![allow(non_snake_case)]

use nalgebra_glm as glm;

/**
 * 摄像机移动的方向
 */
#[derive(Debug)]
pub enum CameraMovement { Forward, Bakcward, Left, Right }

pub struct Camera {
    pos: glm::Vec3,             // 摄像机的位置
    front: glm::Vec3,           // 摄像机指向的向量
    up: glm::Vec3,              
    right: glm::Vec3,
    world_up: glm::Vec3,

    yaw: f32,                   // 偏航角
    pitch: f32,                 // 俯仰角

    move_speed: f32,            
    mouse_sensitivity: f32,
    fov: f32,                   // 视野（Field of View），定义了我们可以看到场景中多大的范围
}

#[allow(dead_code)]
impl Camera {
    pub fn new(pos: glm::Vec3) -> Self {
        let mut ret = Self { 
            pos, 
            front: glm::vec3(0.0, 0.0, -1.0), 
            up: glm::vec3(0.0, 1.0, 0.0), 
            right: glm::vec3(0.0, 0.0, 0.0), 
            world_up: glm::vec3(0.0, 1.0, 0.0), 
            yaw: -90.0, 
            pitch: 0.0, 
            move_speed: 2.5, 
            mouse_sensitivity: 0.1, 
            fov: 45.0 
        };
        ret.update_camera_vector();

        ret
    }

    pub fn get_view_matrix(&self) -> glm::Mat4{
        glm::look_at(&self.pos, &(self.pos + self.front), &self.up)
    }

    pub fn get_pos(&self) -> glm::Vec3 { self.pos }

    pub fn get_front(&self) -> glm::Vec3 { self.front }

    pub fn get_fov(&self) -> f32 { self.fov }

    // 处理键盘事件
    pub fn process_keyboard(&mut self, direction: CameraMovement, delta_time: f32) {
        let velocity = self.move_speed * delta_time;

        match direction {
            CameraMovement::Forward => self.pos += velocity * self.front,
            CameraMovement::Bakcward => self.pos -= velocity * self.front,
            CameraMovement::Left => self.pos -= velocity * self.right,
            CameraMovement::Right => self.pos += velocity * self.right,
        }
    }

    // 处理鼠标移动事件
    pub fn process_mouse_move(&mut self, mut x_offset: f32, mut y_offset: f32) {
        x_offset *= self.mouse_sensitivity;
        y_offset *= self.mouse_sensitivity;

        self.yaw += x_offset;
        self.pitch += y_offset;

        // 限制视角
        if self.pitch > 89.0 { self.pitch = 89.0; }
        if self.pitch < -89.0 { self.pitch = -89.0; }

        self.update_camera_vector();
    }

    // 处理鼠标滚动事件
    pub fn process_mouse_scroll(&mut self, y_offset: f32) {
        self.fov -= y_offset;

        if self.fov < 1.0 { self.fov = 1.0; }
        if self.fov > 45.0 { self.fov = 45.0; }
    }

    fn update_camera_vector(&mut self) {
        let x = f32::to_radians(self.yaw).cos() * f32::to_radians(self.pitch).cos();
        let y = f32::to_radians(self.pitch).sin();
        let z = f32::to_radians(self.yaw).sin() * f32::to_radians(self.pitch).cos();
        self.front = glm::normalize(&glm::Vec3::new(x, y, z));

        self.right = glm::normalize(&glm::cross(&self.front, &self.world_up));
        self.up = glm::normalize(&glm::cross(&self.right, &self.front));
    }
}