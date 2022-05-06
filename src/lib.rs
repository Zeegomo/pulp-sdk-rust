#![no_std]

mod bindings;
pub use bindings::*;

pub fn pi_cl_dma_cmd(ext: u32, loc: u32, size: u32, dir: PiClDmaDirE, cmd: &mut pi_cl_dma_cmd_t) {
    unsafe { pi_cl_dma_cmd_wrap(ext, loc, size, dir, cmd as *mut pi_cl_dma_cmd_t) }
}

pub fn pi_cl_dma_wait(copy: &mut pi_cl_dma_cmd_t) {
    unsafe { pi_cl_dma_wait_wrap(copy as *mut pi_cl_dma_cmd_t as *mut cty::c_void) }
}

// TODO: rewrite this as a safe function
pub unsafe fn pi_cl_team_fork(
    num_cores: usize,
    cluster_fn: extern "C" fn(*mut cty::c_void),
    args: *mut cty::c_void,
) {
    pi_cl_team_fork_wrap(num_cores, cluster_fn, args);
}

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
