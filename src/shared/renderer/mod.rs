mod constants;
mod utils;

use std::ptr;

use constants::{BYTES_PER_PIXEL, FRAGMENT_SRC, VERTEX_SRC};
use gl::types::{GLint, GLuint};

#[derive(Debug)]
pub struct Renderer {
    pub program: GLuint,
    pub front_texture: GLuint,
    pub front_uniform: GLint,
    pub back_texture: GLuint,
    pub back_uniform: GLint,
    pub vao: GLuint,
    pub vbo: GLuint,
    pub fbo: GLuint,
    pub pbo: GLuint,
    pub width: i32,
    pub height: i32,
    pub refresh_rate: u32,
}

impl Renderer {
    pub fn new((width, height): (i32, i32), refresh_rate: u32) -> Self {
        unsafe {
            let vertex_shader = utils::compile_shader(gl::VERTEX_SHADER, VERTEX_SRC);
            let fragment_shader = utils::compile_shader(gl::FRAGMENT_SHADER, FRAGMENT_SRC);
            let program = gl::CreateProgram();

            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);

            gl::LinkProgram(program);
            gl::UseProgram(program);

            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);

            let front_texture = utils::create_texture(width, height);
            let front_uniform = gl::GetUniformLocation(program, c"front_texture".as_ptr() as _);

            let back_texture = utils::create_texture(width, height);
            let back_uniform = gl::GetUniformLocation(program, c"back_texture".as_ptr() as _);

            let (vao, vbo) = utils::create_geometry(program);
            let fbo = utils::create_fbo(back_texture);

            let pbo = utils::create_pbo(width, height);

            let status = gl::CheckFramebufferStatus(gl::FRAMEBUFFER);
            if status != gl::FRAMEBUFFER_COMPLETE {
                panic!("Framebuffer not complete: {status}");
            }

            Self {
                program,
                front_texture,
                front_uniform,
                back_texture,
                back_uniform,
                vao,
                vbo,
                fbo,
                pbo,
                width,
                height,
                refresh_rate,
            }
        }
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        unsafe {
            self.width = width;
            self.height = height;

            gl::Viewport(0, 0, width, height);

            utils::resize_texture(self.back_texture, width, height);
        }
    }

    // A Pixel Buffer Object (PBO) is used to upload the buffer directly to the GPU,
    // offering better performance than direct texture uploads.
    // This helps reduce the time the current GL context remains locked.
    pub fn paint(
        &self,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        buffer: *const u8,
        full_width: i32,
    ) {
        utils::resize_pbo(self.pbo, self.width, self.height);
        utils::resize_texture(self.front_texture, self.width, self.height);

        unsafe {
            gl::BindBuffer(gl::PIXEL_UNPACK_BUFFER, self.pbo);

            let row_bytes = width * BYTES_PER_PIXEL;
            let stride = full_width * BYTES_PER_PIXEL;

            let ptr = gl::MapBuffer(gl::PIXEL_UNPACK_BUFFER, gl::WRITE_ONLY) as *mut u8;
            if !ptr.is_null() {
                for row in 0..height {
                    let src_offset = (y + row) * stride + (x * BYTES_PER_PIXEL);
                    let dst_offset = row * row_bytes;

                    let src_ptr = buffer.add(src_offset as usize);
                    let dst_ptr = ptr.add(dst_offset as usize);

                    ptr::copy_nonoverlapping(src_ptr, dst_ptr, row_bytes as usize);
                }

                gl::UnmapBuffer(gl::PIXEL_UNPACK_BUFFER);
            }

            gl::BindTexture(gl::TEXTURE_2D, self.front_texture);
            gl::TexSubImage2D(
                gl::TEXTURE_2D,
                0,
                x,
                y,
                width,
                height,
                gl::BGRA,
                gl::UNSIGNED_BYTE,
                std::ptr::null(),
            );

            gl::BindBuffer(gl::PIXEL_UNPACK_BUFFER, 0);
        }
    }

    pub fn draw(&self) {
        unsafe {
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::ONE, gl::ONE_MINUS_SRC_ALPHA);
            gl::BlendEquation(gl::FUNC_ADD);

            gl::UseProgram(self.program);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.back_texture);
            gl::Uniform1i(self.back_uniform, 0);

            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, self.front_texture);
            gl::Uniform1i(self.front_uniform, 1);

            gl::BindVertexArray(self.vao);
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        }
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
            gl::DeleteTextures(1, &self.front_texture);
            gl::DeleteTextures(1, &self.back_texture);
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.pbo);
            gl::DeleteBuffers(1, &self.fbo);
        }
    }
}
