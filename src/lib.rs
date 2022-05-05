#![no_std]

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use cty::*;
// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub const PI_BSP_PROFILE_DEFAULT: u32 = 0;
extern "C" {
    pub fn bsp_init();
}
extern "C" {
   pub fn pi_l2_malloc(size: cty::c_int) -> *mut cty::c_void;
}


use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn cluster_fn(a: *const c_int, b: *const c_int, c: *mut c_int, size: size_t, core: size_t, n_cores: size_t) {
    let len = size / n_cores;
    let a = core::slice::from_raw_parts(a.add(len * core), size);
    let b = core::slice::from_raw_parts(b.add(len * core), size);
    let c = core::slice::from_raw_parts_mut(c.add(len * core as usize), len);
    for ((c, a), b) in c.iter_mut().zip(a).zip(b) {
        *c = a + b;
    }
}

#[no_mangle]
pub unsafe extern "C" fn write_42() -> *mut cty::c_int {
    let size: cty::c_int = 4;
    let ptr = pi_l2_malloc(size) as *mut cty::c_int;
    *ptr = 42;
    ptr
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
