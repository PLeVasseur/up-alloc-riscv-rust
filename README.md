# up-alloc-riscv-rust

Safe fixed startup-memory provider entry point for experimental RISC-V work.

```rust
use up_alloc_riscv_rust::{claim_startup, StartupRegion};
use up_rust_alloc::try_vec_in;

static REGION: StartupRegion<4096> = StartupRegion::new();

// During application initialization:
let allocator = claim_startup(&REGION)?;
let mut values = try_vec_in::<u32, _>(&allocator, 8)?;
let reserved = allocator.seal();
values.push(7); // Within the capacity reserved above.
```

The snippet is a call-shape illustration: errors belong to the application's
setup error type. The complete host example handles them explicitly.

## RISC-V profile

The initial portability candidate is SiFive FE310 / HiFive1 Rev B with
`riscv32imac-unknown-none-elf`; QEMU `sifive_e` is an emulator harness, not evidence
of identical physical-board peripherals or timing. FE310 remains **provisional**
until the separate Ferrocene MCU/RTOS-fit investigation establishes the intended
supported target. This package currently contains no FE310 HAL or RTOS binding.

The `profiles/*.json` file states current evidence and intended target check.
The `.cargo.toml` file contains only target selection, to pass with `cargo
--config`; it is not a linker script, board-support package or admission receipt.
A replacement RISC-V board can reuse this platform-independent allocation
mechanism without copying its unsafe implementation.

## Scope and current status

This is an **experimental integration**, not a qualified target allocator or
production SDK release. It uses stable Rust 1.100's standard `Allocator` API in an
optional `no_std + alloc` SDK crate. The integrated production core's MSRV and
four wire/transport pins are unchanged. Ferrocene target/toolchain and `alloc`
qualification are separate evidence questions; no such acceptance is claimed.

The manifest pins [the public experimental SDK](https://github.com/PLeVasseur/up-rust/tree/7cfd1b30fe674f7b93a74725c6d458beec5d041b)
to `7cfd1b30fe674f7b93a74725c6d458beec5d041b`. Both provider repositories use that same revision. There are no
private SDK copies, local path dependencies, or patch overrides in this package.
This experimental SDK does not yet contain the separate native32 repair.

The implementation belongs to the shared SDK's `up-rust-alloc::fixed`, **once**.
This independent crate supplies a safe public entry point and platform-profile
configuration; it does not copy an arena, modify private SDK modules or register
as a privileged provider. Other authors can implement Rust's standard unsafe
`Allocator` in their own crate or use these same public SDK hooks.

`up-rust-core` remains `no_std`, `no_alloc`, dependency-free and feature-free.
The alloc helper does not depend back on either provider. Native payload loans
and retained receive samples remain owned by the underlying transport; this
startup allocator is for SDK/application memory, not a native payload copy layer.

## What fixed-region allocation means

The caller provides `StartupRegion<N>`, whose storage is claimed once through the
SDK. All reservations use core's checked alignment/capacity rules. The common
allocator is local (`!Sync`); it has no RTOS lock, interrupt handler or hidden
cross-thread promise. An application may use distinct regions in distinct
contexts. This optional provider does not change any shared SDK transport handle
contract.

`seal()` permanently prevents new allocator reservations while preserving live
owners and pre-reserved capacity. This is **not** the SDK's full setup/runtime
activation and does not make arbitrary `Vec`/`Box` methods or element destructors
native-runtime safe. Use fallible reservation during setup; ordinary infallible
operations can still panic or reach allocation-error handling. Empty/ZST standard
collections may succeed without consulting the allocator even after sealing.

Deallocation retains physical backing; growth/shrink may consume additional
permanent region bytes. Dropping or forgetting an owner cannot reopen its region.
Core `MemoryReport` counts linked logical reservations, not a full memory budget.
Automatic fixed-memory sizing remains unfinished SDK work.

## Builds and final links

The included `examples/fixed_region.rs` is a **host** application example using
safe public APIs. Its `expect`/asserts are host demonstration code, not approved
runtime failure handling. No bare-metal boot image or RTOS integration is hidden
inside that example. The smallest host check is:

```sh
cargo +1.100.0 check --locked --lib --examples
cargo +1.100.0 run --locked --example fixed_region
```

A `no_std + alloc` library check alone does not establish a final MCU image.
A preserved Rust 1.100 staticlib probe requires global-allocator registration even
when selected operations use only an explicit allocator. Registration, allocation
error handling, panic handling, board memory layout and startup remain final-link
work. No global heap fallback is introduced here. A rejecting global registration
is prepared in the separate, unrun QEMU test harness; its independent review and
full-core build/link gates remain open. It is not part of this provider or a
production runtime guarantee.

The selected full SDK currently imports `AtomicU64`, unavailable on the listed
32-bit MCU targets. The actual target check is therefore expected to expose that
shared-core boundary. No private memory-only SDK fork, force-enabled atomics or
silently disabled core modules are used. Source, host compilation/execution,
target compilation, final link, emulator execution and hardware qualification
must be recorded separately. G1/G2/G4 and clock/entropy/exclusion are not solved
by this allocation experiment.

Host checks against the exact public SDK revision passed on Rust1.100.0.
See [VALIDATION.md](VALIDATION.md) for scope and [SDK-PIN.json](SDK-PIN.json)
for the dependency pin. Target and QEMU results remain open.
