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
optional `no_std + alloc` SDK crate. The production core pin `204eaa24…` and
its Rust 1.88 MSRV are unchanged; the two wires, two uP-L1 transports and both
providers share the experimental SDK revision named below. Ferrocene
target/toolchain and `alloc` qualification are separate evidence questions; no
such acceptance is claimed.

The manifest pins [the public experimental SDK](https://github.com/PLeVasseur/up-rust/tree/dc7f22da2bf90d98d94d468621df26caadf853ca)
to `dc7f22da2bf90d98d94d468621df26caadf853ca` (branch `codex/allocator-mcu-spike`). Both provider repositories use that same revision. There are no
private SDK copies, local path dependencies, or patch overrides in this package.
This SDK revision includes the shared native32 execution-model selection described below.

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
lives in the separate QEMU test harness, which is not part of this provider or a
production runtime guarantee. After its independent review, that harness linked
Rust 1.100 release images from this provider's current library source and a
frozen pre-publication SDK source tree (native32/allocator sources before the
SDK's later G2/G4 composition). On November 14, 2026 the RV32IMAC image passed
its native32 and allocator probes once under QEMU `sifive_e,revb=on`, exit 0
(ELF SHA-256 `9c84390f…`). That run binds that exact image only, not this
provider at the pinned SDK revision.

At this SDK revision the shared core compiles for a bare-metal target only with
an explicitly selected SDK-owned execution model. For
`riscv32imac-unknown-none-elf` that is
`RUSTFLAGS='--cfg up_rust_native32_model="riscv32-sifive-e"'`, the SDK's SiFive E
image profile used with the QEMU `sifive_e` harness; it is not physical HiFive1
Rev B qualification. No private memory-only SDK fork, force-enabled atomics or
silently disabled core modules are used. Source, host compilation/execution,
target compilation, final link, emulator execution and hardware qualification
must be recorded separately. G1/G2/G4 and clock/entropy/exclusion are not solved
by this allocation experiment.

At this SDK revision, locked Rust1.100.0 checks passed for the host library, the
host example and the `riscv32imac-unknown-none-elf` library with the `riscv32-sifive-e` model, on an
aarch64 macOS host. See [VALIDATION.md](VALIDATION.md) for scope and
[SDK-PIN.json](SDK-PIN.json) for the dependency pin. Final link, QEMU and hardware
results remain open for this provider revision.
