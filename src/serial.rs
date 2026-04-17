use super::*;

pub trait Printable {
    fn print_to(&self, serial: &Serial);
}

impl Printable for &str {
    fn print_to(&self, serial: &Serial) {
        for byte in self.bytes() {
            serial.write(byte);
        }
    }
}

impl Printable for u8 {
    fn print_to(&self, serial: &Serial) {
        let mut val = *self;
        
        let mut hundreds = 0;
        while val >= 100 { hundreds += 1; val -= 100; }
        
        let mut tens = 0;
        while val >= 10 { tens += 1; val -= 10; }
        
        let ones = val;

        let mut print_started = false;
        
        if hundreds > 0 {
            serial.write(b'0' + hundreds);
            print_started = true;
        }
        
        if print_started || tens > 0 {
            serial.write(b'0' + tens);
        }
        
        serial.write(b'0' + ones);
    }
}

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

    pub fn print<T: Printable>(&self, val: T) {
        val.print_to(self);
    }

    pub fn println<T: Printable>(&self, val: T) {
        val.print_to(self);
        self.print("\n");
    }
}