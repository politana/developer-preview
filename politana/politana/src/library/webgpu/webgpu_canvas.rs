#[cfg(not(target_arch = "wasm32"))]
pub use stub::*;

#[cfg(target_arch = "wasm32")]
pub use web::*;

#[cfg(not(target_arch = "wasm32"))]
mod stub {
    use crate::{El, library::{GpuContext, Renderer}};

    pub fn WebGPUCanvas<
        AppData: 'static,
        Setup: Fn(GpuContext) -> AppData + 'static,
        RenderFrame: Fn(GpuContext, &mut AppData) + 'static,
        Teardown: Fn(GpuContext, &mut AppData) + 'static
    >(
        renderer: Renderer<AppData, Setup, RenderFrame, Teardown>
    ) -> El {
        let _ = renderer;
        El("canvas", ())
    }
}

#[cfg(target_arch = "wasm32")]
mod web {

use std::{any::Any, cell::RefCell, rc::Rc};

use wasm_bindgen::{JsCast, prelude::Closure};
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlCanvasElement, ResizeObserver, ResizeObserverEntry};
use wgpu::{Backends, DeviceDescriptor, Instance, InstanceDescriptor, Limits, PowerPreference, PresentMode, RequestAdapterOptions, SurfaceConfiguration, SurfaceTarget, TextureUsages};

use crate::{El, Environment, State, library::{RenderMode, webgpu::{gpu_context::GpuContext, renderer::Renderer}}, reactivity::inert::inert, utils::{error_messages, unwrap_or_error::UnwrapOrError}};

const NO_ANIMATION_FRAME: &str = "WebGPUCanvas: Cannot request animation frame";

// TODO: Profile effectful frame rate
pub fn WebGPUCanvas<
    AppData: 'static,
    Setup: Fn(GpuContext) -> AppData + 'static,
    RenderFrame: Fn(GpuContext, &mut AppData) + 'static,
    Teardown: Fn(GpuContext, &mut AppData) + 'static
