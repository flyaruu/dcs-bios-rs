use crate::protocol::Packet;

pub trait Control {
    fn accept(&mut self, packet: Packet<'_>);
    /// Commits the staged value and returns whether the public value changed.
    fn commit(&mut self) -> bool;
}

fn integer_value<const ADDRESS: u16, const MASK: u16, const SHIFT: u8>(
    packet: Packet<'_>,
) -> Option<u16> {
    if ADDRESS < packet.address || ADDRESS.checked_add(2)? > packet.end_address() {
        return None;
    }
    let offset = usize::from(ADDRESS - packet.address);
    let word = u16::from_le_bytes([packet.data[offset], packet.data[offset + 1]]);
    Some((word & MASK) >> SHIFT)
}

fn overlaps<const ADDRESS: u16, const MAX_LENGTH: usize>(
    packet: Packet<'_>,
) -> Option<(usize, usize)> {
    let start = core::cmp::max(ADDRESS, packet.address);
    let end = core::cmp::min(
        ADDRESS.checked_add(MAX_LENGTH as u16)?,
        packet.end_address(),
    );
    if start >= end {
        return None;
    }
    Some((
        usize::from(start - ADDRESS),
        usize::from(start - packet.address),
    ))
}

macro_rules! integer_control {
    ($name:ident) => {
        pub struct $name<const ADDRESS: u16, const MASK: u16, const SHIFT: u8> {
            value: u16,
            pending: u16,
            dirty: bool,
        }

        impl<const ADDRESS: u16, const MASK: u16, const SHIFT: u8> $name<ADDRESS, MASK, SHIFT> {
            pub const fn new() -> Self {
                Self {
                    value: 0,
                    pending: 0,
                    dirty: false,
                }
            }

            pub const fn value(&self) -> u16 {
                self.value
            }
        }

        impl<const ADDRESS: u16, const MASK: u16, const SHIFT: u8> Control
            for $name<ADDRESS, MASK, SHIFT>
        {
            fn accept(&mut self, packet: Packet<'_>) {
                if let Some(value) = integer_value::<ADDRESS, MASK, SHIFT>(packet) {
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

pub struct Led<const ADDRESS: u16, const MASK: u16, const SHIFT: u8> {
    value: bool,
    pending: bool,
    dirty: bool,
}

impl<const ADDRESS: u16, const MASK: u16, const SHIFT: u8> Led<ADDRESS, MASK, SHIFT> {
    pub const fn new() -> Self {
        Self {
            value: false,
            pending: false,
            dirty: false,
        }
    }

    pub const fn is_on(&self) -> bool {
        self.value
    }
}

impl<const ADDRESS: u16, const MASK: u16, const SHIFT: u8> Control for Led<ADDRESS, MASK, SHIFT> {
    fn accept(&mut self, packet: Packet<'_>) {
        if let Some(value) = integer_value::<ADDRESS, MASK, SHIFT>(packet) {
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

pub struct Display<const ADDRESS: u16, const N: usize> {
    value: [u8; N],
    pending: [u8; N],
    dirty: bool,
}

impl<const ADDRESS: u16, const N: usize> Display<ADDRESS, N> {
    pub const fn new() -> Self {
        Self {
            value: [b' '; N],
            pending: [b' '; N],
            dirty: false,
        }
    }

    pub fn value(&self) -> &[u8] {
        &self.value
    }
}

impl<const ADDRESS: u16, const N: usize> Control for Display<ADDRESS, N> {
    fn accept(&mut self, packet: Packet<'_>) {
        if let Some((output_offset, packet_offset)) = overlaps::<ADDRESS, N>(packet) {
            let count = core::cmp::min(packet.data.len() - packet_offset, N - output_offset);
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
