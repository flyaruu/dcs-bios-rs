# dcs-bios-rs-generated

Generated `no_std` Rust bindings for [DCS-BIOS](https://github.com/DCS-Skunkworks/dcs-bios) model definitions.

DCS-BIOS provides a data interface between DCS World and external applications, panels, and hardware. This crate contains Rust bindings generated from the DCS-BIOS JSON model definitions. The generated aircraft modules expose typed controls, selector values, callbacks, and sparse builders for receiving DCS-BIOS output data.

The bindings are generated code. Do not edit the files under `src/` manually; regenerate them from the model definitions instead:

```bash
./generate-planes.sh
```

The generated crate is `no_std` and does not perform network or serial I/O. Applications provide received DCS-BIOS bytes to the aircraft runtime through its `feed` method. Transport handling belongs in the application using this crate.

## Included models

| DCS-BIOS model | Rust module |
| --- | --- |
| A-10C | `a10c` |
| AJS37 | `ajs37` |
| AV-8BNA | `av8bna` |
| CommonData | `commondata` |
| F-14B | `f14b` |
| F-16C_50 | `f16c50` |
| F-5E-3 | `f5e3` |
| FA-18C_hornet | `fa18chornet` |
| FC3 | `fc3` |
| M-2000C | `m2000c` |
| P-51D | `p51d` |
| UH-1H | `uh1h` |

For example, the AV-8BNA builder and a selector value can be imported with:

```rust
use dcs_bios_rs_generated::av8bna::{acp::JettModeValue, Av8bnaBuilder};
```

The model definitions are maintained by the DCS-BIOS project and may change as aircraft modules are updated. Regenerate the bindings when the model definitions change.

## License

This crate is part of `dcs-bios-rs` and is licensed under the GNU General Public License, version 3 only. See the repository [`LICENSE`](../../LICENSE) file for the complete license text.
