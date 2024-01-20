use std::ffi::{c_char, c_int, CStr};

// debug util
#[no_mangle]
pub unsafe extern "C" fn help_debug_message(
    file: *const c_char,
    line: c_int,
    message: *const c_char,
) {
    #[cfg(debug_assertions)]
    {
        let file = CStr::from_ptr(file).to_string_lossy();
        let message = CStr::from_ptr(message).to_string_lossy();
        println!("{}#{} - {}", file, line, message);
    }
}
