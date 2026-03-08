use politana::{Button, Color, Display, Div, El, Input, InputType, IntoLength, Label, P, Politana, State, Step, TypedEventTargets, UniqueId, View, library::{RenderMode, RenderOptions, Renderer, WebGPUCanvas}};
use wgpu::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, BlendState, Buffer, BufferBindingType, BufferDescriptor, BufferUsages, ColorTargetState, ColorWrites, CommandEncoderDescriptor, Device, FragmentState, FrontFace, LoadOp, MultisampleState, Operations, PipelineCompilationOptions, PipelineLayoutDescriptor, PolygonMode, PrimitiveState, PrimitiveTopology, RenderPass, RenderPassColorAttachment, RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, ShaderStages, StoreOp, TextureViewDescriptor, VertexState, include_wgsl};

struct AppData {
    color_buffer: Buffer,
    render_pipeline: RenderPipeline,
    bind_group: BindGroup
}

#[View]
fn Triangle(
    red_component: State<f32>,
    green_component: State<f32>,
    render_count: State<i32>
) -> El {
    let renderer = Renderer {
        render_mode: RenderMode::OnStateChanged,
        setup: |context| {
            let device: Device = context.device();
            let color_buffer = device.create_buffer(&BufferDescriptor {
                label: None,
                size: size_of::<f32>() as u64 * 2,
                usage: BufferUsages::COPY_DST | BufferUsages::UNIFORM,
                mapped_at_creation: false
            });
            let bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: None,
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::VERTEX,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None
                        },
                        count: None
                    }
                ]
            });
            let bind_group = device.create_bind_group(&BindGroupDescriptor {
                label: None,
                layout: &bind_group_layout,
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: color_buffer.as_entire_binding()
                    }
                ]
            });
            let shader_module = device.create_shader_module(include_wgsl!("shader.wgsl"));
            let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
                label: None,
                layout: Some(&device.create_pipeline_layout(&PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[&bind_group_layout],
                    push_constant_ranges: &[]
                })),
                vertex: VertexState {
                    module: &shader_module,
                    entry_point: Some("vertex"),
                    compilation_options: PipelineCompilationOptions::default(),
                    buffers: &[]
                },
                primitive: PrimitiveState {
                    topology: PrimitiveTopology::TriangleStrip,
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
            AppData { render_pipeline, color_buffer, bind_group }
        },
        render_frame: |context, app_data| {
            render_count.set(|c| c + 1);
            let components = [red_component.get(), green_component.get()];
            context.queue().write_buffer(&app_data.color_buffer, 0, bytemuck::cast_slice(&components));
            let frame = context.surface().get_current_texture().unwrap();
            let view = frame.texture.create_view(&TextureViewDescriptor::default());
            let mut encoder = context.device().create_command_encoder(&CommandEncoderDescriptor::default());
            {
                let mut render_pass: RenderPass = encoder.begin_render_pass(&RenderPassDescriptor {
                    color_attachments: &[Some(RenderPassColorAttachment {
                        view: &view,
                        depth_slice: None,
                        resolve_target: None,
                        ops: Operations {
                            load: LoadOp::Clear(wgpu::Color { r: 0.3, g: 0.3, b: 0.3, a: 1.0 }),
                            store: StoreOp::Store
                        }
                    })],
                    ..Default::default()
                });
                render_pass.set_pipeline(&app_data.render_pipeline);
                render_pass.set_bind_group(0, &app_data.bind_group, &[]);
                render_pass.draw(0..4, 0..1);
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
fn Slider(title: &'static str, value: State<f32>) -> El {
    let id = UniqueId::new();
    Div((
        || Label(title)
            .label_for(id)
            .min_width(60.px()),
        || Input()
            .input_type(InputType::Range)
            .id(id)
            .value("0")
            .min_max_step(0.0, 1.0, Step::Any)
            .width(200.px())
            .on_input(|event| {
                let Ok(v) = event.input_target().value().parse()
                    else { return };
                value.put(v);
            })
    ))
        .display(Display::Flex)
}

#[View]
fn App() -> El {
    let red_component = State::new(0.0);
    let green_component = State::new(0.0);
    let render_count = State::new(0);
    let is_showing_gradient = State::new(true);
    Div((
        || Slider("Red", red_component)
            .color(Color::Rgba(255.0, 0.0, 0.0, 1.0)),
        || Slider("Green", green_component)
            .color(Color::Rgba(0.0, 220.0, 0.0, 1.0)),
        || P(|| format!("{} {}", red_component.get(), green_component.get())),
        || P(|| format!("Render count: {}", render_count.get())),
        || Button("Set/Reset")
            .on_click(|_| {
                // This resets more times than it should
                for _ in 0..1000 {
                    red_component.put(1.0);
                    red_component.put(0.0);
                }
            }),
        || Button("Show/Hide")
            .on_click(|_| is_showing_gradient.set(|g| !g)),
        || Div(
            || if is_showing_gradient.get() {
                Triangle(red_component, green_component, render_count)
                    .height(200.px())
                    .flex_grow(1.0)
                    .min_width(0.px())
            } else {
                Div(())
            }
        )
            .display(Display::Flex)
    ))
}

fn main() {
    Politana::launch(App);
}
