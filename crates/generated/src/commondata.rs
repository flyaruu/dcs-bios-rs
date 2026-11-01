//! Generated from model-definitions/CommonData.json.
//! Do not edit manually; rerun dcs-bios-codegen instead.

#![allow(unused_imports)]

use core::marker::PhantomData;
use dcs_bios_rs_lib::{
    AnalogDial, AnalogGauge, Control, Display, FeedError, FixedStepDial, Led, LimitedDial,
    Metadata, Packet, ProtocolParser, Selector, Sink,
};
use heapless::Vec;

pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

pub mod altitude {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// Altitude MSL (ft)
    /// Output: Altitude MSL (ft)
    pub struct AltMslFt(Metadata<1076, 65535, 0>);
    impl AltMslFt {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AltMslFt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AltMslFt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod heading {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// Heading (Degrees)
    /// Output: Heading (Degrees)
    pub struct HdgDeg(Metadata<1078, 511, 0>);
    impl HdgDeg {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HdgDeg {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HdgDeg {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Heading (Fractional Degrees, divide by 127)
    /// Output: Heading (Fractional Degrees, divide by 127)
    pub struct HdgDegFrac(Metadata<1078, 65024, 9>);
    impl HdgDegFrac {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HdgDegFrac {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HdgDegFrac {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod metadata {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// DCS Bios Version
    /// Output: DCS Bios Version
    pub struct DcsBios(Display<1024, 6>);
    impl DcsBios {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for DcsBios {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for DcsBios {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Mission Start Time
    /// Output: Mission Start Time
    pub struct MissTime(Display<1080, 5>);
    impl MissTime {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for MissTime {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MissTime {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Model Time in sec
    /// Output: Model Time in sec
    pub struct ModTime(Display<1086, 5>);
    impl ModTime {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for ModTime {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ModTime {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Pilot Name
    /// Output: Pilot Name
    pub struct Pilotname(Display<1030, 24>);
    impl Pilotname {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for Pilotname {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Pilotname {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod position {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// Latitude Degrees
    /// Output: Latitude Degrees
    pub struct LatDeg(Metadata<1054, 32512, 8>);
    impl LatDeg {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LatDeg {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LatDeg {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Latitude Seconds
    /// Output: Latitude Seconds
    pub struct LatSec(Metadata<1056, 63, 0>);
    impl LatSec {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LatSec {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LatSec {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Latitude Fractional Seconds (divide by 65535)
    /// Output: Latitude Fractional Seconds (divide by 65535)
    pub struct LatSecFrac(Metadata<1058, 65535, 0>);
    impl LatSecFrac {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LatSecFrac {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LatSecFrac {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Latitude Direction
    /// Output: Latitude Direction
    pub struct LatZDir(Display<1054, 1>);
    impl LatZDir {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for LatZDir {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LatZDir {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Longitude Degrees
    /// Output: Longitude Degrees
    pub struct LonDeg(Metadata<1056, 16320, 6>);
    impl LonDeg {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LonDeg {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LonDeg {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Longitude Seconds
    /// Output: Longitude Seconds
    pub struct LonSec(Metadata<1072, 16128, 8>);
    impl LonSec {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LonSec {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LonSec {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Longitude Fractional Seconds (divide by 65535)
    /// Output: Longitude Fractional Seconds (divide by 65535)
    pub struct LonSecFrac(Metadata<1074, 65535, 0>);
    impl LonSecFrac {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LonSecFrac {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LonSecFrac {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Longitude Direction
    /// Output: Longitude Direction
    pub struct LonZDir(Display<1072, 1>);
    impl LonZDir {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for LonZDir {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LonZDir {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod speed {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// Indicated Airspeed KM H
    /// Output: Indicated Airspeed KM H
    pub struct IasEu(Display<1060, 4>);
    impl IasEu {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IasEu {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IasEu {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Indicated Airspeed KM H (Int)
    /// Output: Indicated Airspeed KM H (Int)
    pub struct IasEuInt(Metadata<1068, 65535, 0>);
    impl IasEuInt {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IasEuInt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IasEuInt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Indicated Airspeed KNT
    /// Output: Indicated Airspeed KNT
    pub struct IasUs(Display<1064, 4>);
    impl IasUs {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for IasUs {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IasUs {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Indicated Airspeed KNT (Int)
    /// Output: Indicated Airspeed KNT (Int)
    pub struct IasUsInt(Metadata<1070, 65535, 0>);
    impl IasUsInt {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IasUsInt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IasUsInt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod time {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// Model time high bits in hundredth of a second
    /// Output: Model time high bits in hundredth of a second
    pub struct TimeModelHigh(Metadata<1096, 65535, 0>);
    impl TimeModelHigh {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TimeModelHigh {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TimeModelHigh {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Model time low bits in hundredth of a second
    /// Output: Model Time low bits in hundredth of a second
    pub struct TimeModelLow(Metadata<1098, 65535, 0>);
    impl TimeModelLow {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TimeModelLow {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TimeModelLow {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Start time high bits in seconds
    /// Output: Start time high bits in seconds
    pub struct TimeStartHigh(Metadata<1092, 65535, 0>);
    impl TimeStartHigh {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TimeStartHigh {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TimeStartHigh {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Start time low bits in seconds
    /// Output: Start time low bits in seconds
    pub struct TimeStartLow(Metadata<1094, 65535, 0>);
    impl TimeStartLow {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TimeStartLow {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TimeStartLow {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub enum RegisteredControl<'a> {
    AltMslFt {
        control: altitude::AltMslFt,
        callback: IntegerCallback<'a>,
    },
    DcsBios {
        control: metadata::DcsBios,
        callback: DisplayCallback<'a>,
    },
    HdgDeg {
        control: heading::HdgDeg,
        callback: IntegerCallback<'a>,
    },
    HdgDegFrac {
        control: heading::HdgDegFrac,
        callback: IntegerCallback<'a>,
    },
    IasEu {
        control: speed::IasEu,
        callback: DisplayCallback<'a>,
    },
    IasEuInt {
        control: speed::IasEuInt,
        callback: IntegerCallback<'a>,
    },
    IasUs {
        control: speed::IasUs,
        callback: DisplayCallback<'a>,
    },
    IasUsInt {
        control: speed::IasUsInt,
        callback: IntegerCallback<'a>,
    },
    LatDeg {
        control: position::LatDeg,
        callback: IntegerCallback<'a>,
    },
    LatSec {
        control: position::LatSec,
        callback: IntegerCallback<'a>,
    },
    LatSecFrac {
        control: position::LatSecFrac,
        callback: IntegerCallback<'a>,
    },
    LatZDir {
        control: position::LatZDir,
        callback: DisplayCallback<'a>,
    },
    LonDeg {
        control: position::LonDeg,
        callback: IntegerCallback<'a>,
    },
    LonSec {
        control: position::LonSec,
        callback: IntegerCallback<'a>,
    },
    LonSecFrac {
        control: position::LonSecFrac,
        callback: IntegerCallback<'a>,
    },
    LonZDir {
        control: position::LonZDir,
        callback: DisplayCallback<'a>,
    },
    MissTime {
        control: metadata::MissTime,
        callback: DisplayCallback<'a>,
    },
    ModTime {
        control: metadata::ModTime,
        callback: DisplayCallback<'a>,
    },
    Pilotname {
        control: metadata::Pilotname,
        callback: DisplayCallback<'a>,
    },
    TimeModelHigh {
        control: time::TimeModelHigh,
        callback: IntegerCallback<'a>,
    },
    TimeModelLow {
        control: time::TimeModelLow,
        callback: IntegerCallback<'a>,
    },
    TimeStartHigh {
        control: time::TimeStartHigh,
        callback: IntegerCallback<'a>,
    },
    TimeStartLow {
        control: time::TimeStartLow,
        callback: IntegerCallback<'a>,
    },
}

impl RegisteredControl<'_> {
    fn accept(&mut self, packet: Packet<'_>) {
        match self {
            Self::AltMslFt { control, .. } => control.accept(packet),
            Self::DcsBios { control, .. } => control.accept(packet),
            Self::HdgDeg { control, .. } => control.accept(packet),
            Self::HdgDegFrac { control, .. } => control.accept(packet),
            Self::IasEu { control, .. } => control.accept(packet),
            Self::IasEuInt { control, .. } => control.accept(packet),
            Self::IasUs { control, .. } => control.accept(packet),
            Self::IasUsInt { control, .. } => control.accept(packet),
            Self::LatDeg { control, .. } => control.accept(packet),
            Self::LatSec { control, .. } => control.accept(packet),
            Self::LatSecFrac { control, .. } => control.accept(packet),
            Self::LatZDir { control, .. } => control.accept(packet),
            Self::LonDeg { control, .. } => control.accept(packet),
            Self::LonSec { control, .. } => control.accept(packet),
            Self::LonSecFrac { control, .. } => control.accept(packet),
            Self::LonZDir { control, .. } => control.accept(packet),
            Self::MissTime { control, .. } => control.accept(packet),
            Self::ModTime { control, .. } => control.accept(packet),
            Self::Pilotname { control, .. } => control.accept(packet),
            Self::TimeModelHigh { control, .. } => control.accept(packet),
            Self::TimeModelLow { control, .. } => control.accept(packet),
            Self::TimeStartHigh { control, .. } => control.accept(packet),
            Self::TimeStartLow { control, .. } => control.accept(packet),
        }
    }

