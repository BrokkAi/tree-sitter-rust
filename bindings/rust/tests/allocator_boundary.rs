use brokk_tree_sitter_rust::LANGUAGE;
use std::ffi::c_void;
use std::sync::atomic::{AtomicUsize, Ordering};

static ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);
static FREES: AtomicUsize = AtomicUsize::new(0);

unsafe extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn calloc(count: usize, size: usize) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn brokk_tree_sitter_rust_external_scanner_create() -> *mut c_void;
    fn brokk_tree_sitter_rust_external_scanner_destroy(payload: *mut c_void);
}

unsafe extern "C" fn observed_malloc(size: usize) -> *mut c_void {
    ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
    unsafe { malloc(size) }
}

unsafe extern "C" fn observed_calloc(count: usize, size: usize) -> *mut c_void {
    ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
    unsafe { calloc(count, size) }
}

unsafe extern "C" fn observed_realloc(ptr: *mut c_void, size: usize) -> *mut c_void {
    ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
    unsafe { realloc(ptr, size) }
}

unsafe extern "C" fn observed_free(ptr: *mut c_void) {
    FREES.fetch_add(1, Ordering::Relaxed);
    unsafe { free(ptr) }
}

#[test]
fn external_scanner_uses_tree_sitter_allocator() {
    // This test owns its process: no Tree-sitter objects may exist before installing
    // the allocator, and no other test may allocate Tree-sitter objects in this process.
    unsafe {
        tree_sitter::set_allocator(
            Some(observed_malloc),
            Some(observed_calloc),
            Some(observed_realloc),
            Some(observed_free),
        );
    }

    // Keep the grammar crate's static archive linked in this isolated test executable.
    std::hint::black_box(LANGUAGE);

    let payload = unsafe { brokk_tree_sitter_rust_external_scanner_create() };
    assert!(!payload.is_null());
    unsafe { brokk_tree_sitter_rust_external_scanner_destroy(payload) };

    let allocations = ALLOCATIONS.load(Ordering::Relaxed);
    let frees = FREES.load(Ordering::Relaxed);
    assert!(
        allocations > 0,
        "scanner allocation bypassed Tree-sitter allocator"
    );
    assert!(
        frees > 0,
        "scanner destruction bypassed Tree-sitter allocator"
    );
}
