pub mod buffer;
pub mod camera;
pub mod context;
pub mod light;
pub mod mesh;
pub mod texture;
pub mod render_pass;
pub mod compute_pass;
pub mod shader;
// pub mod svo;
pub mod scene;

pub mod passes;

pub use scene::*;
pub use shader::*;
pub use buffer::*;
pub use camera::*;
pub use context::*;
pub use light::*;
pub use mesh::*;
pub use render_pass::*;
pub use texture::*;
pub use compute_pass::*;
// pub use svo::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct Size(pub u32, pub u32);

pub fn fovy_to_fovx(fovy: f32, aspect: f32) -> f32 {
    // fieldOfViewX = 2 * atan(tan(fieldOfViewY * 0.5) * aspect)
    ((fovy * 0.5).tan() * aspect).atan()
}
pub fn fovx_to_fovy(fovx: f32, aspect: f32) -> f32 {
    // fieldOfViewX = 2 * atan(tan(fieldOfViewY * 0.5) * aspect)
    // fieldOfViewX*0.5 = atan(tan(fieldOfViewY * 0.5) * aspect)
    // tan(fieldOfViewX*0.5) = tan(fieldOfViewY * 0.5) * aspect
    // fieldOfViewY = 2 * atan(tan(fieldOfViewX*0.5)/aspect)
    ((fovx * 0.5).tan() / aspect).atan()
}
