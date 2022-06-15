use vulkano::instance::Instance;
use vulkano::swapchain::Surface;

use vulkano_win::VkSurfaceBuild;

use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

use std::sync::Arc;

#[derive(Clone)]
pub struct WindowSurface {
    pub instance: Arc<Instance>,
    pub recreate_swapchain: bool,
    pub surface: Arc<Surface<Window>>,
    pub window_resized: bool,
}

impl WindowSurface {
    
    pub fn new(name: &str, dimensions: [u32; 2], instance: Arc<Instance>, event_loop: &EventLoop<()>) ->  WindowSurface {
        
        let window = WindowBuilder::new()
        .with_title(name)
        .with_inner_size(LogicalSize {
            width: dimensions[0],
            height: dimensions[1],
        })
        .build_vk_surface(&event_loop, instance.clone())
        .unwrap();

        WindowSurface {
            instance: instance,
            surface: window,
            window_resized: false,
            recreate_swapchain: false,
        }
    }
}