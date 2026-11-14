//! Fixed startup-memory entry point for an experimental RISC-V integration.
//!
//! This thin crate uses the SDK's common fixed-region implementation; it does
//! not maintain another arena or Allocator implementation. All operations use
//! public SDK APIs available to any independent provider author.
//!
//! This source does not implement an RTOS, startup code, interrupt exclusion,
//! clock/identity service or board linker. The supplied profile records name
//! intended checks, not qualified hardware or Ferrocene compatibility.
#![no_std]
#![forbid(unsafe_code)]

pub use up_rust_alloc::fixed::FixedStartupAllocator;
pub use up_rust_alloc::memory::{StartupError, StartupRegion};

/// Claims permanent caller-selected startup storage through the common SDK.
///
/// The returned allocator is local (`!Sync`); this is a property of this
/// optional provider, not a new restriction on SDK transport handles.
/// Allocation failure is returned. Sealing preserves existing allocations;
/// dropping or forgetting the allocator never reclaims its permanent backing.
/// This does not admit native-runtime use of arbitrary collection operations.
///
/// # Errors
/// Returns the common SDK's already-claimed or too-small-region error.
///
/// # Safety-usable invariant
/// Only the common fixed-region claim constructor is used; callers cannot
/// inject an arbitrary RegionSource or forge/reopen its allocation authority.
pub fn claim_startup<const N: usize>(
    region: &'static StartupRegion<N>,
) -> Result<FixedStartupAllocator, StartupError> {
    FixedStartupAllocator::claim(region)
}
