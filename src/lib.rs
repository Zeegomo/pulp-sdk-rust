#![no_std]
#![feature(allocator_api)]
#![feature(alloc_error_handler)]
#![feature(nonnull_slice_from_raw_parts)]

use core::sync::atomic::*;

mod bindings;
mod alloc;

pub use alloc::*;
pub use bindings::*;

pub fn pi_cl_dma_cmd(ext: u32, loc: u32, size: u32, dir: PiClDmaDirE, cmd: &mut PiClDmaCmd) {
    // Prevent the compiler from reordering a volatile read / write
    fence(Ordering::Release);
    unsafe { pi_cl_dma_cmd_wrap(ext, loc, size, dir, cmd as *mut PiClDmaCmd) }
}

pub fn pi_cl_dma_wait(copy: &mut PiClDmaCmd) {
    // Prevent the compiler from reordering a volatile read / write
    fence(Ordering::Acquire);
    unsafe { pi_cl_dma_wait_wrap(copy as *mut PiClDmaCmd as *mut cty::c_void) }
}

// TODO: compiler fence?
pub fn pi_cl_team_barrier() {
    unsafe { pi_cl_team_barrier_wrap () }
}


// TODO: compiler fence?
// TODO: rewrite this as a safe function
pub unsafe fn pi_cl_team_fork(
    num_cores: usize,
    cluster_fn: extern "C" fn(*mut cty::c_void),
    args: *mut cty::c_void,
) {
    pi_cl_team_fork_wrap(num_cores, cluster_fn, args);
}