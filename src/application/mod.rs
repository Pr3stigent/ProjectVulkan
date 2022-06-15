mod device_creation;
mod graphics_pipeline;
mod swapchain;
pub mod window_surface;

use device_creation::logical_device;

use swapchain::get_swapchain;

use vulkano::instance::{Instance, InstanceCreateInfo};

use window_surface::WindowSurface;
use winit::event_loop::EventLoop;

pub fn init(name: &str, dimensions: [u32; 2]) {
    let instance = Instance::new(InstanceCreateInfo {
        enabled_extensions: vulkano_win::required_extensions(),
        ..Default::default()
    })
    .expect("failed to create instance");

    let event_loop = EventLoop::new();

    let window = WindowSurface::new(name, dimensions, instance.clone(), &event_loop);

    let ((physical_device, device), mut queues) = logical_device(&window);

    let queue = queues.next().unwrap();

    let (swapchain, images) = get_swapchain(&window.surface, &physical_device, &device);

    graphics_pipeline::finalise(device, queue, window, swapchain, images, event_loop);
}