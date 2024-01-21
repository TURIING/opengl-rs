use glfw::{WindowHint, OpenGlProfileHint, WindowMode, Context, CursorMode, WindowEvent, Key, Action, GlfwReceiver, };

use crate::{IRenderer, base::{error::GLError, camera::CameraMovement}};

pub struct Engine<T: IRenderer> {
    renderer: T,
    glfw: glfw::Glfw,
    window: glfw::PWindow,
    event_receiver: GlfwReceiver<(f64, WindowEvent)>,

    delta_time: f32,                                            // 上一帧跟当前帧的时间间隔
    last_frame: f32,                                            // 记录上一帧的时间
    is_first_capture: bool,                                     // 鼠标是否第一次被捕获
    cursor_pos: (f32, f32),                                     // 记录鼠标的位置
}

impl<T: IRenderer> Engine<T> {
    pub fn new<F: Fn() -> Result<T, GLError>>(win_title: &str, size: (u32, u32), act: F) -> Result<Self, GLError> {
        let mut glfw = glfw::init_no_callbacks()?;
        glfw.window_hint(WindowHint::ContextVersion(3, 3));
        glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));

        // 创建窗口
        let (mut window, event_receiver) = glfw.create_window(size.0, size.1, win_title, WindowMode::Windowed).expect("Failed to create GLFW window.");
        window.make_current();

        // 找到opengl函数地址
        gl::load_with(|ptr| window.get_proc_address(ptr) as *const _);

        let renderer = act()?;

        // 注册事件
        if renderer.getCamera().is_some() { 
            window.set_cursor_pos_polling(true); 
            // 捕获鼠标
            window.set_cursor_mode(CursorMode::Disabled);
        }
        window.set_framebuffer_size_polling(true);
        
        let cursor_pos = (size.0 as f32 / 2.0, size.1 as f32 / 2.0);
        Ok( Self{ renderer, glfw, window, event_receiver, delta_time: 0.0, last_frame: 0.0, is_first_capture: true, cursor_pos } )
    }

    #[allow(clippy::single_match)]
    pub fn execute(&mut self) -> Result<(), GLError> {
        unsafe { self.renderer.pre_draw()?; }
        
        while !self.window.should_close() {
            let current_frame = self.glfw.get_time() as f32;
            self.delta_time = current_frame - self.last_frame;
            self.last_frame = current_frame;

            self.glfw.poll_events();
            for(_, event) in glfw::flush_messages(&self.event_receiver) {
                match event {
                    WindowEvent::CursorPos(x_pos, y_pos) => {
                        let x_pos = x_pos as f32;
                        let y_pos = y_pos as f32;
                
                        if self.is_first_capture {
                            self.cursor_pos.0 = x_pos;
                            self.cursor_pos.1 = y_pos;
                            self.is_first_capture = false;
                        }
                
                        let x_offset = x_pos - self.cursor_pos.0;
                        let y_offset = self.cursor_pos.1 - y_pos;
                
                        self.cursor_pos.0 = x_pos;
                        self.cursor_pos.1 = y_pos;
                
                        self.renderer.getCamera().unwrap().borrow_mut().process_mouse_move(x_offset, y_offset);
                    },
                    _ => (),
                }
            }

            // 不在事件里处理键盘事件，是因为不能持续触发
            self.handle_keyboard();

            unsafe { self.renderer.draw()?; }
            self.window.swap_buffers();
        }
        Ok(())
    }

    fn handle_keyboard(&mut self) {
        if self.renderer.getCamera().is_some() {
            if self.window.get_key(Key::W) == Action::Press { self.renderer.getCamera().unwrap().borrow_mut().process_keyboard(CameraMovement::Forward, self.delta_time) }
            if self.window.get_key(Key::S) == Action::Press { self.renderer.getCamera().unwrap().borrow_mut().process_keyboard(CameraMovement::Bakcward, self.delta_time) }
            if self.window.get_key(Key::A) == Action::Press { self.renderer.getCamera().unwrap().borrow_mut().process_keyboard(CameraMovement::Left, self.delta_time) }
            if self.window.get_key(Key::D) == Action::Press { self.renderer.getCamera().unwrap().borrow_mut().process_keyboard(CameraMovement::Right, self.delta_time) }

            if self.window.get_key(Key::Space) == Action::Press { 
                match self.window.get_cursor_mode() {
                    CursorMode::Disabled => self.window.set_cursor_mode(CursorMode::Normal),
                    CursorMode::Normal => self.window.set_cursor_mode(CursorMode::Disabled),
                    _ => (),
                }
            }
        }

        if self.window.get_key(Key::Escape) == Action::Press { self.window.set_should_close(true) }
    }
}