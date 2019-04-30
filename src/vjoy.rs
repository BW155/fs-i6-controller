use crate::vjoy_bindgen::*;
use std::ffi::CStr;

const VJOY_ID: u32 = 1;

#[derive(Debug, Clone)]
pub struct VJoy {
   enabled: bool, 
}

impl VJoy {
    pub fn new() -> Self {
        let mut enabled = false;
        unsafe {
            enabled = vJoyEnabled() == 1;
            println!("VJoy acquired: {}", AcquireVJD(VJOY_ID) == 1);
            println!("exists axis: {}", GetVJDAxisExist(VJOY_ID, HID_USAGE_X) == 1);
            ResetVJD(VJOY_ID); 
        }
        VJoy {
            enabled,
        }
    }

    pub fn set(&self, channels: [i64; 6]) -> bool {
        unsafe {
            let result = SetAxis((*channels.get(0).unwrap() as i32) * 327, VJOY_ID, HID_USAGE_X);
            println!("setAxis: {}", result);
        }
        true
    }
}
