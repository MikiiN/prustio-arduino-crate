#![no_std]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

mod ffi {
    include!("bindings.rs");
}

#[repr(u8)]
pub enum PinMode {
    Input = ffi::C_INPUT as u8,
    Output = ffi::C_OUTPUT as u8,
    InputPullup = ffi::C_INPUT_PULLUP as u8,
}

#[repr(u8)]
pub enum PinState {
    Low = ffi::C_LOW as u8,
    High = ffi::C_HIGH as u8,
}

#[repr(u8)]
pub enum InterruptMode {
    Change = ffi::C_CHANGE as u8,
    Falling = ffi::C_FALLING as u8,
    Rising = ffi::C_RISING as u8,
}

pub struct Pin(pub u8);

impl Pin {
    pub const fn new(pin: u8) -> Self {
        Self(pin)
    }

    pub fn pin_mode(&self, mode: PinMode) {
        unsafe { ffi::c_pin_mode(self.0, mode as u8) }
    }

    pub fn digital_write(&self, state: PinState) {
        unsafe { ffi::c_digital_write(self.0, state as u8) }
    }

    pub fn digital_read(&self) -> PinState {
        let val = unsafe { ffi::c_digital_read(self.0) };
        if val == ffi::C_HIGH as i32 { PinState::High } else { PinState::Low }
    }
}

pub mod time {
    use super::ffi;
    
    pub fn millis() -> u32 { unsafe { ffi::c_millis() as u32 } }
    pub fn delay(ms: u32) { unsafe { ffi::c_delay(ms as core::ffi::c_ulong) } }
}