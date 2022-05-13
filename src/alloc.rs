use crate::{pi_l2_free, pi_l2_malloc};
use core::alloc::{Allocator, AllocError, Layout};
use core::ptr::NonNull;
/// Allocate memory on chip L2 memory
/// Wrapper around `pi_l2_malloc` and `pi_l2_free`
const L2_ALIGN: usize = 4;


pub struct L2Alloc;

unsafe impl Allocator for L2Alloc {
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        if layout.align() > L2_ALIGN {
            // TODO: use pi_l2_malloc_align()
            return Err(AllocError);
        }
        let ptr = unsafe { pi_l2_malloc(layout.size().try_into().map_err(|_| AllocError)?) } as *mut u8;
        NonNull::new(ptr).map(|ptr| NonNull::slice_from_raw_parts(ptr, layout.size())).ok_or(AllocError)
    }
    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        pi_l2_free(ptr.as_ptr() as *mut cty::c_void, layout.size() as i32);
    }
}