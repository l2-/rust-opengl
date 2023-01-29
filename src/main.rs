// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
extern crate glfw;
extern crate opencl3;
extern crate pretty_env_logger;

mod common;
mod logging;
mod mesh;
mod cl {
    pub mod device;
    pub mod kernel;
}
mod shader;

use common::*;
use glfw::{ffi::glfwWindowHint, Action, Context as GLFWContext, Key, Window};
use mesh::*;
use shader::*;

const WINDOW_TITLE: &str = "Simple Ray Tracer";
const VERTICES: [Vertex; 3] = [(-0.5, -0.5, 0.0), (0.5, -0.5, 0.0), (0.0, 0.5, 0.0)];
const CLEAR_COLOR: [f32; 4] = [0.8, 0.8, 0.8, 1.0];

#[derive(Debug)]
struct Context {
    shader_program: Option<u32>,
    meshes: Vec<Mesh>,
    window: Option<Window>,
    cl_context: Option<opencl3::context::Context>,
    cl_queue: Option<opencl3::command_queue::CommandQueue>
}

impl Context {
    fn new() -> Context {
        Context {
            shader_program: None,
            meshes: Vec::new(),
            window: None,
            cl_context: None,
            cl_queue: None,
        }
    }
}

fn init_gl(ctx: &mut Context) {
    unsafe {
        gl::load_with(|f_name| ctx.window.as_mut().unwrap().get_proc_address(f_name));
        gl::ClearColor(
            CLEAR_COLOR[0],
            CLEAR_COLOR[1],
            CLEAR_COLOR[2],
            CLEAR_COLOR[3],
        );
    }
    let shader_program = Shader::_create().shader_program;
    ctx.shader_program.replace(shader_program);
}

fn init_cl(ctx: &mut Context) {
    let device = match cl::device::find_most_capable_device() {
        Some(device) => device,
        None => panic!("No opencl capable device"),
    };
    log::info!("Most OpenCL 3.0 capable device {:?}, Compute units: {:?}, local_mem: {:?}, K global_mem: {:?}M", 
        device.name().unwrap(),
        device.max_compute_units().unwrap(),
        device.local_mem_size().unwrap() / (1e3 as u64),
        device.global_mem_size().unwrap() / (1e9 as u64)
    );
    let context = opencl3::context::Context::from_device(&device).expect("Context::from_device failed");
    ctx.cl_context.replace(context);
    let queue = opencl3::command_queue::CommandQueue::create_default_with_properties(&ctx.cl_context.as_ref().unwrap(), opencl3::command_queue::CL_QUEUE_PROFILING_ENABLE, 0).expect("CommandQueue::create_default failed");
    ctx.cl_queue.replace(queue);
}

fn main() {
    logging::init_logger();
    let mut ctx = Context::new();

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

    init_cl(&mut ctx);
    let kernel = cl::kernel::Kernel::create_test_kernel(&ctx.cl_context.as_ref().unwrap());
    let data: Vec<f32> = (0..10).map(|i| i as f32).collect();
    kernel.execute_test_kernel(&ctx.cl_context.as_ref().unwrap(), &ctx.cl_queue.as_ref().unwrap(), data);

    log::info!("{:?}", ctx);
    // Loop until the user closes the window
    while !ctx.window.as_ref().unwrap().should_close() {
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
    log::info!("Finished");
}
