//! Generated from model-definitions/FC3.json.
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

    /// Altitude
    /// Output: Altitude
    pub struct Fc3Altitude(Display<24576, 6>);
    impl Fc3Altitude {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for Fc3Altitude {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Fc3Altitude {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Altitude above Ground
    /// Output: Altitude above Ground
    pub struct Fc3AltitudeGround(Display<24582, 6>);
    impl Fc3AltitudeGround {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for Fc3AltitudeGround {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Fc3AltitudeGround {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Altitude above Sea Level
    /// Output: Altitude above Sea Level
    pub struct Fc3AltitudeSea(Display<24588, 6>);
    impl Fc3AltitudeSea {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for Fc3AltitudeSea {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Fc3AltitudeSea {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Radar Altitude
    /// Output: Radar Altitude
    pub struct Fc3RadarAltitude(Metadata<24602, 256, 8>);
    impl Fc3RadarAltitude {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Fc3RadarAltitude {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Fc3RadarAltitude {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod bar {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// Fuel Bar
    /// Output: Fuel Bar
    pub struct Fc3FuelBar(Metadata<24602, 15872, 9>);
    impl Fc3FuelBar {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Fc3FuelBar {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Fc3FuelBar {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// G Load Bar
    /// Output: G Load Bar
    pub struct Fc3GLoadBar(Metadata<24624, 31, 0>);
    impl Fc3GLoadBar {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Fc3GLoadBar {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Fc3GLoadBar {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Vertical Velocity Bar
    /// Output: Vertical Velocity Bar
    pub struct Fc3VviBar(Metadata<24624, 992, 5>);
    impl Fc3VviBar {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Fc3VviBar {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Fc3VviBar {
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

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// Left Position Light (red)
    /// Output: Left Position Light (red)
    pub struct ExtPositionLightLeft(Metadata<24602, 16384, 14>);
    impl ExtPositionLightLeft {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ExtPositionLightLeft {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ExtPositionLightLeft {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Right Position Light (green)
    /// Output: Right Position Light (green)
    pub struct ExtPositionLightRight(Metadata<24602, 32768, 15>);
    impl ExtPositionLightRight {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ExtPositionLightRight {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ExtPositionLightRight {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Left Speed Brake
    /// Output: Left Speed Brake
    pub struct ExtSpeedBrakeLeft(Metadata<24634, 65535, 0>);
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
    pub struct ExtSpeedBrakeRight(Metadata<24632, 65535, 0>);
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

    /// Strobe Light
    /// Output: Strobe Light
    pub struct ExtStrobe(Metadata<24624, 1024, 10>);
    impl ExtStrobe {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ExtStrobe {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ExtStrobe {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Weight ON Wheels Left Gear
    /// Output: Weight ON Wheels Left Gear
    pub struct ExtWowLeft(Metadata<24624, 8192, 13>);
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
    pub struct ExtWowNose(Metadata<24624, 2048, 11>);
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
    pub struct ExtWowRight(Metadata<24624, 4096, 12>);
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

pub mod float {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// ADI Bank
    /// Output: ADI Bank
    pub struct Fc3AdiBank(Metadata<24626, 255, 0>);
    impl Fc3AdiBank {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Fc3AdiBank {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Fc3AdiBank {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADI Pitch
    /// Output: ADI Pitch
    pub struct Fc3AdiPitch(Metadata<24626, 65280, 8>);
    impl Fc3AdiPitch {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Fc3AdiPitch {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Fc3AdiPitch {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADI Yaw
    /// Output: ADI Yaw
    pub struct Fc3AdiYaw(Metadata<24628, 255, 0>);
    impl Fc3AdiYaw {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Fc3AdiYaw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Fc3AdiYaw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Glide Deviation
    /// Output: Glide Deviation
    pub struct Fc3GlideDeviation(Metadata<24628, 65280, 8>);
    impl Fc3GlideDeviation {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Fc3GlideDeviation {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Fc3GlideDeviation {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Side Deviation
    /// Output: Side Deviation
    pub struct Fc3SideDeviation(Metadata<24630, 255, 0>);
    impl Fc3SideDeviation {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Fc3SideDeviation {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Fc3SideDeviation {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Slip Ball Position
    /// Output: Slip Ball Position
    pub struct Fc3SlipBallPosition(Metadata<24630, 65280, 8>);
    impl Fc3SlipBallPosition {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Fc3SlipBallPosition {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Fc3SlipBallPosition {
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

    /// Indicated Airspeed
    /// Output: Indicated Airspeed
    pub struct Fc3IndicatedAirspeed(Display<24608, 4>);
    impl Fc3IndicatedAirspeed {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for Fc3IndicatedAirspeed {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Fc3IndicatedAirspeed {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Mach Number
    /// Output: Mach Number
    pub struct Fc3MachNumber(Display<24612, 4>);
    impl Fc3MachNumber {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for Fc3MachNumber {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Fc3MachNumber {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// True Airspeed
    /// Output: True Airspeed
    pub struct Fc3TrueAirspeed(Display<24616, 4>);
    impl Fc3TrueAirspeed {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for Fc3TrueAirspeed {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Fc3TrueAirspeed {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Vertical Velocity
    /// Output: Vertical Velocity
    pub struct Fc3VerticalVelocity(Display<24620, 4>);
    impl Fc3VerticalVelocity {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for Fc3VerticalVelocity {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Fc3VerticalVelocity {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod string {
    use super::*;

    pub type IntegerCallback<'a> = &'a mut dyn FnMut(u16);
    pub type LedCallback<'a> = &'a mut dyn FnMut(bool);
    pub type DisplayCallback<'a> = &'a mut dyn FnMut(&[u8]);

    /// Angle of Attack
    /// Output: Angle of Attack
    pub struct Fc3AngleOfAttack(Display<24594, 4>);
    impl Fc3AngleOfAttack {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for Fc3AngleOfAttack {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Fc3AngleOfAttack {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Fuel Remaining
    /// Output: Fuel Remaining
    pub struct Fc3FuelAll(Display<24598, 5>);
    impl Fc3FuelAll {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for Fc3FuelAll {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Fc3FuelAll {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// G Load
    /// Output: G Load
    pub struct Fc3GLoad(Display<24604, 4>);
    impl Fc3GLoad {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for Fc3GLoad {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Fc3GLoad {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub enum RegisteredControl<'a> {
    ExtPositionLightLeft {
        control: externalaircraftmodel::ExtPositionLightLeft,
        callback: IntegerCallback<'a>,
    },
    ExtPositionLightRight {
        control: externalaircraftmodel::ExtPositionLightRight,
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
    ExtStrobe {
        control: externalaircraftmodel::ExtStrobe,
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
    Fc3AdiBank {
        control: float::Fc3AdiBank,
        callback: IntegerCallback<'a>,
    },
    Fc3AdiPitch {
        control: float::Fc3AdiPitch,
        callback: IntegerCallback<'a>,
    },
    Fc3AdiYaw {
        control: float::Fc3AdiYaw,
        callback: IntegerCallback<'a>,
    },
    Fc3Altitude {
        control: altitude::Fc3Altitude,
        callback: DisplayCallback<'a>,
    },
    Fc3AltitudeGround {
        control: altitude::Fc3AltitudeGround,
        callback: DisplayCallback<'a>,
    },
    Fc3AltitudeSea {
        control: altitude::Fc3AltitudeSea,
        callback: DisplayCallback<'a>,
    },
    Fc3AngleOfAttack {
        control: string::Fc3AngleOfAttack,
        callback: DisplayCallback<'a>,
    },
    Fc3FuelAll {
        control: string::Fc3FuelAll,
        callback: DisplayCallback<'a>,
    },
    Fc3FuelBar {
        control: bar::Fc3FuelBar,
        callback: IntegerCallback<'a>,
    },
    Fc3GLoad {
        control: string::Fc3GLoad,
        callback: DisplayCallback<'a>,
    },
    Fc3GLoadBar {
        control: bar::Fc3GLoadBar,
        callback: IntegerCallback<'a>,
    },
    Fc3GlideDeviation {
        control: float::Fc3GlideDeviation,
        callback: IntegerCallback<'a>,
    },
    Fc3IndicatedAirspeed {
        control: speed::Fc3IndicatedAirspeed,
        callback: DisplayCallback<'a>,
    },
    Fc3MachNumber {
        control: speed::Fc3MachNumber,
        callback: DisplayCallback<'a>,
    },
    Fc3RadarAltitude {
        control: altitude::Fc3RadarAltitude,
        callback: IntegerCallback<'a>,
    },
    Fc3SideDeviation {
        control: float::Fc3SideDeviation,
        callback: IntegerCallback<'a>,
    },
    Fc3SlipBallPosition {
        control: float::Fc3SlipBallPosition,
        callback: IntegerCallback<'a>,
    },
    Fc3TrueAirspeed {
        control: speed::Fc3TrueAirspeed,
        callback: DisplayCallback<'a>,
    },
    Fc3VerticalVelocity {
        control: speed::Fc3VerticalVelocity,
        callback: DisplayCallback<'a>,
    },
    Fc3VviBar {
        control: bar::Fc3VviBar,
        callback: IntegerCallback<'a>,
    },
}

impl RegisteredControl<'_> {
    fn accept(&mut self, packet: Packet<'_>) {
        match self {
            Self::ExtPositionLightLeft { control, .. } => control.accept(packet),
            Self::ExtPositionLightRight { control, .. } => control.accept(packet),
            Self::ExtSpeedBrakeLeft { control, .. } => control.accept(packet),
            Self::ExtSpeedBrakeRight { control, .. } => control.accept(packet),
            Self::ExtStrobe { control, .. } => control.accept(packet),
            Self::ExtWowLeft { control, .. } => control.accept(packet),
            Self::ExtWowNose { control, .. } => control.accept(packet),
            Self::ExtWowRight { control, .. } => control.accept(packet),
            Self::Fc3AdiBank { control, .. } => control.accept(packet),
            Self::Fc3AdiPitch { control, .. } => control.accept(packet),
            Self::Fc3AdiYaw { control, .. } => control.accept(packet),
            Self::Fc3Altitude { control, .. } => control.accept(packet),
            Self::Fc3AltitudeGround { control, .. } => control.accept(packet),
            Self::Fc3AltitudeSea { control, .. } => control.accept(packet),
            Self::Fc3AngleOfAttack { control, .. } => control.accept(packet),
            Self::Fc3FuelAll { control, .. } => control.accept(packet),
            Self::Fc3FuelBar { control, .. } => control.accept(packet),
            Self::Fc3GLoad { control, .. } => control.accept(packet),
            Self::Fc3GLoadBar { control, .. } => control.accept(packet),
            Self::Fc3GlideDeviation { control, .. } => control.accept(packet),
            Self::Fc3IndicatedAirspeed { control, .. } => control.accept(packet),
            Self::Fc3MachNumber { control, .. } => control.accept(packet),
            Self::Fc3RadarAltitude { control, .. } => control.accept(packet),
            Self::Fc3SideDeviation { control, .. } => control.accept(packet),
            Self::Fc3SlipBallPosition { control, .. } => control.accept(packet),
            Self::Fc3TrueAirspeed { control, .. } => control.accept(packet),
            Self::Fc3VerticalVelocity { control, .. } => control.accept(packet),
            Self::Fc3VviBar { control, .. } => control.accept(packet),
        }
    }

    fn commit(&mut self) {
        match self {
            Self::ExtPositionLightLeft { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ExtPositionLightRight { control, callback } => {
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
            Self::ExtStrobe { control, callback } => {
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
            Self::Fc3AdiBank { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fc3AdiPitch { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fc3AdiYaw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fc3Altitude { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fc3AltitudeGround { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fc3AltitudeSea { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fc3AngleOfAttack { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fc3FuelAll { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fc3FuelBar { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fc3GLoad { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fc3GLoadBar { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fc3GlideDeviation { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fc3IndicatedAirspeed { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fc3MachNumber { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fc3RadarAltitude { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fc3SideDeviation { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fc3SlipBallPosition { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fc3TrueAirspeed { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fc3VerticalVelocity { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Fc3VviBar { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
        }
    }
}

pub struct Fc3Builder<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> {
    controls: Vec<RegisteredControl<'a>, MAX_CONTROLS>,
    _packet_capacity: PhantomData<ProtocolParser<MAX_PACKET_DATA>>,
}

impl<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize>
    Fc3Builder<'a, MAX_CONTROLS, MAX_PACKET_DATA>
{
    pub fn new() -> Self {
        Self {
            controls: Vec::new(),
            _packet_capacity: PhantomData,
        }
    }

    pub fn register_ext_position_light_left<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtPositionLightLeft {
                control: externalaircraftmodel::ExtPositionLightLeft::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_position_light_right<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtPositionLightRight {
                control: externalaircraftmodel::ExtPositionLightRight::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_speed_brake_left<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtSpeedBrakeLeft {
                control: externalaircraftmodel::ExtSpeedBrakeLeft::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_speed_brake_right<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtSpeedBrakeRight {
                control: externalaircraftmodel::ExtSpeedBrakeRight::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_strobe<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtStrobe {
                control: externalaircraftmodel::ExtStrobe::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_wow_left<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtWowLeft {
                control: externalaircraftmodel::ExtWowLeft::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_wow_nose<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtWowNose {
                control: externalaircraftmodel::ExtWowNose::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_wow_right<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtWowRight {
                control: externalaircraftmodel::ExtWowRight::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fc3_adi_bank<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fc3AdiBank {
                control: float::Fc3AdiBank::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fc3_adi_pitch<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fc3AdiPitch {
                control: float::Fc3AdiPitch::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fc3_adi_yaw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fc3AdiYaw {
                control: float::Fc3AdiYaw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fc3_altitude<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fc3Altitude {
                control: altitude::Fc3Altitude::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fc3_altitude_ground<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fc3AltitudeGround {
                control: altitude::Fc3AltitudeGround::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fc3_altitude_sea<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fc3AltitudeSea {
                control: altitude::Fc3AltitudeSea::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fc3_angle_of_attack<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fc3AngleOfAttack {
                control: string::Fc3AngleOfAttack::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fc3_fuel_all<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fc3FuelAll {
                control: string::Fc3FuelAll::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fc3_fuel_bar<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fc3FuelBar {
                control: bar::Fc3FuelBar::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fc3_g_load<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fc3GLoad {
                control: string::Fc3GLoad::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fc3_g_load_bar<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fc3GLoadBar {
                control: bar::Fc3GLoadBar::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fc3_glide_deviation<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fc3GlideDeviation {
                control: float::Fc3GlideDeviation::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fc3_indicated_airspeed<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fc3IndicatedAirspeed {
                control: speed::Fc3IndicatedAirspeed::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fc3_mach_number<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fc3MachNumber {
                control: speed::Fc3MachNumber::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fc3_radar_altitude<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fc3RadarAltitude {
                control: altitude::Fc3RadarAltitude::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fc3_side_deviation<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fc3SideDeviation {
                control: float::Fc3SideDeviation::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fc3_slip_ball_position<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fc3SlipBallPosition {
                control: float::Fc3SlipBallPosition::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fc3_true_airspeed<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fc3TrueAirspeed {
                control: speed::Fc3TrueAirspeed::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fc3_vertical_velocity<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fc3VerticalVelocity {
                control: speed::Fc3VerticalVelocity::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fc3_vvi_bar<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fc3VviBar {
                control: bar::Fc3VviBar::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn build(self) -> Fc3<'a, MAX_CONTROLS, MAX_PACKET_DATA> {
        Fc3 {
            parser: ProtocolParser::new(),
            controls: self.controls,
            saw_frame: false,
        }
    }
}

impl<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> Default
    for Fc3Builder<'a, MAX_CONTROLS, MAX_PACKET_DATA>
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

pub struct Fc3<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> {
    parser: ProtocolParser<MAX_PACKET_DATA>,
    controls: Vec<RegisteredControl<'a>, MAX_CONTROLS>,
    saw_frame: bool,
}

impl<const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize>
    Fc3<'_, MAX_CONTROLS, MAX_PACKET_DATA>
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
