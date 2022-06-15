use std::sync::Arc;
use vulkano::device::physical::PhysicalDevice;
use vulkano::device::Device;
use vulkano::image::{ImageUsage, SwapchainImage};
use vulkano::swapchain::{
    Surface, Swapchain, SwapchainCreateInfo,
};

use winit::window::Window;

pub fn get_swapchain(window: &Arc<Surface<Window>>, physical_device: &PhysicalDevice, device: &Arc<Device>) -> (Arc<Swapchain<Window>>, Vec<Arc<SwapchainImage<Window>>>) {
    let (swapchain, images) = {
        let caps = physical_device
            .surface_capabilities(&window, Default::default())
            .expect("failed to get surface capabilities");

        let dimensions = window.window().inner_size();
        let composite_alpha = caps.supported_composite_alpha.iter().next().unwrap();
        let image_format = Some(
            physical_device
                .surface_formats(&window, Default::default())
                .unwrap()[0]
                .0,
        );

        Swapchain::new(
            device.clone(),
            window.clone(),
            SwapchainCreateInfo {
                min_image_count: caps.min_image_count,
                image_format,
                image_extent: dimensions.into(),
                image_usage: ImageUsage::color_attachment(),
                composite_alpha,
                ..Default::default()
            },
        )
        .unwrap()
    };
    
    (swapchain, images)
}