//! Host application example: safe public APIs, caller-selected fixed storage.
//! This is not a bare-metal startup image or native-runtime acceptance test.
#![forbid(unsafe_code)]

use up_alloc_riscv_rust::{claim_startup, StartupRegion};
use up_rust_alloc::{try_box_in, try_vec_in};

static REGION: StartupRegion<4096> = StartupRegion::new();

fn main() {
    let allocator = claim_startup(&REGION).expect("one startup claim");
    let mut values = try_vec_in::<u32, _>(&allocator, 8).expect("configured capacity");
    let one = try_box_in(42_u32, &allocator).expect("configured box extent");
    let _reserved_bytes = allocator.seal().expect("one allocator seal");
    // No growth occurs: push stays within capacity reserved during setup.
    values.push(*one);
    assert_eq!(values.as_slice(), &[42]);
    // This explicit fallible growth request is refused after sealing.
    assert!(values.try_reserve_exact(values.capacity()).is_err());
}
