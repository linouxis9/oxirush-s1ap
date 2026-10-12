//! Decoding reserves memory for the elements it reads, not for the count a
//! length determinant claims.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

use oracle::bindings::CellIDCancelledItem;
use oracle::rasn::{self, prelude::*};
use oxirush_s1ap::aper::decode;
use oxirush_s1ap::s1ap::CellIDCancelled;

struct PeakAllocator;

static CURRENT: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for PeakAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let current = CURRENT.fetch_add(layout.size(), Ordering::SeqCst) + layout.size();
        PEAK.fetch_max(current, Ordering::SeqCst);
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, pointer: *mut u8, layout: Layout) {
        CURRENT.fetch_sub(layout.size(), Ordering::SeqCst);
        unsafe { System.dealloc(pointer, layout) }
    }
}

#[global_allocator]
static ALLOCATOR: PeakAllocator = PeakAllocator;

/// `CellID-Cancelled` of the oracle, with the list codec that rasn derives for it in
/// place of the one that the oracle has.
#[derive(AsnType, Decode)]
#[rasn(delegate, size("1..=65535"))]
struct Derived(#[allow(dead_code)] SequenceOf<CellIDCancelledItem>);

/// The most memory that is reserved at once while `read` reads octets that are no
/// value, and refuses them.
fn reserved(read: impl FnOnce() -> bool) -> usize {
    let before = CURRENT.load(Ordering::SeqCst);
    PEAK.store(before, Ordering::SeqCst);
    assert!(!read());
    PEAK.load(Ordering::SeqCst) - before
}

// One test: the counters are those of the process.
#[test]
fn a_claimed_list_length_reserves_no_memory() {
    // After a count, 8192 octets that are no element: the first does not decode.
    let after = |count: &[u8]| [count, &[0xff; 8192]].concat();

    // CellID-Cancelled is SIZE(1..65535): a count of 65535, then no element, then
    // octets that are none.
    for octets in [vec![0xff, 0xfe], after(&[0xff, 0xfe])] {
        let cells = reserved(|| decode::<CellIDCancelled>(&octets).is_ok());
        assert!(cells < 64 * 1024, "reserved {cells} bytes");
    }
    // The list codec of rasn reserves for the count that it reads, as far as the
    // octets after it have bits: the reader of the crate reserves a little, and grows
    // the list with the elements that it reads.
    let octets = after(&[0xff, 0xfe]);
    let derived = reserved(|| rasn::aper::decode::<Derived>(&octets).is_ok());
    assert!(
        derived > 1024 * 1024,
        "the codec of rasn reserved {derived} bytes"
    );
}
