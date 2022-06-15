mod buffer;
mod vertex;

mod vertex_shader {
    vulkano_shaders::shader! {
        ty: "vertex",
        path: "src/shaders/shader.vert"
    }
}

mod fragment_shader {
    vulkano_shaders::shader! {
        ty: "fragment",
        path: "src/shaders/shader.frag"
    }
}

use buffer::{get_command_buffers, get_framebuffers, create_vertex_buffer, create_index_buffer};

use std::sync::Arc;
use super::window_surface::WindowSurface;

use vertex::Vertex;

use vulkano::device::{Device, Queue};
use vulkano::image::SwapchainImage;
use vulkano::pipeline::graphics::input_assembly::InputAssemblyState;
use vulkano::pipeline::graphics::vertex_input::BuffersDefinition;
use vulkano::pipeline::graphics::viewport::{Viewport, ViewportState};
use vulkano::pipeline::GraphicsPipeline;
use vulkano::render_pass::{RenderPass, Subpass};
use vulkano::shader::ShaderModule;
use vulkano::swapchain::{
    AcquireError, Swapchain, SwapchainCreateInfo, SwapchainCreationError, acquire_next_image
};
use vulkano::sync::{self, FenceSignalFuture, FlushError, GpuFuture};


use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

pub fn get_pipeline(device: &Arc<Device>, vertex_shader: &Arc<ShaderModule>, fragment_shader: &Arc<ShaderModule>, render_pass: &Arc<RenderPass>, viewport: &Viewport) -> Arc<GraphicsPipeline> {
    GraphicsPipeline::start()
        .vertex_input_state(BuffersDefinition::new().vertex::<Vertex>())
        .vertex_shader(vertex_shader.entry_point("main").unwrap(), ())
        .input_assembly_state(InputAssemblyState::new())
        .viewport_state(ViewportState::viewport_fixed_scissor_irrelevant([viewport.clone()]))
        .fragment_shader(fragment_shader.entry_point("main").unwrap(), ())
        .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
        .build(device.clone())
        .unwrap()
}

fn get_render_pass(device: &Arc<Device>, swapchain: &Arc<Swapchain<Window>>) -> Arc<RenderPass> {
    vulkano::single_pass_renderpass!(
        device.clone(),
        attachments: {
            color: {
                load: Clear,
                store: Store,
                format: swapchain.image_format(),  // set the format the same as the swapchain
                samples: 1,
            }
        },
        pass: {
            color: [color],
            depth_stencil: {}
        }
    )
    .unwrap()
}

