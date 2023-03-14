// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
extern crate glfw;
extern crate ocl;
extern crate ocl_interop;
extern crate pretty_env_logger;

mod surface;
mod common;
mod logging;
mod mesh;
mod cl {
    pub mod device;
    pub mod kernel;
}
mod shader;

use std::any;

use common::*;
use glfw::{ffi::glfwWindowHint, Action, Context as GLFWContext, Key, Window};
use mesh::*;
use shader::*;
use cl::device::*;

const WINDOW_TITLE: &str = "Simple Ray Tracer";
const VERTICES: [Vertex; 3] = [(-0.5, -0.5, 0.0), (0.5, -0.5, 0.0), (0.0, 0.5, 0.0)];
const CLEAR_COLOR: [f32; 4] = [0.8, 0.8, 0.8, 1.0];
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const CL_GL_CONTEXT_KHR: isize = 0x2008;
const CL_WGL_HDC_KHR: isize = 0x200B;
const CL_CONTEXT_PLATFORM: isize = 0x1084;

#[derive(Debug)]
struct Context {
    shader_program: u32,
    meshes: Vec<Mesh>,
    window: Window,
    cl_context: ocl::Context,
    cl_queue: ocl::Queue,
}

fn init_gl(window: &mut Window) -> u32 {
    unsafe {
        gl::load_with(|f_name| window.get_proc_address(f_name));
        gl::ClearColor(
            CLEAR_COLOR[0],
            CLEAR_COLOR[1],
            CLEAR_COLOR[2],
            CLEAR_COLOR[3],
        );
    }
    return Shader::_create().shader_program;
}

fn init_cl(window: &mut Window) -> ocl::Result<(ocl::Context, ocl::Queue)> {
    let device = match cl::device::find_most_capable_device() {
        Some(device) => device,
        None => panic!("No opencl capable device"),
    };
    log::info!("Most capable OpenCL 3.0 device {:?}, Compute units: {:?}, local_mem: {:.3}K, global_mem: {:.3}M", 
        device.name().unwrap(),
        device.get_compute_units(),
        device.get_local_mem_size() as f64 / 2f64.powi(10),
        device.get_global_mem_size() as f64 / 2f64.powi(10).powi(3),
    );

    let mut properties = ocl::builders::ContextProperties::new();
    properties.set_wgl_hdc(window.get_wgl_context() as *mut _);
    properties.set_gl_context(unsafe {glfw::ffi::glfwGetWGLContext(window.window_ptr())} as *mut _);
    let context = ocl::Context::builder()
        .properties(properties)
        .devices(device.clone())
        .platform(ocl::Platform::default())
        .build()?;
    
    let queue = ocl::Queue::new(&context, device, None)?;

    return ocl::Result::Ok((context, queue));
    
    // switch opencl3 to ocl https://github.com/Nopey/rust-ocl-interop
    // let platform = device.platform().unwrap();
    // let _1 = ctx.window.as_ref().unwrap().get_wgl_context();
    // let _2 = ctx.window.as_ref().unwrap().get_win32_window();
    // let _3 = ctx.window.as_mut().unwrap().get_proc_address(&String::from("simpleraytracer"));
    // let window_ctx = unsafe { glfw::ffi::glfwGetWGLContext(ctx.window.as_ref().unwrap().window_ptr()) };
    // let properties = [
    //     CL_GL_CONTEXT_KHR, window_ctx as isize,
    //     // CL_WGL_HDC_KHR, 
    //     CL_CONTEXT_PLATFORM, platform as isize, 0];
    // cl_context_properties props[] =
    // {
    //     CL_GL_CONTEXT_KHR, (cl_context_properties)glfwGetWGLContext( window ),
    //     CL_WGL_HDC_KHR, (cl_context_properties)wglGetCurrentDC(),
    //     CL_CONTEXT_PLATFORM, (cl_context_properties)platform, 0
    // };
    // let context = match opencl3::context::Context::from_sub_devices(&[opencl3::device::SubDevice::new(device.id())], &properties, None, std::ptr::null_mut()) {
    //     Ok(ctx) => ctx,
    //     Err(err) => panic!("Context::create failed {:?}", String::from(err)),
    // };
    // let context = opencl3::context::Context::from_device(&device).expect("Context::from_device failed");
    // ctx.cl_context.replace(context);
    // match ctx.cl_context.as_ref().unwrap().get_supported_image_formats(opencl3::memory::CL_MEM_READ_ONLY, opencl3::memory::CL_MEM_OBJECT_IMAGE2D) {
    //     Ok(properties) => log::debug!("Properties {:?}", properties),
    //     Err(err) => log::debug!("Error retrieving properties {:?}", String::from(err)),
    // }
    
    // let queue = opencl3::command_queue::CommandQueue::create_default_with_properties(&ctx.cl_context.as_ref().unwrap(), opencl3::command_queue::CL_QUEUE_PROFILING_ENABLE, 0).expect("CommandQueue::create_default failed");
    // ctx.cl_queue.replace(queue);
}

fn main() {
    logging::init_logger();

    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    unsafe {
        glfwWindowHint(glfw::ffi::CONTEXT_VERSION_MAJOR, 3);
        glfwWindowHint(glfw::ffi::CONTEXT_VERSION_MINOR, 3);
        glfwWindowHint(glfw::ffi::OPENGL_PROFILE, glfw::ffi::OPENGL_CORE_PROFILE);
        //glfwWindowHint(glfw::ffi::DOUBLEBUFFER, gl::FALSE as i32); // single buffer. double = vsync?
        glfwWindowHint(glfw::ffi::OPENGL_FORWARD_COMPAT, gl::FALSE as i32);
    }

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw
        .create_window(WIDTH, HEIGHT, WINDOW_TITLE, glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);

    let shader_program = init_gl(&mut window);
    let meshes = vec![Mesh::create(Vec::from(VERTICES))];

    let (cl_context, cl_queue) = match init_cl(&mut window) {
        Ok(res) => res,
        Err(err) => panic!("{}", err.to_string()),
    };

    // let kernel = cl::kernel::Kernel::create_test_kernel(&ctx.cl_context.as_ref().unwrap());
    // let data: Vec<f32> = (0..10).map(|i| i as f32).collect();
    // kernel.execute_test_kernel(&ctx.cl_context.as_ref().unwrap(), &ctx.cl_queue.as_ref().unwrap(), data);

    // let img = surface::create_surface(&ctx.cl_context.as_ref().unwrap(), WIDTH, HEIGHT);
    // log::info!("Image info {:?}", img);
    // use opencl3::memory::Image::create(context, flags, image_format, image_desc, host_ptr) to create image texture you can write to
    // see create_from_gl_texture()
    // https://docs.rs/opencl3/0.9.2/opencl3/memory/struct.Image.html
    log::info!("{:?} {:?} {:?} {:?} {:?}", shader_program, meshes, window, cl_context, cl_queue);
    // Loop until the user closes the window
    while !window.should_close() {
        // Swap front and back buffers
        window.swap_buffers();

        // Poll for and process events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }
                _ => {}
            }
        }

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            meshes.iter().for_each(|m| {
                gl::BindVertexArray(m.vao);
                gl::DrawArrays(gl::TRIANGLES, 0, m.vertex_count);
            });
        }
    }
    log::info!("Finished");
}