>(
    renderer: Renderer<AppData, Setup, RenderFrame, Teardown>
) -> El {
    // User data + config
    let gpu_context: State<Option<State<GpuContext>>> = State::default();
    let app_data: State<Option<State<AppData>>> = State::default();
    let render_frame = Rc::new(renderer.render_frame);
    let render_frame_clone = render_frame.clone();
    let render_frame_clone_2 = render_frame.clone();
    let power_preference = renderer.options.power_preference;
    let render_mode = renderer.render_mode;
    // Resources
    let resize_observer: State<Option<ResizeObserver>> = State::default();
    let retained_closures: State<Vec<Box<dyn Any>>> = State::default();
    let animation_frame_id: State<Option<i32>> = State::default();
    // Internal state
    let did_disappear: Rc<RefCell<bool>> = Rc::default();
    let did_disappear_clone = did_disappear.clone();
    let is_canvas_zero_sized = State::new(false);
    let resize_render_trigger: State<u64> = State::default();
    // Internal state (effectful)
    let waiting_for_next_frame: Rc<RefCell<bool>> = Rc::default();
    let effectful_next_frame_closure: Rc<RefCell<Option<Closure<dyn Fn()>>>> = Rc::default();
    let did_miss_effect: Rc<RefCell<bool>> = Rc::default();
    let force_effectful_frame: State<u64> = State::default();

    let mut el = El("canvas", ())
        .on_appear(move |canvas| {
            // UNEXPECTED: The element is explicitly created as a canvas.
            let canvas = canvas.dyn_into::<HtmlCanvasElement>().ok().unwrap_or_unexpected();
            spawn_local(async move {
                if *did_disappear.borrow() { return }
                let _ = init_wgpu(
                    gpu_context,
                    power_preference,
                    canvas.clone(),
                    did_disappear.clone(),
                    is_canvas_zero_sized,
                    resize_render_trigger
                ).await;
                if *did_disappear.borrow() { return }
                let Some(gpu_context) = gpu_context.get_once() else { return };
                register_surface_config_callback(
                    gpu_context,
                    resize_observer,
                    retained_closures,
                    canvas,
                    is_canvas_zero_sized,
                    resize_render_trigger
                );
                app_data.set(|_| Some(State::new((renderer.setup)(gpu_context.get_once()))));
                if render_mode == RenderMode::EveryFrame {
                    let closure = start_animation_loop(
                        animation_frame_id,
                        move || {
                            if !is_canvas_zero_sized.get_once() {
                                // UNEXPECTED: The variable is set to a Some value earlier in the same function.
                                app_data.get_once().unwrap_or_unexpected().update(|d|
                                    render_frame(gpu_context.get_once(), d)
                                );
                            }
                        }
                    );
                    retained_closures.update(|r| r.push(Box::new(closure)));
                }
            });
        })
        .on_disappear(move |_| {
            did_disappear_clone.replace(true);
            if let Some(id) = animation_frame_id.get_once() {
                let _ = Environment::window().cancel_animation_frame(id);
            }
            resize_observer.update(|r| {
                if let Some(observer) = r {
                    observer.disconnect();
                    *r = None;
                }
            });
            if let Some(gpu_context) = gpu_context.get_once() {
                if let Some(app_data) = app_data.get_once() {
                    app_data.update(|d|
                        (renderer.teardown)(gpu_context.get_once(), d)
                    );
                }
            }
            // Let the closures free when the canvas is removed from the DOM
        })
        // internal API
        .debounced_effect(move || {
            if render_mode != RenderMode::OnStateChanged {
                // do not rerun this effect
                return true;
            }
            // allow one extra frame to be forced
            force_effectful_frame.get();
            if *waiting_for_next_frame.borrow() {
                // keep waiting on future state changes
                *did_miss_effect.borrow_mut() = true;
                return false;
            }
            // depend on the GPU context so we rerun the first time it is set
            let Some(gpu_context) = gpu_context.get() else { return true };
            *waiting_for_next_frame.borrow_mut() = true;
            let render_frame = render_frame_clone.clone();
            if !is_canvas_zero_sized.get_once() {
                // also depend on app_data for the same reason
                // neither app_data nor gpu_context will be statefully updated more than once
                if let Some(app_data) = app_data.get() {
                    app_data.update(|d| render_frame(gpu_context.get_once(), d));
                }
            }
            if effectful_next_frame_closure.borrow().is_none() {
                let next_frame_clone = waiting_for_next_frame.clone();
                let did_miss_effect_clone = did_miss_effect.clone();
                *effectful_next_frame_closure.borrow_mut() = Some(
                    Closure::new(move || {
                        *next_frame_clone.borrow_mut() = false;
                        if *did_miss_effect_clone.borrow() {
                            force_effectful_frame.set(|c| c + 1);
                            *did_miss_effect_clone.borrow_mut() = false;
                        }
                    })
                );
            }
            Environment::window().request_animation_frame(
                // UNEXPECTED: If the variable is None, it is initialized to Some earlier in the function.
                effectful_next_frame_closure.borrow().as_ref().unwrap_or_unexpected()
                    .as_ref().unchecked_ref()
            ).ok().warn_if_none(&error_messages::browser_error(NO_ANIMATION_FRAME));
            true
        })
        // re-render on size change
        .debounced_effect(move || {
            resize_render_trigger.get();
            let Some(gpu_context) = gpu_context.get_once() else { return true };
            let render_frame = render_frame_clone_2.clone();
            if !is_canvas_zero_sized.get_once() {
                if let Some(app_data) = app_data.get_once() {
                    app_data.update(|d|
                        inert(|| {
                            render_frame(gpu_context.get_once(), d);
                        })
                    );
                }
            }
            true
        });

    for effect in renderer.options.effects {
        el = el.effect(move || {
            if let Some(gpu_context) = gpu_context.get() {
                if let Some(app_data) = app_data.get() {
                    app_data.update(|d|
                        effect(gpu_context.get_once(), d)
                    );
                }
            }
        });
    }

    el
}

