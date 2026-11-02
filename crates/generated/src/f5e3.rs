//! Generated from model-definitions/F-5E-3.json.
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

pub mod accelerometer {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AccValue {
        Select1,
        Select2,
    }
    pub type AccValueCallback<'a> = alloc::boxed::Box<dyn FnMut(AccValue) + 'a>;
    impl core::convert::TryFrom<u16> for AccValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Accelerometer - Push to set
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Acc(Selector<30350, 4, 2>);
    impl Acc {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Acc {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Acc {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod ahrs {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AhrsErectValue {
        Select1,
        Select2,
    }
    pub type AhrsErectValueCallback<'a> = alloc::boxed::Box<dyn FnMut(AhrsErectValue) + 'a>;
    impl core::convert::TryFrom<u16> for AhrsErectValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Fast Erect Button - Push to erect
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AhrsErect(Selector<30300, 1024, 10>);
    impl AhrsErect {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AhrsErect {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AhrsErect {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AhrsFastSlaveValue {
        Select1,
        Select2,
        Select3,
    }
    pub type AhrsFastSlaveValueCallback<'a> = alloc::boxed::Box<dyn FnMut(AhrsFastSlaveValue) + 'a>;
    impl core::convert::TryFrom<u16> for AhrsFastSlaveValue {
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

    /// Compass Switch, DIR GYRO/MAG/FAST SLAVE
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct AhrsFastSlave(Selector<30300, 6144, 11>);
    impl AhrsFastSlave {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AhrsFastSlave {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AhrsFastSlave {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AhrsNavModeValue {
        Select1,
        Select2,
    }
    pub type AhrsNavModeValueCallback<'a> = alloc::boxed::Box<dyn FnMut(AhrsNavModeValue) + 'a>;
    impl core::convert::TryFrom<u16> for AhrsNavModeValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Nav Mode Selector Switch, DF/TACAN
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AhrsNavMode(Selector<30300, 8192, 13>);
    impl AhrsNavMode {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AhrsNavMode {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AhrsNavMode {
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

    /// Index Setting Pointer Knob
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct IasSet(AnalogDial<30374, 65535, 0>);
    impl IasSet {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IasSet {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IasSet {
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AltElectValue {
        Select1,
        Select2,
    }
    pub type AltElectValueCallback<'a> = alloc::boxed::Box<dyn FnMut(AltElectValue) + 'a>;
    impl core::convert::TryFrom<u16> for AltElectValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Altimeter Mode Control Lever, ELECT(rical)/PNEU(matic)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AltElect(Selector<30350, 8, 3>);
    impl AltElect {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AltElect {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AltElect {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Altimeter Zero Setting Knob
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct AltZero(AnalogDial<30384, 65535, 0>);
    impl AltZero {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AltZero {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AltZero {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod analr87 {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// RWR INT Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct RwrInt(LimitedDial<30340, 65535, 0>);
    impl RwrInt {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrInt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrInt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod attitudeindicator {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Attitude Indicator Pitch Trim Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct AiPitchTrim(LimitedDial<30404, 65535, 0>);
    impl AiPitchTrim {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AiPitchTrim {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AiPitchTrim {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod camera {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CameraFpsValue {
        Select1,
        Select2,
    }
    pub type CameraFpsValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CameraFpsValue) + 'a>;
    impl core::convert::TryFrom<u16> for CameraFpsValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Sight Camera FPS Select Switch, 24/48
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CameraFps(Selector<30328, 4096, 12>);
    impl CameraFps {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CameraFps {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CameraFps {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Sight Camera Lens f-Stop Selector, 2.8(dull)..22(bright)
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct CameraLens(LimitedDial<30342, 65535, 0>);
    impl CameraLens {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CameraLens {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CameraLens {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CameraOverrunValue {
        Select1,
        Select2,
        Select3,
        Select4,
    }
    pub type CameraOverrunValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CameraOverrunValue) + 'a>;
    impl core::convert::TryFrom<u16> for CameraOverrunValue {
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

    /// Sight Camera Overrun Selector, 0/3/10/20
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct CameraOverrun(Selector<30328, 24576, 13>);
    impl CameraOverrun {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CameraOverrun {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CameraOverrun {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CameraRunValue {
        Select1,
        Select2,
    }
    pub type CameraRunValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CameraRunValue) + 'a>;
    impl core::convert::TryFrom<u16> for CameraRunValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Sight Camera Run (Test) Switch, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CameraRun(Selector<30328, 32768, 15>);
    impl CameraRun {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CameraRun {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CameraRun {
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

    /// ABU-11 CLOCK Setting Knob
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ClockSet(Selector<30350, 128, 7>);
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ClockWindValue {
        Select1,
        Select2,
    }
    pub type ClockWindValueCallback<'a> = alloc::boxed::Box<dyn FnMut(ClockWindValue) + 'a>;
    impl core::convert::TryFrom<u16> for ClockWindValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ABU-11 CLOCK Winding Knob
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ClockWind(Selector<30350, 64, 6>);
    impl ClockWind {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    /// ABU-11 CLOCK Winding Rotary
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct ClockWindRotatry(AnalogDial<30444, 65535, 0>);
    impl ClockWindRotatry {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ClockWindRotatry {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClockWindRotatry {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ElapTimeValue {
        Select1,
        Select2,
    }
    pub type ElapTimeValueCallback<'a> = alloc::boxed::Box<dyn FnMut(ElapTimeValue) + 'a>;
    impl core::convert::TryFrom<u16> for ElapTimeValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ABU-11 CLOCK Elapsed Time Knob
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ElapTime(Selector<30350, 256, 8>);
    impl ElapTime {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ElapTime {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ElapTime {
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
    pub enum ChaffCountValue {
        Select1,
        Select2,
    }
    pub type ChaffCountValueCallback<'a> = alloc::boxed::Box<dyn FnMut(ChaffCountValue) + 'a>;
    impl core::convert::TryFrom<u16> for ChaffCountValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Chaff Counter Reset Button - Push to reset
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ChaffCount(Selector<30288, 16, 4>);
    impl ChaffCount {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ChaffCount {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ChaffCount {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ChaffModeValue {
        Select1,
        Select2,
        Select3,
        Select4,
    }
    pub type ChaffModeValueCallback<'a> = alloc::boxed::Box<dyn FnMut(ChaffModeValue) + 'a>;
    impl core::convert::TryFrom<u16> for ChaffModeValue {
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

