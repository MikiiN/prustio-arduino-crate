use super::*;

pub fn begin(baud_rate: u32) { 
    unsafe { ffi::c_serial_begin(baud_rate as c_ulong) } 
}

pub fn end() { 
    unsafe { ffi::c_serial_end() } 
}

pub fn available() -> i32 { 
    unsafe { ffi::c_serial_available() as i32 } 

}

pub fn read() -> i32 { 
    unsafe { ffi::c_serial_read() as i32 } 
}

pub fn peek() -> i32 { 
    unsafe { ffi::c_serial_peek() as i32 } 
}

pub fn flush() { 
    unsafe { ffi::c_serial_flush() } 
}

pub fn write(val: u8) -> usize { 
    unsafe { ffi::c_serial_write(val) as usize } 
}

pub fn write_buffer(data: &[u8]) -> usize {
    unsafe { ffi::c_serial_write_buffer(data.as_ptr(), data.len() as usize) as usize }
}

pub fn print_cstr(s: &CStr) -> usize {
    unsafe { ffi::c_serial_print_str(s.as_ptr()) as usize }
}

pub fn println_cstr(s: &CStr) -> usize {
    unsafe { ffi::c_serial_println_str(s.as_ptr()) as usize }
}

pub fn print_int(val: i32) -> usize { 
    unsafe { ffi::c_serial_print_int(val as c_int) as usize } 
}

pub fn println_int(val: i32) -> usize { 
    unsafe { ffi::c_serial_println_int(val as c_int) as usize } 
}