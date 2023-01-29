use crate::common::*;

#[derive(Debug)]
pub struct Mesh {
    pub vbo: u32,
    pub vao: u32,
    pub vertex_count: i32
}

impl Mesh {
    pub fn _create() -> Mesh {
        return Mesh { vbo:u32::MAX, vao:u32::MAX, vertex_count: 0 };
    }
    pub fn create(vertices: Vec<Vertex>) -> Mesh {
        let mut mesh = Mesh::_create();
        unsafe {
            gl::GenVertexArrays(1, &mut mesh.vao);
            assert_ne!(mesh.vao, 0);
            gl::BindVertexArray(mesh.vao);

            gl::GenBuffers(1, &mut mesh.vbo);
            assert_ne!(mesh.vbo, 0);
            gl::BindBuffer(gl::ARRAY_BUFFER, mesh.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                core::mem::size_of_val(&vertices) as isize,
                vertices.as_ptr().cast(),
                gl::STATIC_DRAW,
            );

            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                core::mem::size_of::<Vertex>().try_into().unwrap(),
                0 as *const _,
            );
            gl::EnableVertexAttribArray(0);
            gl::BindVertexArray(0);
        }
        mesh.vertex_count = vertices.len() as i32;
        return mesh;
    }
}