    fn commit(&mut self) {
        match self {
            Self::AltMslFt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DcsBios { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HdgDeg { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HdgDegFrac { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IasEu { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IasEuInt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IasUs { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IasUsInt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LatDeg { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LatSec { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LatSecFrac { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LatZDir { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LonDeg { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LonSec { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LonSecFrac { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LonZDir { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MissTime { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ModTime { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Pilotname { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TimeModelHigh { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TimeModelLow { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TimeStartHigh { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TimeStartLow { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
        }
    }
}

pub struct CommondataBuilder<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> {
    controls: Vec<RegisteredControl<'a>, MAX_CONTROLS>,
    _packet_capacity: PhantomData<ProtocolParser<MAX_PACKET_DATA>>,
}

impl<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize>
    CommondataBuilder<'a, MAX_CONTROLS, MAX_PACKET_DATA>
{
    pub fn new() -> Self {
        Self {
            controls: Vec::new(),
            _packet_capacity: PhantomData,
        }
    }

    pub fn register_alt_msl_ft<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltMslFt {
                control: altitude::AltMslFt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dcs_bios<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::DcsBios {
                control: metadata::DcsBios::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hdg_deg<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HdgDeg {
                control: heading::HdgDeg::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hdg_deg_frac<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HdgDegFrac {
                control: heading::HdgDegFrac::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ias_eu<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IasEu {
                control: speed::IasEu::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ias_eu_int<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IasEuInt {
                control: speed::IasEuInt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ias_us<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::IasUs {
                control: speed::IasUs::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ias_us_int<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IasUsInt {
                control: speed::IasUsInt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lat_deg<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LatDeg {
                control: position::LatDeg::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lat_sec<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LatSec {
                control: position::LatSec::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lat_sec_frac<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LatSecFrac {
                control: position::LatSecFrac::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lat_z_dir<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::LatZDir {
                control: position::LatZDir::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lon_deg<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LonDeg {
                control: position::LonDeg::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lon_sec<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LonSec {
                control: position::LonSec::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lon_sec_frac<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LonSecFrac {
                control: position::LonSecFrac::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lon_z_dir<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::LonZDir {
                control: position::LonZDir::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_miss_time<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::MissTime {
                control: metadata::MissTime::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mod_time<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::ModTime {
                control: metadata::ModTime::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pilotname<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Pilotname {
                control: metadata::Pilotname::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_time_model_high<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TimeModelHigh {
                control: time::TimeModelHigh::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_time_model_low<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TimeModelLow {
                control: time::TimeModelLow::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_time_start_high<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TimeStartHigh {
                control: time::TimeStartHigh::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_time_start_low<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TimeStartLow {
                control: time::TimeStartLow::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn build(self) -> Commondata<'a, MAX_CONTROLS, MAX_PACKET_DATA> {
        Commondata {
            parser: ProtocolParser::new(),
            controls: self.controls,
            saw_frame: false,
        }
    }
}

impl<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> Default
    for CommondataBuilder<'a, MAX_CONTROLS, MAX_PACKET_DATA>
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

pub struct Commondata<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> {
    parser: ProtocolParser<MAX_PACKET_DATA>,
    controls: Vec<RegisteredControl<'a>, MAX_CONTROLS>,
    saw_frame: bool,
}

impl<const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize>
    Commondata<'_, MAX_CONTROLS, MAX_PACKET_DATA>
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