async fn init_wgpu(
    gpu_context: State<Option<State<GpuContext>>>,
    power_preference: PowerPreference,
    canvas: HtmlCanvasElement,
    did_disappear: Rc<RefCell<bool>>,
    is_canvas_zero_sized: State<bool>,
    resize_render_trigger: State<u64>
) {
    let instance = Instance::new(&InstanceDescriptor {
        backends: Backends::BROWSER_WEBGPU,
        ..Default::default()
    });
    let Ok(surface) = instance.create_surface(SurfaceTarget::Canvas(canvas.clone()))
        else { return };
    let Ok(adapter) = instance.request_adapter(&RequestAdapterOptions {
        power_preference,
        compatible_surface: Some(&surface),
        ..Default::default()
    }).await else { return };
    if *did_disappear.borrow() { return }
    let Ok((device, queue)) = adapter
        .request_device(&DeviceDescriptor {
            required_limits: Limits::downlevel_defaults().using_resolution(adapter.limits()),
            ..Default::default()
        }).await else { return };
    if *did_disappear.borrow() { return }
    let capabilities = surface.get_capabilities(&adapter);
    let surface_config = SurfaceConfiguration {
        usage: TextureUsages::RENDER_ATTACHMENT,
        format: capabilities.formats[0],
        // The size will be reset during the call to `canvas_resized` later in the function
        width: 0,
        height: 0,
        present_mode: PresentMode::AutoVsync,
        desired_maximum_frame_latency: 2,
        alpha_mode: capabilities.alpha_modes[0],
        view_formats: Vec::new()
    };

    gpu_context.set(|_| Some(State::new(
        GpuContext {
            device, queue, surface_config,
            surface: Rc::new(surface)
        }
    )));

    canvas_resized(
        canvas,
        // UNEXPECTED: The variable is set to Some earlier in the function.
        gpu_context.get_once().unwrap_or_unexpected(),
        is_canvas_zero_sized,
        resize_render_trigger
    );
}

fn register_surface_config_callback(
    gpu_context: State<GpuContext>,
    resize_observer_state: State<Option<ResizeObserver>>,
    closures: State<Vec<Box<dyn Any>>>,
    canvas: HtmlCanvasElement,
    is_canvas_zero_sized: State<bool>,
    resize_render_trigger: State<u64>
) {
    let canvas_clone = canvas.clone();
    let closure = Closure::<dyn Fn(Vec<ResizeObserverEntry>)>::new(
        move |_| {
            canvas_resized(
                canvas_clone.clone(),
                gpu_context,
                is_canvas_zero_sized,
                resize_render_trigger
            );
        }
    );
    let resize_observer = ResizeObserver::new(closure.as_ref().unchecked_ref())
        .ok().unwrap_or_report(&error_messages::browser_error("WebGPUCanvas: ResizeObserver not supported"));
    resize_observer.observe(&canvas);
    // Make sure the closure and observer are not deallocated
    resize_observer_state.put(Some(resize_observer));
    closures.update(|c| c.push(Box::new(closure)));
}

fn canvas_resized(
    canvas: HtmlCanvasElement,
    gpu_context: State<GpuContext>,
    is_canvas_zero_sized: State<bool>,
    resize_render_trigger: State<u64>
) {
    let pixel_density = Environment::window().device_pixel_ratio();
    let width = (canvas.client_width() as f64 * pixel_density) as u32;
    let height = (canvas.client_height() as f64 * pixel_density) as u32;
    is_canvas_zero_sized.put(width == 0 || height == 0);
    // Avoid validation errors in case one zero-sized frame slips through
    let width = width.max(1);
    let height = height.max(1);
    canvas.set_width(width);
    canvas.set_height(height);
    gpu_context.update(|c| {
        c.surface_config.width = width;
        c.surface_config.height = height;
        c.surface.configure(&c.device, &c.surface_config);
    });
    resize_render_trigger.set(|x| x + 1);
}

fn start_animation_loop<F: FnMut() + 'static>(
    animation_frame_id: State<Option<i32>>,
    mut callback: F
) -> Rc<RefCell<Option<Closure<dyn FnMut()>>>>  {
    let request_animation_frame = move |f: &Closure<dyn FnMut()>| {
        let id = Environment::window()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .ok().warn_if_none(&error_messages::browser_error(NO_ANIMATION_FRAME));
        animation_frame_id.put(id);
    };
    let f = Rc::new(RefCell::new(None));
    let w = Rc::downgrade(&f);
    *f.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        if let Some(strong_ref) = w.upgrade() {
            callback();
            // UNEXPECTED: By the time the animation frame callback is called,
            // f will already be set to Some(...). No code would revert it back.
            // It *is* possible for the Rc as a whole to be deallocated, but that
            // is explicitly checked for.
            request_animation_frame(strong_ref.borrow().as_ref().unwrap_or_unexpected());
        }
    }) as Box<dyn FnMut()>));
    // UNEXPECTED: f is set to Some(...) earlier in the function
    request_animation_frame(f.borrow().as_ref().unwrap_or_unexpected());
    f
}

} // mod web
