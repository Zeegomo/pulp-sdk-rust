#[repr(C)]
pub struct pi_cl_dma_cmd_t {
    id: cty::c_int,
    next: *mut Self,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum PiClDmaDirE {
    PI_CL_DMA_DIR_LOC2EXT = 0,
    PI_CL_DMA_DIR_EXT2LOC = 1,
}

extern "C" {
    pub fn pi_cl_dma_cmd_wrap(
        ext: cty::uint32_t,
        loc: cty::uint32_t,
        size: cty::uint32_t,
        dir: PiClDmaDirE,
        cmd: *mut pi_cl_dma_cmd_t,
    );

    pub fn pi_cl_dma_wait_wrap(copy: *mut cty::c_void);

    pub fn abort_all();

    pub fn pi_cl_team_fork_wrap(
        num_cores: usize,
        cluster_fn: extern "C" fn(*mut cty::c_void),
        args: *mut cty::c_void,
    );

    pub fn pi_cl_team_barrier_wrap();

    pub fn pi_l2_malloc(size: cty::c_int) -> *mut cty::c_int;

    pub fn pi_cl_l1_malloc(size: cty::c_int) -> *mut cty::c_int;
}

#[inline(always)]
pub unsafe fn pi_core_id() -> usize {
    let core_id: usize;
    core::arch::asm!("csrr {core_id}, 0x014", core_id = out(reg) core_id,);
    core_id & 0x01f
}
