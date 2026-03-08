#[allow(dead_code)]
pub fn console_log(message: &str) {
    web_sys::console::log_1(&message.into());
}
