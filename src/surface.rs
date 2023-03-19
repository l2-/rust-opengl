use ocl::{Queue, Image, enums::MemObjectType, core::GlTextureTarget};

pub fn create_surface(queue: &Queue, width: usize, height: usize) -> Result<Image<i32>, ocl::Error>  {
    let target = gl::TEXTURE_2D;
    let mut texture_id : u32 = u32::MAX;
    unsafe { 
        gl::GenTextures(1, &mut texture_id);
        gl::BindTexture(target, texture_id);
        gl::TexImage2D(target, 0i32, gl::RGBA as i32, width as i32, height as i32, 0i32, gl::RGBA, gl::UNSIGNED_BYTE, std::ptr::null());
        gl::BindTexture(target, 0);
    };
    log::debug!("Created texture with id {:?}", texture_id);
    let flags = ocl::MemFlags::new();
    let image_desc = ocl::builders::ImageDescriptor::new(MemObjectType::Image2d, width, height, 1, 0, 0, 0, None);
    let cl_buffer = ocl::Image::from_gl_texture(queue, flags, image_desc, GlTextureTarget::GlTexture2d, 0i32, texture_id);
    return cl_buffer;
}