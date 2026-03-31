use super::*;

pub fn attach(interrupt_num: u8, mode: InterruptMode, callback: extern "C" fn()) {
    unsafe {
        let ffi_cb: unsafe extern "C" fn() = core::mem::transmute(callback);
        ffi::c_attach_interrupt(interrupt_num, Some(ffi_cb), mode as c_int);
    }
}

pub fn detach(interrupt_num: u8) {
    unsafe { ffi::c_detach_interrupt(interrupt_num) }
}

pub fn enable() { 
    unsafe { ffi::c_interrupts() } 
}

pub fn disable() { 
    unsafe { ffi::c_no_interrupts() } 
}