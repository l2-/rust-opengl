use ocl::*;

pub fn find_most_capable_device() -> Option<ocl::Device> {
    return match Device::list_all(Platform::default()) {
        Ok(devices) => {
            if devices.len() < 1 { return None; }
            let mut best_device = devices.get(0);
            devices.iter().for_each(|device| {
                let cus = device.get_compute_units();
                log::debug!("Device {:?} cus:{:?}", device.name().unwrap(), cus);
                if cus > best_device.unwrap().get_compute_units() {
                    best_device = Some(device);
                }
            });
            return best_device.copied();
        },
        Err(err) => { log::error!("Error finding OpenCL compatible devices {:?}", err); None },
    };
}

pub trait OclDevice {
    fn get_compute_units(&self) -> i64;
    fn get_local_mem_size(&self) -> i64;
    fn get_global_mem_size(&self) -> i64;
}

impl OclDevice for ocl::Device {
    fn get_compute_units(&self) -> i64 {
        match self.info(ocl::enums::DeviceInfo::MaxComputeUnits) {
            Ok(cus) => (&cus.to_string()).parse::<i64>().unwrap(),
            Err(_) => -1i64,
        }
    }
    fn get_local_mem_size(&self) -> i64 {
        match self.info(ocl::enums::DeviceInfo::LocalMemSize) {
            Ok(cus) => (&cus.to_string()).parse::<i64>().unwrap(),
            Err(_) => -1i64,
        }
    }
    fn get_global_mem_size(&self) -> i64 {
        match self.info(ocl::enums::DeviceInfo::GlobalMemSize) {
            Ok(cus) => (&cus.to_string()).parse::<i64>().unwrap(),
            Err(_) => -1i64,
        }
    }
}