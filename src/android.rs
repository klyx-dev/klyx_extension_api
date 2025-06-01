unsafe extern "C" {
    fn show_toast_impl(ptr: *const u8, len: usize);
}

pub fn show_toast(msg: &str) {
    unsafe {
        show_toast_impl(msg.as_ptr(), msg.len());
    }
}