    /// Chaff Mode Selector, OFF/SINGLE/PRGM/MULT
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct ChaffMode(Selector<30266, 49152, 14>);
    impl ChaffMode {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ChaffMode {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ChaffMode {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Chaff Counter Display
    /// Output: Chaff Counter Display
    pub struct CmChaffcntDisplay(Metadata<30298, 63, 0>);
    impl CmChaffcntDisplay {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CmChaffcntDisplay {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CmChaffcntDisplay {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Flare Counter Display
    /// Output: Flare Counter Display
    pub struct CmFlarecntDisplay(Metadata<30288, 8064, 7>);
    impl CmFlarecntDisplay {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CmFlarecntDisplay {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CmFlarecntDisplay {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FlChaffBtValue {
        Select1,
        Select2,
    }
    pub type FlChaffBtValueCallback<'a> = alloc::boxed::Box<dyn FnMut(FlChaffBtValue) + 'a>;
    impl core::convert::TryFrom<u16> for FlChaffBtValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Flare-Chaff Button - Push to dispense
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FlChaffBt(Selector<30288, 64, 6>);
    impl FlChaffBt {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FlChaffBt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FlChaffBt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FlCountValue {
        Select1,
        Select2,
    }
    pub type FlCountValueCallback<'a> = alloc::boxed::Box<dyn FnMut(FlCountValue) + 'a>;
    impl core::convert::TryFrom<u16> for FlCountValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Flare Counter Reset Button - Push to reset
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FlCount(Selector<30288, 32, 5>);
    impl FlCount {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FlCount {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FlCount {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FlJettCoverValue {
        Select1,
        Select2,
    }
    pub type FlJettCoverValueCallback<'a> = alloc::boxed::Box<dyn FnMut(FlJettCoverValue) + 'a>;
    impl core::convert::TryFrom<u16> for FlJettCoverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Flare Jettison Switch Cover, OPEN/CLOSED
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FlJettCover(Selector<30288, 4, 2>);
    impl FlJettCover {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FlJettCover {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FlJettCover {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FlJettSwValue {
        Select1,
        Select2,
    }
    pub type FlJettSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(FlJettSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for FlJettSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Flare Jettison Switch, OFF/UP
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FlJettSw(Selector<30288, 8, 3>);
    impl FlJettSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FlJettSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FlJettSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FlareModeSelValue {
        Select1,
        Select2,
        Select3,
    }
    pub type FlareModeSelValueCallback<'a> = alloc::boxed::Box<dyn FnMut(FlareModeSelValue) + 'a>;
    impl core::convert::TryFrom<u16> for FlareModeSelValue {
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

    /// Flare Mode Selector, OFF/SINGLE/PRGM
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct FlareModeSel(Selector<30288, 3, 0>);
    impl FlareModeSel {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FlareModeSel {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FlareModeSel {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod cockpit {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CanopyJettisonValue {
        Select1,
        Select2,
    }
    pub type CanopyJettisonValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(CanopyJettisonValue) + 'a>;
    impl core::convert::TryFrom<u16> for CanopyJettisonValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Canopy Jettison T-Handle, PULL/PUSH
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CanopyJettison(Selector<30266, 32, 5>);
    impl CanopyJettison {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CanopyJettison {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CanopyJettison {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CanopyLeverValue {
        Select1,
        Select2,
    }
    pub type CanopyLeverValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CanopyLeverValue) + 'a>;
    impl core::convert::TryFrom<u16> for CanopyLeverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Canopy Handle, OPEN/CLOSE
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CanopyLever(Selector<30266, 4, 2>);
    impl CanopyLever {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CanopyLever {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CanopyLever {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DragChuteValue {
        Select1,
        Select2,
        Select3,
    }
    pub type DragChuteValueCallback<'a> = alloc::boxed::Box<dyn FnMut(DragChuteValue) + 'a>;
    impl core::convert::TryFrom<u16> for DragChuteValue {
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

    /// Chute Handle
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct DragChute(Selector<30266, 24, 3>);
    impl DragChute {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for DragChute {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for DragChute {
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
    pub enum AFlapsValue {
        Select1,
        Select2,
        Select3,
    }
    pub type AFlapsValueCallback<'a> = alloc::boxed::Box<dyn FnMut(AFlapsValue) + 'a>;
    impl core::convert::TryFrom<u16> for AFlapsValue {
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

    /// Auto Flap System Thumb Switch, UP/FIXED/AUTO
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct AFlaps(Selector<30214, 3, 0>);
    impl AFlaps {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AFlaps {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AFlaps {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FlapsValue {
        Select1,
        Select2,
        Select3,
    }
    pub type FlapsValueCallback<'a> = alloc::boxed::Box<dyn FnMut(FlapsValue) + 'a>;
    impl core::convert::TryFrom<u16> for FlapsValue {
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

    /// Flaps Lever, EMER UP/THUMB SW/FULL
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct Flaps(Selector<30210, 49152, 14>);
    impl Flaps {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Flaps {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Flaps {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum PitchDamperValue {
        Select1,
        Select2,
    }
    pub type PitchDamperValueCallback<'a> = alloc::boxed::Box<dyn FnMut(PitchDamperValue) + 'a>;
    impl core::convert::TryFrom<u16> for PitchDamperValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pitch Damper Switch, PITCH/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct PitchDamper(Selector<30210, 4096, 12>);
    impl PitchDamper {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for PitchDamper {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for PitchDamper {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum PitchDamperCutValue {
        Select1,
        Select2,
    }
    pub type PitchDamperCutValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(PitchDamperCutValue) + 'a>;
    impl core::convert::TryFrom<u16> for PitchDamperCutValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pitch Damper Cutoff Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct PitchDamperCut(Selector<30210, 8192, 13>);
    impl PitchDamperCut {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for PitchDamperCut {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for PitchDamperCut {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RudderTValue {
        Select1,
        Select2,
    }
    pub type RudderTValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RudderTValue) + 'a>;
    impl core::convert::TryFrom<u16> for RudderTValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Rudder Pedal Adjust T-Handle, PULL/STOW
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RudderT(Selector<30214, 16, 4>);
    impl RudderT {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RudderT {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RudderT {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Rudder Trim Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct RudderTrim(LimitedDial<30212, 65535, 0>);
    impl RudderTrim {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RudderTrim {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RudderTrim {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SpeedValue {
        Select1,
        Select2,
        Select3,
    }
    pub type SpeedValueCallback<'a> = alloc::boxed::Box<dyn FnMut(SpeedValue) + 'a>;
    impl core::convert::TryFrom<u16> for SpeedValue {
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

    /// Speed Brake Switch, OUT/OFF/IN
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct Speed(Selector<30214, 12, 2>);
    impl Speed {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Speed {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Speed {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum YawDamperValue {
        Select1,
        Select2,
    }
    pub type YawDamperValueCallback<'a> = alloc::boxed::Box<dyn FnMut(YawDamperValue) + 'a>;
    impl core::convert::TryFrom<u16> for YawDamperValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Yaw Damper Switch, YAW/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct YawDamper(Selector<30210, 2048, 11>);
    impl YawDamper {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for YawDamper {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for YawDamper {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod ecinterface {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Canopy Defog Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct CabinDefog(LimitedDial<30270, 65535, 0>);
    impl CabinDefog {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CabinDefog {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CabinDefog {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CabinPCValue {
        Select1,
        Select2,
    }
    pub type CabinPCValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CabinPCValue) + 'a>;
    impl core::convert::TryFrom<u16> for CabinPCValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Cabin Press Switch Cover, OPEN/CLOSED
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CabinPC(Selector<30258, 32768, 15>);
    impl CabinPC {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CabinPC {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CabinPC {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CabinPSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type CabinPSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CabinPSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for CabinPSwValue {
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

    /// Cabin Press Switch, DEFOG ONLY/NORMAL/RAM DUMP
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct CabinPSw(Selector<30258, 24576, 13>);
    impl CabinPSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CabinPSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CabinPSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Cabin Temperature Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct CabinTemp(LimitedDial<30268, 65535, 0>);
    impl CabinTemp {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CabinTemp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CabinTemp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CabinTempSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type CabinTempSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CabinTempSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for CabinTempSwValue {
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

    /// Cabin Temp Switch, AUTO/CENTER/MAN COLD/MAN HOT
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct CabinTempSw(Selector<30266, 3, 0>);
    impl CabinTempSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CabinTempSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CabinTempSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod electric {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FO2SwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type FO2SwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(FO2SwValue) + 'a>;
    impl core::convert::TryFrom<u16> for FO2SwValue {
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

    /// Fuel & Oxygen Switch, GAGE TEST/OFF/QTY CHECK
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct FO2Sw(Selector<30214, 1536, 9>);
    impl FO2Sw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FO2Sw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FO2Sw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LGeneratorValue {
        Select1,
        Select2,
    }
    pub type LGeneratorValueCallback<'a> = alloc::boxed::Box<dyn FnMut(LGeneratorValue) + 'a>;
    impl core::convert::TryFrom<u16> for LGeneratorValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Generator Left
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LGenerator(Selector<30214, 64, 6>);
    impl LGenerator {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LGenerator {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LGenerator {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum PitotHeaterValue {
        Select1,
        Select2,
    }
    pub type PitotHeaterValueCallback<'a> = alloc::boxed::Box<dyn FnMut(PitotHeaterValue) + 'a>;
    impl core::convert::TryFrom<u16> for PitotHeaterValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pitot Heater
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct PitotHeater(Selector<30214, 256, 8>);
    impl PitotHeater {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for PitotHeater {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for PitotHeater {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RGeneratorValue {
        Select1,
        Select2,
    }
    pub type RGeneratorValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RGeneratorValue) + 'a>;
    impl core::convert::TryFrom<u16> for RGeneratorValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Generator Right
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RGenerator(Selector<30214, 128, 7>);
    impl RGenerator {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RGenerator {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RGenerator {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SwBatteryValue {
        Select1,
        Select2,
    }
    pub type SwBatteryValueCallback<'a> = alloc::boxed::Box<dyn FnMut(SwBatteryValue) + 'a>;
    impl core::convert::TryFrom<u16> for SwBatteryValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Battery Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct SwBattery(Selector<30214, 32, 5>);
    impl SwBattery {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SwBattery {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SwBattery {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod engineinterface {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum InletHeaterValue {
        Select1,
        Select2,
    }
    pub type InletHeaterValueCallback<'a> = alloc::boxed::Box<dyn FnMut(InletHeaterValue) + 'a>;
    impl core::convert::TryFrom<u16> for InletHeaterValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Engine Anti-Ice Switch, ENGINE/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct InletHeater(Selector<30222, 256, 8>);
    impl InletHeater {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for InletHeater {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for InletHeater {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LStartValue {
        Select1,
        Select2,
    }
    pub type LStartValueCallback<'a> = alloc::boxed::Box<dyn FnMut(LStartValue) + 'a>;
    impl core::convert::TryFrom<u16> for LStartValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Left Engine Start Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LStart(Selector<30222, 64, 6>);
    impl LStart {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LStart {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LStart {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RStartValue {
        Select1,
        Select2,
    }
    pub type RStartValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RStartValue) + 'a>;
    impl core::convert::TryFrom<u16> for RStartValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Right Engine Start Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RStart(Selector<30222, 128, 7>);
    impl RStart {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RStart {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RStart {
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

    /// Brake Cute
    /// Output: Brake Cute
    pub struct ExtBrakeCute(Metadata<30474, 512, 9>);
    impl ExtBrakeCute {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ExtBrakeCute {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ExtBrakeCute {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Position Lights
    /// Output: Position Lights
    pub struct ExtPositionLights(Metadata<30474, 128, 7>);
    impl ExtPositionLights {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ExtPositionLights {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ExtPositionLights {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Left Speed Brake
    /// Output: Left Speed Brake
    pub struct ExtSpeedBrakeLeft(Metadata<30484, 65535, 0>);
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
    pub struct ExtSpeedBrakeRight(Metadata<30482, 65535, 0>);
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

    /// Strobe Lights
    /// Output: Strobe Lights
    pub struct ExtStrobeLights(Metadata<30474, 256, 8>);
    impl ExtStrobeLights {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ExtStrobeLights {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ExtStrobeLights {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Weight ON Wheels Left Gear
    /// Output: Weight ON Wheels Left Gear
    pub struct ExtWowLeft(Metadata<30474, 4096, 12>);
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
    pub struct ExtWowNose(Metadata<30474, 1024, 10>);
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
    pub struct ExtWowRight(Metadata<30474, 2048, 11>);
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum BeaconValue {
        Select1,
        Select2,
    }
    pub type BeaconValueCallback<'a> = alloc::boxed::Box<dyn FnMut(BeaconValue) + 'a>;
    impl core::convert::TryFrom<u16> for BeaconValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Beacon Light Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Beacon(Selector<30266, 64, 6>);
    impl Beacon {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Beacon {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Beacon {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Formation Lights Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct FormationLights(LimitedDial<30276, 65535, 0>);
    impl FormationLights {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FormationLights {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FormationLights {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LgLightValue {
        Select1,
        Select2,
    }
    pub type LgLightValueCallback<'a> = alloc::boxed::Box<dyn FnMut(LgLightValue) + 'a>;
    impl core::convert::TryFrom<u16> for LgLightValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Landing & Taxi Light Switch, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LgLight(Selector<30266, 128, 7>);
    impl LgLight {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LgLight {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LgLight {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Nav Lights Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct NavLights(LimitedDial<30274, 65535, 0>);
    impl NavLights {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for NavLights {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for NavLights {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod fuel {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AutobalValue {
        Select1,
        Select2,
        Select3,
    }
    pub type AutobalValueCallback<'a> = alloc::boxed::Box<dyn FnMut(AutobalValue) + 'a>;
    impl core::convert::TryFrom<u16> for AutobalValue {
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

    /// Autobalance Switch, LEFT/NEUT/RIGHT
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct Autobal(Selector<30222, 3, 0>);
    impl Autobal {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Autobal {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Autobal {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CrossfeedValue {
        Select1,
        Select2,
    }
    pub type CrossfeedValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CrossfeedValue) + 'a>;
    impl core::convert::TryFrom<u16> for CrossfeedValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Crossfeed Switch, OPEN/CLOSED
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Crossfeed(Selector<30214, 32768, 15>);
    impl Crossfeed {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Crossfeed {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Crossfeed {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ExtCenterSwValue {
        Select1,
        Select2,
    }
    pub type ExtCenterSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(ExtCenterSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for ExtCenterSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Ext Fuel Cl Switch, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ExtCenterSw(Selector<30214, 16384, 14>);
    impl ExtCenterSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ExtCenterSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ExtCenterSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ExtPylonSwValue {
        Select1,
        Select2,
    }
    pub type ExtPylonSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(ExtPylonSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for ExtPylonSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Ext Fuel Pylons Switch, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ExtPylonSw(Selector<30214, 8192, 13>);
    impl ExtPylonSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ExtPylonSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ExtPylonSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LBoostpumpValue {
        Select1,
        Select2,
    }
    pub type LBoostpumpValueCallback<'a> = alloc::boxed::Box<dyn FnMut(LBoostpumpValue) + 'a>;
    impl core::convert::TryFrom<u16> for LBoostpumpValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Left Boost Pump Switch, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LBoostpump(Selector<30222, 4, 2>);
    impl LBoostpump {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LBoostpump {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LBoostpump {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LShutoffCValue {
        Select1,
        Select2,
    }
    pub type LShutoffCValueCallback<'a> = alloc::boxed::Box<dyn FnMut(LShutoffCValue) + 'a>;
    impl core::convert::TryFrom<u16> for LShutoffCValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Left Fuel Shutoff Switch Cover, OPEN/CLOSED
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LShutoffC(Selector<30222, 16, 4>);
    impl LShutoffC {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LShutoffC {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LShutoffC {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LShutoffSValue {
        Select1,
        Select2,
    }
    pub type LShutoffSValueCallback<'a> = alloc::boxed::Box<dyn FnMut(LShutoffSValue) + 'a>;
    impl core::convert::TryFrom<u16> for LShutoffSValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Left Fuel Shutoff Switch, OPEN/CLOSED
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LShutoffS(Selector<30214, 2048, 11>);
    impl LShutoffS {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LShutoffS {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LShutoffS {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RBoostpumpValue {
        Select1,
        Select2,
    }
    pub type RBoostpumpValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RBoostpumpValue) + 'a>;
    impl core::convert::TryFrom<u16> for RBoostpumpValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Right Boost Pump Switch, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RBoostpump(Selector<30222, 8, 3>);
    impl RBoostpump {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RBoostpump {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RBoostpump {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RShutoffCValue {
        Select1,
        Select2,
    }
    pub type RShutoffCValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RShutoffCValue) + 'a>;
    impl core::convert::TryFrom<u16> for RShutoffCValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Right Fuel Shutoff Switch Cover, OPEN/CLOSED
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RShutoffC(Selector<30222, 32, 5>);
    impl RShutoffC {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RShutoffC {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RShutoffC {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RShutoffSValue {
        Select1,
        Select2,
    }
    pub type RShutoffSValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RShutoffSValue) + 'a>;
    impl core::convert::TryFrom<u16> for RShutoffSValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Right Fuel Shutoff Switch, OPEN/CLOSED
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RShutoffS(Selector<30214, 4096, 12>);
    impl RShutoffS {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RShutoffS {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RShutoffS {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod gauges {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Accelerometer
    /// Output: gauge position
    pub struct Accelerometer(AnalogGauge<30368, 65535, 0>);
    impl Accelerometer {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Accelerometer {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Accelerometer {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Accelerometer Max
    /// Output: gauge position
    pub struct AccelerometerMax(AnalogGauge<30372, 65535, 0>);
    impl AccelerometerMax {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AccelerometerMax {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AccelerometerMax {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Accelerometer Min
    /// Output: gauge position
    pub struct AccelerometerMin(AnalogGauge<30370, 65535, 0>);
    impl AccelerometerMin {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AccelerometerMin {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AccelerometerMin {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Attitude Indicator Bank
    /// Output: gauge position
    pub struct AiBank(AnalogGauge<30408, 65535, 0>);
    impl AiBank {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AiBank {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AiBank {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Attitude Indicator OFF Flag
    /// Output: gauge position
    pub struct AiOffFlag(AnalogGauge<30410, 65535, 0>);
    impl AiOffFlag {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AiOffFlag {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AiOffFlag {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Attitude Indicator Pitch
    /// Output: gauge position
    pub struct AiPitch(AnalogGauge<30406, 65535, 0>);
    impl AiPitch {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AiPitch {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AiPitch {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Airspeed
    /// Output: gauge position
    pub struct Airspeed(AnalogGauge<30376, 65535, 0>);
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

    /// Max Airspeed
    /// Output: gauge position
    pub struct AirspeedMax(AnalogGauge<30378, 65535, 0>);
    impl AirspeedMax {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AirspeedMax {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AirspeedMax {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Set Airspeed
    /// Output: gauge position
    pub struct AirspeedSet(AnalogGauge<30380, 65535, 0>);
    impl AirspeedSet {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AirspeedSet {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AirspeedSet {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Altimeter 10000ft Count
    /// Output: gauge position
    pub struct Alt10000C(AnalogGauge<30388, 65535, 0>);
    impl Alt10000C {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Alt10000C {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Alt10000C {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Altimeter 1000ft Count
    /// Output: gauge position
    pub struct Alt1000C(AnalogGauge<30390, 65535, 0>);
    impl Alt1000C {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Alt1000C {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Alt1000C {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Altimeter 100ft Count
    /// Output: gauge position
    pub struct Alt100C(AnalogGauge<30392, 65535, 0>);
    impl Alt100C {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Alt100C {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Alt100C {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Altimeter 100ft Pointer
    /// Output: gauge position
    pub struct Alt100P(AnalogGauge<30386, 65535, 0>);
    impl Alt100P {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Alt100P {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Alt100P {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Altimeter PNEU Flag
    /// Output: gauge position
    pub struct AltPneuFlag(AnalogGauge<30402, 65535, 0>);
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

    /// AOA Units
    /// Output: gauge position
    pub struct AoaGauge(AnalogGauge<30352, 65535, 0>);
    impl AoaGauge {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AoaGauge {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AoaGauge {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Aux Intake Doors
    /// Output: gauge position
    pub struct AuxintDoor(AnalogGauge<30236, 65535, 0>);
    impl AuxintDoor {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AuxintDoor {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AuxintDoor {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Cabin Pressure
    /// Output: gauge position
    pub struct CabinPress(AnalogGauge<30272, 65535, 0>);
    impl CabinPress {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CabinPress {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CabinPress {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Canopy Position
    /// Output: gauge position
    pub struct CanopyPos(AnalogGauge<30366, 65535, 0>);
    impl CanopyPos {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CanopyPos {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CanopyPos {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Chaff Drum Counter 1
    /// Output: gauge position
    pub struct ChaffCount1(AnalogGauge<30292, 65535, 0>);
    impl ChaffCount1 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ChaffCount1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ChaffCount1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Chaff Drum Counter 10
    /// Output: gauge position
    pub struct ChaffCount10(AnalogGauge<30290, 65535, 0>);
    impl ChaffCount10 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ChaffCount10 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ChaffCount10 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// CLOCK Currtime Hours
    /// Output: gauge position
    pub struct ClockCurrH(AnalogGauge<30446, 65535, 0>);
    impl ClockCurrH {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ClockCurrH {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClockCurrH {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// CLOCK Currtime Minutes
    /// Output: gauge position
    pub struct ClockCurrM(AnalogGauge<30448, 65535, 0>);
    impl ClockCurrM {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ClockCurrM {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClockCurrM {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// CLOCK Elapsed Time Minutes
    /// Output: gauge position
    pub struct ClockElapM(AnalogGauge<30450, 65535, 0>);
    impl ClockElapM {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ClockElapM {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClockElapM {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// CLOCK Elapsed Time Seconds
    /// Output: gauge position
    pub struct ClockElapS(AnalogGauge<30452, 65535, 0>);
    impl ClockElapS {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ClockElapS {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClockElapS {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Exhaust Gas Temp Left
    /// Output: gauge position
    pub struct ExhaustTempL(AnalogGauge<30242, 65535, 0>);
    impl ExhaustTempL {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ExhaustTempL {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ExhaustTempL {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Exhaust Gas Temp Right
    /// Output: gauge position
    pub struct ExhaustTempR(AnalogGauge<30244, 65535, 0>);
    impl ExhaustTempR {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ExhaustTempR {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ExhaustTempR {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Flare Drum Counter 1
    /// Output: gauge position
    pub struct FlCount1(AnalogGauge<30296, 65535, 0>);
    impl FlCount1 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FlCount1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FlCount1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Flare Drum Counter 10
    /// Output: gauge position
    pub struct FlCount10(AnalogGauge<30294, 65535, 0>);
    impl FlCount10 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FlCount10 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FlCount10 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Flap Indicator
    /// Output: gauge position
    pub struct FlapIndicator(AnalogGauge<30218, 65535, 0>);
    impl FlapIndicator {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FlapIndicator {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FlapIndicator {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Flight Pressure
    /// Output: gauge position
    pub struct FlightPress(AnalogGauge<30256, 65535, 0>);
    impl FlightPress {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FlightPress {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FlightPress {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Fuel Flow Left
    /// Output: gauge position
    pub struct FuelflowL(AnalogGauge<30230, 65535, 0>);
    impl FuelflowL {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FuelflowL {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FuelflowL {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Fuel Flow Right
    /// Output: gauge position
    pub struct FuelflowR(AnalogGauge<30228, 65535, 0>);
    impl FuelflowR {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FuelflowR {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FuelflowR {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Fuel Quantity Left
    /// Output: gauge position
    pub struct FuelquantityL(AnalogGauge<30224, 65535, 0>);
    impl FuelquantityL {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FuelquantityL {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FuelquantityL {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Fuel Quantity Right
    /// Output: gauge position
    pub struct FuelquantityR(AnalogGauge<30226, 65535, 0>);
    impl FuelquantityR {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FuelquantityR {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FuelquantityR {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// HSI Bearing Pointer
    /// Output: gauge position
    pub struct HsiBerPoint(AnalogGauge<30422, 65535, 0>);
    impl HsiBerPoint {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HsiBerPoint {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HsiBerPoint {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// HSI Compass Card
    /// Output: gauge position
    pub struct HsiComp(AnalogGauge<30420, 65535, 0>);
    impl HsiComp {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HsiComp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HsiComp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// HSI Course Arrow
    /// Output: gauge position
    pub struct HsiCrs(AnalogGauge<30416, 65535, 0>);
    impl HsiCrs {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HsiCrs {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HsiCrs {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// HSI CourseSel 1
    /// Output: gauge position
    pub struct HsiCrs1(AnalogGauge<30434, 65535, 0>);
    impl HsiCrs1 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HsiCrs1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HsiCrs1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// HSI CourseSel 100 10
    /// Output: gauge position
    pub struct HsiCrs10010(AnalogGauge<30432, 65535, 0>);
    impl HsiCrs10010 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HsiCrs10010 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HsiCrs10010 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// HSI Course Dev Indicator
    /// Output: gauge position
    pub struct HsiCrsDev(AnalogGauge<30424, 65535, 0>);
    impl HsiCrsDev {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HsiCrsDev {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HsiCrsDev {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// HSI DevDF Window
    /// Output: gauge position
    pub struct HsiDev(AnalogGauge<30440, 65535, 0>);
    impl HsiDev {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HsiDev {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HsiDev {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// HSI Heading Mark
    /// Output: gauge position
    pub struct HsiHdg(AnalogGauge<30418, 65535, 0>);
    impl HsiHdg {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HsiHdg {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HsiHdg {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// HSI OFF Flag
    /// Output: gauge position
    pub struct HsiOff(AnalogGauge<30438, 65535, 0>);
    impl HsiOff {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HsiOff {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HsiOff {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// HSI Range 1
    /// Output: gauge position
    pub struct HsiRange1(AnalogGauge<30430, 65535, 0>);
    impl HsiRange1 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HsiRange1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HsiRange1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// HSI Range 10
    /// Output: gauge position
    pub struct HsiRange10(AnalogGauge<30428, 65535, 0>);
    impl HsiRange10 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HsiRange10 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HsiRange10 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// HSI Range 100
    /// Output: gauge position
    pub struct HsiRange100(AnalogGauge<30426, 65535, 0>);
    impl HsiRange100 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HsiRange100 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HsiRange100 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// HSI To From
    /// Output: gauge position
    pub struct HsiToFrom(AnalogGauge<30436, 65535, 0>);
    impl HsiToFrom {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HsiToFrom {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HsiToFrom {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// MachIndicator
    /// Output: gauge position
    pub struct Mach(AnalogGauge<30382, 65535, 0>);
    impl Mach {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Mach {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Mach {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Motor Run Knob
    /// Output: gauge position
    pub struct MotorRun(AnalogGauge<30354, 65535, 0>);
    impl MotorRun {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MotorRun {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MotorRun {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Nozzle Position Left
    /// Output: gauge position
    pub struct NozzleposL(AnalogGauge<30232, 65535, 0>);
    impl NozzleposL {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for NozzleposL {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for NozzleposL {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Nozzle Position Right
    /// Output: gauge position
    pub struct NozzleposR(AnalogGauge<30234, 65535, 0>);
    impl NozzleposR {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for NozzleposR {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for NozzleposR {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// O2 Flow Blinker
    /// Output: gauge position
    pub struct O2flowindicator(AnalogGauge<30262, 65535, 0>);
    impl O2flowindicator {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for O2flowindicator {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for O2flowindicator {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// O2 FlowPressure
    /// Output: gauge position
    pub struct O2flowpressure(AnalogGauge<30264, 65535, 0>);
    impl O2flowpressure {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for O2flowpressure {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for O2flowpressure {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// O2 Quantity
    /// Output: gauge position
    pub struct O2quantity(AnalogGauge<30260, 65535, 0>);
    impl O2quantity {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for O2quantity {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for O2quantity {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Oil Pressure Left
    /// Output: gauge position
    pub struct OilpressL(AnalogGauge<30238, 65535, 0>);
    impl OilpressL {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for OilpressL {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for OilpressL {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Oil Pressure Right
    /// Output: gauge position
    pub struct OilpressR(AnalogGauge<30240, 65535, 0>);
    impl OilpressR {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for OilpressR {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for OilpressR {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Pressure Setting 0
    /// Output: gauge position
    pub struct Press0(AnalogGauge<30394, 65535, 0>);
    impl Press0 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Press0 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Press0 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Pressure Setting 1
    /// Output: gauge position
    pub struct Press1(AnalogGauge<30396, 65535, 0>);
    impl Press1 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Press1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Press1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Pressure Setting 2
    /// Output: gauge position
    pub struct Press2(AnalogGauge<30398, 65535, 0>);
    impl Press2 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Press2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Press2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Pressure Setting 3
    /// Output: gauge position
    pub struct Press3(AnalogGauge<30400, 65535, 0>);
    impl Press3 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Press3 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Press3 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// SAI Knob Arrow
    /// Output: gauge position
    pub struct SaiArrow(AnalogGauge<30364, 65535, 0>);
    impl SaiArrow {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SaiArrow {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SaiArrow {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// SAI Bank
    /// Output: gauge position
    pub struct SaiBank(AnalogGauge<30360, 65535, 0>);
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

    /// SAI OFF Flag
    /// Output: gauge position
    pub struct SaiOff(AnalogGauge<30362, 65535, 0>);
    impl SaiOff {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SaiOff {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SaiOff {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// SAI Pitch
    /// Output: gauge position
    pub struct SaiPitch(AnalogGauge<30358, 65535, 0>);
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

    /// Standby Compass Bank
    /// Output: gauge position
    pub struct SbyCompassBank(AnalogGauge<30490, 65535, 0>);
    impl SbyCompassBank {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SbyCompassBank {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SbyCompassBank {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Standby Compass Heading
    /// Output: gauge position
    pub struct SbyCompassHdg(AnalogGauge<30486, 65535, 0>);
    impl SbyCompassHdg {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SbyCompassHdg {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SbyCompassHdg {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Standby Compass Pitch
    /// Output: gauge position
    pub struct SbyCompassPitch(AnalogGauge<30488, 65535, 0>);
    impl SbyCompassPitch {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SbyCompassPitch {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SbyCompassPitch {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Ret Depression Drum 1
    /// Output: gauge position
    pub struct SightDep1(AnalogGauge<30334, 65535, 0>);
    impl SightDep1 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SightDep1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SightDep1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Ret Depression Drum 10
    /// Output: gauge position
    pub struct SightDep10(AnalogGauge<30332, 65535, 0>);
    impl SightDep10 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SightDep10 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SightDep10 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Ret Depression Drum 100
    /// Output: gauge position
    pub struct SightDep100(AnalogGauge<30330, 65535, 0>);
    impl SightDep100 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SightDep100 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SightDep100 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Slipball
    /// Output: gauge position
    pub struct Slipball(AnalogGauge<30220, 65535, 0>);
    impl Slipball {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Slipball {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Slipball {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Tachometer Left
    /// Output: gauge position
    pub struct TachoL(AnalogGauge<30246, 65535, 0>);
    impl TachoL {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TachoL {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TachoL {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Tachometer Percent Left
    /// Output: gauge position
    pub struct TachoPercL(AnalogGauge<30248, 65535, 0>);
    impl TachoPercL {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TachoPercL {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TachoPercL {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Tachometer Percent Right
    /// Output: gauge position
    pub struct TachoPercR(AnalogGauge<30252, 65535, 0>);
    impl TachoPercR {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TachoPercR {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TachoPercR {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Tachometer Right
    /// Output: gauge position
    pub struct TachoR(AnalogGauge<30250, 65535, 0>);
    impl TachoR {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TachoR {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TachoR {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// TdcControlAzimuth
    /// Output: gauge position
    pub struct TdcAzimuth(AnalogGauge<30322, 65535, 0>);
    impl TdcAzimuth {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TdcAzimuth {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TdcAzimuth {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// TdcControlRange
    /// Output: gauge position
    pub struct TdcRange(AnalogGauge<30320, 65535, 0>);
    impl TdcRange {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TdcRange {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TdcRange {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Trim Position
    /// Output: gauge position
    pub struct TrimIndicator(AnalogGauge<30216, 65535, 0>);
    impl TrimIndicator {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TrimIndicator {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TrimIndicator {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// UHF Radio 0.025MHz
    /// Output: gauge position
    pub struct Uhf0025(AnalogGauge<30472, 65535, 0>);
    impl Uhf0025 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Uhf0025 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Uhf0025 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// UHF Radio 0.1MHz
    /// Output: gauge position
    pub struct Uhf01(AnalogGauge<30470, 65535, 0>);
    impl Uhf01 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Uhf01 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Uhf01 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// UHF Radio 1MHz
    /// Output: gauge position
    pub struct Uhf1(AnalogGauge<30468, 65535, 0>);
    impl Uhf1 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Uhf1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Uhf1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// UHF Radio 10MHz
    /// Output: gauge position
    pub struct Uhf10(AnalogGauge<30466, 65535, 0>);
    impl Uhf10 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Uhf10 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Uhf10 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// UHF Radio 100MHz
    /// Output: gauge position
    pub struct Uhf100(AnalogGauge<30464, 65535, 0>);
    impl Uhf100 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Uhf100 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Uhf100 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// UHF Preset Radio Channel
    /// Output: gauge position
    pub struct UhfPresetChan(AnalogGauge<30462, 65535, 0>);
    impl UhfPresetChan {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfPresetChan {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfPresetChan {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Utility Pressure
    /// Output: gauge position
    pub struct UtilPress(AnalogGauge<30254, 65535, 0>);
    impl UtilPress {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UtilPress {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UtilPress {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Variometer
    /// Output: gauge position
    pub struct Variometer(AnalogGauge<30356, 65535, 0>);
    impl Variometer {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Variometer {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Variometer {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod gearinterface {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Gear Warning Lamp (red)
    /// Output: 0 if light is off, 1 if light is on
    pub struct GwLight(Led<30222, 512, 9>);
    impl GwLight {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for GwLight {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for GwLight {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum HookValue {
        Select1,
        Select2,
    }
    pub type HookValueCallback<'a> = alloc::boxed::Box<dyn FnMut(HookValue) + 'a>;
    impl core::convert::TryFrom<u16> for HookValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Arresting Hook Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Hook(Selector<30258, 256, 8>);
    impl Hook {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Hook {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Hook {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Hook Warning Lamp (red)
    /// Output: 0 if light is off, 1 if light is on
    pub struct HookLight(Led<30222, 8192, 13>);
    impl HookLight {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for HookLight {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HookLight {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Gear Left Lamp (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LeftLight(Led<30222, 2048, 11>);
    impl LeftLight {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LeftLight {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LeftLight {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LgAltRelValue {
        Select1,
        Select2,
    }
    pub type LgAltRelValueCallback<'a> = alloc::boxed::Box<dyn FnMut(LgAltRelValue) + 'a>;
    impl core::convert::TryFrom<u16> for LgAltRelValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Landing Gear Alternate Release Handle, Pull and Hold
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LgAltRel(Selector<30222, 32768, 15>);
    impl LgAltRel {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LgAltRel {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LgAltRel {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LgAltResValue {
        Select1,
        Select2,
    }
    pub type LgAltResValueCallback<'a> = alloc::boxed::Box<dyn FnMut(LgAltResValue) + 'a>;
    impl core::convert::TryFrom<u16> for LgAltResValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Landing Gear Alternate Release Reset Control, OFF/RESET
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LgAltRes(Selector<30258, 4, 2>);
    impl LgAltRes {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LgAltRes {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LgAltRes {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LgDownLValue {
        Select1,
        Select2,
    }
    pub type LgDownLValueCallback<'a> = alloc::boxed::Box<dyn FnMut(LgDownLValue) + 'a>;
    impl core::convert::TryFrom<u16> for LgDownLValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Landing Gear Downlock Override Button - Push and Hold
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LgDownL(Selector<30258, 1, 0>);
    impl LgDownL {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LgDownL {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LgDownL {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LgLeverSwitchValue {
        Select1,
        Select2,
    }
    pub type LgLeverSwitchValueCallback<'a> = alloc::boxed::Box<dyn FnMut(LgLeverSwitchValue) + 'a>;
    impl core::convert::TryFrom<u16> for LgLeverSwitchValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Gear Lever
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LgLeverSwitch(Selector<30222, 16384, 14>);
    impl LgLeverSwitch {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LgLeverSwitch {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LgLeverSwitch {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LgTestValue {
        Select1,
        Select2,
    }
    pub type LgTestValueCallback<'a> = alloc::boxed::Box<dyn FnMut(LgTestValue) + 'a>;
    impl core::convert::TryFrom<u16> for LgTestValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Left Landing Gear Test Lamp
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LgTest(Selector<30258, 32, 5>);
    impl LgTest {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LgTest {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LgTest {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LgfSilenceValue {
        Select1,
        Select2,
    }
    pub type LgfSilenceValueCallback<'a> = alloc::boxed::Box<dyn FnMut(LgfSilenceValue) + 'a>;
    impl core::convert::TryFrom<u16> for LgfSilenceValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Landing Gear And Flap Warning Silence Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LgfSilence(Selector<30258, 16, 4>);
    impl LgfSilence {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LgfSilence {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LgfSilence {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NgTestValue {
        Select1,
        Select2,
    }
    pub type NgTestValueCallback<'a> = alloc::boxed::Box<dyn FnMut(NgTestValue) + 'a>;
    impl core::convert::TryFrom<u16> for NgTestValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Nose Landing Gear Test Lamp
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct NgTest(Selector<30258, 64, 6>);
    impl NgTest {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for NgTest {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for NgTest {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Gear Nose Lamp (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct NoseLight(Led<30222, 1024, 10>);
    impl NoseLight {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for NoseLight {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for NoseLight {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NsStructValue {
        Select1,
        Select2,
    }
    pub type NsStructValueCallback<'a> = alloc::boxed::Box<dyn FnMut(NsStructValue) + 'a>;
    impl core::convert::TryFrom<u16> for NsStructValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Nose Strut Switch, EXTEND/RETRACT
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct NsStruct(Selector<30258, 8, 3>);
    impl NsStruct {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for NsStruct {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for NsStruct {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NwsValue {
        Select1,
        Select2,
    }
    pub type NwsValueCallback<'a> = alloc::boxed::Box<dyn FnMut(NwsValue) + 'a>;
    impl core::convert::TryFrom<u16> for NwsValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Nosewheel Steering Button - Pull and Hold
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Nws(Selector<30258, 2, 1>);
    impl Nws {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Nws {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Nws {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RgTestValue {
        Select1,
        Select2,
    }
    pub type RgTestValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RgTestValue) + 'a>;
    impl core::convert::TryFrom<u16> for RgTestValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Right Landing Gear Test Lamp
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RgTest(Selector<30258, 128, 7>);
    impl RgTest {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RgTest {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RgTest {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Gear Right Lamp (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct RightLight(Led<30222, 4096, 12>);
    impl RightLight {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RightLight {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RightLight {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod hsi {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// HSI Course Set Knob
    /// Input: rotate the knob
    /// Output: rotation of the knob (not the value being manipulated!)
    pub struct HsiCrsKnob(AnalogDial<30414, 65535, 0>);
    impl HsiCrsKnob {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HsiCrsKnob {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HsiCrsKnob {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// HSI Heading Set Knob
    /// Input: rotate the knob
    /// Output: rotation of the knob (not the value being manipulated!)
    pub struct HsiHdgKnob(AnalogDial<30412, 65535, 0>);
    impl HsiHdgKnob {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HsiHdgKnob {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HsiHdgKnob {
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
    pub enum Iff4CodeSelValue {
        Select1,
        Select2,
        Select3,
        Select4,
    }
    pub type Iff4CodeSelValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Iff4CodeSelValue) + 'a>;
    impl core::convert::TryFrom<u16> for Iff4CodeSelValue {
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

    /// IFF MODE 4 CODE Selector, ZERO/B/A/HOLD
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct Iff4CodeSel(Selector<30344, 12, 2>);
    impl Iff4CodeSel {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Iff4CodeSel {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Iff4CodeSel {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Iff4CodeSelPullValue {
        Select1,
        Select2,
    }
    pub type Iff4CodeSelPullValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(Iff4CodeSelPullValue) + 'a>;
    impl core::convert::TryFrom<u16> for Iff4CodeSelPullValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// IFF MODE 4 CODE Selector Pull
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Iff4CodeSelPull(Selector<30350, 1, 0>);
    impl Iff4CodeSelPull {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Iff4CodeSelPull {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Iff4CodeSelPull {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Iff4ControlValue {
        Select1,
        Select2,
    }
    pub type Iff4ControlValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Iff4ControlValue) + 'a>;
    impl core::convert::TryFrom<u16> for Iff4ControlValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// IFF MODE 4 Control Switch, ON/OUT
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Iff4Control(Selector<30344, 32768, 15>);
    impl Iff4Control {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Iff4Control {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Iff4Control {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Iff4IdentValue {
        Select1,
        Select2,
        Select3,
    }
    pub type Iff4IdentValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Iff4IdentValue) + 'a>;
    impl core::convert::TryFrom<u16> for Iff4IdentValue {
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

    /// IFF Identification of Position (IP) Switch, IDENT/OUT/MIC
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct Iff4Ident(Selector<30346, 48, 4>);
    impl Iff4Ident {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Iff4Ident {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Iff4Ident {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Iff4M1Value {
        Select1,
        Select2,
        Select3,
    }
    pub type Iff4M1ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Iff4M1Value) + 'a>;
    impl core::convert::TryFrom<u16> for Iff4M1Value {
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

    /// IFF Mode Select/TEST Switch, M-1 /ON/OUT
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct Iff4M1(Selector<30344, 1536, 9>);
    impl Iff4M1 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Iff4M1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Iff4M1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Iff4M2Value {
        Select1,
        Select2,
        Select3,
    }
    pub type Iff4M2ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Iff4M2Value) + 'a>;
    impl core::convert::TryFrom<u16> for Iff4M2Value {
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

    /// IFF Mode Select/TEST Switch, M-2 /ON/OUT
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct Iff4M2(Selector<30344, 6144, 11>);
    impl Iff4M2 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Iff4M2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Iff4M2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Iff4M3Value {
        Select1,
        Select2,
        Select3,
    }
    pub type Iff4M3ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Iff4M3Value) + 'a>;
    impl core::convert::TryFrom<u16> for Iff4M3Value {
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

    /// IFF Mode Select/TEST Switch, M-3/A /ON/OUT
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct Iff4M3(Selector<30344, 24576, 13>);
    impl Iff4M3 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Iff4M3 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Iff4M3 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Iff4MasterValue {
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
        Select11,
    }
    pub type Iff4MasterValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Iff4MasterValue) + 'a>;
    impl core::convert::TryFrom<u16> for Iff4MasterValue {
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
                10 => Ok(Self::Select11),
                value => Err(value),
            }
        }
    }

    /// IFF MODE 3/A Code Selector Wheel 4
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct Iff4Master(Selector<30348, 61440, 12>);
    impl Iff4Master {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Iff4Master {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Iff4Master {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Iff4MasterPullValue {
        Select1,
        Select2,
    }
    pub type Iff4MasterPullValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(Iff4MasterPullValue) + 'a>;
    impl core::convert::TryFrom<u16> for Iff4MasterPullValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// IFF MASTER Control Selector Pull
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Iff4MasterPull(Selector<30350, 2, 1>);
    impl Iff4MasterPull {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Iff4MasterPull {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Iff4MasterPull {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Iff4McValue {
        Select1,
        Select2,
        Select3,
    }
    pub type Iff4McValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Iff4McValue) + 'a>;
    impl core::convert::TryFrom<u16> for Iff4McValue {
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

    /// IFF Mode Select/TEST Switch, M-C /ON/OUT
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct Iff4Mc(Selector<30346, 3, 0>);
    impl Iff4Mc {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Iff4Mc {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Iff4Mc {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Iff4MonContrValue {
        Select1,
        Select2,
        Select3,
    }
    pub type Iff4MonContrValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Iff4MonContrValue) + 'a>;
    impl core::convert::TryFrom<u16> for Iff4MonContrValue {
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

    /// IFF MODE 4 Monitor Control Switch, AUDIO/OUT/LIGHT
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct Iff4MonContr(Selector<30344, 384, 7>);
    impl Iff4MonContr {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Iff4MonContr {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Iff4MonContr {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Iff4RadValue {
        Select1,
        Select2,
        Select3,
    }
    pub type Iff4RadValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Iff4RadValue) + 'a>;
    impl core::convert::TryFrom<u16> for Iff4RadValue {
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

    /// IFF RAD TEST/MON Switch, RAD TEST/OUT/MON
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct Iff4Rad(Selector<30346, 12, 2>);
    impl Iff4Rad {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Iff4Rad {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Iff4Rad {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Iff4RadTestLValue {
        Select1,
        Select2,
    }
    pub type Iff4RadTestLValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Iff4RadTestLValue) + 'a>;
    impl core::convert::TryFrom<u16> for Iff4RadTestLValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Radiation TEST and Monitor Light - Press to test(LMB)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Iff4RadTestL(Selector<30346, 32768, 15>);
    impl Iff4RadTestL {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Iff4RadTestL {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Iff4RadTestL {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Iff4ReplyLValue {
        Select1,
        Select2,
    }
    pub type Iff4ReplyLValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Iff4ReplyLValue) + 'a>;
    impl core::convert::TryFrom<u16> for Iff4ReplyLValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MODE 4 REPLY Light - Press to test(LMB)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Iff4ReplyL(Selector<30346, 16384, 14>);
    impl Iff4ReplyL {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Iff4ReplyL {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Iff4ReplyL {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// IFF Reply Lamp (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct IffReplyL(Led<30344, 1, 0>);
    impl IffReplyL {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for IffReplyL {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffReplyL {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// IFF Test Lamp (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct IffTestL(Led<30344, 2, 1>);
    impl IffTestL {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for IffTestL {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffTestL {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod internallights {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Armament Panel Lights Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct ArmLights(LimitedDial<30286, 65535, 0>);
    impl ArmLights {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ArmLights {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ArmLights {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum BriDimSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type BriDimSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(BriDimSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for BriDimSwValue {
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

    /// Bright/Dim Switch, BRT/NEUT/DIM
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct BriDimSw(Selector<30266, 6144, 11>);
    impl BriDimSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for BriDimSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for BriDimSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CompasLSwValue {
        Select1,
        Select2,
    }
    pub type CompasLSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CompasLSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for CompasLSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Magnetic Compass Light Switch, LIGHT/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CompasLSw(Selector<30266, 512, 9>);
    impl CompasLSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CompasLSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CompasLSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Console Lights Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct ConsoleLights(LimitedDial<30284, 65535, 0>);
    impl ConsoleLights {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ConsoleLights {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ConsoleLights {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Engine Instruments Lights Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct EngineLights(LimitedDial<30282, 65535, 0>);
    impl EngineLights {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EngineLights {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EngineLights {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Flight Instruments Lights Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct FlightLights(LimitedDial<30280, 65535, 0>);
    impl FlightLights {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FlightLights {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FlightLights {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Flood Lights Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct FloodLights(LimitedDial<30278, 65535, 0>);
    impl FloodLights {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FloodLights {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FloodLights {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum McResetBtnValue {
        Select1,
        Select2,
    }
    pub type McResetBtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(McResetBtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for McResetBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Master Caution Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct McResetBtn(Selector<30266, 8192, 13>);
    impl McResetBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for McResetBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for McResetBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum PanelLSwValue {
        Select1,
        Select2,
    }
    pub type PanelLSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(PanelLSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for PanelLSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// AN/ASG-31 Sight Panel Light Button, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct PanelLSw(Selector<30266, 256, 8>);
    impl PanelLSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for PanelLSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for PanelLSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum WarnTSwValue {
        Select1,
        Select2,
    }
    pub type WarnTSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(WarnTSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for WarnTSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Warning Test Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct WarnTSw(Selector<30266, 1024, 10>);
    impl WarnTSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for WarnTSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for WarnTSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod jettison {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EmerJettBValue {
        Select1,
        Select2,
    }
    pub type EmerJettBValueCallback<'a> = alloc::boxed::Box<dyn FnMut(EmerJettBValue) + 'a>;
    impl core::convert::TryFrom<u16> for EmerJettBValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Emergency All Jettison Button - Push to jettison
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct EmerJettB(Selector<30288, 16384, 14>);
    impl EmerJettB {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EmerJettB {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EmerJettB {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EmerJettCoverValue {
        Select1,
        Select2,
    }
    pub type EmerJettCoverValueCallback<'a> = alloc::boxed::Box<dyn FnMut(EmerJettCoverValue) + 'a>;
    impl core::convert::TryFrom<u16> for EmerJettCoverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Emergency All Jettison Button Cover, OPEN
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct EmerJettCover(Selector<30288, 8192, 13>);
    impl EmerJettCover {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EmerJettCover {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EmerJettCover {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum JettBValue {
        Select1,
        Select2,
    }
    pub type JettBValueCallback<'a> = alloc::boxed::Box<dyn FnMut(JettBValue) + 'a>;
    impl core::convert::TryFrom<u16> for JettBValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Select Jettison Button - Push to jettison
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct JettB(Selector<30288, 32768, 15>);
    impl JettB {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for JettB {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for JettB {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum JettSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type JettSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(JettSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for JettSwValue {
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

    /// Select Jettison Switch, SELECT POSITION/OFF/ALL PYLONS
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct JettSw(Selector<30298, 192, 6>);
    impl JettSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for JettSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for JettSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod o2interface {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DiluterLeverValue {
        Select1,
        Select2,
    }
    pub type DiluterLeverValueCallback<'a> = alloc::boxed::Box<dyn FnMut(DiluterLeverValue) + 'a>;
    impl core::convert::TryFrom<u16> for DiluterLeverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Diluter Lever
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct DiluterLever(Selector<30258, 1024, 10>);
    impl DiluterLever {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for DiluterLever {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for DiluterLever {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EmerLValue {
        Select1,
        Select2,
        Select3,
    }
    pub type EmerLValueCallback<'a> = alloc::boxed::Box<dyn FnMut(EmerLValue) + 'a>;
    impl core::convert::TryFrom<u16> for EmerLValue {
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

    /// Emergency Lever, EMERGENCY/NORMAL/TEST MASK
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct EmerL(Selector<30258, 6144, 11>);
    impl EmerL {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EmerL {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EmerL {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum O2LeverValue {
        Select1,
        Select2,
    }
    pub type O2LeverValueCallback<'a> = alloc::boxed::Box<dyn FnMut(O2LeverValue) + 'a>;
    impl core::convert::TryFrom<u16> for O2LeverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Oxygen Supply Lever, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct O2Lever(Selector<30258, 512, 9>);
    impl O2Lever {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for O2Lever {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for O2Lever {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod radar {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RadarAcquisValue {
        Select1,
        Select2,
    }
    pub type RadarAcquisValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RadarAcquisValue) + 'a>;
    impl core::convert::TryFrom<u16> for RadarAcquisValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// AN/APQ-159 Radar Acquisition Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RadarAcquis(Selector<30306, 4, 2>);
    impl RadarAcquis {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RadarAcquis {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadarAcquis {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// AN/APQ-159 Radar Bright Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct RadarBright(LimitedDial<30310, 65535, 0>);
    impl RadarBright {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RadarBright {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadarBright {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// AN/APQ-159 Radar Cursor Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct RadarCursor(LimitedDial<30316, 65535, 0>);
    impl RadarCursor {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RadarCursor {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadarCursor {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// AN/APQ-159 Radar Elevation Antenna Tilt Control Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct RadarElevation(LimitedDial<30304, 65535, 0>);
    impl RadarElevation {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RadarElevation {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadarElevation {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RadarModeValue {
        Select1,
        Select2,
        Select3,
        Select4,
    }
    pub type RadarModeValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RadarModeValue) + 'a>;
    impl core::convert::TryFrom<u16> for RadarModeValue {
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

    /// AN/APQ-159 Radar Mode Selector Switch, OFF/STBY/OPER/TEST
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct RadarMode(Selector<30306, 3, 0>);
    impl RadarMode {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RadarMode {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadarMode {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// AN/APQ-159 Radar Persistence Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct RadarPersis(LimitedDial<30312, 65535, 0>);
    impl RadarPersis {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RadarPersis {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadarPersis {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// AN/APQ-159 Radar Pitch Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct RadarPitch(LimitedDial<30318, 65535, 0>);
    impl RadarPitch {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RadarPitch {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadarPitch {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RadarRangeValue {
        Select1,
        Select2,
        Select3,
        Select4,
    }
    pub type RadarRangeValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RadarRangeValue) + 'a>;
    impl core::convert::TryFrom<u16> for RadarRangeValue {
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

    /// AN/APQ-159 Radar Range Selector Switch [nm], 5/10/20/40
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct RadarRange(Selector<30300, 49152, 14>);
    impl RadarRange {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RadarRange {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadarRange {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// AN/APQ-159 Radar Scale Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct RadarScale(LimitedDial<30308, 65535, 0>);
    impl RadarScale {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RadarScale {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadarScale {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// AN/APQ-159 Radar Video Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct RadarVideo(LimitedDial<30314, 65535, 0>);
    impl RadarVideo {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RadarVideo {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadarVideo {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod rwric {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RwrActValue {
        Select1,
        Select2,
    }
    pub type RwrActValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RwrActValue) + 'a>;
    impl core::convert::TryFrom<u16> for RwrActValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// RWR ACT/PWR Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RwrAct(Selector<30328, 2048, 11>);
    impl RwrAct {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrAct {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrAct {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RwrAltValue {
        Select1,
        Select2,
    }
    pub type RwrAltValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RwrAltValue) + 'a>;
    impl core::convert::TryFrom<u16> for RwrAltValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// RWR ALTITUDE Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RwrAlt(Selector<30328, 32, 5>);
    impl RwrAlt {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrAlt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrAlt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RWR DIM Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct RwrDim(LimitedDial<30336, 65535, 0>);
    impl RwrDim {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrDim {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrDim {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RwrHandValue {
        Select1,
        Select2,
    }
    pub type RwrHandValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RwrHandValue) + 'a>;
    impl core::convert::TryFrom<u16> for RwrHandValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// RWR HANDOFF Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RwrHand(Selector<30328, 16, 4>);
    impl RwrHand {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrHand {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrHand {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RwrLaunchValue {
        Select1,
        Select2,
    }
    pub type RwrLaunchValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RwrLaunchValue) + 'a>;
    impl core::convert::TryFrom<u16> for RwrLaunchValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// RWR LAUNCH Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RwrLaunch(Selector<30328, 1024, 10>);
    impl RwrLaunch {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrLaunch {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrLaunch {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RwrModeValue {
        Select1,
        Select2,
    }
    pub type RwrModeValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RwrModeValue) + 'a>;
    impl core::convert::TryFrom<u16> for RwrModeValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// RWR MODE Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RwrMode(Selector<30328, 4, 2>);
    impl RwrMode {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrMode {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrMode {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RwrPwrValue {
        Select1,
        Select2,
    }
    pub type RwrPwrValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RwrPwrValue) + 'a>;
    impl core::convert::TryFrom<u16> for RwrPwrValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// RWR POWER Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RwrPwr(Selector<30328, 512, 9>);
    impl RwrPwr {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrPwr {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrPwr {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RwrSearchValue {
        Select1,
        Select2,
    }
    pub type RwrSearchValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RwrSearchValue) + 'a>;
    impl core::convert::TryFrom<u16> for RwrSearchValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// RWR SEARCH Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RwrSearch(Selector<30328, 8, 3>);
    impl RwrSearch {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrSearch {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrSearch {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RwrShipValue {
        Select1,
        Select2,
    }
    pub type RwrShipValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RwrShipValue) + 'a>;
    impl core::convert::TryFrom<u16> for RwrShipValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// RWR UNKNOWN SHIP Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RwrShip(Selector<30328, 256, 8>);
    impl RwrShip {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrShip {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrShip {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RwrTValue {
        Select1,
        Select2,
    }
    pub type RwrTValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RwrTValue) + 'a>;
    impl core::convert::TryFrom<u16> for RwrTValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// RWR T Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RwrT(Selector<30328, 64, 6>);
    impl RwrT {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrT {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrT {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RwrTestValue {
        Select1,
        Select2,
    }
    pub type RwrTestValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RwrTestValue) + 'a>;
    impl core::convert::TryFrom<u16> for RwrTestValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// RWR SYS TEST Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RwrTest(Selector<30328, 128, 7>);
    impl RwrTest {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrTest {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrTest {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RWR AUDIO Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct RwrVol(LimitedDial<30338, 65535, 0>);
    impl RwrVol {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrVol {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrVol {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod sight {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SightBitValue {
        Select1,
        Select2,
        Select3,
    }
    pub type SightBitValueCallback<'a> = alloc::boxed::Box<dyn FnMut(SightBitValue) + 'a>;
    impl core::convert::TryFrom<u16> for SightBitValue {
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

    /// AN/ASG-31 Sight BIT Switch, BIT 1/OFF/BIT 2
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct SightBit(Selector<30328, 3, 0>);
    impl SightBit {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SightBit {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SightBit {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SightCageValue {
        Select1,
        Select2,
    }
    pub type SightCageValueCallback<'a> = alloc::boxed::Box<dyn FnMut(SightCageValue) + 'a>;
    impl core::convert::TryFrom<u16> for SightCageValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// AN/ASG-31 Sight Cage Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct SightCage(Selector<30306, 32768, 15>);
    impl SightCage {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SightCage {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SightCage {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// AN/ASG-31 Sight Reticle Depression Knob
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct SightDepress(AnalogDial<30324, 65535, 0>);
    impl SightDepress {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SightDepress {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SightDepress {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// AN/ASG-31 Sight Reticle Intensity Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct SightIntens(LimitedDial<30326, 65535, 0>);
    impl SightIntens {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SightIntens {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SightIntens {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SightModeValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
    }
    pub type SightModeValueCallback<'a> = alloc::boxed::Box<dyn FnMut(SightModeValue) + 'a>;
    impl core::convert::TryFrom<u16> for SightModeValue {
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

    /// AN/ASG-31 Sight Mode Selector, OFF/MSL/A/A1 GUNS/A/A2 GUNS/MAN
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct SightMode(Selector<30306, 28672, 12>);
    impl SightMode {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SightMode {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SightMode {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod standbyattitudeindicator {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

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

    /// Cage SAI
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct SaiCage(Selector<30350, 32, 5>);
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

    /// SAI Pitch Trim
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct SaiPitchTrim(AnalogDial<30442, 65535, 0>);
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

pub mod tacanpanel {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Right Channel Selector
    /// Input: switch to previous or next state
    /// Output: selector position
    pub struct Tacan1(Selector<30474, 15, 0>);
    impl Tacan1 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Tacan1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Tacan1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Left Channel Selector
    /// Input: switch to previous or next state
    /// Output: selector position
    pub struct Tacan10(Selector<30458, 30720, 11>);
    impl Tacan10 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Tacan10 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Tacan10 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// TACAN Channel
    /// Output: TACAN Channel
    pub struct TacanChannel(Display<30478, 4>);
    impl TacanChannel {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for TacanChannel {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TacanChannel {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum TacanHsiValue {
        Select1,
        Select2,
    }
    pub type TacanHsiValueCallback<'a> = alloc::boxed::Box<dyn FnMut(TacanHsiValue) + 'a>;
    impl core::convert::TryFrom<u16> for TacanHsiValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// TACAN Signal on HSI Test Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct TacanHsi(Selector<30474, 64, 6>);
    impl TacanHsi {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TacanHsi {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TacanHsi {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum TacanModeValue {
        Select1,
        Select2,
        Select3,
        Select4,
    }
    pub type TacanModeValueCallback<'a> = alloc::boxed::Box<dyn FnMut(TacanModeValue) + 'a>;
    impl core::convert::TryFrom<u16> for TacanModeValue {
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

    /// TACAN Mode Dial
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct TacanMode(Selector<30474, 48, 4>);
    impl TacanMode {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TacanMode {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TacanMode {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// TACAN Test Indicator Light (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct TacanTest(Led<30458, 1024, 10>);
    impl TacanTest {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for TacanTest {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TacanTest {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// TACAN Signal Volume
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct TacanVol(LimitedDial<30476, 65535, 0>);
    impl TacanVol {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TacanVol {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TacanVol {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum TacanXyValue {
        Select1,
        Select2,
    }
    pub type TacanXyValueCallback<'a> = alloc::boxed::Box<dyn FnMut(TacanXyValue) + 'a>;
    impl core::convert::TryFrom<u16> for TacanXyValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// TACAN Channel X/Y Toggle
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct TacanXy(Selector<30458, 32768, 15>);
    impl TacanXy {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TacanXy {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TacanXy {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod uhfradio {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Uhf0025mhzSelValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
    }
    pub type Uhf0025mhzSelValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Uhf0025mhzSelValue) + 'a>;
    impl core::convert::TryFrom<u16> for Uhf0025mhzSelValue {
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

    /// UHF Radio 0.025 MHz Frequency Selector Knob
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct Uhf0025mhzSel(Selector<30458, 112, 4>);
    impl Uhf0025mhzSel {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Uhf0025mhzSel {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Uhf0025mhzSel {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Uhf01mhzSelValue {
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
        Select11,
    }
    pub type Uhf01mhzSelValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Uhf01mhzSelValue) + 'a>;
    impl core::convert::TryFrom<u16> for Uhf01mhzSelValue {
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
                10 => Ok(Self::Select11),
                value => Err(value),
            }
        }
    }

    /// UHF Radio 0.1 MHz Frequency Selector Knob
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct Uhf01mhzSel(Selector<30458, 15, 0>);
    impl Uhf01mhzSel {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Uhf01mhzSel {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Uhf01mhzSel {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Uhf100mhzSelValue {
        Select1,
        Select2,
        Select3,
        Select4,
    }
    pub type Uhf100mhzSelValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Uhf100mhzSelValue) + 'a>;
    impl core::convert::TryFrom<u16> for Uhf100mhzSelValue {
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

    /// UHF Radio 100 MHz Frequency Selector Knob
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct Uhf100mhzSel(Selector<30454, 96, 5>);
    impl Uhf100mhzSel {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Uhf100mhzSel {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Uhf100mhzSel {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Uhf10mhzSelValue {
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
        Select11,
    }
    pub type Uhf10mhzSelValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Uhf10mhzSelValue) + 'a>;
    impl core::convert::TryFrom<u16> for Uhf10mhzSelValue {
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
                10 => Ok(Self::Select11),
                value => Err(value),
            }
        }
    }

    /// UHF Radio 10 MHz Frequency Selector Knob
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct Uhf10mhzSel(Selector<30454, 1920, 7>);
    impl Uhf10mhzSel {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Uhf10mhzSel {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Uhf10mhzSel {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Uhf1mhzSelValue {
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
        Select11,
    }
    pub type Uhf1mhzSelValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Uhf1mhzSelValue) + 'a>;
    impl core::convert::TryFrom<u16> for Uhf1mhzSelValue {
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
                10 => Ok(Self::Select11),
                value => Err(value),
            }
        }
    }

    /// UHF Radio 1 MHz Frequency Selector Knob
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct Uhf1mhzSel(Selector<30454, 30720, 11>);
    impl Uhf1mhzSel {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Uhf1mhzSel {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Uhf1mhzSel {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UhfAntValue {
        Select1,
        Select2,
        Select3,
    }
    pub type UhfAntValueCallback<'a> = alloc::boxed::Box<dyn FnMut(UhfAntValue) + 'a>;
    impl core::convert::TryFrom<u16> for UhfAntValue {
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

    /// UHF Radio Antenna Selector Switch, UPPER/AUTO/LOWER
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct UhfAnt(Selector<30458, 384, 7>);
    impl UhfAnt {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfAnt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfAnt {
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

    /// Hinged Access Door, OPEN/CLOSE
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UhfDoor(Selector<30454, 32768, 15>);
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
    pub enum UhfFreqValue {
        Select1,
        Select2,
        Select3,
    }
    pub type UhfFreqValueCallback<'a> = alloc::boxed::Box<dyn FnMut(UhfFreqValue) + 'a>;
    impl core::convert::TryFrom<u16> for UhfFreqValue {
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

    /// UHF Radio Frequency Mode Selector Switch, MANUAL/PRESET/GUARD
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct UhfFreq(Selector<30350, 24576, 13>);
    impl UhfFreq {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfFreq {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfFreq {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UhfFuncValue {
        Select1,
        Select2,
        Select3,
        Select4,
    }
    pub type UhfFuncValueCallback<'a> = alloc::boxed::Box<dyn FnMut(UhfFuncValue) + 'a>;
    impl core::convert::TryFrom<u16> for UhfFuncValue {
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

    /// UHF Radio Function Selector Switch, OFF/MAIN/BOTH/ADF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct UhfFunc(Selector<30350, 6144, 11>);
    impl UhfFunc {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfFunc {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfFunc {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UhfMicBtValue {
        Select1,
        Select2,
    }
    pub type UhfMicBtValueCallback<'a> = alloc::boxed::Box<dyn FnMut(UhfMicBtValue) + 'a>;
    impl core::convert::TryFrom<u16> for UhfMicBtValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// UHF Radio Microphone Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UhfMicBt(Selector<30350, 32768, 15>);
    impl UhfMicBt {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfMicBt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfMicBt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UhfPreSetValue {
        Select1,
        Select2,
    }
    pub type UhfPreSetValueCallback<'a> = alloc::boxed::Box<dyn FnMut(UhfPreSetValue) + 'a>;
    impl core::convert::TryFrom<u16> for UhfPreSetValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// UHF Preset Channel Set Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UhfPreSet(Selector<30458, 512, 9>);
    impl UhfPreSet {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfPreSet {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfPreSet {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// UHF Radio Preset Channel Selector Knob
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct UhfPresetSelInt(Selector<30454, 31, 0>);
    impl UhfPresetSelInt {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfPresetSelInt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfPresetSelInt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// UHF Radio Preset Channel Selector Knob
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: possible values: " 1" " 2" " 3" " 4" " 5" " 6" " 7" " 8" " 9" "10" "11" "12" "13" "14" "15" "16" "17" "18" "19" "20"
    pub struct UhfPresetSelStr(Display<30456, 2>);
    impl UhfPresetSelStr {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for UhfPresetSelStr {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfPresetSelStr {
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

    /// UHF Radio Squelch Switch, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UhfSquelchSw(Selector<30350, 1024, 10>);
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
    pub enum UhfToneBtValue {
        Select1,
        Select2,
    }
    pub type UhfToneBtValueCallback<'a> = alloc::boxed::Box<dyn FnMut(UhfToneBtValue) + 'a>;
    impl core::convert::TryFrom<u16> for UhfToneBtValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// UHF Radio Tone Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UhfToneBt(Selector<30350, 512, 9>);
    impl UhfToneBt {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfToneBt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfToneBt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// UHF Radio Volume Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct UhfVol(LimitedDial<30460, 65535, 0>);
    impl UhfVol {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfVol {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfVol {
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

    /// Air Data Computer (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct AirDataComputer(Led<30210, 1, 0>);
    impl AirDataComputer {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for AirDataComputer {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AirDataComputer {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// AoA Flaps (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct AoaFlaps(Led<30208, 32768, 15>);
    impl AoaFlaps {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for AoaFlaps {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AoaFlaps {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// AOA (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct AoaG(Led<30210, 512, 9>);
    impl AoaG {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for AoaG {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AoaG {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// AOA (red)
    /// Output: 0 if light is off, 1 if light is on
    pub struct AoaR(Led<30210, 256, 8>);
    impl AoaR {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for AoaR {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AoaR {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// AOA (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct AoaY(Led<30210, 1024, 10>);
    impl AoaY {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for AoaY {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AoaY {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Canopy Open (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct Canopy(Led<30208, 2, 1>);
    impl Canopy {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for Canopy {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Canopy {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// DC Overload (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct DcOverload(Led<30210, 8, 3>);
    impl DcOverload {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for DcOverload {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for DcOverload {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Directional Gyroscope (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct DirGyro(Led<30210, 2, 1>);
    impl DirGyro {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for DirGyro {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for DirGyro {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Anti Ice (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct EngineAntiIce(Led<30208, 1024, 10>);
    impl EngineAntiIce {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for EngineAntiIce {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EngineAntiIce {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// External Tanks Empty (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct ExtTanks(Led<30208, 64, 6>);
    impl ExtTanks {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ExtTanks {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ExtTanks {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Flight Hydr (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct FlightHydraulic(Led<30208, 32, 5>);
    impl FlightHydraulic {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for FlightHydraulic {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FlightHydraulic {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// HSI Range Flag
    /// Output: 0 if light is off, 1 if light is on
    pub struct HsiRangeFlag(Led<30350, 16, 4>);
    impl HsiRangeFlag {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for HsiRangeFlag {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HsiRangeFlag {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// IFF (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct Iff(Led<30208, 128, 7>);
    impl Iff {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for Iff {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Iff {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Inertial Navigation System (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct Ins(Led<30208, 8192, 13>);
    impl Ins {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for Ins {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Ins {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Left Fire Lamp (red)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LFire(Led<30210, 64, 6>);
    impl LFire {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LFire {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LFire {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// L Fuel Press (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LFuelPress(Led<30208, 4096, 12>);
    impl LFuelPress {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LFuelPress {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LFuelPress {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// L Generator Lamp (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LGen(Led<30208, 1, 0>);
    impl LGen {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LGen {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LGen {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Left Flow Indicator (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LeftFuelLow(Led<30208, 512, 9>);
    impl LeftFuelLow {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LeftFuelLow {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LeftFuelLow {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// MasterCaution Lamp (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct McLight(Led<30210, 32, 5>);
    impl McLight {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for McLight {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for McLight {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Oxygen (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct Oxygen(Led<30208, 256, 8>);
    impl Oxygen {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for Oxygen {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Oxygen {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Right Fire Lamp (red)
    /// Output: 0 if light is off, 1 if light is on
    pub struct RFire(Led<30210, 128, 7>);
    impl RFire {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RFire {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RFire {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// R Fuel Press (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct RFuelPress(Led<30208, 16384, 14>);
    impl RFuelPress {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RFuelPress {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RFuelPress {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// R Generator Lamp (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct RGen(Led<30208, 4, 2>);
    impl RGen {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RGen {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RGen {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Radar ExcessG Light
    /// Output: 0 if light is off, 1 if light is on
    pub struct RadarExcess(Led<30306, 1024, 10>);
    impl RadarExcess {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RadarExcess {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadarExcess {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Radar Fail Light
    /// Output: 0 if light is off, 1 if light is on
    pub struct RadarFail(Led<30306, 256, 8>);
    impl RadarFail {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RadarFail {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadarFail {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Radar InRange Light
    /// Output: 0 if light is off, 1 if light is on
    pub struct RadarInrange(Led<30306, 128, 7>);
    impl RadarInrange {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RadarInrange {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadarInrange {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Radar LockOn Light
    /// Output: 0 if light is off, 1 if light is on
    pub struct RadarLockon(Led<30306, 512, 9>);
    impl RadarLockon {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RadarLockon {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadarLockon {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Radar Range Scale 10
    /// Output: 0 if light is off, 1 if light is on
    pub struct RadarScale10(Led<30306, 16, 4>);
    impl RadarScale10 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RadarScale10 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadarScale10 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Radar Range Scale 20
    /// Output: 0 if light is off, 1 if light is on
    pub struct RadarScale20(Led<30306, 32, 5>);
    impl RadarScale20 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RadarScale20 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadarScale20 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Radar Range Scale 40
    /// Output: 0 if light is off, 1 if light is on
    pub struct RadarScale40(Led<30306, 64, 6>);
    impl RadarScale40 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RadarScale40 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadarScale40 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Radar Range Scale 5
    /// Output: 0 if light is off, 1 if light is on
    pub struct RadarScale5(Led<30306, 8, 3>);
    impl RadarScale5 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RadarScale5 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadarScale5 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Radar Scale Brightness
    /// Output: 0 if light is off, 1 if light is on
    pub struct RadarScaleBright(Led<30306, 2048, 11>);
    impl RadarScaleBright {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RadarScaleBright {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadarScaleBright {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Right Flow Indicator (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct RightFuelLow(Led<30208, 2048, 11>);
    impl RightFuelLow {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RightFuelLow {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RightFuelLow {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Spare 1 (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct Spare1(Led<30208, 16, 4>);
    impl Spare1 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for Spare1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Spare1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Spare 2 (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct Spare2(Led<30210, 4, 2>);
    impl Spare2 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for Spare2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Spare2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Spare 3 (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct Spare3(Led<30210, 16, 4>);
    impl Spare3 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for Spare3 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Spare3 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Utility Hydr (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct UtilHydraulic(Led<30208, 8, 3>);
    impl UtilHydraulic {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for UtilHydraulic {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UtilHydraulic {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod weaponscontrol {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Armpos1Value {
        Select1,
        Select2,
    }
    pub type Armpos1ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Armpos1Value) + 'a>;
    impl core::convert::TryFrom<u16> for Armpos1Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Armament Position Selector - L WINGTIP, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Armpos1(Selector<30298, 256, 8>);
    impl Armpos1 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Armpos1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Armpos1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Armpos2Value {
        Select1,
        Select2,
    }
    pub type Armpos2ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Armpos2Value) + 'a>;
    impl core::convert::TryFrom<u16> for Armpos2Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Armament Position Selector - L OUTBD, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Armpos2(Selector<30298, 512, 9>);
    impl Armpos2 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Armpos2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Armpos2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Armpos3Value {
        Select1,
        Select2,
    }
    pub type Armpos3ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Armpos3Value) + 'a>;
    impl core::convert::TryFrom<u16> for Armpos3Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Armament Position Selector - L INBD, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Armpos3(Selector<30298, 1024, 10>);
    impl Armpos3 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Armpos3 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Armpos3 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Armpos4Value {
        Select1,
        Select2,
    }
    pub type Armpos4ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Armpos4Value) + 'a>;
    impl core::convert::TryFrom<u16> for Armpos4Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Armament Position Selector - CENTER, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Armpos4(Selector<30298, 2048, 11>);
    impl Armpos4 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Armpos4 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Armpos4 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Armpos5Value {
        Select1,
        Select2,
    }
    pub type Armpos5ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Armpos5Value) + 'a>;
    impl core::convert::TryFrom<u16> for Armpos5Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Armament Position Selector - R INBD, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Armpos5(Selector<30298, 4096, 12>);
    impl Armpos5 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Armpos5 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Armpos5 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Armpos6Value {
        Select1,
        Select2,
    }
    pub type Armpos6ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Armpos6Value) + 'a>;
    impl core::convert::TryFrom<u16> for Armpos6Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Armament Position Selector - R OUTBD, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Armpos6(Selector<30298, 8192, 13>);
    impl Armpos6 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Armpos6 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Armpos6 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Armpos7Value {
        Select1,
        Select2,
    }
    pub type Armpos7ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Armpos7Value) + 'a>;
    impl core::convert::TryFrom<u16> for Armpos7Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Armament Position Selector - R WINGTIP, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Armpos7(Selector<30298, 16384, 14>);
    impl Armpos7 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Armpos7 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Armpos7 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum BombsArmSwValue {
        Select1,
        Select2,
        Select3,
        Select4,
    }
    pub type BombsArmSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(BombsArmSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for BombsArmSwValue {
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

    /// Bombs Arm Switch, SAFE/TAIL/NOSE & TAIL/NOSE
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct BombsArmSw(Selector<30300, 12, 2>);
    impl BombsArmSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for BombsArmSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for BombsArmSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ExtStoreValue {
        Select1,
        Select2,
        Select3,
        Select4,
    }
    pub type ExtStoreValueCallback<'a> = alloc::boxed::Box<dyn FnMut(ExtStoreValue) + 'a>;
    impl core::convert::TryFrom<u16> for ExtStoreValue {
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

    /// External Stores Selector RIPL/BOMB/SAFE/RKT DISP
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct ExtStore(Selector<30300, 192, 6>);
    impl ExtStore {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ExtStore {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ExtStore {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IntSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type IntSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IntSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for IntSwValue {
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

    /// Interval Switch [sec], .06/.10/.14
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct IntSw(Selector<30300, 3, 0>);
    impl IntSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IntSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IntSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MasterArmValue {
        Select1,
        Select2,
        Select3,
    }
    pub type MasterArmValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MasterArmValue) + 'a>;
    impl core::convert::TryFrom<u16> for MasterArmValue {
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

    /// Master Arm Guns, GUNS MSL & CAMR/OFF/CAMR ONLY
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct MasterArm(Selector<30300, 48, 4>);
    impl MasterArm {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MasterArm {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MasterArm {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MasterArmGuardValue {
        Select1,
        Select2,
    }
    pub type MasterArmGuardValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(MasterArmGuardValue) + 'a>;
    impl core::convert::TryFrom<u16> for MasterArmGuardValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Master Arm Safe Guard
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MasterArmGuard(Selector<30298, 32768, 15>);
    impl MasterArmGuard {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MasterArmGuard {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MasterArmGuard {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MissileUncageValue {
        Select1,
        Select2,
    }
    pub type MissileUncageValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MissileUncageValue) + 'a>;
    impl core::convert::TryFrom<u16> for MissileUncageValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Missile Uncage Switch - Press and hold to uncage missile seeker head
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MissileUncage(Selector<30300, 256, 8>);
    impl MissileUncage {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MissileUncage {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MissileUncage {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Missile Volume Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct MissileVol(LimitedDial<30302, 65535, 0>);
    impl MissileVol {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MissileVol {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MissileVol {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum WeaponReleaseValue {
        Select1,
        Select2,
    }
    pub type WeaponReleaseValueCallback<'a> = alloc::boxed::Box<dyn FnMut(WeaponReleaseValue) + 'a>;
    impl core::convert::TryFrom<u16> for WeaponReleaseValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Weapon Release Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct WeaponRelease(Selector<30300, 512, 9>);
    impl WeaponRelease {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for WeaponRelease {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for WeaponRelease {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub enum RegisteredControl<'a> {
    AFlaps {
        control: controlinterface::AFlaps,
        callback: controlinterface::AFlapsValueCallback<'a>,
    },
    Acc {
        control: accelerometer::Acc,
        callback: accelerometer::AccValueCallback<'a>,
    },
    Accelerometer {
        control: gauges::Accelerometer,
        callback: IntegerCallback<'a>,
    },
    AccelerometerMax {
        control: gauges::AccelerometerMax,
        callback: IntegerCallback<'a>,
    },
    AccelerometerMin {
        control: gauges::AccelerometerMin,
        callback: IntegerCallback<'a>,
    },
    AhrsErect {
        control: ahrs::AhrsErect,
        callback: ahrs::AhrsErectValueCallback<'a>,
    },
    AhrsFastSlave {
        control: ahrs::AhrsFastSlave,
        callback: ahrs::AhrsFastSlaveValueCallback<'a>,
    },
    AhrsNavMode {
        control: ahrs::AhrsNavMode,
        callback: ahrs::AhrsNavModeValueCallback<'a>,
    },
    AiBank {
        control: gauges::AiBank,
        callback: IntegerCallback<'a>,
    },
    AiOffFlag {
        control: gauges::AiOffFlag,
        callback: IntegerCallback<'a>,
    },
    AiPitch {
        control: gauges::AiPitch,
        callback: IntegerCallback<'a>,
    },
    AiPitchTrim {
        control: attitudeindicator::AiPitchTrim,
        callback: IntegerCallback<'a>,
    },
    AirDataComputer {
        control: warningcautionandindicatorlights::AirDataComputer,
        callback: LedCallback<'a>,
    },
    Airspeed {
        control: gauges::Airspeed,
        callback: IntegerCallback<'a>,
    },
    AirspeedMax {
        control: gauges::AirspeedMax,
        callback: IntegerCallback<'a>,
    },
    AirspeedSet {
        control: gauges::AirspeedSet,
        callback: IntegerCallback<'a>,
    },
    Alt10000C {
        control: gauges::Alt10000C,
        callback: IntegerCallback<'a>,
    },
    Alt1000C {
        control: gauges::Alt1000C,
        callback: IntegerCallback<'a>,
    },
    Alt100C {
        control: gauges::Alt100C,
        callback: IntegerCallback<'a>,
    },
    Alt100P {
        control: gauges::Alt100P,
        callback: IntegerCallback<'a>,
    },
    AltElect {
        control: altimeter::AltElect,
        callback: altimeter::AltElectValueCallback<'a>,
    },
    AltPneuFlag {
        control: gauges::AltPneuFlag,
        callback: IntegerCallback<'a>,
    },
    AltZero {
        control: altimeter::AltZero,
        callback: IntegerCallback<'a>,
    },
    AoaFlaps {
        control: warningcautionandindicatorlights::AoaFlaps,
        callback: LedCallback<'a>,
    },
    AoaG {
        control: warningcautionandindicatorlights::AoaG,
        callback: LedCallback<'a>,
    },
    AoaGauge {
        control: gauges::AoaGauge,
        callback: IntegerCallback<'a>,
    },
    AoaR {
        control: warningcautionandindicatorlights::AoaR,
        callback: LedCallback<'a>,
    },
    AoaY {
        control: warningcautionandindicatorlights::AoaY,
        callback: LedCallback<'a>,
    },
    ArmLights {
        control: internallights::ArmLights,
        callback: IntegerCallback<'a>,
    },
    Armpos1 {
        control: weaponscontrol::Armpos1,
        callback: weaponscontrol::Armpos1ValueCallback<'a>,
    },
    Armpos2 {
        control: weaponscontrol::Armpos2,
        callback: weaponscontrol::Armpos2ValueCallback<'a>,
    },
    Armpos3 {
        control: weaponscontrol::Armpos3,
        callback: weaponscontrol::Armpos3ValueCallback<'a>,
    },
    Armpos4 {
        control: weaponscontrol::Armpos4,
        callback: weaponscontrol::Armpos4ValueCallback<'a>,
    },
    Armpos5 {
        control: weaponscontrol::Armpos5,
        callback: weaponscontrol::Armpos5ValueCallback<'a>,
    },
    Armpos6 {
        control: weaponscontrol::Armpos6,
        callback: weaponscontrol::Armpos6ValueCallback<'a>,
    },
    Armpos7 {
        control: weaponscontrol::Armpos7,
        callback: weaponscontrol::Armpos7ValueCallback<'a>,
    },
    Autobal {
        control: fuel::Autobal,
        callback: fuel::AutobalValueCallback<'a>,
    },
    AuxintDoor {
        control: gauges::AuxintDoor,
        callback: IntegerCallback<'a>,
    },
    Beacon {
        control: externallights::Beacon,
        callback: externallights::BeaconValueCallback<'a>,
    },
    BombsArmSw {
        control: weaponscontrol::BombsArmSw,
        callback: weaponscontrol::BombsArmSwValueCallback<'a>,
    },
    BriDimSw {
        control: internallights::BriDimSw,
        callback: internallights::BriDimSwValueCallback<'a>,
    },
    CabinDefog {
        control: ecinterface::CabinDefog,
        callback: IntegerCallback<'a>,
    },
    CabinPC {
        control: ecinterface::CabinPC,
        callback: ecinterface::CabinPCValueCallback<'a>,
    },
    CabinPSw {
        control: ecinterface::CabinPSw,
        callback: ecinterface::CabinPSwValueCallback<'a>,
    },
    CabinPress {
        control: gauges::CabinPress,
        callback: IntegerCallback<'a>,
    },
    CabinTemp {
        control: ecinterface::CabinTemp,
        callback: IntegerCallback<'a>,
    },
    CabinTempSw {
        control: ecinterface::CabinTempSw,
        callback: ecinterface::CabinTempSwValueCallback<'a>,
    },
    CameraFps {
        control: camera::CameraFps,
        callback: camera::CameraFpsValueCallback<'a>,
    },
    CameraLens {
        control: camera::CameraLens,
        callback: IntegerCallback<'a>,
    },
    CameraOverrun {
        control: camera::CameraOverrun,
        callback: camera::CameraOverrunValueCallback<'a>,
    },
    CameraRun {
        control: camera::CameraRun,
        callback: camera::CameraRunValueCallback<'a>,
    },
    Canopy {
        control: warningcautionandindicatorlights::Canopy,
        callback: LedCallback<'a>,
    },
    CanopyJettison {
        control: cockpit::CanopyJettison,
        callback: cockpit::CanopyJettisonValueCallback<'a>,
    },
    CanopyLever {
        control: cockpit::CanopyLever,
        callback: cockpit::CanopyLeverValueCallback<'a>,
    },
    CanopyPos {
        control: gauges::CanopyPos,
        callback: IntegerCallback<'a>,
    },
    ChaffCount {
        control: cmds::ChaffCount,
        callback: cmds::ChaffCountValueCallback<'a>,
    },
    ChaffCount1 {
        control: gauges::ChaffCount1,
        callback: IntegerCallback<'a>,
    },
    ChaffCount10 {
        control: gauges::ChaffCount10,
        callback: IntegerCallback<'a>,
    },
    ChaffMode {
        control: cmds::ChaffMode,
        callback: cmds::ChaffModeValueCallback<'a>,
    },
    ClockCurrH {
        control: gauges::ClockCurrH,
        callback: IntegerCallback<'a>,
    },
    ClockCurrM {
        control: gauges::ClockCurrM,
        callback: IntegerCallback<'a>,
    },
    ClockElapM {
        control: gauges::ClockElapM,
        callback: IntegerCallback<'a>,
    },
    ClockElapS {
        control: gauges::ClockElapS,
        callback: IntegerCallback<'a>,
    },
    ClockSet {
        control: clock::ClockSet,
        callback: clock::ClockSetValueCallback<'a>,
    },
    ClockWind {
        control: clock::ClockWind,
        callback: clock::ClockWindValueCallback<'a>,
    },
    ClockWindRotatry {
        control: clock::ClockWindRotatry,
        callback: IntegerCallback<'a>,
    },
    CmChaffcntDisplay {
        control: cmds::CmChaffcntDisplay,
        callback: IntegerCallback<'a>,
    },
    CmFlarecntDisplay {
        control: cmds::CmFlarecntDisplay,
        callback: IntegerCallback<'a>,
    },
    CompasLSw {
        control: internallights::CompasLSw,
        callback: internallights::CompasLSwValueCallback<'a>,
    },
    ConsoleLights {
        control: internallights::ConsoleLights,
        callback: IntegerCallback<'a>,
    },
    Crossfeed {
        control: fuel::Crossfeed,
        callback: fuel::CrossfeedValueCallback<'a>,
    },
    DcOverload {
        control: warningcautionandindicatorlights::DcOverload,
        callback: LedCallback<'a>,
    },
    DiluterLever {
        control: o2interface::DiluterLever,
        callback: o2interface::DiluterLeverValueCallback<'a>,
    },
    DirGyro {
        control: warningcautionandindicatorlights::DirGyro,
        callback: LedCallback<'a>,
    },
    DragChute {
        control: cockpit::DragChute,
        callback: cockpit::DragChuteValueCallback<'a>,
    },
    ElapTime {
        control: clock::ElapTime,
        callback: clock::ElapTimeValueCallback<'a>,
    },
    EmerJettB {
        control: jettison::EmerJettB,
        callback: jettison::EmerJettBValueCallback<'a>,
    },
    EmerJettCover {
        control: jettison::EmerJettCover,
        callback: jettison::EmerJettCoverValueCallback<'a>,
    },
    EmerL {
        control: o2interface::EmerL,
        callback: o2interface::EmerLValueCallback<'a>,
    },
    EngineAntiIce {
        control: warningcautionandindicatorlights::EngineAntiIce,
        callback: LedCallback<'a>,
    },
    EngineLights {
        control: internallights::EngineLights,
        callback: IntegerCallback<'a>,
    },
    ExhaustTempL {
        control: gauges::ExhaustTempL,
        callback: IntegerCallback<'a>,
    },
    ExhaustTempR {
        control: gauges::ExhaustTempR,
        callback: IntegerCallback<'a>,
    },
    ExtBrakeCute {
        control: externalaircraftmodel::ExtBrakeCute,
        callback: IntegerCallback<'a>,
    },
    ExtCenterSw {
        control: fuel::ExtCenterSw,
        callback: fuel::ExtCenterSwValueCallback<'a>,
    },
    ExtPositionLights {
        control: externalaircraftmodel::ExtPositionLights,
        callback: IntegerCallback<'a>,
    },
    ExtPylonSw {
        control: fuel::ExtPylonSw,
        callback: fuel::ExtPylonSwValueCallback<'a>,
    },
    ExtSpeedBrakeLeft {
        control: externalaircraftmodel::ExtSpeedBrakeLeft,
        callback: IntegerCallback<'a>,
    },
    ExtSpeedBrakeRight {
        control: externalaircraftmodel::ExtSpeedBrakeRight,
        callback: IntegerCallback<'a>,
    },
    ExtStore {
        control: weaponscontrol::ExtStore,
        callback: weaponscontrol::ExtStoreValueCallback<'a>,
    },
    ExtStrobeLights {
        control: externalaircraftmodel::ExtStrobeLights,
        callback: IntegerCallback<'a>,
    },
    ExtTanks {
        control: warningcautionandindicatorlights::ExtTanks,
        callback: LedCallback<'a>,
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
    FO2Sw {
        control: electric::FO2Sw,
        callback: electric::FO2SwValueCallback<'a>,
    },
    FlChaffBt {
        control: cmds::FlChaffBt,
        callback: cmds::FlChaffBtValueCallback<'a>,
    },
    FlCount {
        control: cmds::FlCount,
        callback: cmds::FlCountValueCallback<'a>,
    },
    FlCount1 {
        control: gauges::FlCount1,
        callback: IntegerCallback<'a>,
    },
    FlCount10 {
        control: gauges::FlCount10,
        callback: IntegerCallback<'a>,
    },
    FlJettCover {
        control: cmds::FlJettCover,
        callback: cmds::FlJettCoverValueCallback<'a>,
    },
    FlJettSw {
        control: cmds::FlJettSw,
        callback: cmds::FlJettSwValueCallback<'a>,
    },
    FlapIndicator {
        control: gauges::FlapIndicator,
        callback: IntegerCallback<'a>,
    },
    Flaps {
        control: controlinterface::Flaps,
        callback: controlinterface::FlapsValueCallback<'a>,
    },
    FlareModeSel {
        control: cmds::FlareModeSel,
        callback: cmds::FlareModeSelValueCallback<'a>,
    },
    FlightHydraulic {
        control: warningcautionandindicatorlights::FlightHydraulic,
        callback: LedCallback<'a>,
    },
    FlightLights {
        control: internallights::FlightLights,
        callback: IntegerCallback<'a>,
    },
    FlightPress {
        control: gauges::FlightPress,
        callback: IntegerCallback<'a>,
    },
    FloodLights {
        control: internallights::FloodLights,
        callback: IntegerCallback<'a>,
    },
    FormationLights {
        control: externallights::FormationLights,
        callback: IntegerCallback<'a>,
    },
    FuelflowL {
        control: gauges::FuelflowL,
        callback: IntegerCallback<'a>,
    },
    FuelflowR {
        control: gauges::FuelflowR,
        callback: IntegerCallback<'a>,
    },
    FuelquantityL {
        control: gauges::FuelquantityL,
        callback: IntegerCallback<'a>,
    },
    FuelquantityR {
        control: gauges::FuelquantityR,
        callback: IntegerCallback<'a>,
    },
    GwLight {
        control: gearinterface::GwLight,
        callback: LedCallback<'a>,
    },
    Hook {
        control: gearinterface::Hook,
        callback: gearinterface::HookValueCallback<'a>,
    },
    HookLight {
        control: gearinterface::HookLight,
        callback: LedCallback<'a>,
    },
    HsiBerPoint {
        control: gauges::HsiBerPoint,
        callback: IntegerCallback<'a>,
    },
    HsiComp {
        control: gauges::HsiComp,
        callback: IntegerCallback<'a>,
    },
    HsiCrs {
        control: gauges::HsiCrs,
        callback: IntegerCallback<'a>,
    },
    HsiCrs1 {
        control: gauges::HsiCrs1,
        callback: IntegerCallback<'a>,
    },
    HsiCrs10010 {
        control: gauges::HsiCrs10010,
        callback: IntegerCallback<'a>,
    },
    HsiCrsDev {
        control: gauges::HsiCrsDev,
        callback: IntegerCallback<'a>,
    },
    HsiCrsKnob {
        control: hsi::HsiCrsKnob,
        callback: IntegerCallback<'a>,
    },
    HsiDev {
        control: gauges::HsiDev,
        callback: IntegerCallback<'a>,
    },
    HsiHdg {
        control: gauges::HsiHdg,
        callback: IntegerCallback<'a>,
    },
    HsiHdgKnob {
        control: hsi::HsiHdgKnob,
        callback: IntegerCallback<'a>,
    },
    HsiOff {
        control: gauges::HsiOff,
        callback: IntegerCallback<'a>,
    },
    HsiRange1 {
        control: gauges::HsiRange1,
        callback: IntegerCallback<'a>,
    },
    HsiRange10 {
        control: gauges::HsiRange10,
        callback: IntegerCallback<'a>,
    },
    HsiRange100 {
        control: gauges::HsiRange100,
        callback: IntegerCallback<'a>,
    },
    HsiRangeFlag {
        control: warningcautionandindicatorlights::HsiRangeFlag,
        callback: LedCallback<'a>,
    },
    HsiToFrom {
        control: gauges::HsiToFrom,
        callback: IntegerCallback<'a>,
    },
    IasSet {
        control: airspeedindicator::IasSet,
        callback: IntegerCallback<'a>,
    },
    Iff {
        control: warningcautionandindicatorlights::Iff,
        callback: LedCallback<'a>,
    },
    Iff4CodeSel {
        control: iff::Iff4CodeSel,
        callback: iff::Iff4CodeSelValueCallback<'a>,
    },
    Iff4CodeSelPull {
        control: iff::Iff4CodeSelPull,
        callback: iff::Iff4CodeSelPullValueCallback<'a>,
    },
    Iff4Control {
        control: iff::Iff4Control,
        callback: iff::Iff4ControlValueCallback<'a>,
    },
    Iff4Ident {
        control: iff::Iff4Ident,
        callback: iff::Iff4IdentValueCallback<'a>,
    },
    Iff4M1 {
        control: iff::Iff4M1,
        callback: iff::Iff4M1ValueCallback<'a>,
    },
    Iff4M2 {
        control: iff::Iff4M2,
        callback: iff::Iff4M2ValueCallback<'a>,
    },
    Iff4M3 {
        control: iff::Iff4M3,
        callback: iff::Iff4M3ValueCallback<'a>,
    },
    Iff4Master {
        control: iff::Iff4Master,
        callback: iff::Iff4MasterValueCallback<'a>,
    },
    Iff4MasterPull {
        control: iff::Iff4MasterPull,
        callback: iff::Iff4MasterPullValueCallback<'a>,
    },
    Iff4Mc {
        control: iff::Iff4Mc,
        callback: iff::Iff4McValueCallback<'a>,
    },
    Iff4MonContr {
        control: iff::Iff4MonContr,
        callback: iff::Iff4MonContrValueCallback<'a>,
    },
    Iff4Rad {
        control: iff::Iff4Rad,
        callback: iff::Iff4RadValueCallback<'a>,
    },
    Iff4RadTestL {
        control: iff::Iff4RadTestL,
        callback: iff::Iff4RadTestLValueCallback<'a>,
    },
    Iff4ReplyL {
        control: iff::Iff4ReplyL,
        callback: iff::Iff4ReplyLValueCallback<'a>,
    },
    IffReplyL {
        control: iff::IffReplyL,
        callback: LedCallback<'a>,
    },
    IffTestL {
        control: iff::IffTestL,
        callback: LedCallback<'a>,
    },
    InletHeater {
        control: engineinterface::InletHeater,
        callback: engineinterface::InletHeaterValueCallback<'a>,
    },
    Ins {
        control: warningcautionandindicatorlights::Ins,
        callback: LedCallback<'a>,
    },
    IntSw {
        control: weaponscontrol::IntSw,
        callback: weaponscontrol::IntSwValueCallback<'a>,
    },
    JettB {
        control: jettison::JettB,
        callback: jettison::JettBValueCallback<'a>,
    },
    JettSw {
        control: jettison::JettSw,
        callback: jettison::JettSwValueCallback<'a>,
    },
    LBoostpump {
        control: fuel::LBoostpump,
        callback: fuel::LBoostpumpValueCallback<'a>,
    },
    LFire {
        control: warningcautionandindicatorlights::LFire,
        callback: LedCallback<'a>,
    },
    LFuelPress {
        control: warningcautionandindicatorlights::LFuelPress,
        callback: LedCallback<'a>,
    },
    LGen {
        control: warningcautionandindicatorlights::LGen,
        callback: LedCallback<'a>,
    },
    LGenerator {
        control: electric::LGenerator,
        callback: electric::LGeneratorValueCallback<'a>,
    },
    LShutoffC {
        control: fuel::LShutoffC,
        callback: fuel::LShutoffCValueCallback<'a>,
    },
    LShutoffS {
        control: fuel::LShutoffS,
        callback: fuel::LShutoffSValueCallback<'a>,
    },
    LStart {
        control: engineinterface::LStart,
        callback: engineinterface::LStartValueCallback<'a>,
    },
    LeftFuelLow {
        control: warningcautionandindicatorlights::LeftFuelLow,
        callback: LedCallback<'a>,
    },
    LeftLight {
        control: gearinterface::LeftLight,
        callback: LedCallback<'a>,
    },
    LgAltRel {
        control: gearinterface::LgAltRel,
        callback: gearinterface::LgAltRelValueCallback<'a>,
    },
    LgAltRes {
        control: gearinterface::LgAltRes,
        callback: gearinterface::LgAltResValueCallback<'a>,
    },
    LgDownL {
        control: gearinterface::LgDownL,
        callback: gearinterface::LgDownLValueCallback<'a>,
    },
    LgLeverSwitch {
        control: gearinterface::LgLeverSwitch,
        callback: gearinterface::LgLeverSwitchValueCallback<'a>,
    },
    LgLight {
        control: externallights::LgLight,
        callback: externallights::LgLightValueCallback<'a>,
    },
    LgTest {
        control: gearinterface::LgTest,
        callback: gearinterface::LgTestValueCallback<'a>,
    },
    LgfSilence {
        control: gearinterface::LgfSilence,
        callback: gearinterface::LgfSilenceValueCallback<'a>,
    },
    Mach {
        control: gauges::Mach,
        callback: IntegerCallback<'a>,
    },
    MasterArm {
        control: weaponscontrol::MasterArm,
        callback: weaponscontrol::MasterArmValueCallback<'a>,
    },
    MasterArmGuard {
        control: weaponscontrol::MasterArmGuard,
        callback: weaponscontrol::MasterArmGuardValueCallback<'a>,
    },
    McLight {
        control: warningcautionandindicatorlights::McLight,
        callback: LedCallback<'a>,
    },
    McResetBtn {
        control: internallights::McResetBtn,
        callback: internallights::McResetBtnValueCallback<'a>,
    },
    MissileUncage {
        control: weaponscontrol::MissileUncage,
        callback: weaponscontrol::MissileUncageValueCallback<'a>,
    },
    MissileVol {
        control: weaponscontrol::MissileVol,
        callback: IntegerCallback<'a>,
    },
    MotorRun {
        control: gauges::MotorRun,
        callback: IntegerCallback<'a>,
    },
    NavLights {
        control: externallights::NavLights,
        callback: IntegerCallback<'a>,
    },
    NgTest {
        control: gearinterface::NgTest,
        callback: gearinterface::NgTestValueCallback<'a>,
    },
    NoseLight {
        control: gearinterface::NoseLight,
        callback: LedCallback<'a>,
    },
    NozzleposL {
        control: gauges::NozzleposL,
        callback: IntegerCallback<'a>,
    },
    NozzleposR {
        control: gauges::NozzleposR,
        callback: IntegerCallback<'a>,
    },
    NsStruct {
        control: gearinterface::NsStruct,
        callback: gearinterface::NsStructValueCallback<'a>,
    },
    Nws {
        control: gearinterface::Nws,
        callback: gearinterface::NwsValueCallback<'a>,
    },
    O2Lever {
        control: o2interface::O2Lever,
        callback: o2interface::O2LeverValueCallback<'a>,
    },
    O2flowindicator {
        control: gauges::O2flowindicator,
        callback: IntegerCallback<'a>,
    },
    O2flowpressure {
        control: gauges::O2flowpressure,
        callback: IntegerCallback<'a>,
    },
    O2quantity {
        control: gauges::O2quantity,
        callback: IntegerCallback<'a>,
    },
    OilpressL {
        control: gauges::OilpressL,
        callback: IntegerCallback<'a>,
    },
    OilpressR {
        control: gauges::OilpressR,
        callback: IntegerCallback<'a>,
    },
    Oxygen {
        control: warningcautionandindicatorlights::Oxygen,
        callback: LedCallback<'a>,
    },
    PanelLSw {
        control: internallights::PanelLSw,
        callback: internallights::PanelLSwValueCallback<'a>,
    },
    PitchDamper {
        control: controlinterface::PitchDamper,
        callback: controlinterface::PitchDamperValueCallback<'a>,
    },
    PitchDamperCut {
        control: controlinterface::PitchDamperCut,
        callback: controlinterface::PitchDamperCutValueCallback<'a>,
    },
    PitotHeater {
        control: electric::PitotHeater,
        callback: electric::PitotHeaterValueCallback<'a>,
    },
    Press0 {
        control: gauges::Press0,
        callback: IntegerCallback<'a>,
    },
    Press1 {
        control: gauges::Press1,
        callback: IntegerCallback<'a>,
    },
    Press2 {
        control: gauges::Press2,
        callback: IntegerCallback<'a>,
    },
    Press3 {
        control: gauges::Press3,
        callback: IntegerCallback<'a>,
    },
    RBoostpump {
        control: fuel::RBoostpump,
        callback: fuel::RBoostpumpValueCallback<'a>,
    },
    RFire {
        control: warningcautionandindicatorlights::RFire,
        callback: LedCallback<'a>,
    },
    RFuelPress {
        control: warningcautionandindicatorlights::RFuelPress,
        callback: LedCallback<'a>,
    },
    RGen {
        control: warningcautionandindicatorlights::RGen,
        callback: LedCallback<'a>,
    },
    RGenerator {
        control: electric::RGenerator,
        callback: electric::RGeneratorValueCallback<'a>,
    },
    RShutoffC {
        control: fuel::RShutoffC,
        callback: fuel::RShutoffCValueCallback<'a>,
    },
    RShutoffS {
        control: fuel::RShutoffS,
        callback: fuel::RShutoffSValueCallback<'a>,
    },
    RStart {
        control: engineinterface::RStart,
        callback: engineinterface::RStartValueCallback<'a>,
    },
    RadarAcquis {
        control: radar::RadarAcquis,
        callback: radar::RadarAcquisValueCallback<'a>,
    },
    RadarBright {
        control: radar::RadarBright,
        callback: IntegerCallback<'a>,
    },
    RadarCursor {
        control: radar::RadarCursor,
        callback: IntegerCallback<'a>,
    },
    RadarElevation {
        control: radar::RadarElevation,
        callback: IntegerCallback<'a>,
    },
    RadarExcess {
        control: warningcautionandindicatorlights::RadarExcess,
        callback: LedCallback<'a>,
    },
    RadarFail {
        control: warningcautionandindicatorlights::RadarFail,
        callback: LedCallback<'a>,
    },
    RadarInrange {
        control: warningcautionandindicatorlights::RadarInrange,
        callback: LedCallback<'a>,
    },
    RadarLockon {
        control: warningcautionandindicatorlights::RadarLockon,
        callback: LedCallback<'a>,
    },
    RadarMode {
        control: radar::RadarMode,
        callback: radar::RadarModeValueCallback<'a>,
    },
    RadarPersis {
        control: radar::RadarPersis,
        callback: IntegerCallback<'a>,
    },
    RadarPitch {
        control: radar::RadarPitch,
        callback: IntegerCallback<'a>,
    },
    RadarRange {
        control: radar::RadarRange,
        callback: radar::RadarRangeValueCallback<'a>,
    },
    RadarScale {
        control: radar::RadarScale,
        callback: IntegerCallback<'a>,
    },
    RadarScale10 {
        control: warningcautionandindicatorlights::RadarScale10,
        callback: LedCallback<'a>,
    },
    RadarScale20 {
        control: warningcautionandindicatorlights::RadarScale20,
        callback: LedCallback<'a>,
    },
    RadarScale40 {
        control: warningcautionandindicatorlights::RadarScale40,
        callback: LedCallback<'a>,
    },
    RadarScale5 {
        control: warningcautionandindicatorlights::RadarScale5,
        callback: LedCallback<'a>,
    },
    RadarScaleBright {
        control: warningcautionandindicatorlights::RadarScaleBright,
        callback: LedCallback<'a>,
    },
    RadarVideo {
        control: radar::RadarVideo,
        callback: IntegerCallback<'a>,
    },
    RgTest {
        control: gearinterface::RgTest,
        callback: gearinterface::RgTestValueCallback<'a>,
    },
    RightFuelLow {
        control: warningcautionandindicatorlights::RightFuelLow,
        callback: LedCallback<'a>,
    },
    RightLight {
        control: gearinterface::RightLight,
        callback: LedCallback<'a>,
    },
    RudderT {
        control: controlinterface::RudderT,
        callback: controlinterface::RudderTValueCallback<'a>,
    },
    RudderTrim {
        control: controlinterface::RudderTrim,
        callback: IntegerCallback<'a>,
    },
    RwrAct {
        control: rwric::RwrAct,
        callback: rwric::RwrActValueCallback<'a>,
    },
    RwrAlt {
        control: rwric::RwrAlt,
        callback: rwric::RwrAltValueCallback<'a>,
    },
    RwrDim {
        control: rwric::RwrDim,
        callback: IntegerCallback<'a>,
    },
    RwrHand {
        control: rwric::RwrHand,
        callback: rwric::RwrHandValueCallback<'a>,
    },
    RwrInt {
        control: analr87::RwrInt,
        callback: IntegerCallback<'a>,
    },
    RwrLaunch {
        control: rwric::RwrLaunch,
        callback: rwric::RwrLaunchValueCallback<'a>,
    },
    RwrMode {
        control: rwric::RwrMode,
        callback: rwric::RwrModeValueCallback<'a>,
    },
    RwrPwr {
        control: rwric::RwrPwr,
        callback: rwric::RwrPwrValueCallback<'a>,
    },
    RwrSearch {
        control: rwric::RwrSearch,
        callback: rwric::RwrSearchValueCallback<'a>,
    },
    RwrShip {
        control: rwric::RwrShip,
        callback: rwric::RwrShipValueCallback<'a>,
    },
    RwrT {
        control: rwric::RwrT,
        callback: rwric::RwrTValueCallback<'a>,
    },
    RwrTest {
        control: rwric::RwrTest,
        callback: rwric::RwrTestValueCallback<'a>,
    },
    RwrVol {
        control: rwric::RwrVol,
        callback: IntegerCallback<'a>,
    },
    SaiArrow {
        control: gauges::SaiArrow,
        callback: IntegerCallback<'a>,
    },
    SaiBank {
        control: gauges::SaiBank,
        callback: IntegerCallback<'a>,
    },
    SaiCage {
        control: standbyattitudeindicator::SaiCage,
        callback: standbyattitudeindicator::SaiCageValueCallback<'a>,
    },
    SaiOff {
        control: gauges::SaiOff,
        callback: IntegerCallback<'a>,
    },
    SaiPitch {
        control: gauges::SaiPitch,
        callback: IntegerCallback<'a>,
    },
    SaiPitchTrim {
        control: standbyattitudeindicator::SaiPitchTrim,
        callback: IntegerCallback<'a>,
    },
    SbyCompassBank {
        control: gauges::SbyCompassBank,
        callback: IntegerCallback<'a>,
    },
    SbyCompassHdg {
        control: gauges::SbyCompassHdg,
        callback: IntegerCallback<'a>,
    },
    SbyCompassPitch {
        control: gauges::SbyCompassPitch,
        callback: IntegerCallback<'a>,
    },
    SightBit {
        control: sight::SightBit,
        callback: sight::SightBitValueCallback<'a>,
    },
    SightCage {
        control: sight::SightCage,
        callback: sight::SightCageValueCallback<'a>,
    },
    SightDep1 {
        control: gauges::SightDep1,
        callback: IntegerCallback<'a>,
    },
    SightDep10 {
        control: gauges::SightDep10,
        callback: IntegerCallback<'a>,
    },
    SightDep100 {
        control: gauges::SightDep100,
        callback: IntegerCallback<'a>,
    },
    SightDepress {
        control: sight::SightDepress,
        callback: IntegerCallback<'a>,
    },
    SightIntens {
        control: sight::SightIntens,
        callback: IntegerCallback<'a>,
    },
    SightMode {
        control: sight::SightMode,
        callback: sight::SightModeValueCallback<'a>,
    },
    Slipball {
        control: gauges::Slipball,
        callback: IntegerCallback<'a>,
    },
    Spare1 {
        control: warningcautionandindicatorlights::Spare1,
        callback: LedCallback<'a>,
    },
    Spare2 {
        control: warningcautionandindicatorlights::Spare2,
        callback: LedCallback<'a>,
    },
    Spare3 {
        control: warningcautionandindicatorlights::Spare3,
        callback: LedCallback<'a>,
    },
    Speed {
        control: controlinterface::Speed,
        callback: controlinterface::SpeedValueCallback<'a>,
    },
    SwBattery {
        control: electric::SwBattery,
        callback: electric::SwBatteryValueCallback<'a>,
    },
    Tacan1 {
        control: tacanpanel::Tacan1,
        callback: IntegerCallback<'a>,
    },
    Tacan10 {
        control: tacanpanel::Tacan10,
        callback: IntegerCallback<'a>,
    },
    TacanChannel {
        control: tacanpanel::TacanChannel,
        callback: DisplayCallback<'a>,
    },
    TacanHsi {
        control: tacanpanel::TacanHsi,
        callback: tacanpanel::TacanHsiValueCallback<'a>,
    },
    TacanMode {
        control: tacanpanel::TacanMode,
        callback: tacanpanel::TacanModeValueCallback<'a>,
    },
    TacanTest {
        control: tacanpanel::TacanTest,
        callback: LedCallback<'a>,
    },
    TacanVol {
        control: tacanpanel::TacanVol,
        callback: IntegerCallback<'a>,
    },
    TacanXy {
        control: tacanpanel::TacanXy,
        callback: tacanpanel::TacanXyValueCallback<'a>,
    },
    TachoL {
        control: gauges::TachoL,
        callback: IntegerCallback<'a>,
    },
    TachoPercL {
        control: gauges::TachoPercL,
        callback: IntegerCallback<'a>,
    },
    TachoPercR {
        control: gauges::TachoPercR,
        callback: IntegerCallback<'a>,
    },
    TachoR {
        control: gauges::TachoR,
        callback: IntegerCallback<'a>,
    },
    TdcAzimuth {
        control: gauges::TdcAzimuth,
        callback: IntegerCallback<'a>,
    },
    TdcRange {
        control: gauges::TdcRange,
        callback: IntegerCallback<'a>,
    },
    TrimIndicator {
        control: gauges::TrimIndicator,
        callback: IntegerCallback<'a>,
    },
    Uhf0025 {
        control: gauges::Uhf0025,
        callback: IntegerCallback<'a>,
    },
    Uhf0025mhzSel {
        control: uhfradio::Uhf0025mhzSel,
        callback: uhfradio::Uhf0025mhzSelValueCallback<'a>,
    },
    Uhf01 {
        control: gauges::Uhf01,
        callback: IntegerCallback<'a>,
    },
    Uhf01mhzSel {
        control: uhfradio::Uhf01mhzSel,
        callback: uhfradio::Uhf01mhzSelValueCallback<'a>,
    },
    Uhf1 {
        control: gauges::Uhf1,
        callback: IntegerCallback<'a>,
    },
    Uhf10 {
        control: gauges::Uhf10,
        callback: IntegerCallback<'a>,
    },
    Uhf100 {
        control: gauges::Uhf100,
        callback: IntegerCallback<'a>,
    },
    Uhf100mhzSel {
        control: uhfradio::Uhf100mhzSel,
        callback: uhfradio::Uhf100mhzSelValueCallback<'a>,
    },
    Uhf10mhzSel {
        control: uhfradio::Uhf10mhzSel,
        callback: uhfradio::Uhf10mhzSelValueCallback<'a>,
    },
    Uhf1mhzSel {
        control: uhfradio::Uhf1mhzSel,
        callback: uhfradio::Uhf1mhzSelValueCallback<'a>,
    },
    UhfAnt {
        control: uhfradio::UhfAnt,
        callback: uhfradio::UhfAntValueCallback<'a>,
    },
    UhfDoor {
        control: uhfradio::UhfDoor,
        callback: uhfradio::UhfDoorValueCallback<'a>,
    },
    UhfFreq {
        control: uhfradio::UhfFreq,
        callback: uhfradio::UhfFreqValueCallback<'a>,
    },
    UhfFunc {
        control: uhfradio::UhfFunc,
        callback: uhfradio::UhfFuncValueCallback<'a>,
    },
    UhfMicBt {
        control: uhfradio::UhfMicBt,
        callback: uhfradio::UhfMicBtValueCallback<'a>,
    },
    UhfPreSet {
        control: uhfradio::UhfPreSet,
        callback: uhfradio::UhfPreSetValueCallback<'a>,
    },
    UhfPresetChan {
        control: gauges::UhfPresetChan,
        callback: IntegerCallback<'a>,
    },
    UhfPresetSelInt {
        control: uhfradio::UhfPresetSelInt,
        callback: IntegerCallback<'a>,
    },
    UhfPresetSelStr {
        control: uhfradio::UhfPresetSelStr,
        callback: DisplayCallback<'a>,
    },
    UhfSquelchSw {
        control: uhfradio::UhfSquelchSw,
        callback: uhfradio::UhfSquelchSwValueCallback<'a>,
    },
    UhfToneBt {
        control: uhfradio::UhfToneBt,
        callback: uhfradio::UhfToneBtValueCallback<'a>,
    },
    UhfVol {
        control: uhfradio::UhfVol,
        callback: IntegerCallback<'a>,
    },
    UtilHydraulic {
        control: warningcautionandindicatorlights::UtilHydraulic,
        callback: LedCallback<'a>,
    },
    UtilPress {
        control: gauges::UtilPress,
        callback: IntegerCallback<'a>,
    },
    Variometer {
        control: gauges::Variometer,
        callback: IntegerCallback<'a>,
    },
    WarnTSw {
        control: internallights::WarnTSw,
        callback: internallights::WarnTSwValueCallback<'a>,
    },
    WeaponRelease {
        control: weaponscontrol::WeaponRelease,
        callback: weaponscontrol::WeaponReleaseValueCallback<'a>,
    },
    YawDamper {
        control: controlinterface::YawDamper,
        callback: controlinterface::YawDamperValueCallback<'a>,
    },
}

impl RegisteredControl<'_> {
    fn accept(&mut self, packet: Packet<'_>) {
        match self {
            Self::AFlaps { control, .. } => control.accept(packet),
            Self::Acc { control, .. } => control.accept(packet),
            Self::Accelerometer { control, .. } => control.accept(packet),
            Self::AccelerometerMax { control, .. } => control.accept(packet),
            Self::AccelerometerMin { control, .. } => control.accept(packet),
            Self::AhrsErect { control, .. } => control.accept(packet),
            Self::AhrsFastSlave { control, .. } => control.accept(packet),
            Self::AhrsNavMode { control, .. } => control.accept(packet),
            Self::AiBank { control, .. } => control.accept(packet),
            Self::AiOffFlag { control, .. } => control.accept(packet),
            Self::AiPitch { control, .. } => control.accept(packet),
            Self::AiPitchTrim { control, .. } => control.accept(packet),
            Self::AirDataComputer { control, .. } => control.accept(packet),
            Self::Airspeed { control, .. } => control.accept(packet),
            Self::AirspeedMax { control, .. } => control.accept(packet),
            Self::AirspeedSet { control, .. } => control.accept(packet),
            Self::Alt10000C { control, .. } => control.accept(packet),
            Self::Alt1000C { control, .. } => control.accept(packet),
            Self::Alt100C { control, .. } => control.accept(packet),
            Self::Alt100P { control, .. } => control.accept(packet),
            Self::AltElect { control, .. } => control.accept(packet),
            Self::AltPneuFlag { control, .. } => control.accept(packet),
            Self::AltZero { control, .. } => control.accept(packet),
            Self::AoaFlaps { control, .. } => control.accept(packet),
            Self::AoaG { control, .. } => control.accept(packet),
            Self::AoaGauge { control, .. } => control.accept(packet),
            Self::AoaR { control, .. } => control.accept(packet),
            Self::AoaY { control, .. } => control.accept(packet),
            Self::ArmLights { control, .. } => control.accept(packet),
            Self::Armpos1 { control, .. } => control.accept(packet),
            Self::Armpos2 { control, .. } => control.accept(packet),
            Self::Armpos3 { control, .. } => control.accept(packet),
            Self::Armpos4 { control, .. } => control.accept(packet),
            Self::Armpos5 { control, .. } => control.accept(packet),
            Self::Armpos6 { control, .. } => control.accept(packet),
            Self::Armpos7 { control, .. } => control.accept(packet),
            Self::Autobal { control, .. } => control.accept(packet),
            Self::AuxintDoor { control, .. } => control.accept(packet),
            Self::Beacon { control, .. } => control.accept(packet),
            Self::BombsArmSw { control, .. } => control.accept(packet),
            Self::BriDimSw { control, .. } => control.accept(packet),
            Self::CabinDefog { control, .. } => control.accept(packet),
            Self::CabinPC { control, .. } => control.accept(packet),
            Self::CabinPSw { control, .. } => control.accept(packet),
            Self::CabinPress { control, .. } => control.accept(packet),
            Self::CabinTemp { control, .. } => control.accept(packet),
            Self::CabinTempSw { control, .. } => control.accept(packet),
            Self::CameraFps { control, .. } => control.accept(packet),
            Self::CameraLens { control, .. } => control.accept(packet),
            Self::CameraOverrun { control, .. } => control.accept(packet),
            Self::CameraRun { control, .. } => control.accept(packet),
            Self::Canopy { control, .. } => control.accept(packet),
            Self::CanopyJettison { control, .. } => control.accept(packet),
            Self::CanopyLever { control, .. } => control.accept(packet),
            Self::CanopyPos { control, .. } => control.accept(packet),
            Self::ChaffCount { control, .. } => control.accept(packet),
            Self::ChaffCount1 { control, .. } => control.accept(packet),
            Self::ChaffCount10 { control, .. } => control.accept(packet),
            Self::ChaffMode { control, .. } => control.accept(packet),
            Self::ClockCurrH { control, .. } => control.accept(packet),
            Self::ClockCurrM { control, .. } => control.accept(packet),
            Self::ClockElapM { control, .. } => control.accept(packet),
            Self::ClockElapS { control, .. } => control.accept(packet),
            Self::ClockSet { control, .. } => control.accept(packet),
            Self::ClockWind { control, .. } => control.accept(packet),
            Self::ClockWindRotatry { control, .. } => control.accept(packet),
            Self::CmChaffcntDisplay { control, .. } => control.accept(packet),
            Self::CmFlarecntDisplay { control, .. } => control.accept(packet),
            Self::CompasLSw { control, .. } => control.accept(packet),
            Self::ConsoleLights { control, .. } => control.accept(packet),
            Self::Crossfeed { control, .. } => control.accept(packet),
            Self::DcOverload { control, .. } => control.accept(packet),
            Self::DiluterLever { control, .. } => control.accept(packet),
            Self::DirGyro { control, .. } => control.accept(packet),
            Self::DragChute { control, .. } => control.accept(packet),
            Self::ElapTime { control, .. } => control.accept(packet),
            Self::EmerJettB { control, .. } => control.accept(packet),
            Self::EmerJettCover { control, .. } => control.accept(packet),
            Self::EmerL { control, .. } => control.accept(packet),
            Self::EngineAntiIce { control, .. } => control.accept(packet),
            Self::EngineLights { control, .. } => control.accept(packet),
            Self::ExhaustTempL { control, .. } => control.accept(packet),
            Self::ExhaustTempR { control, .. } => control.accept(packet),
            Self::ExtBrakeCute { control, .. } => control.accept(packet),
            Self::ExtCenterSw { control, .. } => control.accept(packet),
            Self::ExtPositionLights { control, .. } => control.accept(packet),
            Self::ExtPylonSw { control, .. } => control.accept(packet),
            Self::ExtSpeedBrakeLeft { control, .. } => control.accept(packet),
            Self::ExtSpeedBrakeRight { control, .. } => control.accept(packet),
            Self::ExtStore { control, .. } => control.accept(packet),
            Self::ExtStrobeLights { control, .. } => control.accept(packet),
            Self::ExtTanks { control, .. } => control.accept(packet),
            Self::ExtWowLeft { control, .. } => control.accept(packet),
            Self::ExtWowNose { control, .. } => control.accept(packet),
            Self::ExtWowRight { control, .. } => control.accept(packet),
            Self::FO2Sw { control, .. } => control.accept(packet),
            Self::FlChaffBt { control, .. } => control.accept(packet),
            Self::FlCount { control, .. } => control.accept(packet),
            Self::FlCount1 { control, .. } => control.accept(packet),
            Self::FlCount10 { control, .. } => control.accept(packet),
            Self::FlJettCover { control, .. } => control.accept(packet),
            Self::FlJettSw { control, .. } => control.accept(packet),
            Self::FlapIndicator { control, .. } => control.accept(packet),
            Self::Flaps { control, .. } => control.accept(packet),
            Self::FlareModeSel { control, .. } => control.accept(packet),
            Self::FlightHydraulic { control, .. } => control.accept(packet),
            Self::FlightLights { control, .. } => control.accept(packet),
            Self::FlightPress { control, .. } => control.accept(packet),
            Self::FloodLights { control, .. } => control.accept(packet),
            Self::FormationLights { control, .. } => control.accept(packet),
            Self::FuelflowL { control, .. } => control.accept(packet),
            Self::FuelflowR { control, .. } => control.accept(packet),
            Self::FuelquantityL { control, .. } => control.accept(packet),
            Self::FuelquantityR { control, .. } => control.accept(packet),
            Self::GwLight { control, .. } => control.accept(packet),
            Self::Hook { control, .. } => control.accept(packet),
            Self::HookLight { control, .. } => control.accept(packet),
            Self::HsiBerPoint { control, .. } => control.accept(packet),
            Self::HsiComp { control, .. } => control.accept(packet),
            Self::HsiCrs { control, .. } => control.accept(packet),
            Self::HsiCrs1 { control, .. } => control.accept(packet),
            Self::HsiCrs10010 { control, .. } => control.accept(packet),
            Self::HsiCrsDev { control, .. } => control.accept(packet),
            Self::HsiCrsKnob { control, .. } => control.accept(packet),
            Self::HsiDev { control, .. } => control.accept(packet),
            Self::HsiHdg { control, .. } => control.accept(packet),
            Self::HsiHdgKnob { control, .. } => control.accept(packet),
            Self::HsiOff { control, .. } => control.accept(packet),
            Self::HsiRange1 { control, .. } => control.accept(packet),
            Self::HsiRange10 { control, .. } => control.accept(packet),
            Self::HsiRange100 { control, .. } => control.accept(packet),
            Self::HsiRangeFlag { control, .. } => control.accept(packet),
            Self::HsiToFrom { control, .. } => control.accept(packet),
            Self::IasSet { control, .. } => control.accept(packet),
            Self::Iff { control, .. } => control.accept(packet),
            Self::Iff4CodeSel { control, .. } => control.accept(packet),
            Self::Iff4CodeSelPull { control, .. } => control.accept(packet),
            Self::Iff4Control { control, .. } => control.accept(packet),
            Self::Iff4Ident { control, .. } => control.accept(packet),
            Self::Iff4M1 { control, .. } => control.accept(packet),
            Self::Iff4M2 { control, .. } => control.accept(packet),
            Self::Iff4M3 { control, .. } => control.accept(packet),
            Self::Iff4Master { control, .. } => control.accept(packet),
            Self::Iff4MasterPull { control, .. } => control.accept(packet),
            Self::Iff4Mc { control, .. } => control.accept(packet),
            Self::Iff4MonContr { control, .. } => control.accept(packet),
            Self::Iff4Rad { control, .. } => control.accept(packet),
            Self::Iff4RadTestL { control, .. } => control.accept(packet),
            Self::Iff4ReplyL { control, .. } => control.accept(packet),
            Self::IffReplyL { control, .. } => control.accept(packet),
            Self::IffTestL { control, .. } => control.accept(packet),
            Self::InletHeater { control, .. } => control.accept(packet),
            Self::Ins { control, .. } => control.accept(packet),
            Self::IntSw { control, .. } => control.accept(packet),
            Self::JettB { control, .. } => control.accept(packet),
            Self::JettSw { control, .. } => control.accept(packet),
            Self::LBoostpump { control, .. } => control.accept(packet),
            Self::LFire { control, .. } => control.accept(packet),
            Self::LFuelPress { control, .. } => control.accept(packet),
            Self::LGen { control, .. } => control.accept(packet),
            Self::LGenerator { control, .. } => control.accept(packet),
            Self::LShutoffC { control, .. } => control.accept(packet),
            Self::LShutoffS { control, .. } => control.accept(packet),
            Self::LStart { control, .. } => control.accept(packet),
            Self::LeftFuelLow { control, .. } => control.accept(packet),
            Self::LeftLight { control, .. } => control.accept(packet),
            Self::LgAltRel { control, .. } => control.accept(packet),
            Self::LgAltRes { control, .. } => control.accept(packet),
            Self::LgDownL { control, .. } => control.accept(packet),
            Self::LgLeverSwitch { control, .. } => control.accept(packet),
            Self::LgLight { control, .. } => control.accept(packet),
            Self::LgTest { control, .. } => control.accept(packet),
            Self::LgfSilence { control, .. } => control.accept(packet),
            Self::Mach { control, .. } => control.accept(packet),
            Self::MasterArm { control, .. } => control.accept(packet),
            Self::MasterArmGuard { control, .. } => control.accept(packet),
            Self::McLight { control, .. } => control.accept(packet),
            Self::McResetBtn { control, .. } => control.accept(packet),
            Self::MissileUncage { control, .. } => control.accept(packet),
            Self::MissileVol { control, .. } => control.accept(packet),
            Self::MotorRun { control, .. } => control.accept(packet),
            Self::NavLights { control, .. } => control.accept(packet),
            Self::NgTest { control, .. } => control.accept(packet),
            Self::NoseLight { control, .. } => control.accept(packet),
            Self::NozzleposL { control, .. } => control.accept(packet),
            Self::NozzleposR { control, .. } => control.accept(packet),
            Self::NsStruct { control, .. } => control.accept(packet),
            Self::Nws { control, .. } => control.accept(packet),
            Self::O2Lever { control, .. } => control.accept(packet),
            Self::O2flowindicator { control, .. } => control.accept(packet),
            Self::O2flowpressure { control, .. } => control.accept(packet),
            Self::O2quantity { control, .. } => control.accept(packet),
            Self::OilpressL { control, .. } => control.accept(packet),
            Self::OilpressR { control, .. } => control.accept(packet),
            Self::Oxygen { control, .. } => control.accept(packet),
            Self::PanelLSw { control, .. } => control.accept(packet),
            Self::PitchDamper { control, .. } => control.accept(packet),
            Self::PitchDamperCut { control, .. } => control.accept(packet),
            Self::PitotHeater { control, .. } => control.accept(packet),
            Self::Press0 { control, .. } => control.accept(packet),
            Self::Press1 { control, .. } => control.accept(packet),
            Self::Press2 { control, .. } => control.accept(packet),
            Self::Press3 { control, .. } => control.accept(packet),
            Self::RBoostpump { control, .. } => control.accept(packet),
            Self::RFire { control, .. } => control.accept(packet),
            Self::RFuelPress { control, .. } => control.accept(packet),
            Self::RGen { control, .. } => control.accept(packet),
            Self::RGenerator { control, .. } => control.accept(packet),
            Self::RShutoffC { control, .. } => control.accept(packet),
            Self::RShutoffS { control, .. } => control.accept(packet),
            Self::RStart { control, .. } => control.accept(packet),
            Self::RadarAcquis { control, .. } => control.accept(packet),
            Self::RadarBright { control, .. } => control.accept(packet),
            Self::RadarCursor { control, .. } => control.accept(packet),
            Self::RadarElevation { control, .. } => control.accept(packet),
            Self::RadarExcess { control, .. } => control.accept(packet),
            Self::RadarFail { control, .. } => control.accept(packet),
            Self::RadarInrange { control, .. } => control.accept(packet),
            Self::RadarLockon { control, .. } => control.accept(packet),
            Self::RadarMode { control, .. } => control.accept(packet),
            Self::RadarPersis { control, .. } => control.accept(packet),
            Self::RadarPitch { control, .. } => control.accept(packet),
            Self::RadarRange { control, .. } => control.accept(packet),
            Self::RadarScale { control, .. } => control.accept(packet),
            Self::RadarScale10 { control, .. } => control.accept(packet),
            Self::RadarScale20 { control, .. } => control.accept(packet),
            Self::RadarScale40 { control, .. } => control.accept(packet),
            Self::RadarScale5 { control, .. } => control.accept(packet),
            Self::RadarScaleBright { control, .. } => control.accept(packet),
            Self::RadarVideo { control, .. } => control.accept(packet),
            Self::RgTest { control, .. } => control.accept(packet),
            Self::RightFuelLow { control, .. } => control.accept(packet),
            Self::RightLight { control, .. } => control.accept(packet),
            Self::RudderT { control, .. } => control.accept(packet),
            Self::RudderTrim { control, .. } => control.accept(packet),
            Self::RwrAct { control, .. } => control.accept(packet),
            Self::RwrAlt { control, .. } => control.accept(packet),
            Self::RwrDim { control, .. } => control.accept(packet),
            Self::RwrHand { control, .. } => control.accept(packet),
            Self::RwrInt { control, .. } => control.accept(packet),
            Self::RwrLaunch { control, .. } => control.accept(packet),
            Self::RwrMode { control, .. } => control.accept(packet),
            Self::RwrPwr { control, .. } => control.accept(packet),
            Self::RwrSearch { control, .. } => control.accept(packet),
            Self::RwrShip { control, .. } => control.accept(packet),
            Self::RwrT { control, .. } => control.accept(packet),
            Self::RwrTest { control, .. } => control.accept(packet),
            Self::RwrVol { control, .. } => control.accept(packet),
            Self::SaiArrow { control, .. } => control.accept(packet),
            Self::SaiBank { control, .. } => control.accept(packet),
            Self::SaiCage { control, .. } => control.accept(packet),
            Self::SaiOff { control, .. } => control.accept(packet),
            Self::SaiPitch { control, .. } => control.accept(packet),
            Self::SaiPitchTrim { control, .. } => control.accept(packet),
            Self::SbyCompassBank { control, .. } => control.accept(packet),
            Self::SbyCompassHdg { control, .. } => control.accept(packet),
            Self::SbyCompassPitch { control, .. } => control.accept(packet),
            Self::SightBit { control, .. } => control.accept(packet),
            Self::SightCage { control, .. } => control.accept(packet),
            Self::SightDep1 { control, .. } => control.accept(packet),
            Self::SightDep10 { control, .. } => control.accept(packet),
            Self::SightDep100 { control, .. } => control.accept(packet),
            Self::SightDepress { control, .. } => control.accept(packet),
            Self::SightIntens { control, .. } => control.accept(packet),
            Self::SightMode { control, .. } => control.accept(packet),
            Self::Slipball { control, .. } => control.accept(packet),
            Self::Spare1 { control, .. } => control.accept(packet),
            Self::Spare2 { control, .. } => control.accept(packet),
            Self::Spare3 { control, .. } => control.accept(packet),
            Self::Speed { control, .. } => control.accept(packet),
            Self::SwBattery { control, .. } => control.accept(packet),
            Self::Tacan1 { control, .. } => control.accept(packet),
            Self::Tacan10 { control, .. } => control.accept(packet),
            Self::TacanChannel { control, .. } => control.accept(packet),
            Self::TacanHsi { control, .. } => control.accept(packet),
            Self::TacanMode { control, .. } => control.accept(packet),
            Self::TacanTest { control, .. } => control.accept(packet),
            Self::TacanVol { control, .. } => control.accept(packet),
            Self::TacanXy { control, .. } => control.accept(packet),
            Self::TachoL { control, .. } => control.accept(packet),
            Self::TachoPercL { control, .. } => control.accept(packet),
            Self::TachoPercR { control, .. } => control.accept(packet),
            Self::TachoR { control, .. } => control.accept(packet),
            Self::TdcAzimuth { control, .. } => control.accept(packet),
            Self::TdcRange { control, .. } => control.accept(packet),
            Self::TrimIndicator { control, .. } => control.accept(packet),
            Self::Uhf0025 { control, .. } => control.accept(packet),
            Self::Uhf0025mhzSel { control, .. } => control.accept(packet),
            Self::Uhf01 { control, .. } => control.accept(packet),
            Self::Uhf01mhzSel { control, .. } => control.accept(packet),
            Self::Uhf1 { control, .. } => control.accept(packet),
            Self::Uhf10 { control, .. } => control.accept(packet),
            Self::Uhf100 { control, .. } => control.accept(packet),
            Self::Uhf100mhzSel { control, .. } => control.accept(packet),
            Self::Uhf10mhzSel { control, .. } => control.accept(packet),
            Self::Uhf1mhzSel { control, .. } => control.accept(packet),
            Self::UhfAnt { control, .. } => control.accept(packet),
            Self::UhfDoor { control, .. } => control.accept(packet),
            Self::UhfFreq { control, .. } => control.accept(packet),
            Self::UhfFunc { control, .. } => control.accept(packet),
            Self::UhfMicBt { control, .. } => control.accept(packet),
            Self::UhfPreSet { control, .. } => control.accept(packet),
            Self::UhfPresetChan { control, .. } => control.accept(packet),
            Self::UhfPresetSelInt { control, .. } => control.accept(packet),
            Self::UhfPresetSelStr { control, .. } => control.accept(packet),
            Self::UhfSquelchSw { control, .. } => control.accept(packet),
            Self::UhfToneBt { control, .. } => control.accept(packet),
            Self::UhfVol { control, .. } => control.accept(packet),
            Self::UtilHydraulic { control, .. } => control.accept(packet),
            Self::UtilPress { control, .. } => control.accept(packet),
            Self::Variometer { control, .. } => control.accept(packet),
            Self::WarnTSw { control, .. } => control.accept(packet),
            Self::WeaponRelease { control, .. } => control.accept(packet),
            Self::YawDamper { control, .. } => control.accept(packet),
        }
    }

    fn commit(&mut self) {
        match self {
            Self::AFlaps { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <controlinterface::AFlapsValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Acc { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <accelerometer::AccValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Accelerometer { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AccelerometerMax { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AccelerometerMin { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AhrsErect { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ahrs::AhrsErectValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AhrsFastSlave { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ahrs::AhrsFastSlaveValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AhrsNavMode { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ahrs::AhrsNavModeValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AiBank { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AiOffFlag { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AiPitch { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AiPitchTrim { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AirDataComputer { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Airspeed { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AirspeedMax { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AirspeedSet { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Alt10000C { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Alt1000C { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Alt100C { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Alt100P { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AltElect { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <altimeter::AltElectValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AltPneuFlag { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AltZero { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AoaFlaps { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::AoaG { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::AoaGauge { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AoaR { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::AoaY { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ArmLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Armpos1 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <weaponscontrol::Armpos1Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Armpos2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <weaponscontrol::Armpos2Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Armpos3 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <weaponscontrol::Armpos3Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Armpos4 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <weaponscontrol::Armpos4Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Armpos5 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <weaponscontrol::Armpos5Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Armpos6 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <weaponscontrol::Armpos6Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Armpos7 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <weaponscontrol::Armpos7Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Autobal { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fuel::AutobalValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::AuxintDoor { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Beacon { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <externallights::BeaconValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::BombsArmSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <weaponscontrol::BombsArmSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::BriDimSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <internallights::BriDimSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CabinDefog { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CabinPC { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ecinterface::CabinPCValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CabinPSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ecinterface::CabinPSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CabinPress { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CabinTemp { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CabinTempSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <ecinterface::CabinTempSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CameraFps { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <camera::CameraFpsValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CameraLens { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CameraOverrun { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <camera::CameraOverrunValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CameraRun { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <camera::CameraRunValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Canopy { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::CanopyJettison { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <cockpit::CanopyJettisonValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CanopyLever { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <cockpit::CanopyLeverValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CanopyPos { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ChaffCount { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <cmds::ChaffCountValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::ChaffCount1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ChaffCount10 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ChaffMode { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <cmds::ChaffModeValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::ClockCurrH { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ClockCurrM { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ClockElapM { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ClockElapS { control, callback } => {
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
                    if let Ok(value) =
                        <clock::ClockWindValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::ClockWindRotatry { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CmChaffcntDisplay { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CmFlarecntDisplay { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CompasLSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <internallights::CompasLSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::ConsoleLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Crossfeed { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <fuel::CrossfeedValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::DcOverload { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::DiluterLever { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <o2interface::DiluterLeverValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::DirGyro { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::DragChute { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <cockpit::DragChuteValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::ElapTime { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <clock::ElapTimeValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::EmerJettB { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <jettison::EmerJettBValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::EmerJettCover { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <jettison::EmerJettCoverValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::EmerL { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <o2interface::EmerLValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::EngineAntiIce { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::EngineLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ExhaustTempL { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ExhaustTempR { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ExtBrakeCute { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ExtCenterSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <fuel::ExtCenterSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::ExtPositionLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ExtPylonSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <fuel::ExtPylonSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
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
            Self::ExtStore { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <weaponscontrol::ExtStoreValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::ExtStrobeLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ExtTanks { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
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
            Self::FO2Sw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <electric::FO2SwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::FlChaffBt { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <cmds::FlChaffBtValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::FlCount { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <cmds::FlCountValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::FlCount1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FlCount10 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FlJettCover { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <cmds::FlJettCoverValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::FlJettSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <cmds::FlJettSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::FlapIndicator { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Flaps { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <controlinterface::FlapsValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FlareModeSel { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <cmds::FlareModeSelValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::FlightHydraulic { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::FlightLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FlightPress { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FloodLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FormationLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelflowL { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelflowR { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelquantityL { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelquantityR { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::GwLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Hook { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <gearinterface::HookValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::HookLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::HsiBerPoint { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HsiComp { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HsiCrs { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HsiCrs1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HsiCrs10010 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HsiCrsDev { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HsiCrsKnob { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HsiDev { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HsiHdg { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HsiHdgKnob { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HsiOff { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HsiRange1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HsiRange10 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HsiRange100 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HsiRangeFlag { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::HsiToFrom { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IasSet { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Iff { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Iff4CodeSel { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::Iff4CodeSelValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Iff4CodeSelPull { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::Iff4CodeSelPullValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Iff4Control { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::Iff4ControlValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Iff4Ident { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::Iff4IdentValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Iff4M1 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::Iff4M1Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Iff4M2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::Iff4M2Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Iff4M3 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::Iff4M3Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Iff4Master { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::Iff4MasterValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Iff4MasterPull { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::Iff4MasterPullValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Iff4Mc { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::Iff4McValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Iff4MonContr { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::Iff4MonContrValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Iff4Rad { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <iff::Iff4RadValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::Iff4RadTestL { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::Iff4RadTestLValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Iff4ReplyL { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::Iff4ReplyLValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IffReplyL { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::IffTestL { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::InletHeater { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <engineinterface::InletHeaterValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Ins { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::IntSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <weaponscontrol::IntSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::JettB { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <jettison::JettBValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::JettSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <jettison::JettSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LBoostpump { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <fuel::LBoostpumpValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LFire { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LFuelPress { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LGen { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LGenerator { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <electric::LGeneratorValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LShutoffC { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <fuel::LShutoffCValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LShutoffS { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <fuel::LShutoffSValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LStart { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <engineinterface::LStartValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::LeftFuelLow { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LeftLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LgAltRel { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <gearinterface::LgAltRelValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::LgAltRes { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <gearinterface::LgAltResValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::LgDownL { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <gearinterface::LgDownLValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::LgLeverSwitch { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <gearinterface::LgLeverSwitchValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LgLight { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <externallights::LgLightValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::LgTest { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <gearinterface::LgTestValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LgfSilence { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <gearinterface::LgfSilenceValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Mach { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MasterArm { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <weaponscontrol::MasterArmValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MasterArmGuard { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <weaponscontrol::MasterArmGuardValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::McLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::McResetBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <internallights::McResetBtnValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MissileUncage { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <weaponscontrol::MissileUncageValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::MissileVol { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MotorRun { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::NavLights { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::NgTest { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <gearinterface::NgTestValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::NoseLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::NozzleposL { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::NozzleposR { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::NsStruct { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <gearinterface::NsStructValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Nws { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <gearinterface::NwsValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::O2Lever { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <o2interface::O2LeverValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::O2flowindicator { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::O2flowpressure { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::O2quantity { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::OilpressL { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::OilpressR { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Oxygen { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::PanelLSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <internallights::PanelLSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::PitchDamper { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <controlinterface::PitchDamperValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::PitchDamperCut { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <controlinterface::PitchDamperCutValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::PitotHeater { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <electric::PitotHeaterValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Press0 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Press1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Press2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Press3 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RBoostpump { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <fuel::RBoostpumpValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RFire { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RFuelPress { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RGen { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RGenerator { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <electric::RGeneratorValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RShutoffC { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <fuel::RShutoffCValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RShutoffS { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <fuel::RShutoffSValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RStart { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <engineinterface::RStartValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RadarAcquis { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <radar::RadarAcquisValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RadarBright { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RadarCursor { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RadarElevation { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RadarExcess { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RadarFail { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RadarInrange { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RadarLockon { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RadarMode { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <radar::RadarModeValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RadarPersis { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RadarPitch { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RadarRange { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <radar::RadarRangeValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RadarScale { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RadarScale10 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RadarScale20 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RadarScale40 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RadarScale5 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RadarScaleBright { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RadarVideo { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RgTest { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <gearinterface::RgTestValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RightFuelLow { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RightLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RudderT { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <controlinterface::RudderTValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RudderTrim { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RwrAct { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rwric::RwrActValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::RwrAlt { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rwric::RwrAltValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::RwrDim { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RwrHand { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <rwric::RwrHandValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RwrInt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RwrLaunch { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <rwric::RwrLaunchValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RwrMode { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <rwric::RwrModeValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RwrPwr { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rwric::RwrPwrValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::RwrSearch { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <rwric::RwrSearchValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RwrShip { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <rwric::RwrShipValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RwrT { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <rwric::RwrTValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RwrTest { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <rwric::RwrTestValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RwrVol { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SaiArrow { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SaiBank { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SaiCage { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <standbyattitudeindicator::SaiCageValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::SaiOff { control, callback } => {
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
            Self::SbyCompassBank { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SbyCompassHdg { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SbyCompassPitch { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SightBit { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <sight::SightBitValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::SightCage { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <sight::SightCageValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::SightDep1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SightDep10 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SightDep100 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SightDepress { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SightIntens { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SightMode { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <sight::SightModeValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Slipball { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Spare1 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Spare2 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Spare3 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Speed { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <controlinterface::SpeedValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::SwBattery { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <electric::SwBatteryValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Tacan1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Tacan10 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TacanChannel { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TacanHsi { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <tacanpanel::TacanHsiValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::TacanMode { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <tacanpanel::TacanModeValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::TacanTest { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::TacanVol { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TacanXy { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <tacanpanel::TacanXyValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::TachoL { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TachoPercL { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TachoPercR { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TachoR { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TdcAzimuth { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TdcRange { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TrimIndicator { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Uhf0025 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Uhf0025mhzSel { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <uhfradio::Uhf0025mhzSelValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Uhf01 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Uhf01mhzSel { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <uhfradio::Uhf01mhzSelValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Uhf1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Uhf10 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Uhf100 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Uhf100mhzSel { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <uhfradio::Uhf100mhzSelValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Uhf10mhzSel { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <uhfradio::Uhf10mhzSelValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Uhf1mhzSel { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <uhfradio::Uhf1mhzSelValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfAnt { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <uhfradio::UhfAntValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfDoor { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <uhfradio::UhfDoorValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfFreq { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <uhfradio::UhfFreqValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfFunc { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <uhfradio::UhfFuncValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfMicBt { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <uhfradio::UhfMicBtValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfPreSet { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <uhfradio::UhfPreSetValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfPresetChan { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UhfPresetSelInt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UhfPresetSelStr { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UhfSquelchSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <uhfradio::UhfSquelchSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfToneBt { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <uhfradio::UhfToneBtValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfVol { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UtilHydraulic { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::UtilPress { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Variometer { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::WarnTSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <internallights::WarnTSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::WeaponRelease { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <weaponscontrol::WeaponReleaseValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::YawDamper { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <controlinterface::YawDamperValue as core::convert::TryFrom<u16>>::try_from(
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

pub struct F5e3Builder<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> {
    controls: Vec<RegisteredControl<'a>, MAX_CONTROLS>,
    _packet_capacity: PhantomData<ProtocolParser<MAX_PACKET_DATA>>,
}

impl<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize>
    F5e3Builder<'a, MAX_CONTROLS, MAX_PACKET_DATA>
{
    pub fn new() -> Self {
        Self {
            controls: Vec::new(),
            _packet_capacity: PhantomData,
        }
    }

    pub fn register_a_flaps<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(controlinterface::AFlapsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AFlaps {
                control: controlinterface::AFlaps::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_acc<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(accelerometer::AccValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Acc {
                control: accelerometer::Acc::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_accelerometer<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Accelerometer {
                control: gauges::Accelerometer::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_accelerometer_max<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AccelerometerMax {
                control: gauges::AccelerometerMax::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_accelerometer_min<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AccelerometerMin {
                control: gauges::AccelerometerMin::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ahrs_erect<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ahrs::AhrsErectValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AhrsErect {
                control: ahrs::AhrsErect::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ahrs_fast_slave<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ahrs::AhrsFastSlaveValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AhrsFastSlave {
                control: ahrs::AhrsFastSlave::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ahrs_nav_mode<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ahrs::AhrsNavModeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AhrsNavMode {
                control: ahrs::AhrsNavMode::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ai_bank<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AiBank {
                control: gauges::AiBank::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ai_off_flag<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AiOffFlag {
                control: gauges::AiOffFlag::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ai_pitch<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AiPitch {
                control: gauges::AiPitch::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ai_pitch_trim<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AiPitchTrim {
                control: attitudeindicator::AiPitchTrim::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_air_data_computer<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::AirDataComputer {
                control: warningcautionandindicatorlights::AirDataComputer::new(),
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
                control: gauges::Airspeed::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_airspeed_max<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AirspeedMax {
                control: gauges::AirspeedMax::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_airspeed_set<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AirspeedSet {
                control: gauges::AirspeedSet::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt10000_c<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Alt10000C {
                control: gauges::Alt10000C::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt1000_c<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Alt1000C {
                control: gauges::Alt1000C::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt100_c<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Alt100C {
                control: gauges::Alt100C::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt100_p<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Alt100P {
                control: gauges::Alt100P::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_elect<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(altimeter::AltElectValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltElect {
                control: altimeter::AltElect::new(),
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
                control: gauges::AltPneuFlag::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_zero<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltZero {
                control: altimeter::AltZero::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_aoa_flaps<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::AoaFlaps {
                control: warningcautionandindicatorlights::AoaFlaps::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_aoa_g<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::AoaG {
                control: warningcautionandindicatorlights::AoaG::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_aoa_gauge<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AoaGauge {
                control: gauges::AoaGauge::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_aoa_r<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::AoaR {
                control: warningcautionandindicatorlights::AoaR::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_aoa_y<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::AoaY {
                control: warningcautionandindicatorlights::AoaY::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_arm_lights<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ArmLights {
                control: internallights::ArmLights::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_armpos1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(weaponscontrol::Armpos1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::Armpos1 {
                control: weaponscontrol::Armpos1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_armpos2<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(weaponscontrol::Armpos2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::Armpos2 {
                control: weaponscontrol::Armpos2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_armpos3<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(weaponscontrol::Armpos3Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::Armpos3 {
                control: weaponscontrol::Armpos3::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_armpos4<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(weaponscontrol::Armpos4Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::Armpos4 {
                control: weaponscontrol::Armpos4::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_armpos5<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(weaponscontrol::Armpos5Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::Armpos5 {
                control: weaponscontrol::Armpos5::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_armpos6<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(weaponscontrol::Armpos6Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::Armpos6 {
                control: weaponscontrol::Armpos6::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_armpos7<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(weaponscontrol::Armpos7Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::Armpos7 {
                control: weaponscontrol::Armpos7::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_autobal<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fuel::AutobalValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Autobal {
                control: fuel::Autobal::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_auxint_door<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AuxintDoor {
                control: gauges::AuxintDoor::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_beacon<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(externallights::BeaconValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Beacon {
                control: externallights::Beacon::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_bombs_arm_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(weaponscontrol::BombsArmSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::BombsArmSw {
                control: weaponscontrol::BombsArmSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_bri_dim_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(internallights::BriDimSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::BriDimSw {
                control: internallights::BriDimSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cabin_defog<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CabinDefog {
                control: ecinterface::CabinDefog::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cabin_p_c<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ecinterface::CabinPCValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CabinPC {
                control: ecinterface::CabinPC::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cabin_p_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ecinterface::CabinPSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CabinPSw {
                control: ecinterface::CabinPSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cabin_press<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CabinPress {
                control: gauges::CabinPress::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cabin_temp<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CabinTemp {
                control: ecinterface::CabinTemp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cabin_temp_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ecinterface::CabinTempSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CabinTempSw {
                control: ecinterface::CabinTempSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_camera_fps<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(camera::CameraFpsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CameraFps {
                control: camera::CameraFps::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_camera_lens<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CameraLens {
                control: camera::CameraLens::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_camera_overrun<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(camera::CameraOverrunValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CameraOverrun {
                control: camera::CameraOverrun::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_camera_run<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(camera::CameraRunValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CameraRun {
                control: camera::CameraRun::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_canopy<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Canopy {
                control: warningcautionandindicatorlights::Canopy::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_canopy_jettison<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(cockpit::CanopyJettisonValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CanopyJettison {
                control: cockpit::CanopyJettison::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_canopy_lever<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(cockpit::CanopyLeverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CanopyLever {
                control: cockpit::CanopyLever::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_canopy_pos<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CanopyPos {
                control: gauges::CanopyPos::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_chaff_count<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(cmds::ChaffCountValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ChaffCount {
                control: cmds::ChaffCount::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_chaff_count1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ChaffCount1 {
                control: gauges::ChaffCount1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_chaff_count10<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ChaffCount10 {
                control: gauges::ChaffCount10::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_chaff_mode<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(cmds::ChaffModeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ChaffMode {
                control: cmds::ChaffMode::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_curr_h<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockCurrH {
                control: gauges::ClockCurrH::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_curr_m<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockCurrM {
                control: gauges::ClockCurrM::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_elap_m<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockElapM {
                control: gauges::ClockElapM::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_elap_s<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockElapS {
                control: gauges::ClockElapS::new(),
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
        F: FnMut(clock::ClockWindValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockWind {
                control: clock::ClockWind::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_wind_rotatry<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockWindRotatry {
                control: clock::ClockWindRotatry::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cm_chaffcnt_display<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmChaffcntDisplay {
                control: cmds::CmChaffcntDisplay::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cm_flarecnt_display<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmFlarecntDisplay {
                control: cmds::CmFlarecntDisplay::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_compas_l_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(internallights::CompasLSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CompasLSw {
                control: internallights::CompasLSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_console_lights<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ConsoleLights {
                control: internallights::ConsoleLights::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_crossfeed<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fuel::CrossfeedValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Crossfeed {
                control: fuel::Crossfeed::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dc_overload<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::DcOverload {
                control: warningcautionandindicatorlights::DcOverload::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_diluter_lever<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(o2interface::DiluterLeverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::DiluterLever {
                control: o2interface::DiluterLever::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dir_gyro<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::DirGyro {
                control: warningcautionandindicatorlights::DirGyro::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_drag_chute<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(cockpit::DragChuteValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::DragChute {
                control: cockpit::DragChute::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_elap_time<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(clock::ElapTimeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ElapTime {
                control: clock::ElapTime::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_emer_jett_b<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(jettison::EmerJettBValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EmerJettB {
                control: jettison::EmerJettB::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_emer_jett_cover<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(jettison::EmerJettCoverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EmerJettCover {
                control: jettison::EmerJettCover::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_emer_l<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(o2interface::EmerLValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EmerL {
                control: o2interface::EmerL::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_engine_anti_ice<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngineAntiIce {
                control: warningcautionandindicatorlights::EngineAntiIce::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_engine_lights<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngineLights {
                control: internallights::EngineLights::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_exhaust_temp_l<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExhaustTempL {
                control: gauges::ExhaustTempL::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_exhaust_temp_r<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExhaustTempR {
                control: gauges::ExhaustTempR::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_brake_cute<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtBrakeCute {
                control: externalaircraftmodel::ExtBrakeCute::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_center_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fuel::ExtCenterSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtCenterSw {
                control: fuel::ExtCenterSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_position_lights<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtPositionLights {
                control: externalaircraftmodel::ExtPositionLights::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_pylon_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fuel::ExtPylonSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtPylonSw {
                control: fuel::ExtPylonSw::new(),
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

    pub fn register_ext_store<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(weaponscontrol::ExtStoreValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtStore {
                control: weaponscontrol::ExtStore::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_strobe_lights<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtStrobeLights {
                control: externalaircraftmodel::ExtStrobeLights::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_tanks<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtTanks {
                control: warningcautionandindicatorlights::ExtTanks::new(),
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

    pub fn register_f_o2_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electric::FO2SwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FO2Sw {
                control: electric::FO2Sw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fl_chaff_bt<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(cmds::FlChaffBtValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlChaffBt {
                control: cmds::FlChaffBt::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fl_count<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(cmds::FlCountValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlCount {
                control: cmds::FlCount::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fl_count1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlCount1 {
                control: gauges::FlCount1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fl_count10<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlCount10 {
                control: gauges::FlCount10::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fl_jett_cover<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(cmds::FlJettCoverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlJettCover {
                control: cmds::FlJettCover::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fl_jett_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(cmds::FlJettSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlJettSw {
                control: cmds::FlJettSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flap_indicator<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlapIndicator {
                control: gauges::FlapIndicator::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flaps<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(controlinterface::FlapsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Flaps {
                control: controlinterface::Flaps::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flare_mode_sel<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(cmds::FlareModeSelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlareModeSel {
                control: cmds::FlareModeSel::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flight_hydraulic<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlightHydraulic {
                control: warningcautionandindicatorlights::FlightHydraulic::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flight_lights<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlightLights {
                control: internallights::FlightLights::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flight_press<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlightPress {
                control: gauges::FlightPress::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flood_lights<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FloodLights {
                control: internallights::FloodLights::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_formation_lights<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FormationLights {
                control: externallights::FormationLights::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuelflow_l<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelflowL {
                control: gauges::FuelflowL::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuelflow_r<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelflowR {
                control: gauges::FuelflowR::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuelquantity_l<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelquantityL {
                control: gauges::FuelquantityL::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuelquantity_r<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelquantityR {
                control: gauges::FuelquantityR::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gw_light<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::GwLight {
                control: gearinterface::GwLight::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hook<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(gearinterface::HookValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Hook {
                control: gearinterface::Hook::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hook_light<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::HookLight {
                control: gearinterface::HookLight::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hsi_ber_point<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HsiBerPoint {
                control: gauges::HsiBerPoint::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hsi_comp<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HsiComp {
                control: gauges::HsiComp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hsi_crs<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HsiCrs {
                control: gauges::HsiCrs::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hsi_crs1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HsiCrs1 {
                control: gauges::HsiCrs1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hsi_crs10010<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HsiCrs10010 {
                control: gauges::HsiCrs10010::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hsi_crs_dev<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HsiCrsDev {
                control: gauges::HsiCrsDev::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hsi_crs_knob<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HsiCrsKnob {
                control: hsi::HsiCrsKnob::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hsi_dev<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HsiDev {
                control: gauges::HsiDev::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hsi_hdg<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HsiHdg {
                control: gauges::HsiHdg::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hsi_hdg_knob<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HsiHdgKnob {
                control: hsi::HsiHdgKnob::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hsi_off<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HsiOff {
                control: gauges::HsiOff::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hsi_range1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HsiRange1 {
                control: gauges::HsiRange1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hsi_range10<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HsiRange10 {
                control: gauges::HsiRange10::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hsi_range100<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HsiRange100 {
                control: gauges::HsiRange100::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hsi_range_flag<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::HsiRangeFlag {
                control: warningcautionandindicatorlights::HsiRangeFlag::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hsi_to_from<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HsiToFrom {
                control: gauges::HsiToFrom::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ias_set<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IasSet {
                control: airspeedindicator::IasSet::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Iff {
                control: warningcautionandindicatorlights::Iff::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff4_code_sel<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::Iff4CodeSelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Iff4CodeSel {
                control: iff::Iff4CodeSel::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff4_code_sel_pull<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::Iff4CodeSelPullValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Iff4CodeSelPull {
                control: iff::Iff4CodeSelPull::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff4_control<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::Iff4ControlValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Iff4Control {
                control: iff::Iff4Control::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff4_ident<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::Iff4IdentValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Iff4Ident {
                control: iff::Iff4Ident::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff4_m1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::Iff4M1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::Iff4M1 {
                control: iff::Iff4M1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff4_m2<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::Iff4M2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::Iff4M2 {
                control: iff::Iff4M2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff4_m3<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::Iff4M3Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::Iff4M3 {
                control: iff::Iff4M3::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff4_master<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::Iff4MasterValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Iff4Master {
                control: iff::Iff4Master::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff4_master_pull<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::Iff4MasterPullValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Iff4MasterPull {
                control: iff::Iff4MasterPull::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff4_mc<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::Iff4McValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Iff4Mc {
                control: iff::Iff4Mc::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff4_mon_contr<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::Iff4MonContrValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Iff4MonContr {
                control: iff::Iff4MonContr::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff4_rad<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::Iff4RadValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Iff4Rad {
                control: iff::Iff4Rad::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff4_rad_test_l<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::Iff4RadTestLValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Iff4RadTestL {
                control: iff::Iff4RadTestL::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff4_reply_l<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::Iff4ReplyLValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Iff4ReplyL {
                control: iff::Iff4ReplyL::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_reply_l<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffReplyL {
                control: iff::IffReplyL::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_test_l<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffTestL {
                control: iff::IffTestL::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_inlet_heater<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(engineinterface::InletHeaterValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::InletHeater {
                control: engineinterface::InletHeater::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ins<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ins {
                control: warningcautionandindicatorlights::Ins::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_int_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(weaponscontrol::IntSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IntSw {
                control: weaponscontrol::IntSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_jett_b<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(jettison::JettBValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::JettB {
                control: jettison::JettB::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_jett_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(jettison::JettSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::JettSw {
                control: jettison::JettSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_l_boostpump<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fuel::LBoostpumpValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LBoostpump {
                control: fuel::LBoostpump::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_l_fire<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LFire {
                control: warningcautionandindicatorlights::LFire::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_l_fuel_press<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LFuelPress {
                control: warningcautionandindicatorlights::LFuelPress::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_l_gen<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LGen {
                control: warningcautionandindicatorlights::LGen::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_l_generator<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electric::LGeneratorValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LGenerator {
                control: electric::LGenerator::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_l_shutoff_c<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fuel::LShutoffCValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LShutoffC {
                control: fuel::LShutoffC::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_l_shutoff_s<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fuel::LShutoffSValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LShutoffS {
                control: fuel::LShutoffS::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_l_start<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(engineinterface::LStartValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LStart {
                control: engineinterface::LStart::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_fuel_low<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftFuelLow {
                control: warningcautionandindicatorlights::LeftFuelLow::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_light<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftLight {
                control: gearinterface::LeftLight::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lg_alt_rel<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(gearinterface::LgAltRelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LgAltRel {
                control: gearinterface::LgAltRel::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lg_alt_res<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(gearinterface::LgAltResValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LgAltRes {
                control: gearinterface::LgAltRes::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lg_down_l<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(gearinterface::LgDownLValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LgDownL {
                control: gearinterface::LgDownL::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lg_lever_switch<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(gearinterface::LgLeverSwitchValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LgLeverSwitch {
                control: gearinterface::LgLeverSwitch::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lg_light<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(externallights::LgLightValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LgLight {
                control: externallights::LgLight::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lg_test<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(gearinterface::LgTestValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LgTest {
                control: gearinterface::LgTest::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lgf_silence<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(gearinterface::LgfSilenceValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LgfSilence {
                control: gearinterface::LgfSilence::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mach<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Mach {
                control: gauges::Mach::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_master_arm<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(weaponscontrol::MasterArmValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MasterArm {
                control: weaponscontrol::MasterArm::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_master_arm_guard<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(weaponscontrol::MasterArmGuardValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MasterArmGuard {
                control: weaponscontrol::MasterArmGuard::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mc_light<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::McLight {
                control: warningcautionandindicatorlights::McLight::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mc_reset_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(internallights::McResetBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::McResetBtn {
                control: internallights::McResetBtn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_missile_uncage<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(weaponscontrol::MissileUncageValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MissileUncage {
                control: weaponscontrol::MissileUncage::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_missile_vol<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MissileVol {
                control: weaponscontrol::MissileVol::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_motor_run<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MotorRun {
                control: gauges::MotorRun::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nav_lights<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::NavLights {
                control: externallights::NavLights::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ng_test<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(gearinterface::NgTestValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::NgTest {
                control: gearinterface::NgTest::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nose_light<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::NoseLight {
                control: gearinterface::NoseLight::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nozzlepos_l<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::NozzleposL {
                control: gauges::NozzleposL::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nozzlepos_r<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::NozzleposR {
                control: gauges::NozzleposR::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ns_struct<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(gearinterface::NsStructValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::NsStruct {
                control: gearinterface::NsStruct::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nws<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(gearinterface::NwsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Nws {
                control: gearinterface::Nws::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_o2_lever<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(o2interface::O2LeverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::O2Lever {
                control: o2interface::O2Lever::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_o2flowindicator<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::O2flowindicator {
                control: gauges::O2flowindicator::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_o2flowpressure<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::O2flowpressure {
                control: gauges::O2flowpressure::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_o2quantity<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::O2quantity {
                control: gauges::O2quantity::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oilpress_l<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OilpressL {
                control: gauges::OilpressL::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oilpress_r<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OilpressR {
                control: gauges::OilpressR::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oxygen<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Oxygen {
                control: warningcautionandindicatorlights::Oxygen::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_panel_l_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(internallights::PanelLSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::PanelLSw {
                control: internallights::PanelLSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pitch_damper<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(controlinterface::PitchDamperValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::PitchDamper {
                control: controlinterface::PitchDamper::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pitch_damper_cut<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(controlinterface::PitchDamperCutValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::PitchDamperCut {
                control: controlinterface::PitchDamperCut::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pitot_heater<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electric::PitotHeaterValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::PitotHeater {
                control: electric::PitotHeater::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_press0<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Press0 {
                control: gauges::Press0::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_press1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Press1 {
                control: gauges::Press1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_press2<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Press2 {
                control: gauges::Press2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_press3<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Press3 {
                control: gauges::Press3::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_r_boostpump<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fuel::RBoostpumpValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RBoostpump {
                control: fuel::RBoostpump::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_r_fire<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RFire {
                control: warningcautionandindicatorlights::RFire::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_r_fuel_press<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RFuelPress {
                control: warningcautionandindicatorlights::RFuelPress::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_r_gen<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RGen {
                control: warningcautionandindicatorlights::RGen::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_r_generator<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electric::RGeneratorValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RGenerator {
                control: electric::RGenerator::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_r_shutoff_c<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fuel::RShutoffCValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RShutoffC {
                control: fuel::RShutoffC::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_r_shutoff_s<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fuel::RShutoffSValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RShutoffS {
                control: fuel::RShutoffS::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_r_start<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(engineinterface::RStartValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RStart {
                control: engineinterface::RStart::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_acquis<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(radar::RadarAcquisValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarAcquis {
                control: radar::RadarAcquis::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_bright<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarBright {
                control: radar::RadarBright::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_cursor<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarCursor {
                control: radar::RadarCursor::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_elevation<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarElevation {
                control: radar::RadarElevation::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_excess<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarExcess {
                control: warningcautionandindicatorlights::RadarExcess::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_fail<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarFail {
                control: warningcautionandindicatorlights::RadarFail::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_inrange<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarInrange {
                control: warningcautionandindicatorlights::RadarInrange::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_lockon<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarLockon {
                control: warningcautionandindicatorlights::RadarLockon::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_mode<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(radar::RadarModeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarMode {
                control: radar::RadarMode::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_persis<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarPersis {
                control: radar::RadarPersis::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_pitch<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarPitch {
                control: radar::RadarPitch::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_range<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(radar::RadarRangeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarRange {
                control: radar::RadarRange::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_scale<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarScale {
                control: radar::RadarScale::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_scale10<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarScale10 {
                control: warningcautionandindicatorlights::RadarScale10::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_scale20<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarScale20 {
                control: warningcautionandindicatorlights::RadarScale20::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_scale40<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarScale40 {
                control: warningcautionandindicatorlights::RadarScale40::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_scale5<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarScale5 {
                control: warningcautionandindicatorlights::RadarScale5::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_scale_bright<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarScaleBright {
                control: warningcautionandindicatorlights::RadarScaleBright::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_video<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarVideo {
                control: radar::RadarVideo::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rg_test<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(gearinterface::RgTestValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RgTest {
                control: gearinterface::RgTest::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_fuel_low<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightFuelLow {
                control: warningcautionandindicatorlights::RightFuelLow::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_light<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightLight {
                control: gearinterface::RightLight::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rudder_t<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(controlinterface::RudderTValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RudderT {
                control: controlinterface::RudderT::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rudder_trim<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RudderTrim {
                control: controlinterface::RudderTrim::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_act<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rwric::RwrActValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrAct {
                control: rwric::RwrAct::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_alt<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rwric::RwrAltValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrAlt {
                control: rwric::RwrAlt::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_dim<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrDim {
                control: rwric::RwrDim::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_hand<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rwric::RwrHandValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrHand {
                control: rwric::RwrHand::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_int<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrInt {
                control: analr87::RwrInt::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_launch<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rwric::RwrLaunchValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrLaunch {
                control: rwric::RwrLaunch::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_mode<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rwric::RwrModeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrMode {
                control: rwric::RwrMode::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_pwr<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rwric::RwrPwrValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrPwr {
                control: rwric::RwrPwr::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_search<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rwric::RwrSearchValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrSearch {
                control: rwric::RwrSearch::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_ship<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rwric::RwrShipValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrShip {
                control: rwric::RwrShip::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_t<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rwric::RwrTValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrT {
                control: rwric::RwrT::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_test<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rwric::RwrTestValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrTest {
                control: rwric::RwrTest::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_vol<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrVol {
                control: rwric::RwrVol::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sai_arrow<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SaiArrow {
                control: gauges::SaiArrow::new(),
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
                control: gauges::SaiBank::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sai_cage<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(standbyattitudeindicator::SaiCageValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SaiCage {
                control: standbyattitudeindicator::SaiCage::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sai_off<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SaiOff {
                control: gauges::SaiOff::new(),
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
                control: gauges::SaiPitch::new(),
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
                control: standbyattitudeindicator::SaiPitchTrim::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sby_compass_bank<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SbyCompassBank {
                control: gauges::SbyCompassBank::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sby_compass_hdg<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SbyCompassHdg {
                control: gauges::SbyCompassHdg::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sby_compass_pitch<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SbyCompassPitch {
                control: gauges::SbyCompassPitch::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sight_bit<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(sight::SightBitValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SightBit {
                control: sight::SightBit::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sight_cage<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(sight::SightCageValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SightCage {
                control: sight::SightCage::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sight_dep1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SightDep1 {
                control: gauges::SightDep1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sight_dep10<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SightDep10 {
                control: gauges::SightDep10::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sight_dep100<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SightDep100 {
                control: gauges::SightDep100::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sight_depress<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SightDepress {
                control: sight::SightDepress::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sight_intens<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SightIntens {
                control: sight::SightIntens::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sight_mode<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(sight::SightModeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SightMode {
                control: sight::SightMode::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_slipball<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Slipball {
                control: gauges::Slipball::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_spare1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Spare1 {
                control: warningcautionandindicatorlights::Spare1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_spare2<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Spare2 {
                control: warningcautionandindicatorlights::Spare2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_spare3<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Spare3 {
                control: warningcautionandindicatorlights::Spare3::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_speed<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(controlinterface::SpeedValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Speed {
                control: controlinterface::Speed::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sw_battery<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electric::SwBatteryValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SwBattery {
                control: electric::SwBattery::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tacan1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Tacan1 {
                control: tacanpanel::Tacan1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tacan10<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Tacan10 {
                control: tacanpanel::Tacan10::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tacan_channel<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::TacanChannel {
                control: tacanpanel::TacanChannel::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tacan_hsi<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(tacanpanel::TacanHsiValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::TacanHsi {
                control: tacanpanel::TacanHsi::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tacan_mode<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(tacanpanel::TacanModeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::TacanMode {
                control: tacanpanel::TacanMode::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tacan_test<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::TacanTest {
                control: tacanpanel::TacanTest::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tacan_vol<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TacanVol {
                control: tacanpanel::TacanVol::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tacan_xy<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(tacanpanel::TacanXyValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::TacanXy {
                control: tacanpanel::TacanXy::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tacho_l<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TachoL {
                control: gauges::TachoL::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tacho_perc_l<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TachoPercL {
                control: gauges::TachoPercL::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tacho_perc_r<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TachoPercR {
                control: gauges::TachoPercR::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tacho_r<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TachoR {
                control: gauges::TachoR::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tdc_azimuth<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TdcAzimuth {
                control: gauges::TdcAzimuth::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tdc_range<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TdcRange {
                control: gauges::TdcRange::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_trim_indicator<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TrimIndicator {
                control: gauges::TrimIndicator::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf0025<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Uhf0025 {
                control: gauges::Uhf0025::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf0025mhz_sel<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhfradio::Uhf0025mhzSelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Uhf0025mhzSel {
                control: uhfradio::Uhf0025mhzSel::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf01<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Uhf01 {
                control: gauges::Uhf01::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf01mhz_sel<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhfradio::Uhf01mhzSelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Uhf01mhzSel {
                control: uhfradio::Uhf01mhzSel::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Uhf1 {
                control: gauges::Uhf1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf10<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Uhf10 {
                control: gauges::Uhf10::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf100<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Uhf100 {
                control: gauges::Uhf100::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf100mhz_sel<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhfradio::Uhf100mhzSelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Uhf100mhzSel {
                control: uhfradio::Uhf100mhzSel::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf10mhz_sel<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhfradio::Uhf10mhzSelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Uhf10mhzSel {
                control: uhfradio::Uhf10mhzSel::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf1mhz_sel<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhfradio::Uhf1mhzSelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Uhf1mhzSel {
                control: uhfradio::Uhf1mhzSel::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_ant<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhfradio::UhfAntValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfAnt {
                control: uhfradio::UhfAnt::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_door<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhfradio::UhfDoorValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfDoor {
                control: uhfradio::UhfDoor::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_freq<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhfradio::UhfFreqValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfFreq {
                control: uhfradio::UhfFreq::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_func<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhfradio::UhfFuncValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfFunc {
                control: uhfradio::UhfFunc::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_mic_bt<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhfradio::UhfMicBtValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfMicBt {
                control: uhfradio::UhfMicBt::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_pre_set<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhfradio::UhfPreSetValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfPreSet {
                control: uhfradio::UhfPreSet::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_preset_chan<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfPresetChan {
                control: gauges::UhfPresetChan::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_preset_sel_int<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfPresetSelInt {
                control: uhfradio::UhfPresetSelInt::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_preset_sel_str<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfPresetSelStr {
                control: uhfradio::UhfPresetSelStr::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_squelch_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhfradio::UhfSquelchSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfSquelchSw {
                control: uhfradio::UhfSquelchSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_tone_bt<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhfradio::UhfToneBtValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfToneBt {
                control: uhfradio::UhfToneBt::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_vol<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfVol {
                control: uhfradio::UhfVol::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_util_hydraulic<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::UtilHydraulic {
                control: warningcautionandindicatorlights::UtilHydraulic::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_util_press<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UtilPress {
                control: gauges::UtilPress::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_variometer<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Variometer {
                control: gauges::Variometer::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_warn_t_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(internallights::WarnTSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::WarnTSw {
                control: internallights::WarnTSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_weapon_release<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(weaponscontrol::WeaponReleaseValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::WeaponRelease {
                control: weaponscontrol::WeaponRelease::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_yaw_damper<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(controlinterface::YawDamperValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::YawDamper {
                control: controlinterface::YawDamper::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn build(self) -> F5e3<'a, MAX_CONTROLS, MAX_PACKET_DATA> {
        F5e3 {
            parser: ProtocolParser::new(),
            controls: self.controls,
            saw_frame: false,
        }
    }
}

impl<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> Default
    for F5e3Builder<'a, MAX_CONTROLS, MAX_PACKET_DATA>
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

pub struct F5e3<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> {
    parser: ProtocolParser<MAX_PACKET_DATA>,
    controls: Vec<RegisteredControl<'a>, MAX_CONTROLS>,
    saw_frame: bool,
}

impl<const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize>
    F5e3<'_, MAX_CONTROLS, MAX_PACKET_DATA>
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
