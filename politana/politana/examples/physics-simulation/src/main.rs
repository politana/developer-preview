use politana::{El, IntoLength, Politana, View, library::{GpuContext, RenderMode, RenderOptions, Renderer, WebGPUCanvas}};
use wgpu::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BlendState, Buffer, BufferBindingType, BufferDescriptor, BufferUsages, ColorTargetState, ColorWrites, CommandEncoderDescriptor, ComputePass, ComputePassDescriptor, ComputePipeline, ComputePipelineDescriptor, Device, FragmentState, FrontFace, LoadOp, MultisampleState, Operations, PipelineCompilationOptions, PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology, RenderPass, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, ShaderStages, StoreOp, TextureViewDescriptor, VertexAttribute, VertexBufferLayout, VertexFormat, VertexState, VertexStepMode, include_wgsl};

use crate::model::{Attractor, Particle};

mod model;

const PARTICLE_DIMENSION: u64 = 1000;
const N_ATTRACTORS: u64 = 2;

struct AppData {
    particle_buffer: Buffer,
    compute_bind_group: BindGroup,
    compute_pipeline: ComputePipeline,
    render_pipeline: RenderPipeline,
}

#[View]
fn Simulation() -> El {
    let renderer = Renderer {
        render_mode: RenderMode::EveryFrame,
        setup: |context: GpuContext| {
            let device: Device = context.device();
            let initial_particles = {
                let mut initial_particles = Vec::with_capacity(PARTICLE_DIMENSION.pow(2) as usize);
                let dim = PARTICLE_DIMENSION as f32;
                for i in 0..PARTICLE_DIMENSION {
                    for j in 0..PARTICLE_DIMENSION {
                        let x = (i as f32 / (dim - 1.0)) * 2.0 - 1.0;
                        let y = (j as f32 / (dim - 1.0)) * 2.0 - 1.0;
                        initial_particles.push(Particle {
                            x, y, dx: 0.0, dy: 0.0,
                        });
                    }
                }
                initial_particles
            };
            let initial_attractors = [
                Attractor { x: -0.5, y: 0.0, strength: 0.2, _padding: 0.0 },
                Attractor { x: 0.5, y: 0.0, strength: 0.2, _padding: 0.0 },
            ];
            let particle_buffer = device.create_buffer(&BufferDescriptor {
                label: Some("Particles"),
                size: size_of::<Particle>() as u64 * PARTICLE_DIMENSION.pow(2),
                usage: BufferUsages::STORAGE | BufferUsages::VERTEX | BufferUsages::COPY_DST,
                mapped_at_creation: false
            });
            context.queue().write_buffer(&particle_buffer, 0, bytemuck::cast_slice(&initial_particles));
            let attractor_buffer = device.create_buffer(&BufferDescriptor {
                label: Some("Attractors"),
                size: size_of::<Attractor>() as u64 * N_ATTRACTORS,
                usage: BufferUsages::COPY_DST | BufferUsages::STORAGE,
                mapped_at_creation: false
            });
            context.queue().write_buffer(&attractor_buffer, 0, bytemuck::cast_slice(&initial_attractors));
            let compute_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("Compute bind group layout"),
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Storage { read_only: false },
                            has_dynamic_offset: false,
                            min_binding_size: None
                        },
                        count: None
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStages::COMPUTE,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Storage { read_only: true },
                            has_dynamic_offset: false,
                            min_binding_size: None
                        },
                        count: None
                    }
                ]
            });
            let compute_bind_group = device.create_bind_group(&BindGroupDescriptor {
                label: Some("Compute bind group"),
                layout: &compute_bind_group_layout,
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: particle_buffer.as_entire_binding()
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: attractor_buffer.as_entire_binding()
                    }
                ]
            });
            let shader_module = device.create_shader_module(include_wgsl!("shader.wgsl"));
            let compute_pipeline = device.create_compute_pipeline(&ComputePipelineDescriptor {
                label: Some("Compute pipeline"),
                layout: Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor {
                    label: Some("Compute pipeline layout"),
                    bind_group_layouts: &[&compute_bind_group_layout],
                    push_constant_ranges: &[]
                })),
                module: &shader_module,
                entry_point: Some("step_physics"),
                compilation_options: Default::default(),
                cache: None
            });
            let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
                label: Some("Render pipeline"),
                layout: None,
                vertex: VertexState {
                    module: &shader_module,
                    entry_point: Some("vertex"),
                    compilation_options: PipelineCompilationOptions::default(),
                    buffers: &[
                        VertexBufferLayout {
                            array_stride: size_of::<Particle>() as u64,
                            step_mode: VertexStepMode::Vertex,
                            attributes: &[
                                VertexAttribute {
                                    format: VertexFormat::Float32x2,
                                    offset: 0,
                                    shader_location: 0
                                },
                                VertexAttribute {
                                    format: VertexFormat::Float32x2,
                                    offset: 2 * size_of::<f32>() as u64,
                                    shader_location: 1
                                },
                            ]
                        }
                    ]
                },
                primitive: PrimitiveState {
                    topology: PrimitiveTopology::PointList,
                    strip_index_format: None,
                    front_face: FrontFace::Ccw,
                    cull_mode: None,
                    unclipped_depth: false,
                    polygon_mode: PolygonMode::Fill,
                    conservative: false,
                },
                depth_stencil: None,
                multisample: MultisampleState {
                    count: 1,
                    mask: !0,
                    alpha_to_coverage_enabled: false
                },
                fragment: Some(FragmentState {
                    module: &shader_module,
                    entry_point: Some("fragment"),
                    compilation_options: Default::default(),
                    targets: &[
                        Some(ColorTargetState {
                            format: context.surface_config().format,
                            blend: Some(BlendState::REPLACE),
                            write_mask: ColorWrites::ALL
                        })
                    ]
                }),
                multiview: None,
                cache: None
            });
            AppData { particle_buffer, compute_bind_group, compute_pipeline, render_pipeline }
        },
        render_frame: |context: GpuContext, app_data: &mut AppData| {
            let frame = context.surface().get_current_texture().unwrap();
            let view = frame.texture.create_view(&TextureViewDescriptor::default());
            let mut encoder = context.device().create_command_encoder(&CommandEncoderDescriptor::default());
            {
                let mut compute_pass: ComputePass = encoder.begin_compute_pass(&ComputePassDescriptor {
                    label: Some("Compute pass"),
                    ..Default::default()
                });
                compute_pass.set_pipeline(&app_data.compute_pipeline);
                compute_pass.set_bind_group(0, &app_data.compute_bind_group, &[]);
                compute_pass.dispatch_workgroups(PARTICLE_DIMENSION as u32, PARTICLE_DIMENSION as u32, 1);
            }
            {
                let mut render_pass: RenderPass = encoder.begin_render_pass(&RenderPassDescriptor {
                    label: Some("Render pass"),
                    color_attachments: &[Some(RenderPassColorAttachment {
                        view: &view,
                        depth_slice: None,
                        resolve_target: None,
                        ops: Operations {
                            load: LoadOp::Clear(wgpu::Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 }),
                            store: StoreOp::Store
                        }
                    })],
                    ..Default::default()
                });
                render_pass.set_pipeline(&app_data.render_pipeline);
                render_pass.set_vertex_buffer(0, app_data.particle_buffer.slice(..));
                render_pass.draw(0..(PARTICLE_DIMENSION.pow(2) as u32), 0..1);
            }
            context.queue().submit([encoder.finish()]);
            frame.present();
        },
        teardown: |_, _| {},
        options: RenderOptions::new()
    };
    WebGPUCanvas(renderer)
}

#[View]
fn App() -> El {
    Simulation()
        .width(100.vw())
        .height(100.vh())
        .global_css("
            body {
                padding: 0;
                margin: 0;
            }
        ")
}

fn main() {
    Politana::launch(App);
}
