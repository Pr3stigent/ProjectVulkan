use std::sync::Arc;

use crate::geometry::Vertex;

use vulkano::buffer::{BufferUsage, TypedBufferAccess};
use vulkano::buffer::immutable::ImmutableBuffer;
use vulkano::command_buffer::{
    AutoCommandBufferBuilder, CommandBufferUsage, PrimaryAutoCommandBuffer, SubpassContents,
};

use vulkano::device::{Device, Queue};
use vulkano::image::{SwapchainImage, view::ImageView};
use vulkano::pipeline::GraphicsPipeline;
use vulkano::render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass};
use vulkano::sync::GpuFuture;
use winit::window::Window;

pub fn get_framebuffers(images: &[Arc<SwapchainImage<Window>>], render_pass: &Arc<RenderPass>) -> Vec<Arc<Framebuffer>> {
    images
        .iter()
        .map(|image| {
            let view = ImageView::new_default(image.clone()).unwrap();
            Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![view],
                    ..Default::default()
                },
            )
            .unwrap()
        })
        .collect::<Vec<_>>()
}

pub fn get_command_buffers(device: &Arc<Device>, queue: &Arc<Queue>, pipeline: &Arc<GraphicsPipeline>, framebuffers: &Vec<Arc<Framebuffer>>, vertex_buffer: &Arc<ImmutableBuffer<[Vertex]>>, index_buffer: &Arc<ImmutableBuffer<[u32]>>) -> Vec<Arc<PrimaryAutoCommandBuffer>> {
    framebuffers
        .iter()
        .map(|framebuffer| {
            let mut builder = AutoCommandBufferBuilder::primary(
                device.clone(),
                queue.family(),
                CommandBufferUsage::MultipleSubmit,
            )
            .unwrap();

            builder
                .begin_render_pass(
                    framebuffer.clone(),
                    SubpassContents::Inline,
                    vec![[0.0, 0.0, 0.0, 1.0].into()], // clear colour
                )
                .unwrap()
                
                .bind_pipeline_graphics(pipeline.clone())
                .bind_vertex_buffers(0, vertex_buffer.clone())
                .bind_index_buffer(index_buffer.clone())
                .draw_indexed(index_buffer.len() as u32, 1, 0, 0, 0)
                .unwrap()
                .end_render_pass()
                .unwrap();

            Arc::new(builder.build().unwrap())
        })
        .collect()
}

pub fn create_vertex_buffer(vertices: Vec<Vertex>, queue: &Arc<Queue>) -> Arc<ImmutableBuffer<[Vertex]>> {
    /*CpuAccessibleBuffer::from_iter(
        device.clone(),
        BufferUsage::vertex_buffer(),
        false,
        vertices.into_iter(),
    )
    .unwrap()*/
    let (buffer, future) = ImmutableBuffer::from_iter(
        vertices.iter().cloned(), BufferUsage::vertex_buffer(),
        queue.clone())
        .unwrap();
        future.flush().unwrap();
    
    buffer
}

pub fn create_index_buffer(indices: Vec<u32> , queue: &Arc<Queue>) -> Arc<ImmutableBuffer<[u32]>> {
    let (buffer, future) = ImmutableBuffer::from_iter(
        indices.iter().cloned(), BufferUsage::index_buffer(),
        queue.clone())
        .unwrap();
        future.flush().unwrap();
    
    buffer
}