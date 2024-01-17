use gl::types::{GLuint, GLint };

pub struct VertexArray {
    pub id: GLuint,
}

impl Drop for VertexArray {
    fn drop(&mut self) {
      unsafe { gl::DeleteVertexArrays(1, [self.id].as_ptr()); }
    }
}

impl VertexArray {
    pub unsafe fn new() -> Self {
        let mut id: GLuint = 0;
        gl::GenVertexArrays(1, &mut id);

        let ret = Self { id };
        ret.bind();

        ret
    }

    pub unsafe fn bind(&self) { gl::BindVertexArray(self.id); }
    pub unsafe fn unbind(&self) { gl::BindVertexArray(0); }

    // 解释如何解析顶点位置数据并启用位置顶点属性
    pub unsafe fn set_attribute<V: Sized>(&self, attr_pos: GLuint, count: GLint, offset: GLint) {
        self.bind();
        gl::VertexAttribPointer(
            attr_pos, 
            count, 
            gl::FLOAT, 
            gl::FALSE, 
            std::mem::size_of::<V>() as GLint, 
            offset as *const _
        );
        gl::EnableVertexAttribArray(attr_pos);
    }
}

#[macro_export]
macro_rules! set_attribute {
    ($vao:expr, $pos:tt, $t:ident :: $field:tt) => {{
        let dummy = core::mem::MaybeUninit::<$t>::uninit();
        let dummy_ptr = dummy.as_ptr();
        let member_ptr = core::ptr::addr_of!((*dummy_ptr).$field);
        const fn size_of_raw<T>(_: *const T) -> usize {
            core::mem::size_of::<T>()
        }
        let member_offset = member_ptr as i32 - dummy_ptr as i32;
        $vao.set_attribute::<$t>(
            $pos,
            (size_of_raw(member_ptr) / core::mem::size_of::<f32>()) as i32,
            member_offset,
        )
    }};
}