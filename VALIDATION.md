# Experimental host validation — November14,2026

> Historical record at SDK `7cfd1b30fe674f7b93a74725c6d458beec5d041b`. Successor
> compile checks at `02c6340115f089dd13552f3a03ba30de1d98d8b4` are in the final section.

The independent source review accepted this thin public provider and the common
allocator's body-preserving move into the optional SDK crate. The provider Rust
source and host example remain exactly the reviewed versions. Both public
providers consume SDK`7cfd1b30fe674f7b93a74725c6d458beec5d041b` through Git; Cargo metadata and locks resolve both
up-rust-alloc and up-rust-core to that exact SHA, with no path/patch SDK override.

Rust1.100.0 offline locked checks passed for this library and its host example.
A separate consumer importing both providers and assigning both results to the
same shared SDK allocator type also passed its exact-public-SDK check. The new
local-source consumer had already run successfully: disjoint fixed reservations,
retained contents after seal, growth refusal and region claim-once. That run was
not replayed solely for the public Git pin. Seven accepted generic allocator
controls are reused through the independently verified unchanged implementation;
they were not duplicated for these wrappers.

```sh
cargo +1.100.0 check --offline --locked --lib --examples
```

Fetch the pinned Git dependency once before using offline mode. No default Rust
toolchain or integrated core MSRV changed; this optional experimental crate
requires1.100. `publish=false` keeps registry release disabled.

Target compilation, final link, QEMU and physical-board execution have not passed
for this public provider revision. Shared native32 core/startup work is separate.
No RTOS, autonomous-service, native transport, timing, Ferrocene/ASIL or G1/G2/G4
acceptance is claimed. See README and profiles for the concrete target boundaries.

## Successor compile checks at SDK `02c6340` — November 15, 2026

The manifest and lock resolve `up-rust-alloc` and `up-rust-core` from the public
Git branch `codex/allocator-mcu-spike` at `02c6340115f089dd13552f3a03ba30de1d98d8b4`,
with no path or patch override; only the SDK source lines changed in `Cargo.lock`.
Offline locked Rust1.100.0 checks passed on an aarch64 macOS host:

```sh
cargo +1.100.0 check --offline --locked --lib
cargo +1.100.0 check --offline --locked --examples
RUSTFLAGS='--cfg up_rust_native32_model="riscv32-sifive-e"' \
  cargo +1.100.0 check --offline --locked --lib --target riscv32imac-unknown-none-elf
```

The provider source and host example are unchanged. These are compile checks
only: no example, final link, QEMU or hardware execution ran, and no RTOS,
timing, Ferrocene/ASIL or G1/G2/G4 acceptance is claimed.
