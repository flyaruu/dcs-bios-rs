# dcs-bios-rs-lib

A `no_std` Rust implementation of the [DCS-BIOS](https://github.com/DCS-Skunkworks/dcs-bios) export protocol.

This crate contains the transport-independent protocol and control primitives used by the generated aircraft bindings. It is suitable for embedded applications and does not require an allocator, operating system, network stack, or serial driver.

## What it provides

- Incremental parsing of DCS-BIOS data from arbitrary byte slices
- Packet parsing and frame synchronization
- Frame-boundary commits for staged control values
- Generic output controls:
  - `Selector`
  - `Metadata`
  - `Led`
  - `Display`
  - `AnalogGauge`
  - `AnalogDial`
  - `FixedStepDial`
  - `LimitedDial`
- `heapless` fixed-capacity collections for bounded embedded use
- The `Control` and `Sink` traits for connecting controls to a dispatcher

## Protocol handling

The parser maintains state between calls, so input can arrive in any chunk size:

```rust
use dcs_bios_rs_lib::{ProtocolParser, Sink};

let mut parser = ProtocolParser::<256>::new();
let mut sink = MySink::new();

parser.feed(received_bytes, &mut sink)?;
```

DCS-BIOS uses four consecutive `0x55` bytes to mark a frame boundary. Packet updates are delivered to controls as they arrive, but controls commit their pending values only when the next frame boundary is received. This allows callbacks to observe a consistent set of values from the completed frame.

The library does not own the input transport. An application can feed bytes received over UDP, serial, USB, or any other source.

## Control metadata

Control protocol metadata is encoded in const-generic parameters rather than stored in each instance:

```rust
use dcs_bios_rs_lib::Selector;

let selector = Selector::<30_772, 7_168, 10>::new();
```

The address, mask, and shift are compile-time values. Control instances only contain their runtime and pending state.

Aircraft-specific control names, registration builders, typed callbacks, and selector enums are generated in [`dcs-bios-rs-generated`](../generated).

## Capacity

The parser's packet buffer capacity is a const-generic parameter. Applications choose a capacity appropriate for their target:

```rust
let parser = ProtocolParser::<256>::new();
```

The generated aircraft builders also use a compile-time maximum number of registered controls. This keeps memory usage predictable while allowing applications to register only the controls they need.

## License

This crate is part of `dcs-bios-rs` and is licensed under the GNU General Public License, version 3 only. See the repository [`LICENSE`](../../LICENSE) file for the complete license text.
