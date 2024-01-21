#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(clippy::missing_safety_doc)]

pub mod base;
pub mod advance;

use std::cell::RefCell;
use std::rc::Rc;

use base::error::GLError;
use base::camera::Camera;

pub trait IRenderer: Sized {
    // 绘制前处理
    unsafe fn pre_draw(&self) -> Result<(), GLError> { Ok(()) }
    // 绘制
    unsafe fn draw(&self) -> Result<(), GLError>;

    // 开启深度测试
    fn enable_deep_test(&mut self) { }

    // 清屏
    unsafe fn clear(&self) { }

    // 获取摄像机
    fn getCamera(&self) -> Option<Rc<RefCell<Camera>>> { None }
}