use std::rc::Rc;

use wgpu::{Device, Queue, Surface, SurfaceConfiguration};

#[derive(Clone)]
pub struct GpuContext {
    pub(crate) device: Device,
    pub(crate) queue: Queue,
    pub(crate) surface: Rc<Surface<'static>>,
    pub(crate) surface_config: SurfaceConfiguration
}

impl GpuContext {
    pub fn device(&self) -> Device { self.device.clone() }
    pub fn queue(&self) -> Queue { self.queue.clone() }
    pub fn surface(&self) -> Rc<Surface<'static>> { self.surface.clone() }
    pub fn surface_config(&self) -> SurfaceConfiguration { self.surface_config.clone() }
}
