use super::*;

pub fn seed(seed: u32) { 
    unsafe { ffi::c_random_seed(seed as c_ulong) } 
}

pub fn max(max: i32) -> i32 { 
    unsafe { ffi::c_random_max(max as c_long) as i32 } 
}

pub fn range(min: i32, max: i32) -> i32 { 
    unsafe { ffi::c_random_range(min as c_long, max as c_long) as i32 } 
}