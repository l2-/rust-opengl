use crate::common::{read_lines, flatten_lines};
use opencl3::command_queue::{CommandQueue};
use opencl3::context::Context;
use opencl3::kernel::{ExecuteKernel, Kernel as clKernel};
use opencl3::memory::{Buffer, CL_MEM_READ_ONLY, CL_MEM_WRITE_ONLY};
use opencl3::program::Program;
use opencl3::types::{cl_float, CL_BLOCKING};
use std::ptr;

const TEST_KERNEL_PATH: &str = "src/cl/main.cl";
const TEST_KERNEL_NAME: &str = "square";

pub struct Kernel {
    kernel_ref: clKernel
}

impl Kernel {
    pub fn create(ctx: &Context, source_file_path: &str, kernel_name: &str) -> Self {
        // handle #includes?
        let src = flatten_lines(&read_lines(source_file_path));
        let program = Program::create_and_build_from_source(ctx, &src, "")
            .expect("Program::create_and_build_from_source failed");
        let kernel = clKernel::create(&program, kernel_name).expect("Kernel::create failed");
        return Kernel{ kernel_ref: kernel };
    }
    pub fn create_test_kernel(ctx: &Context) -> Self {
        return Kernel::create(ctx, TEST_KERNEL_PATH, TEST_KERNEL_NAME);
    }
    pub fn execute_test_kernel(&self, ctx: &Context, queue: &CommandQueue, in_1: Vec<f32>) -> () {
        let mut in_buffer = unsafe {
            Buffer::<cl_float>::create(&ctx, CL_MEM_READ_ONLY, in_1.len(), ptr::null_mut()).unwrap()
        };
        let out_buffer = unsafe {
            Buffer::<cl_float>::create(&ctx, CL_MEM_WRITE_ONLY, in_1.len(), ptr::null_mut()).unwrap()
        };
        unsafe { queue.enqueue_write_buffer(&mut in_buffer, CL_BLOCKING, 0, &in_1, &[]).unwrap() };
        let kernel_event = unsafe { 
            ExecuteKernel::new(&self.kernel_ref)
                .set_arg(&in_buffer)
                .set_arg(&out_buffer)
                .set_arg(&(in_1.len() as u32))
                .set_global_work_size(in_1.len())
                .enqueue_nd_range(&queue).unwrap()
        };
        let mut results: Vec<f32> = vec![0.0f32; in_1.len()];
        unsafe { queue.enqueue_read_buffer(&out_buffer, CL_BLOCKING, 0, &mut results, &[]).unwrap() };
        log::info!("Result {:?}", results);

        let start_time = kernel_event.profiling_command_start().unwrap();
        let end_time = kernel_event.profiling_command_end().unwrap();
        let duration = end_time - start_time;
        log::debug!("kernel execution duration (ns): {}", duration);
    }
}
