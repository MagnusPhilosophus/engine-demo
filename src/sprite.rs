extern crate nalgebra;
use nalgebra as na;
use std::time::Instant;

pub struct Sprite{
    vao: gl::types::GLuint,
    sp: gl::types::GLuint,
    model: na::Matrix4::<f32>,
    location: gl::types::GLint,
}

impl Sprite {
    pub fn init(sp: gl::types::GLuint) -> Sprite {
        let vertices: Vec<f32> = vec![0.1, 0.1, 0.1, -0.1, -0.1, -0.1, -0.1, 0.1];
        let indices: Vec<u32> = vec![0, 1, 3, 1, 2, 3];
        let mut vao: gl::types::GLuint = 0;
        let mut vbo: gl::types::GLuint = 0;
        let mut ebo: gl::types::GLuint = 0;
        unsafe{
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);
            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                           (indices.len() * std::mem::size_of::<u32>()) as gl::types::GLsizeiptr,
                           indices.as_ptr() as *const gl::types::GLvoid, gl::STATIC_DRAW);
            gl::BufferData(gl::ARRAY_BUFFER,
                           (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                           vertices.as_ptr() as *const gl::types::GLvoid, gl::STATIC_DRAW);
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0,
                                    2,
                                    gl::FLOAT,
                                    gl::FALSE,
                                    (2 * std::mem::size_of::<f32>()) as gl::types::GLint,
                                    std::ptr::null());
        }
        Sprite{
            vao: vao,
            sp: sp,
            model: na::Matrix4::<f32>::identity(),
            location: unsafe{ gl::GetUniformLocation(sp, "model\0".as_ptr() as *const i8)},
        }
    }

    pub fn update(&mut self, vec: na::Vector3<f32>, time: Instant){
            self.model = na::Matrix4::<f32>::identity();
            self.model = self.model.append_scaling(f32::sin(time.elapsed().as_millis() as f32 / 1000.0));
            self.model.append_translation_mut(&vec);
    }


    pub fn draw(&self){
        unsafe{
            gl::UseProgram(self.sp);
            gl::BindVertexArray(self.vao);
            gl::Uniform3f(gl::GetUniformLocation(self.sp, "color\0".as_ptr() as *const i8), 0.3_f32, 1.0_f32, 0.0_f32);
            gl::UniformMatrix4fv(self.location, 1, gl::FALSE, &(*self.model.as_slice().as_ptr()) as *const f32);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
    }
}
