use super::*;

pub struct Serial {}

impl Serial {
    pub fn begin(baud_rate: u32) -> Serial {
        unsafe { ffi::c_serial_begin(baud_rate as c_ulong) }
        return Serial {  }    
    }

    pub fn end(&self) {
        unsafe { ffi::c_serial_end() } 
    }

    pub fn available(&self) -> i32 {
        unsafe { ffi::c_serial_available() as i32 }    
    }

    pub fn read(&self) -> i32 {
        unsafe { ffi::c_serial_read() as i32 }    
    }

    pub fn peek(&self) -> i32 {
        unsafe { ffi::c_serial_peek() as i32 }    
    }

    pub fn flush(&self) { 
        unsafe { ffi::c_serial_flush() } 
    }

    pub fn write(&self, val: u8) -> usize { 
        unsafe { ffi::c_serial_write(val) as usize } 
    }

    pub fn write_buffer(&self, data: &[u8]) -> usize {
        unsafe { ffi::c_serial_write_buffer(data.as_ptr(), data.len() as usize) as usize }
    }

    pub fn print(&self, s: &CStr) -> usize {
        unsafe { ffi::c_serial_print(s.as_ptr()) as usize }
    }

    pub fn println(&self, s: &CStr) -> usize {
        unsafe { ffi::c_serial_println(s.as_ptr()) as usize }
    }
}