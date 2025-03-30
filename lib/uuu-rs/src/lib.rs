#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub mod devices;
pub mod notification;

/// Run a command using uuu
pub fn run_command(command: &str) -> Result<(), String> {
    let c_command = std::ffi::CString::new(command).unwrap();
    unsafe {
        let result = uuu_run_cmd(c_command.as_ptr() as *const i8, 0);
        match result {
            0 => Ok(()),
            _ => Err(format!("Command execution failed: {}", get_last_error())),
        }
    }
}

/// Get the last error message
pub fn get_last_error() -> String {
    let mut error_str;
    unsafe {
        let error: *const ::std::os::raw::c_char = uuu_get_last_err_string();
        error_str = std::ffi::CStr::from_ptr(error)
            .to_str()
            .unwrap()
            .to_string();
    }
    error_str
}
