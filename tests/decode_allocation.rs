use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

use oxirush_s1ap::s1ap::*;

/// Records the largest allocation this thread requests.
struct LargestAllocation;

thread_local! {
    static LARGEST: Cell<usize> = const { Cell::new(0) };
}

unsafe impl GlobalAlloc for LargestAllocation {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        LARGEST.with(|largest| largest.set(largest.get().max(layout.size())));
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
}

#[global_allocator]
static ALLOCATOR: LargestAllocation = LargestAllocation;

/// A length determinant alone, here a count of 65535 in two octets, does
/// not make the decoder reserve room for 65535 list elements.
#[test]
fn list_count_does_not_reserve_the_whole_list() {
    LARGEST.with(|largest| largest.set(0));
    assert!(rasn::aper::decode::<CellIDCancelled>(&[0xff, 0xfe]).is_err());
    let largest = LARGEST.with(Cell::get);
    assert!(largest < 64 * 1024, "decoding reserved {largest} bytes");
}
