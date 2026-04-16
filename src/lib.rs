#![no_std]

use core::ffi::{c_int, c_long, c_uint, c_ulong};
use ufmt::uWrite;
use core::convert::Infallible;

mod ffi {
    #![allow(non_upper_case_globals)]
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!("bindings.rs");
}

pub mod interrupts;
pub mod math;
pub mod random;
pub mod serial;
pub mod spi;
pub mod wire;

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PinMode {
    Input = ffi::C_INPUT as u8,
    Output = ffi::C_OUTPUT as u8,
    InputPullup = ffi::C_INPUT_PULLUP as u8,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum PinState {
    Low = ffi::C_LOW as u8,
    High = ffi::C_HIGH as u8,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum InterruptMode {
    Change = ffi::C_CHANGE as u8,
    Falling = ffi::C_FALLING as u8,
    Rising = ffi::C_RISING as u8,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BitOrder {
    LsbFirst = ffi::C_LSBFIRST as u8,
    MsbFirst = ffi::C_MSBFIRST as u8,
}


// --- Core I/O ---

pub fn init() {
    unsafe { ffi::c_init() }
}

pub fn analog_reference(mode: u8) {
    unsafe { ffi::c_analog_reference(mode) }
}

#[derive(Copy, Clone, Debug)]
pub struct Pin(pub u8);

impl Pin {
    pub const fn new(pin: u8) -> Self { Self(pin) }
}

pub fn pin_mode(pin: &Pin, mode: PinMode) {
    unsafe { ffi::c_pin_mode(pin.0, mode as u8) }
}

pub fn digital_write(pin: &Pin, state: PinState) {
    unsafe { ffi::c_digital_write(pin.0, state as u8) }
}

pub fn digital_read(pin: &Pin) -> PinState {
    let val = unsafe { ffi::c_digital_read(pin.0) };
    if val == ffi::C_HIGH as c_int { PinState::High } else { PinState::Low }
}

pub fn analog_write(pin: &Pin, val: i32) {
    unsafe { ffi::c_analog_write(pin.0, val as c_int) }
}

pub fn analog_read(pin: &Pin) -> i32 {
    unsafe { ffi::c_analog_read(pin.0) as i32 }
}

pub fn tone(pin: &Pin, frequency: u32, duration_ms: Option<u32>) {
    match duration_ms {
        Some(dur) => unsafe { ffi::c_tone(pin.0, frequency as c_uint, dur as c_ulong) },
        None => unsafe { ffi::c_tone(pin.0, frequency as c_uint, 0 as c_ulong) }
    } 
}

pub fn no_tone(pin: &Pin) {
    unsafe { ffi::c_no_tone(pin.0) }
}


// --- Time ---

pub fn millis() -> u32 { 
    unsafe { ffi::c_millis() as u32 } 
}

pub fn micros() -> u32 { 
    unsafe { ffi::c_micros() as u32 } 
}

pub fn delay(ms: u32) { 
    unsafe { ffi::c_delay(ms as c_ulong) } 
}

pub fn delay_microseconds(us: u32) { 
    unsafe { ffi::c_delay_microseconds(us as c_uint) } 
}


// --- Advanced I/O ---

pub fn shift_out(data_pin: &Pin, clock_pin: &Pin, bit_order: BitOrder, val: u8) {
    unsafe { ffi::c_shift_out(data_pin.0, clock_pin.0, bit_order as u8, val) }
}

pub fn shift_in(data_pin: &Pin, clock_pin: &Pin, bit_order: BitOrder) -> u8 {
    unsafe { ffi::c_shift_in(data_pin.0, clock_pin.0, bit_order as u8) }
}

pub fn pulse_in(pin: &Pin, state: PinState, timeout_us: u32) -> u32 {
    unsafe { ffi::c_pulse_in(pin.0, state as u8, timeout_us as c_ulong) as u32 }
}

// --- Utils ---

// struct that holds str buffer and tracks how much we've written
pub struct StackString {
    buf: [u8; 255],
    len: usize,
}

impl StackString {
    pub fn new() -> Self {
        Self { buf: [0; 255], len: 0 }
    }
    
    pub fn as_str(&self) -> &str {
        unsafe { core::str::from_utf8_unchecked(&self.buf[..self.len]) }
    }
}

// Teach ufmt how to write into our buffer
impl uWrite for StackString {
    type Error = Infallible;

    fn write_str(&mut self, s: &str) -> Result<(), Self::Error> {
        let bytes = s.as_bytes();
        let available = self.buf.len() - self.len;
        let to_copy = bytes.len().min(available);
        
        self.buf[self.len .. self.len + to_copy].copy_from_slice(&bytes[..to_copy]);
        self.len += to_copy;
        
        Ok(())
    }
}

#[macro_export]
macro_rules! format_str {
    ($($arg:tt)*) => {{
        let mut s = StackString::new();
        let _ = ufmt::uwrite!(&mut s, $($arg)*); 
        s 
    }};
}