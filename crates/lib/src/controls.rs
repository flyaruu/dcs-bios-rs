use crate::protocol::Packet;

#[derive(Clone, Copy)]
pub struct IntegerOutput {
    pub address: u16,
    pub mask: u16,
    pub shift: u8,
}

#[derive(Clone, Copy)]
pub struct StringOutput {
    pub address: u16,
    pub max_length: usize,
}

pub trait Control {
    fn accept(&mut self, packet: Packet<'_>);
    /// Commits the staged value and returns whether the public value changed.
    fn commit(&mut self) -> bool;
}

fn integer_value(output: IntegerOutput, packet: Packet<'_>) -> Option<u16> {
    if output.address < packet.address || output.address.checked_add(2)? > packet.end_address() {
        return None;
    }
    let offset = usize::from(output.address - packet.address);
    let word = u16::from_le_bytes([packet.data[offset], packet.data[offset + 1]]);
    Some((word & output.mask) >> output.shift)
}

fn overlaps(output: StringOutput, packet: Packet<'_>) -> Option<(usize, usize)> {
    let start = core::cmp::max(output.address, packet.address);
    let end = core::cmp::min(
        output.address.checked_add(output.max_length as u16)?,
        packet.end_address(),
    );
    if start >= end {
        return None;
    }
    Some((
        usize::from(start - output.address),
        usize::from(start - packet.address),
    ))
}

macro_rules! integer_control {
    ($name:ident) => {
        pub struct $name {
            output: IntegerOutput,
            value: u16,
            pending: u16,
            dirty: bool,
        }

        impl $name {
            pub const fn new(address: u16, mask: u16, shift: u8) -> Self {
                Self {
                    output: IntegerOutput {
                        address,
                        mask,
                        shift,
                    },
                    value: 0,
                    pending: 0,
                    dirty: false,
                }
            }

            pub const fn value(&self) -> u16 {
                self.value
            }
        }

        impl Control for $name {
            fn accept(&mut self, packet: Packet<'_>) {
                if let Some(value) = integer_value(self.output, packet) {
                    self.pending = value;
                    self.dirty = true;
                }
            }

            fn commit(&mut self) -> bool {
                if self.dirty {
                    let changed = self.value != self.pending;
                    self.value = self.pending;
                    self.dirty = false;
                    changed
                } else {
                    false
                }
            }
        }
    };
}

integer_control!(Selector);
integer_control!(AnalogGauge);
integer_control!(LimitedDial);
integer_control!(AnalogDial);
integer_control!(FixedStepDial);
integer_control!(Metadata);

pub struct Led {
    output: IntegerOutput,
    value: bool,
    pending: bool,
    dirty: bool,
}

impl Led {
    pub const fn new(address: u16, mask: u16, shift: u8) -> Self {
        Self {
            output: IntegerOutput {
                address,
                mask,
                shift,
            },
            value: false,
            pending: false,
            dirty: false,
        }
    }

    pub const fn is_on(&self) -> bool {
        self.value
    }
}

impl Control for Led {
    fn accept(&mut self, packet: Packet<'_>) {
        if let Some(value) = integer_value(self.output, packet) {
            self.pending = value != 0;
            self.dirty = true;
        }
    }

    fn commit(&mut self) -> bool {
        if self.dirty {
            let changed = self.value != self.pending;
            self.value = self.pending;
            self.dirty = false;
            changed
        } else {
            false
        }
    }
}

pub struct Display<const N: usize> {
    output: StringOutput,
    value: [u8; N],
    pending: [u8; N],
    dirty: bool,
}

impl<const N: usize> Display<N> {
    pub const fn new(address: u16, max_length: usize) -> Self {
        assert!(max_length <= N);
        Self {
            output: StringOutput {
                address,
                max_length,
            },
            value: [b' '; N],
            pending: [b' '; N],
            dirty: false,
        }
    }

    pub fn value(&self) -> &[u8] {
        &self.value[..self.output.max_length]
    }
}

impl<const N: usize> Control for Display<N> {
    fn accept(&mut self, packet: Packet<'_>) {
        if let Some((output_offset, packet_offset)) = overlaps(self.output, packet) {
            let count = core::cmp::min(
                packet.data.len() - packet_offset,
                self.output.max_length - output_offset,
            );
            self.pending[output_offset..output_offset + count]
                .copy_from_slice(&packet.data[packet_offset..packet_offset + count]);
            self.dirty = true;
        }
    }

    fn commit(&mut self) -> bool {
        if self.dirty {
            let changed = self.value != self.pending;
            self.value = self.pending;
            self.dirty = false;
            changed
        } else {
            false
        }
    }
}
