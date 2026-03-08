use wgpu::PowerPreference;

use crate::library::webgpu::gpu_context::GpuContext;

pub struct Renderer<AppData, Setup, RenderFrame, Teardown>
where Setup: Fn(GpuContext) -> AppData,
      RenderFrame: Fn(GpuContext, &mut AppData),
      Teardown: Fn(GpuContext, &mut AppData) {
    pub render_mode: RenderMode,
    pub setup: Setup,
    pub render_frame: RenderFrame,
    pub teardown: Teardown,
    pub options: RenderOptions<AppData>
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum RenderMode {
    EveryFrame, OnStateChanged
}

pub struct RenderOptions<AppData> {
    pub(crate) effects: Vec<Box<dyn Fn(GpuContext, &mut AppData)>>,
    pub(crate) power_preference: PowerPreference
}

impl <AppData> RenderOptions<AppData> {
    pub fn new() -> RenderOptions<AppData> {
        Self {
            effects: Vec::new(),
            power_preference: PowerPreference::LowPower
        }
    }

    pub fn effect(
        mut self,
        effect: impl Fn(GpuContext, &mut AppData) + 'static
    ) -> Self {
        self.effects.push(Box::new(effect));
        self
    }

    pub fn power_preference(mut self, preference: PowerPreference) -> Self {
        self.power_preference = preference;
        self
    }
}
