use std::iter::{ExactSizeIterator, Iterator};
use std::sync::Arc;

use super::window_surface::WindowSurface;

use vulkano::device::physical::{PhysicalDevice, PhysicalDeviceType};
use vulkano::device::{Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo};

pub fn logical_device<'a>(surface: &WindowSurface) -> ((PhysicalDevice, Arc<Device>), impl ExactSizeIterator + Iterator<Item = Arc<Queue>>) {
    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::none()
    };

    //select_physical_device

    let (physical_device, queue_family) = PhysicalDevice::enumerate(&surface.instance)
    .filter(|&p| p.supported_extensions().is_superset_of(&device_extensions))
    .filter_map(|p| {
        p.queue_families()
            .find(|&q| q.supports_graphics() && q.supports_surface(&surface.surface).unwrap_or(false))
            .map(|q| (p, q))
    })
    .min_by_key(|(p, _)| match p.properties().device_type {
        PhysicalDeviceType::DiscreteGpu => 0,
        PhysicalDeviceType::IntegratedGpu => 1,
        PhysicalDeviceType::VirtualGpu => 2,
        PhysicalDeviceType::Cpu => 3,
        PhysicalDeviceType::Other => 4,
    })
    .expect("no device available");

    let (device, queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo::family(queue_family)],
            enabled_extensions: physical_device
                .required_extensions()
                .union(&device_extensions), // new
            ..Default::default()
        },
    )
    .expect("failed to create device");

    ((physical_device, device), queues)
}
