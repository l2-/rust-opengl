use crate::common::{flatten_lines, read_lines};
use ocl::Kernel as clKernel;
use ocl::*;

const TEST_KERNEL_PATH: &str = "src/cl/main.cl";
const TEST_KERNEL_NAME: &str = "main";

pub struct Kernel {
    kernel_ref: clKernel,
}

impl Kernel {

    // Somehow still cannot share opengl image between opengl and opencl. when acquiring and releasing it goes wrong. buffer should work though?? or at least the ocl test works

    pub fn execute_test_kernel(&mut self, buf: &Image<i32>, width: &i32, height: &i32) -> () {
        // let mut acquire_globj_event: ocl::Event = ocl::Event::empty();
        // buf.cmd().gl_acquire().enew(&mut acquire_globj_event).enq().unwrap();
        let mut kernel_run_event: ocl::Event = ocl::Event::empty();
        // self.kernel_ref.set_arg(1, width).unwrap();
        // self.kernel_ref.set_arg(2, height).unwrap();
        match unsafe { self.kernel_ref.cmd()
            .enew(&mut kernel_run_event)
            // .ewait(acquire_globj_event)
            .enq() 
        }
        {
            Ok(res) => res,
            Err(err) => { log::error!("{}", err); return; }
        };
        //buf.cmd().gl_release().ewait(&kernel_run_event).enq().unwrap();

        buf.default_queue().unwrap().finish().unwrap();
        
        let start_time = kernel_run_event.profiling_info(enums::ProfilingInfo::Start).unwrap().time().unwrap();
        let end_time = kernel_run_event.profiling_info(enums::ProfilingInfo::End).unwrap().time().unwrap();
        let duration = end_time - start_time;
        log::debug!("kernel execution duration (ns): {}", duration);
    }
    pub fn create_test_kernel(queue: &Queue, source_buf: &Image<i32>) -> Self {
        return Kernel::create(queue, TEST_KERNEL_PATH, TEST_KERNEL_NAME, source_buf);
    }
    pub fn create(queue: &Queue, source_file_path: &str, kernel_name: &str, source_buf: &Image<i32>) -> Self {
        // handle #includes?
        let src = flatten_lines(&read_lines(source_file_path));
        let [width, height, _] = source_buf.dims().to_lens().unwrap();
        let program = ocl::Program::builder()
            .src(&src)
            .devices(queue.device())
            .build(&queue.context())
            .unwrap();
        let mut kernel = ocl::Kernel::builder()
            .queue(queue.to_owned())
            .name(kernel_name)
            // .arg(source_buf)
            // .arg(&(width as i32))
            // .arg(&(height as i32))
            .program(&program)
            .build()
            .unwrap();
        kernel.set_default_global_work_size(source_buf.dims().to_owned());

        return Kernel { kernel_ref: kernel };
    }
}
