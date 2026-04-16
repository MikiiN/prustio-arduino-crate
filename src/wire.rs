use super::*;

pub struct Wire {}

impl Wire {
    pub fn begin(address: Option<u8>) -> Wire {
        match address {
            Some(addr) => unsafe {
                ffi::c_wire_begin_slave(addr);
            },
            None => unsafe {
                ffi::c_wire_begin();
            }
        }
        Wire {}
    }

    pub fn request_from(&self, address: u8, quantity: u8, stop: bool) -> u8 {
        unsafe { ffi::c_wire_request_from(address, quantity, stop) }
    }

    pub fn begin_transmission(&self, address: u8) { 
        unsafe { ffi::c_wire_begin_transmission(address) } 
    }

    pub fn end_transmission(&self, stop: bool) -> u8 { 
        unsafe { ffi::c_wire_end_transmission(stop) } 
    }

    pub fn write(&self, value: u8) -> usize { 
        unsafe { ffi::c_wire_write(value) as usize } 
    }
    
    pub fn write_buffer(&self, data: &[u8]) -> usize {
        unsafe { ffi::c_wire_write_buffer(data.as_ptr(), data.len() as usize) as usize }
    }

    pub fn available(&self) -> i32 { 
        unsafe { ffi::c_wire_available() as i32 } 
    }

    pub fn read(&self) -> i32 { 
        unsafe { ffi::c_wire_read() as i32 } 
    }

    pub fn on_receive(&self, callback: extern "C" fn(c_int)) {
        unsafe {
            let ffi_cb: unsafe extern "C" fn(c_int) = core::mem::transmute(callback);
            ffi::c_wire_on_receive(Some(ffi_cb));
        }
    }

    pub fn on_request(&self, callback: extern "C" fn()) {
        unsafe {
            let ffi_cb: unsafe extern "C" fn() = core::mem::transmute(callback);
            ffi::c_wire_on_request(Some(ffi_cb));
        }
    }
}
