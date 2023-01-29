// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
extern crate glfw;

pub mod mesh;
pub mod common;
pub mod shader;

use glfw::{Action, Context as GLFWContext, Key, Window, ffi::glfwWindowHint};
use mesh::*;
use common::*;
use shader::*;

const WINDOW_TITLE: &str = "Simple Ray Tracer";
const VERTICES: [Vertex; 3] = [(-0.5, -0.5, 0.0), (0.5, -0.5, 0.0), (0.0, 0.5, 0.0)];
const CLEAR_COLOR: [f32; 4] = [0.8, 0.8, 0.8, 1.0];

#[derive(Debug)]
struct Context {
    shader_program: Option<u32>,
    meshes: Vec<Mesh>,
    window: Option<Window>,
}

fn init_gl(ctx: &mut Context)
{
    unsafe {
        gl::load_with(|f_name| ctx.window.as_mut().unwrap().get_proc_address(f_name));
        gl::ClearColor(CLEAR_COLOR[0], CLEAR_COLOR[1], CLEAR_COLOR[2], CLEAR_COLOR[3]);
    }
    let shader_program = Shader::_create().shader_program;
    ctx.shader_program.replace(shader_program);
}

fn main() {
    let mut ctx = Context { shader_program: None, meshes: Vec::new(), window: None };

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    unsafe {
        glfwWindowHint(glfw::ffi::CONTEXT_VERSION_MAJOR, 3);
        glfwWindowHint(glfw::ffi::CONTEXT_VERSION_MINOR, 3);
        glfwWindowHint(glfw::ffi::OPENGL_PROFILE, glfw::ffi::OPENGL_CORE_PROFILE);
        //glfwWindowHint(glfw::ffi::DOUBLEBUFFER, gl::FALSE as i32); // single buffer. double = vsync?
        glfwWindowHint(glfw::ffi::OPENGL_FORWARD_COMPAT, gl::FALSE as i32);
    }
    
    // Create a windowed mode window and its OpenGL context
    let (window, events) = glfw
        .create_window(800, 600, WINDOW_TITLE, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
    ctx.window = Some(window);

    // Make the window's context current
    ctx.window.as_mut().unwrap().make_current();
    ctx.window.as_mut().unwrap().set_key_polling(true);

    init_gl(&mut ctx);
    ctx.meshes.push(Mesh::create(Vec::from(VERTICES)));

    println!("Context {:?}", ctx);
    // Loop until the user closes the window
    while !ctx.window.as_mut().unwrap().should_close() {
        // Swap front and back buffers
        ctx.window.as_mut().unwrap().swap_buffers();

        // Poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    ctx.window.as_mut().unwrap().set_should_close(true)
                }
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            ctx.meshes.iter().for_each(|m| {
                gl::BindVertexArray(m.vao);
                gl::DrawArrays(gl::TRIANGLES, 0, m.vertex_count);
            });
        }
    }
}
