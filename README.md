# dcs-bios-rs

This is a 'port' of 

A `no_std` Rust implementation of the DCS-BIOS export protocol, intended for embedded targets such as `esp-hal` as well as normal Rust applications.

The project currently contains a reusable protocol library, generated aircraft bindings, a code generator, and a standard-library UDP example.

## Workspace structure

```text
.
├── model-definitions/       DCS-BIOS JSON model definitions
├── crates/
│   ├── lib/                 no_std protocol and control library
│   ├── generated/           generated aircraft-specific bindings
│   ├── dcs-bios-codegen/   JSON-to-Rust code generator
│   └── std-bin/             std UDP example/application
└── generate-planes.sh       regenerate aircraft bindings
```

### `dcs-bios-rs-lib`

The core library is `no_std` and does not perform any transport or I/O. It accepts byte slices, so the caller can provide data received from UDP, serial, or another transport.

It contains:

- The DCS-BIOS packet parser
- Frame synchronization handling
- Generic output controls such as `Selector`, `Led`, `Display`, and dial/gauge types
- Fixed-capacity dispatch using `heapless::Vec`
- Aircraft-independent control and protocol logic

Control protocol metadata is encoded in const-generic types. For example:

```rust
Selector<30772, 7168, 10>
```

represents a selector at address `30772`, using mask `7168` and shift `10`. These values are compile-time metadata and are not stored in every control instance. Each instance only stores its runtime value, pending value, and dirty state.

## DCS-BIOS protocol flow

The parser accepts arbitrary byte slices and maintains state between calls. It recognizes four consecutive `0x55` bytes as a frame boundary.

Control updates are staged while packets are received. At the next frame boundary, all registered controls commit their staged values. Callbacks run only when the committed public value changed.

```text
incoming bytes
      │
      ▼
ProtocolParser
      │
      ├── packet data ──► registered controls stage values
      │
      └── 0x55 × 4 ────► controls commit and changed callbacks run
```

The first synchronization sequence establishes the initial frame. A later synchronization sequence commits the preceding frame's updates.

## Sparse control registration

Applications generally use only a small subset of the controls in an aircraft model. The builder therefore stores only the controls explicitly registered by the application instead of creating every possible aircraft control.

Registration happens during startup. After building the aircraft runtime, the application only feeds received bytes to it.

```rust
use dcs_bios_rs_generated::av8bna::{
    acp::JettModeValue,
    Av8bnaBuilder,
};

let mut on_jett_mode = |value: JettModeValue| {
    // Handle a changed selector value.
    println!("JETT_MODE: {:?}", value);
};

let mut builder = Av8bnaBuilder::<4, 256>::new();
builder.register_jett_mode(&mut on_jett_mode)?;

let mut aircraft = builder.build();
aircraft.feed(received_bytes)?;
```

The capacity parameters are compile-time constants:

- `MAX_CONTROLS` is the maximum number of registered controls.
- `MAX_PACKET_DATA` is the largest packet payload the parser will store.

If registration exceeds `MAX_CONTROLS`, registration returns an error. If a packet exceeds `MAX_PACKET_DATA`, feeding returns a parse error.

## Typed selector values

Selectors with a JSON `max_value` from `0` through `10` receive generated enums. DCS-BIOS selector values are zero-based on the wire, while the generated public variants are named `Select1` through `SelectN`:

```rust
pub enum JettModeValue {
    Select1,
    Select2,
    Select3,
    Select4,
    Select5,
}
```

For `JETT_MODE`, the mapping is:

```text
wire value 0 ──► Select1
wire value 1 ──► Select2
wire value 2 ──► Select3
wire value 3 ──► Select4
wire value 4 ──► Select5
```

The JSON descriptions do not consistently provide machine-readable names for selector positions, so generated enums use the generic `SelectN` names. Human-readable names can be added later through model metadata or explicit overrides.

Selectors without a bounded range, or with a maximum above `10`, continue to use a `u16` callback value.

## Generated aircraft bindings

The `model-definitions` directory contains DCS-BIOS JSON files for aircraft such as:

- AV-8BNA
- A-10C
- AJS37
- F-16C 50
- P-51D
- UH-1H

- The generated crate exposes one module per aircraft, with JSON groups as submodules, for example:

```rust
use dcs_bios_rs_generated::av8bna::{acp::JettModeValue, Av8bnaBuilder};
use dcs_bios_rs_generated::a10c::A10cBuilder;
```

Each generated aircraft module contains:

- Typed wrappers for output-bearing controls
- Const-generic protocol mappings
- Typed selector, LED, display, and integer callbacks
- A sparse `RegisteredControl` collection
- A builder and runtime dispatcher
- A `feed(&[u8])` method

Controls with multiple outputs are generated as separate output-specific controls using the JSON output suffix where available. Controls that have no outputs are input-only and are not registered with the output dispatcher.

Generated files contain a header indicating that they should not be edited manually.

## Regenerating aircraft bindings

The root script uses the standard model and output directories by default:

```bash
./generate-planes.sh
```

This is equivalent to:

```bash
cargo run -p dcs-bios-codegen -- \
  --input-dir model-definitions \
  --output-dir crates/generated/src
```

Custom paths can be supplied to the script and are forwarded to the generator:

```bash
./generate-planes.sh \
  --input-dir /path/to/model-definitions \
  --output-dir /path/to/generated/src
```

The script also runs `cargo fmt --all` after generation.

## Standard-library example

`crates/std-bin` contains a standard-library example that binds a UDP socket, receives DCS-BIOS bytes, and feeds them to an aircraft runtime. The core library itself remains transport-independent and `no_std`.

## Validation

The workspace can be checked with:

```bash
cargo test --workspace --all-targets
cargo check --workspace
cargo fmt --all -- --check
```

## License

This project is licensed under the GNU General Public License, version 3 only. See [`LICENSE`](LICENSE) for the complete license text.
