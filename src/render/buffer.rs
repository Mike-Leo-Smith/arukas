use std::marker::PhantomData;

use wgpu::util::DeviceExt;

use super::RenderContext;
pub trait BufferData: Default + Copy + Clone + bytemuck::Pod + bytemuck::Zeroable {
    type Native;
    fn update(&mut self, value: &Self::Native);
}

pub fn create_uniform_bind_group_layout(
    ctx: &RenderContext,
    binding: u32,
    visibility: wgpu::ShaderStage,
    label: Option<&str>,
) -> wgpu::BindGroupLayout {
    ctx.device
        .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[wgpu::BindGroupLayoutEntry {
                binding,
                visibility,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label,
        })
}

pub struct Buffer<T>
where
    T: BufferData,
{
    pub buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    pub binding:u32,
    pub layout: wgpu::BindGroupLayout,
    marker: PhantomData<T>,
}
impl<T> Buffer<T>
where
    T: BufferData,
{
    pub fn new_uniform_buffer(
        ctx: &RenderContext,
        binding: u32,
        visibility: wgpu::ShaderStage,
        init: &[T],
        label: Option<&str>,
    ) -> Self {
        Self::new(
            ctx,
            binding,
            visibility,
            wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
            &init,
            label,
        )
    }
    pub fn new_storage_buffer(
        ctx: &RenderContext,
        binding: u32,
        visibility: wgpu::ShaderStage,
        init: &[T],
        label: Option<&str>,
    ) -> Self {
        Self::new(
            ctx,
            binding,
            visibility,
            wgpu::BufferUsage::STORAGE | wgpu::BufferUsage::COPY_DST,
            &init,
            label,
        )
    }
    pub fn new(
        ctx: &RenderContext,
        binding: u32,
        visibility: wgpu::ShaderStage,
        usage: wgpu::BufferUsage,
        init: &[T],
        label: Option<&str>,
    ) -> Self {
        let device = &ctx.device;
        let layout = create_uniform_bind_group_layout(ctx, binding, visibility, label);
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Buffer"),
            contents: bytemuck::cast_slice(init),
            usage: usage, //wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: binding,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("buffer_bind_group"),
        });
        Self {
            buffer,
            bind_group,
            layout,
            binding,
            marker: PhantomData,
        }
    }
    pub fn upload(&self, ctx: &RenderContext, values: &[T]) {
        ctx.queue
            .write_buffer(&self.buffer, 0, bytemuck::cast_slice(values));
    }
}
