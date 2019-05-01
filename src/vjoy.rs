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
            let r1 = SetAxis((*channels.get(0).unwrap() as i32), VJOY_ID, HID_USAGE_X) == 1;
            let r2 = SetAxis((*channels.get(1).unwrap() as i32), VJOY_ID, HID_USAGE_Y) == 1;
            let r3 = SetAxis((*channels.get(2).unwrap() as i32), VJOY_ID, HID_USAGE_Z) == 1;
            let r4 = SetAxis((*channels.get(3).unwrap() as i32), VJOY_ID, HID_USAGE_RX) == 1;
            let r5 = SetAxis((*channels.get(4).unwrap() as i32), VJOY_ID, HID_USAGE_RY) == 1;
            let r6 = SetAxis((*channels.get(5).unwrap() as i32), VJOY_ID, HID_USAGE_RZ) == 1;
            return r1 && r2 && r3 && r4 && r5 && r6;
        }
        false
    }
}
