// use opencl3::error_codes::ClError;
// use opencl3::memory::*;
// use opencl3::context::*;

// pub fn create_surface(ctx: &Context, width: u32, height: u32) -> Option<Image> {
//     let target = gl::TEXTURE_2D;
//     let mut texture_id : u32 = u32::MAX;
//     unsafe { 
//         gl::GenTextures(1, &mut texture_id); 
//         gl::TexImage2D(target, 0i32, gl::RGB as i32, width as i32, height as i32, 0i32, gl::BGR, gl::UNSIGNED_BYTE, std::ptr::null());
//     };
//     log::debug!("Created texture with id {:?}", texture_id);
//     let surface = unsafe {
//         match Image::create_from_gl_texture(ctx, CL_MEM_READ_ONLY, target, 0, texture_id) {
//             Ok(image) => Some(image),
//             Err(err) => { log::error!("Error creating render surface! {:?}", String::from(err)); return None; },
//         }
//     };
//     return surface;
// }