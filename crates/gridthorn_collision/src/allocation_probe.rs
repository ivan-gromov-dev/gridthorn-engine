//! Test-binary allocator for manual collision heap measurements.

#[global_allocator]
static ALLOCATOR: dhat::Alloc = dhat::Alloc;