pub fn finalise(device: Arc<Device>, queue: Arc<Queue>, mut surface: WindowSurface, mut swapchain: Arc<Swapchain<Window>>, images: Vec<Arc<SwapchainImage<Window>>>, event_loop: EventLoop<()>) {
    let render_pass = get_render_pass(&device, &swapchain);
    let framebuffers = get_framebuffers(&images, &render_pass);

    vulkano::impl_vertex!(Vertex, position, colour);
    
    let mut size = 0.25;

    let vertex1 = Vertex {
        position: [-0.5* size, 0.5* size] ,
        colour: [255.0, 0.0, 0.0]
    };
    let vertex2 = Vertex {
        position: [0.5* size, 0.5* size],
        colour: [0.0, 255.0, 0.0]
    };
    let vertex3 = Vertex {
        position: [0.5* size, -0.5* size],
        colour: [0.0, 0.0, 255.0]
    };
    let vertex4 = Vertex {
        position: [-0.5* size, -0.5* size],
        colour: [255.0, 255.0, 0.0]
    };
    let vertex_buffer = create_vertex_buffer(vec![vertex1, vertex2, vertex3, vertex4], &queue);
    
    let index_buffer = create_index_buffer(vec![0, 1, 2, 2, 3, 0], &queue);

    println!("{:?}", vertex::get_middle_position(vec![vertex1, vertex2, vertex3, vertex4]).position);

    let vertex_shader = vertex_shader::load(device.clone()).expect("failed to create shader module");
    let fragment_shader = fragment_shader::load(device.clone()).expect("failed to create shader module");

    let mut viewport = Viewport {
        origin: [1.0, 0.0],
        dimensions: surface.surface.window().inner_size().into(),
        depth_range: 0.0..1.0,
    };

    let pipeline = get_pipeline(
        &device,
        &vertex_shader,
        &fragment_shader,
        &render_pass,
        &viewport,
    );

    let mut command_buffers = get_command_buffers(
        &device,
        &queue,
        &pipeline,
        &framebuffers,
        &vertex_buffer,
        &index_buffer,
    );

    let frames_in_flight = images.len();
    let mut fences: Vec<Option<Arc<FenceSignalFuture<_>>>> = vec![None; frames_in_flight];
    let mut previous_fence_i = 0;

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            *control_flow = ControlFlow::Exit;
        }
        Event::WindowEvent {
            event: WindowEvent::Resized(_),
            ..
        } => {
            surface.window_resized = true;
        }
        Event::MainEventsCleared => {
            let new_dimensions = surface.surface.window().inner_size();

            let (new_swapchain, new_images) = match swapchain.recreate(SwapchainCreateInfo {
                image_extent: new_dimensions.into(),
                ..swapchain.create_info()
            }) {
                Ok(r) => r,
                Err(SwapchainCreationError::ImageExtentNotSupported { .. }) => return,
                Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
            };
            swapchain = new_swapchain;
            let new_framebuffers = get_framebuffers(&new_images, &render_pass);

            let new_vertex_buffer = {
                let vertex1 = Vertex { // top right
                    position: [-0.5* size, 0.5* size] ,
                    colour: [255.0, 0.0, 0.0]
                };
                let vertex2 = Vertex { // top left
                    position: [0.5* size, 0.5* size],
                    colour: [0.0, 255.0, 0.0]
                };
                let vertex3 = Vertex {
                    position: [0.5* size, -0.5* size], // bottom left
                    colour: [0.0, 0.0, 255.0]
                };
                let vertex4 = Vertex {
                    position: [-0.5* size, -0.5* size], // bottom right
                    colour: [255.0, 255.0, 0.0]
                };
                
                let new_vertex_buffer = create_vertex_buffer(vec![vertex1, vertex2, vertex3, vertex4], &queue);
                new_vertex_buffer
            };

            //size = size + 0.001;

            viewport.dimensions = new_dimensions.into();

            let new_pipeline = get_pipeline(
                &device,
                &vertex_shader,
                &fragment_shader,
                &render_pass,
                &viewport
            );
            
            command_buffers = get_command_buffers(
                &device,
                &queue,
                &new_pipeline,
                &new_framebuffers,
                &new_vertex_buffer,
                &index_buffer,
            );

            let (image_i, suboptimal, acquire_future) =
                match acquire_next_image(swapchain.clone(), None) {
                    Ok(r) => r,
                    Err(AcquireError::OutOfDate) => {
                        surface.recreate_swapchain = true;
                        return;
                    }
                    Err(e) => panic!("Failed to acquire next image: {:?}", e),
                };

            if suboptimal {
                surface.recreate_swapchain = true;
            }

            // wait for the fence related to this image to finish (normally this would be the oldest fence)
            if let Some(image_fence) = &fences[image_i] {
                image_fence.wait(None).unwrap();
            }

            let previous_future = match fences[previous_fence_i].clone() {
                // Create a NowFuture
                None => {
                    let mut now = sync::now(device.clone());
                    now.cleanup_finished();

                    now.boxed()
                }
                // Use the existing FenceSignalFuture
                Some(fence) => fence.boxed(),
            };

            let future = previous_future
                .join(acquire_future)
                .then_execute(queue.clone(), command_buffers[image_i].clone())
                .unwrap()
                .then_swapchain_present(queue.clone(), swapchain.clone(), image_i)
                .then_signal_fence_and_flush();

            fences[image_i] = match future {
                Ok(value) => Some(Arc::new(value)),
                Err(FlushError::OutOfDate) => {
                    surface.recreate_swapchain = true;
                    None
                }
                Err(e) => {
                    println!("Failed to flush future: {:?}", e);
                    None
                }
            };

            previous_fence_i = image_i;
        }
        _ => (),
    });
}