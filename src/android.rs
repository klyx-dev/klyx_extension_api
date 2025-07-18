#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastDuration {
    Short = 0,
    Long = 1,
}

#[link(wasm_import_module = "Android")]
unsafe extern "C" {
    fn show_toast_impl(ptr: *const u8, len: usize, duration: u8);
}

pub fn show_toast(msg: &str, duration: Option<ToastDuration>) {
    let duration = duration.unwrap_or(ToastDuration::Short);

    unsafe {
        show_toast_impl(msg.as_ptr(), msg.len(), duration as u8);
    }
}
