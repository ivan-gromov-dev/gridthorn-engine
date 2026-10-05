//! Test-binary allocator for manual grid heap measurements.

#[global_allocator]
static ALLOCATOR: dhat::Alloc = dhat::Alloc;
