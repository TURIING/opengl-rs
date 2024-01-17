use gl::types::{GLuint, GLsizeiptr};

pub struct Buffer {
    pub id: GLuint,
    target: GLuint,
}

impl Drop for Buffer {
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, [self.id].as_ptr()); }
    }
}

impl Buffer {
    pub unsafe fn new<T>(target: GLuint, data: &[T], usage: GLuint) -> Self {
        let mut id: GLuint = 0;
        gl::GenBuffers(1, &mut id);

        let ret = Self { id, target };
        ret.set_data(data, usage);

        ret
    }

    pub unsafe fn bind(&self) { gl::BindBuffer(self.target, self.id); }

    unsafe fn set_data<T>(&self, data: &[T], usage: GLuint) {
        self.bind();

        let (_, data_bytes, _) = data.align_to::<u8>();
        gl::BufferData(
            self.target, 
            data_bytes.len() as GLsizeiptr, 
            data_bytes.as_ptr() as *const _, 
            usage
        );
    }
}