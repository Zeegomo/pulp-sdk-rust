#[repr(C)]
pub struct PiClDmaCmd {
    id: cty::c_int,
    next: *mut Self,
}

impl PiClDmaCmd {
    pub fn new() -> Self {
        Self {
            id: 0,
            next: core::ptr::null_mut(),
        }
    }
}

impl Default for PiClDmaCmd {
    fn default() -> Self {
        Self::new()
    }
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum PiClDmaDirE {
    PI_CL_DMA_DIR_LOC2EXT = 0,
    PI_CL_DMA_DIR_EXT2LOC = 1,
}

#[repr(C)]
pub struct PiDevice {
    api: *mut PiDeviceApi,
    config: *mut cty::c_void,
    data: *mut cty::c_void,
}

#[repr(C)]
pub struct PiDeviceApi {
    _data: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}

extern "C" {
    pub fn pi_cl_dma_cmd_wrap(
        ext: cty::uint32_t,
        loc: cty::uint32_t,
        size: cty::uint32_t,
        dir: PiClDmaDirE,
        cmd: *mut PiClDmaCmd,
    );

    pub fn pi_cl_dma_wait_wrap(copy: *mut cty::c_void);

    pub fn pi_cl_ram_read_wait_wrap(req: *mut PiClRamReq);

    pub fn pi_cl_ram_write_wait_wrap(req: *mut PiClRamReq);

    pub fn pi_cl_ram_read_wrap(
        device: *mut PiDevice,
        pi_ram_addr: u32,
        addr: *mut cty::c_void,
        size: u32,
        req: *mut PiClRamReq,
    );

    pub fn pi_cl_ram_write_wrap(
        device: *mut PiDevice,
        pi_ram_addr: u32,
        addr: *mut cty::c_void,
        size: u32,
        req: *mut PiClRamReq,
    );

    pub fn abort_all();

    pub fn pi_cl_team_fork_wrap(
        num_cores: usize,
        cluster_fn: extern "C" fn(*mut cty::c_void),
        args: *mut cty::c_void,
    );

    pub fn pi_cl_team_barrier_wrap();

    pub fn pi_l2_malloc(size: cty::c_int) -> *mut cty::c_void;

    pub fn pi_l2_free(chunk: *mut cty::c_void, size: cty::c_int);

    pub fn pi_cl_l1_malloc(cluster: *mut PiDevice, size: cty::c_int) -> *mut cty::c_void;

    pub fn pi_cl_l1_free(cluster: *mut PiDevice, chunk: *mut cty::c_void, size: cty::c_int);

}

#[repr(C)]
pub struct PiClRamReq {
    device: *mut PiDevice,
    addr: *mut cty::c_void,
    ram_addr: u32,
    size: u32,
    stride: u32,
    length: u32,
    event: PiTask,
    next: *mut Self,
    done: u8,
    cid: cty::c_char,
    ext2loc: cty::c_char,
    is_2d: cty::c_char,
}

impl PiClRamReq {
    pub fn new(device: *mut PiDevice) -> Self {
        Self {
            device,
            addr: core::ptr::null_mut(),
            ram_addr: 0,
            size: 0,
            stride: 0,
            length: 0,
            event: PiTask::new(),
            next: core::ptr::null_mut(),
            done: 0,
            cid: 0,
            ext2loc: 0,
            is_2d: 0,
        }
    }

    pub fn is_in_transfer(&self) -> bool {
        self.ext2loc != 0
    }
}

const PI_TASK_IMPLEM_NB_DATA: usize = 8;

#[derive(Default)]
#[repr(C)]
#[repr(packed)]
struct PiTaskImplem {
    time: cty::c_uint,
}

#[repr(C)]
pub struct PiTask {
    // Warning, might be accessed inline in asm, and thus can not be moved
    next: *mut Self,
    arg: [usize; 4],
    done: i8,
    id: cty::c_int,
    data: [u32; PI_TASK_IMPLEM_NB_DATA],
    implem: PiTaskImplem,
}

impl PiTask {
    fn new() -> Self {
        Self {
            next: core::ptr::null_mut(),
            arg: [0; 4],
            done: 0,
            id: 0,
            data: [0; PI_TASK_IMPLEM_NB_DATA],
            implem: PiTaskImplem::default(),
        }
    }
}

#[inline(always)]
pub unsafe fn pi_core_id() -> usize {
    let core_id: usize;
    core::arch::asm!("csrr {core_id}, 0x014", core_id = out(reg) core_id,);
    core_id & 0x01f
}
