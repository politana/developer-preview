use wasm_bindgen::prelude::*;
use js_sys::Function;

pub fn js_try(f: impl FnOnce()) -> Option<String> {
    let mut f_opt = Some(f);
    let closure = Closure::wrap(Box::new(move || {
        if let Some(inner_f) = f_opt.take() {
            inner_f();
        }
    }) as Box<dyn FnMut()>);
    let js_func: &Function = closure.as_ref().unchecked_ref();
    let try_catch_wrapper = Function::new_with_args("f", "
        try {
            f();
            return null;
        } catch (e) {
            return String(e);
        }
    ");
    let result = try_catch_wrapper.call1(&JsValue::NULL, js_func);
    match result {
        Ok(val) if val.is_null() => None,
        Ok(val) => Some(val.as_string().unwrap_or_else(|| "Unknown JS Error".to_string())),
        Err(e) => Some(format!("Runner Error: {:?}", e)),
    }
}
