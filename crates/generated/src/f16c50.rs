//! Generated from model-definitions/F-16C_50.json.
//! Do not edit manually; rerun dcs-bios-codegen instead.

#![allow(unused_imports)]

use core::marker::PhantomData;
use dcs_bios_rs_lib::{
    AnalogDial, AnalogGauge, Control, Display, FeedError, FixedStepDial, Led, LimitedDial,
    Metadata, Packet, ProtocolParser, Selector, Sink,
};
use heapless::Vec;

pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

pub mod adi {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// ADI AUX Flag
    /// Output: gauge position
    pub struct AdiAuxFlag(AnalogGauge<17592, 65535, 0>);
    impl AdiAuxFlag {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdiAuxFlag {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdiAuxFlag {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADI Bank
    /// Output: gauge position
    pub struct AdiBank(AnalogGauge<17586, 65535, 0>);
    impl AdiBank {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdiBank {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdiBank {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADI Bubble
    /// Output: gauge position
    pub struct AdiBubble(AnalogGauge<17604, 65535, 0>);
    impl AdiBubble {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdiBubble {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdiBubble {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADI GS Bar
    /// Output: gauge position
    pub struct AdiGsBar(AnalogGauge<17598, 65535, 0>);
    impl AdiGsBar {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdiGsBar {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdiGsBar {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADI GS Flag
    /// Output: gauge position
    pub struct AdiGsFlag(AnalogGauge<17594, 65535, 0>);
    impl AdiGsFlag {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdiGsFlag {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdiGsFlag {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADI GS Pointer
    /// Output: gauge position
    pub struct AdiGsPointer(AnalogGauge<17602, 65535, 0>);
    impl AdiGsPointer {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdiGsPointer {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdiGsPointer {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADI LOC Bar
    /// Output: gauge position
    pub struct AdiLocBar(AnalogGauge<17596, 65535, 0>);
    impl AdiLocBar {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdiLocBar {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdiLocBar {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADI LOC Flag
    /// Output: gauge position
    pub struct AdiLocFlag(AnalogGauge<17590, 65535, 0>);
    impl AdiLocFlag {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdiLocFlag {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdiLocFlag {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADI Off Flag
    /// Output: gauge position
    pub struct AdiOffFlag(AnalogGauge<17588, 65535, 0>);
    impl AdiOffFlag {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdiOffFlag {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdiOffFlag {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADI Pitch
    /// Output: gauge position
    pub struct AdiPitch(AnalogGauge<17584, 65535, 0>);
    impl AdiPitch {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdiPitch {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdiPitch {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADI Pitch Trim Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct AdiPitchTrim(LimitedDial<17514, 65535, 0>);
    impl AdiPitchTrim {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdiPitchTrim {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdiPitchTrim {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADI TurnRate
    /// Output: gauge position
    pub struct AdiTurnrate(AnalogGauge<17600, 65535, 0>);
    impl AdiTurnrate {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdiTurnrate {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdiTurnrate {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ECM DIM Rotary Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct EcmDimKnb(LimitedDial<17842, 65535, 0>);
    impl EcmDimKnb {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EcmDimKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EcmDimKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod airspeedindicator {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Airspeed Indicator
    /// Output: gauge position
    pub struct Airspeed(AnalogGauge<17564, 65535, 0>);
    impl Airspeed {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Airspeed {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Airspeed {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// SET INDEX Knob
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct AirspeedSetKnb(AnalogDial<17508, 65535, 0>);
    impl AirspeedSetKnb {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AirspeedSetKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AirspeedSetKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Mach Indicator
    /// Output: gauge position
    pub struct MachIndicator(AnalogGauge<17570, 65535, 0>);
    impl MachIndicator {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MachIndicator {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MachIndicator {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Max Airspeed
    /// Output: gauge position
    pub struct MaxAirspeed(AnalogGauge<17566, 65535, 0>);
    impl MaxAirspeed {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MaxAirspeed {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MaxAirspeed {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Set Airspeed
    /// Output: gauge position
    pub struct SetAirspeed(AnalogGauge<17568, 65535, 0>);
    impl SetAirspeed {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SetAirspeed {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SetAirspeed {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod altimeter {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Altimeter 10000 ft count
    /// Output: gauge position
    pub struct Alt10000FtCnt(AnalogGauge<17548, 65535, 0>);
    impl Alt10000FtCnt {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Alt10000FtCnt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Alt10000FtCnt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Altimeter 1000 ft count
    /// Output: gauge position
    pub struct Alt1000FtCnt(AnalogGauge<17550, 65535, 0>);
    impl Alt1000FtCnt {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Alt1000FtCnt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Alt1000FtCnt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Altimeter 100 ft count
    /// Output: gauge position
    pub struct Alt100FtCnt(AnalogGauge<17552, 65535, 0>);
    impl Alt100FtCnt {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Alt100FtCnt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Alt100FtCnt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Altimeter 100 ft pointer
    /// Output: gauge position
    pub struct Alt100FtPtr(AnalogGauge<17546, 65535, 0>);
    impl Alt100FtPtr {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Alt100FtPtr {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Alt100FtPtr {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Altimeter Barometric Setting Knob
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct AltBaroSetKnb(AnalogDial<17510, 65535, 0>);
    impl AltBaroSetKnb {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AltBaroSetKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AltBaroSetKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Altimeter Mode Lever, ELEC/OFF/PNEU
    /// Input: set the switch position
    /// Output: switch position -- 0 = Down, 1 = Mid ,  2 = UP
    pub struct AltModeLv(Selector<17506, 768, 8>);
    impl AltModeLv {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AltModeLv {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AltModeLv {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Altimeter PNEU Flag
    /// Output: gauge position
    pub struct AltPneuFlag(AnalogGauge<17562, 65535, 0>);
    impl AltPneuFlag {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AltPneuFlag {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AltPneuFlag {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Altimeter Pressure Setting Drum 0
    /// Output: gauge position
    pub struct AltPressureDrum0Cnt(AnalogGauge<17554, 65535, 0>);
    impl AltPressureDrum0Cnt {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AltPressureDrum0Cnt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AltPressureDrum0Cnt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Altimeter Pressure Setting Drum 1
    /// Output: gauge position
    pub struct AltPressureDrum1Cnt(AnalogGauge<17556, 65535, 0>);
    impl AltPressureDrum1Cnt {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AltPressureDrum1Cnt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AltPressureDrum1Cnt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Altimeter Pressure Setting Drum 2
    /// Output: gauge position
    pub struct AltPressureDrum2Cnt(AnalogGauge<17558, 65535, 0>);
    impl AltPressureDrum2Cnt {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AltPressureDrum2Cnt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AltPressureDrum2Cnt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Altimeter Pressure Setting Drum 3
    /// Output: gauge position
    pub struct AltPressureDrum3Cnt(AnalogGauge<17560, 65535, 0>);
    impl AltPressureDrum3Cnt {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AltPressureDrum3Cnt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AltPressureDrum3Cnt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod aoa {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// AoA Value
    /// Output: gauge position
    pub struct AoaValue(AnalogGauge<17544, 65535, 0>);
    impl AoaValue {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AoaValue {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AoaValue {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod audiopanel {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Comm1ModeKnbValue {
        Select1,
        Select2,
        Select3,
    }
    pub type Comm1ModeKnbValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Comm1ModeKnbValue) + 'a>;
    impl core::convert::TryFrom<u16> for Comm1ModeKnbValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// COMM 1 (UHF) Mode Knob
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct Comm1ModeKnb(Selector<17458, 49152, 14>);
    impl Comm1ModeKnb {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Comm1ModeKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Comm1ModeKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// COMM 1 Power Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct Comm1PwrKnb(LimitedDial<17462, 65535, 0>);
    impl Comm1PwrKnb {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Comm1PwrKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Comm1PwrKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Comm2ModeKnbValue {
        Select1,
        Select2,
        Select3,
    }
    pub type Comm2ModeKnbValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Comm2ModeKnbValue) + 'a>;
    impl core::convert::TryFrom<u16> for Comm2ModeKnbValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// COMM 2 (VHF) Mode Knob
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct Comm2ModeKnb(Selector<17460, 3, 0>);
    impl Comm2ModeKnb {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Comm2ModeKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Comm2ModeKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// COMM 2 Power Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct Comm2PwrKnb(LimitedDial<17464, 65535, 0>);
    impl Comm2PwrKnb {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Comm2PwrKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Comm2PwrKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum HotMicSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type HotMicSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(HotMicSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for HotMicSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// HOT MIC CIPHER Switch, HOT MIC / OFF / CIPHER
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct HotMicSw(Selector<17460, 12, 2>);
    impl HotMicSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HotMicSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HotMicSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffAntSelSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type IffAntSelSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IffAntSelSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for IffAntSelSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// IFF ANT SEL Switch, LOWER/NORM/UPPER
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct IffAntSelSw(Selector<17460, 96, 5>);
    impl IffAntSelSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IffAntSelSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffAntSelSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ILS Power Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct IlsPwrKnb(LimitedDial<17478, 65535, 0>);
    impl IlsPwrKnb {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IlsPwrKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IlsPwrKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// INTERCOM Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct IntercomKnb(LimitedDial<17474, 65535, 0>);
    impl IntercomKnb {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IntercomKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IntercomKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// MSL Tone Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct MslKnb(LimitedDial<17468, 65535, 0>);
    impl MslKnb {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MslKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MslKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// SECURE VOICE Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct SecVoiceKnb(LimitedDial<17466, 65535, 0>);
    impl SecVoiceKnb {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SecVoiceKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SecVoiceKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// TACAN Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct TacanKnb(LimitedDial<17476, 65535, 0>);
    impl TacanKnb {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TacanKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TacanKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// TF Tone Knob Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct TfKnb(LimitedDial<17470, 65535, 0>);
    impl TfKnb {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TfKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TfKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// THREAT Tone Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct ThreatKnb(LimitedDial<17472, 65535, 0>);
    impl ThreatKnb {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ThreatKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ThreatKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UhfAntSelSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type UhfAntSelSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(UhfAntSelSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for UhfAntSelSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// UHF ANT SEL Switch, LOWER/NORM/UPPER
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct UhfAntSelSw(Selector<17460, 384, 7>);
    impl UhfAntSelSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfAntSelSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfAntSelSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum VmsInhibitSwValue {
        Select1,
        Select2,
    }
    pub type VmsInhibitSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(VmsInhibitSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for VmsInhibitSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Voice Message Inhibit Switch, VOICE MESSAGE/INHIBIT
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct VmsInhibitSw(Selector<17460, 16, 4>);
    impl VmsInhibitSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for VmsInhibitSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for VmsInhibitSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod avionicpanel {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DlSwValue {
        Select1,
        Select2,
    }
    pub type DlSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(DlSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for DlSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// DL Switch, DL/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct DlSw(Selector<17444, 16, 4>);
    impl DlSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for DlSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for DlSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GpsSwValue {
        Select1,
        Select2,
    }
    pub type GpsSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(GpsSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for GpsSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// GPS Switch, GPS/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct GpsSw(Selector<17442, 8192, 13>);
    impl GpsSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for GpsSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for GpsSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum InsKnbValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
        Select6,
        Select7,
    }
    pub type InsKnbValueCallback<'a> = alloc::boxed::Box<dyn FnMut(InsKnbValue) + 'a>;
    impl core::convert::TryFrom<u16> for InsKnbValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                3 => Ok(Self::Select4),
                4 => Ok(Self::Select5),
                5 => Ok(Self::Select6),
                6 => Ok(Self::Select7),
                value => Err(value),
            }
        }
    }

    /// INS Knob, OFF/STOR HDG/NORM/NAV/CAL/INFLT ALIGN/ATT
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct InsKnb(Selector<17444, 7, 0>);
    impl InsKnb {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for InsKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for InsKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MapSwValue {
        Select1,
        Select2,
    }
    pub type MapSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MapSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for MapSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MAP Switch, MAP/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MapSw(Selector<17444, 8, 3>);
    impl MapSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MapSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MapSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdSwValue {
        Select1,
        Select2,
    }
    pub type MfdSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for MfdSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Switch, MFD/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdSw(Selector<17442, 2048, 11>);
    impl MfdSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MidsLvtKnbValue {
        Select1,
        Select2,
        Select3,
    }
    pub type MidsLvtKnbValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MidsLvtKnbValue) + 'a>;
    impl core::convert::TryFrom<u16> for MidsLvtKnbValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// MIDS LVT Knob, ZERO/OFF/ON
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct MidsLvtKnb(Selector<17442, 49152, 14>);
    impl MidsLvtKnb {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MidsLvtKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MidsLvtKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MmcPwrSwValue {
        Select1,
        Select2,
    }
    pub type MmcPwrSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MmcPwrSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for MmcPwrSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MMC Switch, MMC/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MmcPwrSw(Selector<17442, 512, 9>);
    impl MmcPwrSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MmcPwrSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MmcPwrSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum StStaSwValue {
        Select1,
        Select2,
    }
    pub type StStaSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(StStaSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for StStaSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ST STA Switch, ST STA/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct StStaSw(Selector<17442, 1024, 10>);
    impl StStaSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for StStaSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for StStaSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UfcSwValue {
        Select1,
        Select2,
    }
    pub type UfcSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(UfcSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for UfcSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// UFC Switch, UFC/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UfcSw(Selector<17442, 4096, 12>);
    impl UfcSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UfcSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UfcSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod clock {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Current Time Hours
    /// Output: gauge position
    pub struct ClockCurrtimeH(AnalogGauge<17656, 65535, 0>);
    impl ClockCurrtimeH {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ClockCurrtimeH {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClockCurrtimeH {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Current Time Minutes
    /// Output: gauge position
    pub struct ClockCurrtimeMs(AnalogGauge<17658, 65535, 0>);
    impl ClockCurrtimeMs {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ClockCurrtimeMs {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClockCurrtimeMs {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ClockElapsedValue {
        Select1,
        Select2,
    }
    pub type ClockElapsedValueCallback<'a> = alloc::boxed::Box<dyn FnMut(ClockElapsedValue) + 'a>;
    impl core::convert::TryFrom<u16> for ClockElapsedValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Clock Elapsed Time Knob
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ClockElapsed(Selector<17506, 16384, 14>);
    impl ClockElapsed {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ClockElapsed {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClockElapsed {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Elapsed Time Minutes
    /// Output: gauge position
    pub struct ClockElapsedTimeM(AnalogGauge<17660, 65535, 0>);
    impl ClockElapsedTimeM {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ClockElapsedTimeM {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClockElapsedTimeM {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Elapsed Time Seconds
    /// Output: gauge position
    pub struct ClockElapsedTimeSec(AnalogGauge<17662, 65535, 0>);
    impl ClockElapsedTimeSec {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ClockElapsedTimeSec {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClockElapsedTimeSec {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ClockSetValue {
        Select1,
        Select2,
    }
    pub type ClockSetValueCallback<'a> = alloc::boxed::Box<dyn FnMut(ClockSetValue) + 'a>;
    impl core::convert::TryFrom<u16> for ClockSetValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Clock Setting Knob
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ClockSet(Selector<17506, 8192, 13>);
    impl ClockSet {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ClockSet {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClockSet {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Clock Wind
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct ClockWind(AnalogDial<17520, 65535, 0>);
    impl ClockWind {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ClockWind {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClockWind {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod cmds {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Cmds01ExpCatSwValue {
        Select1,
        Select2,
    }
    pub type Cmds01ExpCatSwValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(Cmds01ExpCatSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for Cmds01ExpCatSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// CMDS O1 Expendable Category Switch, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Cmds01ExpCatSw(Selector<17498, 32, 5>);
    impl Cmds01ExpCatSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Cmds01ExpCatSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Cmds01ExpCatSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Cmds02ExpCatSwValue {
        Select1,
        Select2,
    }
    pub type Cmds02ExpCatSwValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(Cmds02ExpCatSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for Cmds02ExpCatSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// CMDS O2 Expendable Category Switch, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Cmds02ExpCatSw(Selector<17498, 64, 6>);
    impl Cmds02ExpCatSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Cmds02ExpCatSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Cmds02ExpCatSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// CMDS CH Amount Display
    /// Output: CMDS CH Amount Display
    pub struct CmdsChAmount(Display<17822, 4>);
    impl CmdsChAmount {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for CmdsChAmount {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CmdsChAmount {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CmdsChExpCatSwValue {
        Select1,
        Select2,
    }
    pub type CmdsChExpCatSwValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(CmdsChExpCatSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for CmdsChExpCatSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// CMDS CH Expendable Category Switch, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CmdsChExpCatSw(Selector<17498, 128, 7>);
    impl CmdsChExpCatSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CmdsChExpCatSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CmdsChExpCatSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CmdsDispenseBtnValue {
        Select1,
        Select2,
    }
    pub type CmdsDispenseBtnValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(CmdsDispenseBtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for CmdsDispenseBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// CHAFF/FLARE Dispense Button - Push to dispense
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CmdsDispenseBtn(Selector<17498, 1, 0>);
    impl CmdsDispenseBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CmdsDispenseBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CmdsDispenseBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// CMDS FL Amount Display
    /// Output: CMDS FL Amount Display
    pub struct CmdsFlAmount(Display<17826, 4>);
    impl CmdsFlAmount {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for CmdsFlAmount {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CmdsFlAmount {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CmdsFlExpCatSwValue {
        Select1,
        Select2,
    }
    pub type CmdsFlExpCatSwValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(CmdsFlExpCatSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for CmdsFlExpCatSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// CMDS FL Expendable Category Switch, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CmdsFlExpCatSw(Selector<17498, 256, 8>);
    impl CmdsFlExpCatSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CmdsFlExpCatSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CmdsFlExpCatSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CmdsJettSwValue {
        Select1,
        Select2,
    }
    pub type CmdsJettSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CmdsJettSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for CmdsJettSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Jettison Switch, JETT/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CmdsJettSw(Selector<17498, 16, 4>);
    impl CmdsJettSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CmdsJettSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CmdsJettSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CmdsJmrSourcheSwValue {
        Select1,
        Select2,
    }
    pub type CmdsJmrSourcheSwValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(CmdsJmrSourcheSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for CmdsJmrSourcheSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// CMDS JMR Source Switch, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CmdsJmrSourcheSw(Selector<17498, 4, 2>);
    impl CmdsJmrSourcheSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CmdsJmrSourcheSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CmdsJmrSourcheSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CmdsModeKnbValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
        Select6,
    }
    pub type CmdsModeKnbValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CmdsModeKnbValue) + 'a>;
    impl core::convert::TryFrom<u16> for CmdsModeKnbValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                3 => Ok(Self::Select4),
                4 => Ok(Self::Select5),
                5 => Ok(Self::Select6),
                value => Err(value),
            }
        }
    }

    /// CMDS MODE Knob, OFF/STBY/MAN/SEMI/AUTO/BYP
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct CmdsModeKnb(Selector<17498, 28672, 12>);
    impl CmdsModeKnb {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CmdsModeKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CmdsModeKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CmdsMwsSourcheSwValue {
        Select1,
        Select2,
    }
    pub type CmdsMwsSourcheSwValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(CmdsMwsSourcheSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for CmdsMwsSourcheSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// CMDS MWS Source Switch, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CmdsMwsSourcheSw(Selector<17498, 8, 3>);
    impl CmdsMwsSourcheSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CmdsMwsSourcheSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CmdsMwsSourcheSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// CMDS O1 Amount Display
    /// Output: CMDS O1 Amount Display
    pub struct CmdsO1Amount(Display<17814, 4>);
    impl CmdsO1Amount {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for CmdsO1Amount {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CmdsO1Amount {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// CMDS O2 Amount Display
    /// Output: CMDS O2 Amount Display
    pub struct CmdsO2Amount(Display<17818, 4>);
    impl CmdsO2Amount {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for CmdsO2Amount {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CmdsO2Amount {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CmdsProgKnbValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
    }
    pub type CmdsProgKnbValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CmdsProgKnbValue) + 'a>;
    impl core::convert::TryFrom<u16> for CmdsProgKnbValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                3 => Ok(Self::Select4),
                4 => Ok(Self::Select5),
                value => Err(value),
            }
        }
    }

    /// CMDS PROGRAM Knob, BIT/1/2/3/4
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct CmdsProgKnb(Selector<17498, 3584, 9>);
    impl CmdsProgKnb {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CmdsProgKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CmdsProgKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CmdsPwrSourcheSwValue {
        Select1,
        Select2,
    }
    pub type CmdsPwrSourcheSwValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(CmdsPwrSourcheSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for CmdsPwrSourcheSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// CMDS RWR Source Switch, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CmdsPwrSourcheSw(Selector<17498, 2, 1>);
    impl CmdsPwrSourcheSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CmdsPwrSourcheSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CmdsPwrSourcheSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod cockpitmechanics {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CanopyHandleValue {
        Select1,
        Select2,
    }
    pub type CanopyHandleValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CanopyHandleValue) + 'a>;
    impl core::convert::TryFrom<u16> for CanopyHandleValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Canopy Handle, UP/DOWN
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CanopyHandle(Selector<17506, 32768, 15>);
    impl CanopyHandle {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CanopyHandle {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CanopyHandle {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CanopyJettThandleValue {
        Select1,
        Select2,
    }
    pub type CanopyJettThandleValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(CanopyJettThandleValue) + 'a>;
    impl core::convert::TryFrom<u16> for CanopyJettThandleValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// CANOPY JETTISON T-Handle, PULL/STOW
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CanopyJettThandle(Selector<17522, 4, 2>);
    impl CanopyJettThandle {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CanopyJettThandle {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CanopyJettThandle {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Canopy Switch, OPEN/HOLD/CLOSE
    /// Input: set the switch position
    /// Output: switch position -- 0 = Down, 1 = Mid ,  2 = UP
    pub struct CanopySw(Selector<17522, 48, 4>);
    impl CanopySw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CanopySw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CanopySw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Canopy Position
    /// Output: gauge position
    pub struct CanopyValue(AnalogGauge<17540, 65535, 0>);
    impl CanopyValue {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CanopyValue {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CanopyValue {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum HideStickValue {
        Select1,
        Select2,
    }
    pub type HideStickValueCallback<'a> = alloc::boxed::Box<dyn FnMut(HideStickValue) + 'a>;
    impl core::convert::TryFrom<u16> for HideStickValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Hide Stick toggle
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct HideStick(Selector<17522, 64, 6>);
    impl HideStick {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HideStick {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HideStick {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// SEAT ADJ Switch, UP/OFF/DOWN
    /// Input: set the switch position
    /// Output: switch position -- 0 = Down, 1 = Mid ,  2 = UP
    pub struct SeatAdj(Selector<17522, 3, 0>);
    impl SeatAdj {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SeatAdj {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SeatAdj {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SeatEjectHandleValue {
        Select1,
        Select2,
    }
    pub type SeatEjectHandleValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(SeatEjectHandleValue) + 'a>;
    impl core::convert::TryFrom<u16> for SeatEjectHandleValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Ejection Handle, PULL/STOW
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct SeatEjectHandle(Selector<17538, 1024, 10>);
    impl SeatEjectHandle {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SeatEjectHandle {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SeatEjectHandle {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SeatEjectSafeValue {
        Select1,
        Select2,
    }
    pub type SeatEjectSafeValueCallback<'a> = alloc::boxed::Box<dyn FnMut(SeatEjectSafeValue) + 'a>;
    impl core::convert::TryFrom<u16> for SeatEjectSafeValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Ejection Safety Lever, ARMED/LOCKED
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct SeatEjectSafe(Selector<17522, 8, 3>);
    impl SeatEjectSafe {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SeatEjectSafe {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SeatEjectSafe {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Seat Height
    /// Output: gauge position
    pub struct SeatHeight(AnalogGauge<17542, 65535, 0>);
    impl SeatHeight {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SeatHeight {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SeatHeight {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod controlinterface {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AdvModeSwValue {
        Select1,
        Select2,
    }
    pub type AdvModeSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(AdvModeSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for AdvModeSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ADV MODE Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AdvModeSw(Selector<17408, 4096, 12>);
    impl AdvModeSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdvModeSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdvModeSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AltFlapsSwValue {
        Select1,
        Select2,
    }
    pub type AltFlapsSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(AltFlapsSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for AltFlapsSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ALT FLAPS Switch, NORM/EXTEND
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AltFlapsSw(Selector<17408, 2, 1>);
    impl AltFlapsSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AltFlapsSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AltFlapsSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Autopilot PITCH Switch, ATT HOLD/ A/P OFF/ ALT HOLD
    /// Input: set the switch position
    /// Output: switch position -- 0 = Down, 1 = Mid ,  2 = UP
    pub struct ApPitchSw(Selector<17408, 768, 8>);
    impl ApPitchSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ApPitchSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ApPitchSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ApRollSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type ApRollSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(ApRollSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for ApRollSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// Autopilot ROLL Switch, STRG SEL/ATT HOLD/HDG SEL
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct ApRollSw(Selector<17408, 3072, 10>);
    impl ApRollSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ApRollSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ApRollSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum BitSwValue {
        Select1,
        Select2,
    }
    pub type BitSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(BitSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for BitSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// BIT Switch, OFF/BIT
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct BitSw(Selector<17408, 4, 2>);
    impl BitSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for BitSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for BitSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DigiBakSwValue {
        Select1,
        Select2,
    }
    pub type DigiBakSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(DigiBakSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for DigiBakSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// DIGITAL BACKUP Switch, OFF/BACKUP
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct DigiBakSw(Selector<17408, 1, 0>);
    impl DigiBakSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for DigiBakSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for DigiBakSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FlcsResetSwValue {
        Select1,
        Select2,
    }
    pub type FlcsResetSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(FlcsResetSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for FlcsResetSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// FLCS RESET Switch, OFF/RESET
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FlcsResetSw(Selector<17408, 8, 3>);
    impl FlcsResetSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FlcsResetSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FlcsResetSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LeFlapsSwValue {
        Select1,
        Select2,
    }
    pub type LeFlapsSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(LeFlapsSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for LeFlapsSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// LE FLAPS Switch, AUTO/LOCK
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LeFlapsSw(Selector<17408, 16, 4>);
    impl LeFlapsSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LeFlapsSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LeFlapsSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ManTfFlyupSwValue {
        Select1,
        Select2,
    }
    pub type ManTfFlyupSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(ManTfFlyupSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for ManTfFlyupSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MANUAL TF FLYUP Switch, ENABLE/DISABLE
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ManTfFlyupSw(Selector<17408, 8192, 13>);
    impl ManTfFlyupSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ManTfFlyupSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ManTfFlyupSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ManualPitchSwValue {
        Select1,
        Select2,
    }
    pub type ManualPitchSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(ManualPitchSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for ManualPitchSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MANUAL PITCH Override Switch, OVRD/NORM
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ManualPitchSw(Selector<17408, 64, 6>);
    impl ManualPitchSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ManualPitchSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ManualPitchSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// PITCH TRIM Wheel
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct PitchTrim(LimitedDial<17412, 65535, 0>);
    impl PitchTrim {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for PitchTrim {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for PitchTrim {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ROLL TRIM Wheel
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct RollTrim(LimitedDial<17410, 65535, 0>);
    impl RollTrim {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RollTrim {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RollTrim {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum StoresConfigSwValue {
        Select1,
        Select2,
    }
    pub type StoresConfigSwValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(StoresConfigSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for StoresConfigSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// STORES CONFIG Switch, CAT III/CAT I
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct StoresConfigSw(Selector<17408, 128, 7>);
    impl StoresConfigSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for StoresConfigSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for StoresConfigSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum TrimApDiscSwValue {
        Select1,
        Select2,
    }
    pub type TrimApDiscSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(TrimApDiscSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for TrimApDiscSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// TRIM/AP DISC Switch, DISC/NORM
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct TrimApDiscSw(Selector<17408, 32, 5>);
    impl TrimApDiscSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TrimApDiscSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TrimApDiscSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// YAW TRIM Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct YawTrim(LimitedDial<17414, 65535, 0>);
    impl YawTrim {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for YawTrim {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for YawTrim {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod dedoutputdata {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// DED Display Line 1
    /// Output: DED Display Line 1
    pub struct DedLine1(Display<17664, 29>);
    impl DedLine1 {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for DedLine1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for DedLine1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// DED Display Line 2
    /// Output: DED Display Line 2
    pub struct DedLine2(Display<17694, 29>);
    impl DedLine2 {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for DedLine2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for DedLine2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// DED Display Line 3
    /// Output: DED Display Line 3
    pub struct DedLine3(Display<17724, 29>);
    impl DedLine3 {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for DedLine3 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for DedLine3 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// DED Display Line 4
    /// Output: DED Display Line 4
    pub struct DedLine4(Display<17754, 29>);
    impl DedLine4 {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for DedLine4 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for DedLine4 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// DED Display Line 5
    /// Output: DED Display Line 5
    pub struct DedLine5(Display<17784, 29>);
    impl DedLine5 {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for DedLine5 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for DedLine5 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod ecm {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Ecm1BtnValue {
        Select1,
        Select2,
    }
    pub type Ecm1BtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Ecm1BtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for Ecm1BtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ECM 1 Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Ecm1Btn(Selector<17692, 512, 9>);
    impl Ecm1Btn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Ecm1Btn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Ecm1Btn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Ecm2BtnValue {
        Select1,
        Select2,
    }
    pub type Ecm2BtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Ecm2BtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for Ecm2BtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ECM 2 Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Ecm2Btn(Selector<17692, 1024, 10>);
    impl Ecm2Btn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Ecm2Btn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Ecm2Btn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Ecm3BtnValue {
        Select1,
        Select2,
    }
    pub type Ecm3BtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Ecm3BtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for Ecm3BtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ECM 3 Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Ecm3Btn(Selector<17692, 2048, 11>);
    impl Ecm3Btn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Ecm3Btn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Ecm3Btn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Ecm4BtnValue {
        Select1,
        Select2,
    }
    pub type Ecm4BtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Ecm4BtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for Ecm4BtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ECM 4 Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Ecm4Btn(Selector<17692, 4096, 12>);
    impl Ecm4Btn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Ecm4Btn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Ecm4Btn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Ecm5BtnValue {
        Select1,
        Select2,
    }
    pub type Ecm5BtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Ecm5BtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for Ecm5BtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ECM 5 Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Ecm5Btn(Selector<17692, 8192, 13>);
    impl Ecm5Btn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Ecm5Btn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Ecm5Btn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Ecm6BtnValue {
        Select1,
        Select2,
    }
    pub type Ecm6BtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Ecm6BtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for Ecm6BtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ECM 6 Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Ecm6Btn(Selector<17692, 16384, 14>);
    impl Ecm6Btn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Ecm6Btn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Ecm6Btn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EcmBitBtnValue {
        Select1,
        Select2,
    }
    pub type EcmBitBtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(EcmBitBtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for EcmBitBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ECM Bit Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct EcmBitBtn(Selector<17692, 256, 8>);
    impl EcmBitBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EcmBitBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EcmBitBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EcmFrmBtnValue {
        Select1,
        Select2,
    }
    pub type EcmFrmBtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(EcmFrmBtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for EcmFrmBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ECM FRM Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct EcmFrmBtn(Selector<17692, 32768, 15>);
    impl EcmFrmBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EcmFrmBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EcmFrmBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EcmPwSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type EcmPwSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(EcmPwSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for EcmPwSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// ECM Power Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct EcmPwSw(Selector<17538, 6144, 11>);
    impl EcmPwSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EcmPwSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EcmPwSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EcmResetBtnValue {
        Select1,
        Select2,
    }
    pub type EcmResetBtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(EcmResetBtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for EcmResetBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ECM Reset Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct EcmResetBtn(Selector<17538, 32768, 15>);
    impl EcmResetBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EcmResetBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EcmResetBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EcmSplBtnValue {
        Select1,
        Select2,
    }
    pub type EcmSplBtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(EcmSplBtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for EcmSplBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ECM SPL Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct EcmSplBtn(Selector<17722, 256, 8>);
    impl EcmSplBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EcmSplBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EcmSplBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EcmXmitSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type EcmXmitSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(EcmXmitSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for EcmXmitSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// ECM XMIT Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct EcmXmitSw(Selector<17538, 24576, 13>);
    impl EcmXmitSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EcmXmitSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EcmXmitSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod ecs {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AirSourceKnbValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
    }
    pub type AirSourceKnbValueCallback<'a> = alloc::boxed::Box<dyn FnMut(AirSourceKnbValue) + 'a>;
    impl core::convert::TryFrom<u16> for AirSourceKnbValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                3 => Ok(Self::Select4),
                4 => Ok(Self::Select5),
                value => Err(value),
            }
        }
    }

    /// AIR SOURCE Knob, OFF/NORM/DUMP/RAM
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct AirSourceKnb(Selector<17438, 28672, 12>);
    impl AirSourceKnb {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AirSourceKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AirSourceKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DefogLeverValue {
        Select1,
        Select2,
    }
    pub type DefogLeverValueCallback<'a> = alloc::boxed::Box<dyn FnMut(DefogLeverValue) + 'a>;
    impl core::convert::TryFrom<u16> for DefogLeverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// DEFOG Lever
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct DefogLever(Selector<17438, 32768, 15>);
    impl DefogLever {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for DefogLever {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for DefogLever {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum TempKnbValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
        Select6,
        Select7,
    }
    pub type TempKnbValueCallback<'a> = alloc::boxed::Box<dyn FnMut(TempKnbValue) + 'a>;
    impl core::convert::TryFrom<u16> for TempKnbValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                3 => Ok(Self::Select4),
                4 => Ok(Self::Select5),
                5 => Ok(Self::Select6),
                6 => Ok(Self::Select7),
                value => Err(value),
            }
        }
    }

    /// TEMP Knob
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct TempKnb(Selector<17438, 3584, 9>);
    impl TempKnb {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TempKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TempKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod ehsi {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EhsiCrsSetValue {
        Select1,
        Select2,
    }
    pub type EhsiCrsSetValueCallback<'a> = alloc::boxed::Box<dyn FnMut(EhsiCrsSetValue) + 'a>;
    impl core::convert::TryFrom<u16> for EhsiCrsSetValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// EHSI CRS Set
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct EhsiCrsSet(Selector<17506, 1024, 10>);
    impl EhsiCrsSet {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EhsiCrsSet {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EhsiCrsSet {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// EHSI CRS Set Knob
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct EhsiCrsSetKnb(AnalogDial<17516, 65535, 0>);
    impl EhsiCrsSetKnb {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EhsiCrsSetKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EhsiCrsSetKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EhsiHdgSetBtnValue {
        Select1,
        Select2,
    }
    pub type EhsiHdgSetBtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(EhsiHdgSetBtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for EhsiHdgSetBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// EHSI HDG Set Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct EhsiHdgSetBtn(Selector<17506, 2048, 11>);
    impl EhsiHdgSetBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EhsiHdgSetBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EhsiHdgSetBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// EHSI HDG Set Knob
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct EhsiHdgSetKnb(AnalogDial<17518, 65535, 0>);
    impl EhsiHdgSetKnb {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EhsiHdgSetKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EhsiHdgSetKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EhsiModeValue {
        Select1,
        Select2,
    }
    pub type EhsiModeValueCallback<'a> = alloc::boxed::Box<dyn FnMut(EhsiModeValue) + 'a>;
    impl core::convert::TryFrom<u16> for EhsiModeValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// EHSI Mode (M) Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct EhsiMode(Selector<17506, 4096, 12>);
    impl EhsiMode {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EhsiMode {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EhsiMode {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod electricsystem {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ElecCautionValue {
        Select1,
        Select2,
    }
    pub type ElecCautionValueCallback<'a> = alloc::boxed::Box<dyn FnMut(ElecCautionValue) + 'a>;
    impl core::convert::TryFrom<u16> for ElecCautionValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ELEC CAUTION RESET Button - Push to reset
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ElecCaution(Selector<17416, 32768, 15>);
    impl ElecCaution {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ElecCaution {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ElecCaution {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EpuGenTestSwValue {
        Select1,
        Select2,
    }
    pub type EpuGenTestSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(EpuGenTestSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for EpuGenTestSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// EPU/GEN Test Switch, EPU/GEN /OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct EpuGenTestSw(Selector<17436, 4, 2>);
    impl EpuGenTestSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EpuGenTestSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EpuGenTestSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FLCS PWR TEST Switch, MAINT/NORM/TEST
    /// Input: set the switch position
    /// Output: switch position -- 0 = Down, 1 = Mid ,  2 = UP
    pub struct FlcsPwrTestSw(Selector<17436, 96, 5>);
    impl FlcsPwrTestSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FlcsPwrTestSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FlcsPwrTestSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MainPwrSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type MainPwrSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MainPwrSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for MainPwrSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// MAIN PWR Switch, MAIN PWR/BATT/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct MainPwrSw(Selector<17436, 3, 0>);
    impl MainPwrSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MainPwrSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MainPwrSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// PROBE HEAT Switch, PROBE HEAT/OFF/TEST
    /// Input: set the switch position
    /// Output: switch position -- 0 = Down, 1 = Mid ,  2 = UP
    pub struct ProbeHeatSw(Selector<17436, 24, 3>);
    impl ProbeHeatSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ProbeHeatSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ProbeHeatSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod engine {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// AB RESET Switch, AB RESET/NORM/ENG DATA
    /// Input: set the switch position
    /// Output: switch position -- 0 = Down, 1 = Mid ,  2 = UP
    pub struct AbResetSw(Selector<17440, 6144, 11>);
    impl AbResetSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AbResetSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AbResetSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EngAntiIceValue {
        Select1,
        Select2,
        Select3,
    }
    pub type EngAntiIceValueCallback<'a> = alloc::boxed::Box<dyn FnMut(EngAntiIceValue) + 'a>;
    impl core::convert::TryFrom<u16> for EngAntiIceValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// Engine ANTI ICE Switch, ON/AUTO/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct EngAntiIce(Selector<17440, 48, 4>);
    impl EngAntiIce {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EngAntiIce {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EngAntiIce {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EngContSwValue {
        Select1,
        Select2,
    }
    pub type EngContSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(EngContSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for EngContSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ENG CONT Switch, PRI/SEC
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct EngContSw(Selector<17440, 512, 9>);
    impl EngContSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EngContSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EngContSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EngContSwCoverValue {
        Select1,
        Select2,
    }
    pub type EngContSwCoverValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(EngContSwCoverValue) + 'a>;
    impl core::convert::TryFrom<u16> for EngContSwCoverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ENG CONT Switch Cover, OPEN/CLOSE
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct EngContSwCover(Selector<17440, 256, 8>);
    impl EngContSwCover {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EngContSwCover {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EngContSwCover {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Engine FTIT Indicator
    /// Output: gauge position
    pub struct EngineFtit(AnalogGauge<17622, 65535, 0>);
    impl EngineFtit {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EngineFtit {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EngineFtit {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Engine Nozzle Position Indicator
    /// Output: gauge position
    pub struct EngineNozzlePosition(AnalogGauge<17618, 65535, 0>);
    impl EngineNozzlePosition {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EngineNozzlePosition {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EngineNozzlePosition {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Oil Pressure Indicator
    /// Output: gauge position
    pub struct EngineOilPressure(AnalogGauge<17616, 65535, 0>);
    impl EngineOilPressure {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EngineOilPressure {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EngineOilPressure {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Engine Tachometer Indicator
    /// Output: gauge position
    pub struct EngineTachometer(AnalogGauge<17620, 65535, 0>);
    impl EngineTachometer {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EngineTachometer {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EngineTachometer {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FireOheatDetectBtnValue {
        Select1,
        Select2,
    }
    pub type FireOheatDetectBtnValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(FireOheatDetectBtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for FireOheatDetectBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// FIRE & OHEAT DETECT Test Button - Push to test
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FireOheatDetectBtn(Selector<17440, 8192, 13>);
    impl FireOheatDetectBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FireOheatDetectBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FireOheatDetectBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// JFS Switch, START 1/OFF/START 2
    /// Input: set the switch position
    /// Output: switch position -- 0 = Down, 1 = Mid ,  2 = UP
    pub struct JfsSw(Selector<17440, 192, 6>);
    impl JfsSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for JfsSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for JfsSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MaxPwrSwValue {
        Select1,
        Select2,
    }
    pub type MaxPwrSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MaxPwrSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for MaxPwrSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MAX POWER Switch, MAX POWER/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MaxPwrSw(Selector<17440, 1024, 10>);
    impl MaxPwrSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MaxPwrSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MaxPwrSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod epu {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EpuSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type EpuSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(EpuSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for EpuSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// EPU Switch, ON/NORM/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct EpuSw(Selector<17440, 12, 2>);
    impl EpuSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EpuSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EpuSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EpuSwCoverOffValue {
        Select1,
        Select2,
    }
    pub type EpuSwCoverOffValueCallback<'a> = alloc::boxed::Box<dyn FnMut(EpuSwCoverOffValue) + 'a>;
    impl core::convert::TryFrom<u16> for EpuSwCoverOffValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// EPU Switch Cover for OFF, OPEN/CLOSE
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct EpuSwCoverOff(Selector<17440, 2, 1>);
    impl EpuSwCoverOff {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EpuSwCoverOff {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EpuSwCoverOff {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EpuSwCoverOnValue {
        Select1,
        Select2,
    }
    pub type EpuSwCoverOnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(EpuSwCoverOnValue) + 'a>;
    impl core::convert::TryFrom<u16> for EpuSwCoverOnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// EPU Switch Cover for ON, OPEN/CLOSE
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct EpuSwCoverOn(Selector<17440, 1, 0>);
    impl EpuSwCoverOn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EpuSwCoverOn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EpuSwCoverOn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Hydrazin Volume Indicator
    /// Output: gauge position
    pub struct HydrazinVolume(AnalogGauge<17624, 65535, 0>);
    impl HydrazinVolume {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HydrazinVolume {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HydrazinVolume {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod externalaircraftmodel {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Formation Lights
    /// Output: Formation Lights
    pub struct ExtFormationLights(Metadata<17848, 65535, 0>);
    impl ExtFormationLights {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ExtFormationLights {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ExtFormationLights {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Fuselage Position Lights (L-red; R-green;Back-white)
    /// Output: Fuselage Position Lights (L-red; R-green;Back-white)
    pub struct ExtPositionLightFuselage(Metadata<17722, 1024, 10>);
    impl ExtPositionLightFuselage {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ExtPositionLightFuselage {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ExtPositionLightFuselage {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Wing Position Lights (L-red; R-green)
    /// Output: Wing Position Lights (L-red; R-green)
    pub struct ExtPositionLightsWing(Metadata<17722, 512, 9>);
    impl ExtPositionLightsWing {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ExtPositionLightsWing {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ExtPositionLightsWing {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Left Speed Brake
    /// Output: Left Speed Brake
    pub struct ExtSpeedBrakeLeft(Metadata<17846, 65535, 0>);
    impl ExtSpeedBrakeLeft {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ExtSpeedBrakeLeft {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ExtSpeedBrakeLeft {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Right Speed Brake
    /// Output: Right Speed Brake
    pub struct ExtSpeedBrakeRight(Metadata<17844, 65535, 0>);
    impl ExtSpeedBrakeRight {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ExtSpeedBrakeRight {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ExtSpeedBrakeRight {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Tail Strobe Light
    /// Output: Tail Strobe Light
    pub struct ExtStrobeTail(Metadata<17722, 2048, 11>);
    impl ExtStrobeTail {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ExtStrobeTail {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ExtStrobeTail {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Tail Lights
    /// Output: Tail Lights
    pub struct ExtTailLight(Metadata<17850, 65535, 0>);
    impl ExtTailLight {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ExtTailLight {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ExtTailLight {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Weight ON Wheels Left Gear
    /// Output: Weight ON Wheels Left Gear
    pub struct ExtWowLeft(Metadata<17722, 16384, 14>);
    impl ExtWowLeft {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ExtWowLeft {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ExtWowLeft {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Weight ON Wheels Nose Gear
    /// Output: Weight ON Wheels Nose Gear
    pub struct ExtWowNose(Metadata<17722, 4096, 12>);
    impl ExtWowNose {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ExtWowNose {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ExtWowNose {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Weight ON Wheels Right Gear
    /// Output: Weight ON Wheels Right Gear
    pub struct ExtWowRight(Metadata<17722, 8192, 13>);
    impl ExtWowRight {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ExtWowRight {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ExtWowRight {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod externallights {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// AERIAL REFUELING Light Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct AirRefuelLightKnb(LimitedDial<17420, 65535, 0>);
    impl AirRefuelLightKnb {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AirRefuelLightKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AirRefuelLightKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AntiCollLightKnbValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
        Select6,
        Select7,
        Select8,
    }
    pub type AntiCollLightKnbValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(AntiCollLightKnbValue) + 'a>;
    impl core::convert::TryFrom<u16> for AntiCollLightKnbValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                3 => Ok(Self::Select4),
                4 => Ok(Self::Select5),
                5 => Ok(Self::Select6),
                6 => Ok(Self::Select7),
                7 => Ok(Self::Select8),
                value => Err(value),
            }
        }
    }

    /// ANTI-COLL Knob, OFF/1/2/3/4/A/B/C
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct AntiCollLightKnb(Selector<17416, 7, 0>);
    impl AntiCollLightKnb {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AntiCollLightKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AntiCollLightKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EngineFeedKnbValue {
        Select1,
        Select2,
        Select3,
        Select4,
    }
    pub type EngineFeedKnbValueCallback<'a> = alloc::boxed::Box<dyn FnMut(EngineFeedKnbValue) + 'a>;
    impl core::convert::TryFrom<u16> for EngineFeedKnbValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                3 => Ok(Self::Select4),
                value => Err(value),
            }
        }
    }

    /// ENGINE FEED Knob, OFF/NORM/AFT/FWD
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct EngineFeedKnb(Selector<17436, 3072, 10>);
    impl EngineFeedKnb {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EngineFeedKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EngineFeedKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FORM Light Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct FormLightKnb(LimitedDial<17418, 65535, 0>);
    impl FormLightKnb {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FormLightKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FormLightKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LandTaxiLightSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type LandTaxiLightSwValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(LandTaxiLightSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for LandTaxiLightSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// LANDING TAXI LIGHTS Switch, LANDING/OFF/TAXI
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct LandTaxiLightSw(Selector<17416, 3072, 10>);
    impl LandTaxiLightSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LandTaxiLightSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LandTaxiLightSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// MAL & IND LTS Switch, BRT/Center/DIM
    /// Input: set the switch position
    /// Output: switch position -- 0 = Down, 1 = Mid ,  2 = UP
    pub struct MalIndLtsBrtSw(Selector<17416, 24576, 13>);
    impl MalIndLtsBrtSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MalIndLtsBrtSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MalIndLtsBrtSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MasterLightSwValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
    }
    pub type MasterLightSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MasterLightSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for MasterLightSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                3 => Ok(Self::Select4),
                4 => Ok(Self::Select5),
                value => Err(value),
            }
        }
    }

    /// MASTER Light Switch, OFF/ALL/A-C/FORM/NORM
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct MasterLightSw(Selector<17416, 896, 7>);
    impl MasterLightSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MasterLightSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MasterLightSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum PosFlashLightSwValue {
        Select1,
        Select2,
    }
    pub type PosFlashLightSwValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(PosFlashLightSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for PosFlashLightSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// FLASH STEADY Light Switch, FLASH/STEADY
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct PosFlashLightSw(Selector<17408, 16384, 14>);
    impl PosFlashLightSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for PosFlashLightSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for PosFlashLightSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum PosFuselageLightSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type PosFuselageLightSwValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(PosFuselageLightSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for PosFuselageLightSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// FUSELAGE Light Switch, BRT/OFF/DIM
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct PosFuselageLightSw(Selector<17416, 96, 5>);
    impl PosFuselageLightSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for PosFuselageLightSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for PosFuselageLightSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum PosWingTailLightSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type PosWingTailLightSwValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(PosWingTailLightSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for PosWingTailLightSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// WING/TAIL Light Switch, BRT/OFF/DIM
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct PosWingTailLightSw(Selector<17416, 24, 3>);
    impl PosWingTailLightSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for PosWingTailLightSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for PosWingTailLightSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod fuelsystem {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AirRefuelSwValue {
        Select1,
        Select2,
    }
    pub type AirRefuelSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(AirRefuelSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for AirRefuelSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// AIR REFUEL Switch, OPEN/CLOSE
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AirRefuelSw(Selector<17436, 4096, 12>);
    impl AirRefuelSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AirRefuelSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AirRefuelSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ExtFuelTransSwValue {
        Select1,
        Select2,
    }
    pub type ExtFuelTransSwValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(ExtFuelTransSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for ExtFuelTransSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// External Fuel Transfer Switch, NORM/ WING FIRST
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ExtFuelTransSw(Selector<17436, 8192, 13>);
    impl ExtFuelTransSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ExtFuelTransSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ExtFuelTransSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Fuel Quantity Indicator AL
    /// Output: gauge position
    pub struct FuelAl(AnalogGauge<17632, 65535, 0>);
    impl FuelAl {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FuelAl {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FuelAl {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Fuel Quantity Indicator FR
    /// Output: gauge position
    pub struct FuelFr(AnalogGauge<17634, 65535, 0>);
    impl FuelFr {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FuelFr {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FuelFr {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FuelMasterCvValue {
        Select1,
        Select2,
    }
    pub type FuelMasterCvValueCallback<'a> = alloc::boxed::Box<dyn FnMut(FuelMasterCvValue) + 'a>;
    impl core::convert::TryFrom<u16> for FuelMasterCvValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// FUEL MASTER Switch Cover, OPEN/CLOSE
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FuelMasterCv(Selector<17436, 256, 8>);
    impl FuelMasterCv {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FuelMasterCv {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FuelMasterCv {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FuelMasterSwValue {
        Select1,
        Select2,
    }
    pub type FuelMasterSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(FuelMasterSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for FuelMasterSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// FUEL MASTER Switch, MASTER/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FuelMasterSw(Selector<17436, 128, 7>);
    impl FuelMasterSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FuelMasterSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FuelMasterSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FuelQtySelKnbValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
    }
    pub type FuelQtySelKnbValueCallback<'a> = alloc::boxed::Box<dyn FnMut(FuelQtySelKnbValue) + 'a>;
    impl core::convert::TryFrom<u16> for FuelQtySelKnbValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                3 => Ok(Self::Select4),
                4 => Ok(Self::Select5),
                value => Err(value),
            }
        }
    }

    /// FUEL QTY SEL Knob, NORM/RSVR/INT WING/EXT WING/EXT CTR
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct FuelQtySelKnb(Selector<17438, 7, 0>);
    impl FuelQtySelKnb {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FuelQtySelKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FuelQtySelKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FUEL QTY SEL Knob, TEST
    /// Input: switch to previous or next state
    /// Output: selector position
    pub struct FuelQtySelTKnb(Selector<17436, 16384, 14>);
    impl FuelQtySelTKnb {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FuelQtySelTKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FuelQtySelTKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Fuel Flow Counter 100
    /// Output: gauge position
    pub struct Fuelflowcounter100(AnalogGauge<17630, 65535, 0>);
    impl Fuelflowcounter100 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Fuelflowcounter100 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Fuelflowcounter100 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Fuel Flow Counter 10k
    /// Output: gauge position
    pub struct Fuelflowcounter10k(AnalogGauge<17626, 65535, 0>);
    impl Fuelflowcounter10k {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Fuelflowcounter10k {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Fuelflowcounter10k {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Fuel Flow Counter 1k
    /// Output: gauge position
    pub struct Fuelflowcounter1k(AnalogGauge<17628, 65535, 0>);
    impl Fuelflowcounter1k {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Fuelflowcounter1k {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Fuelflowcounter1k {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Fuel Totalizer Counter 100
    /// Output: gauge position
    pub struct Fueltotalizer100(AnalogGauge<17640, 65535, 0>);
    impl Fueltotalizer100 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Fueltotalizer100 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Fueltotalizer100 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Fuel Totalizer Counter 10K
    /// Output: gauge position
    pub struct Fueltotalizer10k(AnalogGauge<17636, 65535, 0>);
    impl Fueltotalizer10k {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Fueltotalizer10k {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Fueltotalizer10k {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Fuel Totalizer Counter 1K
    /// Output: gauge position
    pub struct Fueltotalizer1k(AnalogGauge<17638, 65535, 0>);
    impl Fueltotalizer1k {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Fueltotalizer1k {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Fueltotalizer1k {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum TankIntertingSwValue {
        Select1,
        Select2,
    }
    pub type TankIntertingSwValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(TankIntertingSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for TankIntertingSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// TANK INERTING Switch, TANK INERTING /OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct TankIntertingSw(Selector<17436, 512, 9>);
    impl TankIntertingSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TankIntertingSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TankIntertingSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod gearsystem {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// ANTI-SKID Switch, PARKING BRAKE/ANTI-SKID/OFF
    /// Input: set the switch position
    /// Output: switch position -- 0 = Down, 1 = Mid ,  2 = UP
    pub struct AntiSkidSw(Selector<17438, 384, 7>);
    impl AntiSkidSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AntiSkidSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AntiSkidSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum BrakeChanSwValue {
        Select1,
        Select2,
    }
    pub type BrakeChanSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(BrakeChanSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for BrakeChanSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// BRAKES Channel Switch, CHAN 1/CHAN 2
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct BrakeChanSw(Selector<17438, 64, 6>);
    impl BrakeChanSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for BrakeChanSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for BrakeChanSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DnLockBtnValue {
        Select1,
        Select2,
    }
    pub type DnLockBtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(DnLockBtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for DnLockBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// DN LOCK REL Button - Push to reset
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct DnLockBtn(Selector<17438, 8, 3>);
    impl DnLockBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for DnLockBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for DnLockBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GearHandleValue {
        Select1,
        Select2,
    }
    pub type GearHandleValueCallback<'a> = alloc::boxed::Box<dyn FnMut(GearHandleValue) + 'a>;
    impl core::convert::TryFrom<u16> for GearHandleValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// LG Handle, UP/DN
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct GearHandle(Selector<17436, 32768, 15>);
    impl GearHandle {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for GearHandle {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for GearHandle {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum HookSwValue {
        Select1,
        Select2,
    }
    pub type HookSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(HookSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for HookSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// HOOK Switch, UP/DN
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct HookSw(Selector<17438, 16, 4>);
    impl HookSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HookSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HookSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum HornSilenceBtnValue {
        Select1,
        Select2,
    }
    pub type HornSilenceBtnValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(HornSilenceBtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for HornSilenceBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// HORN SILENCER Button - Push to reset
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct HornSilenceBtn(Selector<17438, 32, 5>);
    impl HornSilenceBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HornSilenceBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HornSilenceBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod hmcs {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// HMCS SYMBOLOGY INT Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct HmcsIntKnb(LimitedDial<17492, 65535, 0>);
    impl HmcsIntKnb {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HmcsIntKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HmcsIntKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod hudcontrolpanel {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum HudAltSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type HudAltSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(HudAltSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for HudAltSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// HUD Altitude Switch, RADAR / BARO / AUTO
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct HudAltSw(Selector<17458, 768, 8>);
    impl HudAltSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HudAltSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HudAltSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum HudBrtSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type HudBrtSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(HudBrtSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for HudBrtSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// HUD Brightness Control Switch, DAY / AUTO BRT / NIGHT
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct HudBrtSw(Selector<17458, 3072, 10>);
    impl HudBrtSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HudBrtSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HudBrtSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum HudDedDataSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type HudDedDataSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(HudDedDataSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for HudDedDataSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// HUD DED/PFLD Data Switch, DED / PFL / OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct HudDedDataSw(Selector<17458, 12, 2>);
    impl HudDedDataSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HudDedDataSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HudDedDataSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum HudDepressRetSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type HudDepressRetSwValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(HudDepressRetSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for HudDepressRetSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// HUD Depressible Reticle Switch, STBY / PRI / OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct HudDepressRetSw(Selector<17458, 48, 4>);
    impl HudDepressRetSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HudDepressRetSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HudDepressRetSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum HudFpMarkerSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type HudFpMarkerSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(HudFpMarkerSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for HudFpMarkerSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// HUD Flightpath Marker Switch, ATT/FPM / FPM / OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct HudFpMarkerSw(Selector<17458, 3, 0>);
    impl HudFpMarkerSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HudFpMarkerSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HudFpMarkerSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum HudScalesSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type HudScalesSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(HudScalesSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for HudScalesSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// HUD Scales Switch, VV/VAH / VAH / OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct HudScalesSw(Selector<17456, 49152, 14>);
    impl HudScalesSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HudScalesSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HudScalesSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum HudSpeedSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type HudSpeedSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(HudSpeedSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for HudSpeedSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// HUD Velocity Switch, CAS / TAS / GND SPD
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct HudSpeedSw(Selector<17458, 192, 6>);
    impl HudSpeedSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HudSpeedSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HudSpeedSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum HudTestSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type HudTestSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(HudTestSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for HudTestSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// HUD TEST Switch, STEP / ON / OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct HudTestSw(Selector<17458, 12288, 12>);
    impl HudTestSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HudTestSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HudTestSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod hydraulicpressureindicators {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// System A Pressure
    /// Output: gauge position
    pub struct SysaPressure(AnalogGauge<17612, 65535, 0>);
    impl SysaPressure {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SysaPressure {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SysaPressure {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// System B Pressure
    /// Output: gauge position
    pub struct SysbPressure(AnalogGauge<17614, 65535, 0>);
    impl SysbPressure {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SysbPressure {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SysbPressure {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod iff {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffCIKnbValue {
        Select1,
        Select2,
    }
    pub type IffCIKnbValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IffCIKnbValue) + 'a>;
    impl core::convert::TryFrom<u16> for IffCIKnbValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// IFF C & I Knob, UFC/BACKUP
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IffCIKnb(Selector<17482, 128, 7>);
    impl IffCIKnb {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IffCIKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffCIKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// IFF Code Digit 1
    /// Output: gauge position
    pub struct IffCodeDrumDigit1(AnalogGauge<17648, 65535, 0>);
    impl IffCodeDrumDigit1 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IffCodeDrumDigit1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffCodeDrumDigit1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// IFF Code Digit 2
    /// Output: gauge position
    pub struct IffCodeDrumDigit2(AnalogGauge<17650, 65535, 0>);
    impl IffCodeDrumDigit2 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IffCodeDrumDigit2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffCodeDrumDigit2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// IFF Code Digit 3
    /// Output: gauge position
    pub struct IffCodeDrumDigit3(AnalogGauge<17652, 65535, 0>);
    impl IffCodeDrumDigit3 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IffCodeDrumDigit3 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffCodeDrumDigit3 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// IFF Code Digit 4
    /// Output: gauge position
    pub struct IffCodeDrumDigit4(AnalogGauge<17654, 65535, 0>);
    impl IffCodeDrumDigit4 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IffCodeDrumDigit4 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffCodeDrumDigit4 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffEnableSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type IffEnableSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IffEnableSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for IffEnableSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// IFF ENABLE Switch, M1/M3 /OFF/ M3/MS
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct IffEnableSw(Selector<17486, 3, 0>);
    impl IffEnableSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IffEnableSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffEnableSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffM1Sel1Value {
        Select1,
        Select2,
        Select3,
    }
    pub type IffM1Sel1ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IffM1Sel1Value) + 'a>;
    impl core::convert::TryFrom<u16> for IffM1Sel1Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// IFF MODE 1 Selector Lever, DIGIT 1
    /// Input: switch to previous or next state
    /// Input: set the switch position -- 0 = held left/down, 1 = centered, 2 = held right/up
    /// Output: selector position
    pub struct IffM1Sel1(Selector<17486, 12, 2>);
    impl IffM1Sel1 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IffM1Sel1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffM1Sel1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffM1Sel2Value {
        Select1,
        Select2,
        Select3,
    }
    pub type IffM1Sel2ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IffM1Sel2Value) + 'a>;
    impl core::convert::TryFrom<u16> for IffM1Sel2Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// IFF MODE 1 Selector Lever, DIGIT 2
    /// Input: switch to previous or next state
    /// Input: set the switch position -- 0 = held left/down, 1 = centered, 2 = held right/up
    /// Output: selector position
    pub struct IffM1Sel2(Selector<17486, 48, 4>);
    impl IffM1Sel2 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IffM1Sel2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffM1Sel2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffM3Sel1Value {
        Select1,
        Select2,
        Select3,
    }
    pub type IffM3Sel1ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IffM3Sel1Value) + 'a>;
    impl core::convert::TryFrom<u16> for IffM3Sel1Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// IFF MODE 3 Selector Lever, DIGIT 1
    /// Input: switch to previous or next state
    /// Input: set the switch position -- 0 = held left/down, 1 = centered, 2 = held right/up
    /// Output: selector position
    pub struct IffM3Sel1(Selector<17486, 192, 6>);
    impl IffM3Sel1 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IffM3Sel1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffM3Sel1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffM3Sel2Value {
        Select1,
        Select2,
        Select3,
    }
    pub type IffM3Sel2ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IffM3Sel2Value) + 'a>;
    impl core::convert::TryFrom<u16> for IffM3Sel2Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// IFF MODE 3 Selector Lever, DIGIT 2
    /// Input: switch to previous or next state
    /// Input: set the switch position -- 0 = held left/down, 1 = centered, 2 = held right/up
    /// Output: selector position
    pub struct IffM3Sel2(Selector<17486, 768, 8>);
    impl IffM3Sel2 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IffM3Sel2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffM3Sel2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffM4CodeSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type IffM4CodeSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IffM4CodeSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for IffM4CodeSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// IFF M-4 CODE Switch, HOLD/ A/B /ZERO
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct IffM4CodeSw(Selector<17482, 6144, 11>);
    impl IffM4CodeSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IffM4CodeSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffM4CodeSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffM4MonitorSwValue {
        Select1,
        Select2,
    }
    pub type IffM4MonitorSwValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(IffM4MonitorSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for IffM4MonitorSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// IFF MODE 4 MONITOR Switch, OUT/AUDIO
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IffM4MonitorSw(Selector<17482, 32768, 15>);
    impl IffM4MonitorSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IffM4MonitorSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffM4MonitorSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffM4ReplySwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type IffM4ReplySwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IffM4ReplySwValue) + 'a>;
    impl core::convert::TryFrom<u16> for IffM4ReplySwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// IFF MODE 4 REPLY Switch, OUT/A/B
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct IffM4ReplySw(Selector<17482, 24576, 13>);
    impl IffM4ReplySw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IffM4ReplySw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffM4ReplySw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffMasterKnbValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
    }
    pub type IffMasterKnbValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IffMasterKnbValue) + 'a>;
    impl core::convert::TryFrom<u16> for IffMasterKnbValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                3 => Ok(Self::Select4),
                4 => Ok(Self::Select5),
                value => Err(value),
            }
        }
    }

    /// IFF MASTER Knob, OFF/STBY/LOW/NORM/EMER
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct IffMasterKnb(Selector<17482, 1792, 8>);
    impl IffMasterKnb {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IffMasterKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffMasterKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod interiorlights {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// AOA Indexer Dimming Lever
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct AoaIndexBrtKnb(LimitedDial<17432, 65535, 0>);
    impl AoaIndexBrtKnb {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AoaIndexBrtKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AoaIndexBrtKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// AR Status Indicator Dimming Lever
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct ArStatusBrtKnb(LimitedDial<17434, 65535, 0>);
    impl ArStatusBrtKnb {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ArStatusBrtKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ArStatusBrtKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FLOOD CONSOLES BRT Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct FloodConsolesBrtKnb(LimitedDial<17428, 65535, 0>);
    impl FloodConsolesBrtKnb {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FloodConsolesBrtKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FloodConsolesBrtKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FLOOD INST PNL BRT Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct FloodInstPnlBrtKnb(LimitedDial<17430, 65535, 0>);
    impl FloodInstPnlBrtKnb {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FloodInstPnlBrtKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FloodInstPnlBrtKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MalIndLtsTestValue {
        Select1,
        Select2,
    }
    pub type MalIndLtsTestValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MalIndLtsTestValue) + 'a>;
    impl core::convert::TryFrom<u16> for MalIndLtsTestValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MAL & IND LTS Test Button - Push to test
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MalIndLtsTest(Selector<17416, 4096, 12>);
    impl MalIndLtsTest {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MalIndLtsTest {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MalIndLtsTest {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MasterCautionValue {
        Select1,
        Select2,
    }
    pub type MasterCautionValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MasterCautionValue) + 'a>;
    impl core::convert::TryFrom<u16> for MasterCautionValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Master Caution Button - Push to reset
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MasterCaution(Selector<17408, 32768, 15>);
    impl MasterCaution {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MasterCaution {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MasterCaution {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// PRIMARY CONSOLES BRT Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct PriConsolesBrtKnb(LimitedDial<17422, 65535, 0>);
    impl PriConsolesBrtKnb {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for PriConsolesBrtKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for PriConsolesBrtKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// PRIMARY DATA ENTRY DISPLAY BRT Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct PriDataDisplayBrtKnb(LimitedDial<17426, 65535, 0>);
    impl PriDataDisplayBrtKnb {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for PriDataDisplayBrtKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for PriDataDisplayBrtKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// PRIMARY INST PNL BRT Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct PriInstPnlBrtKnb(LimitedDial<17424, 65535, 0>);
    impl PriInstPnlBrtKnb {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for PriInstPnlBrtKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for PriInstPnlBrtKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod ky58 {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Ky58FillKnbValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
        Select6,
        Select7,
        Select8,
    }
    pub type Ky58FillKnbValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Ky58FillKnbValue) + 'a>;
    impl core::convert::TryFrom<u16> for Ky58FillKnbValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                3 => Ok(Self::Select4),
                4 => Ok(Self::Select5),
                5 => Ok(Self::Select6),
                6 => Ok(Self::Select7),
                7 => Ok(Self::Select8),
                value => Err(value),
            }
        }
    }

    /// KY-58 FILL Knob, Z 1-5/1/2/3/4/5/6/Z ALL
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct Ky58FillKnb(Selector<17486, 28672, 12>);
    impl Ky58FillKnb {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Ky58FillKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Ky58FillKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Ky58ModeKnbValue {
        Select1,
        Select2,
        Select3,
        Select4,
    }
    pub type Ky58ModeKnbValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Ky58ModeKnbValue) + 'a>;
    impl core::convert::TryFrom<u16> for Ky58ModeKnbValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                3 => Ok(Self::Select4),
                value => Err(value),
            }
        }
    }

    /// KY-58 MODE Knob, P/C/LD/RV
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct Ky58ModeKnb(Selector<17486, 3072, 10>);
    impl Ky58ModeKnb {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Ky58ModeKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Ky58ModeKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Ky58PwrKnbValue {
        Select1,
        Select2,
        Select3,
    }
    pub type Ky58PwrKnbValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Ky58PwrKnbValue) + 'a>;
    impl core::convert::TryFrom<u16> for Ky58PwrKnbValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// KY-58 Power Knob, OFF/ON/TD
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct Ky58PwrKnb(Selector<17490, 3, 0>);
    impl Ky58PwrKnb {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Ky58PwrKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Ky58PwrKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// KY-58 VOLUME Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct Ky58VolKnb(LimitedDial<17488, 65535, 0>);
    impl Ky58VolKnb {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Ky58VolKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Ky58VolKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum PlainCipherSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type PlainCipherSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(PlainCipherSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for PlainCipherSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// PLAIN Cipher Switch, CRAD 1/PLAIN/CRAD 2
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct PlainCipherSw(Selector<17490, 12, 2>);
    impl PlainCipherSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for PlainCipherSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for PlainCipherSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ZeroizeSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type ZeroizeSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(ZeroizeSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for ZeroizeSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// ZEROIZE Switch, OFP/OFF/DATA
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct ZeroizeSw(Selector<17490, 48, 4>);
    impl ZeroizeSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ZeroizeSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ZeroizeSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ZeroizeSwCoverValue {
        Select1,
        Select2,
    }
    pub type ZeroizeSwCoverValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(ZeroizeSwCoverValue) + 'a>;
    impl core::convert::TryFrom<u16> for ZeroizeSwCoverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ZEROIZE Switch Cover, OPEN/CLOSE
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ZeroizeSwCover(Selector<17486, 32768, 15>);
    impl ZeroizeSwCover {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ZeroizeSwCover {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ZeroizeSwCover {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod mfdleft {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdL1Value {
        Select1,
        Select2,
    }
    pub type MfdL1ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdL1Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdL1Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Left Button 1
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdL1(Selector<17498, 32768, 15>);
    impl MfdL1 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdL1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdL1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdL10Value {
        Select1,
        Select2,
    }
    pub type MfdL10ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdL10Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdL10Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Left Button 10
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdL10(Selector<17500, 256, 8>);
    impl MfdL10 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdL10 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdL10 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdL11Value {
        Select1,
        Select2,
    }
    pub type MfdL11ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdL11Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdL11Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Left Button 11
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdL11(Selector<17500, 512, 9>);
    impl MfdL11 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdL11 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdL11 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdL12Value {
        Select1,
        Select2,
    }
    pub type MfdL12ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdL12Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdL12Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Left Button 12
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdL12(Selector<17500, 1024, 10>);
    impl MfdL12 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdL12 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdL12 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdL13Value {
        Select1,
        Select2,
    }
    pub type MfdL13ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdL13Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdL13Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Left Button 13
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdL13(Selector<17500, 2048, 11>);
    impl MfdL13 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdL13 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdL13 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdL14Value {
        Select1,
        Select2,
    }
    pub type MfdL14ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdL14Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdL14Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Left Button 14
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdL14(Selector<17500, 4096, 12>);
    impl MfdL14 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdL14 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdL14 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdL15Value {
        Select1,
        Select2,
    }
    pub type MfdL15ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdL15Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdL15Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Left Button 15
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdL15(Selector<17500, 8192, 13>);
    impl MfdL15 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdL15 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdL15 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdL16Value {
        Select1,
        Select2,
    }
    pub type MfdL16ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdL16Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdL16Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Left Button 16
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdL16(Selector<17500, 16384, 14>);
    impl MfdL16 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdL16 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdL16 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdL17Value {
        Select1,
        Select2,
    }
    pub type MfdL17ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdL17Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdL17Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Left Button 17
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdL17(Selector<17500, 32768, 15>);
    impl MfdL17 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdL17 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdL17 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdL18Value {
        Select1,
        Select2,
    }
    pub type MfdL18ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdL18Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdL18Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Left Button 18
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdL18(Selector<17502, 1, 0>);
    impl MfdL18 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdL18 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdL18 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdL19Value {
        Select1,
        Select2,
    }
    pub type MfdL19ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdL19Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdL19Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Left Button 19
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdL19(Selector<17502, 2, 1>);
    impl MfdL19 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdL19 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdL19 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdL2Value {
        Select1,
        Select2,
    }
    pub type MfdL2ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdL2Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdL2Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Left Button 2
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdL2(Selector<17500, 1, 0>);
    impl MfdL2 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdL2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdL2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdL20Value {
        Select1,
        Select2,
    }
    pub type MfdL20ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdL20Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdL20Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Left Button 20
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdL20(Selector<17502, 4, 2>);
    impl MfdL20 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdL20 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdL20 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdL3Value {
        Select1,
        Select2,
    }
    pub type MfdL3ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdL3Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdL3Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Left Button 3
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdL3(Selector<17500, 2, 1>);
    impl MfdL3 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdL3 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdL3 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdL4Value {
        Select1,
        Select2,
    }
    pub type MfdL4ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdL4Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdL4Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Left Button 4
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdL4(Selector<17500, 4, 2>);
    impl MfdL4 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdL4 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdL4 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdL5Value {
        Select1,
        Select2,
    }
    pub type MfdL5ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdL5Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdL5Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Left Button 5
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdL5(Selector<17500, 8, 3>);
    impl MfdL5 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdL5 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdL5 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdL6Value {
        Select1,
        Select2,
    }
    pub type MfdL6ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdL6Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdL6Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Left Button 6
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdL6(Selector<17500, 16, 4>);
    impl MfdL6 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdL6 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdL6 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdL7Value {
        Select1,
        Select2,
    }
    pub type MfdL7ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdL7Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdL7Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Left Button 7
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdL7(Selector<17500, 32, 5>);
    impl MfdL7 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdL7 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdL7 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdL8Value {
        Select1,
        Select2,
    }
    pub type MfdL8ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdL8Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdL8Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Left Button 8
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdL8(Selector<17500, 64, 6>);
    impl MfdL8 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdL8 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdL8 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdL9Value {
        Select1,
        Select2,
    }
    pub type MfdL9ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdL9Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdL9Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Left Button 9
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdL9(Selector<17500, 128, 7>);
    impl MfdL9 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdL9 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdL9 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdLBrtSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type MfdLBrtSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdLBrtSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for MfdLBrtSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// MFD Left BRT Rocker Switch
    /// Input: switch to previous or next state
    /// Input: set the switch position -- 0 = held left/down, 1 = centered, 2 = held right/up
    /// Output: selector position
    pub struct MfdLBrtSw(Selector<17502, 1536, 9>);
    impl MfdLBrtSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdLBrtSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdLBrtSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdLConSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type MfdLConSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdLConSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for MfdLConSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// MFD Left CON Rocker Switch
    /// Input: switch to previous or next state
    /// Input: set the switch position -- 0 = held left/down, 1 = centered, 2 = held right/up
    /// Output: selector position
    pub struct MfdLConSw(Selector<17502, 384, 7>);
    impl MfdLConSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdLConSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdLConSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdLGainSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type MfdLGainSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdLGainSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for MfdLGainSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// MFD Left GAIN Rocker Switch
    /// Input: switch to previous or next state
    /// Input: set the switch position -- 0 = held left/down, 1 = centered, 2 = held right/up
    /// Output: selector position
    pub struct MfdLGainSw(Selector<17502, 24, 3>);
    impl MfdLGainSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdLGainSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdLGainSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdLSymSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type MfdLSymSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdLSymSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for MfdLSymSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// MFD Left SYM Rocker Switch
    /// Input: switch to previous or next state
    /// Input: set the switch position -- 0 = held left/down, 1 = centered, 2 = held right/up
    /// Output: selector position
    pub struct MfdLSymSw(Selector<17502, 96, 5>);
    impl MfdLSymSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdLSymSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdLSymSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod mfdright {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdR1Value {
        Select1,
        Select2,
    }
    pub type MfdR1ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdR1Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdR1Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Right Button 1
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdR1(Selector<17502, 2048, 11>);
    impl MfdR1 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdR1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdR1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdR10Value {
        Select1,
        Select2,
    }
    pub type MfdR10ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdR10Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdR10Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Right Button 10
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdR10(Selector<17504, 16, 4>);
    impl MfdR10 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdR10 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdR10 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdR11Value {
        Select1,
        Select2,
    }
    pub type MfdR11ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdR11Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdR11Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Right Button 11
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdR11(Selector<17504, 32, 5>);
    impl MfdR11 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdR11 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdR11 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdR12Value {
        Select1,
        Select2,
    }
    pub type MfdR12ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdR12Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdR12Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Right Button 12
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdR12(Selector<17504, 64, 6>);
    impl MfdR12 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdR12 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdR12 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdR13Value {
        Select1,
        Select2,
    }
    pub type MfdR13ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdR13Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdR13Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Right Button 13
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdR13(Selector<17504, 128, 7>);
    impl MfdR13 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdR13 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdR13 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdR14Value {
        Select1,
        Select2,
    }
    pub type MfdR14ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdR14Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdR14Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Right Button 14
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdR14(Selector<17504, 256, 8>);
    impl MfdR14 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdR14 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdR14 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdR15Value {
        Select1,
        Select2,
    }
    pub type MfdR15ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdR15Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdR15Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Right Button 15
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdR15(Selector<17504, 512, 9>);
    impl MfdR15 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdR15 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdR15 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdR16Value {
        Select1,
        Select2,
    }
    pub type MfdR16ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdR16Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdR16Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Right Button 16
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdR16(Selector<17504, 1024, 10>);
    impl MfdR16 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdR16 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdR16 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdR17Value {
        Select1,
        Select2,
    }
    pub type MfdR17ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdR17Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdR17Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Right Button 17
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdR17(Selector<17504, 2048, 11>);
    impl MfdR17 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdR17 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdR17 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdR18Value {
        Select1,
        Select2,
    }
    pub type MfdR18ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdR18Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdR18Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Right Button 18
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdR18(Selector<17504, 4096, 12>);
    impl MfdR18 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdR18 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdR18 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdR19Value {
        Select1,
        Select2,
    }
    pub type MfdR19ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdR19Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdR19Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Right Button 19
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdR19(Selector<17504, 8192, 13>);
    impl MfdR19 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdR19 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdR19 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdR2Value {
        Select1,
        Select2,
    }
    pub type MfdR2ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdR2Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdR2Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Right Button 2
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdR2(Selector<17502, 4096, 12>);
    impl MfdR2 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdR2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdR2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdR20Value {
        Select1,
        Select2,
    }
    pub type MfdR20ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdR20Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdR20Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Right Button 20
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdR20(Selector<17504, 16384, 14>);
    impl MfdR20 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdR20 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdR20 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdR3Value {
        Select1,
        Select2,
    }
    pub type MfdR3ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdR3Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdR3Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Right Button 3
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdR3(Selector<17502, 8192, 13>);
    impl MfdR3 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdR3 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdR3 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdR4Value {
        Select1,
        Select2,
    }
    pub type MfdR4ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdR4Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdR4Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Right Button 4
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdR4(Selector<17502, 16384, 14>);
    impl MfdR4 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdR4 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdR4 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdR5Value {
        Select1,
        Select2,
    }
    pub type MfdR5ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdR5Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdR5Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Right Button 5
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdR5(Selector<17502, 32768, 15>);
    impl MfdR5 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdR5 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdR5 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdR6Value {
        Select1,
        Select2,
    }
    pub type MfdR6ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdR6Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdR6Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Right Button 6
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdR6(Selector<17504, 1, 0>);
    impl MfdR6 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdR6 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdR6 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdR7Value {
        Select1,
        Select2,
    }
    pub type MfdR7ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdR7Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdR7Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Right Button 7
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdR7(Selector<17504, 2, 1>);
    impl MfdR7 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdR7 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdR7 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdR8Value {
        Select1,
        Select2,
    }
    pub type MfdR8ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdR8Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdR8Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Right Button 8
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdR8(Selector<17504, 4, 2>);
    impl MfdR8 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdR8 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdR8 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdR9Value {
        Select1,
        Select2,
    }
    pub type MfdR9ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdR9Value) + 'a>;
    impl core::convert::TryFrom<u16> for MfdR9Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MFD Right Button 9
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MfdR9(Selector<17504, 8, 3>);
    impl MfdR9 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdR9 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdR9 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdRBrtSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type MfdRBrtSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdRBrtSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for MfdRBrtSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// MFD Right BRT Rocker Switch
    /// Input: switch to previous or next state
    /// Input: set the switch position -- 0 = held left/down, 1 = centered, 2 = held right/up
    /// Output: selector position
    pub struct MfdRBrtSw(Selector<17506, 192, 6>);
    impl MfdRBrtSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdRBrtSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdRBrtSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdRConSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type MfdRConSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdRConSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for MfdRConSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// MFD Right CON Rocker Switch
    /// Input: switch to previous or next state
    /// Input: set the switch position -- 0 = held left/down, 1 = centered, 2 = held right/up
    /// Output: selector position
    pub struct MfdRConSw(Selector<17506, 48, 4>);
    impl MfdRConSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdRConSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdRConSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdRGainSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type MfdRGainSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdRGainSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for MfdRGainSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// MFD Right GAIN Rocker Switch
    /// Input: switch to previous or next state
    /// Input: set the switch position -- 0 = held left/down, 1 = centered, 2 = held right/up
    /// Output: selector position
    pub struct MfdRGainSw(Selector<17506, 3, 0>);
    impl MfdRGainSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdRGainSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdRGainSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MfdRSymSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type MfdRSymSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MfdRSymSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for MfdRSymSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// MFD Right SYM Rocker Switch
    /// Input: switch to previous or next state
    /// Input: set the switch position -- 0 = held left/down, 1 = centered, 2 = held right/up
    /// Output: selector position
    pub struct MfdRSymSw(Selector<17506, 12, 2>);
    impl MfdRSymSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MfdRSymSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MfdRSymSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod mmc {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AltRelBtnValue {
        Select1,
        Select2,
    }
    pub type AltRelBtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(AltRelBtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for AltRelBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ALT REL Button - Push to release
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AltRelBtn(Selector<17444, 512, 9>);
    impl AltRelBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AltRelBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AltRelBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EmergStroreJettValue {
        Select1,
        Select2,
    }
    pub type EmergStroreJettValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(EmergStroreJettValue) + 'a>;
    impl core::convert::TryFrom<u16> for EmergStroreJettValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// EMER STORES JETTISON Button - Push to jettison
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct EmergStroreJett(Selector<17444, 128, 7>);
    impl EmergStroreJett {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EmergStroreJett {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EmergStroreJett {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GndJettEnableSwValue {
        Select1,
        Select2,
    }
    pub type GndJettEnableSwValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(GndJettEnableSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for GndJettEnableSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// GND JETT ENABLE Switch, ENABLE/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct GndJettEnableSw(Selector<17444, 256, 8>);
    impl GndJettEnableSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for GndJettEnableSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for GndJettEnableSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LaserArmSwValue {
        Select1,
        Select2,
    }
    pub type LaserArmSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(LaserArmSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for LaserArmSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// LASER ARM Switch, ARM/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LaserArmSw(Selector<17444, 1024, 10>);
    impl LaserArmSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LaserArmSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LaserArmSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MasterArmSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type MasterArmSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MasterArmSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for MasterArmSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// MASTER ARM Switch, MASTER ARM/OFF/SIMULATE
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct MasterArmSw(Selector<17444, 96, 5>);
    impl MasterArmSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MasterArmSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MasterArmSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod oxygensystem {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Cabin Pressure Altimeter
    /// Output: gauge position
    pub struct CockpitAlititude(AnalogGauge<17642, 65535, 0>);
    impl CockpitAlititude {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CockpitAlititude {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CockpitAlititude {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Flow Indicator Gauge
    /// Output: gauge position
    pub struct FlowIndicator(AnalogGauge<17646, 65535, 0>);
    impl FlowIndicator {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FlowIndicator {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FlowIndicator {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Flow Indicator Light
    /// Output: 0 if light is off, 1 if light is on
    pub struct FlowIndicatorLight(Led<17538, 512, 9>);
    impl FlowIndicatorLight {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for FlowIndicatorLight {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FlowIndicatorLight {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ObogsSwValue {
        Select1,
        Select2,
    }
    pub type ObogsSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(ObogsSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for ObogsSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// OBOGS BIT Switch, BIT/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ObogsSw(Selector<17442, 8, 3>);
    impl ObogsSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ObogsSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ObogsSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum OxyDiluterLvrValue {
        Select1,
        Select2,
    }
    pub type OxyDiluterLvrValueCallback<'a> = alloc::boxed::Box<dyn FnMut(OxyDiluterLvrValue) + 'a>;
    impl core::convert::TryFrom<u16> for OxyDiluterLvrValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Diluter Lever, 100 percent/NORM
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct OxyDiluterLvr(Selector<17442, 1, 0>);
    impl OxyDiluterLvr {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for OxyDiluterLvr {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for OxyDiluterLvr {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Emergency Lever, EMERGENCY/NORMAL/TEST MASK
    /// Input: set the switch position
    /// Output: switch position -- 0 = Down, 1 = Mid ,  2 = UP
    pub struct OxyEmergLvr(Selector<17442, 6, 1>);
    impl OxyEmergLvr {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for OxyEmergLvr {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for OxyEmergLvr {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum OxySupplyLvrValue {
        Select1,
        Select2,
        Select3,
    }
    pub type OxySupplyLvrValueCallback<'a> = alloc::boxed::Box<dyn FnMut(OxySupplyLvrValue) + 'a>;
    impl core::convert::TryFrom<u16> for OxySupplyLvrValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// Supply Lever, PBG/ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct OxySupplyLvr(Selector<17440, 49152, 14>);
    impl OxySupplyLvr {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for OxySupplyLvr {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for OxySupplyLvr {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Oxygen Pressure
    /// Output: gauge position
    pub struct OxygenPressure(AnalogGauge<17644, 65535, 0>);
    impl OxygenPressure {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for OxygenPressure {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for OxygenPressure {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod rwr {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RwrActPwrBtnValue {
        Select1,
        Select2,
    }
    pub type RwrActPwrBtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RwrActPwrBtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for RwrActPwrBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// RWR Indicator Control ACT/PWR Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RwrActPwrBtn(Selector<17490, 8192, 13>);
    impl RwrActPwrBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrActPwrBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrActPwrBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RwrAltBtnValue {
        Select1,
        Select2,
    }
    pub type RwrAltBtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RwrAltBtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for RwrAltBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// RWR Indicator Control ALTITUDE Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RwrAltBtn(Selector<17490, 16384, 14>);
    impl RwrAltBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrAltBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrAltBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RwrHandoffBtnValue {
        Select1,
        Select2,
    }
    pub type RwrHandoffBtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RwrHandoffBtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for RwrHandoffBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// RWR Indicator Control HANDOFF Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RwrHandoffBtn(Selector<17490, 64, 6>);
    impl RwrHandoffBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrHandoffBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrHandoffBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RWR Indicator Control DIM Knob - Rotate to adjust brightness
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct RwrIndDimKnb(LimitedDial<17496, 65535, 0>);
    impl RwrIndDimKnb {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrIndDimKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrIndDimKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RWR Intensity Knob - Rotate to adjust brightness
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct RwrIntensKnb(LimitedDial<17494, 65535, 0>);
    impl RwrIntensKnb {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrIntensKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrIntensKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RwrLaunchBtnValue {
        Select1,
        Select2,
    }
    pub type RwrLaunchBtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RwrLaunchBtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for RwrLaunchBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// RWR Indicator Control LAUNCH Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RwrLaunchBtn(Selector<17490, 128, 7>);
    impl RwrLaunchBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrLaunchBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrLaunchBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RwrModeBtnValue {
        Select1,
        Select2,
    }
    pub type RwrModeBtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RwrModeBtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for RwrModeBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// RWR Indicator Control MODE Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RwrModeBtn(Selector<17490, 256, 8>);
    impl RwrModeBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrModeBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrModeBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RwrPwrBtnValue {
        Select1,
        Select2,
    }
    pub type RwrPwrBtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RwrPwrBtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for RwrPwrBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// RWR Indicator Control POWER Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RwrPwrBtn(Selector<17490, 32768, 15>);
    impl RwrPwrBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrPwrBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrPwrBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RwrSearchBtnValue {
        Select1,
        Select2,
    }
    pub type RwrSearchBtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RwrSearchBtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for RwrSearchBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// RWR Indicator Control SEARCH Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RwrSearchBtn(Selector<17490, 4096, 12>);
    impl RwrSearchBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrSearchBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrSearchBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RwrSysTestBtnValue {
        Select1,
        Select2,
    }
    pub type RwrSysTestBtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RwrSysTestBtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for RwrSysTestBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// RWR Indicator Control SYS TEST Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RwrSysTestBtn(Selector<17490, 1024, 10>);
    impl RwrSysTestBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrSysTestBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrSysTestBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RwrTBtnValue {
        Select1,
        Select2,
    }
    pub type RwrTBtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RwrTBtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for RwrTBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// RWR Indicator Control T Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RwrTBtn(Selector<17490, 2048, 11>);
    impl RwrTBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrTBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrTBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RwrUnknownShipBtnValue {
        Select1,
        Select2,
    }
    pub type RwrUnknownShipBtnValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(RwrUnknownShipBtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for RwrUnknownShipBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// RWR Indicator Control UNKNOWN SHIP Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RwrUnknownShipBtn(Selector<17490, 512, 9>);
    impl RwrUnknownShipBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrUnknownShipBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrUnknownShipBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod sai {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// SAI Aircraft Reference Symbol
    /// Output: gauge position
    pub struct SaiAircraftreferencesymbol(AnalogGauge<17578, 65535, 0>);
    impl SaiAircraftreferencesymbol {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SaiAircraftreferencesymbol {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SaiAircraftreferencesymbol {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// SAI Bank
    /// Output: gauge position
    pub struct SaiBank(AnalogGauge<17574, 65535, 0>);
    impl SaiBank {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SaiBank {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SaiBank {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// SAI Bank Arrow
    /// Output: gauge position
    pub struct SaiBankArrow(AnalogGauge<17840, 65535, 0>);
    impl SaiBankArrow {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SaiBankArrow {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SaiBankArrow {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SaiCageValue {
        Select1,
        Select2,
    }
    pub type SaiCageValueCallback<'a> = alloc::boxed::Box<dyn FnMut(SaiCageValue) + 'a>;
    impl core::convert::TryFrom<u16> for SaiCageValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// SAI Cage Knob, (LMB) Pull to cage
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct SaiCage(Selector<17504, 32768, 15>);
    impl SaiCage {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SaiCage {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SaiCage {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// SAI Knob Arrow
    /// Output: gauge position
    pub struct SaiKnbArrow(AnalogGauge<17580, 65535, 0>);
    impl SaiKnbArrow {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SaiKnbArrow {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SaiKnbArrow {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// SAI Off Flag
    /// Output: gauge position
    pub struct SaiOffFlag(AnalogGauge<17576, 65535, 0>);
    impl SaiOffFlag {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SaiOffFlag {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SaiOffFlag {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    ///  SAI Pitch
    /// Output: gauge position
    pub struct SaiPitch(AnalogGauge<17572, 65535, 0>);
    impl SaiPitch {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SaiPitch {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SaiPitch {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// SAI Cage Knob, (MW) Adjust aircraft reference symbol
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct SaiPitchTrim(AnalogDial<17512, 65535, 0>);
    impl SaiPitchTrim {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SaiPitchTrim {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SaiPitchTrim {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod sensorpanel {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FcrPwrSwValue {
        Select1,
        Select2,
    }
    pub type FcrPwrSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(FcrPwrSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for FcrPwrSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// FCR Switch, FCR/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FcrPwrSw(Selector<17442, 64, 6>);
    impl FcrPwrSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FcrPwrSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FcrPwrSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum HdptSwLValue {
        Select1,
        Select2,
    }
    pub type HdptSwLValueCallback<'a> = alloc::boxed::Box<dyn FnMut(HdptSwLValue) + 'a>;
    impl core::convert::TryFrom<u16> for HdptSwLValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// LEFT HDPT Switch, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct HdptSwL(Selector<17442, 16, 4>);
    impl HdptSwL {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HdptSwL {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HdptSwL {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum HdptSwRValue {
        Select1,
        Select2,
    }
    pub type HdptSwRValueCallback<'a> = alloc::boxed::Box<dyn FnMut(HdptSwRValue) + 'a>;
    impl core::convert::TryFrom<u16> for HdptSwRValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// RIGHT HDPT Switch, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct HdptSwR(Selector<17442, 32, 5>);
    impl HdptSwR {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HdptSwR {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HdptSwR {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RdrAltPwrSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type RdrAltPwrSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RdrAltPwrSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for RdrAltPwrSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// RDR ALT Switch, RDR ALT/STBY/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct RdrAltPwrSw(Selector<17442, 384, 7>);
    impl RdrAltPwrSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RdrAltPwrSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RdrAltPwrSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod speedbrake {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Speed Brake Indicator
    /// Output: gauge position
    pub struct SpeedbrakeIndicator(AnalogGauge<17610, 65535, 0>);
    impl SpeedbrakeIndicator {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SpeedbrakeIndicator {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SpeedbrakeIndicator {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod trimindicators {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Pitch Trim Indicator
    /// Output: gauge position
    pub struct Pitchtrimind(AnalogGauge<17608, 65535, 0>);
    impl Pitchtrimind {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Pitchtrimind {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Pitchtrimind {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Roll Trim Indicator
    /// Output: gauge position
    pub struct Rolltrimind(AnalogGauge<17606, 65535, 0>);
    impl Rolltrimind {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Rolltrimind {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Rolltrimind {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod ufc {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FAckBtnValue {
        Select1,
        Select2,
    }
    pub type FAckBtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(FAckBtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for FAckBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// F-ACK Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FAckBtn(Selector<17456, 1024, 10>);
    impl FAckBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FAckBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FAckBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IcpAaModeBtnValue {
        Select1,
        Select2,
    }
    pub type IcpAaModeBtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IcpAaModeBtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for IcpAaModeBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ICP Master Mode Button, A-A
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IcpAaModeBtn(Selector<17446, 512, 9>);
    impl IcpAaModeBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IcpAaModeBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IcpAaModeBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IcpAgModeBtnValue {
        Select1,
        Select2,
    }
    pub type IcpAgModeBtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IcpAgModeBtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for IcpAgModeBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ICP Master Mode Button, A-G
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IcpAgModeBtn(Selector<17446, 1024, 10>);
    impl IcpAgModeBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IcpAgModeBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IcpAgModeBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IcpBtn0Value {
        Select1,
        Select2,
    }
    pub type IcpBtn0ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IcpBtn0Value) + 'a>;
    impl core::convert::TryFrom<u16> for IcpBtn0Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ICP Priority Function Button, 0(M-SEL)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IcpBtn0(Selector<17446, 16, 4>);
    impl IcpBtn0 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IcpBtn0 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IcpBtn0 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IcpBtn1Value {
        Select1,
        Select2,
    }
    pub type IcpBtn1ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IcpBtn1Value) + 'a>;
    impl core::convert::TryFrom<u16> for IcpBtn1Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ICP Priority Function Button, 1(T-ILS)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IcpBtn1(Selector<17444, 2048, 11>);
    impl IcpBtn1 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IcpBtn1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IcpBtn1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IcpBtn2Value {
        Select1,
        Select2,
    }
    pub type IcpBtn2ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IcpBtn2Value) + 'a>;
    impl core::convert::TryFrom<u16> for IcpBtn2Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ICP Priority Function Button, 2/N(ALOW)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IcpBtn2(Selector<17444, 4096, 12>);
    impl IcpBtn2 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IcpBtn2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IcpBtn2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IcpBtn3Value {
        Select1,
        Select2,
    }
    pub type IcpBtn3ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IcpBtn3Value) + 'a>;
    impl core::convert::TryFrom<u16> for IcpBtn3Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ICP Priority Function Button, 3
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IcpBtn3(Selector<17444, 8192, 13>);
    impl IcpBtn3 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IcpBtn3 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IcpBtn3 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IcpBtn4Value {
        Select1,
        Select2,
    }
    pub type IcpBtn4ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IcpBtn4Value) + 'a>;
    impl core::convert::TryFrom<u16> for IcpBtn4Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ICP Priority Function Button, 4/W(STPT)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IcpBtn4(Selector<17444, 16384, 14>);
    impl IcpBtn4 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IcpBtn4 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IcpBtn4 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IcpBtn5Value {
        Select1,
        Select2,
    }
    pub type IcpBtn5ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IcpBtn5Value) + 'a>;
    impl core::convert::TryFrom<u16> for IcpBtn5Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ICP Priority Function Button, 5(CRUS)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IcpBtn5(Selector<17444, 32768, 15>);
    impl IcpBtn5 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IcpBtn5 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IcpBtn5 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IcpBtn6Value {
        Select1,
        Select2,
    }
    pub type IcpBtn6ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IcpBtn6Value) + 'a>;
    impl core::convert::TryFrom<u16> for IcpBtn6Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ICP Priority Function Button, 6/E(TIME)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IcpBtn6(Selector<17446, 1, 0>);
    impl IcpBtn6 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IcpBtn6 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IcpBtn6 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IcpBtn7Value {
        Select1,
        Select2,
    }
    pub type IcpBtn7ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IcpBtn7Value) + 'a>;
    impl core::convert::TryFrom<u16> for IcpBtn7Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ICP Priority Function Button, 7(MARK)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IcpBtn7(Selector<17446, 2, 1>);
    impl IcpBtn7 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IcpBtn7 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IcpBtn7 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IcpBtn8Value {
        Select1,
        Select2,
    }
    pub type IcpBtn8ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IcpBtn8Value) + 'a>;
    impl core::convert::TryFrom<u16> for IcpBtn8Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ICP Priority Function Button, 8/S(FIX)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IcpBtn8(Selector<17446, 4, 2>);
    impl IcpBtn8 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IcpBtn8 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IcpBtn8 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IcpBtn9Value {
        Select1,
        Select2,
    }
    pub type IcpBtn9ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IcpBtn9Value) + 'a>;
    impl core::convert::TryFrom<u16> for IcpBtn9Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ICP Priority Function Button, 9(A-CAL)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IcpBtn9(Selector<17446, 8, 3>);
    impl IcpBtn9 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IcpBtn9 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IcpBtn9 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IcpCom1BtnValue {
        Select1,
        Select2,
    }
    pub type IcpCom1BtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IcpCom1BtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for IcpCom1BtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ICP COM Override Button, COM1(UHF)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IcpCom1Btn(Selector<17446, 32, 5>);
    impl IcpCom1Btn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IcpCom1Btn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IcpCom1Btn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IcpCom2BtnValue {
        Select1,
        Select2,
    }
    pub type IcpCom2BtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IcpCom2BtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for IcpCom2BtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ICP COM Override Button, COM2(VHF)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IcpCom2Btn(Selector<17446, 64, 6>);
    impl IcpCom2Btn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IcpCom2Btn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IcpCom2Btn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ICP Data Control Switch, RTN-SEQ
    /// Input: set the switch position
    /// Output: switch position -- 0 = Down, 1 = Mid ,  2 = UP
    pub struct IcpDataRtnSeqSw(Selector<17456, 192, 6>);
    impl IcpDataRtnSeqSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IcpDataRtnSeqSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IcpDataRtnSeqSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ICP Data Control Switch, UP-DN
    /// Input: set the switch position
    /// Output: switch position -- 0 = Down, 1 = Mid ,  2 = UP
    pub struct IcpDataUpDnSw(Selector<17456, 768, 8>);
    impl IcpDataUpDnSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IcpDataUpDnSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IcpDataUpDnSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ICP DED Increment/Decrement Switch
    /// Input: set the switch position
    /// Output: switch position -- 0 = Down, 1 = Mid ,  2 = UP
    pub struct IcpDedSw(Selector<17456, 3, 0>);
    impl IcpDedSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IcpDedSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IcpDedSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ICP DRIFT CUTOUT/WARN RESET Switch, DRIFT C/O /NORM/WARN RESET
    /// Input: set the switch position
    /// Output: switch position -- 0 = Down, 1 = Mid ,  2 = UP
    pub struct IcpDriftSw(Selector<17456, 48, 4>);
    impl IcpDriftSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IcpDriftSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IcpDriftSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IcpEntrBtnValue {
        Select1,
        Select2,
    }
    pub type IcpEntrBtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IcpEntrBtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for IcpEntrBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ICP Enter Button, ENTR
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IcpEntrBtn(Selector<17446, 4096, 12>);
    impl IcpEntrBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IcpEntrBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IcpEntrBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IcpFlirGainSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type IcpFlirGainSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IcpFlirGainSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for IcpFlirGainSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// ICP FLIR GAIN/LEVEL Switch, GAIN/LVL/AUTO
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct IcpFlirGainSw(Selector<17446, 49152, 14>);
    impl IcpFlirGainSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IcpFlirGainSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IcpFlirGainSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ICP FLIR Increment/Decrement Switch
    /// Input: set the switch position
    /// Output: switch position -- 0 = Down, 1 = Mid ,  2 = UP
    pub struct IcpFlirSw(Selector<17456, 12, 2>);
    impl IcpFlirSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IcpFlirSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IcpFlirSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ICP HUD Symbology Intensity Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct IcpHudBrtKnb(LimitedDial<17454, 65535, 0>);
    impl IcpHudBrtKnb {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IcpHudBrtKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IcpHudBrtKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IcpIffBtnValue {
        Select1,
        Select2,
    }
    pub type IcpIffBtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IcpIffBtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for IcpIffBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ICP IFF Override Button, IFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IcpIffBtn(Selector<17446, 128, 7>);
    impl IcpIffBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IcpIffBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IcpIffBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IcpListBtnValue {
        Select1,
        Select2,
    }
    pub type IcpListBtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IcpListBtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for IcpListBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ICP LIST Override Button, LIST
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IcpListBtn(Selector<17446, 256, 8>);
    impl IcpListBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IcpListBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IcpListBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ICP Raster Intensity Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct IcpRasterBrtKnb(LimitedDial<17452, 65535, 0>);
    impl IcpRasterBrtKnb {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IcpRasterBrtKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IcpRasterBrtKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ICP Raster Contrast Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct IcpRasterContrKnb(LimitedDial<17450, 65535, 0>);
    impl IcpRasterContrKnb {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IcpRasterContrKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IcpRasterContrKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IcpRclBtnValue {
        Select1,
        Select2,
    }
    pub type IcpRclBtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IcpRclBtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for IcpRclBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ICP Recall Button, RCL
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IcpRclBtn(Selector<17446, 2048, 11>);
    impl IcpRclBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IcpRclBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IcpRclBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ICP Reticle Depression Control Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct IcpReticleDepressKnb(LimitedDial<17448, 65535, 0>);
    impl IcpReticleDepressKnb {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IcpReticleDepressKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IcpReticleDepressKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IcpWxBtnValue {
        Select1,
        Select2,
    }
    pub type IcpWxBtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IcpWxBtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for IcpWxBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ICP FLIR Polarity Button, Wx
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IcpWxBtn(Selector<17446, 8192, 13>);
    impl IcpWxBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IcpWxBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IcpWxBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffIdBtnValue {
        Select1,
        Select2,
    }
    pub type IffIdBtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IffIdBtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for IffIdBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// IFF IDENT Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IffIdBtn(Selector<17456, 2048, 11>);
    impl IffIdBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IffIdBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffIdBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RfSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type RfSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RfSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for RfSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// RF Switch, SILENT/QUIET/NORM
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct RfSw(Selector<17456, 12288, 12>);
    impl RfSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RfSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RfSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod uhf {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// UHF CHAN Display
    /// Output: UHF CHAN Display
    pub struct UhfChanDisp(Display<17830, 2>);
    impl UhfChanDisp {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for UhfChanDisp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfChanDisp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// UHF CHAN Knob
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct UhfChanKnb(Selector<17460, 15872, 9>);
    impl UhfChanKnb {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfChanKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfChanKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UhfDoorValue {
        Select1,
        Select2,
    }
    pub type UhfDoorValueCallback<'a> = alloc::boxed::Box<dyn FnMut(UhfDoorValue) + 'a>;
    impl core::convert::TryFrom<u16> for UhfDoorValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// UHF Access Door, OPEN/CLOSE
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UhfDoor(Selector<17482, 64, 6>);
    impl UhfDoor {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfDoor {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfDoor {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UhfFreq0025KnbValue {
        Select1,
        Select2,
        Select3,
        Select4,
    }
    pub type UhfFreq0025KnbValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(UhfFreq0025KnbValue) + 'a>;
    impl core::convert::TryFrom<u16> for UhfFreq0025KnbValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                3 => Ok(Self::Select4),
                value => Err(value),
            }
        }
    }

    /// UHF Manual Frequency Knob 0.025 MHz
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct UhfFreq0025Knb(Selector<17480, 12288, 12>);
    impl UhfFreq0025Knb {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfFreq0025Knb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfFreq0025Knb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UhfFreq01KnbValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
        Select6,
        Select7,
        Select8,
        Select9,
        Select10,
    }
    pub type UhfFreq01KnbValueCallback<'a> = alloc::boxed::Box<dyn FnMut(UhfFreq01KnbValue) + 'a>;
    impl core::convert::TryFrom<u16> for UhfFreq01KnbValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                3 => Ok(Self::Select4),
                4 => Ok(Self::Select5),
                5 => Ok(Self::Select6),
                6 => Ok(Self::Select7),
                7 => Ok(Self::Select8),
                8 => Ok(Self::Select9),
                9 => Ok(Self::Select10),
                value => Err(value),
            }
        }
    }

    /// UHF Manual Frequency Knob 0.1 MHz
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct UhfFreq01Knb(Selector<17480, 3840, 8>);
    impl UhfFreq01Knb {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfFreq01Knb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfFreq01Knb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UhfFreq100KnbValue {
        Select1,
        Select2,
        Select3,
    }
    pub type UhfFreq100KnbValueCallback<'a> = alloc::boxed::Box<dyn FnMut(UhfFreq100KnbValue) + 'a>;
    impl core::convert::TryFrom<u16> for UhfFreq100KnbValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// UHF Manual Frequency Knob 100 MHz
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct UhfFreq100Knb(Selector<17460, 49152, 14>);
    impl UhfFreq100Knb {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfFreq100Knb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfFreq100Knb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UhfFreq10KnbValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
        Select6,
        Select7,
        Select8,
        Select9,
        Select10,
    }
    pub type UhfFreq10KnbValueCallback<'a> = alloc::boxed::Box<dyn FnMut(UhfFreq10KnbValue) + 'a>;
    impl core::convert::TryFrom<u16> for UhfFreq10KnbValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                3 => Ok(Self::Select4),
                4 => Ok(Self::Select5),
                5 => Ok(Self::Select6),
                6 => Ok(Self::Select7),
                7 => Ok(Self::Select8),
                8 => Ok(Self::Select9),
                9 => Ok(Self::Select10),
                value => Err(value),
            }
        }
    }

    /// UHF Manual Frequency Knob 10 MHz
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct UhfFreq10Knb(Selector<17480, 15, 0>);
    impl UhfFreq10Knb {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfFreq10Knb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfFreq10Knb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UhfFreq1KnbValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
        Select6,
        Select7,
        Select8,
        Select9,
        Select10,
    }
    pub type UhfFreq1KnbValueCallback<'a> = alloc::boxed::Box<dyn FnMut(UhfFreq1KnbValue) + 'a>;
    impl core::convert::TryFrom<u16> for UhfFreq1KnbValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                3 => Ok(Self::Select4),
                4 => Ok(Self::Select5),
                5 => Ok(Self::Select6),
                6 => Ok(Self::Select7),
                7 => Ok(Self::Select8),
                8 => Ok(Self::Select9),
                9 => Ok(Self::Select10),
                value => Err(value),
            }
        }
    }

    /// UHF Manual Frequency Knob 1 MHz
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct UhfFreq1Knb(Selector<17480, 240, 4>);
    impl UhfFreq1Knb {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfFreq1Knb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfFreq1Knb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// UHF Manual Frequency Display
    /// Output: UHF Manual Frequency Display
    pub struct UhfFreqDisp(Display<17832, 7>);
    impl UhfFreqDisp {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for UhfFreqDisp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfFreqDisp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UhfFuncKnbValue {
        Select1,
        Select2,
        Select3,
        Select4,
    }
    pub type UhfFuncKnbValueCallback<'a> = alloc::boxed::Box<dyn FnMut(UhfFuncKnbValue) + 'a>;
    impl core::convert::TryFrom<u16> for UhfFuncKnbValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                3 => Ok(Self::Select4),
                value => Err(value),
            }
        }
    }

    /// UHF Function Knob
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct UhfFuncKnb(Selector<17480, 49152, 14>);
    impl UhfFuncKnb {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfFuncKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfFuncKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UhfModeKnbValue {
        Select1,
        Select2,
        Select3,
    }
    pub type UhfModeKnbValueCallback<'a> = alloc::boxed::Box<dyn FnMut(UhfModeKnbValue) + 'a>;
    impl core::convert::TryFrom<u16> for UhfModeKnbValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                2 => Ok(Self::Select3),
                value => Err(value),
            }
        }
    }

    /// UHF Mode Knob
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct UhfModeKnb(Selector<17482, 3, 0>);
    impl UhfModeKnb {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfModeKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfModeKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UhfSquelchSwValue {
        Select1,
        Select2,
    }
    pub type UhfSquelchSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(UhfSquelchSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for UhfSquelchSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// UHF SQUELCH Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UhfSquelchSw(Selector<17482, 8, 3>);
    impl UhfSquelchSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfSquelchSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfSquelchSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UhfStatusBtnValue {
        Select1,
        Select2,
    }
    pub type UhfStatusBtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(UhfStatusBtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for UhfStatusBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// UHF STATUS Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UhfStatusBtn(Selector<17482, 32, 5>);
    impl UhfStatusBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfStatusBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfStatusBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UhfTestBtnValue {
        Select1,
        Select2,
    }
    pub type UhfTestBtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(UhfTestBtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for UhfTestBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// UHF TEST DISPLAY Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UhfTestBtn(Selector<17482, 16, 4>);
    impl UhfTestBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfTestBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfTestBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UhfToneBtnValue {
        Select1,
        Select2,
    }
    pub type UhfToneBtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(UhfToneBtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for UhfToneBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// UHF Tone Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UhfToneBtn(Selector<17482, 4, 2>);
    impl UhfToneBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfToneBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfToneBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// UHF VOL Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct UhfVolKnb(LimitedDial<17484, 65535, 0>);
    impl UhfVolKnb {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfVolKnb {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfVolKnb {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod verticalvelocityindicator {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// VVI
    /// Output: gauge position
    pub struct Vvi(AnalogGauge<17582, 65535, 0>);
    impl Vvi {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Vvi {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Vvi {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod warningcautionandindicatorlights {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// ACFT BATT FAIL Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightAcftBattFail(Led<17530, 256, 8>);
    impl LightAcftBattFail {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightAcftBattFail {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightAcftBattFail {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ACTIVE Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightActive(Led<17530, 512, 9>);
    impl LightActive {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightActive {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightActive {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// AFT FUEL LOW Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightAftFuelLow(Led<17526, 8, 3>);
    impl LightAftFuelLow {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightAftFuelLow {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightAftFuelLow {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// AIR Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightAir(Led<17528, 16384, 14>);
    impl LightAir {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightAir {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightAir {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ANTI SKID Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightAntiSkid(Led<17524, 4, 2>);
    impl LightAntiSkid {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightAntiSkid {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightAntiSkid {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// AoA Light down (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightAoaDn(Led<17528, 16, 4>);
    impl LightAoaDn {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightAoaDn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightAoaDn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// AoA Light mid (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightAoaMid(Led<17528, 8, 3>);
    impl LightAoaMid {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightAoaMid {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightAoaMid {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// AoA Light up (red)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightAoaUp(Led<17528, 4, 2>);
    impl LightAoaUp {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightAoaUp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightAoaUp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// AR NWS Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightArNws(Led<17528, 1024, 10>);
    impl LightArNws {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightArNws {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightArNws {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ATF NOT ENGAGED Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightAtfNot(Led<17524, 2048, 11>);
    impl LightAtfNot {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightAtfNot {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightAtfNot {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// AVIONICS FAULT Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightAvionicsFault(Led<17522, 512, 9>);
    impl LightAvionicsFault {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightAvionicsFault {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightAvionicsFault {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// BUC Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightBuc(Led<17526, 1, 0>);
    impl LightBuc {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightBuc {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightBuc {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// CABIN PRESS Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightCabinPress(Led<17524, 16384, 14>);
    impl LightCabinPress {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightCabinPress {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightCabinPress {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// CADC Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightCadc(Led<17524, 8, 3>);
    impl LightCadc {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightCadc {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightCadc {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// CANOPY Light (red)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightCanopy(Led<17528, 1, 0>);
    impl LightCanopy {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightCanopy {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightCanopy {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// 1. Caution - Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightCaution1(Led<17524, 8192, 13>);
    impl LightCaution1 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightCaution1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightCaution1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// 2. Caution - Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightCaution2(Led<17526, 2, 1>);
    impl LightCaution2 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightCaution2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightCaution2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// 3. Caution - Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightCaution3(Led<17526, 4, 2>);
    impl LightCaution3 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightCaution3 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightCaution3 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// 4. Caution - Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightCaution4(Led<17526, 16, 4>);
    impl LightCaution4 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightCaution4 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightCaution4 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// 5. Caution - Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightCaution5(Led<17526, 32, 5>);
    impl LightCaution5 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightCaution5 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightCaution5 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// 6. Caution - Light  (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightCaution6(Led<17526, 64, 6>);
    impl LightCaution6 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightCaution6 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightCaution6 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// CMDS DISP Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightCmdsDisp(Led<17534, 4, 2>);
    impl LightCmdsDisp {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightCmdsDisp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightCmdsDisp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// CMDS GO Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightCmdsGo(Led<17534, 2, 1>);
    impl LightCmdsGo {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightCmdsGo {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightCmdsGo {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// CMDS NO GO Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightCmdsNoGo(Led<17534, 1, 0>);
    impl LightCmdsNoGo {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightCmdsNoGo {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightCmdsNoGo {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// CMDS RDY Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightCmdsRdy(Led<17534, 8, 3>);
    impl LightCmdsRdy {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightCmdsRdy {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightCmdsRdy {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Consoles Lightning (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightConsles(Led<17534, 16, 4>);
    impl LightConsles {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightConsles {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightConsles {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Consoles Flood Lightning (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightConslesFlood(Led<17534, 64, 6>);
    impl LightConslesFlood {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightConslesFlood {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightConslesFlood {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// DBU ON Light (red)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightDbuOn(Led<17526, 16384, 14>);
    impl LightDbuOn {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightDbuOn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightDbuOn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// DISC Light (red)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightDisc(Led<17528, 2048, 11>);
    impl LightDisc {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightDisc {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightDisc {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ECM Button 1 A Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEcm1A(Led<17534, 512, 9>);
    impl LightEcm1A {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEcm1A {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEcm1A {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ECM Button 1 F Light (red)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEcm1F(Led<17534, 1024, 10>);
    impl LightEcm1F {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEcm1F {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEcm1F {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ECM Button 1 S Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEcm1S(Led<17534, 256, 8>);
    impl LightEcm1S {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEcm1S {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEcm1S {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ECM Button 1 T Light (blue)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEcm1T(Led<17534, 2048, 11>);
    impl LightEcm1T {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEcm1T {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEcm1T {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ECM Button 2 A Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEcm2A(Led<17534, 8192, 13>);
    impl LightEcm2A {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEcm2A {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEcm2A {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ECM Button 2 F Light (red)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEcm2F(Led<17534, 16384, 14>);
    impl LightEcm2F {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEcm2F {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEcm2F {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ECM Button 2 S Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEcm2S(Led<17534, 4096, 12>);
    impl LightEcm2S {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEcm2S {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEcm2S {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ECM Button 2 T Light (blue)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEcm2T(Led<17534, 32768, 15>);
    impl LightEcm2T {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEcm2T {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEcm2T {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ECM Button 3 A Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEcm3A(Led<17536, 2, 1>);
    impl LightEcm3A {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEcm3A {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEcm3A {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ECM Button 3 F Light (red)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEcm3F(Led<17536, 4, 2>);
    impl LightEcm3F {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEcm3F {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEcm3F {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ECM Button 3 S Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEcm3S(Led<17536, 1, 0>);
    impl LightEcm3S {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEcm3S {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEcm3S {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ECM Button 3 T Light (blue)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEcm3T(Led<17536, 8, 3>);
    impl LightEcm3T {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEcm3T {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEcm3T {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ECM Button 4 A Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEcm4A(Led<17536, 32, 5>);
    impl LightEcm4A {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEcm4A {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEcm4A {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ECM Button 4 F Light (red)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEcm4F(Led<17536, 64, 6>);
    impl LightEcm4F {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEcm4F {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEcm4F {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ECM Button 4 S Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEcm4S(Led<17536, 16, 4>);
    impl LightEcm4S {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEcm4S {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEcm4S {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ECM Button 4 T Light (blue)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEcm4T(Led<17536, 128, 7>);
    impl LightEcm4T {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEcm4T {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEcm4T {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ECM Button 5 A Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEcm5A(Led<17536, 512, 9>);
    impl LightEcm5A {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEcm5A {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEcm5A {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ECM Button 5 F Light (red)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEcm5F(Led<17536, 1024, 10>);
    impl LightEcm5F {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEcm5F {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEcm5F {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ECM Button 5 S Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEcm5S(Led<17536, 256, 8>);
    impl LightEcm5S {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEcm5S {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEcm5S {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ECM Button 5 T Light (blue)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEcm5T(Led<17536, 2048, 11>);
    impl LightEcm5T {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEcm5T {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEcm5T {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ECM Button A Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEcmA(Led<17536, 8192, 13>);
    impl LightEcmA {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEcmA {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEcmA {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ECM Button F Light (red)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEcmF(Led<17536, 16384, 14>);
    impl LightEcmF {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEcmF {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEcmF {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ECM Button FRM A Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEcmFrmA(Led<17538, 2, 1>);
    impl LightEcmFrmA {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEcmFrmA {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEcmFrmA {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ECM Button FRM F Light (red)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEcmFrmF(Led<17538, 4, 2>);
    impl LightEcmFrmF {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEcmFrmF {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEcmFrmF {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ECM Button FRM S Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEcmFrmS(Led<17538, 1, 0>);
    impl LightEcmFrmS {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEcmFrmS {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEcmFrmS {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ECM Button FRM T Light (blue)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEcmFrmT(Led<17538, 8, 3>);
    impl LightEcmFrmT {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEcmFrmT {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEcmFrmT {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ECM Button S Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEcmS(Led<17536, 4096, 12>);
    impl LightEcmS {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEcmS {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEcmS {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ECM Button SPL A Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEcmSplA(Led<17538, 32, 5>);
    impl LightEcmSplA {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEcmSplA {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEcmSplA {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ECM Button SPL F Light (red)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEcmSplF(Led<17538, 64, 6>);
    impl LightEcmSplF {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEcmSplF {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEcmSplF {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ECM Button SPL S Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEcmSplS(Led<17538, 16, 4>);
    impl LightEcmSplS {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEcmSplS {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEcmSplS {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ECM Button SPL T Light (blue)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEcmSplT(Led<17538, 128, 7>);
    impl LightEcmSplT {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEcmSplT {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEcmSplT {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ECM Button T Light (blue)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEcmT(Led<17536, 32768, 15>);
    impl LightEcmT {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEcmT {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEcmT {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Edge - Light (red)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEdge(Led<17526, 256, 8>);
    impl LightEdge {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEdge {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEdge {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// EEC Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEec(Led<17524, 4096, 12>);
    impl LightEec {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEec {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEec {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Elec - Light (red)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightElec(Led<17530, 8, 3>);
    impl LightElec {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightElec {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightElec {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ELEC SYS Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightElecSys(Led<17522, 2048, 11>);
    impl LightElecSys {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightElecSys {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightElecSys {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ENG FIRE Light (red)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEngFire(Led<17526, 1024, 10>);
    impl LightEngFire {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEngFire {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEngFire {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ENGINE Light (red)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEngine(Led<17526, 2048, 11>);
    impl LightEngine {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEngine {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEngine {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ENGINE FAULT Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEngineFault(Led<17522, 256, 8>);
    impl LightEngineFault {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEngineFault {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEngineFault {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// EPU Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEpu(Led<17528, 32768, 15>);
    impl LightEpu {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEpu {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEpu {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// EPU GEN Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEpuGen(Led<17530, 16, 4>);
    impl LightEpuGen {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEpuGen {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEpuGen {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// EPU PMG Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEpuPmg(Led<17530, 32, 5>);
    impl LightEpuPmg {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEpuPmg {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEpuPmg {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// EQUIP HOT Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightEquipHot(Led<17522, 8192, 13>);
    impl LightEquipHot {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightEquipHot {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightEquipHot {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FL FAIL Light (red)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightFlFail(Led<17530, 4096, 12>);
    impl LightFlFail {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightFlFail {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightFlFail {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FL RUN Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightFlRun(Led<17530, 2048, 11>);
    impl LightFlRun {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightFlRun {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightFlRun {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FLCS Light (red)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightFlcs(Led<17526, 8192, 13>);
    impl LightFlcs {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightFlcs {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightFlcs {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FLCS FAULT Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightFlcsFault(Led<17522, 128, 7>);
    impl LightFlcsFault {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightFlcsFault {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightFlcsFault {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FLCS PMG Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightFlcsPmg(Led<17530, 1, 0>);
    impl LightFlcsPmg {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightFlcsPmg {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightFlcsPmg {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FLCS PWR A Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightFlcsPwrA(Led<17530, 8192, 13>);
    impl LightFlcsPwrA {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightFlcsPwrA {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightFlcsPwrA {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FLCS PWR B Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightFlcsPwrB(Led<17530, 16384, 14>);
    impl LightFlcsPwrB {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightFlcsPwrB {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightFlcsPwrB {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FLCS PWR C Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightFlcsPwrC(Led<17530, 32768, 15>);
    impl LightFlcsPwrC {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightFlcsPwrC {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightFlcsPwrC {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FLCS PWR D Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightFlcsPwrD(Led<17532, 1, 0>);
    impl LightFlcsPwrD {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightFlcsPwrD {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightFlcsPwrD {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FLCS RLY Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightFlcsRly(Led<17530, 128, 7>);
    impl LightFlcsRly {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightFlcsRly {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightFlcsRly {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FUEL OIL HOT Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightFuelOilHot(Led<17524, 1, 0>);
    impl LightFuelOilHot {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightFuelOilHot {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightFuelOilHot {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FWD FUEL LOW Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightFwdFuelLow(Led<17524, 32768, 15>);
    impl LightFwdFuelLow {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightFwdFuelLow {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightFwdFuelLow {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Left Gear Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightGearL(Led<17528, 64, 6>);
    impl LightGearL {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightGearL {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightGearL {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Nose Gear Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightGearN(Led<17528, 32, 5>);
    impl LightGearN {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightGearN {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightGearN {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Right Gear Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightGearR(Led<17528, 128, 7>);
    impl LightGearR {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightGearR {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightGearR {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Gear Warning Light (red)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightGearWarn(Led<17528, 256, 8>);
    impl LightGearWarn {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightGearWarn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightGearWarn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// HOOK Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightHook(Led<17524, 64, 6>);
    impl LightHook {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightHook {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightHook {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// HYD_OIL_PRESS Light (red)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightHydOilPress(Led<17526, 4096, 12>);
    impl LightHydOilPress {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightHydOilPress {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightHydOilPress {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// HYDRAZN Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightHydrazn(Led<17528, 8192, 13>);
    impl LightHydrazn {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightHydrazn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightHydrazn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// IFF Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightIff(Led<17524, 32, 5>);
    impl LightIff {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightIff {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightIff {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// INLET ICING Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightInletIcing(Led<17524, 16, 4>);
    impl LightInletIcing {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightInletIcing {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightInletIcing {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Instrumen Panel Lightning (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightInstPnl(Led<17534, 32, 5>);
    impl LightInstPnl {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightInstPnl {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightInstPnl {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Instrumen Panel Flood Lightning (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightInstPnlFlood(Led<17534, 128, 7>);
    impl LightInstPnlFlood {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightInstPnlFlood {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightInstPnlFlood {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// JFS RUN Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightJfsRun(Led<17528, 4096, 12>);
    impl LightJfsRun {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightJfsRun {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightJfsRun {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// MAIN GEN Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightMainGen(Led<17530, 2, 1>);
    impl LightMainGen {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightMainGen {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightMainGen {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// MARKER BEACON Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightMarkerBeacon(Led<17538, 256, 8>);
    impl LightMarkerBeacon {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightMarkerBeacon {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightMarkerBeacon {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// MASTER CAUTION Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightMasterCaution(Led<17526, 128, 7>);
    impl LightMasterCaution {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightMasterCaution {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightMasterCaution {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// NUCLEAR Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightNuclear(Led<17524, 512, 9>);
    impl LightNuclear {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightNuclear {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightNuclear {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// NWS FAIL Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightNwsFail(Led<17522, 16384, 14>);
    impl LightNwsFail {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightNwsFail {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightNwsFail {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// OBOGS Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightObogs(Led<17524, 1024, 10>);
    impl LightObogs {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightObogs {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightObogs {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// OVERHEAT Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightOverheat(Led<17524, 256, 8>);
    impl LightOverheat {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightOverheat {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightOverheat {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// OXY LOW Light (red)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightOxyLow(Led<17528, 2, 1>);
    impl LightOxyLow {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightOxyLow {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightOxyLow {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// PROBE HEAT Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightProbeHeat(Led<17522, 32768, 15>);
    impl LightProbeHeat {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightProbeHeat {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightProbeHeat {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RADAR ALT Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightRadarAlt(Led<17524, 2, 1>);
    impl LightRadarAlt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightRadarAlt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightRadarAlt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RDY Light (blue)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightRdy(Led<17528, 512, 9>);
    impl LightRdy {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightRdy {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightRdy {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RWR ACT POWER Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightRwrActPower(Led<17532, 8, 3>);
    impl LightRwrActPower {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightRwrActPower {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightRwrActPower {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RWR ACTIVITY Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightRwrActivity(Led<17532, 4, 2>);
    impl LightRwrActivity {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightRwrActivity {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightRwrActivity {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RWR ALT Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightRwrAlt(Led<17532, 32, 5>);
    impl LightRwrAlt {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightRwrAlt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightRwrAlt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RWR ALT LOW Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightRwrAltLow(Led<17532, 16, 4>);
    impl LightRwrAltLow {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightRwrAltLow {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightRwrAltLow {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RWR HANDOFF H Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightRwrHandoffH(Led<17532, 256, 8>);
    impl LightRwrHandoffH {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightRwrHandoffH {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightRwrHandoffH {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RWR HANDOFF UP Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightRwrHandoffUp(Led<17532, 128, 7>);
    impl LightRwrHandoffUp {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightRwrHandoffUp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightRwrHandoffUp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RWR MODE OPEN Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightRwrModeOpen(Led<17532, 2048, 11>);
    impl LightRwrModeOpen {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightRwrModeOpen {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightRwrModeOpen {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RWR MODE PRI Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightRwrModePri(Led<17532, 1024, 10>);
    impl LightRwrModePri {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightRwrModePri {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightRwrModePri {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RWR MSL_LAUNCH Light (red)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightRwrMslLaunch(Led<17532, 512, 9>);
    impl LightRwrMslLaunch {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightRwrMslLaunch {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightRwrMslLaunch {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RWR POWER Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightRwrPower(Led<17532, 64, 6>);
    impl LightRwrPower {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightRwrPower {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightRwrPower {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RWR SEARCH Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightRwrSearch(Led<17532, 2, 1>);
    impl LightRwrSearch {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightRwrSearch {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightRwrSearch {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RWR SHIP UNKNOWN Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightRwrShipUnk(Led<17532, 4096, 12>);
    impl LightRwrShipUnk {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightRwrShipUnk {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightRwrShipUnk {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RWR SYSTEST Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightRwrSystest(Led<17532, 8192, 13>);
    impl LightRwrSystest {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightRwrSystest {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightRwrSystest {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RWR TGTSEP DOWN Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightRwrTgtsepDn(Led<17532, 32768, 15>);
    impl LightRwrTgtsepDn {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightRwrTgtsepDn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightRwrTgtsepDn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RWR TGTSEP UP Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightRwrTgtsepUp(Led<17532, 16384, 14>);
    impl LightRwrTgtsepUp {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightRwrTgtsepUp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightRwrTgtsepUp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// SEAT NOT ARMED Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightSeatNot(Led<17522, 1024, 10>);
    impl LightSeatNot {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightSeatNot {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightSeatNot {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// SEC Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightSec(Led<17522, 4096, 12>);
    impl LightSec {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightSec {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightSec {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// STBY Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightStby(Led<17530, 1024, 10>);
    impl LightStby {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightStby {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightStby {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// STBY GEN Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightStbyGen(Led<17530, 4, 2>);
    impl LightStbyGen {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightStbyGen {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightStbyGen {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// STORES CONFIG Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightStoresConfig(Led<17524, 128, 7>);
    impl LightStoresConfig {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightStoresConfig {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightStoresConfig {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// TF FAIL Light (red)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightTfFail(Led<17526, 512, 9>);
    impl LightTfFail {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightTfFail {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightTfFail {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// TO FLCS Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightToFlcs(Led<17530, 64, 6>);
    impl LightToFlcs {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightToFlcs {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightToFlcs {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// TO LDG CONFIG Light (red)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LightToLdgConfig(Led<17526, 32768, 15>);
    impl LightToLdgConfig {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LightToLdgConfig {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightToLdgConfig {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub enum RegisteredControl<'a> {
    AbResetSw {
        control: engine::AbResetSw,
        callback: IntegerCallback<'a>,
    },
    AdiAuxFlag {
        control: adi::AdiAuxFlag,
        callback: IntegerCallback<'a>,
    },
    AdiBank {
        control: adi::AdiBank,
        callback: IntegerCallback<'a>,
    },
    AdiBubble {
        control: adi::AdiBubble,
        callback: IntegerCallback<'a>,
    },
    AdiGsBar {
        control: adi::AdiGsBar,
        callback: IntegerCallback<'a>,
    },
    AdiGsFlag {
        control: adi::AdiGsFlag,
        callback: IntegerCallback<'a>,
    },
    AdiGsPointer {
        control: adi::AdiGsPointer,
        callback: IntegerCallback<'a>,
    },
    AdiLocBar {
        control: adi::AdiLocBar,
        callback: IntegerCallback<'a>,
    },
    AdiLocFlag {
        control: adi::AdiLocFlag,
        callback: IntegerCallback<'a>,
    },
    AdiOffFlag {
        control: adi::AdiOffFlag,
        callback: IntegerCallback<'a>,
    },
    AdiPitch {
        control: adi::AdiPitch,
        callback: IntegerCallback<'a>,
    },
    AdiPitchTrim {
        control: adi::AdiPitchTrim,
        callback: IntegerCallback<'a>,
    },
    AdiTurnrate {
        control: adi::AdiTurnrate,
        callback: IntegerCallback<'a>,
    },
    AdvModeSw {
        control: controlinterface::AdvModeSw,
        callback: controlinterface::AdvModeSwValueCallback<'a>,
    },
    AirRefuelLightKnb {
        control: externallights::AirRefuelLightKnb,
        callback: IntegerCallback<'a>,
    },
    AirRefuelSw {
        control: fuelsystem::AirRefuelSw,
        callback: fuelsystem::AirRefuelSwValueCallback<'a>,
    },
    AirSourceKnb {
        control: ecs::AirSourceKnb,
        callback: ecs::AirSourceKnbValueCallback<'a>,
    },
    Airspeed {
        control: airspeedindicator::Airspeed,
        callback: IntegerCallback<'a>,
    },
    AirspeedSetKnb {
        control: airspeedindicator::AirspeedSetKnb,
        callback: IntegerCallback<'a>,
    },
    Alt10000FtCnt {
        control: altimeter::Alt10000FtCnt,
        callback: IntegerCallback<'a>,
    },
    Alt1000FtCnt {
        control: altimeter::Alt1000FtCnt,
        callback: IntegerCallback<'a>,
    },
    Alt100FtCnt {
        control: altimeter::Alt100FtCnt,
        callback: IntegerCallback<'a>,
    },
    Alt100FtPtr {
        control: altimeter::Alt100FtPtr,
        callback: IntegerCallback<'a>,
    },
    AltBaroSetKnb {
        control: altimeter::AltBaroSetKnb,
        callback: IntegerCallback<'a>,
    },
    AltFlapsSw {
        control: controlinterface::AltFlapsSw,
        callback: controlinterface::AltFlapsSwValueCallback<'a>,
    },
    AltModeLv {
        control: altimeter::AltModeLv,
        callback: IntegerCallback<'a>,
    },
    AltPneuFlag {
        control: altimeter::AltPneuFlag,
        callback: IntegerCallback<'a>,
    },
    AltPressureDrum0Cnt {
        control: altimeter::AltPressureDrum0Cnt,
        callback: IntegerCallback<'a>,
    },
    AltPressureDrum1Cnt {
        control: altimeter::AltPressureDrum1Cnt,
        callback: IntegerCallback<'a>,
    },
    AltPressureDrum2Cnt {
        control: altimeter::AltPressureDrum2Cnt,
        callback: IntegerCallback<'a>,
    },
    AltPressureDrum3Cnt {
        control: altimeter::AltPressureDrum3Cnt,
        callback: IntegerCallback<'a>,
    },
    AltRelBtn {
        control: mmc::AltRelBtn,
        callback: mmc::AltRelBtnValueCallback<'a>,
    },
    AntiCollLightKnb {
        control: externallights::AntiCollLightKnb,
        callback: externallights::AntiCollLightKnbValueCallback<'a>,
    },
    AntiSkidSw {
        control: gearsystem::AntiSkidSw,
        callback: IntegerCallback<'a>,
    },
    AoaIndexBrtKnb {
        control: interiorlights::AoaIndexBrtKnb,
        callback: IntegerCallback<'a>,
    },
    AoaValue {
        control: aoa::AoaValue,
        callback: IntegerCallback<'a>,
    },
    ApPitchSw {
        control: controlinterface::ApPitchSw,
        callback: IntegerCallback<'a>,
    },
    ApRollSw {
        control: controlinterface::ApRollSw,
        callback: controlinterface::ApRollSwValueCallback<'a>,
    },
    ArStatusBrtKnb {
        control: interiorlights::ArStatusBrtKnb,
        callback: IntegerCallback<'a>,
    },
    BitSw {
        control: controlinterface::BitSw,
        callback: controlinterface::BitSwValueCallback<'a>,
    },
    BrakeChanSw {
        control: gearsystem::BrakeChanSw,
        callback: gearsystem::BrakeChanSwValueCallback<'a>,
    },
    CanopyHandle {
        control: cockpitmechanics::CanopyHandle,
        callback: cockpitmechanics::CanopyHandleValueCallback<'a>,
    },
    CanopyJettThandle {
        control: cockpitmechanics::CanopyJettThandle,
        callback: cockpitmechanics::CanopyJettThandleValueCallback<'a>,
    },
    CanopySw {
        control: cockpitmechanics::CanopySw,
        callback: IntegerCallback<'a>,
    },
    CanopyValue {
        control: cockpitmechanics::CanopyValue,
        callback: IntegerCallback<'a>,
    },
    ClockCurrtimeH {
        control: clock::ClockCurrtimeH,
        callback: IntegerCallback<'a>,
    },
    ClockCurrtimeMs {
        control: clock::ClockCurrtimeMs,
        callback: IntegerCallback<'a>,
    },
    ClockElapsed {
        control: clock::ClockElapsed,
        callback: clock::ClockElapsedValueCallback<'a>,
    },
    ClockElapsedTimeM {
        control: clock::ClockElapsedTimeM,
        callback: IntegerCallback<'a>,
    },
    ClockElapsedTimeSec {
        control: clock::ClockElapsedTimeSec,
        callback: IntegerCallback<'a>,
    },
    ClockSet {
        control: clock::ClockSet,
        callback: clock::ClockSetValueCallback<'a>,
    },
    ClockWind {
        control: clock::ClockWind,
        callback: IntegerCallback<'a>,
    },
    Cmds01ExpCatSw {
        control: cmds::Cmds01ExpCatSw,
        callback: cmds::Cmds01ExpCatSwValueCallback<'a>,
    },
    Cmds02ExpCatSw {
        control: cmds::Cmds02ExpCatSw,
        callback: cmds::Cmds02ExpCatSwValueCallback<'a>,
    },
    CmdsChAmount {
        control: cmds::CmdsChAmount,
        callback: DisplayCallback<'a>,
    },
    CmdsChExpCatSw {
        control: cmds::CmdsChExpCatSw,
        callback: cmds::CmdsChExpCatSwValueCallback<'a>,
    },
    CmdsDispenseBtn {
        control: cmds::CmdsDispenseBtn,
        callback: cmds::CmdsDispenseBtnValueCallback<'a>,
    },
    CmdsFlAmount {
        control: cmds::CmdsFlAmount,
        callback: DisplayCallback<'a>,
    },
    CmdsFlExpCatSw {
        control: cmds::CmdsFlExpCatSw,
        callback: cmds::CmdsFlExpCatSwValueCallback<'a>,
    },
    CmdsJettSw {
        control: cmds::CmdsJettSw,
        callback: cmds::CmdsJettSwValueCallback<'a>,
    },
    CmdsJmrSourcheSw {
        control: cmds::CmdsJmrSourcheSw,
        callback: cmds::CmdsJmrSourcheSwValueCallback<'a>,
    },
    CmdsModeKnb {
        control: cmds::CmdsModeKnb,
        callback: cmds::CmdsModeKnbValueCallback<'a>,
    },
    CmdsMwsSourcheSw {
        control: cmds::CmdsMwsSourcheSw,
        callback: cmds::CmdsMwsSourcheSwValueCallback<'a>,
    },
    CmdsO1Amount {
        control: cmds::CmdsO1Amount,
        callback: DisplayCallback<'a>,
    },
    CmdsO2Amount {
        control: cmds::CmdsO2Amount,
        callback: DisplayCallback<'a>,
    },
    CmdsProgKnb {
        control: cmds::CmdsProgKnb,
        callback: cmds::CmdsProgKnbValueCallback<'a>,
    },
    CmdsPwrSourcheSw {
        control: cmds::CmdsPwrSourcheSw,
        callback: cmds::CmdsPwrSourcheSwValueCallback<'a>,
    },
    CockpitAlititude {
        control: oxygensystem::CockpitAlititude,
        callback: IntegerCallback<'a>,
    },
    Comm1ModeKnb {
        control: audiopanel::Comm1ModeKnb,
        callback: audiopanel::Comm1ModeKnbValueCallback<'a>,
    },
    Comm1PwrKnb {
        control: audiopanel::Comm1PwrKnb,
        callback: IntegerCallback<'a>,
    },
    Comm2ModeKnb {
        control: audiopanel::Comm2ModeKnb,
        callback: audiopanel::Comm2ModeKnbValueCallback<'a>,
    },
    Comm2PwrKnb {
        control: audiopanel::Comm2PwrKnb,
        callback: IntegerCallback<'a>,
    },
    DedLine1 {
        control: dedoutputdata::DedLine1,
        callback: DisplayCallback<'a>,
    },
    DedLine2 {
        control: dedoutputdata::DedLine2,
        callback: DisplayCallback<'a>,
    },
    DedLine3 {
        control: dedoutputdata::DedLine3,
        callback: DisplayCallback<'a>,
    },
    DedLine4 {
        control: dedoutputdata::DedLine4,
        callback: DisplayCallback<'a>,
    },
    DedLine5 {
        control: dedoutputdata::DedLine5,
        callback: DisplayCallback<'a>,
    },
    DefogLever {
        control: ecs::DefogLever,
        callback: ecs::DefogLeverValueCallback<'a>,
    },
    DigiBakSw {
        control: controlinterface::DigiBakSw,
        callback: controlinterface::DigiBakSwValueCallback<'a>,
    },
    DlSw {
        control: avionicpanel::DlSw,
        callback: avionicpanel::DlSwValueCallback<'a>,
    },
    DnLockBtn {
        control: gearsystem::DnLockBtn,
        callback: gearsystem::DnLockBtnValueCallback<'a>,
    },
    Ecm1Btn {
        control: ecm::Ecm1Btn,
        callback: ecm::Ecm1BtnValueCallback<'a>,
    },
    Ecm2Btn {
        control: ecm::Ecm2Btn,
        callback: ecm::Ecm2BtnValueCallback<'a>,
    },
    Ecm3Btn {
        control: ecm::Ecm3Btn,
        callback: ecm::Ecm3BtnValueCallback<'a>,
    },
    Ecm4Btn {
        control: ecm::Ecm4Btn,
        callback: ecm::Ecm4BtnValueCallback<'a>,
    },
    Ecm5Btn {
        control: ecm::Ecm5Btn,
        callback: ecm::Ecm5BtnValueCallback<'a>,
    },
    Ecm6Btn {
        control: ecm::Ecm6Btn,
        callback: ecm::Ecm6BtnValueCallback<'a>,
    },
    EcmBitBtn {
        control: ecm::EcmBitBtn,
        callback: ecm::EcmBitBtnValueCallback<'a>,
    },
    EcmDimKnb {
        control: adi::EcmDimKnb,
        callback: IntegerCallback<'a>,
    },
    EcmFrmBtn {
        control: ecm::EcmFrmBtn,
        callback: ecm::EcmFrmBtnValueCallback<'a>,
    },
    EcmPwSw {
        control: ecm::EcmPwSw,
        callback: ecm::EcmPwSwValueCallback<'a>,
    },
    EcmResetBtn {
        control: ecm::EcmResetBtn,
        callback: ecm::EcmResetBtnValueCallback<'a>,
    },
    EcmSplBtn {
        control: ecm::EcmSplBtn,
        callback: ecm::EcmSplBtnValueCallback<'a>,
    },
    EcmXmitSw {
        control: ecm::EcmXmitSw,
        callback: ecm::EcmXmitSwValueCallback<'a>,
    },
    EhsiCrsSet {
        control: ehsi::EhsiCrsSet,
        callback: ehsi::EhsiCrsSetValueCallback<'a>,
    },
    EhsiCrsSetKnb {
        control: ehsi::EhsiCrsSetKnb,
        callback: IntegerCallback<'a>,
    },
    EhsiHdgSetBtn {
        control: ehsi::EhsiHdgSetBtn,
        callback: ehsi::EhsiHdgSetBtnValueCallback<'a>,
    },
    EhsiHdgSetKnb {
        control: ehsi::EhsiHdgSetKnb,
        callback: IntegerCallback<'a>,
    },
    EhsiMode {
        control: ehsi::EhsiMode,
        callback: ehsi::EhsiModeValueCallback<'a>,
    },
    ElecCaution {
        control: electricsystem::ElecCaution,
        callback: electricsystem::ElecCautionValueCallback<'a>,
    },
    EmergStroreJett {
        control: mmc::EmergStroreJett,
        callback: mmc::EmergStroreJettValueCallback<'a>,
    },
    EngAntiIce {
        control: engine::EngAntiIce,
        callback: engine::EngAntiIceValueCallback<'a>,
    },
    EngContSw {
        control: engine::EngContSw,
        callback: engine::EngContSwValueCallback<'a>,
    },
    EngContSwCover {
        control: engine::EngContSwCover,
        callback: engine::EngContSwCoverValueCallback<'a>,
    },
    EngineFeedKnb {
        control: externallights::EngineFeedKnb,
        callback: externallights::EngineFeedKnbValueCallback<'a>,
    },
    EngineFtit {
        control: engine::EngineFtit,
        callback: IntegerCallback<'a>,
    },
    EngineNozzlePosition {
        control: engine::EngineNozzlePosition,
        callback: IntegerCallback<'a>,
    },
    EngineOilPressure {
        control: engine::EngineOilPressure,
        callback: IntegerCallback<'a>,
    },
    EngineTachometer {
        control: engine::EngineTachometer,
        callback: IntegerCallback<'a>,
    },
    EpuGenTestSw {
        control: electricsystem::EpuGenTestSw,
        callback: electricsystem::EpuGenTestSwValueCallback<'a>,
    },
    EpuSw {
        control: epu::EpuSw,
        callback: epu::EpuSwValueCallback<'a>,
    },
    EpuSwCoverOff {
        control: epu::EpuSwCoverOff,
        callback: epu::EpuSwCoverOffValueCallback<'a>,
    },
    EpuSwCoverOn {
        control: epu::EpuSwCoverOn,
        callback: epu::EpuSwCoverOnValueCallback<'a>,
    },
    ExtFormationLights {
        control: externalaircraftmodel::ExtFormationLights,
        callback: IntegerCallback<'a>,
    },
    ExtFuelTransSw {
        control: fuelsystem::ExtFuelTransSw,
        callback: fuelsystem::ExtFuelTransSwValueCallback<'a>,
    },
    ExtPositionLightFuselage {
        control: externalaircraftmodel::ExtPositionLightFuselage,
        callback: IntegerCallback<'a>,
    },
    ExtPositionLightsWing {
        control: externalaircraftmodel::ExtPositionLightsWing,
        callback: IntegerCallback<'a>,
    },
    ExtSpeedBrakeLeft {
        control: externalaircraftmodel::ExtSpeedBrakeLeft,
        callback: IntegerCallback<'a>,
    },
    ExtSpeedBrakeRight {
        control: externalaircraftmodel::ExtSpeedBrakeRight,
        callback: IntegerCallback<'a>,
    },
    ExtStrobeTail {
        control: externalaircraftmodel::ExtStrobeTail,
        callback: IntegerCallback<'a>,
    },
    ExtTailLight {
        control: externalaircraftmodel::ExtTailLight,
        callback: IntegerCallback<'a>,
    },
    ExtWowLeft {
        control: externalaircraftmodel::ExtWowLeft,
        callback: IntegerCallback<'a>,
    },
    ExtWowNose {
        control: externalaircraftmodel::ExtWowNose,
        callback: IntegerCallback<'a>,
    },
    ExtWowRight {
        control: externalaircraftmodel::ExtWowRight,
        callback: IntegerCallback<'a>,
    },
    FAckBtn {
        control: ufc::FAckBtn,
        callback: ufc::FAckBtnValueCallback<'a>,
    },
    FcrPwrSw {
        control: sensorpanel::FcrPwrSw,
        callback: sensorpanel::FcrPwrSwValueCallback<'a>,
    },
    FireOheatDetectBtn {
        control: engine::FireOheatDetectBtn,
        callback: engine::FireOheatDetectBtnValueCallback<'a>,
    },
    FlcsPwrTestSw {
        control: electricsystem::FlcsPwrTestSw,
        callback: IntegerCallback<'a>,
    },
    FlcsResetSw {
        control: controlinterface::FlcsResetSw,
        callback: controlinterface::FlcsResetSwValueCallback<'a>,
    },
    FloodConsolesBrtKnb {
        control: interiorlights::FloodConsolesBrtKnb,
        callback: IntegerCallback<'a>,
    },
    FloodInstPnlBrtKnb {
        control: interiorlights::FloodInstPnlBrtKnb,
        callback: IntegerCallback<'a>,
    },
    FlowIndicator {
        control: oxygensystem::FlowIndicator,
        callback: IntegerCallback<'a>,
    },
    FlowIndicatorLight {
        control: oxygensystem::FlowIndicatorLight,
        callback: LedCallback<'a>,
    },
    FormLightKnb {
        control: externallights::FormLightKnb,
        callback: IntegerCallback<'a>,
    },
    FuelAl {
        control: fuelsystem::FuelAl,
        callback: IntegerCallback<'a>,
    },
    FuelFr {
        control: fuelsystem::FuelFr,
        callback: IntegerCallback<'a>,
    },
    FuelMasterCv {
        control: fuelsystem::FuelMasterCv,
        callback: fuelsystem::FuelMasterCvValueCallback<'a>,
    },
    FuelMasterSw {
        control: fuelsystem::FuelMasterSw,
        callback: fuelsystem::FuelMasterSwValueCallback<'a>,
    },
    FuelQtySelKnb {
        control: fuelsystem::FuelQtySelKnb,
        callback: fuelsystem::FuelQtySelKnbValueCallback<'a>,
    },
    FuelQtySelTKnb {
        control: fuelsystem::FuelQtySelTKnb,
        callback: IntegerCallback<'a>,
    },
    Fuelflowcounter100 {
        control: fuelsystem::Fuelflowcounter100,
        callback: IntegerCallback<'a>,
    },
    Fuelflowcounter10k {
        control: fuelsystem::Fuelflowcounter10k,
        callback: IntegerCallback<'a>,
    },
    Fuelflowcounter1k {
        control: fuelsystem::Fuelflowcounter1k,
        callback: IntegerCallback<'a>,
    },
    Fueltotalizer100 {
        control: fuelsystem::Fueltotalizer100,
        callback: IntegerCallback<'a>,
    },
    Fueltotalizer10k {
        control: fuelsystem::Fueltotalizer10k,
        callback: IntegerCallback<'a>,
    },
    Fueltotalizer1k {
        control: fuelsystem::Fueltotalizer1k,
        callback: IntegerCallback<'a>,
    },
    GearHandle {
        control: gearsystem::GearHandle,
        callback: gearsystem::GearHandleValueCallback<'a>,
    },
    GndJettEnableSw {
        control: mmc::GndJettEnableSw,
        callback: mmc::GndJettEnableSwValueCallback<'a>,
    },
    GpsSw {
        control: avionicpanel::GpsSw,
        callback: avionicpanel::GpsSwValueCallback<'a>,
    },
    HdptSwL {
        control: sensorpanel::HdptSwL,
        callback: sensorpanel::HdptSwLValueCallback<'a>,
    },
    HdptSwR {
        control: sensorpanel::HdptSwR,
        callback: sensorpanel::HdptSwRValueCallback<'a>,
    },
    HideStick {
        control: cockpitmechanics::HideStick,
        callback: cockpitmechanics::HideStickValueCallback<'a>,
    },
    HmcsIntKnb {
        control: hmcs::HmcsIntKnb,
        callback: IntegerCallback<'a>,
    },
    HookSw {
        control: gearsystem::HookSw,
        callback: gearsystem::HookSwValueCallback<'a>,
    },
    HornSilenceBtn {
        control: gearsystem::HornSilenceBtn,
        callback: gearsystem::HornSilenceBtnValueCallback<'a>,
    },
    HotMicSw {
        control: audiopanel::HotMicSw,
        callback: audiopanel::HotMicSwValueCallback<'a>,
    },
    HudAltSw {
        control: hudcontrolpanel::HudAltSw,
        callback: hudcontrolpanel::HudAltSwValueCallback<'a>,
    },
    HudBrtSw {
        control: hudcontrolpanel::HudBrtSw,
        callback: hudcontrolpanel::HudBrtSwValueCallback<'a>,
    },
    HudDedDataSw {
        control: hudcontrolpanel::HudDedDataSw,
        callback: hudcontrolpanel::HudDedDataSwValueCallback<'a>,
    },
    HudDepressRetSw {
        control: hudcontrolpanel::HudDepressRetSw,
        callback: hudcontrolpanel::HudDepressRetSwValueCallback<'a>,
    },
    HudFpMarkerSw {
        control: hudcontrolpanel::HudFpMarkerSw,
        callback: hudcontrolpanel::HudFpMarkerSwValueCallback<'a>,
    },
    HudScalesSw {
        control: hudcontrolpanel::HudScalesSw,
        callback: hudcontrolpanel::HudScalesSwValueCallback<'a>,
    },
    HudSpeedSw {
        control: hudcontrolpanel::HudSpeedSw,
        callback: hudcontrolpanel::HudSpeedSwValueCallback<'a>,
    },
    HudTestSw {
        control: hudcontrolpanel::HudTestSw,
        callback: hudcontrolpanel::HudTestSwValueCallback<'a>,
    },
    HydrazinVolume {
        control: epu::HydrazinVolume,
        callback: IntegerCallback<'a>,
    },
    IcpAaModeBtn {
        control: ufc::IcpAaModeBtn,
        callback: ufc::IcpAaModeBtnValueCallback<'a>,
    },
    IcpAgModeBtn {
        control: ufc::IcpAgModeBtn,
        callback: ufc::IcpAgModeBtnValueCallback<'a>,
    },
    IcpBtn0 {
        control: ufc::IcpBtn0,
        callback: ufc::IcpBtn0ValueCallback<'a>,
    },
    IcpBtn1 {
        control: ufc::IcpBtn1,
        callback: ufc::IcpBtn1ValueCallback<'a>,
    },
    IcpBtn2 {
        control: ufc::IcpBtn2,
        callback: ufc::IcpBtn2ValueCallback<'a>,
    },
    IcpBtn3 {
        control: ufc::IcpBtn3,
        callback: ufc::IcpBtn3ValueCallback<'a>,
    },
    IcpBtn4 {
        control: ufc::IcpBtn4,
        callback: ufc::IcpBtn4ValueCallback<'a>,
    },
    IcpBtn5 {
        control: ufc::IcpBtn5,
        callback: ufc::IcpBtn5ValueCallback<'a>,
    },
    IcpBtn6 {
        control: ufc::IcpBtn6,
        callback: ufc::IcpBtn6ValueCallback<'a>,
    },
    IcpBtn7 {
        control: ufc::IcpBtn7,
        callback: ufc::IcpBtn7ValueCallback<'a>,
    },
    IcpBtn8 {
        control: ufc::IcpBtn8,
        callback: ufc::IcpBtn8ValueCallback<'a>,
    },
    IcpBtn9 {
        control: ufc::IcpBtn9,
        callback: ufc::IcpBtn9ValueCallback<'a>,
    },
    IcpCom1Btn {
        control: ufc::IcpCom1Btn,
        callback: ufc::IcpCom1BtnValueCallback<'a>,
    },
    IcpCom2Btn {
        control: ufc::IcpCom2Btn,
        callback: ufc::IcpCom2BtnValueCallback<'a>,
    },
    IcpDataRtnSeqSw {
        control: ufc::IcpDataRtnSeqSw,
        callback: IntegerCallback<'a>,
    },
    IcpDataUpDnSw {
        control: ufc::IcpDataUpDnSw,
        callback: IntegerCallback<'a>,
    },
    IcpDedSw {
        control: ufc::IcpDedSw,
        callback: IntegerCallback<'a>,
    },
    IcpDriftSw {
        control: ufc::IcpDriftSw,
        callback: IntegerCallback<'a>,
    },
    IcpEntrBtn {
        control: ufc::IcpEntrBtn,
        callback: ufc::IcpEntrBtnValueCallback<'a>,
    },
    IcpFlirGainSw {
        control: ufc::IcpFlirGainSw,
        callback: ufc::IcpFlirGainSwValueCallback<'a>,
    },
    IcpFlirSw {
        control: ufc::IcpFlirSw,
        callback: IntegerCallback<'a>,
    },
    IcpHudBrtKnb {
        control: ufc::IcpHudBrtKnb,
        callback: IntegerCallback<'a>,
    },
    IcpIffBtn {
        control: ufc::IcpIffBtn,
        callback: ufc::IcpIffBtnValueCallback<'a>,
    },
    IcpListBtn {
        control: ufc::IcpListBtn,
        callback: ufc::IcpListBtnValueCallback<'a>,
    },
    IcpRasterBrtKnb {
        control: ufc::IcpRasterBrtKnb,
        callback: IntegerCallback<'a>,
    },
    IcpRasterContrKnb {
        control: ufc::IcpRasterContrKnb,
        callback: IntegerCallback<'a>,
    },
    IcpRclBtn {
        control: ufc::IcpRclBtn,
        callback: ufc::IcpRclBtnValueCallback<'a>,
    },
    IcpReticleDepressKnb {
        control: ufc::IcpReticleDepressKnb,
        callback: IntegerCallback<'a>,
    },
    IcpWxBtn {
        control: ufc::IcpWxBtn,
        callback: ufc::IcpWxBtnValueCallback<'a>,
    },
    IffAntSelSw {
        control: audiopanel::IffAntSelSw,
        callback: audiopanel::IffAntSelSwValueCallback<'a>,
    },
    IffCIKnb {
        control: iff::IffCIKnb,
        callback: iff::IffCIKnbValueCallback<'a>,
    },
    IffCodeDrumDigit1 {
        control: iff::IffCodeDrumDigit1,
        callback: IntegerCallback<'a>,
    },
    IffCodeDrumDigit2 {
        control: iff::IffCodeDrumDigit2,
        callback: IntegerCallback<'a>,
    },
    IffCodeDrumDigit3 {
        control: iff::IffCodeDrumDigit3,
        callback: IntegerCallback<'a>,
    },
    IffCodeDrumDigit4 {
        control: iff::IffCodeDrumDigit4,
        callback: IntegerCallback<'a>,
    },
    IffEnableSw {
        control: iff::IffEnableSw,
        callback: iff::IffEnableSwValueCallback<'a>,
    },
    IffIdBtn {
        control: ufc::IffIdBtn,
        callback: ufc::IffIdBtnValueCallback<'a>,
    },
    IffM1Sel1 {
        control: iff::IffM1Sel1,
        callback: iff::IffM1Sel1ValueCallback<'a>,
    },
    IffM1Sel2 {
        control: iff::IffM1Sel2,
        callback: iff::IffM1Sel2ValueCallback<'a>,
    },
    IffM3Sel1 {
        control: iff::IffM3Sel1,
        callback: iff::IffM3Sel1ValueCallback<'a>,
    },
    IffM3Sel2 {
        control: iff::IffM3Sel2,
        callback: iff::IffM3Sel2ValueCallback<'a>,
    },
    IffM4CodeSw {
        control: iff::IffM4CodeSw,
        callback: iff::IffM4CodeSwValueCallback<'a>,
    },
    IffM4MonitorSw {
        control: iff::IffM4MonitorSw,
        callback: iff::IffM4MonitorSwValueCallback<'a>,
    },
    IffM4ReplySw {
        control: iff::IffM4ReplySw,
        callback: iff::IffM4ReplySwValueCallback<'a>,
    },
    IffMasterKnb {
        control: iff::IffMasterKnb,
        callback: iff::IffMasterKnbValueCallback<'a>,
    },
    IlsPwrKnb {
        control: audiopanel::IlsPwrKnb,
        callback: IntegerCallback<'a>,
    },
    InsKnb {
        control: avionicpanel::InsKnb,
        callback: avionicpanel::InsKnbValueCallback<'a>,
    },
    IntercomKnb {
        control: audiopanel::IntercomKnb,
        callback: IntegerCallback<'a>,
    },
    JfsSw {
        control: engine::JfsSw,
        callback: IntegerCallback<'a>,
    },
    Ky58FillKnb {
        control: ky58::Ky58FillKnb,
        callback: ky58::Ky58FillKnbValueCallback<'a>,
    },
    Ky58ModeKnb {
        control: ky58::Ky58ModeKnb,
        callback: ky58::Ky58ModeKnbValueCallback<'a>,
    },
    Ky58PwrKnb {
        control: ky58::Ky58PwrKnb,
        callback: ky58::Ky58PwrKnbValueCallback<'a>,
    },
    Ky58VolKnb {
        control: ky58::Ky58VolKnb,
        callback: IntegerCallback<'a>,
    },
    LandTaxiLightSw {
        control: externallights::LandTaxiLightSw,
        callback: externallights::LandTaxiLightSwValueCallback<'a>,
    },
    LaserArmSw {
        control: mmc::LaserArmSw,
        callback: mmc::LaserArmSwValueCallback<'a>,
    },
    LeFlapsSw {
        control: controlinterface::LeFlapsSw,
        callback: controlinterface::LeFlapsSwValueCallback<'a>,
    },
    LightAcftBattFail {
        control: warningcautionandindicatorlights::LightAcftBattFail,
        callback: LedCallback<'a>,
    },
    LightActive {
        control: warningcautionandindicatorlights::LightActive,
        callback: LedCallback<'a>,
    },
    LightAftFuelLow {
        control: warningcautionandindicatorlights::LightAftFuelLow,
        callback: LedCallback<'a>,
    },
    LightAir {
        control: warningcautionandindicatorlights::LightAir,
        callback: LedCallback<'a>,
    },
    LightAntiSkid {
        control: warningcautionandindicatorlights::LightAntiSkid,
        callback: LedCallback<'a>,
    },
    LightAoaDn {
        control: warningcautionandindicatorlights::LightAoaDn,
        callback: LedCallback<'a>,
    },
    LightAoaMid {
        control: warningcautionandindicatorlights::LightAoaMid,
        callback: LedCallback<'a>,
    },
    LightAoaUp {
        control: warningcautionandindicatorlights::LightAoaUp,
        callback: LedCallback<'a>,
    },
    LightArNws {
        control: warningcautionandindicatorlights::LightArNws,
        callback: LedCallback<'a>,
    },
    LightAtfNot {
        control: warningcautionandindicatorlights::LightAtfNot,
        callback: LedCallback<'a>,
    },
    LightAvionicsFault {
        control: warningcautionandindicatorlights::LightAvionicsFault,
        callback: LedCallback<'a>,
    },
    LightBuc {
        control: warningcautionandindicatorlights::LightBuc,
        callback: LedCallback<'a>,
    },
    LightCabinPress {
        control: warningcautionandindicatorlights::LightCabinPress,
        callback: LedCallback<'a>,
    },
    LightCadc {
        control: warningcautionandindicatorlights::LightCadc,
        callback: LedCallback<'a>,
    },
    LightCanopy {
        control: warningcautionandindicatorlights::LightCanopy,
        callback: LedCallback<'a>,
    },
    LightCaution1 {
        control: warningcautionandindicatorlights::LightCaution1,
        callback: LedCallback<'a>,
    },
    LightCaution2 {
        control: warningcautionandindicatorlights::LightCaution2,
        callback: LedCallback<'a>,
    },
    LightCaution3 {
        control: warningcautionandindicatorlights::LightCaution3,
        callback: LedCallback<'a>,
    },
    LightCaution4 {
        control: warningcautionandindicatorlights::LightCaution4,
        callback: LedCallback<'a>,
    },
    LightCaution5 {
        control: warningcautionandindicatorlights::LightCaution5,
        callback: LedCallback<'a>,
    },
    LightCaution6 {
        control: warningcautionandindicatorlights::LightCaution6,
        callback: LedCallback<'a>,
    },
    LightCmdsDisp {
        control: warningcautionandindicatorlights::LightCmdsDisp,
        callback: LedCallback<'a>,
    },
    LightCmdsGo {
        control: warningcautionandindicatorlights::LightCmdsGo,
        callback: LedCallback<'a>,
    },
    LightCmdsNoGo {
        control: warningcautionandindicatorlights::LightCmdsNoGo,
        callback: LedCallback<'a>,
    },
    LightCmdsRdy {
        control: warningcautionandindicatorlights::LightCmdsRdy,
        callback: LedCallback<'a>,
    },
    LightConsles {
        control: warningcautionandindicatorlights::LightConsles,
        callback: LedCallback<'a>,
    },
    LightConslesFlood {
        control: warningcautionandindicatorlights::LightConslesFlood,
        callback: LedCallback<'a>,
    },
    LightDbuOn {
        control: warningcautionandindicatorlights::LightDbuOn,
        callback: LedCallback<'a>,
    },
    LightDisc {
        control: warningcautionandindicatorlights::LightDisc,
        callback: LedCallback<'a>,
    },
    LightEcm1A {
        control: warningcautionandindicatorlights::LightEcm1A,
        callback: LedCallback<'a>,
    },
    LightEcm1F {
        control: warningcautionandindicatorlights::LightEcm1F,
        callback: LedCallback<'a>,
    },
    LightEcm1S {
        control: warningcautionandindicatorlights::LightEcm1S,
        callback: LedCallback<'a>,
    },
    LightEcm1T {
        control: warningcautionandindicatorlights::LightEcm1T,
        callback: LedCallback<'a>,
    },
    LightEcm2A {
        control: warningcautionandindicatorlights::LightEcm2A,
        callback: LedCallback<'a>,
    },
    LightEcm2F {
        control: warningcautionandindicatorlights::LightEcm2F,
        callback: LedCallback<'a>,
    },
    LightEcm2S {
        control: warningcautionandindicatorlights::LightEcm2S,
        callback: LedCallback<'a>,
    },
    LightEcm2T {
        control: warningcautionandindicatorlights::LightEcm2T,
        callback: LedCallback<'a>,
    },
    LightEcm3A {
        control: warningcautionandindicatorlights::LightEcm3A,
        callback: LedCallback<'a>,
    },
    LightEcm3F {
        control: warningcautionandindicatorlights::LightEcm3F,
        callback: LedCallback<'a>,
    },
    LightEcm3S {
        control: warningcautionandindicatorlights::LightEcm3S,
        callback: LedCallback<'a>,
    },
    LightEcm3T {
        control: warningcautionandindicatorlights::LightEcm3T,
        callback: LedCallback<'a>,
    },
    LightEcm4A {
        control: warningcautionandindicatorlights::LightEcm4A,
        callback: LedCallback<'a>,
    },
    LightEcm4F {
        control: warningcautionandindicatorlights::LightEcm4F,
        callback: LedCallback<'a>,
    },
    LightEcm4S {
        control: warningcautionandindicatorlights::LightEcm4S,
        callback: LedCallback<'a>,
    },
    LightEcm4T {
        control: warningcautionandindicatorlights::LightEcm4T,
        callback: LedCallback<'a>,
    },
    LightEcm5A {
        control: warningcautionandindicatorlights::LightEcm5A,
        callback: LedCallback<'a>,
    },
    LightEcm5F {
        control: warningcautionandindicatorlights::LightEcm5F,
        callback: LedCallback<'a>,
    },
    LightEcm5S {
        control: warningcautionandindicatorlights::LightEcm5S,
        callback: LedCallback<'a>,
    },
    LightEcm5T {
        control: warningcautionandindicatorlights::LightEcm5T,
        callback: LedCallback<'a>,
    },
    LightEcmA {
        control: warningcautionandindicatorlights::LightEcmA,
        callback: LedCallback<'a>,
    },
    LightEcmF {
        control: warningcautionandindicatorlights::LightEcmF,
        callback: LedCallback<'a>,
    },
    LightEcmFrmA {
        control: warningcautionandindicatorlights::LightEcmFrmA,
        callback: LedCallback<'a>,
    },
    LightEcmFrmF {
        control: warningcautionandindicatorlights::LightEcmFrmF,
        callback: LedCallback<'a>,
    },
    LightEcmFrmS {
        control: warningcautionandindicatorlights::LightEcmFrmS,
        callback: LedCallback<'a>,
    },
    LightEcmFrmT {
        control: warningcautionandindicatorlights::LightEcmFrmT,
        callback: LedCallback<'a>,
    },
    LightEcmS {
        control: warningcautionandindicatorlights::LightEcmS,
        callback: LedCallback<'a>,
    },
    LightEcmSplA {
        control: warningcautionandindicatorlights::LightEcmSplA,
        callback: LedCallback<'a>,
    },
    LightEcmSplF {
        control: warningcautionandindicatorlights::LightEcmSplF,
        callback: LedCallback<'a>,
    },
    LightEcmSplS {
        control: warningcautionandindicatorlights::LightEcmSplS,
        callback: LedCallback<'a>,
    },
    LightEcmSplT {
        control: warningcautionandindicatorlights::LightEcmSplT,
        callback: LedCallback<'a>,
    },
    LightEcmT {
        control: warningcautionandindicatorlights::LightEcmT,
        callback: LedCallback<'a>,
    },
    LightEdge {
        control: warningcautionandindicatorlights::LightEdge,
        callback: LedCallback<'a>,
    },
    LightEec {
        control: warningcautionandindicatorlights::LightEec,
        callback: LedCallback<'a>,
    },
    LightElec {
        control: warningcautionandindicatorlights::LightElec,
        callback: LedCallback<'a>,
    },
    LightElecSys {
        control: warningcautionandindicatorlights::LightElecSys,
        callback: LedCallback<'a>,
    },
    LightEngFire {
        control: warningcautionandindicatorlights::LightEngFire,
        callback: LedCallback<'a>,
    },
    LightEngine {
        control: warningcautionandindicatorlights::LightEngine,
        callback: LedCallback<'a>,
    },
    LightEngineFault {
        control: warningcautionandindicatorlights::LightEngineFault,
        callback: LedCallback<'a>,
    },
    LightEpu {
        control: warningcautionandindicatorlights::LightEpu,
        callback: LedCallback<'a>,
    },
    LightEpuGen {
        control: warningcautionandindicatorlights::LightEpuGen,
        callback: LedCallback<'a>,
    },
    LightEpuPmg {
        control: warningcautionandindicatorlights::LightEpuPmg,
        callback: LedCallback<'a>,
    },
    LightEquipHot {
        control: warningcautionandindicatorlights::LightEquipHot,
        callback: LedCallback<'a>,
    },
    LightFlFail {
        control: warningcautionandindicatorlights::LightFlFail,
        callback: LedCallback<'a>,
    },
    LightFlRun {
        control: warningcautionandindicatorlights::LightFlRun,
        callback: LedCallback<'a>,
    },
    LightFlcs {
        control: warningcautionandindicatorlights::LightFlcs,
        callback: LedCallback<'a>,
    },
    LightFlcsFault {
        control: warningcautionandindicatorlights::LightFlcsFault,
        callback: LedCallback<'a>,
    },
    LightFlcsPmg {
        control: warningcautionandindicatorlights::LightFlcsPmg,
        callback: LedCallback<'a>,
    },
    LightFlcsPwrA {
        control: warningcautionandindicatorlights::LightFlcsPwrA,
        callback: LedCallback<'a>,
    },
    LightFlcsPwrB {
        control: warningcautionandindicatorlights::LightFlcsPwrB,
        callback: LedCallback<'a>,
    },
    LightFlcsPwrC {
        control: warningcautionandindicatorlights::LightFlcsPwrC,
        callback: LedCallback<'a>,
    },
    LightFlcsPwrD {
        control: warningcautionandindicatorlights::LightFlcsPwrD,
        callback: LedCallback<'a>,
    },
    LightFlcsRly {
        control: warningcautionandindicatorlights::LightFlcsRly,
        callback: LedCallback<'a>,
    },
    LightFuelOilHot {
        control: warningcautionandindicatorlights::LightFuelOilHot,
        callback: LedCallback<'a>,
    },
    LightFwdFuelLow {
        control: warningcautionandindicatorlights::LightFwdFuelLow,
        callback: LedCallback<'a>,
    },
    LightGearL {
        control: warningcautionandindicatorlights::LightGearL,
        callback: LedCallback<'a>,
    },
    LightGearN {
        control: warningcautionandindicatorlights::LightGearN,
        callback: LedCallback<'a>,
    },
    LightGearR {
        control: warningcautionandindicatorlights::LightGearR,
        callback: LedCallback<'a>,
    },
    LightGearWarn {
        control: warningcautionandindicatorlights::LightGearWarn,
        callback: LedCallback<'a>,
    },
    LightHook {
        control: warningcautionandindicatorlights::LightHook,
        callback: LedCallback<'a>,
    },
    LightHydOilPress {
        control: warningcautionandindicatorlights::LightHydOilPress,
        callback: LedCallback<'a>,
    },
    LightHydrazn {
        control: warningcautionandindicatorlights::LightHydrazn,
        callback: LedCallback<'a>,
    },
    LightIff {
        control: warningcautionandindicatorlights::LightIff,
        callback: LedCallback<'a>,
    },
    LightInletIcing {
        control: warningcautionandindicatorlights::LightInletIcing,
        callback: LedCallback<'a>,
    },
    LightInstPnl {
        control: warningcautionandindicatorlights::LightInstPnl,
        callback: LedCallback<'a>,
    },
    LightInstPnlFlood {
        control: warningcautionandindicatorlights::LightInstPnlFlood,
        callback: LedCallback<'a>,
    },
    LightJfsRun {
        control: warningcautionandindicatorlights::LightJfsRun,
        callback: LedCallback<'a>,
    },
    LightMainGen {
        control: warningcautionandindicatorlights::LightMainGen,
        callback: LedCallback<'a>,
    },
    LightMarkerBeacon {
        control: warningcautionandindicatorlights::LightMarkerBeacon,
        callback: LedCallback<'a>,
    },
    LightMasterCaution {
        control: warningcautionandindicatorlights::LightMasterCaution,
        callback: LedCallback<'a>,
    },
    LightNuclear {
        control: warningcautionandindicatorlights::LightNuclear,
        callback: LedCallback<'a>,
    },
    LightNwsFail {
        control: warningcautionandindicatorlights::LightNwsFail,
        callback: LedCallback<'a>,
    },
    LightObogs {
        control: warningcautionandindicatorlights::LightObogs,
        callback: LedCallback<'a>,
    },
    LightOverheat {
        control: warningcautionandindicatorlights::LightOverheat,
        callback: LedCallback<'a>,
    },
    LightOxyLow {
        control: warningcautionandindicatorlights::LightOxyLow,
        callback: LedCallback<'a>,
    },
    LightProbeHeat {
        control: warningcautionandindicatorlights::LightProbeHeat,
        callback: LedCallback<'a>,
    },
    LightRadarAlt {
        control: warningcautionandindicatorlights::LightRadarAlt,
        callback: LedCallback<'a>,
    },
    LightRdy {
        control: warningcautionandindicatorlights::LightRdy,
        callback: LedCallback<'a>,
    },
    LightRwrActPower {
        control: warningcautionandindicatorlights::LightRwrActPower,
        callback: LedCallback<'a>,
    },
    LightRwrActivity {
        control: warningcautionandindicatorlights::LightRwrActivity,
        callback: LedCallback<'a>,
    },
    LightRwrAlt {
        control: warningcautionandindicatorlights::LightRwrAlt,
        callback: LedCallback<'a>,
    },
    LightRwrAltLow {
        control: warningcautionandindicatorlights::LightRwrAltLow,
        callback: LedCallback<'a>,
    },
    LightRwrHandoffH {
        control: warningcautionandindicatorlights::LightRwrHandoffH,
        callback: LedCallback<'a>,
    },
    LightRwrHandoffUp {
        control: warningcautionandindicatorlights::LightRwrHandoffUp,
        callback: LedCallback<'a>,
    },
    LightRwrModeOpen {
        control: warningcautionandindicatorlights::LightRwrModeOpen,
        callback: LedCallback<'a>,
    },
    LightRwrModePri {
        control: warningcautionandindicatorlights::LightRwrModePri,
        callback: LedCallback<'a>,
    },
    LightRwrMslLaunch {
        control: warningcautionandindicatorlights::LightRwrMslLaunch,
        callback: LedCallback<'a>,
    },
    LightRwrPower {
        control: warningcautionandindicatorlights::LightRwrPower,
        callback: LedCallback<'a>,
    },
    LightRwrSearch {
        control: warningcautionandindicatorlights::LightRwrSearch,
        callback: LedCallback<'a>,
    },
    LightRwrShipUnk {
        control: warningcautionandindicatorlights::LightRwrShipUnk,
        callback: LedCallback<'a>,
    },
    LightRwrSystest {
        control: warningcautionandindicatorlights::LightRwrSystest,
        callback: LedCallback<'a>,
    },
    LightRwrTgtsepDn {
        control: warningcautionandindicatorlights::LightRwrTgtsepDn,
        callback: LedCallback<'a>,
    },
    LightRwrTgtsepUp {
        control: warningcautionandindicatorlights::LightRwrTgtsepUp,
        callback: LedCallback<'a>,
    },
    LightSeatNot {
        control: warningcautionandindicatorlights::LightSeatNot,
        callback: LedCallback<'a>,
    },
    LightSec {
        control: warningcautionandindicatorlights::LightSec,
        callback: LedCallback<'a>,
    },
    LightStby {
        control: warningcautionandindicatorlights::LightStby,
        callback: LedCallback<'a>,
    },
    LightStbyGen {
        control: warningcautionandindicatorlights::LightStbyGen,
        callback: LedCallback<'a>,
    },
    LightStoresConfig {
        control: warningcautionandindicatorlights::LightStoresConfig,
        callback: LedCallback<'a>,
    },
    LightTfFail {
        control: warningcautionandindicatorlights::LightTfFail,
        callback: LedCallback<'a>,
    },
    LightToFlcs {
        control: warningcautionandindicatorlights::LightToFlcs,
        callback: LedCallback<'a>,
    },
    LightToLdgConfig {
        control: warningcautionandindicatorlights::LightToLdgConfig,
        callback: LedCallback<'a>,
    },
    MachIndicator {
        control: airspeedindicator::MachIndicator,
        callback: IntegerCallback<'a>,
    },
    MainPwrSw {
        control: electricsystem::MainPwrSw,
        callback: electricsystem::MainPwrSwValueCallback<'a>,
    },
    MalIndLtsBrtSw {
        control: externallights::MalIndLtsBrtSw,
        callback: IntegerCallback<'a>,
    },
    MalIndLtsTest {
        control: interiorlights::MalIndLtsTest,
        callback: interiorlights::MalIndLtsTestValueCallback<'a>,
    },
    ManTfFlyupSw {
        control: controlinterface::ManTfFlyupSw,
        callback: controlinterface::ManTfFlyupSwValueCallback<'a>,
    },
    ManualPitchSw {
        control: controlinterface::ManualPitchSw,
        callback: controlinterface::ManualPitchSwValueCallback<'a>,
    },
    MapSw {
        control: avionicpanel::MapSw,
        callback: avionicpanel::MapSwValueCallback<'a>,
    },
    MasterArmSw {
        control: mmc::MasterArmSw,
        callback: mmc::MasterArmSwValueCallback<'a>,
    },
    MasterCaution {
        control: interiorlights::MasterCaution,
        callback: interiorlights::MasterCautionValueCallback<'a>,
    },
    MasterLightSw {
        control: externallights::MasterLightSw,
        callback: externallights::MasterLightSwValueCallback<'a>,
    },
    MaxAirspeed {
        control: airspeedindicator::MaxAirspeed,
        callback: IntegerCallback<'a>,
    },
    MaxPwrSw {
        control: engine::MaxPwrSw,
        callback: engine::MaxPwrSwValueCallback<'a>,
    },
    MfdL1 {
        control: mfdleft::MfdL1,
        callback: mfdleft::MfdL1ValueCallback<'a>,
    },
    MfdL10 {
        control: mfdleft::MfdL10,
        callback: mfdleft::MfdL10ValueCallback<'a>,
    },
    MfdL11 {
        control: mfdleft::MfdL11,
        callback: mfdleft::MfdL11ValueCallback<'a>,
    },
    MfdL12 {
        control: mfdleft::MfdL12,
        callback: mfdleft::MfdL12ValueCallback<'a>,
    },
    MfdL13 {
        control: mfdleft::MfdL13,
        callback: mfdleft::MfdL13ValueCallback<'a>,
    },
    MfdL14 {
        control: mfdleft::MfdL14,
        callback: mfdleft::MfdL14ValueCallback<'a>,
    },
    MfdL15 {
        control: mfdleft::MfdL15,
        callback: mfdleft::MfdL15ValueCallback<'a>,
    },
    MfdL16 {
        control: mfdleft::MfdL16,
        callback: mfdleft::MfdL16ValueCallback<'a>,
    },
    MfdL17 {
        control: mfdleft::MfdL17,
        callback: mfdleft::MfdL17ValueCallback<'a>,
    },
    MfdL18 {
        control: mfdleft::MfdL18,
        callback: mfdleft::MfdL18ValueCallback<'a>,
    },
    MfdL19 {
        control: mfdleft::MfdL19,
        callback: mfdleft::MfdL19ValueCallback<'a>,
    },
    MfdL2 {
        control: mfdleft::MfdL2,
        callback: mfdleft::MfdL2ValueCallback<'a>,
    },
    MfdL20 {
        control: mfdleft::MfdL20,
        callback: mfdleft::MfdL20ValueCallback<'a>,
    },
    MfdL3 {
        control: mfdleft::MfdL3,
        callback: mfdleft::MfdL3ValueCallback<'a>,
    },
    MfdL4 {
        control: mfdleft::MfdL4,
        callback: mfdleft::MfdL4ValueCallback<'a>,
    },
    MfdL5 {
        control: mfdleft::MfdL5,
        callback: mfdleft::MfdL5ValueCallback<'a>,
    },
    MfdL6 {
        control: mfdleft::MfdL6,
        callback: mfdleft::MfdL6ValueCallback<'a>,
    },
    MfdL7 {
        control: mfdleft::MfdL7,
        callback: mfdleft::MfdL7ValueCallback<'a>,
    },
    MfdL8 {
        control: mfdleft::MfdL8,
        callback: mfdleft::MfdL8ValueCallback<'a>,
    },
    MfdL9 {
        control: mfdleft::MfdL9,
        callback: mfdleft::MfdL9ValueCallback<'a>,
    },
    MfdLBrtSw {
        control: mfdleft::MfdLBrtSw,
        callback: mfdleft::MfdLBrtSwValueCallback<'a>,
    },
    MfdLConSw {
        control: mfdleft::MfdLConSw,
        callback: mfdleft::MfdLConSwValueCallback<'a>,
    },
    MfdLGainSw {
        control: mfdleft::MfdLGainSw,
        callback: mfdleft::MfdLGainSwValueCallback<'a>,
    },
    MfdLSymSw {
        control: mfdleft::MfdLSymSw,
        callback: mfdleft::MfdLSymSwValueCallback<'a>,
    },
    MfdR1 {
        control: mfdright::MfdR1,
        callback: mfdright::MfdR1ValueCallback<'a>,
    },
    MfdR10 {
        control: mfdright::MfdR10,
        callback: mfdright::MfdR10ValueCallback<'a>,
    },
    MfdR11 {
        control: mfdright::MfdR11,
        callback: mfdright::MfdR11ValueCallback<'a>,
    },
    MfdR12 {
        control: mfdright::MfdR12,
        callback: mfdright::MfdR12ValueCallback<'a>,
    },
    MfdR13 {
        control: mfdright::MfdR13,
        callback: mfdright::MfdR13ValueCallback<'a>,
    },
    MfdR14 {
        control: mfdright::MfdR14,
        callback: mfdright::MfdR14ValueCallback<'a>,
    },
    MfdR15 {
        control: mfdright::MfdR15,
        callback: mfdright::MfdR15ValueCallback<'a>,
    },
    MfdR16 {
        control: mfdright::MfdR16,
        callback: mfdright::MfdR16ValueCallback<'a>,
    },
    MfdR17 {
        control: mfdright::MfdR17,
        callback: mfdright::MfdR17ValueCallback<'a>,
    },
    MfdR18 {
        control: mfdright::MfdR18,
        callback: mfdright::MfdR18ValueCallback<'a>,
    },
    MfdR19 {
        control: mfdright::MfdR19,
        callback: mfdright::MfdR19ValueCallback<'a>,
    },
    MfdR2 {
        control: mfdright::MfdR2,
        callback: mfdright::MfdR2ValueCallback<'a>,
    },
    MfdR20 {
        control: mfdright::MfdR20,
        callback: mfdright::MfdR20ValueCallback<'a>,
    },
    MfdR3 {
        control: mfdright::MfdR3,
        callback: mfdright::MfdR3ValueCallback<'a>,
    },
    MfdR4 {
        control: mfdright::MfdR4,
        callback: mfdright::MfdR4ValueCallback<'a>,
    },
    MfdR5 {
        control: mfdright::MfdR5,
        callback: mfdright::MfdR5ValueCallback<'a>,
    },
    MfdR6 {
        control: mfdright::MfdR6,
        callback: mfdright::MfdR6ValueCallback<'a>,
    },
    MfdR7 {
        control: mfdright::MfdR7,
        callback: mfdright::MfdR7ValueCallback<'a>,
    },
    MfdR8 {
        control: mfdright::MfdR8,
        callback: mfdright::MfdR8ValueCallback<'a>,
    },
    MfdR9 {
        control: mfdright::MfdR9,
        callback: mfdright::MfdR9ValueCallback<'a>,
    },
    MfdRBrtSw {
        control: mfdright::MfdRBrtSw,
        callback: mfdright::MfdRBrtSwValueCallback<'a>,
    },
    MfdRConSw {
        control: mfdright::MfdRConSw,
        callback: mfdright::MfdRConSwValueCallback<'a>,
    },
    MfdRGainSw {
        control: mfdright::MfdRGainSw,
        callback: mfdright::MfdRGainSwValueCallback<'a>,
    },
    MfdRSymSw {
        control: mfdright::MfdRSymSw,
        callback: mfdright::MfdRSymSwValueCallback<'a>,
    },
    MfdSw {
        control: avionicpanel::MfdSw,
        callback: avionicpanel::MfdSwValueCallback<'a>,
    },
    MidsLvtKnb {
        control: avionicpanel::MidsLvtKnb,
        callback: avionicpanel::MidsLvtKnbValueCallback<'a>,
    },
    MmcPwrSw {
        control: avionicpanel::MmcPwrSw,
        callback: avionicpanel::MmcPwrSwValueCallback<'a>,
    },
    MslKnb {
        control: audiopanel::MslKnb,
        callback: IntegerCallback<'a>,
    },
    ObogsSw {
        control: oxygensystem::ObogsSw,
        callback: oxygensystem::ObogsSwValueCallback<'a>,
    },
    OxyDiluterLvr {
        control: oxygensystem::OxyDiluterLvr,
        callback: oxygensystem::OxyDiluterLvrValueCallback<'a>,
    },
    OxyEmergLvr {
        control: oxygensystem::OxyEmergLvr,
        callback: IntegerCallback<'a>,
    },
    OxySupplyLvr {
        control: oxygensystem::OxySupplyLvr,
        callback: oxygensystem::OxySupplyLvrValueCallback<'a>,
    },
    OxygenPressure {
        control: oxygensystem::OxygenPressure,
        callback: IntegerCallback<'a>,
    },
    PitchTrim {
        control: controlinterface::PitchTrim,
        callback: IntegerCallback<'a>,
    },
    Pitchtrimind {
        control: trimindicators::Pitchtrimind,
        callback: IntegerCallback<'a>,
    },
    PlainCipherSw {
        control: ky58::PlainCipherSw,
        callback: ky58::PlainCipherSwValueCallback<'a>,
    },
    PosFlashLightSw {
        control: externallights::PosFlashLightSw,
        callback: externallights::PosFlashLightSwValueCallback<'a>,
    },
    PosFuselageLightSw {
        control: externallights::PosFuselageLightSw,
        callback: externallights::PosFuselageLightSwValueCallback<'a>,
    },
    PosWingTailLightSw {
        control: externallights::PosWingTailLightSw,
        callback: externallights::PosWingTailLightSwValueCallback<'a>,
    },
    PriConsolesBrtKnb {
        control: interiorlights::PriConsolesBrtKnb,
        callback: IntegerCallback<'a>,
    },
    PriDataDisplayBrtKnb {
        control: interiorlights::PriDataDisplayBrtKnb,
        callback: IntegerCallback<'a>,
    },
    PriInstPnlBrtKnb {
        control: interiorlights::PriInstPnlBrtKnb,
        callback: IntegerCallback<'a>,
    },
    ProbeHeatSw {
        control: electricsystem::ProbeHeatSw,
        callback: IntegerCallback<'a>,
    },
    RdrAltPwrSw {
        control: sensorpanel::RdrAltPwrSw,
        callback: sensorpanel::RdrAltPwrSwValueCallback<'a>,
    },
    RfSw {
        control: ufc::RfSw,
        callback: ufc::RfSwValueCallback<'a>,
    },
    RollTrim {
        control: controlinterface::RollTrim,
        callback: IntegerCallback<'a>,
    },
    Rolltrimind {
        control: trimindicators::Rolltrimind,
        callback: IntegerCallback<'a>,
    },
    RwrActPwrBtn {
        control: rwr::RwrActPwrBtn,
        callback: rwr::RwrActPwrBtnValueCallback<'a>,
    },
    RwrAltBtn {
        control: rwr::RwrAltBtn,
        callback: rwr::RwrAltBtnValueCallback<'a>,
    },
    RwrHandoffBtn {
        control: rwr::RwrHandoffBtn,
        callback: rwr::RwrHandoffBtnValueCallback<'a>,
    },
    RwrIndDimKnb {
        control: rwr::RwrIndDimKnb,
        callback: IntegerCallback<'a>,
    },
    RwrIntensKnb {
        control: rwr::RwrIntensKnb,
        callback: IntegerCallback<'a>,
    },
    RwrLaunchBtn {
        control: rwr::RwrLaunchBtn,
        callback: rwr::RwrLaunchBtnValueCallback<'a>,
    },
    RwrModeBtn {
        control: rwr::RwrModeBtn,
        callback: rwr::RwrModeBtnValueCallback<'a>,
    },
    RwrPwrBtn {
        control: rwr::RwrPwrBtn,
        callback: rwr::RwrPwrBtnValueCallback<'a>,
    },
    RwrSearchBtn {
        control: rwr::RwrSearchBtn,
        callback: rwr::RwrSearchBtnValueCallback<'a>,
    },
    RwrSysTestBtn {
        control: rwr::RwrSysTestBtn,
        callback: rwr::RwrSysTestBtnValueCallback<'a>,
    },
    RwrTBtn {
        control: rwr::RwrTBtn,
        callback: rwr::RwrTBtnValueCallback<'a>,
    },
    RwrUnknownShipBtn {
        control: rwr::RwrUnknownShipBtn,
        callback: rwr::RwrUnknownShipBtnValueCallback<'a>,
    },
    SaiAircraftreferencesymbol {
        control: sai::SaiAircraftreferencesymbol,
        callback: IntegerCallback<'a>,
    },
    SaiBank {
        control: sai::SaiBank,
        callback: IntegerCallback<'a>,
    },
    SaiBankArrow {
        control: sai::SaiBankArrow,
        callback: IntegerCallback<'a>,
    },
    SaiCage {
        control: sai::SaiCage,
        callback: sai::SaiCageValueCallback<'a>,
    },
    SaiKnbArrow {
        control: sai::SaiKnbArrow,
        callback: IntegerCallback<'a>,
    },
    SaiOffFlag {
        control: sai::SaiOffFlag,
        callback: IntegerCallback<'a>,
    },
    SaiPitch {
        control: sai::SaiPitch,
        callback: IntegerCallback<'a>,
    },
    SaiPitchTrim {
        control: sai::SaiPitchTrim,
        callback: IntegerCallback<'a>,
    },
    SeatAdj {
        control: cockpitmechanics::SeatAdj,
        callback: IntegerCallback<'a>,
    },
    SeatEjectHandle {
        control: cockpitmechanics::SeatEjectHandle,
        callback: cockpitmechanics::SeatEjectHandleValueCallback<'a>,
    },
    SeatEjectSafe {
        control: cockpitmechanics::SeatEjectSafe,
        callback: cockpitmechanics::SeatEjectSafeValueCallback<'a>,
    },
    SeatHeight {
        control: cockpitmechanics::SeatHeight,
        callback: IntegerCallback<'a>,
    },
    SecVoiceKnb {
        control: audiopanel::SecVoiceKnb,
        callback: IntegerCallback<'a>,
    },
    SetAirspeed {
        control: airspeedindicator::SetAirspeed,
        callback: IntegerCallback<'a>,
    },
    SpeedbrakeIndicator {
        control: speedbrake::SpeedbrakeIndicator,
        callback: IntegerCallback<'a>,
    },
    StStaSw {
        control: avionicpanel::StStaSw,
        callback: avionicpanel::StStaSwValueCallback<'a>,
    },
    StoresConfigSw {
        control: controlinterface::StoresConfigSw,
        callback: controlinterface::StoresConfigSwValueCallback<'a>,
    },
    SysaPressure {
        control: hydraulicpressureindicators::SysaPressure,
        callback: IntegerCallback<'a>,
    },
    SysbPressure {
        control: hydraulicpressureindicators::SysbPressure,
        callback: IntegerCallback<'a>,
    },
    TacanKnb {
        control: audiopanel::TacanKnb,
        callback: IntegerCallback<'a>,
    },
    TankIntertingSw {
        control: fuelsystem::TankIntertingSw,
        callback: fuelsystem::TankIntertingSwValueCallback<'a>,
    },
    TempKnb {
        control: ecs::TempKnb,
        callback: ecs::TempKnbValueCallback<'a>,
    },
    TfKnb {
        control: audiopanel::TfKnb,
        callback: IntegerCallback<'a>,
    },
    ThreatKnb {
        control: audiopanel::ThreatKnb,
        callback: IntegerCallback<'a>,
    },
    TrimApDiscSw {
        control: controlinterface::TrimApDiscSw,
        callback: controlinterface::TrimApDiscSwValueCallback<'a>,
    },
    UfcSw {
        control: avionicpanel::UfcSw,
        callback: avionicpanel::UfcSwValueCallback<'a>,
    },
    UhfAntSelSw {
        control: audiopanel::UhfAntSelSw,
        callback: audiopanel::UhfAntSelSwValueCallback<'a>,
    },
    UhfChanDisp {
        control: uhf::UhfChanDisp,
        callback: DisplayCallback<'a>,
    },
    UhfChanKnb {
        control: uhf::UhfChanKnb,
        callback: IntegerCallback<'a>,
    },
    UhfDoor {
        control: uhf::UhfDoor,
        callback: uhf::UhfDoorValueCallback<'a>,
    },
    UhfFreq0025Knb {
        control: uhf::UhfFreq0025Knb,
        callback: uhf::UhfFreq0025KnbValueCallback<'a>,
    },
    UhfFreq01Knb {
        control: uhf::UhfFreq01Knb,
        callback: uhf::UhfFreq01KnbValueCallback<'a>,
    },
    UhfFreq100Knb {
        control: uhf::UhfFreq100Knb,
        callback: uhf::UhfFreq100KnbValueCallback<'a>,
    },
    UhfFreq10Knb {
        control: uhf::UhfFreq10Knb,
        callback: uhf::UhfFreq10KnbValueCallback<'a>,
    },
    UhfFreq1Knb {
        control: uhf::UhfFreq1Knb,
        callback: uhf::UhfFreq1KnbValueCallback<'a>,
    },
    UhfFreqDisp {
        control: uhf::UhfFreqDisp,
        callback: DisplayCallback<'a>,
    },
    UhfFuncKnb {
        control: uhf::UhfFuncKnb,
        callback: uhf::UhfFuncKnbValueCallback<'a>,
    },
    UhfModeKnb {
        control: uhf::UhfModeKnb,
        callback: uhf::UhfModeKnbValueCallback<'a>,
    },
    UhfSquelchSw {
        control: uhf::UhfSquelchSw,
        callback: uhf::UhfSquelchSwValueCallback<'a>,
    },
    UhfStatusBtn {
        control: uhf::UhfStatusBtn,
        callback: uhf::UhfStatusBtnValueCallback<'a>,
    },
    UhfTestBtn {
        control: uhf::UhfTestBtn,
        callback: uhf::UhfTestBtnValueCallback<'a>,
    },
    UhfToneBtn {
        control: uhf::UhfToneBtn,
        callback: uhf::UhfToneBtnValueCallback<'a>,
    },
    UhfVolKnb {
        control: uhf::UhfVolKnb,
        callback: IntegerCallback<'a>,
    },
    VmsInhibitSw {
        control: audiopanel::VmsInhibitSw,
        callback: audiopanel::VmsInhibitSwValueCallback<'a>,
    },
    Vvi {
        control: verticalvelocityindicator::Vvi,
        callback: IntegerCallback<'a>,
    },
    YawTrim {
        control: controlinterface::YawTrim,
        callback: IntegerCallback<'a>,
    },
    ZeroizeSw {
        control: ky58::ZeroizeSw,
        callback: ky58::ZeroizeSwValueCallback<'a>,
    },
    ZeroizeSwCover {
        control: ky58::ZeroizeSwCover,
        callback: ky58::ZeroizeSwCoverValueCallback<'a>,
    },
}

impl RegisteredControl<'_> {
    fn accept(&mut self, packet: Packet<'_>) {
        match self {
            Self::AbResetSw { control, .. } => control.accept(packet),
            Self::AdiAuxFlag { control, .. } => control.accept(packet),
            Self::AdiBank { control, .. } => control.accept(packet),
            Self::AdiBubble { control, .. } => control.accept(packet),
            Self::AdiGsBar { control, .. } => control.accept(packet),
            Self::AdiGsFlag { control, .. } => control.accept(packet),
            Self::AdiGsPointer { control, .. } => control.accept(packet),
            Self::AdiLocBar { control, .. } => control.accept(packet),
            Self::AdiLocFlag { control, .. } => control.accept(packet),
            Self::AdiOffFlag { control, .. } => control.accept(packet),
            Self::AdiPitch { control, .. } => control.accept(packet),
            Self::AdiPitchTrim { control, .. } => control.accept(packet),
            Self::AdiTurnrate { control, .. } => control.accept(packet),
            Self::AdvModeSw { control, .. } => control.accept(packet),
            Self::AirRefuelLightKnb { control, .. } => control.accept(packet),
            Self::AirRefuelSw { control, .. } => control.accept(packet),
            Self::AirSourceKnb { control, .. } => control.accept(packet),
            Self::Airspeed { control, .. } => control.accept(packet),
            Self::AirspeedSetKnb { control, .. } => control.accept(packet),
            Self::Alt10000FtCnt { control, .. } => control.accept(packet),
            Self::Alt1000FtCnt { control, .. } => control.accept(packet),
            Self::Alt100FtCnt { control, .. } => control.accept(packet),
            Self::Alt100FtPtr { control, .. } => control.accept(packet),
            Self::AltBaroSetKnb { control, .. } => control.accept(packet),
            Self::AltFlapsSw { control, .. } => control.accept(packet),
            Self::AltModeLv { control, .. } => control.accept(packet),
            Self::AltPneuFlag { control, .. } => control.accept(packet),
            Self::AltPressureDrum0Cnt { control, .. } => control.accept(packet),
            Self::AltPressureDrum1Cnt { control, .. } => control.accept(packet),
            Self::AltPressureDrum2Cnt { control, .. } => control.accept(packet),
            Self::AltPressureDrum3Cnt { control, .. } => control.accept(packet),
            Self::AltRelBtn { control, .. } => control.accept(packet),
            Self::AntiCollLightKnb { control, .. } => control.accept(packet),
            Self::AntiSkidSw { control, .. } => control.accept(packet),
            Self::AoaIndexBrtKnb { control, .. } => control.accept(packet),
            Self::AoaValue { control, .. } => control.accept(packet),
            Self::ApPitchSw { control, .. } => control.accept(packet),
            Self::ApRollSw { control, .. } => control.accept(packet),
            Self::ArStatusBrtKnb { control, .. } => control.accept(packet),
            Self::BitSw { control, .. } => control.accept(packet),
            Self::BrakeChanSw { control, .. } => control.accept(packet),
            Self::CanopyHandle { control, .. } => control.accept(packet),
            Self::CanopyJettThandle { control, .. } => control.accept(packet),
            Self::CanopySw { control, .. } => control.accept(packet),
            Self::CanopyValue { control, .. } => control.accept(packet),
            Self::ClockCurrtimeH { control, .. } => control.accept(packet),
            Self::ClockCurrtimeMs { control, .. } => control.accept(packet),
            Self::ClockElapsed { control, .. } => control.accept(packet),
            Self::ClockElapsedTimeM { control, .. } => control.accept(packet),
            Self::ClockElapsedTimeSec { control, .. } => control.accept(packet),
            Self::ClockSet { control, .. } => control.accept(packet),
            Self::ClockWind { control, .. } => control.accept(packet),
            Self::Cmds01ExpCatSw { control, .. } => control.accept(packet),
            Self::Cmds02ExpCatSw { control, .. } => control.accept(packet),
            Self::CmdsChAmount { control, .. } => control.accept(packet),
            Self::CmdsChExpCatSw { control, .. } => control.accept(packet),
            Self::CmdsDispenseBtn { control, .. } => control.accept(packet),
            Self::CmdsFlAmount { control, .. } => control.accept(packet),
            Self::CmdsFlExpCatSw { control, .. } => control.accept(packet),
            Self::CmdsJettSw { control, .. } => control.accept(packet),
            Self::CmdsJmrSourcheSw { control, .. } => control.accept(packet),
            Self::CmdsModeKnb { control, .. } => control.accept(packet),
            Self::CmdsMwsSourcheSw { control, .. } => control.accept(packet),
            Self::CmdsO1Amount { control, .. } => control.accept(packet),
            Self::CmdsO2Amount { control, .. } => control.accept(packet),
            Self::CmdsProgKnb { control, .. } => control.accept(packet),
            Self::CmdsPwrSourcheSw { control, .. } => control.accept(packet),
            Self::CockpitAlititude { control, .. } => control.accept(packet),
            Self::Comm1ModeKnb { control, .. } => control.accept(packet),
            Self::Comm1PwrKnb { control, .. } => control.accept(packet),
            Self::Comm2ModeKnb { control, .. } => control.accept(packet),
            Self::Comm2PwrKnb { control, .. } => control.accept(packet),
            Self::DedLine1 { control, .. } => control.accept(packet),
            Self::DedLine2 { control, .. } => control.accept(packet),
            Self::DedLine3 { control, .. } => control.accept(packet),
            Self::DedLine4 { control, .. } => control.accept(packet),
            Self::DedLine5 { control, .. } => control.accept(packet),
            Self::DefogLever { control, .. } => control.accept(packet),
            Self::DigiBakSw { control, .. } => control.accept(packet),
            Self::DlSw { control, .. } => control.accept(packet),
            Self::DnLockBtn { control, .. } => control.accept(packet),
            Self::Ecm1Btn { control, .. } => control.accept(packet),
            Self::Ecm2Btn { control, .. } => control.accept(packet),
            Self::Ecm3Btn { control, .. } => control.accept(packet),
            Self::Ecm4Btn { control, .. } => control.accept(packet),
            Self::Ecm5Btn { control, .. } => control.accept(packet),
            Self::Ecm6Btn { control, .. } => control.accept(packet),
            Self::EcmBitBtn { control, .. } => control.accept(packet),
            Self::EcmDimKnb { control, .. } => control.accept(packet),
            Self::EcmFrmBtn { control, .. } => control.accept(packet),
            Self::EcmPwSw { control, .. } => control.accept(packet),
            Self::EcmResetBtn { control, .. } => control.accept(packet),
            Self::EcmSplBtn { control, .. } => control.accept(packet),
            Self::EcmXmitSw { control, .. } => control.accept(packet),
            Self::EhsiCrsSet { control, .. } => control.accept(packet),
            Self::EhsiCrsSetKnb { control, .. } => control.accept(packet),
            Self::EhsiHdgSetBtn { control, .. } => control.accept(packet),
            Self::EhsiHdgSetKnb { control, .. } => control.accept(packet),
            Self::EhsiMode { control, .. } => control.accept(packet),
            Self::ElecCaution { control, .. } => control.accept(packet),
            Self::EmergStroreJett { control, .. } => control.accept(packet),
            Self::EngAntiIce { control, .. } => control.accept(packet),
            Self::EngContSw { control, .. } => control.accept(packet),
            Self::EngContSwCover { control, .. } => control.accept(packet),
            Self::EngineFeedKnb { control, .. } => control.accept(packet),
            Self::EngineFtit { control, .. } => control.accept(packet),
            Self::EngineNozzlePosition { control, .. } => control.accept(packet),
            Self::EngineOilPressure { control, .. } => control.accept(packet),
            Self::EngineTachometer { control, .. } => control.accept(packet),
            Self::EpuGenTestSw { control, .. } => control.accept(packet),
            Self::EpuSw { control, .. } => control.accept(packet),
            Self::EpuSwCoverOff { control, .. } => control.accept(packet),
            Self::EpuSwCoverOn { control, .. } => control.accept(packet),
            Self::ExtFormationLights { control, .. } => control.accept(packet),
            Self::ExtFuelTransSw { control, .. } => control.accept(packet),
            Self::ExtPositionLightFuselage { control, .. } => control.accept(packet),
            Self::ExtPositionLightsWing { control, .. } => control.accept(packet),
            Self::ExtSpeedBrakeLeft { control, .. } => control.accept(packet),
            Self::ExtSpeedBrakeRight { control, .. } => control.accept(packet),
            Self::ExtStrobeTail { control, .. } => control.accept(packet),
            Self::ExtTailLight { control, .. } => control.accept(packet),
            Self::ExtWowLeft { control, .. } => control.accept(packet),
            Self::ExtWowNose { control, .. } => control.accept(packet),
            Self::ExtWowRight { control, .. } => control.accept(packet),
            Self::FAckBtn { control, .. } => control.accept(packet),
            Self::FcrPwrSw { control, .. } => control.accept(packet),
            Self::FireOheatDetectBtn { control, .. } => control.accept(packet),
            Self::FlcsPwrTestSw { control, .. } => control.accept(packet),
            Self::FlcsResetSw { control, .. } => control.accept(packet),
            Self::FloodConsolesBrtKnb { control, .. } => control.accept(packet),
            Self::FloodInstPnlBrtKnb { control, .. } => control.accept(packet),
            Self::FlowIndicator { control, .. } => control.accept(packet),
            Self::FlowIndicatorLight { control, .. } => control.accept(packet),
            Self::FormLightKnb { control, .. } => control.accept(packet),
            Self::FuelAl { control, .. } => control.accept(packet),
            Self::FuelFr { control, .. } => control.accept(packet),
            Self::FuelMasterCv { control, .. } => control.accept(packet),
            Self::FuelMasterSw { control, .. } => control.accept(packet),
            Self::FuelQtySelKnb { control, .. } => control.accept(packet),
            Self::FuelQtySelTKnb { control, .. } => control.accept(packet),
            Self::Fuelflowcounter100 { control, .. } => control.accept(packet),
            Self::Fuelflowcounter10k { control, .. } => control.accept(packet),
            Self::Fuelflowcounter1k { control, .. } => control.accept(packet),
            Self::Fueltotalizer100 { control, .. } => control.accept(packet),
            Self::Fueltotalizer10k { control, .. } => control.accept(packet),
            Self::Fueltotalizer1k { control, .. } => control.accept(packet),
            Self::GearHandle { control, .. } => control.accept(packet),
            Self::GndJettEnableSw { control, .. } => control.accept(packet),
            Self::GpsSw { control, .. } => control.accept(packet),
            Self::HdptSwL { control, .. } => control.accept(packet),
            Self::HdptSwR { control, .. } => control.accept(packet),
            Self::HideStick { control, .. } => control.accept(packet),
            Self::HmcsIntKnb { control, .. } => control.accept(packet),
            Self::HookSw { control, .. } => control.accept(packet),
            Self::HornSilenceBtn { control, .. } => control.accept(packet),
            Self::HotMicSw { control, .. } => control.accept(packet),
            Self::HudAltSw { control, .. } => control.accept(packet),
            Self::HudBrtSw { control, .. } => control.accept(packet),
            Self::HudDedDataSw { control, .. } => control.accept(packet),
            Self::HudDepressRetSw { control, .. } => control.accept(packet),
            Self::HudFpMarkerSw { control, .. } => control.accept(packet),
            Self::HudScalesSw { control, .. } => control.accept(packet),
            Self::HudSpeedSw { control, .. } => control.accept(packet),
            Self::HudTestSw { control, .. } => control.accept(packet),
            Self::HydrazinVolume { control, .. } => control.accept(packet),
            Self::IcpAaModeBtn { control, .. } => control.accept(packet),
            Self::IcpAgModeBtn { control, .. } => control.accept(packet),
            Self::IcpBtn0 { control, .. } => control.accept(packet),
            Self::IcpBtn1 { control, .. } => control.accept(packet),
            Self::IcpBtn2 { control, .. } => control.accept(packet),
            Self::IcpBtn3 { control, .. } => control.accept(packet),
            Self::IcpBtn4 { control, .. } => control.accept(packet),
            Self::IcpBtn5 { control, .. } => control.accept(packet),
            Self::IcpBtn6 { control, .. } => control.accept(packet),
            Self::IcpBtn7 { control, .. } => control.accept(packet),
            Self::IcpBtn8 { control, .. } => control.accept(packet),
            Self::IcpBtn9 { control, .. } => control.accept(packet),
            Self::IcpCom1Btn { control, .. } => control.accept(packet),
            Self::IcpCom2Btn { control, .. } => control.accept(packet),
            Self::IcpDataRtnSeqSw { control, .. } => control.accept(packet),
            Self::IcpDataUpDnSw { control, .. } => control.accept(packet),
            Self::IcpDedSw { control, .. } => control.accept(packet),
            Self::IcpDriftSw { control, .. } => control.accept(packet),
            Self::IcpEntrBtn { control, .. } => control.accept(packet),
            Self::IcpFlirGainSw { control, .. } => control.accept(packet),
            Self::IcpFlirSw { control, .. } => control.accept(packet),
            Self::IcpHudBrtKnb { control, .. } => control.accept(packet),
            Self::IcpIffBtn { control, .. } => control.accept(packet),
            Self::IcpListBtn { control, .. } => control.accept(packet),
            Self::IcpRasterBrtKnb { control, .. } => control.accept(packet),
            Self::IcpRasterContrKnb { control, .. } => control.accept(packet),
            Self::IcpRclBtn { control, .. } => control.accept(packet),
            Self::IcpReticleDepressKnb { control, .. } => control.accept(packet),
            Self::IcpWxBtn { control, .. } => control.accept(packet),
            Self::IffAntSelSw { control, .. } => control.accept(packet),
            Self::IffCIKnb { control, .. } => control.accept(packet),
            Self::IffCodeDrumDigit1 { control, .. } => control.accept(packet),
            Self::IffCodeDrumDigit2 { control, .. } => control.accept(packet),
            Self::IffCodeDrumDigit3 { control, .. } => control.accept(packet),
            Self::IffCodeDrumDigit4 { control, .. } => control.accept(packet),
            Self::IffEnableSw { control, .. } => control.accept(packet),
            Self::IffIdBtn { control, .. } => control.accept(packet),
            Self::IffM1Sel1 { control, .. } => control.accept(packet),
            Self::IffM1Sel2 { control, .. } => control.accept(packet),
            Self::IffM3Sel1 { control, .. } => control.accept(packet),
            Self::IffM3Sel2 { control, .. } => control.accept(packet),
            Self::IffM4CodeSw { control, .. } => control.accept(packet),
            Self::IffM4MonitorSw { control, .. } => control.accept(packet),
            Self::IffM4ReplySw { control, .. } => control.accept(packet),
            Self::IffMasterKnb { control, .. } => control.accept(packet),
            Self::IlsPwrKnb { control, .. } => control.accept(packet),
            Self::InsKnb { control, .. } => control.accept(packet),
            Self::IntercomKnb { control, .. } => control.accept(packet),
            Self::JfsSw { control, .. } => control.accept(packet),
            Self::Ky58FillKnb { control, .. } => control.accept(packet),
            Self::Ky58ModeKnb { control, .. } => control.accept(packet),
            Self::Ky58PwrKnb { control, .. } => control.accept(packet),
            Self::Ky58VolKnb { control, .. } => control.accept(packet),
            Self::LandTaxiLightSw { control, .. } => control.accept(packet),
            Self::LaserArmSw { control, .. } => control.accept(packet),
            Self::LeFlapsSw { control, .. } => control.accept(packet),
            Self::LightAcftBattFail { control, .. } => control.accept(packet),
            Self::LightActive { control, .. } => control.accept(packet),
            Self::LightAftFuelLow { control, .. } => control.accept(packet),
            Self::LightAir { control, .. } => control.accept(packet),
            Self::LightAntiSkid { control, .. } => control.accept(packet),
            Self::LightAoaDn { control, .. } => control.accept(packet),
            Self::LightAoaMid { control, .. } => control.accept(packet),
            Self::LightAoaUp { control, .. } => control.accept(packet),
            Self::LightArNws { control, .. } => control.accept(packet),
            Self::LightAtfNot { control, .. } => control.accept(packet),
            Self::LightAvionicsFault { control, .. } => control.accept(packet),
            Self::LightBuc { control, .. } => control.accept(packet),
            Self::LightCabinPress { control, .. } => control.accept(packet),
            Self::LightCadc { control, .. } => control.accept(packet),
            Self::LightCanopy { control, .. } => control.accept(packet),
            Self::LightCaution1 { control, .. } => control.accept(packet),
            Self::LightCaution2 { control, .. } => control.accept(packet),
            Self::LightCaution3 { control, .. } => control.accept(packet),
            Self::LightCaution4 { control, .. } => control.accept(packet),
            Self::LightCaution5 { control, .. } => control.accept(packet),
            Self::LightCaution6 { control, .. } => control.accept(packet),
            Self::LightCmdsDisp { control, .. } => control.accept(packet),
            Self::LightCmdsGo { control, .. } => control.accept(packet),
            Self::LightCmdsNoGo { control, .. } => control.accept(packet),
            Self::LightCmdsRdy { control, .. } => control.accept(packet),
            Self::LightConsles { control, .. } => control.accept(packet),
            Self::LightConslesFlood { control, .. } => control.accept(packet),
            Self::LightDbuOn { control, .. } => control.accept(packet),
            Self::LightDisc { control, .. } => control.accept(packet),
            Self::LightEcm1A { control, .. } => control.accept(packet),
            Self::LightEcm1F { control, .. } => control.accept(packet),
            Self::LightEcm1S { control, .. } => control.accept(packet),
            Self::LightEcm1T { control, .. } => control.accept(packet),
            Self::LightEcm2A { control, .. } => control.accept(packet),
            Self::LightEcm2F { control, .. } => control.accept(packet),
            Self::LightEcm2S { control, .. } => control.accept(packet),
            Self::LightEcm2T { control, .. } => control.accept(packet),
            Self::LightEcm3A { control, .. } => control.accept(packet),
            Self::LightEcm3F { control, .. } => control.accept(packet),
            Self::LightEcm3S { control, .. } => control.accept(packet),
            Self::LightEcm3T { control, .. } => control.accept(packet),
            Self::LightEcm4A { control, .. } => control.accept(packet),
            Self::LightEcm4F { control, .. } => control.accept(packet),
            Self::LightEcm4S { control, .. } => control.accept(packet),
            Self::LightEcm4T { control, .. } => control.accept(packet),
            Self::LightEcm5A { control, .. } => control.accept(packet),
            Self::LightEcm5F { control, .. } => control.accept(packet),
            Self::LightEcm5S { control, .. } => control.accept(packet),
            Self::LightEcm5T { control, .. } => control.accept(packet),
            Self::LightEcmA { control, .. } => control.accept(packet),
            Self::LightEcmF { control, .. } => control.accept(packet),
            Self::LightEcmFrmA { control, .. } => control.accept(packet),
            Self::LightEcmFrmF { control, .. } => control.accept(packet),
            Self::LightEcmFrmS { control, .. } => control.accept(packet),
            Self::LightEcmFrmT { control, .. } => control.accept(packet),
            Self::LightEcmS { control, .. } => control.accept(packet),
            Self::LightEcmSplA { control, .. } => control.accept(packet),
            Self::LightEcmSplF { control, .. } => control.accept(packet),
            Self::LightEcmSplS { control, .. } => control.accept(packet),
            Self::LightEcmSplT { control, .. } => control.accept(packet),
            Self::LightEcmT { control, .. } => control.accept(packet),
            Self::LightEdge { control, .. } => control.accept(packet),
            Self::LightEec { control, .. } => control.accept(packet),
            Self::LightElec { control, .. } => control.accept(packet),
            Self::LightElecSys { control, .. } => control.accept(packet),
            Self::LightEngFire { control, .. } => control.accept(packet),
            Self::LightEngine { control, .. } => control.accept(packet),
            Self::LightEngineFault { control, .. } => control.accept(packet),
            Self::LightEpu { control, .. } => control.accept(packet),
            Self::LightEpuGen { control, .. } => control.accept(packet),
            Self::LightEpuPmg { control, .. } => control.accept(packet),
            Self::LightEquipHot { control, .. } => control.accept(packet),
            Self::LightFlFail { control, .. } => control.accept(packet),
            Self::LightFlRun { control, .. } => control.accept(packet),
            Self::LightFlcs { control, .. } => control.accept(packet),
            Self::LightFlcsFault { control, .. } => control.accept(packet),
            Self::LightFlcsPmg { control, .. } => control.accept(packet),
            Self::LightFlcsPwrA { control, .. } => control.accept(packet),
            Self::LightFlcsPwrB { control, .. } => control.accept(packet),
            Self::LightFlcsPwrC { control, .. } => control.accept(packet),
            Self::LightFlcsPwrD { control, .. } => control.accept(packet),
            Self::LightFlcsRly { control, .. } => control.accept(packet),
            Self::LightFuelOilHot { control, .. } => control.accept(packet),
            Self::LightFwdFuelLow { control, .. } => control.accept(packet),
            Self::LightGearL { control, .. } => control.accept(packet),
            Self::LightGearN { control, .. } => control.accept(packet),
            Self::LightGearR { control, .. } => control.accept(packet),
            Self::LightGearWarn { control, .. } => control.accept(packet),
            Self::LightHook { control, .. } => control.accept(packet),
            Self::LightHydOilPress { control, .. } => control.accept(packet),
            Self::LightHydrazn { control, .. } => control.accept(packet),
            Self::LightIff { control, .. } => control.accept(packet),
            Self::LightInletIcing { control, .. } => control.accept(packet),
            Self::LightInstPnl { control, .. } => control.accept(packet),
            Self::LightInstPnlFlood { control, .. } => control.accept(packet),
            Self::LightJfsRun { control, .. } => control.accept(packet),
            Self::LightMainGen { control, .. } => control.accept(packet),
            Self::LightMarkerBeacon { control, .. } => control.accept(packet),
            Self::LightMasterCaution { control, .. } => control.accept(packet),
            Self::LightNuclear { control, .. } => control.accept(packet),
            Self::LightNwsFail { control, .. } => control.accept(packet),
            Self::LightObogs { control, .. } => control.accept(packet),
            Self::LightOverheat { control, .. } => control.accept(packet),
            Self::LightOxyLow { control, .. } => control.accept(packet),
            Self::LightProbeHeat { control, .. } => control.accept(packet),
            Self::LightRadarAlt { control, .. } => control.accept(packet),
            Self::LightRdy { control, .. } => control.accept(packet),
            Self::LightRwrActPower { control, .. } => control.accept(packet),
            Self::LightRwrActivity { control, .. } => control.accept(packet),
            Self::LightRwrAlt { control, .. } => control.accept(packet),
            Self::LightRwrAltLow { control, .. } => control.accept(packet),
            Self::LightRwrHandoffH { control, .. } => control.accept(packet),
            Self::LightRwrHandoffUp { control, .. } => control.accept(packet),
            Self::LightRwrModeOpen { control, .. } => control.accept(packet),
            Self::LightRwrModePri { control, .. } => control.accept(packet),
            Self::LightRwrMslLaunch { control, .. } => control.accept(packet),
            Self::LightRwrPower { control, .. } => control.accept(packet),
            Self::LightRwrSearch { control, .. } => control.accept(packet),
            Self::LightRwrShipUnk { control, .. } => control.accept(packet),
            Self::LightRwrSystest { control, .. } => control.accept(packet),
            Self::LightRwrTgtsepDn { control, .. } => control.accept(packet),
            Self::LightRwrTgtsepUp { control, .. } => control.accept(packet),
            Self::LightSeatNot { control, .. } => control.accept(packet),
            Self::LightSec { control, .. } => control.accept(packet),
            Self::LightStby { control, .. } => control.accept(packet),
            Self::LightStbyGen { control, .. } => control.accept(packet),
            Self::LightStoresConfig { control, .. } => control.accept(packet),
            Self::LightTfFail { control, .. } => control.accept(packet),
            Self::LightToFlcs { control, .. } => control.accept(packet),
            Self::LightToLdgConfig { control, .. } => control.accept(packet),
            Self::MachIndicator { control, .. } => control.accept(packet),
            Self::MainPwrSw { control, .. } => control.accept(packet),
            Self::MalIndLtsBrtSw { control, .. } => control.accept(packet),
            Self::MalIndLtsTest { control, .. } => control.accept(packet),
            Self::ManTfFlyupSw { control, .. } => control.accept(packet),
            Self::ManualPitchSw { control, .. } => control.accept(packet),
            Self::MapSw { control, .. } => control.accept(packet),
            Self::MasterArmSw { control, .. } => control.accept(packet),
            Self::MasterCaution { control, .. } => control.accept(packet),
            Self::MasterLightSw { control, .. } => control.accept(packet),
            Self::MaxAirspeed { control, .. } => control.accept(packet),
            Self::MaxPwrSw { control, .. } => control.accept(packet),
            Self::MfdL1 { control, .. } => control.accept(packet),
            Self::MfdL10 { control, .. } => control.accept(packet),
            Self::MfdL11 { control, .. } => control.accept(packet),
            Self::MfdL12 { control, .. } => control.accept(packet),
            Self::MfdL13 { control, .. } => control.accept(packet),
            Self::MfdL14 { control, .. } => control.accept(packet),
            Self::MfdL15 { control, .. } => control.accept(packet),
            Self::MfdL16 { control, .. } => control.accept(packet),
            Self::MfdL17 { control, .. } => control.accept(packet),
            Self::MfdL18 { control, .. } => control.accept(packet),
            Self::MfdL19 { control, .. } => control.accept(packet),
            Self::MfdL2 { control, .. } => control.accept(packet),
            Self::MfdL20 { control, .. } => control.accept(packet),
            Self::MfdL3 { control, .. } => control.accept(packet),
            Self::MfdL4 { control, .. } => control.accept(packet),
            Self::MfdL5 { control, .. } => control.accept(packet),
            Self::MfdL6 { control, .. } => control.accept(packet),
            Self::MfdL7 { control, .. } => control.accept(packet),
            Self::MfdL8 { control, .. } => control.accept(packet),
            Self::MfdL9 { control, .. } => control.accept(packet),
            Self::MfdLBrtSw { control, .. } => control.accept(packet),
            Self::MfdLConSw { control, .. } => control.accept(packet),
            Self::MfdLGainSw { control, .. } => control.accept(packet),
            Self::MfdLSymSw { control, .. } => control.accept(packet),
            Self::MfdR1 { control, .. } => control.accept(packet),
            Self::MfdR10 { control, .. } => control.accept(packet),
            Self::MfdR11 { control, .. } => control.accept(packet),
            Self::MfdR12 { control, .. } => control.accept(packet),
            Self::MfdR13 { control, .. } => control.accept(packet),
            Self::MfdR14 { control, .. } => control.accept(packet),
            Self::MfdR15 { control, .. } => control.accept(packet),
            Self::MfdR16 { control, .. } => control.accept(packet),
            Self::MfdR17 { control, .. } => control.accept(packet),
            Self::MfdR18 { control, .. } => control.accept(packet),
            Self::MfdR19 { control, .. } => control.accept(packet),
            Self::MfdR2 { control, .. } => control.accept(packet),
            Self::MfdR20 { control, .. } => control.accept(packet),
            Self::MfdR3 { control, .. } => control.accept(packet),
            Self::MfdR4 { control, .. } => control.accept(packet),
            Self::MfdR5 { control, .. } => control.accept(packet),
            Self::MfdR6 { control, .. } => control.accept(packet),
            Self::MfdR7 { control, .. } => control.accept(packet),
            Self::MfdR8 { control, .. } => control.accept(packet),
            Self::MfdR9 { control, .. } => control.accept(packet),
            Self::MfdRBrtSw { control, .. } => control.accept(packet),
            Self::MfdRConSw { control, .. } => control.accept(packet),
            Self::MfdRGainSw { control, .. } => control.accept(packet),
            Self::MfdRSymSw { control, .. } => control.accept(packet),
            Self::MfdSw { control, .. } => control.accept(packet),
            Self::MidsLvtKnb { control, .. } => control.accept(packet),
            Self::MmcPwrSw { control, .. } => control.accept(packet),
            Self::MslKnb { control, .. } => control.accept(packet),
            Self::ObogsSw { control, .. } => control.accept(packet),
            Self::OxyDiluterLvr { control, .. } => control.accept(packet),
            Self::OxyEmergLvr { control, .. } => control.accept(packet),
            Self::OxySupplyLvr { control, .. } => control.accept(packet),
            Self::OxygenPressure { control, .. } => control.accept(packet),
            Self::PitchTrim { control, .. } => control.accept(packet),
            Self::Pitchtrimind { control, .. } => control.accept(packet),
            Self::PlainCipherSw { control, .. } => control.accept(packet),
            Self::PosFlashLightSw { control, .. } => control.accept(packet),
            Self::PosFuselageLightSw { control, .. } => control.accept(packet),
            Self::PosWingTailLightSw { control, .. } => control.accept(packet),
            Self::PriConsolesBrtKnb { control, .. } => control.accept(packet),
            Self::PriDataDisplayBrtKnb { control, .. } => control.accept(packet),
            Self::PriInstPnlBrtKnb { control, .. } => control.accept(packet),
            Self::ProbeHeatSw { control, .. } => control.accept(packet),
            Self::RdrAltPwrSw { control, .. } => control.accept(packet),
            Self::RfSw { control, .. } => control.accept(packet),
            Self::RollTrim { control, .. } => control.accept(packet),
            Self::Rolltrimind { control, .. } => control.accept(packet),
            Self::RwrActPwrBtn { control, .. } => control.accept(packet),
            Self::RwrAltBtn { control, .. } => control.accept(packet),
            Self::RwrHandoffBtn { control, .. } => control.accept(packet),
            Self::RwrIndDimKnb { control, .. } => control.accept(packet),
            Self::RwrIntensKnb { control, .. } => control.accept(packet),
            Self::RwrLaunchBtn { control, .. } => control.accept(packet),
            Self::RwrModeBtn { control, .. } => control.accept(packet),
            Self::RwrPwrBtn { control, .. } => control.accept(packet),
            Self::RwrSearchBtn { control, .. } => control.accept(packet),
            Self::RwrSysTestBtn { control, .. } => control.accept(packet),
            Self::RwrTBtn { control, .. } => control.accept(packet),
            Self::RwrUnknownShipBtn { control, .. } => control.accept(packet),
            Self::SaiAircraftreferencesymbol { control, .. } => control.accept(packet),
            Self::SaiBank { control, .. } => control.accept(packet),
            Self::SaiBankArrow { control, .. } => control.accept(packet),
            Self::SaiCage { control, .. } => control.accept(packet),
            Self::SaiKnbArrow { control, .. } => control.accept(packet),
            Self::SaiOffFlag { control, .. } => control.accept(packet),
            Self::SaiPitch { control, .. } => control.accept(packet),
            Self::SaiPitchTrim { control, .. } => control.accept(packet),
            Self::SeatAdj { control, .. } => control.accept(packet),
            Self::SeatEjectHandle { control, .. } => control.accept(packet),
            Self::SeatEjectSafe { control, .. } => control.accept(packet),
            Self::SeatHeight { control, .. } => control.accept(packet),
            Self::SecVoiceKnb { control, .. } => control.accept(packet),
            Self::SetAirspeed { control, .. } => control.accept(packet),
            Self::SpeedbrakeIndicator { control, .. } => control.accept(packet),
            Self::StStaSw { control, .. } => control.accept(packet),
            Self::StoresConfigSw { control, .. } => control.accept(packet),
            Self::SysaPressure { control, .. } => control.accept(packet),
            Self::SysbPressure { control, .. } => control.accept(packet),
            Self::TacanKnb { control, .. } => control.accept(packet),
            Self::TankIntertingSw { control, .. } => control.accept(packet),
            Self::TempKnb { control, .. } => control.accept(packet),
            Self::TfKnb { control, .. } => control.accept(packet),
            Self::ThreatKnb { control, .. } => control.accept(packet),
            Self::TrimApDiscSw { control, .. } => control.accept(packet),
            Self::UfcSw { control, .. } => control.accept(packet),
            Self::UhfAntSelSw { control, .. } => control.accept(packet),
            Self::UhfChanDisp { control, .. } => control.accept(packet),
            Self::UhfChanKnb { control, .. } => control.accept(packet),
            Self::UhfDoor { control, .. } => control.accept(packet),
            Self::UhfFreq0025Knb { control, .. } => control.accept(packet),
            Self::UhfFreq01Knb { control, .. } => control.accept(packet),
            Self::UhfFreq100Knb { control, .. } => control.accept(packet),
            Self::UhfFreq10Knb { control, .. } => control.accept(packet),
            Self::UhfFreq1Knb { control, .. } => control.accept(packet),
            Self::UhfFreqDisp { control, .. } => control.accept(packet),
            Self::UhfFuncKnb { control, .. } => control.accept(packet),
            Self::UhfModeKnb { control, .. } => control.accept(packet),
            Self::UhfSquelchSw { control, .. } => control.accept(packet),
            Self::UhfStatusBtn { control, .. } => control.accept(packet),
            Self::UhfTestBtn { control, .. } => control.accept(packet),
            Self::UhfToneBtn { control, .. } => control.accept(packet),
            Self::UhfVolKnb { control, .. } => control.accept(packet),
            Self::VmsInhibitSw { control, .. } => control.accept(packet),
            Self::Vvi { control, .. } => control.accept(packet),
            Self::YawTrim { control, .. } => control.accept(packet),
            Self::ZeroizeSw { control, .. } => control.accept(packet),
            Self::ZeroizeSwCover { control, .. } => control.accept(packet),
        }
    }

    fn commit(&mut self) {
        match self {
            Self::AbResetSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdiAuxFlag { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdiBank { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdiBubble { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdiGsBar { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdiGsFlag { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdiGsPointer { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdiLocBar { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdiLocFlag { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdiOffFlag { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdiPitch { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdiPitchTrim { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdiTurnrate { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdvModeSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <controlinterface::AdvModeSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AirRefuelLightKnb { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AirRefuelSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fuelsystem::AirRefuelSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::AirSourceKnb { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ecs::AirSourceKnbValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Airspeed { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AirspeedSetKnb { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Alt10000FtCnt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Alt1000FtCnt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Alt100FtCnt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Alt100FtPtr { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AltBaroSetKnb { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AltFlapsSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <controlinterface::AltFlapsSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AltModeLv { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AltPneuFlag { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AltPressureDrum0Cnt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AltPressureDrum1Cnt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AltPressureDrum2Cnt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AltPressureDrum3Cnt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AltRelBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mmc::AltRelBtnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AntiCollLightKnb { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <externallights::AntiCollLightKnbValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::AntiSkidSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AoaIndexBrtKnb { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AoaValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ApPitchSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ApRollSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <controlinterface::ApRollSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::ArStatusBrtKnb { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::BitSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <controlinterface::BitSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::BrakeChanSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <gearsystem::BrakeChanSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CanopyHandle { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <cockpitmechanics::CanopyHandleValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::CanopyJettThandle { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <cockpitmechanics::CanopyJettThandleValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::CanopySw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CanopyValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ClockCurrtimeH { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ClockCurrtimeMs { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ClockElapsed { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <clock::ClockElapsedValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::ClockElapsedTimeM { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ClockElapsedTimeSec { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ClockSet { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <clock::ClockSetValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::ClockWind { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Cmds01ExpCatSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <cmds::Cmds01ExpCatSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Cmds02ExpCatSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <cmds::Cmds02ExpCatSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CmdsChAmount { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CmdsChExpCatSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <cmds::CmdsChExpCatSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CmdsDispenseBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <cmds::CmdsDispenseBtnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CmdsFlAmount { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CmdsFlExpCatSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <cmds::CmdsFlExpCatSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CmdsJettSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <cmds::CmdsJettSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CmdsJmrSourcheSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <cmds::CmdsJmrSourcheSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CmdsModeKnb { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <cmds::CmdsModeKnbValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CmdsMwsSourcheSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <cmds::CmdsMwsSourcheSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CmdsO1Amount { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CmdsO2Amount { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CmdsProgKnb { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <cmds::CmdsProgKnbValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CmdsPwrSourcheSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <cmds::CmdsPwrSourcheSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CockpitAlititude { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Comm1ModeKnb { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <audiopanel::Comm1ModeKnbValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Comm1PwrKnb { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Comm2ModeKnb { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <audiopanel::Comm2ModeKnbValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Comm2PwrKnb { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DedLine1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DedLine2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DedLine3 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DedLine4 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DedLine5 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DefogLever { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ecs::DefogLeverValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::DigiBakSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <controlinterface::DigiBakSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::DlSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <avionicpanel::DlSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::DnLockBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <gearsystem::DnLockBtnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Ecm1Btn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <ecm::Ecm1BtnValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::Ecm2Btn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <ecm::Ecm2BtnValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::Ecm3Btn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <ecm::Ecm3BtnValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::Ecm4Btn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <ecm::Ecm4BtnValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::Ecm5Btn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <ecm::Ecm5BtnValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::Ecm6Btn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <ecm::Ecm6BtnValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::EcmBitBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ecm::EcmBitBtnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::EcmDimKnb { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::EcmFrmBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ecm::EcmFrmBtnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::EcmPwSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <ecm::EcmPwSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::EcmResetBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ecm::EcmResetBtnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::EcmSplBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ecm::EcmSplBtnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::EcmXmitSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ecm::EcmXmitSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::EhsiCrsSet { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ehsi::EhsiCrsSetValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::EhsiCrsSetKnb { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::EhsiHdgSetBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ehsi::EhsiHdgSetBtnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::EhsiHdgSetKnb { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::EhsiMode { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ehsi::EhsiModeValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::ElecCaution { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <electricsystem::ElecCautionValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::EmergStroreJett { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mmc::EmergStroreJettValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::EngAntiIce { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <engine::EngAntiIceValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::EngContSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <engine::EngContSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::EngContSwCover { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <engine::EngContSwCoverValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::EngineFeedKnb { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <externallights::EngineFeedKnbValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::EngineFtit { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::EngineNozzlePosition { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::EngineOilPressure { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::EngineTachometer { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::EpuGenTestSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <electricsystem::EpuGenTestSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::EpuSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <epu::EpuSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::EpuSwCoverOff { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <epu::EpuSwCoverOffValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::EpuSwCoverOn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <epu::EpuSwCoverOnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::ExtFormationLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ExtFuelTransSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fuelsystem::ExtFuelTransSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::ExtPositionLightFuselage { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ExtPositionLightsWing { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ExtSpeedBrakeLeft { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ExtSpeedBrakeRight { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ExtStrobeTail { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ExtTailLight { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ExtWowLeft { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ExtWowNose { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ExtWowRight { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FAckBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <ufc::FAckBtnValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::FcrPwrSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <sensorpanel::FcrPwrSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::FireOheatDetectBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <engine::FireOheatDetectBtnValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FlcsPwrTestSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FlcsResetSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <controlinterface::FlcsResetSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::FloodConsolesBrtKnb { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FloodInstPnlBrtKnb { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FlowIndicator { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FlowIndicatorLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::FormLightKnb { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelAl { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelFr { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelMasterCv { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fuelsystem::FuelMasterCvValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FuelMasterSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fuelsystem::FuelMasterSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FuelQtySelKnb { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fuelsystem::FuelQtySelKnbValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FuelQtySelTKnb { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fuelflowcounter100 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fuelflowcounter10k { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fuelflowcounter1k { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fueltotalizer100 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fueltotalizer10k { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fueltotalizer1k { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::GearHandle { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <gearsystem::GearHandleValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::GndJettEnableSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mmc::GndJettEnableSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::GpsSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <avionicpanel::GpsSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::HdptSwL { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <sensorpanel::HdptSwLValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::HdptSwR { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <sensorpanel::HdptSwRValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::HideStick { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <cockpitmechanics::HideStickValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::HmcsIntKnb { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HookSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <gearsystem::HookSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::HornSilenceBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <gearsystem::HornSilenceBtnValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::HotMicSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <audiopanel::HotMicSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::HudAltSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <hudcontrolpanel::HudAltSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::HudBrtSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <hudcontrolpanel::HudBrtSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::HudDedDataSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <hudcontrolpanel::HudDedDataSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::HudDepressRetSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <hudcontrolpanel::HudDepressRetSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::HudFpMarkerSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <hudcontrolpanel::HudFpMarkerSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::HudScalesSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <hudcontrolpanel::HudScalesSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::HudSpeedSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <hudcontrolpanel::HudSpeedSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::HudTestSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <hudcontrolpanel::HudTestSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::HydrazinVolume { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IcpAaModeBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ufc::IcpAaModeBtnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IcpAgModeBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ufc::IcpAgModeBtnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IcpBtn0 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <ufc::IcpBtn0Value as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::IcpBtn1 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <ufc::IcpBtn1Value as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::IcpBtn2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <ufc::IcpBtn2Value as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::IcpBtn3 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <ufc::IcpBtn3Value as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::IcpBtn4 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <ufc::IcpBtn4Value as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::IcpBtn5 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <ufc::IcpBtn5Value as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::IcpBtn6 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <ufc::IcpBtn6Value as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::IcpBtn7 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <ufc::IcpBtn7Value as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::IcpBtn8 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <ufc::IcpBtn8Value as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::IcpBtn9 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <ufc::IcpBtn9Value as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::IcpCom1Btn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ufc::IcpCom1BtnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IcpCom2Btn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ufc::IcpCom2BtnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IcpDataRtnSeqSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IcpDataUpDnSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IcpDedSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IcpDriftSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IcpEntrBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ufc::IcpEntrBtnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IcpFlirGainSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ufc::IcpFlirGainSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IcpFlirSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IcpHudBrtKnb { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IcpIffBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ufc::IcpIffBtnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IcpListBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ufc::IcpListBtnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IcpRasterBrtKnb { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IcpRasterContrKnb { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IcpRclBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ufc::IcpRclBtnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IcpReticleDepressKnb { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IcpWxBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <ufc::IcpWxBtnValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::IffAntSelSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <audiopanel::IffAntSelSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IffCIKnb { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <iff::IffCIKnbValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::IffCodeDrumDigit1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IffCodeDrumDigit2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IffCodeDrumDigit3 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IffCodeDrumDigit4 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IffEnableSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::IffEnableSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IffIdBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <ufc::IffIdBtnValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::IffM1Sel1 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::IffM1Sel1Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IffM1Sel2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::IffM1Sel2Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IffM3Sel1 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::IffM3Sel1Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IffM3Sel2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::IffM3Sel2Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IffM4CodeSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::IffM4CodeSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IffM4MonitorSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::IffM4MonitorSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IffM4ReplySw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::IffM4ReplySwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IffMasterKnb { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::IffMasterKnbValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IlsPwrKnb { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::InsKnb { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <avionicpanel::InsKnbValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IntercomKnb { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::JfsSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Ky58FillKnb { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ky58::Ky58FillKnbValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Ky58ModeKnb { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ky58::Ky58ModeKnbValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Ky58PwrKnb { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ky58::Ky58PwrKnbValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Ky58VolKnb { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LandTaxiLightSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <externallights::LandTaxiLightSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::LaserArmSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mmc::LaserArmSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LeFlapsSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <controlinterface::LeFlapsSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LightAcftBattFail { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightActive { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightAftFuelLow { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightAir { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightAntiSkid { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightAoaDn { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightAoaMid { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightAoaUp { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightArNws { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightAtfNot { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightAvionicsFault { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightBuc { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightCabinPress { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightCadc { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightCanopy { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightCaution1 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightCaution2 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightCaution3 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightCaution4 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightCaution5 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightCaution6 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightCmdsDisp { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightCmdsGo { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightCmdsNoGo { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightCmdsRdy { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightConsles { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightConslesFlood { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightDbuOn { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightDisc { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEcm1A { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEcm1F { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEcm1S { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEcm1T { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEcm2A { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEcm2F { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEcm2S { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEcm2T { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEcm3A { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEcm3F { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEcm3S { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEcm3T { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEcm4A { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEcm4F { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEcm4S { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEcm4T { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEcm5A { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEcm5F { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEcm5S { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEcm5T { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEcmA { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEcmF { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEcmFrmA { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEcmFrmF { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEcmFrmS { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEcmFrmT { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEcmS { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEcmSplA { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEcmSplF { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEcmSplS { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEcmSplT { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEcmT { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEdge { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEec { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightElec { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightElecSys { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEngFire { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEngine { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEngineFault { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEpu { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEpuGen { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEpuPmg { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightEquipHot { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightFlFail { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightFlRun { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightFlcs { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightFlcsFault { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightFlcsPmg { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightFlcsPwrA { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightFlcsPwrB { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightFlcsPwrC { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightFlcsPwrD { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightFlcsRly { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightFuelOilHot { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightFwdFuelLow { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightGearL { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightGearN { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightGearR { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightGearWarn { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightHook { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightHydOilPress { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightHydrazn { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightIff { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightInletIcing { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightInstPnl { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightInstPnlFlood { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightJfsRun { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightMainGen { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightMarkerBeacon { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightMasterCaution { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightNuclear { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightNwsFail { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightObogs { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightOverheat { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightOxyLow { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightProbeHeat { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightRadarAlt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightRdy { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightRwrActPower { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightRwrActivity { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightRwrAlt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightRwrAltLow { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightRwrHandoffH { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightRwrHandoffUp { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightRwrModeOpen { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightRwrModePri { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightRwrMslLaunch { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightRwrPower { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightRwrSearch { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightRwrShipUnk { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightRwrSystest { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightRwrTgtsepDn { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightRwrTgtsepUp { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightSeatNot { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightSec { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightStby { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightStbyGen { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightStoresConfig { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightTfFail { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightToFlcs { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LightToLdgConfig { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::MachIndicator { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MainPwrSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <electricsystem::MainPwrSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MalIndLtsBrtSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MalIndLtsTest { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <interiorlights::MalIndLtsTestValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::ManTfFlyupSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <controlinterface::ManTfFlyupSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::ManualPitchSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <controlinterface::ManualPitchSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::MapSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <avionicpanel::MapSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MasterArmSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mmc::MasterArmSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MasterCaution { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <interiorlights::MasterCautionValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::MasterLightSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <externallights::MasterLightSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::MaxAirspeed { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MaxPwrSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <engine::MaxPwrSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL1 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdleft::MfdL1Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL10 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdleft::MfdL10Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL11 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdleft::MfdL11Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL12 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdleft::MfdL12Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL13 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdleft::MfdL13Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL14 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdleft::MfdL14Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL15 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdleft::MfdL15Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL16 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdleft::MfdL16Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL17 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdleft::MfdL17Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL18 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdleft::MfdL18Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL19 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdleft::MfdL19Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdleft::MfdL2Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL20 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdleft::MfdL20Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL3 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdleft::MfdL3Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL4 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdleft::MfdL4Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL5 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdleft::MfdL5Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL6 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdleft::MfdL6Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL7 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdleft::MfdL7Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL8 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdleft::MfdL8Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL9 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdleft::MfdL9Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdLBrtSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdleft::MfdLBrtSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdLConSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdleft::MfdLConSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdLGainSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdleft::MfdLGainSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdLSymSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdleft::MfdLSymSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR1 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdright::MfdR1Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR10 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdright::MfdR10Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR11 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdright::MfdR11Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR12 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdright::MfdR12Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR13 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdright::MfdR13Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR14 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdright::MfdR14Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR15 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdright::MfdR15Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR16 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdright::MfdR16Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR17 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdright::MfdR17Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR18 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdright::MfdR18Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR19 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdright::MfdR19Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdright::MfdR2Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR20 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdright::MfdR20Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR3 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdright::MfdR3Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR4 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdright::MfdR4Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR5 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdright::MfdR5Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR6 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdright::MfdR6Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR7 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdright::MfdR7Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR8 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdright::MfdR8Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR9 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdright::MfdR9Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdRBrtSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdright::MfdRBrtSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdRConSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdright::MfdRConSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdRGainSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdright::MfdRGainSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdRSymSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <mfdright::MfdRSymSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <avionicpanel::MfdSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MidsLvtKnb { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <avionicpanel::MidsLvtKnbValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MmcPwrSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <avionicpanel::MmcPwrSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MslKnb { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ObogsSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <oxygensystem::ObogsSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::OxyDiluterLvr { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <oxygensystem::OxyDiluterLvrValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::OxyEmergLvr { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::OxySupplyLvr { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <oxygensystem::OxySupplyLvrValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::OxygenPressure { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PitchTrim { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Pitchtrimind { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PlainCipherSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ky58::PlainCipherSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::PosFlashLightSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <externallights::PosFlashLightSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::PosFuselageLightSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <externallights::PosFuselageLightSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::PosWingTailLightSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <externallights::PosWingTailLightSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::PriConsolesBrtKnb { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PriDataDisplayBrtKnb { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PriInstPnlBrtKnb { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ProbeHeatSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RdrAltPwrSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <sensorpanel::RdrAltPwrSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RfSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ufc::RfSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RollTrim { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Rolltrimind { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RwrActPwrBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <rwr::RwrActPwrBtnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RwrAltBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <rwr::RwrAltBtnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RwrHandoffBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <rwr::RwrHandoffBtnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RwrIndDimKnb { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RwrIntensKnb { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RwrLaunchBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <rwr::RwrLaunchBtnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RwrModeBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <rwr::RwrModeBtnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RwrPwrBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <rwr::RwrPwrBtnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RwrSearchBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <rwr::RwrSearchBtnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RwrSysTestBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <rwr::RwrSysTestBtnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RwrTBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rwr::RwrTBtnValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::RwrUnknownShipBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rwr::RwrUnknownShipBtnValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::SaiAircraftreferencesymbol { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SaiBank { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SaiBankArrow { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SaiCage { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <sai::SaiCageValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::SaiKnbArrow { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SaiOffFlag { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SaiPitch { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SaiPitchTrim { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SeatAdj { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SeatEjectHandle { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <cockpitmechanics::SeatEjectHandleValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::SeatEjectSafe { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <cockpitmechanics::SeatEjectSafeValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::SeatHeight { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SecVoiceKnb { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SetAirspeed { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SpeedbrakeIndicator { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::StStaSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <avionicpanel::StStaSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::StoresConfigSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <controlinterface::StoresConfigSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::SysaPressure { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SysbPressure { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TacanKnb { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TankIntertingSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <fuelsystem::TankIntertingSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::TempKnb { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <ecs::TempKnbValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::TfKnb { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ThreatKnb { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TrimApDiscSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <controlinterface::TrimApDiscSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::UfcSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <avionicpanel::UfcSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfAntSelSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <audiopanel::UhfAntSelSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfChanDisp { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UhfChanKnb { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UhfDoor { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <uhf::UhfDoorValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::UhfFreq0025Knb { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <uhf::UhfFreq0025KnbValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfFreq01Knb { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <uhf::UhfFreq01KnbValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfFreq100Knb { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <uhf::UhfFreq100KnbValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfFreq10Knb { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <uhf::UhfFreq10KnbValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfFreq1Knb { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <uhf::UhfFreq1KnbValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfFreqDisp { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UhfFuncKnb { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <uhf::UhfFuncKnbValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfModeKnb { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <uhf::UhfModeKnbValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfSquelchSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <uhf::UhfSquelchSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfStatusBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <uhf::UhfStatusBtnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfTestBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <uhf::UhfTestBtnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfToneBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <uhf::UhfToneBtnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfVolKnb { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VmsInhibitSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <audiopanel::VmsInhibitSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Vvi { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::YawTrim { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ZeroizeSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ky58::ZeroizeSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::ZeroizeSwCover { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ky58::ZeroizeSwCoverValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
        }
    }
}

pub struct F16c50Builder<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> {
    controls: Vec<RegisteredControl<'a>, MAX_CONTROLS>,
    _packet_capacity: PhantomData<ProtocolParser<MAX_PACKET_DATA>>,
}

impl<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize>
    F16c50Builder<'a, MAX_CONTROLS, MAX_PACKET_DATA>
{
    pub fn new() -> Self {
        Self {
            controls: Vec::new(),
            _packet_capacity: PhantomData,
        }
    }

    pub fn register_ab_reset_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AbResetSw {
                control: engine::AbResetSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_aux_flag<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiAuxFlag {
                control: adi::AdiAuxFlag::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_bank<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiBank {
                control: adi::AdiBank::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_bubble<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiBubble {
                control: adi::AdiBubble::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_gs_bar<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiGsBar {
                control: adi::AdiGsBar::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_gs_flag<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiGsFlag {
                control: adi::AdiGsFlag::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_gs_pointer<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiGsPointer {
                control: adi::AdiGsPointer::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_loc_bar<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiLocBar {
                control: adi::AdiLocBar::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_loc_flag<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiLocFlag {
                control: adi::AdiLocFlag::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_off_flag<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiOffFlag {
                control: adi::AdiOffFlag::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_pitch<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiPitch {
                control: adi::AdiPitch::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_pitch_trim<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiPitchTrim {
                control: adi::AdiPitchTrim::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_turnrate<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiTurnrate {
                control: adi::AdiTurnrate::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adv_mode_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(controlinterface::AdvModeSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdvModeSw {
                control: controlinterface::AdvModeSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_air_refuel_light_knb<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AirRefuelLightKnb {
                control: externallights::AirRefuelLightKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_air_refuel_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fuelsystem::AirRefuelSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AirRefuelSw {
                control: fuelsystem::AirRefuelSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_air_source_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ecs::AirSourceKnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AirSourceKnb {
                control: ecs::AirSourceKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_airspeed<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Airspeed {
                control: airspeedindicator::Airspeed::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_airspeed_set_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AirspeedSetKnb {
                control: airspeedindicator::AirspeedSetKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt10000_ft_cnt<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Alt10000FtCnt {
                control: altimeter::Alt10000FtCnt::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt1000_ft_cnt<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Alt1000FtCnt {
                control: altimeter::Alt1000FtCnt::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt100_ft_cnt<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Alt100FtCnt {
                control: altimeter::Alt100FtCnt::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt100_ft_ptr<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Alt100FtPtr {
                control: altimeter::Alt100FtPtr::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_baro_set_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltBaroSetKnb {
                control: altimeter::AltBaroSetKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_flaps_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(controlinterface::AltFlapsSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltFlapsSw {
                control: controlinterface::AltFlapsSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_mode_lv<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltModeLv {
                control: altimeter::AltModeLv::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_pneu_flag<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltPneuFlag {
                control: altimeter::AltPneuFlag::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_pressure_drum0_cnt<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltPressureDrum0Cnt {
                control: altimeter::AltPressureDrum0Cnt::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_pressure_drum1_cnt<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltPressureDrum1Cnt {
                control: altimeter::AltPressureDrum1Cnt::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_pressure_drum2_cnt<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltPressureDrum2Cnt {
                control: altimeter::AltPressureDrum2Cnt::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_pressure_drum3_cnt<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltPressureDrum3Cnt {
                control: altimeter::AltPressureDrum3Cnt::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_rel_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mmc::AltRelBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltRelBtn {
                control: mmc::AltRelBtn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_anti_coll_light_knb<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(externallights::AntiCollLightKnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AntiCollLightKnb {
                control: externallights::AntiCollLightKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_anti_skid_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AntiSkidSw {
                control: gearsystem::AntiSkidSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_aoa_index_brt_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AoaIndexBrtKnb {
                control: interiorlights::AoaIndexBrtKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_aoa_value<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AoaValue {
                control: aoa::AoaValue::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ap_pitch_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ApPitchSw {
                control: controlinterface::ApPitchSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ap_roll_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(controlinterface::ApRollSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ApRollSw {
                control: controlinterface::ApRollSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ar_status_brt_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ArStatusBrtKnb {
                control: interiorlights::ArStatusBrtKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_bit_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(controlinterface::BitSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::BitSw {
                control: controlinterface::BitSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_brake_chan_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(gearsystem::BrakeChanSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::BrakeChanSw {
                control: gearsystem::BrakeChanSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_canopy_handle<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(cockpitmechanics::CanopyHandleValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CanopyHandle {
                control: cockpitmechanics::CanopyHandle::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_canopy_jett_thandle<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(cockpitmechanics::CanopyJettThandleValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CanopyJettThandle {
                control: cockpitmechanics::CanopyJettThandle::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_canopy_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CanopySw {
                control: cockpitmechanics::CanopySw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_canopy_value<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CanopyValue {
                control: cockpitmechanics::CanopyValue::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_currtime_h<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockCurrtimeH {
                control: clock::ClockCurrtimeH::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_currtime_ms<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockCurrtimeMs {
                control: clock::ClockCurrtimeMs::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_elapsed<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(clock::ClockElapsedValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockElapsed {
                control: clock::ClockElapsed::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_elapsed_time_m<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockElapsedTimeM {
                control: clock::ClockElapsedTimeM::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_elapsed_time_sec<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockElapsedTimeSec {
                control: clock::ClockElapsedTimeSec::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_set<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(clock::ClockSetValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockSet {
                control: clock::ClockSet::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_wind<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockWind {
                control: clock::ClockWind::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cmds01_exp_cat_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(cmds::Cmds01ExpCatSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Cmds01ExpCatSw {
                control: cmds::Cmds01ExpCatSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cmds02_exp_cat_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(cmds::Cmds02ExpCatSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Cmds02ExpCatSw {
                control: cmds::Cmds02ExpCatSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cmds_ch_amount<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmdsChAmount {
                control: cmds::CmdsChAmount::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cmds_ch_exp_cat_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(cmds::CmdsChExpCatSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmdsChExpCatSw {
                control: cmds::CmdsChExpCatSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cmds_dispense_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(cmds::CmdsDispenseBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmdsDispenseBtn {
                control: cmds::CmdsDispenseBtn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cmds_fl_amount<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmdsFlAmount {
                control: cmds::CmdsFlAmount::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cmds_fl_exp_cat_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(cmds::CmdsFlExpCatSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmdsFlExpCatSw {
                control: cmds::CmdsFlExpCatSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cmds_jett_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(cmds::CmdsJettSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmdsJettSw {
                control: cmds::CmdsJettSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cmds_jmr_sourche_sw<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(cmds::CmdsJmrSourcheSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmdsJmrSourcheSw {
                control: cmds::CmdsJmrSourcheSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cmds_mode_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(cmds::CmdsModeKnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmdsModeKnb {
                control: cmds::CmdsModeKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cmds_mws_sourche_sw<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(cmds::CmdsMwsSourcheSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmdsMwsSourcheSw {
                control: cmds::CmdsMwsSourcheSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cmds_o1_amount<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmdsO1Amount {
                control: cmds::CmdsO1Amount::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cmds_o2_amount<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmdsO2Amount {
                control: cmds::CmdsO2Amount::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cmds_prog_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(cmds::CmdsProgKnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmdsProgKnb {
                control: cmds::CmdsProgKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cmds_pwr_sourche_sw<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(cmds::CmdsPwrSourcheSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmdsPwrSourcheSw {
                control: cmds::CmdsPwrSourcheSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cockpit_alititude<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CockpitAlititude {
                control: oxygensystem::CockpitAlititude::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_comm1_mode_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(audiopanel::Comm1ModeKnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Comm1ModeKnb {
                control: audiopanel::Comm1ModeKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_comm1_pwr_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Comm1PwrKnb {
                control: audiopanel::Comm1PwrKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_comm2_mode_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(audiopanel::Comm2ModeKnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Comm2ModeKnb {
                control: audiopanel::Comm2ModeKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_comm2_pwr_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Comm2PwrKnb {
                control: audiopanel::Comm2PwrKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ded_line1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::DedLine1 {
                control: dedoutputdata::DedLine1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ded_line2<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::DedLine2 {
                control: dedoutputdata::DedLine2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ded_line3<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::DedLine3 {
                control: dedoutputdata::DedLine3::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ded_line4<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::DedLine4 {
                control: dedoutputdata::DedLine4::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ded_line5<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::DedLine5 {
                control: dedoutputdata::DedLine5::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_defog_lever<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ecs::DefogLeverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::DefogLever {
                control: ecs::DefogLever::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_digi_bak_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(controlinterface::DigiBakSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::DigiBakSw {
                control: controlinterface::DigiBakSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dl_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(avionicpanel::DlSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::DlSw {
                control: avionicpanel::DlSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dn_lock_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(gearsystem::DnLockBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::DnLockBtn {
                control: gearsystem::DnLockBtn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecm1_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ecm::Ecm1BtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ecm1Btn {
                control: ecm::Ecm1Btn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecm2_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ecm::Ecm2BtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ecm2Btn {
                control: ecm::Ecm2Btn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecm3_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ecm::Ecm3BtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ecm3Btn {
                control: ecm::Ecm3Btn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecm4_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ecm::Ecm4BtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ecm4Btn {
                control: ecm::Ecm4Btn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecm5_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ecm::Ecm5BtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ecm5Btn {
                control: ecm::Ecm5Btn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecm6_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ecm::Ecm6BtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ecm6Btn {
                control: ecm::Ecm6Btn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecm_bit_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ecm::EcmBitBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EcmBitBtn {
                control: ecm::EcmBitBtn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecm_dim_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EcmDimKnb {
                control: adi::EcmDimKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecm_frm_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ecm::EcmFrmBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EcmFrmBtn {
                control: ecm::EcmFrmBtn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecm_pw_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ecm::EcmPwSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EcmPwSw {
                control: ecm::EcmPwSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecm_reset_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ecm::EcmResetBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EcmResetBtn {
                control: ecm::EcmResetBtn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecm_spl_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ecm::EcmSplBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EcmSplBtn {
                control: ecm::EcmSplBtn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecm_xmit_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ecm::EcmXmitSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EcmXmitSw {
                control: ecm::EcmXmitSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ehsi_crs_set<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ehsi::EhsiCrsSetValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EhsiCrsSet {
                control: ehsi::EhsiCrsSet::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ehsi_crs_set_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EhsiCrsSetKnb {
                control: ehsi::EhsiCrsSetKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ehsi_hdg_set_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ehsi::EhsiHdgSetBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EhsiHdgSetBtn {
                control: ehsi::EhsiHdgSetBtn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ehsi_hdg_set_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EhsiHdgSetKnb {
                control: ehsi::EhsiHdgSetKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ehsi_mode<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ehsi::EhsiModeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EhsiMode {
                control: ehsi::EhsiMode::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_elec_caution<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electricsystem::ElecCautionValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ElecCaution {
                control: electricsystem::ElecCaution::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_emerg_strore_jett<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mmc::EmergStroreJettValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EmergStroreJett {
                control: mmc::EmergStroreJett::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_eng_anti_ice<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(engine::EngAntiIceValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngAntiIce {
                control: engine::EngAntiIce::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_eng_cont_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(engine::EngContSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngContSw {
                control: engine::EngContSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_eng_cont_sw_cover<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(engine::EngContSwCoverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngContSwCover {
                control: engine::EngContSwCover::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_engine_feed_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(externallights::EngineFeedKnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngineFeedKnb {
                control: externallights::EngineFeedKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_engine_ftit<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngineFtit {
                control: engine::EngineFtit::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_engine_nozzle_position<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngineNozzlePosition {
                control: engine::EngineNozzlePosition::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_engine_oil_pressure<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngineOilPressure {
                control: engine::EngineOilPressure::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_engine_tachometer<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngineTachometer {
                control: engine::EngineTachometer::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_epu_gen_test_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electricsystem::EpuGenTestSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EpuGenTestSw {
                control: electricsystem::EpuGenTestSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_epu_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(epu::EpuSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EpuSw {
                control: epu::EpuSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_epu_sw_cover_off<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(epu::EpuSwCoverOffValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EpuSwCoverOff {
                control: epu::EpuSwCoverOff::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_epu_sw_cover_on<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(epu::EpuSwCoverOnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EpuSwCoverOn {
                control: epu::EpuSwCoverOn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_formation_lights<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtFormationLights {
                control: externalaircraftmodel::ExtFormationLights::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_fuel_trans_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fuelsystem::ExtFuelTransSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtFuelTransSw {
                control: fuelsystem::ExtFuelTransSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_position_light_fuselage<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtPositionLightFuselage {
                control: externalaircraftmodel::ExtPositionLightFuselage::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_position_lights_wing<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtPositionLightsWing {
                control: externalaircraftmodel::ExtPositionLightsWing::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_speed_brake_left<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtSpeedBrakeLeft {
                control: externalaircraftmodel::ExtSpeedBrakeLeft::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_speed_brake_right<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtSpeedBrakeRight {
                control: externalaircraftmodel::ExtSpeedBrakeRight::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_strobe_tail<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtStrobeTail {
                control: externalaircraftmodel::ExtStrobeTail::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_tail_light<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtTailLight {
                control: externalaircraftmodel::ExtTailLight::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_wow_left<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtWowLeft {
                control: externalaircraftmodel::ExtWowLeft::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_wow_nose<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtWowNose {
                control: externalaircraftmodel::ExtWowNose::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_wow_right<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtWowRight {
                control: externalaircraftmodel::ExtWowRight::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_f_ack_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ufc::FAckBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FAckBtn {
                control: ufc::FAckBtn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fcr_pwr_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(sensorpanel::FcrPwrSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FcrPwrSw {
                control: sensorpanel::FcrPwrSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fire_oheat_detect_btn<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(engine::FireOheatDetectBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FireOheatDetectBtn {
                control: engine::FireOheatDetectBtn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flcs_pwr_test_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlcsPwrTestSw {
                control: electricsystem::FlcsPwrTestSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flcs_reset_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(controlinterface::FlcsResetSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlcsResetSw {
                control: controlinterface::FlcsResetSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flood_consoles_brt_knb<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FloodConsolesBrtKnb {
                control: interiorlights::FloodConsolesBrtKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flood_inst_pnl_brt_knb<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FloodInstPnlBrtKnb {
                control: interiorlights::FloodInstPnlBrtKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flow_indicator<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlowIndicator {
                control: oxygensystem::FlowIndicator::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flow_indicator_light<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlowIndicatorLight {
                control: oxygensystem::FlowIndicatorLight::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_form_light_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FormLightKnb {
                control: externallights::FormLightKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_al<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelAl {
                control: fuelsystem::FuelAl::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_fr<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelFr {
                control: fuelsystem::FuelFr::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_master_cv<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fuelsystem::FuelMasterCvValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelMasterCv {
                control: fuelsystem::FuelMasterCv::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_master_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fuelsystem::FuelMasterSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelMasterSw {
                control: fuelsystem::FuelMasterSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_qty_sel_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fuelsystem::FuelQtySelKnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelQtySelKnb {
                control: fuelsystem::FuelQtySelKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_qty_sel_t_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelQtySelTKnb {
                control: fuelsystem::FuelQtySelTKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuelflowcounter100<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fuelflowcounter100 {
                control: fuelsystem::Fuelflowcounter100::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuelflowcounter10k<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fuelflowcounter10k {
                control: fuelsystem::Fuelflowcounter10k::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuelflowcounter1k<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fuelflowcounter1k {
                control: fuelsystem::Fuelflowcounter1k::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fueltotalizer100<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fueltotalizer100 {
                control: fuelsystem::Fueltotalizer100::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fueltotalizer10k<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fueltotalizer10k {
                control: fuelsystem::Fueltotalizer10k::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fueltotalizer1k<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fueltotalizer1k {
                control: fuelsystem::Fueltotalizer1k::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gear_handle<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(gearsystem::GearHandleValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::GearHandle {
                control: gearsystem::GearHandle::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gnd_jett_enable_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mmc::GndJettEnableSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::GndJettEnableSw {
                control: mmc::GndJettEnableSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gps_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(avionicpanel::GpsSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::GpsSw {
                control: avionicpanel::GpsSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hdpt_sw_l<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(sensorpanel::HdptSwLValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HdptSwL {
                control: sensorpanel::HdptSwL::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hdpt_sw_r<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(sensorpanel::HdptSwRValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HdptSwR {
                control: sensorpanel::HdptSwR::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hide_stick<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(cockpitmechanics::HideStickValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HideStick {
                control: cockpitmechanics::HideStick::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hmcs_int_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HmcsIntKnb {
                control: hmcs::HmcsIntKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hook_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(gearsystem::HookSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HookSw {
                control: gearsystem::HookSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_horn_silence_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(gearsystem::HornSilenceBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HornSilenceBtn {
                control: gearsystem::HornSilenceBtn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hot_mic_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(audiopanel::HotMicSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HotMicSw {
                control: audiopanel::HotMicSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_alt_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(hudcontrolpanel::HudAltSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudAltSw {
                control: hudcontrolpanel::HudAltSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_brt_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(hudcontrolpanel::HudBrtSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudBrtSw {
                control: hudcontrolpanel::HudBrtSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_ded_data_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(hudcontrolpanel::HudDedDataSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudDedDataSw {
                control: hudcontrolpanel::HudDedDataSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_depress_ret_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(hudcontrolpanel::HudDepressRetSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudDepressRetSw {
                control: hudcontrolpanel::HudDepressRetSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_fp_marker_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(hudcontrolpanel::HudFpMarkerSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudFpMarkerSw {
                control: hudcontrolpanel::HudFpMarkerSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_scales_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(hudcontrolpanel::HudScalesSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudScalesSw {
                control: hudcontrolpanel::HudScalesSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_speed_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(hudcontrolpanel::HudSpeedSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudSpeedSw {
                control: hudcontrolpanel::HudSpeedSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_test_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(hudcontrolpanel::HudTestSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudTestSw {
                control: hudcontrolpanel::HudTestSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hydrazin_volume<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HydrazinVolume {
                control: epu::HydrazinVolume::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_aa_mode_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ufc::IcpAaModeBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpAaModeBtn {
                control: ufc::IcpAaModeBtn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_ag_mode_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ufc::IcpAgModeBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpAgModeBtn {
                control: ufc::IcpAgModeBtn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_btn0<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ufc::IcpBtn0Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpBtn0 {
                control: ufc::IcpBtn0::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_btn1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ufc::IcpBtn1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpBtn1 {
                control: ufc::IcpBtn1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_btn2<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ufc::IcpBtn2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpBtn2 {
                control: ufc::IcpBtn2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_btn3<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ufc::IcpBtn3Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpBtn3 {
                control: ufc::IcpBtn3::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_btn4<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ufc::IcpBtn4Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpBtn4 {
                control: ufc::IcpBtn4::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_btn5<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ufc::IcpBtn5Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpBtn5 {
                control: ufc::IcpBtn5::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_btn6<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ufc::IcpBtn6Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpBtn6 {
                control: ufc::IcpBtn6::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_btn7<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ufc::IcpBtn7Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpBtn7 {
                control: ufc::IcpBtn7::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_btn8<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ufc::IcpBtn8Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpBtn8 {
                control: ufc::IcpBtn8::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_btn9<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ufc::IcpBtn9Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpBtn9 {
                control: ufc::IcpBtn9::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_com1_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ufc::IcpCom1BtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpCom1Btn {
                control: ufc::IcpCom1Btn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_com2_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ufc::IcpCom2BtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpCom2Btn {
                control: ufc::IcpCom2Btn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_data_rtn_seq_sw<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpDataRtnSeqSw {
                control: ufc::IcpDataRtnSeqSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_data_up_dn_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpDataUpDnSw {
                control: ufc::IcpDataUpDnSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_ded_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpDedSw {
                control: ufc::IcpDedSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_drift_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpDriftSw {
                control: ufc::IcpDriftSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_entr_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ufc::IcpEntrBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpEntrBtn {
                control: ufc::IcpEntrBtn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_flir_gain_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ufc::IcpFlirGainSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpFlirGainSw {
                control: ufc::IcpFlirGainSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_flir_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpFlirSw {
                control: ufc::IcpFlirSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_hud_brt_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpHudBrtKnb {
                control: ufc::IcpHudBrtKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_iff_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ufc::IcpIffBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpIffBtn {
                control: ufc::IcpIffBtn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_list_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ufc::IcpListBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpListBtn {
                control: ufc::IcpListBtn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_raster_brt_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpRasterBrtKnb {
                control: ufc::IcpRasterBrtKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_raster_contr_knb<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpRasterContrKnb {
                control: ufc::IcpRasterContrKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_rcl_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ufc::IcpRclBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpRclBtn {
                control: ufc::IcpRclBtn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_reticle_depress_knb<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpReticleDepressKnb {
                control: ufc::IcpReticleDepressKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_wx_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ufc::IcpWxBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpWxBtn {
                control: ufc::IcpWxBtn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_ant_sel_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(audiopanel::IffAntSelSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffAntSelSw {
                control: audiopanel::IffAntSelSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_c_i_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::IffCIKnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffCIKnb {
                control: iff::IffCIKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_code_drum_digit1<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffCodeDrumDigit1 {
                control: iff::IffCodeDrumDigit1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_code_drum_digit2<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffCodeDrumDigit2 {
                control: iff::IffCodeDrumDigit2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_code_drum_digit3<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffCodeDrumDigit3 {
                control: iff::IffCodeDrumDigit3::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_code_drum_digit4<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffCodeDrumDigit4 {
                control: iff::IffCodeDrumDigit4::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_enable_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::IffEnableSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffEnableSw {
                control: iff::IffEnableSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_id_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ufc::IffIdBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffIdBtn {
                control: ufc::IffIdBtn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_m1_sel1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::IffM1Sel1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffM1Sel1 {
                control: iff::IffM1Sel1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_m1_sel2<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::IffM1Sel2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffM1Sel2 {
                control: iff::IffM1Sel2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_m3_sel1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::IffM3Sel1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffM3Sel1 {
                control: iff::IffM3Sel1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_m3_sel2<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::IffM3Sel2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffM3Sel2 {
                control: iff::IffM3Sel2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_m4_code_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::IffM4CodeSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffM4CodeSw {
                control: iff::IffM4CodeSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_m4_monitor_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::IffM4MonitorSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffM4MonitorSw {
                control: iff::IffM4MonitorSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_m4_reply_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::IffM4ReplySwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffM4ReplySw {
                control: iff::IffM4ReplySw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_master_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::IffMasterKnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffMasterKnb {
                control: iff::IffMasterKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ils_pwr_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IlsPwrKnb {
                control: audiopanel::IlsPwrKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ins_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(avionicpanel::InsKnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::InsKnb {
                control: avionicpanel::InsKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_intercom_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IntercomKnb {
                control: audiopanel::IntercomKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_jfs_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::JfsSw {
                control: engine::JfsSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ky58_fill_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ky58::Ky58FillKnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ky58FillKnb {
                control: ky58::Ky58FillKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ky58_mode_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ky58::Ky58ModeKnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ky58ModeKnb {
                control: ky58::Ky58ModeKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ky58_pwr_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ky58::Ky58PwrKnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ky58PwrKnb {
                control: ky58::Ky58PwrKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ky58_vol_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ky58VolKnb {
                control: ky58::Ky58VolKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_land_taxi_light_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(externallights::LandTaxiLightSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LandTaxiLightSw {
                control: externallights::LandTaxiLightSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_laser_arm_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mmc::LaserArmSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LaserArmSw {
                control: mmc::LaserArmSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_le_flaps_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(controlinterface::LeFlapsSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeFlapsSw {
                control: controlinterface::LeFlapsSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_acft_batt_fail<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightAcftBattFail {
                control: warningcautionandindicatorlights::LightAcftBattFail::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_active<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightActive {
                control: warningcautionandindicatorlights::LightActive::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_aft_fuel_low<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightAftFuelLow {
                control: warningcautionandindicatorlights::LightAftFuelLow::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_air<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightAir {
                control: warningcautionandindicatorlights::LightAir::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_anti_skid<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightAntiSkid {
                control: warningcautionandindicatorlights::LightAntiSkid::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_aoa_dn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightAoaDn {
                control: warningcautionandindicatorlights::LightAoaDn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_aoa_mid<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightAoaMid {
                control: warningcautionandindicatorlights::LightAoaMid::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_aoa_up<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightAoaUp {
                control: warningcautionandindicatorlights::LightAoaUp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ar_nws<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightArNws {
                control: warningcautionandindicatorlights::LightArNws::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_atf_not<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightAtfNot {
                control: warningcautionandindicatorlights::LightAtfNot::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_avionics_fault<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightAvionicsFault {
                control: warningcautionandindicatorlights::LightAvionicsFault::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_buc<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightBuc {
                control: warningcautionandindicatorlights::LightBuc::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_cabin_press<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightCabinPress {
                control: warningcautionandindicatorlights::LightCabinPress::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_cadc<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightCadc {
                control: warningcautionandindicatorlights::LightCadc::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_canopy<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightCanopy {
                control: warningcautionandindicatorlights::LightCanopy::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_caution1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightCaution1 {
                control: warningcautionandindicatorlights::LightCaution1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_caution2<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightCaution2 {
                control: warningcautionandindicatorlights::LightCaution2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_caution3<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightCaution3 {
                control: warningcautionandindicatorlights::LightCaution3::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_caution4<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightCaution4 {
                control: warningcautionandindicatorlights::LightCaution4::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_caution5<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightCaution5 {
                control: warningcautionandindicatorlights::LightCaution5::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_caution6<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightCaution6 {
                control: warningcautionandindicatorlights::LightCaution6::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_cmds_disp<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightCmdsDisp {
                control: warningcautionandindicatorlights::LightCmdsDisp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_cmds_go<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightCmdsGo {
                control: warningcautionandindicatorlights::LightCmdsGo::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_cmds_no_go<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightCmdsNoGo {
                control: warningcautionandindicatorlights::LightCmdsNoGo::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_cmds_rdy<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightCmdsRdy {
                control: warningcautionandindicatorlights::LightCmdsRdy::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_consles<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightConsles {
                control: warningcautionandindicatorlights::LightConsles::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_consles_flood<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightConslesFlood {
                control: warningcautionandindicatorlights::LightConslesFlood::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_dbu_on<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightDbuOn {
                control: warningcautionandindicatorlights::LightDbuOn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_disc<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightDisc {
                control: warningcautionandindicatorlights::LightDisc::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm1_a<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm1A {
                control: warningcautionandindicatorlights::LightEcm1A::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm1_f<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm1F {
                control: warningcautionandindicatorlights::LightEcm1F::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm1_s<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm1S {
                control: warningcautionandindicatorlights::LightEcm1S::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm1_t<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm1T {
                control: warningcautionandindicatorlights::LightEcm1T::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm2_a<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm2A {
                control: warningcautionandindicatorlights::LightEcm2A::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm2_f<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm2F {
                control: warningcautionandindicatorlights::LightEcm2F::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm2_s<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm2S {
                control: warningcautionandindicatorlights::LightEcm2S::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm2_t<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm2T {
                control: warningcautionandindicatorlights::LightEcm2T::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm3_a<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm3A {
                control: warningcautionandindicatorlights::LightEcm3A::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm3_f<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm3F {
                control: warningcautionandindicatorlights::LightEcm3F::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm3_s<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm3S {
                control: warningcautionandindicatorlights::LightEcm3S::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm3_t<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm3T {
                control: warningcautionandindicatorlights::LightEcm3T::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm4_a<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm4A {
                control: warningcautionandindicatorlights::LightEcm4A::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm4_f<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm4F {
                control: warningcautionandindicatorlights::LightEcm4F::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm4_s<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm4S {
                control: warningcautionandindicatorlights::LightEcm4S::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm4_t<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm4T {
                control: warningcautionandindicatorlights::LightEcm4T::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm5_a<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm5A {
                control: warningcautionandindicatorlights::LightEcm5A::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm5_f<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm5F {
                control: warningcautionandindicatorlights::LightEcm5F::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm5_s<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm5S {
                control: warningcautionandindicatorlights::LightEcm5S::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm5_t<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm5T {
                control: warningcautionandindicatorlights::LightEcm5T::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm_a<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcmA {
                control: warningcautionandindicatorlights::LightEcmA::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm_f<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcmF {
                control: warningcautionandindicatorlights::LightEcmF::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm_frm_a<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcmFrmA {
                control: warningcautionandindicatorlights::LightEcmFrmA::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm_frm_f<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcmFrmF {
                control: warningcautionandindicatorlights::LightEcmFrmF::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm_frm_s<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcmFrmS {
                control: warningcautionandindicatorlights::LightEcmFrmS::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm_frm_t<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcmFrmT {
                control: warningcautionandindicatorlights::LightEcmFrmT::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm_s<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcmS {
                control: warningcautionandindicatorlights::LightEcmS::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm_spl_a<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcmSplA {
                control: warningcautionandindicatorlights::LightEcmSplA::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm_spl_f<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcmSplF {
                control: warningcautionandindicatorlights::LightEcmSplF::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm_spl_s<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcmSplS {
                control: warningcautionandindicatorlights::LightEcmSplS::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm_spl_t<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcmSplT {
                control: warningcautionandindicatorlights::LightEcmSplT::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm_t<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcmT {
                control: warningcautionandindicatorlights::LightEcmT::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_edge<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEdge {
                control: warningcautionandindicatorlights::LightEdge::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_eec<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEec {
                control: warningcautionandindicatorlights::LightEec::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_elec<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightElec {
                control: warningcautionandindicatorlights::LightElec::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_elec_sys<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightElecSys {
                control: warningcautionandindicatorlights::LightElecSys::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_eng_fire<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEngFire {
                control: warningcautionandindicatorlights::LightEngFire::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_engine<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEngine {
                control: warningcautionandindicatorlights::LightEngine::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_engine_fault<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEngineFault {
                control: warningcautionandindicatorlights::LightEngineFault::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_epu<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEpu {
                control: warningcautionandindicatorlights::LightEpu::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_epu_gen<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEpuGen {
                control: warningcautionandindicatorlights::LightEpuGen::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_epu_pmg<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEpuPmg {
                control: warningcautionandindicatorlights::LightEpuPmg::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_equip_hot<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEquipHot {
                control: warningcautionandindicatorlights::LightEquipHot::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_fl_fail<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightFlFail {
                control: warningcautionandindicatorlights::LightFlFail::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_fl_run<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightFlRun {
                control: warningcautionandindicatorlights::LightFlRun::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_flcs<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightFlcs {
                control: warningcautionandindicatorlights::LightFlcs::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_flcs_fault<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightFlcsFault {
                control: warningcautionandindicatorlights::LightFlcsFault::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_flcs_pmg<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightFlcsPmg {
                control: warningcautionandindicatorlights::LightFlcsPmg::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_flcs_pwr_a<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightFlcsPwrA {
                control: warningcautionandindicatorlights::LightFlcsPwrA::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_flcs_pwr_b<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightFlcsPwrB {
                control: warningcautionandindicatorlights::LightFlcsPwrB::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_flcs_pwr_c<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightFlcsPwrC {
                control: warningcautionandindicatorlights::LightFlcsPwrC::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_flcs_pwr_d<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightFlcsPwrD {
                control: warningcautionandindicatorlights::LightFlcsPwrD::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_flcs_rly<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightFlcsRly {
                control: warningcautionandindicatorlights::LightFlcsRly::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_fuel_oil_hot<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightFuelOilHot {
                control: warningcautionandindicatorlights::LightFuelOilHot::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_fwd_fuel_low<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightFwdFuelLow {
                control: warningcautionandindicatorlights::LightFwdFuelLow::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_gear_l<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightGearL {
                control: warningcautionandindicatorlights::LightGearL::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_gear_n<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightGearN {
                control: warningcautionandindicatorlights::LightGearN::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_gear_r<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightGearR {
                control: warningcautionandindicatorlights::LightGearR::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_gear_warn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightGearWarn {
                control: warningcautionandindicatorlights::LightGearWarn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_hook<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightHook {
                control: warningcautionandindicatorlights::LightHook::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_hyd_oil_press<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightHydOilPress {
                control: warningcautionandindicatorlights::LightHydOilPress::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_hydrazn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightHydrazn {
                control: warningcautionandindicatorlights::LightHydrazn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_iff<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightIff {
                control: warningcautionandindicatorlights::LightIff::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_inlet_icing<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightInletIcing {
                control: warningcautionandindicatorlights::LightInletIcing::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_inst_pnl<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightInstPnl {
                control: warningcautionandindicatorlights::LightInstPnl::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_inst_pnl_flood<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightInstPnlFlood {
                control: warningcautionandindicatorlights::LightInstPnlFlood::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_jfs_run<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightJfsRun {
                control: warningcautionandindicatorlights::LightJfsRun::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_main_gen<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightMainGen {
                control: warningcautionandindicatorlights::LightMainGen::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_marker_beacon<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightMarkerBeacon {
                control: warningcautionandindicatorlights::LightMarkerBeacon::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_master_caution<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightMasterCaution {
                control: warningcautionandindicatorlights::LightMasterCaution::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_nuclear<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightNuclear {
                control: warningcautionandindicatorlights::LightNuclear::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_nws_fail<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightNwsFail {
                control: warningcautionandindicatorlights::LightNwsFail::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_obogs<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightObogs {
                control: warningcautionandindicatorlights::LightObogs::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_overheat<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightOverheat {
                control: warningcautionandindicatorlights::LightOverheat::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_oxy_low<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightOxyLow {
                control: warningcautionandindicatorlights::LightOxyLow::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_probe_heat<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightProbeHeat {
                control: warningcautionandindicatorlights::LightProbeHeat::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_radar_alt<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightRadarAlt {
                control: warningcautionandindicatorlights::LightRadarAlt::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_rdy<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightRdy {
                control: warningcautionandindicatorlights::LightRdy::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_rwr_act_power<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightRwrActPower {
                control: warningcautionandindicatorlights::LightRwrActPower::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_rwr_activity<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightRwrActivity {
                control: warningcautionandindicatorlights::LightRwrActivity::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_rwr_alt<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightRwrAlt {
                control: warningcautionandindicatorlights::LightRwrAlt::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_rwr_alt_low<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightRwrAltLow {
                control: warningcautionandindicatorlights::LightRwrAltLow::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_rwr_handoff_h<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightRwrHandoffH {
                control: warningcautionandindicatorlights::LightRwrHandoffH::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_rwr_handoff_up<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightRwrHandoffUp {
                control: warningcautionandindicatorlights::LightRwrHandoffUp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_rwr_mode_open<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightRwrModeOpen {
                control: warningcautionandindicatorlights::LightRwrModeOpen::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_rwr_mode_pri<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightRwrModePri {
                control: warningcautionandindicatorlights::LightRwrModePri::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_rwr_msl_launch<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightRwrMslLaunch {
                control: warningcautionandindicatorlights::LightRwrMslLaunch::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_rwr_power<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightRwrPower {
                control: warningcautionandindicatorlights::LightRwrPower::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_rwr_search<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightRwrSearch {
                control: warningcautionandindicatorlights::LightRwrSearch::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_rwr_ship_unk<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightRwrShipUnk {
                control: warningcautionandindicatorlights::LightRwrShipUnk::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_rwr_systest<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightRwrSystest {
                control: warningcautionandindicatorlights::LightRwrSystest::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_rwr_tgtsep_dn<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightRwrTgtsepDn {
                control: warningcautionandindicatorlights::LightRwrTgtsepDn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_rwr_tgtsep_up<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightRwrTgtsepUp {
                control: warningcautionandindicatorlights::LightRwrTgtsepUp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_seat_not<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightSeatNot {
                control: warningcautionandindicatorlights::LightSeatNot::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_sec<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightSec {
                control: warningcautionandindicatorlights::LightSec::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_stby<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightStby {
                control: warningcautionandindicatorlights::LightStby::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_stby_gen<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightStbyGen {
                control: warningcautionandindicatorlights::LightStbyGen::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_stores_config<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightStoresConfig {
                control: warningcautionandindicatorlights::LightStoresConfig::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_tf_fail<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightTfFail {
                control: warningcautionandindicatorlights::LightTfFail::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_to_flcs<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightToFlcs {
                control: warningcautionandindicatorlights::LightToFlcs::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_to_ldg_config<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightToLdgConfig {
                control: warningcautionandindicatorlights::LightToLdgConfig::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mach_indicator<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MachIndicator {
                control: airspeedindicator::MachIndicator::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_main_pwr_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electricsystem::MainPwrSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MainPwrSw {
                control: electricsystem::MainPwrSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mal_ind_lts_brt_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MalIndLtsBrtSw {
                control: externallights::MalIndLtsBrtSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mal_ind_lts_test<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(interiorlights::MalIndLtsTestValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MalIndLtsTest {
                control: interiorlights::MalIndLtsTest::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_man_tf_flyup_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(controlinterface::ManTfFlyupSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ManTfFlyupSw {
                control: controlinterface::ManTfFlyupSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_manual_pitch_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(controlinterface::ManualPitchSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ManualPitchSw {
                control: controlinterface::ManualPitchSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_map_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(avionicpanel::MapSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MapSw {
                control: avionicpanel::MapSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_master_arm_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mmc::MasterArmSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MasterArmSw {
                control: mmc::MasterArmSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_master_caution<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(interiorlights::MasterCautionValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MasterCaution {
                control: interiorlights::MasterCaution::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_master_light_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(externallights::MasterLightSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MasterLightSw {
                control: externallights::MasterLightSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_max_airspeed<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MaxAirspeed {
                control: airspeedindicator::MaxAirspeed::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_max_pwr_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(engine::MaxPwrSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MaxPwrSw {
                control: engine::MaxPwrSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdleft::MfdL1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL1 {
                control: mfdleft::MfdL1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l10<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdleft::MfdL10Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL10 {
                control: mfdleft::MfdL10::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l11<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdleft::MfdL11Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL11 {
                control: mfdleft::MfdL11::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l12<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdleft::MfdL12Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL12 {
                control: mfdleft::MfdL12::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l13<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdleft::MfdL13Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL13 {
                control: mfdleft::MfdL13::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l14<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdleft::MfdL14Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL14 {
                control: mfdleft::MfdL14::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l15<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdleft::MfdL15Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL15 {
                control: mfdleft::MfdL15::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l16<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdleft::MfdL16Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL16 {
                control: mfdleft::MfdL16::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l17<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdleft::MfdL17Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL17 {
                control: mfdleft::MfdL17::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l18<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdleft::MfdL18Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL18 {
                control: mfdleft::MfdL18::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l19<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdleft::MfdL19Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL19 {
                control: mfdleft::MfdL19::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l2<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdleft::MfdL2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL2 {
                control: mfdleft::MfdL2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l20<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdleft::MfdL20Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL20 {
                control: mfdleft::MfdL20::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l3<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdleft::MfdL3Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL3 {
                control: mfdleft::MfdL3::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l4<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdleft::MfdL4Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL4 {
                control: mfdleft::MfdL4::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l5<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdleft::MfdL5Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL5 {
                control: mfdleft::MfdL5::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l6<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdleft::MfdL6Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL6 {
                control: mfdleft::MfdL6::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l7<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdleft::MfdL7Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL7 {
                control: mfdleft::MfdL7::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l8<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdleft::MfdL8Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL8 {
                control: mfdleft::MfdL8::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l9<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdleft::MfdL9Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL9 {
                control: mfdleft::MfdL9::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l_brt_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdleft::MfdLBrtSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdLBrtSw {
                control: mfdleft::MfdLBrtSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l_con_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdleft::MfdLConSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdLConSw {
                control: mfdleft::MfdLConSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l_gain_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdleft::MfdLGainSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdLGainSw {
                control: mfdleft::MfdLGainSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l_sym_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdleft::MfdLSymSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdLSymSw {
                control: mfdleft::MfdLSymSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdright::MfdR1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR1 {
                control: mfdright::MfdR1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r10<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdright::MfdR10Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR10 {
                control: mfdright::MfdR10::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r11<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdright::MfdR11Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR11 {
                control: mfdright::MfdR11::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r12<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdright::MfdR12Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR12 {
                control: mfdright::MfdR12::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r13<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdright::MfdR13Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR13 {
                control: mfdright::MfdR13::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r14<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdright::MfdR14Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR14 {
                control: mfdright::MfdR14::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r15<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdright::MfdR15Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR15 {
                control: mfdright::MfdR15::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r16<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdright::MfdR16Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR16 {
                control: mfdright::MfdR16::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r17<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdright::MfdR17Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR17 {
                control: mfdright::MfdR17::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r18<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdright::MfdR18Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR18 {
                control: mfdright::MfdR18::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r19<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdright::MfdR19Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR19 {
                control: mfdright::MfdR19::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r2<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdright::MfdR2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR2 {
                control: mfdright::MfdR2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r20<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdright::MfdR20Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR20 {
                control: mfdright::MfdR20::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r3<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdright::MfdR3Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR3 {
                control: mfdright::MfdR3::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r4<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdright::MfdR4Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR4 {
                control: mfdright::MfdR4::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r5<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdright::MfdR5Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR5 {
                control: mfdright::MfdR5::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r6<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdright::MfdR6Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR6 {
                control: mfdright::MfdR6::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r7<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdright::MfdR7Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR7 {
                control: mfdright::MfdR7::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r8<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdright::MfdR8Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR8 {
                control: mfdright::MfdR8::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r9<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdright::MfdR9Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR9 {
                control: mfdright::MfdR9::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r_brt_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdright::MfdRBrtSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdRBrtSw {
                control: mfdright::MfdRBrtSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r_con_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdright::MfdRConSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdRConSw {
                control: mfdright::MfdRConSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r_gain_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdright::MfdRGainSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdRGainSw {
                control: mfdright::MfdRGainSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r_sym_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(mfdright::MfdRSymSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdRSymSw {
                control: mfdright::MfdRSymSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(avionicpanel::MfdSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdSw {
                control: avionicpanel::MfdSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mids_lvt_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(avionicpanel::MidsLvtKnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MidsLvtKnb {
                control: avionicpanel::MidsLvtKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mmc_pwr_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(avionicpanel::MmcPwrSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MmcPwrSw {
                control: avionicpanel::MmcPwrSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_msl_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MslKnb {
                control: audiopanel::MslKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_obogs_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(oxygensystem::ObogsSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ObogsSw {
                control: oxygensystem::ObogsSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oxy_diluter_lvr<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(oxygensystem::OxyDiluterLvrValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::OxyDiluterLvr {
                control: oxygensystem::OxyDiluterLvr::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oxy_emerg_lvr<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OxyEmergLvr {
                control: oxygensystem::OxyEmergLvr::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oxy_supply_lvr<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(oxygensystem::OxySupplyLvrValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::OxySupplyLvr {
                control: oxygensystem::OxySupplyLvr::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oxygen_pressure<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OxygenPressure {
                control: oxygensystem::OxygenPressure::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pitch_trim<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::PitchTrim {
                control: controlinterface::PitchTrim::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pitchtrimind<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Pitchtrimind {
                control: trimindicators::Pitchtrimind::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_plain_cipher_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ky58::PlainCipherSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::PlainCipherSw {
                control: ky58::PlainCipherSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pos_flash_light_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(externallights::PosFlashLightSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::PosFlashLightSw {
                control: externallights::PosFlashLightSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pos_fuselage_light_sw<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(externallights::PosFuselageLightSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::PosFuselageLightSw {
                control: externallights::PosFuselageLightSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pos_wing_tail_light_sw<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(externallights::PosWingTailLightSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::PosWingTailLightSw {
                control: externallights::PosWingTailLightSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pri_consoles_brt_knb<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::PriConsolesBrtKnb {
                control: interiorlights::PriConsolesBrtKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pri_data_display_brt_knb<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::PriDataDisplayBrtKnb {
                control: interiorlights::PriDataDisplayBrtKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pri_inst_pnl_brt_knb<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::PriInstPnlBrtKnb {
                control: interiorlights::PriInstPnlBrtKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_probe_heat_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ProbeHeatSw {
                control: electricsystem::ProbeHeatSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rdr_alt_pwr_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(sensorpanel::RdrAltPwrSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RdrAltPwrSw {
                control: sensorpanel::RdrAltPwrSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rf_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ufc::RfSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RfSw {
                control: ufc::RfSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_roll_trim<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RollTrim {
                control: controlinterface::RollTrim::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rolltrimind<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Rolltrimind {
                control: trimindicators::Rolltrimind::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_act_pwr_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rwr::RwrActPwrBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrActPwrBtn {
                control: rwr::RwrActPwrBtn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_alt_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rwr::RwrAltBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrAltBtn {
                control: rwr::RwrAltBtn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_handoff_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rwr::RwrHandoffBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrHandoffBtn {
                control: rwr::RwrHandoffBtn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_ind_dim_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrIndDimKnb {
                control: rwr::RwrIndDimKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_intens_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrIntensKnb {
                control: rwr::RwrIntensKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_launch_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rwr::RwrLaunchBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrLaunchBtn {
                control: rwr::RwrLaunchBtn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_mode_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rwr::RwrModeBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrModeBtn {
                control: rwr::RwrModeBtn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_pwr_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rwr::RwrPwrBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrPwrBtn {
                control: rwr::RwrPwrBtn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_search_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rwr::RwrSearchBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrSearchBtn {
                control: rwr::RwrSearchBtn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_sys_test_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rwr::RwrSysTestBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrSysTestBtn {
                control: rwr::RwrSysTestBtn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_t_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rwr::RwrTBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrTBtn {
                control: rwr::RwrTBtn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_unknown_ship_btn<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rwr::RwrUnknownShipBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrUnknownShipBtn {
                control: rwr::RwrUnknownShipBtn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sai_aircraftreferencesymbol<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SaiAircraftreferencesymbol {
                control: sai::SaiAircraftreferencesymbol::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sai_bank<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SaiBank {
                control: sai::SaiBank::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sai_bank_arrow<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SaiBankArrow {
                control: sai::SaiBankArrow::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sai_cage<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(sai::SaiCageValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SaiCage {
                control: sai::SaiCage::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sai_knb_arrow<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SaiKnbArrow {
                control: sai::SaiKnbArrow::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sai_off_flag<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SaiOffFlag {
                control: sai::SaiOffFlag::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sai_pitch<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SaiPitch {
                control: sai::SaiPitch::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sai_pitch_trim<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SaiPitchTrim {
                control: sai::SaiPitchTrim::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_seat_adj<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SeatAdj {
                control: cockpitmechanics::SeatAdj::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_seat_eject_handle<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(cockpitmechanics::SeatEjectHandleValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SeatEjectHandle {
                control: cockpitmechanics::SeatEjectHandle::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_seat_eject_safe<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(cockpitmechanics::SeatEjectSafeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SeatEjectSafe {
                control: cockpitmechanics::SeatEjectSafe::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_seat_height<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SeatHeight {
                control: cockpitmechanics::SeatHeight::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sec_voice_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SecVoiceKnb {
                control: audiopanel::SecVoiceKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_set_airspeed<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SetAirspeed {
                control: airspeedindicator::SetAirspeed::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_speedbrake_indicator<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SpeedbrakeIndicator {
                control: speedbrake::SpeedbrakeIndicator::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_st_sta_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(avionicpanel::StStaSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::StStaSw {
                control: avionicpanel::StStaSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_stores_config_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(controlinterface::StoresConfigSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::StoresConfigSw {
                control: controlinterface::StoresConfigSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sysa_pressure<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SysaPressure {
                control: hydraulicpressureindicators::SysaPressure::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sysb_pressure<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SysbPressure {
                control: hydraulicpressureindicators::SysbPressure::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tacan_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TacanKnb {
                control: audiopanel::TacanKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tank_interting_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fuelsystem::TankIntertingSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::TankIntertingSw {
                control: fuelsystem::TankIntertingSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_temp_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ecs::TempKnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::TempKnb {
                control: ecs::TempKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tf_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TfKnb {
                control: audiopanel::TfKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_threat_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ThreatKnb {
                control: audiopanel::ThreatKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_trim_ap_disc_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(controlinterface::TrimApDiscSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::TrimApDiscSw {
                control: controlinterface::TrimApDiscSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(avionicpanel::UfcSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcSw {
                control: avionicpanel::UfcSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_ant_sel_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(audiopanel::UhfAntSelSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfAntSelSw {
                control: audiopanel::UhfAntSelSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_chan_disp<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfChanDisp {
                control: uhf::UhfChanDisp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_chan_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfChanKnb {
                control: uhf::UhfChanKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_door<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhf::UhfDoorValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfDoor {
                control: uhf::UhfDoor::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_freq0025_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhf::UhfFreq0025KnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfFreq0025Knb {
                control: uhf::UhfFreq0025Knb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_freq01_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhf::UhfFreq01KnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfFreq01Knb {
                control: uhf::UhfFreq01Knb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_freq100_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhf::UhfFreq100KnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfFreq100Knb {
                control: uhf::UhfFreq100Knb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_freq10_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhf::UhfFreq10KnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfFreq10Knb {
                control: uhf::UhfFreq10Knb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_freq1_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhf::UhfFreq1KnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfFreq1Knb {
                control: uhf::UhfFreq1Knb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_freq_disp<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfFreqDisp {
                control: uhf::UhfFreqDisp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_func_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhf::UhfFuncKnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfFuncKnb {
                control: uhf::UhfFuncKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_mode_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhf::UhfModeKnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfModeKnb {
                control: uhf::UhfModeKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_squelch_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhf::UhfSquelchSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfSquelchSw {
                control: uhf::UhfSquelchSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_status_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhf::UhfStatusBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfStatusBtn {
                control: uhf::UhfStatusBtn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_test_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhf::UhfTestBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfTestBtn {
                control: uhf::UhfTestBtn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_tone_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhf::UhfToneBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfToneBtn {
                control: uhf::UhfToneBtn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_vol_knb<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfVolKnb {
                control: uhf::UhfVolKnb::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vms_inhibit_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(audiopanel::VmsInhibitSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::VmsInhibitSw {
                control: audiopanel::VmsInhibitSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vvi<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Vvi {
                control: verticalvelocityindicator::Vvi::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_yaw_trim<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::YawTrim {
                control: controlinterface::YawTrim::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_zeroize_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ky58::ZeroizeSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ZeroizeSw {
                control: ky58::ZeroizeSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_zeroize_sw_cover<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ky58::ZeroizeSwCoverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ZeroizeSwCover {
                control: ky58::ZeroizeSwCover::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn build(self) -> F16c50<'a, MAX_CONTROLS, MAX_PACKET_DATA> {
        F16c50 {
            parser: ProtocolParser::new(),
            controls: self.controls,
            saw_frame: false,
        }
    }
}

impl<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> Default
    for F16c50Builder<'a, MAX_CONTROLS, MAX_PACKET_DATA>
{
    fn default() -> Self {
        Self::new()
    }
}

struct Dispatcher<'bus, 'callback, const MAX_CONTROLS: usize> {
    controls: &'bus mut Vec<RegisteredControl<'callback>, MAX_CONTROLS>,
    saw_frame: &'bus mut bool,
}

impl<'bus, 'callback, const MAX_CONTROLS: usize> Sink
    for Dispatcher<'bus, 'callback, MAX_CONTROLS>
{
    fn on_write(&mut self, packet: Packet<'_>) {
        for control in self.controls.iter_mut() {
            control.accept(packet);
        }
    }
    fn on_frame_sync(&mut self) {
        if *self.saw_frame {
            for control in self.controls.iter_mut() {
                control.commit();
            }
        } else {
            *self.saw_frame = true;
        }
    }
}

pub struct F16c50<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> {
    parser: ProtocolParser<MAX_PACKET_DATA>,
    controls: Vec<RegisteredControl<'a>, MAX_CONTROLS>,
    saw_frame: bool,
}

impl<const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize>
    F16c50<'_, MAX_CONTROLS, MAX_PACKET_DATA>
{
    pub fn feed(&mut self, bytes: &[u8]) -> Result<(), FeedError> {
        let mut dispatcher = Dispatcher {
            controls: &mut self.controls,
            saw_frame: &mut self.saw_frame,
        };
        self.parser
            .feed(bytes, &mut dispatcher)
            .map_err(FeedError::Parse)
    }
}
