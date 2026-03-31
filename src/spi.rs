use super::*;

pub fn begin() { 
    unsafe { ffi::c_spi_begin() } 
}

pub fn end() { 
    unsafe { ffi::c_spi_end() } 
}

pub fn begin_transaction(clock_speed: u32, bit_order: BitOrder, data_mode: u8) {
    unsafe { ffi::c_spi_begin_transaction(clock_speed, bit_order as u8, data_mode) }
}

pub fn end_transaction() { 
    unsafe { ffi::c_spi_end_transaction() } 
}

pub fn transfer(val: u8) -> u8 { 
    unsafe { ffi::c_spi_transfer(val) } 
}

pub fn transfer16(val: u16) -> u16 { 
    unsafe { ffi::c_spi_transfer16(val) } 
}

pub fn transfer_buffer(data: &mut [u8]) {
    unsafe {
        ffi::c_spi_transfer_buffer(
            data.as_mut_ptr() as *mut core::ffi::c_void,
            data.len() as usize,
        )
    }
}