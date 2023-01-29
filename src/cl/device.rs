pub fn find_most_capable_device() -> Option<opencl3::device::Device> {
    return match opencl3::device::get_all_devices(opencl3::device::CL_DEVICE_TYPE_GPU) {
        Ok(devices) => {
            let mut max_compute_units = 0;
            let mut best_device = None;
            let mut device_names: Vec<String> = Vec::new();
            for device_id in devices {
                let device = opencl3::device::Device::new(device_id);
                device_names.push(device.name().unwrap());
                if max_compute_units < device.max_compute_units().unwrap() {
                    best_device = Some(device);
                    max_compute_units = device.max_compute_units().unwrap();
                }
            }
            log::debug!("OpenCL devices: {:?}", device_names);
            return best_device;
        }
        Err(_) => None,
    };
}
