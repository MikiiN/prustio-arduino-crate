use super::*;

pub fn map(value: i32, from_low: i32, from_high: i32, to_low: i32, to_high: i32) -> i32 {
    unsafe {
        ffi::c_map(
            value as c_long, 
            from_low as c_long, 
            from_high as c_long,
            to_low as c_long, 
            to_high as c_long,
        ) as i32
    }
}

pub fn constrain(x: i32, a: i32, b: i32) -> i32 {
    unsafe { ffi::c_constrain(x as c_long, a as c_long, b as c_long) as i32 }
}