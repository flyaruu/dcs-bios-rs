//! Generated from model-definitions/SA342M.json.
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

pub mod adf {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Adf1Adf2SelectValue {
        Select1,
        Select2,
    }
    pub type Adf1Adf2SelectValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(Adf1Adf2SelectValue) + 'a>;
    impl core::convert::TryFrom<u16> for Adf1Adf2SelectValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ADF1/ADF2 Select
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Adf1Adf2Select(Selector<22574, 4096, 12>);
    impl Adf1Adf2Select {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Adf1Adf2Select {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Adf1Adf2Select {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADF Gain
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct AdfGain(AnalogDial<22586, 65535, 0>);
    impl AdfGain {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdfGain {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdfGain {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AdfModeValue {
        Select1,
        Select2,
        Select3,
        Select4,
    }
    pub type AdfModeValueCallback<'a> = alloc::boxed::Box<dyn FnMut(AdfModeValue) + 'a>;
    impl core::convert::TryFrom<u16> for AdfModeValue {
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

    /// ADF Mode
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct AdfMode(Selector<22574, 49152, 14>);
    impl AdfMode {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdfMode {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdfMode {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADF NAV1 1s
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct AdfNav11(AnalogDial<22592, 65535, 0>);
    impl AdfNav11 {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdfNav11 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdfNav11 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADF NAV1 10s
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct AdfNav110(AnalogDial<22590, 65535, 0>);
    impl AdfNav110 {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdfNav110 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdfNav110 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADF NAV1 100s
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct AdfNav1100(AnalogDial<22588, 65535, 0>);
    impl AdfNav1100 {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdfNav1100 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdfNav1100 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADF NAV2 1s
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct AdfNav21(AnalogDial<22598, 65535, 0>);
    impl AdfNav21 {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdfNav21 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdfNav21 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADF NAV2 10s
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct AdfNav210(AnalogDial<22596, 65535, 0>);
    impl AdfNav210 {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdfNav210 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdfNav210 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADF NAV2 100s
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct AdfNav2100(AnalogDial<22594, 65535, 0>);
    impl AdfNav2100 {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdfNav2100 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdfNav2100 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AdfToneOnOffValue {
        Select1,
        Select2,
    }
    pub type AdfToneOnOffValueCallback<'a> = alloc::boxed::Box<dyn FnMut(AdfToneOnOffValue) + 'a>;
    impl core::convert::TryFrom<u16> for AdfToneOnOffValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ADF Tone On/Off
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AdfToneOnOff(Selector<22574, 8192, 13>);
    impl AdfToneOnOff {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdfToneOnOff {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdfToneOnOff {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod amradio {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AmFrequencySelectorValue {
        Select1,
        Select2,
    }
    pub type AmFrequencySelectorValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(AmFrequencySelectorValue) + 'a>;
    impl core::convert::TryFrom<u16> for AmFrequencySelectorValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// AM Radio 25/50kHz Selector
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AmFrequencySelector(Selector<22530, 128, 7>);
    impl AmFrequencySelector {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AmFrequencySelector {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AmFrequencySelector {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// AM Radio Frequency Dial Left
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct AmRadioFrequencyDialLeft(AnalogDial<22546, 65535, 0>);
    impl AmRadioFrequencyDialLeft {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AmRadioFrequencyDialLeft {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AmRadioFrequencyDialLeft {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// AM Radio Frequency Dial Right
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct AmRadioFrequencyDialRight(AnalogDial<22544, 65535, 0>);
    impl AmRadioFrequencyDialRight {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AmRadioFrequencyDialRight {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AmRadioFrequencyDialRight {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AmRadioSelectorValue {
        Select1,
        Select2,
        Select3,
        Select4,
    }
    pub type AmRadioSelectorValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(AmRadioSelectorValue) + 'a>;
    impl core::convert::TryFrom<u16> for AmRadioSelectorValue {
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

    /// AM Radio Selector
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct AmRadioSelector(Selector<22530, 96, 5>);
    impl AmRadioSelector {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AmRadioSelector {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AmRadioSelector {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod autopilot {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AutoHoverValue {
        Select1,
        Select2,
    }
    pub type AutoHoverValueCallback<'a> = alloc::boxed::Box<dyn FnMut(AutoHoverValue) + 'a>;
    impl core::convert::TryFrom<u16> for AutoHoverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Auto-Hover
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AutoHover(Selector<22530, 1024, 10>);
    impl AutoHover {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AutoHover {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AutoHover {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AutopilotButtonValue {
        Select1,
        Select2,
    }
    pub type AutopilotButtonValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(AutopilotButtonValue) + 'a>;
    impl core::convert::TryFrom<u16> for AutopilotButtonValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Autopilot Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AutopilotButton(Selector<22530, 256, 8>);
    impl AutopilotButton {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AutopilotButton {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AutopilotButton {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AutopilotModeValue {
        Select1,
        Select2,
        Select3,
    }
    pub type AutopilotModeValueCallback<'a> = alloc::boxed::Box<dyn FnMut(AutopilotModeValue) + 'a>;
    impl core::convert::TryFrom<u16> for AutopilotModeValue {
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

    /// Autopilot Mode Speed/Off/Altitude
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct AutopilotMode(Selector<22548, 192, 6>);
    impl AutopilotMode {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AutopilotMode {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AutopilotMode {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AutopilotOnOffValue {
        Select1,
        Select2,
    }
    pub type AutopilotOnOffValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(AutopilotOnOffValue) + 'a>;
    impl core::convert::TryFrom<u16> for AutopilotOnOffValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Autopilot On/Off
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AutopilotOnOff(Selector<22530, 32768, 15>);
    impl AutopilotOnOff {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AutopilotOnOff {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AutopilotOnOff {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AutopilotPitchValue {
        Select1,
        Select2,
    }
    pub type AutopilotPitchValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(AutopilotPitchValue) + 'a>;
    impl core::convert::TryFrom<u16> for AutopilotPitchValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Autopilot Pitch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AutopilotPitch(Selector<22548, 8, 3>);
    impl AutopilotPitch {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AutopilotPitch {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AutopilotPitch {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AutopilotRollValue {
        Select1,
        Select2,
    }
    pub type AutopilotRollValueCallback<'a> = alloc::boxed::Box<dyn FnMut(AutopilotRollValue) + 'a>;
    impl core::convert::TryFrom<u16> for AutopilotRollValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Autopilot Roll
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AutopilotRoll(Selector<22548, 16, 4>);
    impl AutopilotRoll {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AutopilotRoll {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AutopilotRoll {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AutopilotSlaveValue {
        Select1,
        Select2,
    }
    pub type AutopilotSlaveValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(AutopilotSlaveValue) + 'a>;
    impl core::convert::TryFrom<u16> for AutopilotSlaveValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Autopilot Slave
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AutopilotSlave(Selector<22530, 512, 9>);
    impl AutopilotSlave {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AutopilotSlave {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AutopilotSlave {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AutopilotTrimValue {
        Select1,
        Select2,
    }
    pub type AutopilotTrimValueCallback<'a> = alloc::boxed::Box<dyn FnMut(AutopilotTrimValue) + 'a>;
    impl core::convert::TryFrom<u16> for AutopilotTrimValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Autopilot Trim
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AutopilotTrim(Selector<22548, 256, 8>);
    impl AutopilotTrim {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AutopilotTrim {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AutopilotTrim {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AutopilotYawValue {
        Select1,
        Select2,
    }
    pub type AutopilotYawValueCallback<'a> = alloc::boxed::Box<dyn FnMut(AutopilotYawValue) + 'a>;
    impl core::convert::TryFrom<u16> for AutopilotYawValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Autopilot Yaw
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AutopilotYaw(Selector<22548, 32, 5>);
    impl AutopilotYaw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AutopilotYaw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AutopilotYaw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GyroKnobValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
    }
    pub type GyroKnobValueCallback<'a> = alloc::boxed::Box<dyn FnMut(GyroKnobValue) + 'a>;
    impl core::convert::TryFrom<u16> for GyroKnobValue {
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

    /// Gyro Knob
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct GyroKnob(Selector<22548, 7, 0>);
    impl GyroKnob {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for GyroKnob {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for GyroKnob {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GyroSwitchValue {
        Select1,
        Select2,
        Select3,
    }
    pub type GyroSwitchValueCallback<'a> = alloc::boxed::Box<dyn FnMut(GyroSwitchValue) + 'a>;
    impl core::convert::TryFrom<u16> for GyroSwitchValue {
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

    /// Gyro Left/Center/Right
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct GyroSwitch(Selector<22530, 24576, 13>);
    impl GyroSwitch {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for GyroSwitch {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for GyroSwitch {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GyroTestCoverValue {
        Select1,
        Select2,
    }
    pub type GyroTestCoverValueCallback<'a> = alloc::boxed::Box<dyn FnMut(GyroTestCoverValue) + 'a>;
    impl core::convert::TryFrom<u16> for GyroTestCoverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Gyro Test Cover Open/Close
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct GyroTestCover(Selector<22530, 2048, 11>);
    impl GyroTestCover {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for GyroTestCover {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for GyroTestCover {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GyroTestSwitchValue {
        Select1,
        Select2,
    }
    pub type GyroTestSwitchValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(GyroTestSwitchValue) + 'a>;
    impl core::convert::TryFrom<u16> for GyroTestSwitchValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Gyro Test Switch On/Off
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct GyroTestSwitch(Selector<22530, 4096, 12>);
    impl GyroTestSwitch {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for GyroTestSwitch {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for GyroTestSwitch {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MagneticBrakeValue {
        Select1,
        Select2,
    }
    pub type MagneticBrakeValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MagneticBrakeValue) + 'a>;
    impl core::convert::TryFrom<u16> for MagneticBrakeValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Magnetic Brake
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MagneticBrake(Selector<22548, 512, 9>);
    impl MagneticBrake {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MagneticBrake {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MagneticBrake {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod baroaltimeter {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Barometric Pressure Adjust
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct BarometricAltimeter(AnalogDial<22582, 65535, 0>);
    impl BarometricAltimeter {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for BarometricAltimeter {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for BarometricAltimeter {
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
    pub enum ClockPushButtonValue {
        Select1,
        Select2,
    }
    pub type ClockPushButtonValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(ClockPushButtonValue) + 'a>;
    impl core::convert::TryFrom<u16> for ClockPushButtonValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Clock Push Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ClockPushButton(Selector<22574, 512, 9>);
    impl ClockPushButton {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ClockPushButton {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClockPushButton {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ClockResetValue {
        Select1,
        Select2,
    }
    pub type ClockResetValueCallback<'a> = alloc::boxed::Box<dyn FnMut(ClockResetValue) + 'a>;
    impl core::convert::TryFrom<u16> for ClockResetValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Clock Reset
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ClockReset(Selector<22574, 2048, 11>);
    impl ClockReset {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ClockReset {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClockReset {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Clock Set
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct ClockSet(AnalogDial<22584, 65535, 0>);
    impl ClockSet {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
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
    pub enum ClockStartStopValue {
        Select1,
        Select2,
    }
    pub type ClockStartStopValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(ClockStartStopValue) + 'a>;
    impl core::convert::TryFrom<u16> for ClockStartStopValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Clock Start Stop
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ClockStartStop(Selector<22574, 1024, 10>);
    impl ClockStartStop {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ClockStartStop {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClockStartStop {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod controls {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MagneticBrakeButtonValue {
        Select1,
        Select2,
    }
    pub type MagneticBrakeButtonValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(MagneticBrakeButtonValue) + 'a>;
    impl core::convert::TryFrom<u16> for MagneticBrakeButtonValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Magnetic Brake Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MagneticBrakeButton(Selector<22548, 1024, 10>);
    impl MagneticBrakeButton {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MagneticBrakeButton {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MagneticBrakeButton {
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
    pub enum AdditionalFuelTankValue {
        Select1,
        Select2,
    }
    pub type AdditionalFuelTankValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(AdditionalFuelTankValue) + 'a>;
    impl core::convert::TryFrom<u16> for AdditionalFuelTankValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Additional Fuel Tank
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AdditionalFuelTank(Selector<22528, 128, 7>);
    impl AdditionalFuelTank {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdditionalFuelTank {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdditionalFuelTank {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AlternatorValue {
        Select1,
        Select2,
    }
    pub type AlternatorValueCallback<'a> = alloc::boxed::Box<dyn FnMut(AlternatorValue) + 'a>;
    impl core::convert::TryFrom<u16> for AlternatorValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Alternator
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Alternator(Selector<22528, 4, 2>);
    impl Alternator {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Alternator {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Alternator {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum BatteryValue {
        Select1,
        Select2,
    }
    pub type BatteryValueCallback<'a> = alloc::boxed::Box<dyn FnMut(BatteryValue) + 'a>;
    impl core::convert::TryFrom<u16> for BatteryValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Battery
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Battery(Selector<22528, 2, 1>);
    impl Battery {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Battery {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Battery {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ConvoyTankValue {
        Select1,
        Select2,
    }
    pub type ConvoyTankValueCallback<'a> = alloc::boxed::Box<dyn FnMut(ConvoyTankValue) + 'a>;
    impl core::convert::TryFrom<u16> for ConvoyTankValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Convoy Tank On/Off
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ConvoyTank(Selector<22530, 8, 3>);
    impl ConvoyTank {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ConvoyTank {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ConvoyTank {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CopilotWiperValue {
        Select1,
        Select2,
        Select3,
    }
    pub type CopilotWiperValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CopilotWiperValue) + 'a>;
    impl core::convert::TryFrom<u16> for CopilotWiperValue {
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

    /// Copilot Wiper
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct CopilotWiper(Selector<22528, 6144, 11>);
    impl CopilotWiper {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CopilotWiper {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CopilotWiper {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FuelPumpValue {
        Select1,
        Select2,
    }
    pub type FuelPumpValueCallback<'a> = alloc::boxed::Box<dyn FnMut(FuelPumpValue) + 'a>;
    impl core::convert::TryFrom<u16> for FuelPumpValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Fuel Pump
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FuelPump(Selector<22528, 64, 6>);
    impl FuelPump {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FuelPump {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FuelPump {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GeneratorValue {
        Select1,
        Select2,
    }
    pub type GeneratorValueCallback<'a> = alloc::boxed::Box<dyn FnMut(GeneratorValue) + 'a>;
    impl core::convert::TryFrom<u16> for GeneratorValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Generator
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Generator(Selector<22528, 8, 3>);
    impl Generator {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Generator {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Generator {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum HydTestValue {
        Select1,
        Select2,
    }
    pub type HydTestValueCallback<'a> = alloc::boxed::Box<dyn FnMut(HydTestValue) + 'a>;
    impl core::convert::TryFrom<u16> for HydTestValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// HYD Test
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct HydTest(Selector<22530, 1, 0>);
    impl HydTest {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HydTest {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HydTest {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum PilotWiperValue {
        Select1,
        Select2,
        Select3,
    }
    pub type PilotWiperValueCallback<'a> = alloc::boxed::Box<dyn FnMut(PilotWiperValue) + 'a>;
    impl core::convert::TryFrom<u16> for PilotWiperValue {
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

    /// Pilot Wiper
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct PilotWiper(Selector<22528, 24576, 13>);
    impl PilotWiper {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for PilotWiper {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for PilotWiper {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum PitotValue {
        Select1,
        Select2,
    }
    pub type PitotValueCallback<'a> = alloc::boxed::Box<dyn FnMut(PitotValue) + 'a>;
    impl core::convert::TryFrom<u16> for PitotValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pitot
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Pitot(Selector<22528, 32, 5>);
    impl Pitot {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Pitot {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Pitot {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RearmAlterValue {
        Select1,
        Select2,
    }
    pub type RearmAlterValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RearmAlterValue) + 'a>;
    impl core::convert::TryFrom<u16> for RearmAlterValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Alter Rearm
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RearmAlter(Selector<22530, 2, 1>);
    impl RearmAlter {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RearmAlter {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RearmAlter {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RearmGeneValue {
        Select1,
        Select2,
    }
    pub type RearmGeneValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RearmGeneValue) + 'a>;
    impl core::convert::TryFrom<u16> for RearmGeneValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Gene Rearm
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RearmGene(Selector<22530, 4, 2>);
    impl RearmGene {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RearmGene {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RearmGene {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SandFilterValue {
        Select1,
        Select2,
    }
    pub type SandFilterValueCallback<'a> = alloc::boxed::Box<dyn FnMut(SandFilterValue) + 'a>;
    impl core::convert::TryFrom<u16> for SandFilterValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Sand Filter On/Off
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct SandFilter(Selector<22530, 16, 4>);
    impl SandFilter {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SandFilter {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SandFilter {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum StarterValue {
        Select1,
        Select2,
        Select3,
    }
    pub type StarterValueCallback<'a> = alloc::boxed::Box<dyn FnMut(StarterValue) + 'a>;
    impl core::convert::TryFrom<u16> for StarterValue {
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

    /// Starter Start/Stop/Air
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct Starter(Selector<22528, 768, 8>);
    impl Starter {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Starter {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Starter {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UnassignedSwitchValue {
        Select1,
        Select2,
    }
    pub type UnassignedSwitchValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(UnassignedSwitchValue) + 'a>;
    impl core::convert::TryFrom<u16> for UnassignedSwitchValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Unassigned Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UnassignedSwitch(Selector<22528, 32768, 15>);
    impl UnassignedSwitch {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UnassignedSwitch {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UnassignedSwitch {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum VoltmeterTestValue {
        Select1,
        Select2,
    }
    pub type VoltmeterTestValueCallback<'a> = alloc::boxed::Box<dyn FnMut(VoltmeterTestValue) + 'a>;
    impl core::convert::TryFrom<u16> for VoltmeterTestValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Voltmeter Test
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct VoltmeterTest(Selector<22528, 16, 4>);
    impl VoltmeterTest {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for VoltmeterTest {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for VoltmeterTest {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum WarningPanelTestValue {
        Select1,
        Select2,
    }
    pub type WarningPanelTestValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(WarningPanelTestValue) + 'a>;
    impl core::convert::TryFrom<u16> for WarningPanelTestValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Warning Panel Test
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct WarningPanelTest(Selector<22528, 1024, 10>);
    impl WarningPanelTest {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for WarningPanelTest {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for WarningPanelTest {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum WiperOnceValue {
        Select1,
        Select2,
    }
    pub type WiperOnceValueCallback<'a> = alloc::boxed::Box<dyn FnMut(WiperOnceValue) + 'a>;
    impl core::convert::TryFrom<u16> for WiperOnceValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Wiper Once
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct WiperOnce(Selector<22528, 1, 0>);
    impl WiperOnce {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for WiperOnce {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for WiperOnce {
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

    /// Left Position Light (red)
    /// Output: Left Position Light (red)
    pub struct ExtPositionLightLeft(Metadata<22804, 1, 0>);
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
    pub struct ExtPositionLightRight(Metadata<22804, 2, 1>);
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

    /// Strobe Light
    /// Output: Strobe Light
    pub struct ExtStrobe(Metadata<22804, 4, 2>);
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
}

pub mod flaredispenser {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FlareDispFireCapValue {
        Select1,
        Select2,
    }
    pub type FlareDispFireCapValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(FlareDispFireCapValue) + 'a>;
    impl core::convert::TryFrom<u16> for FlareDispFireCapValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Flare Dispense Fire Button Cap
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FlareDispFireCap(Selector<22610, 8, 3>);
    impl FlareDispFireCap {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FlareDispFireCap {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FlareDispFireCap {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FlareDispSideSelectValue {
        Select1,
        Select2,
        Select3,
    }
    pub type FlareDispSideSelectValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(FlareDispSideSelectValue) + 'a>;
    impl core::convert::TryFrom<u16> for FlareDispSideSelectValue {
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

    /// Flare Dispenser Side Selector D/G+D/G
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct FlareDispSideSelect(Selector<22604, 49152, 14>);
    impl FlareDispSideSelect {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FlareDispSideSelect {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FlareDispSideSelect {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FlareDispenseButtonValue {
        Select1,
        Select2,
    }
    pub type FlareDispenseButtonValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(FlareDispenseButtonValue) + 'a>;
    impl core::convert::TryFrom<u16> for FlareDispenseButtonValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Flare Dispenser Fire Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FlareDispenseButton(Selector<22610, 16, 4>);
    impl FlareDispenseButton {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FlareDispenseButton {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FlareDispenseButton {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FlareDispenseModeValue {
        Select1,
        Select2,
    }
    pub type FlareDispenseModeValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(FlareDispenseModeValue) + 'a>;
    impl core::convert::TryFrom<u16> for FlareDispenseModeValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Flare Dispense Mode
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FlareDispenseMode(Selector<22610, 1, 0>);
    impl FlareDispenseMode {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FlareDispenseMode {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FlareDispenseMode {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FlareDispenserOffOnValue {
        Select1,
        Select2,
        Select3,
    }
    pub type FlareDispenserOffOnValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(FlareDispenserOffOnValue) + 'a>;
    impl core::convert::TryFrom<u16> for FlareDispenserOffOnValue {
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

    /// Flare Dispenser Off/On/Speed Select
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct FlareDispenserOffOn(Selector<22610, 6, 1>);
    impl FlareDispenserOffOn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FlareDispenserOffOn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FlareDispenserOffOn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod flightcontrols {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// HA Adjust
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct HaAdjust(AnalogDial<22550, 65535, 0>);
    impl HaAdjust {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HaAdjust {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HaAdjust {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum HaSourceValue {
        Select1,
        Select2,
        Select3,
        Select4,
    }
    pub type HaSourceValueCallback<'a> = alloc::boxed::Box<dyn FnMut(HaSourceValue) + 'a>;
    impl core::convert::TryFrom<u16> for HaSourceValue {
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

    /// HA Source
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct HaSource(Selector<22548, 24576, 13>);
    impl HaSource {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HaSource {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HaSource {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum HaUnlockValue {
        Select1,
        Select2,
    }
    pub type HaUnlockValueCallback<'a> = alloc::boxed::Box<dyn FnMut(HaUnlockValue) + 'a>;
    impl core::convert::TryFrom<u16> for HaUnlockValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// HA Unlock
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct HaUnlock(Selector<22548, 2048, 11>);
    impl HaUnlock {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HaUnlock {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HaUnlock {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Standby HA Adjust
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct StandbyHaAdjust(AnalogDial<22552, 65535, 0>);
    impl StandbyHaAdjust {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for StandbyHaAdjust {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for StandbyHaAdjust {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum StandbyHaUnlockValue {
        Select1,
        Select2,
    }
    pub type StandbyHaUnlockValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(StandbyHaUnlockValue) + 'a>;
    impl core::convert::TryFrom<u16> for StandbyHaUnlockValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Standby HA Unlock
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct StandbyHaUnlock(Selector<22548, 4096, 12>);
    impl StandbyHaUnlock {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for StandbyHaUnlock {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for StandbyHaUnlock {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod fmradio {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FmRadioButton0Value {
        Select1,
        Select2,
    }
    pub type FmRadioButton0ValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(FmRadioButton0Value) + 'a>;
    impl core::convert::TryFrom<u16> for FmRadioButton0Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// FM Radio Button 0
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FmRadioButton0(Selector<22614, 16384, 14>);
    impl FmRadioButton0 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FmRadioButton0 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FmRadioButton0 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FmRadioButton1Value {
        Select1,
        Select2,
    }
    pub type FmRadioButton1ValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(FmRadioButton1Value) + 'a>;
    impl core::convert::TryFrom<u16> for FmRadioButton1Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// FM Radio Button 1
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FmRadioButton1(Selector<22618, 32, 5>);
    impl FmRadioButton1 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FmRadioButton1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FmRadioButton1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FmRadioButton2Value {
        Select1,
        Select2,
    }
    pub type FmRadioButton2ValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(FmRadioButton2Value) + 'a>;
    impl core::convert::TryFrom<u16> for FmRadioButton2Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// FM Radio Button 2
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FmRadioButton2(Selector<22618, 64, 6>);
    impl FmRadioButton2 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FmRadioButton2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FmRadioButton2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FmRadioButton3Value {
        Select1,
        Select2,
    }
    pub type FmRadioButton3ValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(FmRadioButton3Value) + 'a>;
    impl core::convert::TryFrom<u16> for FmRadioButton3Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// FM Radio Button 3
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FmRadioButton3(Selector<22618, 128, 7>);
    impl FmRadioButton3 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FmRadioButton3 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FmRadioButton3 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FmRadioButton4Value {
        Select1,
        Select2,
    }
    pub type FmRadioButton4ValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(FmRadioButton4Value) + 'a>;
    impl core::convert::TryFrom<u16> for FmRadioButton4Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// FM Radio Button 4
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FmRadioButton4(Selector<22618, 1, 0>);
    impl FmRadioButton4 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FmRadioButton4 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FmRadioButton4 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FmRadioButton5Value {
        Select1,
        Select2,
    }
    pub type FmRadioButton5ValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(FmRadioButton5Value) + 'a>;
    impl core::convert::TryFrom<u16> for FmRadioButton5Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// FM Radio Button 5
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FmRadioButton5(Selector<22618, 2, 1>);
    impl FmRadioButton5 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FmRadioButton5 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FmRadioButton5 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FmRadioButton6Value {
        Select1,
        Select2,
    }
    pub type FmRadioButton6ValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(FmRadioButton6Value) + 'a>;
    impl core::convert::TryFrom<u16> for FmRadioButton6Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// FM Radio Button 6
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FmRadioButton6(Selector<22618, 4, 2>);
    impl FmRadioButton6 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FmRadioButton6 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FmRadioButton6 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FmRadioButton7Value {
        Select1,
        Select2,
    }
    pub type FmRadioButton7ValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(FmRadioButton7Value) + 'a>;
    impl core::convert::TryFrom<u16> for FmRadioButton7Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// FM Radio Button 7
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FmRadioButton7(Selector<22610, 16384, 14>);
    impl FmRadioButton7 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FmRadioButton7 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FmRadioButton7 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FmRadioButton8Value {
        Select1,
        Select2,
    }
    pub type FmRadioButton8ValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(FmRadioButton8Value) + 'a>;
    impl core::convert::TryFrom<u16> for FmRadioButton8Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// FM Radio Button 8
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FmRadioButton8(Selector<22610, 32768, 15>);
    impl FmRadioButton8 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FmRadioButton8 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FmRadioButton8 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FmRadioButton9Value {
        Select1,
        Select2,
    }
    pub type FmRadioButton9ValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(FmRadioButton9Value) + 'a>;
    impl core::convert::TryFrom<u16> for FmRadioButton9Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// FM Radio Button 9
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FmRadioButton9(Selector<22614, 8192, 13>);
    impl FmRadioButton9 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FmRadioButton9 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FmRadioButton9 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FmRadioButtonDownValue {
        Select1,
        Select2,
    }
    pub type FmRadioButtonDownValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(FmRadioButtonDownValue) + 'a>;
    impl core::convert::TryFrom<u16> for FmRadioButtonDownValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// FM Radio Button DOWN
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FmRadioButtonDown(Selector<22618, 512, 9>);
    impl FmRadioButtonDown {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FmRadioButtonDown {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FmRadioButtonDown {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FmRadioButtonRcValue {
        Select1,
        Select2,
    }
    pub type FmRadioButtonRcValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(FmRadioButtonRcValue) + 'a>;
    impl core::convert::TryFrom<u16> for FmRadioButtonRcValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// FM Radio Button RC
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FmRadioButtonRc(Selector<22618, 8, 3>);
    impl FmRadioButtonRc {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FmRadioButtonRc {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FmRadioButtonRc {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FmRadioButtonUpValue {
        Select1,
        Select2,
    }
    pub type FmRadioButtonUpValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(FmRadioButtonUpValue) + 'a>;
    impl core::convert::TryFrom<u16> for FmRadioButtonUpValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// FM Radio Button UP
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FmRadioButtonUp(Selector<22618, 16, 4>);
    impl FmRadioButtonUp {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FmRadioButtonUp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FmRadioButtonUp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FmRadioButtonValValue {
        Select1,
        Select2,
    }
    pub type FmRadioButtonValValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(FmRadioButtonValValue) + 'a>;
    impl core::convert::TryFrom<u16> for FmRadioButtonValValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// FM Radio Button VAL
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FmRadioButtonVal(Selector<22618, 256, 8>);
    impl FmRadioButtonVal {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FmRadioButtonVal {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FmRadioButtonVal {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FmRadioButtonXValue {
        Select1,
        Select2,
    }
    pub type FmRadioButtonXValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(FmRadioButtonXValue) + 'a>;
    impl core::convert::TryFrom<u16> for FmRadioButtonXValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// FM Radio Button X
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FmRadioButtonX(Selector<22614, 32768, 15>);
    impl FmRadioButtonX {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FmRadioButtonX {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FmRadioButtonX {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FmRadioChannelValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
        Select6,
        Select7,
        Select8,
    }
    pub type FmRadioChannelValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(FmRadioChannelValue) + 'a>;
    impl core::convert::TryFrom<u16> for FmRadioChannelValue {
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

    /// FM Radio Channel Select
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct FmRadioChannel(Selector<22614, 7168, 10>);
    impl FmRadioChannel {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FmRadioChannel {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FmRadioChannel {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FmRadioModeValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
    }
    pub type FmRadioModeValueCallback<'a> = alloc::boxed::Box<dyn FnMut(FmRadioModeValue) + 'a>;
    impl core::convert::TryFrom<u16> for FmRadioModeValue {
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

    /// FM Radio Mode Select
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct FmRadioMode(Selector<22614, 896, 7>);
    impl FmRadioMode {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FmRadioMode {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FmRadioMode {
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
    pub enum FuelFlowLeverValue {
        Select1,
        Select2,
    }
    pub type FuelFlowLeverValueCallback<'a> = alloc::boxed::Box<dyn FnMut(FuelFlowLeverValue) + 'a>;
    impl core::convert::TryFrom<u16> for FuelFlowLeverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Fuel Flow Lever
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FuelFlowLever(Selector<22548, 32768, 15>);
    impl FuelFlowLever {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FuelFlowLever {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FuelFlowLever {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod gaugevalues {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// ADF Distance Value
    /// Output: ADF Distance Value
    pub struct AdfDistanceValue(Metadata<22786, 65535, 0>);
    impl AdfDistanceValue {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdfDistanceValue {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdfDistanceValue {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADF Heading Value
    /// Output: ADF Heading Value
    pub struct AdfHeadingValue(Metadata<22784, 65535, 0>);
    impl AdfHeadingValue {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdfHeadingValue {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdfHeadingValue {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Barometric Altimeter Value
    /// Output: Barometric Altimeter Value
    pub struct BaroAltValue(Metadata<22788, 65535, 0>);
    impl BaroAltValue {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for BaroAltValue {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for BaroAltValue {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Barometric Pressure Value
    /// Output: Barometric Pressure Value
    pub struct BaroPressValue(Metadata<22790, 65535, 0>);
    impl BaroPressValue {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for BaroPressValue {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for BaroPressValue {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Fuel Level Value
    /// Output: Fuel Level Value
    pub struct FuelLevelValue(Metadata<22796, 65535, 0>);
    impl FuelLevelValue {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FuelLevelValue {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FuelLevelValue {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Heading Value
    /// Output: Heading Value
    pub struct HeadingValue(Metadata<22780, 65535, 0>);
    impl HeadingValue {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HeadingValue {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HeadingValue {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Airspeed IAS Value
    /// Output: Airspeed IAS Value
    pub struct IasValue(Metadata<22798, 65535, 0>);
    impl IasValue {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IasValue {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IasValue {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// NADIR Heading Value
    /// Output: NADIR Heading Value
    pub struct NadirHeadingValue(Metadata<22782, 65535, 0>);
    impl NadirHeadingValue {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for NadirHeadingValue {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for NadirHeadingValue {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Radar Altimeter Value
    /// Output: Radar Altimeter Value
    pub struct RadarAltValue(Metadata<22792, 65535, 0>);
    impl RadarAltValue {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RadarAltValue {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadarAltValue {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Rotor RPM Value
    /// Output: Rotor RPM Value
    pub struct RotorRpmValue(Metadata<22802, 65535, 0>);
    impl RotorRpmValue {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RotorRpmValue {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RotorRpmValue {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Torque Value
    /// Output: Torque Value
    pub struct TorqueValue(Metadata<22794, 65535, 0>);
    impl TorqueValue {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TorqueValue {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TorqueValue {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Turbine RPM Value
    /// Output: Turbine RPM Value
    pub struct TurbineRpmValue(Metadata<22800, 65535, 0>);
    impl TurbineRpmValue {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TurbineRpmValue {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TurbineRpmValue {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod hot3 {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// HOT3 Brightness
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct Hot3Brightness(LimitedDial<22616, 65535, 0>);
    impl Hot3Brightness {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Hot3Brightness {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Hot3Brightness {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Hot3PanelTestOffOnValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
    }
    pub type Hot3PanelTestOffOnValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(Hot3PanelTestOffOnValue) + 'a>;
    impl core::convert::TryFrom<u16> for Hot3PanelTestOffOnValue {
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

    /// HOT3 Panel Test/Off/On
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct Hot3PanelTestOffOn(Selector<22614, 7, 0>);
    impl Hot3PanelTestOffOn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Hot3PanelTestOffOn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Hot3PanelTestOffOn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Hot3StationSelectValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
        Select6,
        Select7,
        Select8,
        Select9,
    }
    pub type Hot3StationSelectValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(Hot3StationSelectValue) + 'a>;
    impl core::convert::TryFrom<u16> for Hot3StationSelectValue {
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
                value => Err(value),
            }
        }
    }

    /// HOT3 Station Select
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct Hot3StationSelect(Selector<22614, 120, 3>);
    impl Hot3StationSelect {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Hot3StationSelect {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Hot3StationSelect {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod indicatorlights {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Additional Fuel Tank Lamp
    /// Output: 0 if light is off, 1 if light is on
    pub struct AdditionalFuelTankLamp(Led<22624, 4096, 12>);
    impl AdditionalFuelTankLamp {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for AdditionalFuelTankLamp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdditionalFuelTankLamp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Alarm Warning Lamp
    /// Output: 0 if light is off, 1 if light is on
    pub struct AlarmWarningLamp(Led<22624, 32768, 15>);
    impl AlarmWarningLamp {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for AlarmWarningLamp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AlarmWarningLamp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// AM Radio Lamp
    /// Output: 0 if light is off, 1 if light is on
    pub struct AmRadioLamp(Led<22624, 8, 3>);
    impl AmRadioLamp {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for AmRadioLamp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AmRadioLamp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Block Engine Warning Lamp
    /// Output: 0 if light is off, 1 if light is on
    pub struct BlockedEngineWarningLamp(Led<22624, 2048, 11>);
    impl BlockedEngineWarningLamp {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for BlockedEngineWarningLamp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for BlockedEngineWarningLamp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Convoy Fuel Tank Lamp
    /// Output: 0 if light is off, 1 if light is on
    pub struct ConvoyFuelTankLamp(Led<22624, 8192, 13>);
    impl ConvoyFuelTankLamp {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ConvoyFuelTankLamp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ConvoyFuelTankLamp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// DEM Start Lamp
    /// Output: 0 if light is off, 1 if light is on
    pub struct DemStartLamp(Led<22624, 512, 9>);
    impl DemStartLamp {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for DemStartLamp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for DemStartLamp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FD Lamp D OPE
    /// Output: 0 if light is off, 1 if light is on
    pub struct FdLampDOpe(Led<22622, 4096, 12>);
    impl FdLampDOpe {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for FdLampDOpe {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FdLampDOpe {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FD Lamp D Vide1
    /// Output: 0 if light is off, 1 if light is on
    pub struct FdLampDVide1(Led<22624, 1, 0>);
    impl FdLampDVide1 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for FdLampDVide1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FdLampDVide1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FD Lamp D Vide2
    /// Output: 0 if light is off, 1 if light is on
    pub struct FdLampDVide2(Led<22624, 2, 1>);
    impl FdLampDVide2 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for FdLampDVide2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FdLampDVide2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FD Lamp G OPE
    /// Output: 0 if light is off, 1 if light is on
    pub struct FdLampGOpe(Led<22622, 2048, 11>);
    impl FdLampGOpe {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for FdLampGOpe {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FdLampGOpe {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FD Lamp G Vide1
    /// Output: 0 if light is off, 1 if light is on
    pub struct FdLampGVide1(Led<22622, 16384, 14>);
    impl FdLampGVide1 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for FdLampGVide1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FdLampGVide1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FD Lamp G Vide2
    /// Output: 0 if light is off, 1 if light is on
    pub struct FdLampGVide2(Led<22622, 32768, 15>);
    impl FdLampGVide2 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for FdLampGVide2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FdLampGVide2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FD Lamp LEU
    /// Output: 0 if light is off, 1 if light is on
    pub struct FdLampLeu(Led<22622, 8192, 13>);
    impl FdLampLeu {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for FdLampLeu {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FdLampLeu {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FD Lamp VE
    /// Output: 0 if light is off, 1 if light is on
    pub struct FdLampVe(Led<22622, 1024, 10>);
    impl FdLampVe {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for FdLampVe {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FdLampVe {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Gyro BPP Warning Lamp
    /// Output: 0 if light is off, 1 if light is on
    pub struct GyroBppWarningLamp(Led<22624, 128, 7>);
    impl GyroBppWarningLamp {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for GyroBppWarningLamp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for GyroBppWarningLamp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Gyro Test Lamp
    /// Output: 0 if light is off, 1 if light is on
    pub struct GyroTestLamp(Led<22624, 32, 5>);
    impl GyroTestLamp {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for GyroTestLamp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for GyroTestLamp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Gyro Trim Warning Lamp
    /// Output: 0 if light is off, 1 if light is on
    pub struct GyroTrimWarningLamp(Led<22624, 64, 6>);
    impl GyroTrimWarningLamp {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for GyroTrimWarningLamp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for GyroTrimWarningLamp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Radar Altimeter Alarm Lamp
    /// Output: 0 if light is off, 1 if light is on
    pub struct RadarAltAlarmLamp(Led<22624, 4, 2>);
    impl RadarAltAlarmLamp {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RadarAltAlarmLamp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadarAltAlarmLamp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RLT Idle Lamp
    /// Output: 0 if light is off, 1 if light is on
    pub struct RltIdleLamp(Led<22624, 1024, 10>);
    impl RltIdleLamp {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RltIdleLamp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RltIdleLamp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RWR Light
    /// Output: 0 if light is off, 1 if light is on
    pub struct RwrLight(Led<22624, 16, 4>);
    impl RwrLight {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RwrLight {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrLight {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Sand Filter Lamp
    /// Output: 0 if light is off, 1 if light is on
    pub struct SandFilterLamp(Led<22624, 16384, 14>);
    impl SandFilterLamp {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for SandFilterLamp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SandFilterLamp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Torque Warning Lamp
    /// Output: 0 if light is off, 1 if light is on
    pub struct TorqueWarningLamp(Led<22624, 256, 8>);
    impl TorqueWarningLamp {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for TorqueWarningLamp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TorqueWarningLamp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// TV Lamp
    /// Output: 0 if light is off, 1 if light is on
    pub struct TvLamp(Led<22626, 1, 0>);
    impl TvLamp {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for TvLamp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TvLamp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Weapons Panel 2 Left Lamp
    /// Output: 0 if light is off, 1 if light is on
    pub struct Wp2LeftLamp(Led<22626, 2, 1>);
    impl Wp2LeftLamp {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for Wp2LeftLamp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Wp2LeftLamp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Weapons Panel 2 Right Lamp
    /// Output: 0 if light is off, 1 if light is on
    pub struct Wp2RightLamp(Led<22626, 4, 2>);
    impl Wp2RightLamp {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for Wp2RightLamp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Wp2RightLamp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod intercom {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// FM Radio Volume 1
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct FmRadioVolume1(AnalogDial<22534, 65535, 0>);
    impl FmRadioVolume1 {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FmRadioVolume1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FmRadioVolume1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FM Radio Volume 2
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct FmRadioVolume2(AnalogDial<22540, 65535, 0>);
    impl FmRadioVolume2 {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FmRadioVolume2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FmRadioVolume2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// UHF Radio Volume 1
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct UhfRadioVolume1(AnalogDial<22536, 65535, 0>);
    impl UhfRadioVolume1 {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfRadioVolume1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfRadioVolume1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// UHF Radio Volume 2
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct UhfRadioVolume2(AnalogDial<22542, 65535, 0>);
    impl UhfRadioVolume2 {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfRadioVolume2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfRadioVolume2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// VHF AM Radio Volume 1
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct VhfAmRadioVolume1(AnalogDial<22532, 65535, 0>);
    impl VhfAmRadioVolume1 {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for VhfAmRadioVolume1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for VhfAmRadioVolume1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// VHF AM Radio Volume 2
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct VhfAmRadioVolume2(AnalogDial<22538, 65535, 0>);
    impl VhfAmRadioVolume2 {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for VhfAmRadioVolume2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for VhfAmRadioVolume2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod lights {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Console Lighting
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct ConsoleLighting(LimitedDial<22562, 65535, 0>);
    impl ConsoleLighting {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ConsoleLighting {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ConsoleLighting {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Main Dashboard Lighting
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct DashboardLighting(LimitedDial<22560, 65535, 0>);
    impl DashboardLighting {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for DashboardLighting {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for DashboardLighting {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LightsNormBnlValue {
        Select1,
        Select2,
    }
    pub type LightsNormBnlValueCallback<'a> = alloc::boxed::Box<dyn FnMut(LightsNormBnlValue) + 'a>;
    impl core::convert::TryFrom<u16> for LightsNormBnlValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Lights Norm/BNL
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LightsNormBnl(Selector<22554, 8, 3>);
    impl LightsNormBnl {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LightsNormBnl {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LightsNormBnl {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RedLensOnOffValue {
        Select1,
        Select2,
    }
    pub type RedLensOnOffValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RedLensOnOffValue) + 'a>;
    impl core::convert::TryFrom<u16> for RedLensOnOffValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Red Lens On/Off
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RedLensOnOff(Selector<22554, 16, 4>);
    impl RedLensOnOff {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RedLensOnOff {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RedLensOnOff {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Roof Lamp Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct RoofLamp(LimitedDial<22566, 65535, 0>);
    impl RoofLamp {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RoofLamp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RoofLamp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// UV Lighting
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct UvLighting(LimitedDial<22564, 65535, 0>);
    impl UvLighting {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UvLighting {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UvLighting {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod nadir {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Nadir0ButtonValue {
        Select1,
        Select2,
    }
    pub type Nadir0ButtonValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Nadir0ButtonValue) + 'a>;
    impl core::convert::TryFrom<u16> for Nadir0ButtonValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// NADIR 0 Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Nadir0Button(Selector<22602, 2048, 11>);
    impl Nadir0Button {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Nadir0Button {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Nadir0Button {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Nadir1ButtonValue {
        Select1,
        Select2,
    }
    pub type Nadir1ButtonValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Nadir1ButtonValue) + 'a>;
    impl core::convert::TryFrom<u16> for Nadir1ButtonValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// NADIR 1 Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Nadir1Button(Selector<22602, 4096, 12>);
    impl Nadir1Button {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Nadir1Button {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Nadir1Button {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Nadir2ButtonValue {
        Select1,
        Select2,
    }
    pub type Nadir2ButtonValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Nadir2ButtonValue) + 'a>;
    impl core::convert::TryFrom<u16> for Nadir2ButtonValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// NADIR 2 Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Nadir2Button(Selector<22602, 8192, 13>);
    impl Nadir2Button {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Nadir2Button {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Nadir2Button {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Nadir3ButtonValue {
        Select1,
        Select2,
    }
    pub type Nadir3ButtonValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Nadir3ButtonValue) + 'a>;
    impl core::convert::TryFrom<u16> for Nadir3ButtonValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// NADIR 3 Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Nadir3Button(Selector<22602, 16384, 14>);
    impl Nadir3Button {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Nadir3Button {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Nadir3Button {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Nadir4ButtonValue {
        Select1,
        Select2,
    }
    pub type Nadir4ButtonValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Nadir4ButtonValue) + 'a>;
    impl core::convert::TryFrom<u16> for Nadir4ButtonValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// NADIR 4 Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Nadir4Button(Selector<22602, 32768, 15>);
    impl Nadir4Button {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Nadir4Button {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Nadir4Button {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Nadir5ButtonValue {
        Select1,
        Select2,
    }
    pub type Nadir5ButtonValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Nadir5ButtonValue) + 'a>;
    impl core::convert::TryFrom<u16> for Nadir5ButtonValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// NADIR 5 Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Nadir5Button(Selector<22604, 1, 0>);
    impl Nadir5Button {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Nadir5Button {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Nadir5Button {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Nadir6ButtonValue {
        Select1,
        Select2,
    }
    pub type Nadir6ButtonValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Nadir6ButtonValue) + 'a>;
    impl core::convert::TryFrom<u16> for Nadir6ButtonValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// NADIR 6 Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Nadir6Button(Selector<22604, 2, 1>);
    impl Nadir6Button {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Nadir6Button {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Nadir6Button {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Nadir7ButtonValue {
        Select1,
        Select2,
    }
    pub type Nadir7ButtonValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Nadir7ButtonValue) + 'a>;
    impl core::convert::TryFrom<u16> for Nadir7ButtonValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// NADIR 7 Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Nadir7Button(Selector<22604, 4, 2>);
    impl Nadir7Button {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Nadir7Button {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Nadir7Button {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Nadir8ButtonValue {
        Select1,
        Select2,
    }
    pub type Nadir8ButtonValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Nadir8ButtonValue) + 'a>;
    impl core::convert::TryFrom<u16> for Nadir8ButtonValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// NADIR 8 Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Nadir8Button(Selector<22604, 8, 3>);
    impl Nadir8Button {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Nadir8Button {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Nadir8Button {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Nadir9ButtonValue {
        Select1,
        Select2,
    }
    pub type Nadir9ButtonValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Nadir9ButtonValue) + 'a>;
    impl core::convert::TryFrom<u16> for Nadir9ButtonValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// NADIR 9 Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Nadir9Button(Selector<22604, 16, 4>);
    impl Nadir9Button {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Nadir9Button {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Nadir9Button {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NadirAuxButtonValue {
        Select1,
        Select2,
    }
    pub type NadirAuxButtonValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(NadirAuxButtonValue) + 'a>;
    impl core::convert::TryFrom<u16> for NadirAuxButtonValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// NADIR Auxiliary Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct NadirAuxButton(Selector<22602, 256, 8>);
    impl NadirAuxButton {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for NadirAuxButton {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for NadirAuxButton {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NadirDestButtonValue {
        Select1,
        Select2,
    }
    pub type NadirDestButtonValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(NadirDestButtonValue) + 'a>;
    impl core::convert::TryFrom<u16> for NadirDestButtonValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// NADIR Destination Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct NadirDestButton(Selector<22602, 128, 7>);
    impl NadirDestButton {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for NadirDestButton {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for NadirDestButton {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NadirDopplerModeValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
        Select6,
    }
    pub type NadirDopplerModeValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(NadirDopplerModeValue) + 'a>;
    impl core::convert::TryFrom<u16> for NadirDopplerModeValue {
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

    /// NADIR Doppler Mode
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct NadirDopplerMode(Selector<22602, 7, 0>);
    impl NadirDopplerMode {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for NadirDopplerMode {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for NadirDopplerMode {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NadirDownArrowValue {
        Select1,
        Select2,
    }
    pub type NadirDownArrowValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(NadirDownArrowValue) + 'a>;
    impl core::convert::TryFrom<u16> for NadirDownArrowValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// NADIR Down Arrow Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct NadirDownArrow(Selector<22602, 1024, 10>);
    impl NadirDownArrow {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for NadirDownArrow {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for NadirDownArrow {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NadirEffButtonValue {
        Select1,
        Select2,
    }
    pub type NadirEffButtonValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(NadirEffButtonValue) + 'a>;
    impl core::convert::TryFrom<u16> for NadirEffButtonValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// NADIR EFF Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct NadirEffButton(Selector<22604, 512, 9>);
    impl NadirEffButton {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for NadirEffButton {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for NadirEffButton {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NadirEnterButtonValue {
        Select1,
        Select2,
    }
    pub type NadirEnterButtonValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(NadirEnterButtonValue) + 'a>;
    impl core::convert::TryFrom<u16> for NadirEnterButtonValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// NADIR Enter Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct NadirEnterButton(Selector<22602, 64, 6>);
    impl NadirEnterButton {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for NadirEnterButton {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for NadirEnterButton {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NadirGelButtonValue {
        Select1,
        Select2,
    }
    pub type NadirGelButtonValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(NadirGelButtonValue) + 'a>;
    impl core::convert::TryFrom<u16> for NadirGelButtonValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// NADIR GEL Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct NadirGelButton(Selector<22604, 256, 8>);
    impl NadirGelButton {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for NadirGelButton {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for NadirGelButton {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NadirGeoButtonValue {
        Select1,
        Select2,
    }
    pub type NadirGeoButtonValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(NadirGeoButtonValue) + 'a>;
    impl core::convert::TryFrom<u16> for NadirGeoButtonValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// NADIR GEO Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct NadirGeoButton(Selector<22604, 64, 6>);
    impl NadirGeoButton {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for NadirGeoButton {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for NadirGeoButton {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NadirMapIndButtonValue {
        Select1,
        Select2,
    }
    pub type NadirMapIndButtonValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(NadirMapIndButtonValue) + 'a>;
    impl core::convert::TryFrom<u16> for NadirMapIndButtonValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// NADIR Map Indicator Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct NadirMapIndButton(Selector<22602, 512, 9>);
    impl NadirMapIndButton {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for NadirMapIndButton {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for NadirMapIndButton {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// NADIR On/Off/Brightness
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct NadirOnOffBrightness(LimitedDial<22600, 65535, 0>);
    impl NadirOnOffBrightness {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for NadirOnOffBrightness {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for NadirOnOffBrightness {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NadirParameterValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
        Select6,
    }
    pub type NadirParameterValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(NadirParameterValue) + 'a>;
    impl core::convert::TryFrom<u16> for NadirParameterValue {
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

    /// NADIR Parameter
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct NadirParameter(Selector<22602, 56, 3>);
    impl NadirParameter {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for NadirParameter {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for NadirParameter {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NadirPolButtonValue {
        Select1,
        Select2,
    }
    pub type NadirPolButtonValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(NadirPolButtonValue) + 'a>;
    impl core::convert::TryFrom<u16> for NadirPolButtonValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// NADIR POL Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct NadirPolButton(Selector<22604, 32, 5>);
    impl NadirPolButton {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for NadirPolButton {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for NadirPolButton {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NadirPosButtonValue {
        Select1,
        Select2,
    }
    pub type NadirPosButtonValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(NadirPosButtonValue) + 'a>;
    impl core::convert::TryFrom<u16> for NadirPosButtonValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// NADIR POS Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct NadirPosButton(Selector<22604, 128, 7>);
    impl NadirPosButton {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for NadirPosButton {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for NadirPosButton {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod navlights {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Anticollision Light Intensity
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct AnticollIntensity(LimitedDial<22568, 65535, 0>);
    impl AnticollIntensity {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AnticollIntensity {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AnticollIntensity {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AnticollLightsValue {
        Select1,
        Select2,
        Select3,
    }
    pub type AnticollLightsValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(AnticollLightsValue) + 'a>;
    impl core::convert::TryFrom<u16> for AnticollLightsValue {
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

    /// Anticollision Light NOR/OFF/ATT
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct AnticollLights(Selector<22554, 384, 7>);
    impl AnticollLights {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AnticollLights {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AnticollLights {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Formation Lights Intensity
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct FormationIntensity(LimitedDial<22570, 65535, 0>);
    impl FormationIntensity {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FormationIntensity {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FormationIntensity {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FormationLightsValue {
        Select1,
        Select2,
    }
    pub type FormationLightsValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(FormationLightsValue) + 'a>;
    impl core::convert::TryFrom<u16> for FormationLightsValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Formation Lights On/Off
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FormationLights(Selector<22554, 16384, 14>);
    impl FormationLights {
        pub const fn new() -> Self {
            Self(Selector::new())
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
    pub enum LandingLightExtendValue {
        Select1,
        Select2,
    }
    pub type LandingLightExtendValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(LandingLightExtendValue) + 'a>;
    impl core::convert::TryFrom<u16> for LandingLightExtendValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Landing Light Extend
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LandingLightExtend(Selector<22554, 2048, 11>);
    impl LandingLightExtend {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LandingLightExtend {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LandingLightExtend {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LandingLightRetractValue {
        Select1,
        Select2,
    }
    pub type LandingLightRetractValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(LandingLightRetractValue) + 'a>;
    impl core::convert::TryFrom<u16> for LandingLightRetractValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Landing Light Retract
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LandingLightRetract(Selector<22554, 4096, 12>);
    impl LandingLightRetract {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LandingLightRetract {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LandingLightRetract {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LandingLightsValue {
        Select1,
        Select2,
        Select3,
    }
    pub type LandingLightsValueCallback<'a> = alloc::boxed::Box<dyn FnMut(LandingLightsValue) + 'a>;
    impl core::convert::TryFrom<u16> for LandingLightsValue {
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

    /// Landing Light Off/Vario/On
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct LandingLights(Selector<22554, 1536, 9>);
    impl LandingLights {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LandingLights {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LandingLights {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NavLightsValue {
        Select1,
        Select2,
        Select3,
    }
    pub type NavLightsValueCallback<'a> = alloc::boxed::Box<dyn FnMut(NavLightsValue) + 'a>;
    impl core::convert::TryFrom<u16> for NavLightsValue {
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

    /// Navigation Lights CLI/OFF/FIX
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct NavLights(Selector<22554, 96, 5>);
    impl NavLights {
        pub const fn new() -> Self {
            Self(Selector::new())
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum PanelLightingValue {
        Select1,
        Select2,
    }
    pub type PanelLightingValueCallback<'a> = alloc::boxed::Box<dyn FnMut(PanelLightingValue) + 'a>;
    impl core::convert::TryFrom<u16> for PanelLightingValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Panel Lighting On/Off
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct PanelLighting(Selector<22554, 8192, 13>);
    impl PanelLighting {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for PanelLighting {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for PanelLighting {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod pe {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CameraCenteringValue {
        Select1,
        Select2,
    }
    pub type CameraCenteringValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(CameraCenteringValue) + 'a>;
    impl core::convert::TryFrom<u16> for CameraCenteringValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Camera Centering
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CameraCentering(Selector<22572, 8192, 13>);
    impl CameraCentering {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CameraCentering {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CameraCentering {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CameraModeValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
    }
    pub type CameraModeValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CameraModeValue) + 'a>;
    impl core::convert::TryFrom<u16> for CameraModeValue {
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

    /// Camera Mode
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct CameraMode(Selector<22574, 112, 4>);
    impl CameraMode {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CameraMode {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CameraMode {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CameraZoomInValue {
        Select1,
        Select2,
    }
    pub type CameraZoomInValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CameraZoomInValue) + 'a>;
    impl core::convert::TryFrom<u16> for CameraZoomInValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Camera Zoom In
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CameraZoomIn(Selector<22572, 32768, 15>);
    impl CameraZoomIn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CameraZoomIn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CameraZoomIn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CameraZoomOutValue {
        Select1,
        Select2,
    }
    pub type CameraZoomOutValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CameraZoomOutValue) + 'a>;
    impl core::convert::TryFrom<u16> for CameraZoomOutValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Camera Zoom Out
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CameraZoomOut(Selector<22574, 1, 0>);
    impl CameraZoomOut {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CameraZoomOut {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CameraZoomOut {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CthPowerValue {
        Select1,
        Select2,
        Select3,
    }
    pub type CthPowerValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CthPowerValue) + 'a>;
    impl core::convert::TryFrom<u16> for CthPowerValue {
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

    /// CTH Power
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct CthPower(Selector<22574, 6, 1>);
    impl CthPower {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CthPower {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CthPower {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ImageBrightnessDownValue {
        Select1,
        Select2,
    }
    pub type ImageBrightnessDownValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(ImageBrightnessDownValue) + 'a>;
    impl core::convert::TryFrom<u16> for ImageBrightnessDownValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Image Brightness Down
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ImageBrightnessDown(Selector<22572, 512, 9>);
    impl ImageBrightnessDown {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ImageBrightnessDown {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ImageBrightnessDown {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ImageBrightnessUpValue {
        Select1,
        Select2,
    }
    pub type ImageBrightnessUpValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(ImageBrightnessUpValue) + 'a>;
    impl core::convert::TryFrom<u16> for ImageBrightnessUpValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Image Brightness Up
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ImageBrightnessUp(Selector<22572, 1024, 10>);
    impl ImageBrightnessUp {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ImageBrightnessUp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ImageBrightnessUp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ImageFocusDownValue {
        Select1,
        Select2,
    }
    pub type ImageFocusDownValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(ImageFocusDownValue) + 'a>;
    impl core::convert::TryFrom<u16> for ImageFocusDownValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Image Focus Down
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ImageFocusDown(Selector<22572, 32, 5>);
    impl ImageFocusDown {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ImageFocusDown {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ImageFocusDown {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ImageFocusUpValue {
        Select1,
        Select2,
    }
    pub type ImageFocusUpValueCallback<'a> = alloc::boxed::Box<dyn FnMut(ImageFocusUpValue) + 'a>;
    impl core::convert::TryFrom<u16> for ImageFocusUpValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Image Focus Up
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ImageFocusUp(Selector<22572, 64, 6>);
    impl ImageFocusUp {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ImageFocusUp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ImageFocusUp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ImageGainLeftValue {
        Select1,
        Select2,
    }
    pub type ImageGainLeftValueCallback<'a> = alloc::boxed::Box<dyn FnMut(ImageGainLeftValue) + 'a>;
    impl core::convert::TryFrom<u16> for ImageGainLeftValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Image Gain Left
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ImageGainLeft(Selector<22572, 128, 7>);
    impl ImageGainLeft {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ImageGainLeft {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ImageGainLeft {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ImageGainRightValue {
        Select1,
        Select2,
    }
    pub type ImageGainRightValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(ImageGainRightValue) + 'a>;
    impl core::convert::TryFrom<u16> for ImageGainRightValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Image Gain Right
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ImageGainRight(Selector<22572, 256, 8>);
    impl ImageGainRight {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ImageGainRight {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ImageGainRight {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum InversedImageValue {
        Select1,
        Select2,
    }
    pub type InversedImageValueCallback<'a> = alloc::boxed::Box<dyn FnMut(InversedImageValue) + 'a>;
    impl core::convert::TryFrom<u16> for InversedImageValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Inversed Image
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct InversedImage(Selector<22572, 16, 4>);
    impl InversedImage {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for InversedImage {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for InversedImage {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum InversedSymbologyValue {
        Select1,
        Select2,
    }
    pub type InversedSymbologyValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(InversedSymbologyValue) + 'a>;
    impl core::convert::TryFrom<u16> for InversedSymbologyValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Inversed Symbology
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct InversedSymbology(Selector<22572, 8, 3>);
    impl InversedSymbology {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for InversedSymbology {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for InversedSymbology {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LaserPowerValue {
        Select1,
        Select2,
    }
    pub type LaserPowerValueCallback<'a> = alloc::boxed::Box<dyn FnMut(LaserPowerValue) + 'a>;
    impl core::convert::TryFrom<u16> for LaserPowerValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Laser Power
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LaserPower(Selector<22574, 8, 3>);
    impl LaserPower {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LaserPower {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LaserPower {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LasingButtonValue {
        Select1,
        Select2,
    }
    pub type LasingButtonValueCallback<'a> = alloc::boxed::Box<dyn FnMut(LasingButtonValue) + 'a>;
    impl core::convert::TryFrom<u16> for LasingButtonValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Lasing Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LasingButton(Selector<22572, 1, 0>);
    impl LasingButton {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LasingButton {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LasingButton {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LasingButtonCoverValue {
        Select1,
        Select2,
    }
    pub type LasingButtonCoverValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(LasingButtonCoverValue) + 'a>;
    impl core::convert::TryFrom<u16> for LasingButtonCoverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Lasing Button Cover
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LasingButtonCover(Selector<22554, 32768, 15>);
    impl LasingButtonCover {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LasingButtonCover {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LasingButtonCover {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MissileLaunchValue {
        Select1,
        Select2,
    }
    pub type MissileLaunchValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MissileLaunchValue) + 'a>;
    impl core::convert::TryFrom<u16> for MissileLaunchValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Missile Launch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MissileLaunch(Selector<22572, 4, 2>);
    impl MissileLaunch {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MissileLaunch {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MissileLaunch {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MissileLaunchCoverValue {
        Select1,
        Select2,
    }
    pub type MissileLaunchCoverValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(MissileLaunchCoverValue) + 'a>;
    impl core::convert::TryFrom<u16> for MissileLaunchCoverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Missile Launch Cover
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MissileLaunchCover(Selector<22572, 2, 1>);
    impl MissileLaunchCover {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MissileLaunchCover {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MissileLaunchCover {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SymbologyBrightnessLeftValue {
        Select1,
        Select2,
    }
    pub type SymbologyBrightnessLeftValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(SymbologyBrightnessLeftValue) + 'a>;
    impl core::convert::TryFrom<u16> for SymbologyBrightnessLeftValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Symbology Brightness Left
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct SymbologyBrightnessLeft(Selector<22572, 2048, 11>);
    impl SymbologyBrightnessLeft {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SymbologyBrightnessLeft {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SymbologyBrightnessLeft {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SymbologyBrightnessRightValue {
        Select1,
        Select2,
    }
    pub type SymbologyBrightnessRightValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(SymbologyBrightnessRightValue) + 'a>;
    impl core::convert::TryFrom<u16> for SymbologyBrightnessRightValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Symbology Brightness Right
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct SymbologyBrightnessRight(Selector<22572, 4096, 12>);
    impl SymbologyBrightnessRight {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SymbologyBrightnessRight {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SymbologyBrightnessRight {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ToggleVdoVthValue {
        Select1,
        Select2,
    }
    pub type ToggleVdoVthValueCallback<'a> = alloc::boxed::Box<dyn FnMut(ToggleVdoVthValue) + 'a>;
    impl core::convert::TryFrom<u16> for ToggleVdoVthValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Toggle VDO/VTH
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ToggleVdoVth(Selector<22572, 16384, 14>);
    impl ToggleVdoVth {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ToggleVdoVth {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ToggleVdoVth {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod radaraltimeter {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Radar Altimeter Bug Adjust
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct RadarAltBug(AnalogDial<22578, 65535, 0>);
    impl RadarAltBug {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RadarAltBug {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadarAltBug {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Radar Altimeter Power
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct RadarAltPower(AnalogDial<22580, 65535, 0>);
    impl RadarAltPower {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RadarAltPower {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadarAltPower {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RadarAltTestValue {
        Select1,
        Select2,
    }
    pub type RadarAltTestValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RadarAltTestValue) + 'a>;
    impl core::convert::TryFrom<u16> for RadarAltTestValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Radar Altimeter Test
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RadarAltTest(Selector<22574, 256, 8>);
    impl RadarAltTest {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RadarAltTest {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadarAltTest {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod rawgaugevalues {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// ADF Background
    /// Output: gauge position
    pub struct AdfBackground(AnalogGauge<22656, 65535, 0>);
    impl AdfBackground {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdfBackground {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdfBackground {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADF Distance 100s
    /// Output: gauge position
    pub struct AdfDistance100s(AnalogGauge<22668, 65535, 0>);
    impl AdfDistance100s {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdfDistance100s {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdfDistance100s {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADF Distance 10s
    /// Output: gauge position
    pub struct AdfDistance10s(AnalogGauge<22670, 65535, 0>);
    impl AdfDistance10s {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdfDistance10s {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdfDistance10s {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADF Distance 1s
    /// Output: gauge position
    pub struct AdfDistance1s(AnalogGauge<22672, 65535, 0>);
    impl AdfDistance1s {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdfDistance1s {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdfDistance1s {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADF Flag But
    /// Output: gauge position
    pub struct AdfFlagBut(AnalogGauge<22664, 65535, 0>);
    impl AdfFlagBut {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdfFlagBut {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdfFlagBut {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADF Flag CAP
    /// Output: gauge position
    pub struct AdfFlagCap(AnalogGauge<22662, 65535, 0>);
    impl AdfFlagCap {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdfFlagCap {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdfFlagCap {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADF Flag Counter
    /// Output: gauge position
    pub struct AdfFlagCounter(AnalogGauge<22666, 65535, 0>);
    impl AdfFlagCounter {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdfFlagCounter {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdfFlagCounter {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADF NADIR Needle
    /// Output: gauge position
    pub struct AdfNadirNeedle(AnalogGauge<22658, 65535, 0>);
    impl AdfNadirNeedle {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdfNadirNeedle {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdfNadirNeedle {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADF NAV1 Frequency 100s
    /// Output: gauge position
    pub struct AdfNav1Freq100s(AnalogGauge<22640, 65535, 0>);
    impl AdfNav1Freq100s {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdfNav1Freq100s {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdfNav1Freq100s {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADF NAV1 Frequency 10s
    /// Output: gauge position
    pub struct AdfNav1Freq10s(AnalogGauge<22642, 65535, 0>);
    impl AdfNav1Freq10s {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdfNav1Freq10s {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdfNav1Freq10s {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADF NAV1 Frequency 1s
    /// Output: gauge position
    pub struct AdfNav1Freq1s(AnalogGauge<22644, 65535, 0>);
    impl AdfNav1Freq1s {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdfNav1Freq1s {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdfNav1Freq1s {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADF NAV1 Frequency Decimal
    /// Output: gauge position
    pub struct AdfNav1FreqDecimal(AnalogGauge<22646, 65535, 0>);
    impl AdfNav1FreqDecimal {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdfNav1FreqDecimal {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdfNav1FreqDecimal {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADF NAV2 Frequency 100s
    /// Output: gauge position
    pub struct AdfNav2Freq100s(AnalogGauge<22648, 65535, 0>);
    impl AdfNav2Freq100s {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdfNav2Freq100s {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdfNav2Freq100s {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADF NAV2 Frequency 10s
    /// Output: gauge position
    pub struct AdfNav2Freq10s(AnalogGauge<22650, 65535, 0>);
    impl AdfNav2Freq10s {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdfNav2Freq10s {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdfNav2Freq10s {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADF NAV2 Frequency 1s
    /// Output: gauge position
    pub struct AdfNav2Freq1s(AnalogGauge<22652, 65535, 0>);
    impl AdfNav2Freq1s {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdfNav2Freq1s {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdfNav2Freq1s {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADF NAV2 Frequency Decimal
    /// Output: gauge position
    pub struct AdfNav2FreqDecimal(AnalogGauge<22654, 65535, 0>);
    impl AdfNav2FreqDecimal {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdfNav2FreqDecimal {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdfNav2FreqDecimal {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADF Small Needle
    /// Output: gauge position
    pub struct AdfSmallNeedle(AnalogGauge<22660, 65535, 0>);
    impl AdfSmallNeedle {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdfSmallNeedle {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdfSmallNeedle {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// AM Radio Frequency 100s
    /// Output: gauge position
    pub struct AmRadioFreq100s(AnalogGauge<22770, 65535, 0>);
    impl AmRadioFreq100s {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AmRadioFreq100s {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AmRadioFreq100s {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// AM Radio Frequency 10s
    /// Output: gauge position
    pub struct AmRadioFreq10s(AnalogGauge<22772, 65535, 0>);
    impl AmRadioFreq10s {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AmRadioFreq10s {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AmRadioFreq10s {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// AM Radio Frequency 1s
    /// Output: gauge position
    pub struct AmRadioFreq1s(AnalogGauge<22774, 65535, 0>);
    impl AmRadioFreq1s {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AmRadioFreq1s {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AmRadioFreq1s {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// AM Radio Frequency Hundredths
    /// Output: gauge position
    pub struct AmRadioFreqHundredths(AnalogGauge<22778, 65535, 0>);
    impl AmRadioFreqHundredths {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AmRadioFreqHundredths {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AmRadioFreqHundredths {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// AM Radio Frequency Tenths
    /// Output: gauge position
    pub struct AmRadioFreqTenths(AnalogGauge<22776, 65535, 0>);
    impl AmRadioFreqTenths {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AmRadioFreqTenths {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AmRadioFreqTenths {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Artificial Horizon Ball
    /// Output: gauge position
    pub struct ArtHorizBall(AnalogGauge<22736, 65535, 0>);
    impl ArtHorizBall {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ArtHorizBall {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ArtHorizBall {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Artificial Horizon Curseur
    /// Output: gauge position
    pub struct ArtHorizCurseur(AnalogGauge<22744, 65535, 0>);
    impl ArtHorizCurseur {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ArtHorizCurseur {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ArtHorizCurseur {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Artificial Horizon Flag GS
    /// Output: gauge position
    pub struct ArtHorizFlagGs(AnalogGauge<22738, 65535, 0>);
    impl ArtHorizFlagGs {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ArtHorizFlagGs {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ArtHorizFlagGs {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Artificial Horizon Flag HS
    /// Output: gauge position
    pub struct ArtHorizFlagHs(AnalogGauge<22740, 65535, 0>);
    impl ArtHorizFlagHs {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ArtHorizFlagHs {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ArtHorizFlagHs {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Artificial Horizon Flag LOC
    /// Output: gauge position
    pub struct ArtHorizFlagLoc(AnalogGauge<22742, 65535, 0>);
    impl ArtHorizFlagLoc {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ArtHorizFlagLoc {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ArtHorizFlagLoc {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Artificial Horizon Horizontal Bar
    /// Output: gauge position
    pub struct ArtHorizHorizontalBar(AnalogGauge<22750, 65535, 0>);
    impl ArtHorizHorizontalBar {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ArtHorizHorizontalBar {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ArtHorizHorizontalBar {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Artificial Horizon Pitch
    /// Output: gauge position
    pub struct ArtHorizPitch(AnalogGauge<22732, 65535, 0>);
    impl ArtHorizPitch {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ArtHorizPitch {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ArtHorizPitch {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Artificial Horizon Roll
    /// Output: gauge position
    pub struct ArtHorizRoll(AnalogGauge<22734, 65535, 0>);
    impl ArtHorizRoll {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ArtHorizRoll {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ArtHorizRoll {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Artificial Horizon Vertical Bar
    /// Output: gauge position
    pub struct ArtHorizVertBar(AnalogGauge<22748, 65535, 0>);
    impl ArtHorizVertBar {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ArtHorizVertBar {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ArtHorizVertBar {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Artificial Horizon W
    /// Output: gauge position
    pub struct ArtHorizW(AnalogGauge<22746, 65535, 0>);
    impl ArtHorizW {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ArtHorizW {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ArtHorizW {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Auxiliary Fuel Tank
    /// Output: gauge position
    pub struct AuxFuelTank(AnalogGauge<22768, 65535, 0>);
    impl AuxFuelTank {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AuxFuelTank {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AuxFuelTank {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Baro Altimeter 100s
    /// Output: gauge position
    pub struct BaroAltHundreds(AnalogGauge<22694, 65535, 0>);
    impl BaroAltHundreds {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for BaroAltHundreds {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for BaroAltHundreds {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Baro Altimeter 1000s
    /// Output: gauge position
    pub struct BaroAltThousands(AnalogGauge<22692, 65535, 0>);
    impl BaroAltThousands {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for BaroAltThousands {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for BaroAltThousands {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Baro Pressure 100s
    /// Output: gauge position
    pub struct BaroPressureHundreds(AnalogGauge<22700, 65535, 0>);
    impl BaroPressureHundreds {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for BaroPressureHundreds {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for BaroPressureHundreds {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Baro Pressure 10s
    /// Output: gauge position
    pub struct BaroPressureTens(AnalogGauge<22698, 65535, 0>);
    impl BaroPressureTens {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for BaroPressureTens {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for BaroPressureTens {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Baro Pressure 1000s
    /// Output: gauge position
    pub struct BaroPressureThousands(AnalogGauge<22702, 65535, 0>);
    impl BaroPressureThousands {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for BaroPressureThousands {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for BaroPressureThousands {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Baro Pressure 1s
    /// Output: gauge position
    pub struct BaroPressureUnits(AnalogGauge<22696, 65535, 0>);
    impl BaroPressureUnits {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for BaroPressureUnits {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for BaroPressureUnits {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Center Console Light
    /// Output: gauge position
    pub struct CenterConsoleLight(AnalogGauge<22690, 65535, 0>);
    impl CenterConsoleLight {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CenterConsoleLight {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CenterConsoleLight {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Clock Hour
    /// Output: gauge position
    pub struct ClockHour(AnalogGauge<22674, 65535, 0>);
    impl ClockHour {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ClockHour {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClockHour {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Clock Mini
    /// Output: gauge position
    pub struct ClockMini(AnalogGauge<22680, 65535, 0>);
    impl ClockMini {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ClockMini {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClockMini {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Clock Minute
    /// Output: gauge position
    pub struct ClockMinute(AnalogGauge<22676, 65535, 0>);
    impl ClockMinute {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ClockMinute {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClockMinute {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Clock Second
    /// Output: gauge position
    pub struct ClockSecond(AnalogGauge<22678, 65535, 0>);
    impl ClockSecond {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ClockSecond {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClockSecond {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Fuel
    /// Output: gauge position
    pub struct Fuel(AnalogGauge<22730, 65535, 0>);
    impl Fuel {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Fuel {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Fuel {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Gyro Compass Heading
    /// Output: gauge position
    pub struct GyroCompassHeading(AnalogGauge<22718, 65535, 0>);
    impl GyroCompassHeading {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for GyroCompassHeading {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for GyroCompassHeading {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Gyro Needle
    /// Output: gauge position
    pub struct GyroNeedle(AnalogGauge<22628, 65535, 0>);
    impl GyroNeedle {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for GyroNeedle {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for GyroNeedle {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Indicated Air Speed IAS
    /// Output: gauge position
    pub struct IndAirSpeedIas(AnalogGauge<22754, 65535, 0>);
    impl IndAirSpeedIas {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IndAirSpeedIas {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IndAirSpeedIas {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Main Panel Light
    /// Output: gauge position
    pub struct MainPanelLight(AnalogGauge<22688, 65535, 0>);
    impl MainPanelLight {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MainPanelLight {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MainPanelLight {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// NADIR Brightness
    /// Output: gauge position
    pub struct NadirBrightness(AnalogGauge<22638, 65535, 0>);
    impl NadirBrightness {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for NadirBrightness {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for NadirBrightness {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Pitch Needle
    /// Output: gauge position
    pub struct PitchNeedle(AnalogGauge<22630, 65535, 0>);
    impl PitchNeedle {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for PitchNeedle {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for PitchNeedle {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Quatre
    /// Output: gauge position
    pub struct Quatre(AnalogGauge<22762, 65535, 0>);
    impl Quatre {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Quatre {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Quatre {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Radar Altimeter Flag On/Off
    /// Output: gauge position
    pub struct RadarAltFlagOnOff(AnalogGauge<22710, 65535, 0>);
    impl RadarAltFlagOnOff {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RadarAltFlagOnOff {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadarAltFlagOnOff {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Radar Altimeter Flag Panne
    /// Output: gauge position
    pub struct RadarAltFlagPenne(AnalogGauge<22708, 65535, 0>);
    impl RadarAltFlagPenne {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RadarAltFlagPenne {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadarAltFlagPenne {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Radar Altimeter Knob
    /// Output: gauge position
    pub struct RadarAltKnob(AnalogGauge<22712, 65535, 0>);
    impl RadarAltKnob {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RadarAltKnob {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadarAltKnob {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Radar Altimeter (raw)
    /// Output: gauge position
    pub struct RadarAltimeter(AnalogGauge<22704, 65535, 0>);
    impl RadarAltimeter {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RadarAltimeter {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadarAltimeter {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Radar Altimeter Bug
    /// Output: gauge position
    pub struct RadarAltimeterBug(AnalogGauge<22706, 65535, 0>);
    impl RadarAltimeterBug {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RadarAltimeterBug {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadarAltimeterBug {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Roll Needle
    /// Output: gauge position
    pub struct RollNeedle(AnalogGauge<22632, 65535, 0>);
    impl RollNeedle {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RollNeedle {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RollNeedle {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Roof Light
    /// Output: gauge position
    pub struct RoofLight(AnalogGauge<22686, 65535, 0>);
    impl RoofLight {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RoofLight {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RoofLight {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Rotor RPM
    /// Output: gauge position
    pub struct RotorRpm(AnalogGauge<22758, 65535, 0>);
    impl RotorRpm {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RotorRpm {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RotorRpm {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Standby Artificial Horizon Curseur
    /// Output: gauge position
    pub struct StbyArtHorCurseur(AnalogGauge<22728, 65535, 0>);
    impl StbyArtHorCurseur {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for StbyArtHorCurseur {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for StbyArtHorCurseur {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Standby Artificial Horizon Flag
    /// Output: gauge position
    pub struct StbyArtHorFlag(AnalogGauge<22724, 65535, 0>);
    impl StbyArtHorFlag {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for StbyArtHorFlag {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for StbyArtHorFlag {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Standby Artificial Horizon Pitch
    /// Output: gauge position
    pub struct StbyArtHorPitch(AnalogGauge<22722, 65535, 0>);
    impl StbyArtHorPitch {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for StbyArtHorPitch {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for StbyArtHorPitch {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Standby Artificial Horizon Roll
    /// Output: gauge position
    pub struct StbyArtHorRoll(AnalogGauge<22720, 65535, 0>);
    impl StbyArtHorRoll {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for StbyArtHorRoll {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for StbyArtHorRoll {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Standby Artificial Horizon W
    /// Output: gauge position
    pub struct StbyArtHorW(AnalogGauge<22726, 65535, 0>);
    impl StbyArtHorW {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for StbyArtHorW {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for StbyArtHorW {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Engine Oil Temperature
    /// Output: gauge position
    pub struct TempEngineOil(AnalogGauge<22766, 65535, 0>);
    impl TempEngineOil {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TempEngineOil {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TempEngineOil {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// External Temperature
    /// Output: gauge position
    pub struct TempExternal(AnalogGauge<22764, 65535, 0>);
    impl TempExternal {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TempExternal {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TempExternal {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Torque Bug
    /// Output: gauge position
    pub struct TorqueBug(AnalogGauge<22716, 65535, 0>);
    impl TorqueBug {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TorqueBug {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TorqueBug {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Torque Needle
    /// Output: gauge position
    pub struct TorqueNeedle(AnalogGauge<22714, 65535, 0>);
    impl TorqueNeedle {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TorqueNeedle {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TorqueNeedle {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Turbine RPM
    /// Output: gauge position
    pub struct TurbineRpm(AnalogGauge<22756, 65535, 0>);
    impl TurbineRpm {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TurbineRpm {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TurbineRpm {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// TV Brightness
    /// Output: gauge position
    pub struct TvBrightnessValue(AnalogGauge<22636, 65535, 0>);
    impl TvBrightnessValue {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TvBrightnessValue {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TvBrightnessValue {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Variometer Vertical Velocity VVI
    /// Output: gauge position
    pub struct VariometerVvi(AnalogGauge<22752, 65535, 0>);
    impl VariometerVvi {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for VariometerVvi {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for VariometerVvi {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Volt Meter
    /// Output: gauge position
    pub struct VoltMeter(AnalogGauge<22760, 65535, 0>);
    impl VoltMeter {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for VoltMeter {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for VoltMeter {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Wipers Copilot
    /// Output: gauge position
    pub struct WipersCopilot(AnalogGauge<22684, 65535, 0>);
    impl WipersCopilot {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for WipersCopilot {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for WipersCopilot {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Wipers Pilot
    /// Output: gauge position
    pub struct WipersPilot(AnalogGauge<22682, 65535, 0>);
    impl WipersPilot {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for WipersPilot {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for WipersPilot {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Yaw Needle
    /// Output: gauge position
    pub struct YawNeedle(AnalogGauge<22634, 65535, 0>);
    impl YawNeedle {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for YawNeedle {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for YawNeedle {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod rotors {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RotorBrakeValue {
        Select1,
        Select2,
    }
    pub type RotorBrakeValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RotorBrakeValue) + 'a>;
    impl core::convert::TryFrom<u16> for RotorBrakeValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Rotor Brake
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RotorBrake(Selector<22554, 1, 0>);
    impl RotorBrake {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RotorBrake {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RotorBrake {
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

    /// RWR Audio Volume
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct RwrAudio(LimitedDial<22606, 65535, 0>);
    impl RwrAudio {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrAudio {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrAudio {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RWR Brightness
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct RwrBrightness(LimitedDial<22608, 65535, 0>);
    impl RwrBrightness {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrBrightness {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrBrightness {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RwrMarkerValue {
        Select1,
        Select2,
    }
    pub type RwrMarkerValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RwrMarkerValue) + 'a>;
    impl core::convert::TryFrom<u16> for RwrMarkerValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// RWR Marker
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RwrMarker(Selector<22604, 4096, 12>);
    impl RwrMarker {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrMarker {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrMarker {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RwrOffOnCrocValue {
        Select1,
        Select2,
        Select3,
    }
    pub type RwrOffOnCrocValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RwrOffOnCrocValue) + 'a>;
    impl core::convert::TryFrom<u16> for RwrOffOnCrocValue {
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

    /// RWR Off/On/Croc
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct RwrOffOnCroc(Selector<22604, 3072, 10>);
    impl RwrOffOnCroc {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrOffOnCroc {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrOffOnCroc {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RwrPageValue {
        Select1,
        Select2,
    }
    pub type RwrPageValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RwrPageValue) + 'a>;
    impl core::convert::TryFrom<u16> for RwrPageValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// RWR Page
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RwrPage(Selector<22604, 8192, 13>);
    impl RwrPage {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RwrPage {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RwrPage {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod torque {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Torque Bug Adjust
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct TorqueBugAdjust(AnalogDial<22576, 65535, 0>);
    impl TorqueBugAdjust {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TorqueBugAdjust {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TorqueBugAdjust {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum TorqueTestValue {
        Select1,
        Select2,
    }
    pub type TorqueTestValueCallback<'a> = alloc::boxed::Box<dyn FnMut(TorqueTestValue) + 'a>;
    impl core::convert::TryFrom<u16> for TorqueTestValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Torque Test
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct TorqueTest(Selector<22574, 128, 7>);
    impl TorqueTest {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TorqueTest {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TorqueTest {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod tv {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// TV Brightness
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct TvBrightness(LimitedDial<22558, 65535, 0>);
    impl TvBrightness {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TvBrightness {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TvBrightness {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// TV Contrast
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct TvContrast(LimitedDial<22556, 65535, 0>);
    impl TvContrast {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TvContrast {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TvContrast {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum TvCoverValue {
        Select1,
        Select2,
    }
    pub type TvCoverValueCallback<'a> = alloc::boxed::Box<dyn FnMut(TvCoverValue) + 'a>;
    impl core::convert::TryFrom<u16> for TvCoverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// TV Cover
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct TvCover(Selector<22554, 4, 2>);
    impl TvCover {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TvCover {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TvCover {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum TvOnOffValue {
        Select1,
        Select2,
    }
    pub type TvOnOffValueCallback<'a> = alloc::boxed::Box<dyn FnMut(TvOnOffValue) + 'a>;
    impl core::convert::TryFrom<u16> for TvOnOffValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// TV On/Off
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct TvOnOff(Selector<22554, 2, 1>);
    impl TvOnOff {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TvOnOff {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TvOnOff {
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

    /// UHF Radio Page Knob
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct UhfPageKnob(AnalogDial<22620, 65535, 0>);
    impl UhfPageKnob {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfPageKnob {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfPageKnob {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UhfRadioButton0Value {
        Select1,
        Select2,
    }
    pub type UhfRadioButton0ValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(UhfRadioButton0Value) + 'a>;
    impl core::convert::TryFrom<u16> for UhfRadioButton0Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// UHF Radio Button 0
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UhfRadioButton0(Selector<22622, 512, 9>);
    impl UhfRadioButton0 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfRadioButton0 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfRadioButton0 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UhfRadioButton1Value {
        Select1,
        Select2,
    }
    pub type UhfRadioButton1ValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(UhfRadioButton1Value) + 'a>;
    impl core::convert::TryFrom<u16> for UhfRadioButton1Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// UHF Radio Button 1
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UhfRadioButton1(Selector<22622, 1, 0>);
    impl UhfRadioButton1 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfRadioButton1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfRadioButton1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UhfRadioButton2Value {
        Select1,
        Select2,
    }
    pub type UhfRadioButton2ValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(UhfRadioButton2Value) + 'a>;
    impl core::convert::TryFrom<u16> for UhfRadioButton2Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// UHF Radio Button 2
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UhfRadioButton2(Selector<22622, 2, 1>);
    impl UhfRadioButton2 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfRadioButton2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfRadioButton2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UhfRadioButton3Value {
        Select1,
        Select2,
    }
    pub type UhfRadioButton3ValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(UhfRadioButton3Value) + 'a>;
    impl core::convert::TryFrom<u16> for UhfRadioButton3Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// UHF Radio Button 3
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UhfRadioButton3(Selector<22622, 4, 2>);
    impl UhfRadioButton3 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfRadioButton3 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfRadioButton3 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UhfRadioButton4Value {
        Select1,
        Select2,
    }
    pub type UhfRadioButton4ValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(UhfRadioButton4Value) + 'a>;
    impl core::convert::TryFrom<u16> for UhfRadioButton4Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// UHF Radio Button 4
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UhfRadioButton4(Selector<22622, 8, 3>);
    impl UhfRadioButton4 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfRadioButton4 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfRadioButton4 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UhfRadioButton5Value {
        Select1,
        Select2,
    }
    pub type UhfRadioButton5ValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(UhfRadioButton5Value) + 'a>;
    impl core::convert::TryFrom<u16> for UhfRadioButton5Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// UHF Radio Button 5
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UhfRadioButton5(Selector<22622, 16, 4>);
    impl UhfRadioButton5 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfRadioButton5 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfRadioButton5 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UhfRadioButton6Value {
        Select1,
        Select2,
    }
    pub type UhfRadioButton6ValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(UhfRadioButton6Value) + 'a>;
    impl core::convert::TryFrom<u16> for UhfRadioButton6Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// UHF Radio Button 6
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UhfRadioButton6(Selector<22622, 32, 5>);
    impl UhfRadioButton6 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfRadioButton6 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfRadioButton6 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UhfRadioButton7Value {
        Select1,
        Select2,
    }
    pub type UhfRadioButton7ValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(UhfRadioButton7Value) + 'a>;
    impl core::convert::TryFrom<u16> for UhfRadioButton7Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// UHF Radio Button 7
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UhfRadioButton7(Selector<22622, 64, 6>);
    impl UhfRadioButton7 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfRadioButton7 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfRadioButton7 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UhfRadioButton8Value {
        Select1,
        Select2,
    }
    pub type UhfRadioButton8ValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(UhfRadioButton8Value) + 'a>;
    impl core::convert::TryFrom<u16> for UhfRadioButton8Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// UHF Radio Button 8
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UhfRadioButton8(Selector<22622, 128, 7>);
    impl UhfRadioButton8 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfRadioButton8 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfRadioButton8 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UhfRadioButton9Value {
        Select1,
        Select2,
    }
    pub type UhfRadioButton9ValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(UhfRadioButton9Value) + 'a>;
    impl core::convert::TryFrom<u16> for UhfRadioButton9Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// UHF Radio Button 9
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UhfRadioButton9(Selector<22622, 256, 8>);
    impl UhfRadioButton9 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfRadioButton9 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfRadioButton9 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UhfRadioButtonConfValue {
        Select1,
        Select2,
    }
    pub type UhfRadioButtonConfValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(UhfRadioButtonConfValue) + 'a>;
    impl core::convert::TryFrom<u16> for UhfRadioButtonConfValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// UHF Radio Button CONF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UhfRadioButtonConf(Selector<22618, 32768, 15>);
    impl UhfRadioButtonConf {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfRadioButtonConf {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfRadioButtonConf {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UhfRadioButtonDrwValue {
        Select1,
        Select2,
    }
    pub type UhfRadioButtonDrwValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(UhfRadioButtonDrwValue) + 'a>;
    impl core::convert::TryFrom<u16> for UhfRadioButtonDrwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// UHF Radio Button DRW
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UhfRadioButtonDrw(Selector<22618, 8192, 13>);
    impl UhfRadioButtonDrw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfRadioButtonDrw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfRadioButtonDrw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UhfRadioButtonVldValue {
        Select1,
        Select2,
    }
    pub type UhfRadioButtonVldValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(UhfRadioButtonVldValue) + 'a>;
    impl core::convert::TryFrom<u16> for UhfRadioButtonVldValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// UHF Radio Button VLD
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UhfRadioButtonVld(Selector<22618, 16384, 14>);
    impl UhfRadioButtonVld {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfRadioButtonVld {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfRadioButtonVld {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UhfRadioModeValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
        Select6,
        Select7,
    }
    pub type UhfRadioModeValueCallback<'a> = alloc::boxed::Box<dyn FnMut(UhfRadioModeValue) + 'a>;
    impl core::convert::TryFrom<u16> for UhfRadioModeValue {
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

    /// UHF Radio Mode Select
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct UhfRadioMode(Selector<22618, 7168, 10>);
    impl UhfRadioMode {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfRadioMode {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfRadioMode {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod warningpanel {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// AC 115 V Bus/NAV Warning
    /// Output: 0 if light is off, 1 if light is on
    pub struct Ac115vBusNav(Led<22626, 2048, 11>);
    impl Ac115vBusNav {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for Ac115vBusNav {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Ac115vBusNav {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Alternator Warning
    /// Output: 0 if light is off, 1 if light is on
    pub struct AlternatorWarning(Led<22626, 256, 8>);
    impl AlternatorWarning {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for AlternatorWarning {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AlternatorWarning {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Autopilot Warning
    /// Output: 0 if light is off, 1 if light is on
    pub struct Autopilot(Led<22626, 1024, 10>);
    impl Autopilot {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for Autopilot {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Autopilot {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Battery Warning
    /// Output: 0 if light is off, 1 if light is on
    pub struct BatteryWarning(Led<22626, 512, 9>);
    impl BatteryWarning {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for BatteryWarning {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for BatteryWarning {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Chips In Oil Warning
    /// Output: 0 if light is off, 1 if light is on
    pub struct ChipsInOil(Led<22626, 16384, 14>);
    impl ChipsInOil {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ChipsInOil {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ChipsInOil {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Generator Warning
    /// Output: 0 if light is off, 1 if light is on
    pub struct GeneratorWarning(Led<22626, 128, 7>);
    impl GeneratorWarning {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for GeneratorWarning {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for GeneratorWarning {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Low Fuel Warning
    /// Output: 0 if light is off, 1 if light is on
    pub struct LowFuel(Led<22626, 4096, 12>);
    impl LowFuel {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LowFuel {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LowFuel {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Low Rotor RPM Warning
    /// Output: 0 if light is off, 1 if light is on
    pub struct LowRotorRpm(Led<22626, 8192, 13>);
    impl LowRotorRpm {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LowRotorRpm {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LowRotorRpm {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Low Turbine RPM Warning
    /// Output: 0 if light is off, 1 if light is on
    pub struct LowTurbineRpm(Led<22626, 64, 6>);
    impl LowTurbineRpm {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LowTurbineRpm {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LowTurbineRpm {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Pitot Tube Warning
    /// Output: 0 if light is off, 1 if light is on
    pub struct PitotTubeWarning(Led<22626, 8, 3>);
    impl PitotTubeWarning {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for PitotTubeWarning {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for PitotTubeWarning {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Main Rotor Oil Pressure Warning
    /// Output: 0 if light is off, 1 if light is on
    pub struct RotorOilPressure(Led<22626, 32, 5>);
    impl RotorOilPressure {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RotorOilPressure {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RotorOilPressure {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Sand Filter Warning
    /// Output: 0 if light is off, 1 if light is on
    pub struct SandFilterWarning(Led<22626, 32768, 15>);
    impl SandFilterWarning {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for SandFilterWarning {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SandFilterWarning {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Turbine Oil Pressure Warning
    /// Output: 0 if light is off, 1 if light is on
    pub struct TurbineOilPressure(Led<22626, 16, 4>);
    impl TurbineOilPressure {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for TurbineOilPressure {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TurbineOilPressure {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod weapons {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LeftMasterArmValue {
        Select1,
        Select2,
    }
    pub type LeftMasterArmValueCallback<'a> = alloc::boxed::Box<dyn FnMut(LeftMasterArmValue) + 'a>;
    impl core::convert::TryFrom<u16> for LeftMasterArmValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Left Master Arm On/Off
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LeftMasterArm(Selector<22610, 128, 7>);
    impl LeftMasterArm {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LeftMasterArm {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LeftMasterArm {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LeftMasterArmCoverValue {
        Select1,
        Select2,
    }
    pub type LeftMasterArmCoverValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(LeftMasterArmCoverValue) + 'a>;
    impl core::convert::TryFrom<u16> for LeftMasterArmCoverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Left Master Arm Cover
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LeftMasterArmCover(Selector<22610, 256, 8>);
    impl LeftMasterArmCover {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LeftMasterArmCover {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LeftMasterArmCover {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum PilotSightValue {
        Select1,
        Select2,
    }
    pub type PilotSightValueCallback<'a> = alloc::boxed::Box<dyn FnMut(PilotSightValue) + 'a>;
    impl core::convert::TryFrom<u16> for PilotSightValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pilot Sight
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct PilotSight(Selector<22610, 4096, 12>);
    impl PilotSight {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for PilotSight {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for PilotSight {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RightMasterArmValue {
        Select1,
        Select2,
    }
    pub type RightMasterArmValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(RightMasterArmValue) + 'a>;
    impl core::convert::TryFrom<u16> for RightMasterArmValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Right Master Arm On/Off
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RightMasterArm(Selector<22610, 512, 9>);
    impl RightMasterArm {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RightMasterArm {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RightMasterArm {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RightMasterArmCoverValue {
        Select1,
        Select2,
    }
    pub type RightMasterArmCoverValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(RightMasterArmCoverValue) + 'a>;
    impl core::convert::TryFrom<u16> for RightMasterArmCoverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Right Master Arm Cover
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RightMasterArmCover(Selector<22610, 1024, 10>);
    impl RightMasterArmCover {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RightMasterArmCover {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RightMasterArmCover {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum WeaponLaunchSeqValue {
        Select1,
        Select2,
    }
    pub type WeaponLaunchSeqValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(WeaponLaunchSeqValue) + 'a>;
    impl core::convert::TryFrom<u16> for WeaponLaunchSeqValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Weapon Launch Sequence
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct WeaponLaunchSeq(Selector<22610, 2048, 11>);
    impl WeaponLaunchSeq {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for WeaponLaunchSeq {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for WeaponLaunchSeq {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum WeaponsMasterArmValue {
        Select1,
        Select2,
    }
    pub type WeaponsMasterArmValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(WeaponsMasterArmValue) + 'a>;
    impl core::convert::TryFrom<u16> for WeaponsMasterArmValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Weapons Master Arm On/Off
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct WeaponsMasterArm(Selector<22610, 8192, 13>);
    impl WeaponsMasterArm {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for WeaponsMasterArm {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for WeaponsMasterArm {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Weapons Panel 1 Display Brightness
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct WeaponsPanel1DispBright(LimitedDial<22612, 65535, 0>);
    impl WeaponsPanel1DispBright {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for WeaponsPanel1DispBright {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for WeaponsPanel1DispBright {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum WeaponsPanel1OffOnStbyValue {
        Select1,
        Select2,
        Select3,
    }
    pub type WeaponsPanel1OffOnStbyValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(WeaponsPanel1OffOnStbyValue) + 'a>;
    impl core::convert::TryFrom<u16> for WeaponsPanel1OffOnStbyValue {
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

    /// Weapons Panel 1 Off/On/Standby
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct WeaponsPanel1OffOnStby(Selector<22610, 96, 5>);
    impl WeaponsPanel1OffOnStby {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for WeaponsPanel1OffOnStby {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for WeaponsPanel1OffOnStby {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub enum RegisteredControl<'a> {
    Ac115vBusNav {
        control: warningpanel::Ac115vBusNav,
        callback: LedCallback<'a>,
    },
    AdditionalFuelTank {
        control: electricsystem::AdditionalFuelTank,
        callback: electricsystem::AdditionalFuelTankValueCallback<'a>,
    },
    AdditionalFuelTankLamp {
        control: indicatorlights::AdditionalFuelTankLamp,
        callback: LedCallback<'a>,
    },
    Adf1Adf2Select {
        control: adf::Adf1Adf2Select,
        callback: adf::Adf1Adf2SelectValueCallback<'a>,
    },
    AdfBackground {
        control: rawgaugevalues::AdfBackground,
        callback: IntegerCallback<'a>,
    },
    AdfDistance100s {
        control: rawgaugevalues::AdfDistance100s,
        callback: IntegerCallback<'a>,
    },
    AdfDistance10s {
        control: rawgaugevalues::AdfDistance10s,
        callback: IntegerCallback<'a>,
    },
    AdfDistance1s {
        control: rawgaugevalues::AdfDistance1s,
        callback: IntegerCallback<'a>,
    },
    AdfDistanceValue {
        control: gaugevalues::AdfDistanceValue,
        callback: IntegerCallback<'a>,
    },
    AdfFlagBut {
        control: rawgaugevalues::AdfFlagBut,
        callback: IntegerCallback<'a>,
    },
    AdfFlagCap {
        control: rawgaugevalues::AdfFlagCap,
        callback: IntegerCallback<'a>,
    },
    AdfFlagCounter {
        control: rawgaugevalues::AdfFlagCounter,
        callback: IntegerCallback<'a>,
    },
    AdfGain {
        control: adf::AdfGain,
        callback: IntegerCallback<'a>,
    },
    AdfHeadingValue {
        control: gaugevalues::AdfHeadingValue,
        callback: IntegerCallback<'a>,
    },
    AdfMode {
        control: adf::AdfMode,
        callback: adf::AdfModeValueCallback<'a>,
    },
    AdfNadirNeedle {
        control: rawgaugevalues::AdfNadirNeedle,
        callback: IntegerCallback<'a>,
    },
    AdfNav11 {
        control: adf::AdfNav11,
        callback: IntegerCallback<'a>,
    },
    AdfNav110 {
        control: adf::AdfNav110,
        callback: IntegerCallback<'a>,
    },
    AdfNav1100 {
        control: adf::AdfNav1100,
        callback: IntegerCallback<'a>,
    },
    AdfNav1Freq100s {
        control: rawgaugevalues::AdfNav1Freq100s,
        callback: IntegerCallback<'a>,
    },
    AdfNav1Freq10s {
        control: rawgaugevalues::AdfNav1Freq10s,
        callback: IntegerCallback<'a>,
    },
    AdfNav1Freq1s {
        control: rawgaugevalues::AdfNav1Freq1s,
        callback: IntegerCallback<'a>,
    },
    AdfNav1FreqDecimal {
        control: rawgaugevalues::AdfNav1FreqDecimal,
        callback: IntegerCallback<'a>,
    },
    AdfNav21 {
        control: adf::AdfNav21,
        callback: IntegerCallback<'a>,
    },
    AdfNav210 {
        control: adf::AdfNav210,
        callback: IntegerCallback<'a>,
    },
    AdfNav2100 {
        control: adf::AdfNav2100,
        callback: IntegerCallback<'a>,
    },
    AdfNav2Freq100s {
        control: rawgaugevalues::AdfNav2Freq100s,
        callback: IntegerCallback<'a>,
    },
    AdfNav2Freq10s {
        control: rawgaugevalues::AdfNav2Freq10s,
        callback: IntegerCallback<'a>,
    },
    AdfNav2Freq1s {
        control: rawgaugevalues::AdfNav2Freq1s,
        callback: IntegerCallback<'a>,
    },
    AdfNav2FreqDecimal {
        control: rawgaugevalues::AdfNav2FreqDecimal,
        callback: IntegerCallback<'a>,
    },
    AdfSmallNeedle {
        control: rawgaugevalues::AdfSmallNeedle,
        callback: IntegerCallback<'a>,
    },
    AdfToneOnOff {
        control: adf::AdfToneOnOff,
        callback: adf::AdfToneOnOffValueCallback<'a>,
    },
    AlarmWarningLamp {
        control: indicatorlights::AlarmWarningLamp,
        callback: LedCallback<'a>,
    },
    Alternator {
        control: electricsystem::Alternator,
        callback: electricsystem::AlternatorValueCallback<'a>,
    },
    AlternatorWarning {
        control: warningpanel::AlternatorWarning,
        callback: LedCallback<'a>,
    },
    AmFrequencySelector {
        control: amradio::AmFrequencySelector,
        callback: amradio::AmFrequencySelectorValueCallback<'a>,
    },
    AmRadioFreq100s {
        control: rawgaugevalues::AmRadioFreq100s,
        callback: IntegerCallback<'a>,
    },
    AmRadioFreq10s {
        control: rawgaugevalues::AmRadioFreq10s,
        callback: IntegerCallback<'a>,
    },
    AmRadioFreq1s {
        control: rawgaugevalues::AmRadioFreq1s,
        callback: IntegerCallback<'a>,
    },
    AmRadioFreqHundredths {
        control: rawgaugevalues::AmRadioFreqHundredths,
        callback: IntegerCallback<'a>,
    },
    AmRadioFreqTenths {
        control: rawgaugevalues::AmRadioFreqTenths,
        callback: IntegerCallback<'a>,
    },
    AmRadioFrequencyDialLeft {
        control: amradio::AmRadioFrequencyDialLeft,
        callback: IntegerCallback<'a>,
    },
    AmRadioFrequencyDialRight {
        control: amradio::AmRadioFrequencyDialRight,
        callback: IntegerCallback<'a>,
    },
    AmRadioLamp {
        control: indicatorlights::AmRadioLamp,
        callback: LedCallback<'a>,
    },
    AmRadioSelector {
        control: amradio::AmRadioSelector,
        callback: amradio::AmRadioSelectorValueCallback<'a>,
    },
    AnticollIntensity {
        control: navlights::AnticollIntensity,
        callback: IntegerCallback<'a>,
    },
    AnticollLights {
        control: navlights::AnticollLights,
        callback: navlights::AnticollLightsValueCallback<'a>,
    },
    ArtHorizBall {
        control: rawgaugevalues::ArtHorizBall,
        callback: IntegerCallback<'a>,
    },
    ArtHorizCurseur {
        control: rawgaugevalues::ArtHorizCurseur,
        callback: IntegerCallback<'a>,
    },
    ArtHorizFlagGs {
        control: rawgaugevalues::ArtHorizFlagGs,
        callback: IntegerCallback<'a>,
    },
    ArtHorizFlagHs {
        control: rawgaugevalues::ArtHorizFlagHs,
        callback: IntegerCallback<'a>,
    },
    ArtHorizFlagLoc {
        control: rawgaugevalues::ArtHorizFlagLoc,
        callback: IntegerCallback<'a>,
    },
    ArtHorizHorizontalBar {
        control: rawgaugevalues::ArtHorizHorizontalBar,
        callback: IntegerCallback<'a>,
    },
    ArtHorizPitch {
        control: rawgaugevalues::ArtHorizPitch,
        callback: IntegerCallback<'a>,
    },
    ArtHorizRoll {
        control: rawgaugevalues::ArtHorizRoll,
        callback: IntegerCallback<'a>,
    },
    ArtHorizVertBar {
        control: rawgaugevalues::ArtHorizVertBar,
        callback: IntegerCallback<'a>,
    },
    ArtHorizW {
        control: rawgaugevalues::ArtHorizW,
        callback: IntegerCallback<'a>,
    },
    AutoHover {
        control: autopilot::AutoHover,
        callback: autopilot::AutoHoverValueCallback<'a>,
    },
    Autopilot {
        control: warningpanel::Autopilot,
        callback: LedCallback<'a>,
    },
    AutopilotButton {
        control: autopilot::AutopilotButton,
        callback: autopilot::AutopilotButtonValueCallback<'a>,
    },
    AutopilotMode {
        control: autopilot::AutopilotMode,
        callback: autopilot::AutopilotModeValueCallback<'a>,
    },
    AutopilotOnOff {
        control: autopilot::AutopilotOnOff,
        callback: autopilot::AutopilotOnOffValueCallback<'a>,
    },
    AutopilotPitch {
        control: autopilot::AutopilotPitch,
        callback: autopilot::AutopilotPitchValueCallback<'a>,
    },
    AutopilotRoll {
        control: autopilot::AutopilotRoll,
        callback: autopilot::AutopilotRollValueCallback<'a>,
    },
    AutopilotSlave {
        control: autopilot::AutopilotSlave,
        callback: autopilot::AutopilotSlaveValueCallback<'a>,
    },
    AutopilotTrim {
        control: autopilot::AutopilotTrim,
        callback: autopilot::AutopilotTrimValueCallback<'a>,
    },
    AutopilotYaw {
        control: autopilot::AutopilotYaw,
        callback: autopilot::AutopilotYawValueCallback<'a>,
    },
    AuxFuelTank {
        control: rawgaugevalues::AuxFuelTank,
        callback: IntegerCallback<'a>,
    },
    BaroAltHundreds {
        control: rawgaugevalues::BaroAltHundreds,
        callback: IntegerCallback<'a>,
    },
    BaroAltThousands {
        control: rawgaugevalues::BaroAltThousands,
        callback: IntegerCallback<'a>,
    },
    BaroAltValue {
        control: gaugevalues::BaroAltValue,
        callback: IntegerCallback<'a>,
    },
    BaroPressValue {
        control: gaugevalues::BaroPressValue,
        callback: IntegerCallback<'a>,
    },
    BaroPressureHundreds {
        control: rawgaugevalues::BaroPressureHundreds,
        callback: IntegerCallback<'a>,
    },
    BaroPressureTens {
        control: rawgaugevalues::BaroPressureTens,
        callback: IntegerCallback<'a>,
    },
    BaroPressureThousands {
        control: rawgaugevalues::BaroPressureThousands,
        callback: IntegerCallback<'a>,
    },
    BaroPressureUnits {
        control: rawgaugevalues::BaroPressureUnits,
        callback: IntegerCallback<'a>,
    },
    BarometricAltimeter {
        control: baroaltimeter::BarometricAltimeter,
        callback: IntegerCallback<'a>,
    },
    Battery {
        control: electricsystem::Battery,
        callback: electricsystem::BatteryValueCallback<'a>,
    },
    BatteryWarning {
        control: warningpanel::BatteryWarning,
        callback: LedCallback<'a>,
    },
    BlockedEngineWarningLamp {
        control: indicatorlights::BlockedEngineWarningLamp,
        callback: LedCallback<'a>,
    },
    CameraCentering {
        control: pe::CameraCentering,
        callback: pe::CameraCenteringValueCallback<'a>,
    },
    CameraMode {
        control: pe::CameraMode,
        callback: pe::CameraModeValueCallback<'a>,
    },
    CameraZoomIn {
        control: pe::CameraZoomIn,
        callback: pe::CameraZoomInValueCallback<'a>,
    },
    CameraZoomOut {
        control: pe::CameraZoomOut,
        callback: pe::CameraZoomOutValueCallback<'a>,
    },
    CenterConsoleLight {
        control: rawgaugevalues::CenterConsoleLight,
        callback: IntegerCallback<'a>,
    },
    ChipsInOil {
        control: warningpanel::ChipsInOil,
        callback: LedCallback<'a>,
    },
    ClockHour {
        control: rawgaugevalues::ClockHour,
        callback: IntegerCallback<'a>,
    },
    ClockMini {
        control: rawgaugevalues::ClockMini,
        callback: IntegerCallback<'a>,
    },
    ClockMinute {
        control: rawgaugevalues::ClockMinute,
        callback: IntegerCallback<'a>,
    },
    ClockPushButton {
        control: clock::ClockPushButton,
        callback: clock::ClockPushButtonValueCallback<'a>,
    },
    ClockReset {
        control: clock::ClockReset,
        callback: clock::ClockResetValueCallback<'a>,
    },
    ClockSecond {
        control: rawgaugevalues::ClockSecond,
        callback: IntegerCallback<'a>,
    },
    ClockSet {
        control: clock::ClockSet,
        callback: IntegerCallback<'a>,
    },
    ClockStartStop {
        control: clock::ClockStartStop,
        callback: clock::ClockStartStopValueCallback<'a>,
    },
    ConsoleLighting {
        control: lights::ConsoleLighting,
        callback: IntegerCallback<'a>,
    },
    ConvoyFuelTankLamp {
        control: indicatorlights::ConvoyFuelTankLamp,
        callback: LedCallback<'a>,
    },
    ConvoyTank {
        control: electricsystem::ConvoyTank,
        callback: electricsystem::ConvoyTankValueCallback<'a>,
    },
    CopilotWiper {
        control: electricsystem::CopilotWiper,
        callback: electricsystem::CopilotWiperValueCallback<'a>,
    },
    CthPower {
        control: pe::CthPower,
        callback: pe::CthPowerValueCallback<'a>,
    },
    DashboardLighting {
        control: lights::DashboardLighting,
        callback: IntegerCallback<'a>,
    },
    DemStartLamp {
        control: indicatorlights::DemStartLamp,
        callback: LedCallback<'a>,
    },
    ExtPositionLightLeft {
        control: externalaircraftmodel::ExtPositionLightLeft,
        callback: IntegerCallback<'a>,
    },
    ExtPositionLightRight {
        control: externalaircraftmodel::ExtPositionLightRight,
        callback: IntegerCallback<'a>,
    },
    ExtStrobe {
        control: externalaircraftmodel::ExtStrobe,
        callback: IntegerCallback<'a>,
    },
    FdLampDOpe {
        control: indicatorlights::FdLampDOpe,
        callback: LedCallback<'a>,
    },
    FdLampDVide1 {
        control: indicatorlights::FdLampDVide1,
        callback: LedCallback<'a>,
    },
    FdLampDVide2 {
        control: indicatorlights::FdLampDVide2,
        callback: LedCallback<'a>,
    },
    FdLampGOpe {
        control: indicatorlights::FdLampGOpe,
        callback: LedCallback<'a>,
    },
    FdLampGVide1 {
        control: indicatorlights::FdLampGVide1,
        callback: LedCallback<'a>,
    },
    FdLampGVide2 {
        control: indicatorlights::FdLampGVide2,
        callback: LedCallback<'a>,
    },
    FdLampLeu {
        control: indicatorlights::FdLampLeu,
        callback: LedCallback<'a>,
    },
    FdLampVe {
        control: indicatorlights::FdLampVe,
        callback: LedCallback<'a>,
    },
    FlareDispFireCap {
        control: flaredispenser::FlareDispFireCap,
        callback: flaredispenser::FlareDispFireCapValueCallback<'a>,
    },
    FlareDispSideSelect {
        control: flaredispenser::FlareDispSideSelect,
        callback: flaredispenser::FlareDispSideSelectValueCallback<'a>,
    },
    FlareDispenseButton {
        control: flaredispenser::FlareDispenseButton,
        callback: flaredispenser::FlareDispenseButtonValueCallback<'a>,
    },
    FlareDispenseMode {
        control: flaredispenser::FlareDispenseMode,
        callback: flaredispenser::FlareDispenseModeValueCallback<'a>,
    },
    FlareDispenserOffOn {
        control: flaredispenser::FlareDispenserOffOn,
        callback: flaredispenser::FlareDispenserOffOnValueCallback<'a>,
    },
    FmRadioButton0 {
        control: fmradio::FmRadioButton0,
        callback: fmradio::FmRadioButton0ValueCallback<'a>,
    },
    FmRadioButton1 {
        control: fmradio::FmRadioButton1,
        callback: fmradio::FmRadioButton1ValueCallback<'a>,
    },
    FmRadioButton2 {
        control: fmradio::FmRadioButton2,
        callback: fmradio::FmRadioButton2ValueCallback<'a>,
    },
    FmRadioButton3 {
        control: fmradio::FmRadioButton3,
        callback: fmradio::FmRadioButton3ValueCallback<'a>,
    },
    FmRadioButton4 {
        control: fmradio::FmRadioButton4,
        callback: fmradio::FmRadioButton4ValueCallback<'a>,
    },
    FmRadioButton5 {
        control: fmradio::FmRadioButton5,
        callback: fmradio::FmRadioButton5ValueCallback<'a>,
    },
    FmRadioButton6 {
        control: fmradio::FmRadioButton6,
        callback: fmradio::FmRadioButton6ValueCallback<'a>,
    },
    FmRadioButton7 {
        control: fmradio::FmRadioButton7,
        callback: fmradio::FmRadioButton7ValueCallback<'a>,
    },
    FmRadioButton8 {
        control: fmradio::FmRadioButton8,
        callback: fmradio::FmRadioButton8ValueCallback<'a>,
    },
    FmRadioButton9 {
        control: fmradio::FmRadioButton9,
        callback: fmradio::FmRadioButton9ValueCallback<'a>,
    },
    FmRadioButtonDown {
        control: fmradio::FmRadioButtonDown,
        callback: fmradio::FmRadioButtonDownValueCallback<'a>,
    },
    FmRadioButtonRc {
        control: fmradio::FmRadioButtonRc,
        callback: fmradio::FmRadioButtonRcValueCallback<'a>,
    },
    FmRadioButtonUp {
        control: fmradio::FmRadioButtonUp,
        callback: fmradio::FmRadioButtonUpValueCallback<'a>,
    },
    FmRadioButtonVal {
        control: fmradio::FmRadioButtonVal,
        callback: fmradio::FmRadioButtonValValueCallback<'a>,
    },
    FmRadioButtonX {
        control: fmradio::FmRadioButtonX,
        callback: fmradio::FmRadioButtonXValueCallback<'a>,
    },
    FmRadioChannel {
        control: fmradio::FmRadioChannel,
        callback: fmradio::FmRadioChannelValueCallback<'a>,
    },
    FmRadioMode {
        control: fmradio::FmRadioMode,
        callback: fmradio::FmRadioModeValueCallback<'a>,
    },
    FmRadioVolume1 {
        control: intercom::FmRadioVolume1,
        callback: IntegerCallback<'a>,
    },
    FmRadioVolume2 {
        control: intercom::FmRadioVolume2,
        callback: IntegerCallback<'a>,
    },
    FormationIntensity {
        control: navlights::FormationIntensity,
        callback: IntegerCallback<'a>,
    },
    FormationLights {
        control: navlights::FormationLights,
        callback: navlights::FormationLightsValueCallback<'a>,
    },
    Fuel {
        control: rawgaugevalues::Fuel,
        callback: IntegerCallback<'a>,
    },
    FuelFlowLever {
        control: fuel::FuelFlowLever,
        callback: fuel::FuelFlowLeverValueCallback<'a>,
    },
    FuelLevelValue {
        control: gaugevalues::FuelLevelValue,
        callback: IntegerCallback<'a>,
    },
    FuelPump {
        control: electricsystem::FuelPump,
        callback: electricsystem::FuelPumpValueCallback<'a>,
    },
    Generator {
        control: electricsystem::Generator,
        callback: electricsystem::GeneratorValueCallback<'a>,
    },
    GeneratorWarning {
        control: warningpanel::GeneratorWarning,
        callback: LedCallback<'a>,
    },
    GyroBppWarningLamp {
        control: indicatorlights::GyroBppWarningLamp,
        callback: LedCallback<'a>,
    },
    GyroCompassHeading {
        control: rawgaugevalues::GyroCompassHeading,
        callback: IntegerCallback<'a>,
    },
    GyroKnob {
        control: autopilot::GyroKnob,
        callback: autopilot::GyroKnobValueCallback<'a>,
    },
    GyroNeedle {
        control: rawgaugevalues::GyroNeedle,
        callback: IntegerCallback<'a>,
    },
    GyroSwitch {
        control: autopilot::GyroSwitch,
        callback: autopilot::GyroSwitchValueCallback<'a>,
    },
    GyroTestCover {
        control: autopilot::GyroTestCover,
        callback: autopilot::GyroTestCoverValueCallback<'a>,
    },
    GyroTestLamp {
        control: indicatorlights::GyroTestLamp,
        callback: LedCallback<'a>,
    },
    GyroTestSwitch {
        control: autopilot::GyroTestSwitch,
        callback: autopilot::GyroTestSwitchValueCallback<'a>,
    },
    GyroTrimWarningLamp {
        control: indicatorlights::GyroTrimWarningLamp,
        callback: LedCallback<'a>,
    },
    HaAdjust {
        control: flightcontrols::HaAdjust,
        callback: IntegerCallback<'a>,
    },
    HaSource {
        control: flightcontrols::HaSource,
        callback: flightcontrols::HaSourceValueCallback<'a>,
    },
    HaUnlock {
        control: flightcontrols::HaUnlock,
        callback: flightcontrols::HaUnlockValueCallback<'a>,
    },
    HeadingValue {
        control: gaugevalues::HeadingValue,
        callback: IntegerCallback<'a>,
    },
    Hot3Brightness {
        control: hot3::Hot3Brightness,
        callback: IntegerCallback<'a>,
    },
    Hot3PanelTestOffOn {
        control: hot3::Hot3PanelTestOffOn,
        callback: hot3::Hot3PanelTestOffOnValueCallback<'a>,
    },
    Hot3StationSelect {
        control: hot3::Hot3StationSelect,
        callback: hot3::Hot3StationSelectValueCallback<'a>,
    },
    HydTest {
        control: electricsystem::HydTest,
        callback: electricsystem::HydTestValueCallback<'a>,
    },
    IasValue {
        control: gaugevalues::IasValue,
        callback: IntegerCallback<'a>,
    },
    ImageBrightnessDown {
        control: pe::ImageBrightnessDown,
        callback: pe::ImageBrightnessDownValueCallback<'a>,
    },
    ImageBrightnessUp {
        control: pe::ImageBrightnessUp,
        callback: pe::ImageBrightnessUpValueCallback<'a>,
    },
    ImageFocusDown {
        control: pe::ImageFocusDown,
        callback: pe::ImageFocusDownValueCallback<'a>,
    },
    ImageFocusUp {
        control: pe::ImageFocusUp,
        callback: pe::ImageFocusUpValueCallback<'a>,
    },
    ImageGainLeft {
        control: pe::ImageGainLeft,
        callback: pe::ImageGainLeftValueCallback<'a>,
    },
    ImageGainRight {
        control: pe::ImageGainRight,
        callback: pe::ImageGainRightValueCallback<'a>,
    },
    IndAirSpeedIas {
        control: rawgaugevalues::IndAirSpeedIas,
        callback: IntegerCallback<'a>,
    },
    InversedImage {
        control: pe::InversedImage,
        callback: pe::InversedImageValueCallback<'a>,
    },
    InversedSymbology {
        control: pe::InversedSymbology,
        callback: pe::InversedSymbologyValueCallback<'a>,
    },
    LandingLightExtend {
        control: navlights::LandingLightExtend,
        callback: navlights::LandingLightExtendValueCallback<'a>,
    },
    LandingLightRetract {
        control: navlights::LandingLightRetract,
        callback: navlights::LandingLightRetractValueCallback<'a>,
    },
    LandingLights {
        control: navlights::LandingLights,
        callback: navlights::LandingLightsValueCallback<'a>,
    },
    LaserPower {
        control: pe::LaserPower,
        callback: pe::LaserPowerValueCallback<'a>,
    },
    LasingButton {
        control: pe::LasingButton,
        callback: pe::LasingButtonValueCallback<'a>,
    },
    LasingButtonCover {
        control: pe::LasingButtonCover,
        callback: pe::LasingButtonCoverValueCallback<'a>,
    },
    LeftMasterArm {
        control: weapons::LeftMasterArm,
        callback: weapons::LeftMasterArmValueCallback<'a>,
    },
    LeftMasterArmCover {
        control: weapons::LeftMasterArmCover,
        callback: weapons::LeftMasterArmCoverValueCallback<'a>,
    },
    LightsNormBnl {
        control: lights::LightsNormBnl,
        callback: lights::LightsNormBnlValueCallback<'a>,
    },
    LowFuel {
        control: warningpanel::LowFuel,
        callback: LedCallback<'a>,
    },
    LowRotorRpm {
        control: warningpanel::LowRotorRpm,
        callback: LedCallback<'a>,
    },
    LowTurbineRpm {
        control: warningpanel::LowTurbineRpm,
        callback: LedCallback<'a>,
    },
    MagneticBrake {
        control: autopilot::MagneticBrake,
        callback: autopilot::MagneticBrakeValueCallback<'a>,
    },
    MagneticBrakeButton {
        control: controls::MagneticBrakeButton,
        callback: controls::MagneticBrakeButtonValueCallback<'a>,
    },
    MainPanelLight {
        control: rawgaugevalues::MainPanelLight,
        callback: IntegerCallback<'a>,
    },
    MissileLaunch {
        control: pe::MissileLaunch,
        callback: pe::MissileLaunchValueCallback<'a>,
    },
    MissileLaunchCover {
        control: pe::MissileLaunchCover,
        callback: pe::MissileLaunchCoverValueCallback<'a>,
    },
    Nadir0Button {
        control: nadir::Nadir0Button,
        callback: nadir::Nadir0ButtonValueCallback<'a>,
    },
    Nadir1Button {
        control: nadir::Nadir1Button,
        callback: nadir::Nadir1ButtonValueCallback<'a>,
    },
    Nadir2Button {
        control: nadir::Nadir2Button,
        callback: nadir::Nadir2ButtonValueCallback<'a>,
    },
    Nadir3Button {
        control: nadir::Nadir3Button,
        callback: nadir::Nadir3ButtonValueCallback<'a>,
    },
    Nadir4Button {
        control: nadir::Nadir4Button,
        callback: nadir::Nadir4ButtonValueCallback<'a>,
    },
    Nadir5Button {
        control: nadir::Nadir5Button,
        callback: nadir::Nadir5ButtonValueCallback<'a>,
    },
    Nadir6Button {
        control: nadir::Nadir6Button,
        callback: nadir::Nadir6ButtonValueCallback<'a>,
    },
    Nadir7Button {
        control: nadir::Nadir7Button,
        callback: nadir::Nadir7ButtonValueCallback<'a>,
    },
    Nadir8Button {
        control: nadir::Nadir8Button,
        callback: nadir::Nadir8ButtonValueCallback<'a>,
    },
    Nadir9Button {
        control: nadir::Nadir9Button,
        callback: nadir::Nadir9ButtonValueCallback<'a>,
    },
    NadirAuxButton {
        control: nadir::NadirAuxButton,
        callback: nadir::NadirAuxButtonValueCallback<'a>,
    },
    NadirBrightness {
        control: rawgaugevalues::NadirBrightness,
        callback: IntegerCallback<'a>,
    },
    NadirDestButton {
        control: nadir::NadirDestButton,
        callback: nadir::NadirDestButtonValueCallback<'a>,
    },
    NadirDopplerMode {
        control: nadir::NadirDopplerMode,
        callback: nadir::NadirDopplerModeValueCallback<'a>,
    },
    NadirDownArrow {
        control: nadir::NadirDownArrow,
        callback: nadir::NadirDownArrowValueCallback<'a>,
    },
    NadirEffButton {
        control: nadir::NadirEffButton,
        callback: nadir::NadirEffButtonValueCallback<'a>,
    },
    NadirEnterButton {
        control: nadir::NadirEnterButton,
        callback: nadir::NadirEnterButtonValueCallback<'a>,
    },
    NadirGelButton {
        control: nadir::NadirGelButton,
        callback: nadir::NadirGelButtonValueCallback<'a>,
    },
    NadirGeoButton {
        control: nadir::NadirGeoButton,
        callback: nadir::NadirGeoButtonValueCallback<'a>,
    },
    NadirHeadingValue {
        control: gaugevalues::NadirHeadingValue,
        callback: IntegerCallback<'a>,
    },
    NadirMapIndButton {
        control: nadir::NadirMapIndButton,
        callback: nadir::NadirMapIndButtonValueCallback<'a>,
    },
    NadirOnOffBrightness {
        control: nadir::NadirOnOffBrightness,
        callback: IntegerCallback<'a>,
    },
    NadirParameter {
        control: nadir::NadirParameter,
        callback: nadir::NadirParameterValueCallback<'a>,
    },
    NadirPolButton {
        control: nadir::NadirPolButton,
        callback: nadir::NadirPolButtonValueCallback<'a>,
    },
    NadirPosButton {
        control: nadir::NadirPosButton,
        callback: nadir::NadirPosButtonValueCallback<'a>,
    },
    NavLights {
        control: navlights::NavLights,
        callback: navlights::NavLightsValueCallback<'a>,
    },
    PanelLighting {
        control: navlights::PanelLighting,
        callback: navlights::PanelLightingValueCallback<'a>,
    },
    PilotSight {
        control: weapons::PilotSight,
        callback: weapons::PilotSightValueCallback<'a>,
    },
    PilotWiper {
        control: electricsystem::PilotWiper,
        callback: electricsystem::PilotWiperValueCallback<'a>,
    },
    PitchNeedle {
        control: rawgaugevalues::PitchNeedle,
        callback: IntegerCallback<'a>,
    },
    Pitot {
        control: electricsystem::Pitot,
        callback: electricsystem::PitotValueCallback<'a>,
    },
    PitotTubeWarning {
        control: warningpanel::PitotTubeWarning,
        callback: LedCallback<'a>,
    },
    Quatre {
        control: rawgaugevalues::Quatre,
        callback: IntegerCallback<'a>,
    },
    RadarAltAlarmLamp {
        control: indicatorlights::RadarAltAlarmLamp,
        callback: LedCallback<'a>,
    },
    RadarAltBug {
        control: radaraltimeter::RadarAltBug,
        callback: IntegerCallback<'a>,
    },
    RadarAltFlagOnOff {
        control: rawgaugevalues::RadarAltFlagOnOff,
        callback: IntegerCallback<'a>,
    },
    RadarAltFlagPenne {
        control: rawgaugevalues::RadarAltFlagPenne,
        callback: IntegerCallback<'a>,
    },
    RadarAltKnob {
        control: rawgaugevalues::RadarAltKnob,
        callback: IntegerCallback<'a>,
    },
    RadarAltPower {
        control: radaraltimeter::RadarAltPower,
        callback: IntegerCallback<'a>,
    },
    RadarAltTest {
        control: radaraltimeter::RadarAltTest,
        callback: radaraltimeter::RadarAltTestValueCallback<'a>,
    },
    RadarAltValue {
        control: gaugevalues::RadarAltValue,
        callback: IntegerCallback<'a>,
    },
    RadarAltimeter {
        control: rawgaugevalues::RadarAltimeter,
        callback: IntegerCallback<'a>,
    },
    RadarAltimeterBug {
        control: rawgaugevalues::RadarAltimeterBug,
        callback: IntegerCallback<'a>,
    },
    RearmAlter {
        control: electricsystem::RearmAlter,
        callback: electricsystem::RearmAlterValueCallback<'a>,
    },
    RearmGene {
        control: electricsystem::RearmGene,
        callback: electricsystem::RearmGeneValueCallback<'a>,
    },
    RedLensOnOff {
        control: lights::RedLensOnOff,
        callback: lights::RedLensOnOffValueCallback<'a>,
    },
    RightMasterArm {
        control: weapons::RightMasterArm,
        callback: weapons::RightMasterArmValueCallback<'a>,
    },
    RightMasterArmCover {
        control: weapons::RightMasterArmCover,
        callback: weapons::RightMasterArmCoverValueCallback<'a>,
    },
    RltIdleLamp {
        control: indicatorlights::RltIdleLamp,
        callback: LedCallback<'a>,
    },
    RollNeedle {
        control: rawgaugevalues::RollNeedle,
        callback: IntegerCallback<'a>,
    },
    RoofLamp {
        control: lights::RoofLamp,
        callback: IntegerCallback<'a>,
    },
    RoofLight {
        control: rawgaugevalues::RoofLight,
        callback: IntegerCallback<'a>,
    },
    RotorBrake {
        control: rotors::RotorBrake,
        callback: rotors::RotorBrakeValueCallback<'a>,
    },
    RotorOilPressure {
        control: warningpanel::RotorOilPressure,
        callback: LedCallback<'a>,
    },
    RotorRpm {
        control: rawgaugevalues::RotorRpm,
        callback: IntegerCallback<'a>,
    },
    RotorRpmValue {
        control: gaugevalues::RotorRpmValue,
        callback: IntegerCallback<'a>,
    },
    RwrAudio {
        control: rwr::RwrAudio,
        callback: IntegerCallback<'a>,
    },
    RwrBrightness {
        control: rwr::RwrBrightness,
        callback: IntegerCallback<'a>,
    },
    RwrLight {
        control: indicatorlights::RwrLight,
        callback: LedCallback<'a>,
    },
    RwrMarker {
        control: rwr::RwrMarker,
        callback: rwr::RwrMarkerValueCallback<'a>,
    },
    RwrOffOnCroc {
        control: rwr::RwrOffOnCroc,
        callback: rwr::RwrOffOnCrocValueCallback<'a>,
    },
    RwrPage {
        control: rwr::RwrPage,
        callback: rwr::RwrPageValueCallback<'a>,
    },
    SandFilter {
        control: electricsystem::SandFilter,
        callback: electricsystem::SandFilterValueCallback<'a>,
    },
    SandFilterLamp {
        control: indicatorlights::SandFilterLamp,
        callback: LedCallback<'a>,
    },
    SandFilterWarning {
        control: warningpanel::SandFilterWarning,
        callback: LedCallback<'a>,
    },
    StandbyHaAdjust {
        control: flightcontrols::StandbyHaAdjust,
        callback: IntegerCallback<'a>,
    },
    StandbyHaUnlock {
        control: flightcontrols::StandbyHaUnlock,
        callback: flightcontrols::StandbyHaUnlockValueCallback<'a>,
    },
    Starter {
        control: electricsystem::Starter,
        callback: electricsystem::StarterValueCallback<'a>,
    },
    StbyArtHorCurseur {
        control: rawgaugevalues::StbyArtHorCurseur,
        callback: IntegerCallback<'a>,
    },
    StbyArtHorFlag {
        control: rawgaugevalues::StbyArtHorFlag,
        callback: IntegerCallback<'a>,
    },
    StbyArtHorPitch {
        control: rawgaugevalues::StbyArtHorPitch,
        callback: IntegerCallback<'a>,
    },
    StbyArtHorRoll {
        control: rawgaugevalues::StbyArtHorRoll,
        callback: IntegerCallback<'a>,
    },
    StbyArtHorW {
        control: rawgaugevalues::StbyArtHorW,
        callback: IntegerCallback<'a>,
    },
    SymbologyBrightnessLeft {
        control: pe::SymbologyBrightnessLeft,
        callback: pe::SymbologyBrightnessLeftValueCallback<'a>,
    },
    SymbologyBrightnessRight {
        control: pe::SymbologyBrightnessRight,
        callback: pe::SymbologyBrightnessRightValueCallback<'a>,
    },
    TempEngineOil {
        control: rawgaugevalues::TempEngineOil,
        callback: IntegerCallback<'a>,
    },
    TempExternal {
        control: rawgaugevalues::TempExternal,
        callback: IntegerCallback<'a>,
    },
    ToggleVdoVth {
        control: pe::ToggleVdoVth,
        callback: pe::ToggleVdoVthValueCallback<'a>,
    },
    TorqueBug {
        control: rawgaugevalues::TorqueBug,
        callback: IntegerCallback<'a>,
    },
    TorqueBugAdjust {
        control: torque::TorqueBugAdjust,
        callback: IntegerCallback<'a>,
    },
    TorqueNeedle {
        control: rawgaugevalues::TorqueNeedle,
        callback: IntegerCallback<'a>,
    },
    TorqueTest {
        control: torque::TorqueTest,
        callback: torque::TorqueTestValueCallback<'a>,
    },
    TorqueValue {
        control: gaugevalues::TorqueValue,
        callback: IntegerCallback<'a>,
    },
    TorqueWarningLamp {
        control: indicatorlights::TorqueWarningLamp,
        callback: LedCallback<'a>,
    },
    TurbineOilPressure {
        control: warningpanel::TurbineOilPressure,
        callback: LedCallback<'a>,
    },
    TurbineRpm {
        control: rawgaugevalues::TurbineRpm,
        callback: IntegerCallback<'a>,
    },
    TurbineRpmValue {
        control: gaugevalues::TurbineRpmValue,
        callback: IntegerCallback<'a>,
    },
    TvBrightness {
        control: tv::TvBrightness,
        callback: IntegerCallback<'a>,
    },
    TvBrightnessValue {
        control: rawgaugevalues::TvBrightnessValue,
        callback: IntegerCallback<'a>,
    },
    TvContrast {
        control: tv::TvContrast,
        callback: IntegerCallback<'a>,
    },
    TvCover {
        control: tv::TvCover,
        callback: tv::TvCoverValueCallback<'a>,
    },
    TvLamp {
        control: indicatorlights::TvLamp,
        callback: LedCallback<'a>,
    },
    TvOnOff {
        control: tv::TvOnOff,
        callback: tv::TvOnOffValueCallback<'a>,
    },
    UhfPageKnob {
        control: uhfradio::UhfPageKnob,
        callback: IntegerCallback<'a>,
    },
    UhfRadioButton0 {
        control: uhfradio::UhfRadioButton0,
        callback: uhfradio::UhfRadioButton0ValueCallback<'a>,
    },
    UhfRadioButton1 {
        control: uhfradio::UhfRadioButton1,
        callback: uhfradio::UhfRadioButton1ValueCallback<'a>,
    },
    UhfRadioButton2 {
        control: uhfradio::UhfRadioButton2,
        callback: uhfradio::UhfRadioButton2ValueCallback<'a>,
    },
    UhfRadioButton3 {
        control: uhfradio::UhfRadioButton3,
        callback: uhfradio::UhfRadioButton3ValueCallback<'a>,
    },
    UhfRadioButton4 {
        control: uhfradio::UhfRadioButton4,
        callback: uhfradio::UhfRadioButton4ValueCallback<'a>,
    },
    UhfRadioButton5 {
        control: uhfradio::UhfRadioButton5,
        callback: uhfradio::UhfRadioButton5ValueCallback<'a>,
    },
    UhfRadioButton6 {
        control: uhfradio::UhfRadioButton6,
        callback: uhfradio::UhfRadioButton6ValueCallback<'a>,
    },
    UhfRadioButton7 {
        control: uhfradio::UhfRadioButton7,
        callback: uhfradio::UhfRadioButton7ValueCallback<'a>,
    },
    UhfRadioButton8 {
        control: uhfradio::UhfRadioButton8,
        callback: uhfradio::UhfRadioButton8ValueCallback<'a>,
    },
    UhfRadioButton9 {
        control: uhfradio::UhfRadioButton9,
        callback: uhfradio::UhfRadioButton9ValueCallback<'a>,
    },
    UhfRadioButtonConf {
        control: uhfradio::UhfRadioButtonConf,
        callback: uhfradio::UhfRadioButtonConfValueCallback<'a>,
    },
    UhfRadioButtonDrw {
        control: uhfradio::UhfRadioButtonDrw,
        callback: uhfradio::UhfRadioButtonDrwValueCallback<'a>,
    },
    UhfRadioButtonVld {
        control: uhfradio::UhfRadioButtonVld,
        callback: uhfradio::UhfRadioButtonVldValueCallback<'a>,
    },
    UhfRadioMode {
        control: uhfradio::UhfRadioMode,
        callback: uhfradio::UhfRadioModeValueCallback<'a>,
    },
    UhfRadioVolume1 {
        control: intercom::UhfRadioVolume1,
        callback: IntegerCallback<'a>,
    },
    UhfRadioVolume2 {
        control: intercom::UhfRadioVolume2,
        callback: IntegerCallback<'a>,
    },
    UnassignedSwitch {
        control: electricsystem::UnassignedSwitch,
        callback: electricsystem::UnassignedSwitchValueCallback<'a>,
    },
    UvLighting {
        control: lights::UvLighting,
        callback: IntegerCallback<'a>,
    },
    VariometerVvi {
        control: rawgaugevalues::VariometerVvi,
        callback: IntegerCallback<'a>,
    },
    VhfAmRadioVolume1 {
        control: intercom::VhfAmRadioVolume1,
        callback: IntegerCallback<'a>,
    },
    VhfAmRadioVolume2 {
        control: intercom::VhfAmRadioVolume2,
        callback: IntegerCallback<'a>,
    },
    VoltMeter {
        control: rawgaugevalues::VoltMeter,
        callback: IntegerCallback<'a>,
    },
    VoltmeterTest {
        control: electricsystem::VoltmeterTest,
        callback: electricsystem::VoltmeterTestValueCallback<'a>,
    },
    WarningPanelTest {
        control: electricsystem::WarningPanelTest,
        callback: electricsystem::WarningPanelTestValueCallback<'a>,
    },
    WeaponLaunchSeq {
        control: weapons::WeaponLaunchSeq,
        callback: weapons::WeaponLaunchSeqValueCallback<'a>,
    },
    WeaponsMasterArm {
        control: weapons::WeaponsMasterArm,
        callback: weapons::WeaponsMasterArmValueCallback<'a>,
    },
    WeaponsPanel1DispBright {
        control: weapons::WeaponsPanel1DispBright,
        callback: IntegerCallback<'a>,
    },
    WeaponsPanel1OffOnStby {
        control: weapons::WeaponsPanel1OffOnStby,
        callback: weapons::WeaponsPanel1OffOnStbyValueCallback<'a>,
    },
    WiperOnce {
        control: electricsystem::WiperOnce,
        callback: electricsystem::WiperOnceValueCallback<'a>,
    },
    WipersCopilot {
        control: rawgaugevalues::WipersCopilot,
        callback: IntegerCallback<'a>,
    },
    WipersPilot {
        control: rawgaugevalues::WipersPilot,
        callback: IntegerCallback<'a>,
    },
    Wp2LeftLamp {
        control: indicatorlights::Wp2LeftLamp,
        callback: LedCallback<'a>,
    },
    Wp2RightLamp {
        control: indicatorlights::Wp2RightLamp,
        callback: LedCallback<'a>,
    },
    YawNeedle {
        control: rawgaugevalues::YawNeedle,
        callback: IntegerCallback<'a>,
    },
}

impl RegisteredControl<'_> {
    fn accept(&mut self, packet: Packet<'_>) {
        match self {
            Self::Ac115vBusNav { control, .. } => control.accept(packet),
            Self::AdditionalFuelTank { control, .. } => control.accept(packet),
            Self::AdditionalFuelTankLamp { control, .. } => control.accept(packet),
            Self::Adf1Adf2Select { control, .. } => control.accept(packet),
            Self::AdfBackground { control, .. } => control.accept(packet),
            Self::AdfDistance100s { control, .. } => control.accept(packet),
            Self::AdfDistance10s { control, .. } => control.accept(packet),
            Self::AdfDistance1s { control, .. } => control.accept(packet),
            Self::AdfDistanceValue { control, .. } => control.accept(packet),
            Self::AdfFlagBut { control, .. } => control.accept(packet),
            Self::AdfFlagCap { control, .. } => control.accept(packet),
            Self::AdfFlagCounter { control, .. } => control.accept(packet),
            Self::AdfGain { control, .. } => control.accept(packet),
            Self::AdfHeadingValue { control, .. } => control.accept(packet),
            Self::AdfMode { control, .. } => control.accept(packet),
            Self::AdfNadirNeedle { control, .. } => control.accept(packet),
            Self::AdfNav11 { control, .. } => control.accept(packet),
            Self::AdfNav110 { control, .. } => control.accept(packet),
            Self::AdfNav1100 { control, .. } => control.accept(packet),
            Self::AdfNav1Freq100s { control, .. } => control.accept(packet),
            Self::AdfNav1Freq10s { control, .. } => control.accept(packet),
            Self::AdfNav1Freq1s { control, .. } => control.accept(packet),
            Self::AdfNav1FreqDecimal { control, .. } => control.accept(packet),
            Self::AdfNav21 { control, .. } => control.accept(packet),
            Self::AdfNav210 { control, .. } => control.accept(packet),
            Self::AdfNav2100 { control, .. } => control.accept(packet),
            Self::AdfNav2Freq100s { control, .. } => control.accept(packet),
            Self::AdfNav2Freq10s { control, .. } => control.accept(packet),
            Self::AdfNav2Freq1s { control, .. } => control.accept(packet),
            Self::AdfNav2FreqDecimal { control, .. } => control.accept(packet),
            Self::AdfSmallNeedle { control, .. } => control.accept(packet),
            Self::AdfToneOnOff { control, .. } => control.accept(packet),
            Self::AlarmWarningLamp { control, .. } => control.accept(packet),
            Self::Alternator { control, .. } => control.accept(packet),
            Self::AlternatorWarning { control, .. } => control.accept(packet),
            Self::AmFrequencySelector { control, .. } => control.accept(packet),
            Self::AmRadioFreq100s { control, .. } => control.accept(packet),
            Self::AmRadioFreq10s { control, .. } => control.accept(packet),
            Self::AmRadioFreq1s { control, .. } => control.accept(packet),
            Self::AmRadioFreqHundredths { control, .. } => control.accept(packet),
            Self::AmRadioFreqTenths { control, .. } => control.accept(packet),
            Self::AmRadioFrequencyDialLeft { control, .. } => control.accept(packet),
            Self::AmRadioFrequencyDialRight { control, .. } => control.accept(packet),
            Self::AmRadioLamp { control, .. } => control.accept(packet),
            Self::AmRadioSelector { control, .. } => control.accept(packet),
            Self::AnticollIntensity { control, .. } => control.accept(packet),
            Self::AnticollLights { control, .. } => control.accept(packet),
            Self::ArtHorizBall { control, .. } => control.accept(packet),
            Self::ArtHorizCurseur { control, .. } => control.accept(packet),
            Self::ArtHorizFlagGs { control, .. } => control.accept(packet),
            Self::ArtHorizFlagHs { control, .. } => control.accept(packet),
            Self::ArtHorizFlagLoc { control, .. } => control.accept(packet),
            Self::ArtHorizHorizontalBar { control, .. } => control.accept(packet),
            Self::ArtHorizPitch { control, .. } => control.accept(packet),
            Self::ArtHorizRoll { control, .. } => control.accept(packet),
            Self::ArtHorizVertBar { control, .. } => control.accept(packet),
            Self::ArtHorizW { control, .. } => control.accept(packet),
            Self::AutoHover { control, .. } => control.accept(packet),
            Self::Autopilot { control, .. } => control.accept(packet),
            Self::AutopilotButton { control, .. } => control.accept(packet),
            Self::AutopilotMode { control, .. } => control.accept(packet),
            Self::AutopilotOnOff { control, .. } => control.accept(packet),
            Self::AutopilotPitch { control, .. } => control.accept(packet),
            Self::AutopilotRoll { control, .. } => control.accept(packet),
            Self::AutopilotSlave { control, .. } => control.accept(packet),
            Self::AutopilotTrim { control, .. } => control.accept(packet),
            Self::AutopilotYaw { control, .. } => control.accept(packet),
            Self::AuxFuelTank { control, .. } => control.accept(packet),
            Self::BaroAltHundreds { control, .. } => control.accept(packet),
            Self::BaroAltThousands { control, .. } => control.accept(packet),
            Self::BaroAltValue { control, .. } => control.accept(packet),
            Self::BaroPressValue { control, .. } => control.accept(packet),
            Self::BaroPressureHundreds { control, .. } => control.accept(packet),
            Self::BaroPressureTens { control, .. } => control.accept(packet),
            Self::BaroPressureThousands { control, .. } => control.accept(packet),
            Self::BaroPressureUnits { control, .. } => control.accept(packet),
            Self::BarometricAltimeter { control, .. } => control.accept(packet),
            Self::Battery { control, .. } => control.accept(packet),
            Self::BatteryWarning { control, .. } => control.accept(packet),
            Self::BlockedEngineWarningLamp { control, .. } => control.accept(packet),
            Self::CameraCentering { control, .. } => control.accept(packet),
            Self::CameraMode { control, .. } => control.accept(packet),
            Self::CameraZoomIn { control, .. } => control.accept(packet),
            Self::CameraZoomOut { control, .. } => control.accept(packet),
            Self::CenterConsoleLight { control, .. } => control.accept(packet),
            Self::ChipsInOil { control, .. } => control.accept(packet),
            Self::ClockHour { control, .. } => control.accept(packet),
            Self::ClockMini { control, .. } => control.accept(packet),
            Self::ClockMinute { control, .. } => control.accept(packet),
            Self::ClockPushButton { control, .. } => control.accept(packet),
            Self::ClockReset { control, .. } => control.accept(packet),
            Self::ClockSecond { control, .. } => control.accept(packet),
            Self::ClockSet { control, .. } => control.accept(packet),
            Self::ClockStartStop { control, .. } => control.accept(packet),
            Self::ConsoleLighting { control, .. } => control.accept(packet),
            Self::ConvoyFuelTankLamp { control, .. } => control.accept(packet),
            Self::ConvoyTank { control, .. } => control.accept(packet),
            Self::CopilotWiper { control, .. } => control.accept(packet),
            Self::CthPower { control, .. } => control.accept(packet),
            Self::DashboardLighting { control, .. } => control.accept(packet),
            Self::DemStartLamp { control, .. } => control.accept(packet),
            Self::ExtPositionLightLeft { control, .. } => control.accept(packet),
            Self::ExtPositionLightRight { control, .. } => control.accept(packet),
            Self::ExtStrobe { control, .. } => control.accept(packet),
            Self::FdLampDOpe { control, .. } => control.accept(packet),
            Self::FdLampDVide1 { control, .. } => control.accept(packet),
            Self::FdLampDVide2 { control, .. } => control.accept(packet),
            Self::FdLampGOpe { control, .. } => control.accept(packet),
            Self::FdLampGVide1 { control, .. } => control.accept(packet),
            Self::FdLampGVide2 { control, .. } => control.accept(packet),
            Self::FdLampLeu { control, .. } => control.accept(packet),
            Self::FdLampVe { control, .. } => control.accept(packet),
            Self::FlareDispFireCap { control, .. } => control.accept(packet),
            Self::FlareDispSideSelect { control, .. } => control.accept(packet),
            Self::FlareDispenseButton { control, .. } => control.accept(packet),
            Self::FlareDispenseMode { control, .. } => control.accept(packet),
            Self::FlareDispenserOffOn { control, .. } => control.accept(packet),
            Self::FmRadioButton0 { control, .. } => control.accept(packet),
            Self::FmRadioButton1 { control, .. } => control.accept(packet),
            Self::FmRadioButton2 { control, .. } => control.accept(packet),
            Self::FmRadioButton3 { control, .. } => control.accept(packet),
            Self::FmRadioButton4 { control, .. } => control.accept(packet),
            Self::FmRadioButton5 { control, .. } => control.accept(packet),
            Self::FmRadioButton6 { control, .. } => control.accept(packet),
            Self::FmRadioButton7 { control, .. } => control.accept(packet),
            Self::FmRadioButton8 { control, .. } => control.accept(packet),
            Self::FmRadioButton9 { control, .. } => control.accept(packet),
            Self::FmRadioButtonDown { control, .. } => control.accept(packet),
            Self::FmRadioButtonRc { control, .. } => control.accept(packet),
            Self::FmRadioButtonUp { control, .. } => control.accept(packet),
            Self::FmRadioButtonVal { control, .. } => control.accept(packet),
            Self::FmRadioButtonX { control, .. } => control.accept(packet),
            Self::FmRadioChannel { control, .. } => control.accept(packet),
            Self::FmRadioMode { control, .. } => control.accept(packet),
            Self::FmRadioVolume1 { control, .. } => control.accept(packet),
            Self::FmRadioVolume2 { control, .. } => control.accept(packet),
            Self::FormationIntensity { control, .. } => control.accept(packet),
            Self::FormationLights { control, .. } => control.accept(packet),
            Self::Fuel { control, .. } => control.accept(packet),
            Self::FuelFlowLever { control, .. } => control.accept(packet),
            Self::FuelLevelValue { control, .. } => control.accept(packet),
            Self::FuelPump { control, .. } => control.accept(packet),
            Self::Generator { control, .. } => control.accept(packet),
            Self::GeneratorWarning { control, .. } => control.accept(packet),
            Self::GyroBppWarningLamp { control, .. } => control.accept(packet),
            Self::GyroCompassHeading { control, .. } => control.accept(packet),
            Self::GyroKnob { control, .. } => control.accept(packet),
            Self::GyroNeedle { control, .. } => control.accept(packet),
            Self::GyroSwitch { control, .. } => control.accept(packet),
            Self::GyroTestCover { control, .. } => control.accept(packet),
            Self::GyroTestLamp { control, .. } => control.accept(packet),
            Self::GyroTestSwitch { control, .. } => control.accept(packet),
            Self::GyroTrimWarningLamp { control, .. } => control.accept(packet),
            Self::HaAdjust { control, .. } => control.accept(packet),
            Self::HaSource { control, .. } => control.accept(packet),
            Self::HaUnlock { control, .. } => control.accept(packet),
            Self::HeadingValue { control, .. } => control.accept(packet),
            Self::Hot3Brightness { control, .. } => control.accept(packet),
            Self::Hot3PanelTestOffOn { control, .. } => control.accept(packet),
            Self::Hot3StationSelect { control, .. } => control.accept(packet),
            Self::HydTest { control, .. } => control.accept(packet),
            Self::IasValue { control, .. } => control.accept(packet),
            Self::ImageBrightnessDown { control, .. } => control.accept(packet),
            Self::ImageBrightnessUp { control, .. } => control.accept(packet),
            Self::ImageFocusDown { control, .. } => control.accept(packet),
            Self::ImageFocusUp { control, .. } => control.accept(packet),
            Self::ImageGainLeft { control, .. } => control.accept(packet),
            Self::ImageGainRight { control, .. } => control.accept(packet),
            Self::IndAirSpeedIas { control, .. } => control.accept(packet),
            Self::InversedImage { control, .. } => control.accept(packet),
            Self::InversedSymbology { control, .. } => control.accept(packet),
            Self::LandingLightExtend { control, .. } => control.accept(packet),
            Self::LandingLightRetract { control, .. } => control.accept(packet),
            Self::LandingLights { control, .. } => control.accept(packet),
            Self::LaserPower { control, .. } => control.accept(packet),
            Self::LasingButton { control, .. } => control.accept(packet),
            Self::LasingButtonCover { control, .. } => control.accept(packet),
            Self::LeftMasterArm { control, .. } => control.accept(packet),
            Self::LeftMasterArmCover { control, .. } => control.accept(packet),
            Self::LightsNormBnl { control, .. } => control.accept(packet),
            Self::LowFuel { control, .. } => control.accept(packet),
            Self::LowRotorRpm { control, .. } => control.accept(packet),
            Self::LowTurbineRpm { control, .. } => control.accept(packet),
            Self::MagneticBrake { control, .. } => control.accept(packet),
            Self::MagneticBrakeButton { control, .. } => control.accept(packet),
            Self::MainPanelLight { control, .. } => control.accept(packet),
            Self::MissileLaunch { control, .. } => control.accept(packet),
            Self::MissileLaunchCover { control, .. } => control.accept(packet),
            Self::Nadir0Button { control, .. } => control.accept(packet),
            Self::Nadir1Button { control, .. } => control.accept(packet),
            Self::Nadir2Button { control, .. } => control.accept(packet),
            Self::Nadir3Button { control, .. } => control.accept(packet),
            Self::Nadir4Button { control, .. } => control.accept(packet),
            Self::Nadir5Button { control, .. } => control.accept(packet),
            Self::Nadir6Button { control, .. } => control.accept(packet),
            Self::Nadir7Button { control, .. } => control.accept(packet),
            Self::Nadir8Button { control, .. } => control.accept(packet),
            Self::Nadir9Button { control, .. } => control.accept(packet),
            Self::NadirAuxButton { control, .. } => control.accept(packet),
            Self::NadirBrightness { control, .. } => control.accept(packet),
            Self::NadirDestButton { control, .. } => control.accept(packet),
            Self::NadirDopplerMode { control, .. } => control.accept(packet),
            Self::NadirDownArrow { control, .. } => control.accept(packet),
            Self::NadirEffButton { control, .. } => control.accept(packet),
            Self::NadirEnterButton { control, .. } => control.accept(packet),
            Self::NadirGelButton { control, .. } => control.accept(packet),
            Self::NadirGeoButton { control, .. } => control.accept(packet),
            Self::NadirHeadingValue { control, .. } => control.accept(packet),
            Self::NadirMapIndButton { control, .. } => control.accept(packet),
            Self::NadirOnOffBrightness { control, .. } => control.accept(packet),
            Self::NadirParameter { control, .. } => control.accept(packet),
            Self::NadirPolButton { control, .. } => control.accept(packet),
            Self::NadirPosButton { control, .. } => control.accept(packet),
            Self::NavLights { control, .. } => control.accept(packet),
            Self::PanelLighting { control, .. } => control.accept(packet),
            Self::PilotSight { control, .. } => control.accept(packet),
            Self::PilotWiper { control, .. } => control.accept(packet),
            Self::PitchNeedle { control, .. } => control.accept(packet),
            Self::Pitot { control, .. } => control.accept(packet),
            Self::PitotTubeWarning { control, .. } => control.accept(packet),
            Self::Quatre { control, .. } => control.accept(packet),
            Self::RadarAltAlarmLamp { control, .. } => control.accept(packet),
            Self::RadarAltBug { control, .. } => control.accept(packet),
            Self::RadarAltFlagOnOff { control, .. } => control.accept(packet),
            Self::RadarAltFlagPenne { control, .. } => control.accept(packet),
            Self::RadarAltKnob { control, .. } => control.accept(packet),
            Self::RadarAltPower { control, .. } => control.accept(packet),
            Self::RadarAltTest { control, .. } => control.accept(packet),
            Self::RadarAltValue { control, .. } => control.accept(packet),
            Self::RadarAltimeter { control, .. } => control.accept(packet),
            Self::RadarAltimeterBug { control, .. } => control.accept(packet),
            Self::RearmAlter { control, .. } => control.accept(packet),
            Self::RearmGene { control, .. } => control.accept(packet),
            Self::RedLensOnOff { control, .. } => control.accept(packet),
            Self::RightMasterArm { control, .. } => control.accept(packet),
            Self::RightMasterArmCover { control, .. } => control.accept(packet),
            Self::RltIdleLamp { control, .. } => control.accept(packet),
            Self::RollNeedle { control, .. } => control.accept(packet),
            Self::RoofLamp { control, .. } => control.accept(packet),
            Self::RoofLight { control, .. } => control.accept(packet),
            Self::RotorBrake { control, .. } => control.accept(packet),
            Self::RotorOilPressure { control, .. } => control.accept(packet),
            Self::RotorRpm { control, .. } => control.accept(packet),
            Self::RotorRpmValue { control, .. } => control.accept(packet),
            Self::RwrAudio { control, .. } => control.accept(packet),
            Self::RwrBrightness { control, .. } => control.accept(packet),
            Self::RwrLight { control, .. } => control.accept(packet),
            Self::RwrMarker { control, .. } => control.accept(packet),
            Self::RwrOffOnCroc { control, .. } => control.accept(packet),
            Self::RwrPage { control, .. } => control.accept(packet),
            Self::SandFilter { control, .. } => control.accept(packet),
            Self::SandFilterLamp { control, .. } => control.accept(packet),
            Self::SandFilterWarning { control, .. } => control.accept(packet),
            Self::StandbyHaAdjust { control, .. } => control.accept(packet),
            Self::StandbyHaUnlock { control, .. } => control.accept(packet),
            Self::Starter { control, .. } => control.accept(packet),
            Self::StbyArtHorCurseur { control, .. } => control.accept(packet),
            Self::StbyArtHorFlag { control, .. } => control.accept(packet),
            Self::StbyArtHorPitch { control, .. } => control.accept(packet),
            Self::StbyArtHorRoll { control, .. } => control.accept(packet),
            Self::StbyArtHorW { control, .. } => control.accept(packet),
            Self::SymbologyBrightnessLeft { control, .. } => control.accept(packet),
            Self::SymbologyBrightnessRight { control, .. } => control.accept(packet),
            Self::TempEngineOil { control, .. } => control.accept(packet),
            Self::TempExternal { control, .. } => control.accept(packet),
            Self::ToggleVdoVth { control, .. } => control.accept(packet),
            Self::TorqueBug { control, .. } => control.accept(packet),
            Self::TorqueBugAdjust { control, .. } => control.accept(packet),
            Self::TorqueNeedle { control, .. } => control.accept(packet),
            Self::TorqueTest { control, .. } => control.accept(packet),
            Self::TorqueValue { control, .. } => control.accept(packet),
            Self::TorqueWarningLamp { control, .. } => control.accept(packet),
            Self::TurbineOilPressure { control, .. } => control.accept(packet),
            Self::TurbineRpm { control, .. } => control.accept(packet),
            Self::TurbineRpmValue { control, .. } => control.accept(packet),
            Self::TvBrightness { control, .. } => control.accept(packet),
            Self::TvBrightnessValue { control, .. } => control.accept(packet),
            Self::TvContrast { control, .. } => control.accept(packet),
            Self::TvCover { control, .. } => control.accept(packet),
            Self::TvLamp { control, .. } => control.accept(packet),
            Self::TvOnOff { control, .. } => control.accept(packet),
            Self::UhfPageKnob { control, .. } => control.accept(packet),
            Self::UhfRadioButton0 { control, .. } => control.accept(packet),
            Self::UhfRadioButton1 { control, .. } => control.accept(packet),
            Self::UhfRadioButton2 { control, .. } => control.accept(packet),
            Self::UhfRadioButton3 { control, .. } => control.accept(packet),
            Self::UhfRadioButton4 { control, .. } => control.accept(packet),
            Self::UhfRadioButton5 { control, .. } => control.accept(packet),
            Self::UhfRadioButton6 { control, .. } => control.accept(packet),
            Self::UhfRadioButton7 { control, .. } => control.accept(packet),
            Self::UhfRadioButton8 { control, .. } => control.accept(packet),
            Self::UhfRadioButton9 { control, .. } => control.accept(packet),
            Self::UhfRadioButtonConf { control, .. } => control.accept(packet),
            Self::UhfRadioButtonDrw { control, .. } => control.accept(packet),
            Self::UhfRadioButtonVld { control, .. } => control.accept(packet),
            Self::UhfRadioMode { control, .. } => control.accept(packet),
            Self::UhfRadioVolume1 { control, .. } => control.accept(packet),
            Self::UhfRadioVolume2 { control, .. } => control.accept(packet),
            Self::UnassignedSwitch { control, .. } => control.accept(packet),
            Self::UvLighting { control, .. } => control.accept(packet),
            Self::VariometerVvi { control, .. } => control.accept(packet),
            Self::VhfAmRadioVolume1 { control, .. } => control.accept(packet),
            Self::VhfAmRadioVolume2 { control, .. } => control.accept(packet),
            Self::VoltMeter { control, .. } => control.accept(packet),
            Self::VoltmeterTest { control, .. } => control.accept(packet),
            Self::WarningPanelTest { control, .. } => control.accept(packet),
            Self::WeaponLaunchSeq { control, .. } => control.accept(packet),
            Self::WeaponsMasterArm { control, .. } => control.accept(packet),
            Self::WeaponsPanel1DispBright { control, .. } => control.accept(packet),
            Self::WeaponsPanel1OffOnStby { control, .. } => control.accept(packet),
            Self::WiperOnce { control, .. } => control.accept(packet),
            Self::WipersCopilot { control, .. } => control.accept(packet),
            Self::WipersPilot { control, .. } => control.accept(packet),
            Self::Wp2LeftLamp { control, .. } => control.accept(packet),
            Self::Wp2RightLamp { control, .. } => control.accept(packet),
            Self::YawNeedle { control, .. } => control.accept(packet),
        }
    }

    fn commit(&mut self) {
        match self {
            Self::Ac115vBusNav { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::AdditionalFuelTank { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <electricsystem::AdditionalFuelTankValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::AdditionalFuelTankLamp { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Adf1Adf2Select { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <adf::Adf1Adf2SelectValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AdfBackground { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdfDistance100s { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdfDistance10s { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdfDistance1s { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdfDistanceValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdfFlagBut { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdfFlagCap { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdfFlagCounter { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdfGain { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdfHeadingValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdfMode { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <adf::AdfModeValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::AdfNadirNeedle { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdfNav11 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdfNav110 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdfNav1100 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdfNav1Freq100s { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdfNav1Freq10s { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdfNav1Freq1s { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdfNav1FreqDecimal { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdfNav21 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdfNav210 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdfNav2100 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdfNav2Freq100s { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdfNav2Freq10s { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdfNav2Freq1s { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdfNav2FreqDecimal { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdfSmallNeedle { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdfToneOnOff { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <adf::AdfToneOnOffValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AlarmWarningLamp { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Alternator { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <electricsystem::AlternatorValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::AlternatorWarning { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::AmFrequencySelector { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <amradio::AmFrequencySelectorValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AmRadioFreq100s { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AmRadioFreq10s { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AmRadioFreq1s { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AmRadioFreqHundredths { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AmRadioFreqTenths { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AmRadioFrequencyDialLeft { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AmRadioFrequencyDialRight { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AmRadioLamp { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::AmRadioSelector { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <amradio::AmRadioSelectorValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::AnticollIntensity { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AnticollLights { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <navlights::AnticollLightsValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::ArtHorizBall { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ArtHorizCurseur { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ArtHorizFlagGs { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ArtHorizFlagHs { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ArtHorizFlagLoc { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ArtHorizHorizontalBar { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ArtHorizPitch { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ArtHorizRoll { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ArtHorizVertBar { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ArtHorizW { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AutoHover { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <autopilot::AutoHoverValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Autopilot { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::AutopilotButton { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <autopilot::AutopilotButtonValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::AutopilotMode { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <autopilot::AutopilotModeValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::AutopilotOnOff { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <autopilot::AutopilotOnOffValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::AutopilotPitch { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <autopilot::AutopilotPitchValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::AutopilotRoll { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <autopilot::AutopilotRollValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::AutopilotSlave { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <autopilot::AutopilotSlaveValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::AutopilotTrim { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <autopilot::AutopilotTrimValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::AutopilotYaw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <autopilot::AutopilotYawValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::AuxFuelTank { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::BaroAltHundreds { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::BaroAltThousands { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::BaroAltValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::BaroPressValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::BaroPressureHundreds { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::BaroPressureTens { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::BaroPressureThousands { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::BaroPressureUnits { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::BarometricAltimeter { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Battery { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <electricsystem::BatteryValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::BatteryWarning { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::BlockedEngineWarningLamp { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::CameraCentering { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <pe::CameraCenteringValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CameraMode { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <pe::CameraModeValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CameraZoomIn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <pe::CameraZoomInValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CameraZoomOut { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <pe::CameraZoomOutValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CenterConsoleLight { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ChipsInOil { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClockHour { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ClockMini { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ClockMinute { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ClockPushButton { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <clock::ClockPushButtonValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::ClockReset { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <clock::ClockResetValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::ClockSecond { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ClockSet { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ClockStartStop { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <clock::ClockStartStopValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::ConsoleLighting { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ConvoyFuelTankLamp { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ConvoyTank { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <electricsystem::ConvoyTankValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CopilotWiper { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <electricsystem::CopilotWiperValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CthPower { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <pe::CthPowerValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::DashboardLighting { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DemStartLamp { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
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
            Self::ExtStrobe { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FdLampDOpe { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::FdLampDVide1 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::FdLampDVide2 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::FdLampGOpe { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::FdLampGVide1 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::FdLampGVide2 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::FdLampLeu { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::FdLampVe { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::FlareDispFireCap { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <flaredispenser::FlareDispFireCapValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::FlareDispSideSelect { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <flaredispenser::FlareDispSideSelectValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::FlareDispenseButton { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <flaredispenser::FlareDispenseButtonValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::FlareDispenseMode { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <flaredispenser::FlareDispenseModeValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::FlareDispenserOffOn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <flaredispenser::FlareDispenserOffOnValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::FmRadioButton0 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fmradio::FmRadioButton0Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FmRadioButton1 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fmradio::FmRadioButton1Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FmRadioButton2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fmradio::FmRadioButton2Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FmRadioButton3 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fmradio::FmRadioButton3Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FmRadioButton4 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fmradio::FmRadioButton4Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FmRadioButton5 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fmradio::FmRadioButton5Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FmRadioButton6 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fmradio::FmRadioButton6Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FmRadioButton7 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fmradio::FmRadioButton7Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FmRadioButton8 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fmradio::FmRadioButton8Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FmRadioButton9 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fmradio::FmRadioButton9Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FmRadioButtonDown { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fmradio::FmRadioButtonDownValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FmRadioButtonRc { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fmradio::FmRadioButtonRcValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FmRadioButtonUp { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fmradio::FmRadioButtonUpValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FmRadioButtonVal { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fmradio::FmRadioButtonValValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FmRadioButtonX { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fmradio::FmRadioButtonXValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FmRadioChannel { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fmradio::FmRadioChannelValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FmRadioMode { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <fmradio::FmRadioModeValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::FmRadioVolume1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FmRadioVolume2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FormationIntensity { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FormationLights { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <navlights::FormationLightsValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Fuel { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelFlowLever { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <fuel::FuelFlowLeverValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::FuelLevelValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelPump { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <electricsystem::FuelPumpValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Generator { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <electricsystem::GeneratorValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::GeneratorWarning { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::GyroBppWarningLamp { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::GyroCompassHeading { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::GyroKnob { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <autopilot::GyroKnobValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::GyroNeedle { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::GyroSwitch { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <autopilot::GyroSwitchValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::GyroTestCover { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <autopilot::GyroTestCoverValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::GyroTestLamp { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::GyroTestSwitch { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <autopilot::GyroTestSwitchValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::GyroTrimWarningLamp { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::HaAdjust { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HaSource { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <flightcontrols::HaSourceValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::HaUnlock { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <flightcontrols::HaUnlockValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::HeadingValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Hot3Brightness { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Hot3PanelTestOffOn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <hot3::Hot3PanelTestOffOnValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Hot3StationSelect { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <hot3::Hot3StationSelectValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::HydTest { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <electricsystem::HydTestValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IasValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ImageBrightnessDown { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <pe::ImageBrightnessDownValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::ImageBrightnessUp { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <pe::ImageBrightnessUpValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::ImageFocusDown { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <pe::ImageFocusDownValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::ImageFocusUp { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <pe::ImageFocusUpValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::ImageGainLeft { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <pe::ImageGainLeftValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::ImageGainRight { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <pe::ImageGainRightValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IndAirSpeedIas { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::InversedImage { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <pe::InversedImageValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::InversedSymbology { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <pe::InversedSymbologyValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LandingLightExtend { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <navlights::LandingLightExtendValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::LandingLightRetract { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <navlights::LandingLightRetractValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::LandingLights { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <navlights::LandingLightsValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::LaserPower { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <pe::LaserPowerValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LasingButton { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <pe::LasingButtonValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LasingButtonCover { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <pe::LasingButtonCoverValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LeftMasterArm { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <weapons::LeftMasterArmValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::LeftMasterArmCover { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <weapons::LeftMasterArmCoverValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LightsNormBnl { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <lights::LightsNormBnlValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LowFuel { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LowRotorRpm { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LowTurbineRpm { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::MagneticBrake { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <autopilot::MagneticBrakeValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MagneticBrakeButton { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <controls::MagneticBrakeButtonValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::MainPanelLight { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MissileLaunch { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <pe::MissileLaunchValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MissileLaunchCover { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <pe::MissileLaunchCoverValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Nadir0Button { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <nadir::Nadir0ButtonValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Nadir1Button { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <nadir::Nadir1ButtonValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Nadir2Button { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <nadir::Nadir2ButtonValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Nadir3Button { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <nadir::Nadir3ButtonValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Nadir4Button { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <nadir::Nadir4ButtonValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Nadir5Button { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <nadir::Nadir5ButtonValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Nadir6Button { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <nadir::Nadir6ButtonValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Nadir7Button { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <nadir::Nadir7ButtonValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Nadir8Button { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <nadir::Nadir8ButtonValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Nadir9Button { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <nadir::Nadir9ButtonValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::NadirAuxButton { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <nadir::NadirAuxButtonValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::NadirBrightness { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::NadirDestButton { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <nadir::NadirDestButtonValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::NadirDopplerMode { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <nadir::NadirDopplerModeValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::NadirDownArrow { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <nadir::NadirDownArrowValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::NadirEffButton { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <nadir::NadirEffButtonValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::NadirEnterButton { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <nadir::NadirEnterButtonValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::NadirGelButton { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <nadir::NadirGelButtonValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::NadirGeoButton { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <nadir::NadirGeoButtonValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::NadirHeadingValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::NadirMapIndButton { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <nadir::NadirMapIndButtonValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::NadirOnOffBrightness { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::NadirParameter { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <nadir::NadirParameterValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::NadirPolButton { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <nadir::NadirPolButtonValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::NadirPosButton { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <nadir::NadirPosButtonValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::NavLights { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <navlights::NavLightsValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::PanelLighting { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <navlights::PanelLightingValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::PilotSight { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <weapons::PilotSightValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::PilotWiper { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <electricsystem::PilotWiperValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::PitchNeedle { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Pitot { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <electricsystem::PitotValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::PitotTubeWarning { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Quatre { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RadarAltAlarmLamp { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RadarAltBug { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RadarAltFlagOnOff { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RadarAltFlagPenne { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RadarAltKnob { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RadarAltPower { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RadarAltTest { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <radaraltimeter::RadarAltTestValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RadarAltValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RadarAltimeter { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RadarAltimeterBug { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RearmAlter { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <electricsystem::RearmAlterValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RearmGene { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <electricsystem::RearmGeneValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RedLensOnOff { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <lights::RedLensOnOffValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RightMasterArm { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <weapons::RightMasterArmValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RightMasterArmCover { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <weapons::RightMasterArmCoverValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RltIdleLamp { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RollNeedle { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RoofLamp { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RoofLight { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RotorBrake { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <rotors::RotorBrakeValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RotorOilPressure { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RotorRpm { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RotorRpmValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RwrAudio { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RwrBrightness { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RwrLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RwrMarker { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <rwr::RwrMarkerValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RwrOffOnCroc { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <rwr::RwrOffOnCrocValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RwrPage { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <rwr::RwrPageValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::SandFilter { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <electricsystem::SandFilterValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::SandFilterLamp { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::SandFilterWarning { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::StandbyHaAdjust { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::StandbyHaUnlock { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <flightcontrols::StandbyHaUnlockValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::Starter { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <electricsystem::StarterValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::StbyArtHorCurseur { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::StbyArtHorFlag { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::StbyArtHorPitch { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::StbyArtHorRoll { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::StbyArtHorW { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SymbologyBrightnessLeft { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <pe::SymbologyBrightnessLeftValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::SymbologyBrightnessRight { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <pe::SymbologyBrightnessRightValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::TempEngineOil { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TempExternal { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ToggleVdoVth { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <pe::ToggleVdoVthValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::TorqueBug { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TorqueBugAdjust { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TorqueNeedle { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TorqueTest { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <torque::TorqueTestValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::TorqueValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TorqueWarningLamp { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::TurbineOilPressure { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::TurbineRpm { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TurbineRpmValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TvBrightness { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TvBrightnessValue { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TvContrast { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TvCover { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <tv::TvCoverValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::TvLamp { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::TvOnOff { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <tv::TvOnOffValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfPageKnob { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UhfRadioButton0 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <uhfradio::UhfRadioButton0Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfRadioButton1 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <uhfradio::UhfRadioButton1Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfRadioButton2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <uhfradio::UhfRadioButton2Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfRadioButton3 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <uhfradio::UhfRadioButton3Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfRadioButton4 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <uhfradio::UhfRadioButton4Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfRadioButton5 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <uhfradio::UhfRadioButton5Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfRadioButton6 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <uhfradio::UhfRadioButton6Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfRadioButton7 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <uhfradio::UhfRadioButton7Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfRadioButton8 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <uhfradio::UhfRadioButton8Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfRadioButton9 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <uhfradio::UhfRadioButton9Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfRadioButtonConf { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <uhfradio::UhfRadioButtonConfValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfRadioButtonDrw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <uhfradio::UhfRadioButtonDrwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfRadioButtonVld { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <uhfradio::UhfRadioButtonVldValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfRadioMode { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <uhfradio::UhfRadioModeValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfRadioVolume1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UhfRadioVolume2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UnassignedSwitch { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <electricsystem::UnassignedSwitchValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::UvLighting { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VariometerVvi { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VhfAmRadioVolume1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VhfAmRadioVolume2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VoltMeter { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VoltmeterTest { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <electricsystem::VoltmeterTestValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::WarningPanelTest { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <electricsystem::WarningPanelTestValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::WeaponLaunchSeq { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <weapons::WeaponLaunchSeqValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::WeaponsMasterArm { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <weapons::WeaponsMasterArmValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::WeaponsPanel1DispBright { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::WeaponsPanel1OffOnStby { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <weapons::WeaponsPanel1OffOnStbyValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::WiperOnce { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <electricsystem::WiperOnceValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::WipersCopilot { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::WipersPilot { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Wp2LeftLamp { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Wp2RightLamp { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::YawNeedle { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
        }
    }
}

pub struct Sa342mBuilder<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> {
    controls: Vec<RegisteredControl<'a>, MAX_CONTROLS>,
    _packet_capacity: PhantomData<ProtocolParser<MAX_PACKET_DATA>>,
}

impl<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize>
    Sa342mBuilder<'a, MAX_CONTROLS, MAX_PACKET_DATA>
{
    pub fn new() -> Self {
        Self {
            controls: Vec::new(),
            _packet_capacity: PhantomData,
        }
    }

    pub fn register_ac115v_bus_nav<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ac115vBusNav {
                control: warningpanel::Ac115vBusNav::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_additional_fuel_tank<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electricsystem::AdditionalFuelTankValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdditionalFuelTank {
                control: electricsystem::AdditionalFuelTank::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_additional_fuel_tank_lamp<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdditionalFuelTankLamp {
                control: indicatorlights::AdditionalFuelTankLamp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf1_adf2_select<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(adf::Adf1Adf2SelectValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Adf1Adf2Select {
                control: adf::Adf1Adf2Select::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_background<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfBackground {
                control: rawgaugevalues::AdfBackground::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_distance100s<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfDistance100s {
                control: rawgaugevalues::AdfDistance100s::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_distance10s<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfDistance10s {
                control: rawgaugevalues::AdfDistance10s::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_distance1s<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfDistance1s {
                control: rawgaugevalues::AdfDistance1s::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_distance_value<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfDistanceValue {
                control: gaugevalues::AdfDistanceValue::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_flag_but<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfFlagBut {
                control: rawgaugevalues::AdfFlagBut::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_flag_cap<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfFlagCap {
                control: rawgaugevalues::AdfFlagCap::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_flag_counter<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfFlagCounter {
                control: rawgaugevalues::AdfFlagCounter::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_gain<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfGain {
                control: adf::AdfGain::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_heading_value<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfHeadingValue {
                control: gaugevalues::AdfHeadingValue::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_mode<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(adf::AdfModeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfMode {
                control: adf::AdfMode::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_nadir_needle<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfNadirNeedle {
                control: rawgaugevalues::AdfNadirNeedle::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_nav11<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfNav11 {
                control: adf::AdfNav11::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_nav110<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfNav110 {
                control: adf::AdfNav110::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_nav1100<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfNav1100 {
                control: adf::AdfNav1100::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_nav1_freq100s<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfNav1Freq100s {
                control: rawgaugevalues::AdfNav1Freq100s::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_nav1_freq10s<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfNav1Freq10s {
                control: rawgaugevalues::AdfNav1Freq10s::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_nav1_freq1s<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfNav1Freq1s {
                control: rawgaugevalues::AdfNav1Freq1s::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_nav1_freq_decimal<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfNav1FreqDecimal {
                control: rawgaugevalues::AdfNav1FreqDecimal::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_nav21<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfNav21 {
                control: adf::AdfNav21::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_nav210<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfNav210 {
                control: adf::AdfNav210::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_nav2100<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfNav2100 {
                control: adf::AdfNav2100::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_nav2_freq100s<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfNav2Freq100s {
                control: rawgaugevalues::AdfNav2Freq100s::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_nav2_freq10s<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfNav2Freq10s {
                control: rawgaugevalues::AdfNav2Freq10s::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_nav2_freq1s<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfNav2Freq1s {
                control: rawgaugevalues::AdfNav2Freq1s::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_nav2_freq_decimal<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfNav2FreqDecimal {
                control: rawgaugevalues::AdfNav2FreqDecimal::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_small_needle<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfSmallNeedle {
                control: rawgaugevalues::AdfSmallNeedle::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_tone_on_off<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(adf::AdfToneOnOffValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfToneOnOff {
                control: adf::AdfToneOnOff::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alarm_warning_lamp<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::AlarmWarningLamp {
                control: indicatorlights::AlarmWarningLamp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alternator<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electricsystem::AlternatorValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Alternator {
                control: electricsystem::Alternator::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alternator_warning<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::AlternatorWarning {
                control: warningpanel::AlternatorWarning::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_am_frequency_selector<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(amradio::AmFrequencySelectorValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmFrequencySelector {
                control: amradio::AmFrequencySelector::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_am_radio_freq100s<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmRadioFreq100s {
                control: rawgaugevalues::AmRadioFreq100s::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_am_radio_freq10s<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmRadioFreq10s {
                control: rawgaugevalues::AmRadioFreq10s::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_am_radio_freq1s<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmRadioFreq1s {
                control: rawgaugevalues::AmRadioFreq1s::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_am_radio_freq_hundredths<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmRadioFreqHundredths {
                control: rawgaugevalues::AmRadioFreqHundredths::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_am_radio_freq_tenths<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmRadioFreqTenths {
                control: rawgaugevalues::AmRadioFreqTenths::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_am_radio_frequency_dial_left<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmRadioFrequencyDialLeft {
                control: amradio::AmRadioFrequencyDialLeft::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_am_radio_frequency_dial_right<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmRadioFrequencyDialRight {
                control: amradio::AmRadioFrequencyDialRight::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_am_radio_lamp<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmRadioLamp {
                control: indicatorlights::AmRadioLamp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_am_radio_selector<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(amradio::AmRadioSelectorValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmRadioSelector {
                control: amradio::AmRadioSelector::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_anticoll_intensity<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AnticollIntensity {
                control: navlights::AnticollIntensity::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_anticoll_lights<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(navlights::AnticollLightsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AnticollLights {
                control: navlights::AnticollLights::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_art_horiz_ball<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ArtHorizBall {
                control: rawgaugevalues::ArtHorizBall::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_art_horiz_curseur<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ArtHorizCurseur {
                control: rawgaugevalues::ArtHorizCurseur::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_art_horiz_flag_gs<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ArtHorizFlagGs {
                control: rawgaugevalues::ArtHorizFlagGs::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_art_horiz_flag_hs<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ArtHorizFlagHs {
                control: rawgaugevalues::ArtHorizFlagHs::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_art_horiz_flag_loc<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ArtHorizFlagLoc {
                control: rawgaugevalues::ArtHorizFlagLoc::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_art_horiz_horizontal_bar<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ArtHorizHorizontalBar {
                control: rawgaugevalues::ArtHorizHorizontalBar::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_art_horiz_pitch<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ArtHorizPitch {
                control: rawgaugevalues::ArtHorizPitch::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_art_horiz_roll<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ArtHorizRoll {
                control: rawgaugevalues::ArtHorizRoll::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_art_horiz_vert_bar<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ArtHorizVertBar {
                control: rawgaugevalues::ArtHorizVertBar::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_art_horiz_w<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ArtHorizW {
                control: rawgaugevalues::ArtHorizW::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_auto_hover<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(autopilot::AutoHoverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AutoHover {
                control: autopilot::AutoHover::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_autopilot<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Autopilot {
                control: warningpanel::Autopilot::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_autopilot_button<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(autopilot::AutopilotButtonValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AutopilotButton {
                control: autopilot::AutopilotButton::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_autopilot_mode<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(autopilot::AutopilotModeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AutopilotMode {
                control: autopilot::AutopilotMode::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_autopilot_on_off<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(autopilot::AutopilotOnOffValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AutopilotOnOff {
                control: autopilot::AutopilotOnOff::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_autopilot_pitch<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(autopilot::AutopilotPitchValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AutopilotPitch {
                control: autopilot::AutopilotPitch::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_autopilot_roll<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(autopilot::AutopilotRollValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AutopilotRoll {
                control: autopilot::AutopilotRoll::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_autopilot_slave<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(autopilot::AutopilotSlaveValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AutopilotSlave {
                control: autopilot::AutopilotSlave::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_autopilot_trim<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(autopilot::AutopilotTrimValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AutopilotTrim {
                control: autopilot::AutopilotTrim::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_autopilot_yaw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(autopilot::AutopilotYawValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AutopilotYaw {
                control: autopilot::AutopilotYaw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_aux_fuel_tank<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AuxFuelTank {
                control: rawgaugevalues::AuxFuelTank::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_baro_alt_hundreds<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BaroAltHundreds {
                control: rawgaugevalues::BaroAltHundreds::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_baro_alt_thousands<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BaroAltThousands {
                control: rawgaugevalues::BaroAltThousands::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_baro_alt_value<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BaroAltValue {
                control: gaugevalues::BaroAltValue::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_baro_press_value<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BaroPressValue {
                control: gaugevalues::BaroPressValue::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_baro_pressure_hundreds<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BaroPressureHundreds {
                control: rawgaugevalues::BaroPressureHundreds::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_baro_pressure_tens<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BaroPressureTens {
                control: rawgaugevalues::BaroPressureTens::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_baro_pressure_thousands<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BaroPressureThousands {
                control: rawgaugevalues::BaroPressureThousands::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_baro_pressure_units<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BaroPressureUnits {
                control: rawgaugevalues::BaroPressureUnits::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_barometric_altimeter<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BarometricAltimeter {
                control: baroaltimeter::BarometricAltimeter::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_battery<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electricsystem::BatteryValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Battery {
                control: electricsystem::Battery::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_battery_warning<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::BatteryWarning {
                control: warningpanel::BatteryWarning::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_blocked_engine_warning_lamp<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::BlockedEngineWarningLamp {
                control: indicatorlights::BlockedEngineWarningLamp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_camera_centering<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(pe::CameraCenteringValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CameraCentering {
                control: pe::CameraCentering::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_camera_mode<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(pe::CameraModeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CameraMode {
                control: pe::CameraMode::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_camera_zoom_in<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(pe::CameraZoomInValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CameraZoomIn {
                control: pe::CameraZoomIn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_camera_zoom_out<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(pe::CameraZoomOutValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CameraZoomOut {
                control: pe::CameraZoomOut::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_center_console_light<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CenterConsoleLight {
                control: rawgaugevalues::CenterConsoleLight::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_chips_in_oil<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ChipsInOil {
                control: warningpanel::ChipsInOil::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_hour<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockHour {
                control: rawgaugevalues::ClockHour::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_mini<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockMini {
                control: rawgaugevalues::ClockMini::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_minute<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockMinute {
                control: rawgaugevalues::ClockMinute::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_push_button<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(clock::ClockPushButtonValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockPushButton {
                control: clock::ClockPushButton::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_reset<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(clock::ClockResetValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockReset {
                control: clock::ClockReset::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_second<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockSecond {
                control: rawgaugevalues::ClockSecond::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_set<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockSet {
                control: clock::ClockSet::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_start_stop<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(clock::ClockStartStopValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockStartStop {
                control: clock::ClockStartStop::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_console_lighting<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ConsoleLighting {
                control: lights::ConsoleLighting::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_convoy_fuel_tank_lamp<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ConvoyFuelTankLamp {
                control: indicatorlights::ConvoyFuelTankLamp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_convoy_tank<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electricsystem::ConvoyTankValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ConvoyTank {
                control: electricsystem::ConvoyTank::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_copilot_wiper<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electricsystem::CopilotWiperValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CopilotWiper {
                control: electricsystem::CopilotWiper::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cth_power<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(pe::CthPowerValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CthPower {
                control: pe::CthPower::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dashboard_lighting<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DashboardLighting {
                control: lights::DashboardLighting::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dem_start_lamp<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::DemStartLamp {
                control: indicatorlights::DemStartLamp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_position_light_left<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtPositionLightLeft {
                control: externalaircraftmodel::ExtPositionLightLeft::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_position_light_right<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtPositionLightRight {
                control: externalaircraftmodel::ExtPositionLightRight::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_strobe<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtStrobe {
                control: externalaircraftmodel::ExtStrobe::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fd_lamp_d_ope<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::FdLampDOpe {
                control: indicatorlights::FdLampDOpe::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fd_lamp_d_vide1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::FdLampDVide1 {
                control: indicatorlights::FdLampDVide1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fd_lamp_d_vide2<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::FdLampDVide2 {
                control: indicatorlights::FdLampDVide2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fd_lamp_g_ope<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::FdLampGOpe {
                control: indicatorlights::FdLampGOpe::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fd_lamp_g_vide1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::FdLampGVide1 {
                control: indicatorlights::FdLampGVide1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fd_lamp_g_vide2<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::FdLampGVide2 {
                control: indicatorlights::FdLampGVide2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fd_lamp_leu<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::FdLampLeu {
                control: indicatorlights::FdLampLeu::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fd_lamp_ve<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::FdLampVe {
                control: indicatorlights::FdLampVe::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flare_disp_fire_cap<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flaredispenser::FlareDispFireCapValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlareDispFireCap {
                control: flaredispenser::FlareDispFireCap::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flare_disp_side_select<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flaredispenser::FlareDispSideSelectValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlareDispSideSelect {
                control: flaredispenser::FlareDispSideSelect::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flare_dispense_button<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flaredispenser::FlareDispenseButtonValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlareDispenseButton {
                control: flaredispenser::FlareDispenseButton::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flare_dispense_mode<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flaredispenser::FlareDispenseModeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlareDispenseMode {
                control: flaredispenser::FlareDispenseMode::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flare_dispenser_off_on<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flaredispenser::FlareDispenserOffOnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlareDispenserOffOn {
                control: flaredispenser::FlareDispenserOffOn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fm_radio_button0<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fmradio::FmRadioButton0Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::FmRadioButton0 {
                control: fmradio::FmRadioButton0::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fm_radio_button1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fmradio::FmRadioButton1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::FmRadioButton1 {
                control: fmradio::FmRadioButton1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fm_radio_button2<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fmradio::FmRadioButton2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::FmRadioButton2 {
                control: fmradio::FmRadioButton2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fm_radio_button3<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fmradio::FmRadioButton3Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::FmRadioButton3 {
                control: fmradio::FmRadioButton3::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fm_radio_button4<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fmradio::FmRadioButton4Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::FmRadioButton4 {
                control: fmradio::FmRadioButton4::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fm_radio_button5<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fmradio::FmRadioButton5Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::FmRadioButton5 {
                control: fmradio::FmRadioButton5::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fm_radio_button6<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fmradio::FmRadioButton6Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::FmRadioButton6 {
                control: fmradio::FmRadioButton6::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fm_radio_button7<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fmradio::FmRadioButton7Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::FmRadioButton7 {
                control: fmradio::FmRadioButton7::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fm_radio_button8<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fmradio::FmRadioButton8Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::FmRadioButton8 {
                control: fmradio::FmRadioButton8::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fm_radio_button9<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fmradio::FmRadioButton9Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::FmRadioButton9 {
                control: fmradio::FmRadioButton9::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fm_radio_button_down<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fmradio::FmRadioButtonDownValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FmRadioButtonDown {
                control: fmradio::FmRadioButtonDown::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fm_radio_button_rc<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fmradio::FmRadioButtonRcValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FmRadioButtonRc {
                control: fmradio::FmRadioButtonRc::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fm_radio_button_up<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fmradio::FmRadioButtonUpValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FmRadioButtonUp {
                control: fmradio::FmRadioButtonUp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fm_radio_button_val<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fmradio::FmRadioButtonValValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FmRadioButtonVal {
                control: fmradio::FmRadioButtonVal::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fm_radio_button_x<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fmradio::FmRadioButtonXValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FmRadioButtonX {
                control: fmradio::FmRadioButtonX::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fm_radio_channel<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fmradio::FmRadioChannelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FmRadioChannel {
                control: fmradio::FmRadioChannel::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fm_radio_mode<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fmradio::FmRadioModeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FmRadioMode {
                control: fmradio::FmRadioMode::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fm_radio_volume1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FmRadioVolume1 {
                control: intercom::FmRadioVolume1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fm_radio_volume2<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FmRadioVolume2 {
                control: intercom::FmRadioVolume2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_formation_intensity<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FormationIntensity {
                control: navlights::FormationIntensity::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_formation_lights<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(navlights::FormationLightsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FormationLights {
                control: navlights::FormationLights::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fuel {
                control: rawgaugevalues::Fuel::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_flow_lever<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fuel::FuelFlowLeverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelFlowLever {
                control: fuel::FuelFlowLever::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_level_value<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelLevelValue {
                control: gaugevalues::FuelLevelValue::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_pump<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electricsystem::FuelPumpValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelPump {
                control: electricsystem::FuelPump::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_generator<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electricsystem::GeneratorValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Generator {
                control: electricsystem::Generator::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_generator_warning<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::GeneratorWarning {
                control: warningpanel::GeneratorWarning::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gyro_bpp_warning_lamp<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::GyroBppWarningLamp {
                control: indicatorlights::GyroBppWarningLamp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gyro_compass_heading<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::GyroCompassHeading {
                control: rawgaugevalues::GyroCompassHeading::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gyro_knob<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(autopilot::GyroKnobValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::GyroKnob {
                control: autopilot::GyroKnob::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gyro_needle<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::GyroNeedle {
                control: rawgaugevalues::GyroNeedle::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gyro_switch<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(autopilot::GyroSwitchValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::GyroSwitch {
                control: autopilot::GyroSwitch::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gyro_test_cover<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(autopilot::GyroTestCoverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::GyroTestCover {
                control: autopilot::GyroTestCover::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gyro_test_lamp<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::GyroTestLamp {
                control: indicatorlights::GyroTestLamp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gyro_test_switch<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(autopilot::GyroTestSwitchValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::GyroTestSwitch {
                control: autopilot::GyroTestSwitch::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gyro_trim_warning_lamp<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::GyroTrimWarningLamp {
                control: indicatorlights::GyroTrimWarningLamp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ha_adjust<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HaAdjust {
                control: flightcontrols::HaAdjust::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ha_source<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flightcontrols::HaSourceValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HaSource {
                control: flightcontrols::HaSource::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ha_unlock<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flightcontrols::HaUnlockValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HaUnlock {
                control: flightcontrols::HaUnlock::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_heading_value<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HeadingValue {
                control: gaugevalues::HeadingValue::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hot3_brightness<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Hot3Brightness {
                control: hot3::Hot3Brightness::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hot3_panel_test_off_on<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(hot3::Hot3PanelTestOffOnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Hot3PanelTestOffOn {
                control: hot3::Hot3PanelTestOffOn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hot3_station_select<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(hot3::Hot3StationSelectValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Hot3StationSelect {
                control: hot3::Hot3StationSelect::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hyd_test<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electricsystem::HydTestValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HydTest {
                control: electricsystem::HydTest::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ias_value<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IasValue {
                control: gaugevalues::IasValue::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_image_brightness_down<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(pe::ImageBrightnessDownValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ImageBrightnessDown {
                control: pe::ImageBrightnessDown::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_image_brightness_up<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(pe::ImageBrightnessUpValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ImageBrightnessUp {
                control: pe::ImageBrightnessUp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_image_focus_down<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(pe::ImageFocusDownValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ImageFocusDown {
                control: pe::ImageFocusDown::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_image_focus_up<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(pe::ImageFocusUpValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ImageFocusUp {
                control: pe::ImageFocusUp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_image_gain_left<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(pe::ImageGainLeftValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ImageGainLeft {
                control: pe::ImageGainLeft::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_image_gain_right<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(pe::ImageGainRightValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ImageGainRight {
                control: pe::ImageGainRight::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ind_air_speed_ias<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IndAirSpeedIas {
                control: rawgaugevalues::IndAirSpeedIas::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_inversed_image<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(pe::InversedImageValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::InversedImage {
                control: pe::InversedImage::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_inversed_symbology<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(pe::InversedSymbologyValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::InversedSymbology {
                control: pe::InversedSymbology::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_landing_light_extend<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(navlights::LandingLightExtendValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LandingLightExtend {
                control: navlights::LandingLightExtend::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_landing_light_retract<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(navlights::LandingLightRetractValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LandingLightRetract {
                control: navlights::LandingLightRetract::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_landing_lights<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(navlights::LandingLightsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LandingLights {
                control: navlights::LandingLights::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_laser_power<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(pe::LaserPowerValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LaserPower {
                control: pe::LaserPower::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lasing_button<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(pe::LasingButtonValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LasingButton {
                control: pe::LasingButton::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lasing_button_cover<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(pe::LasingButtonCoverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LasingButtonCover {
                control: pe::LasingButtonCover::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_master_arm<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(weapons::LeftMasterArmValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftMasterArm {
                control: weapons::LeftMasterArm::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_left_master_arm_cover<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(weapons::LeftMasterArmCoverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeftMasterArmCover {
                control: weapons::LeftMasterArmCover::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lights_norm_bnl<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(lights::LightsNormBnlValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightsNormBnl {
                control: lights::LightsNormBnl::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_low_fuel<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LowFuel {
                control: warningpanel::LowFuel::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_low_rotor_rpm<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LowRotorRpm {
                control: warningpanel::LowRotorRpm::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_low_turbine_rpm<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LowTurbineRpm {
                control: warningpanel::LowTurbineRpm::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_magnetic_brake<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(autopilot::MagneticBrakeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MagneticBrake {
                control: autopilot::MagneticBrake::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_magnetic_brake_button<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(controls::MagneticBrakeButtonValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MagneticBrakeButton {
                control: controls::MagneticBrakeButton::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_main_panel_light<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MainPanelLight {
                control: rawgaugevalues::MainPanelLight::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_missile_launch<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(pe::MissileLaunchValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MissileLaunch {
                control: pe::MissileLaunch::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_missile_launch_cover<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(pe::MissileLaunchCoverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MissileLaunchCover {
                control: pe::MissileLaunchCover::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nadir0_button<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(nadir::Nadir0ButtonValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Nadir0Button {
                control: nadir::Nadir0Button::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nadir1_button<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(nadir::Nadir1ButtonValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Nadir1Button {
                control: nadir::Nadir1Button::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nadir2_button<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(nadir::Nadir2ButtonValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Nadir2Button {
                control: nadir::Nadir2Button::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nadir3_button<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(nadir::Nadir3ButtonValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Nadir3Button {
                control: nadir::Nadir3Button::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nadir4_button<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(nadir::Nadir4ButtonValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Nadir4Button {
                control: nadir::Nadir4Button::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nadir5_button<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(nadir::Nadir5ButtonValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Nadir5Button {
                control: nadir::Nadir5Button::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nadir6_button<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(nadir::Nadir6ButtonValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Nadir6Button {
                control: nadir::Nadir6Button::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nadir7_button<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(nadir::Nadir7ButtonValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Nadir7Button {
                control: nadir::Nadir7Button::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nadir8_button<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(nadir::Nadir8ButtonValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Nadir8Button {
                control: nadir::Nadir8Button::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nadir9_button<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(nadir::Nadir9ButtonValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Nadir9Button {
                control: nadir::Nadir9Button::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nadir_aux_button<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(nadir::NadirAuxButtonValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::NadirAuxButton {
                control: nadir::NadirAuxButton::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nadir_brightness<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::NadirBrightness {
                control: rawgaugevalues::NadirBrightness::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nadir_dest_button<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(nadir::NadirDestButtonValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::NadirDestButton {
                control: nadir::NadirDestButton::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nadir_doppler_mode<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(nadir::NadirDopplerModeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::NadirDopplerMode {
                control: nadir::NadirDopplerMode::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nadir_down_arrow<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(nadir::NadirDownArrowValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::NadirDownArrow {
                control: nadir::NadirDownArrow::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nadir_eff_button<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(nadir::NadirEffButtonValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::NadirEffButton {
                control: nadir::NadirEffButton::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nadir_enter_button<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(nadir::NadirEnterButtonValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::NadirEnterButton {
                control: nadir::NadirEnterButton::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nadir_gel_button<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(nadir::NadirGelButtonValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::NadirGelButton {
                control: nadir::NadirGelButton::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nadir_geo_button<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(nadir::NadirGeoButtonValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::NadirGeoButton {
                control: nadir::NadirGeoButton::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nadir_heading_value<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::NadirHeadingValue {
                control: gaugevalues::NadirHeadingValue::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nadir_map_ind_button<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(nadir::NadirMapIndButtonValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::NadirMapIndButton {
                control: nadir::NadirMapIndButton::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nadir_on_off_brightness<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::NadirOnOffBrightness {
                control: nadir::NadirOnOffBrightness::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nadir_parameter<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(nadir::NadirParameterValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::NadirParameter {
                control: nadir::NadirParameter::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nadir_pol_button<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(nadir::NadirPolButtonValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::NadirPolButton {
                control: nadir::NadirPolButton::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nadir_pos_button<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(nadir::NadirPosButtonValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::NadirPosButton {
                control: nadir::NadirPosButton::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nav_lights<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(navlights::NavLightsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::NavLights {
                control: navlights::NavLights::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_panel_lighting<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(navlights::PanelLightingValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::PanelLighting {
                control: navlights::PanelLighting::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pilot_sight<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(weapons::PilotSightValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::PilotSight {
                control: weapons::PilotSight::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pilot_wiper<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electricsystem::PilotWiperValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::PilotWiper {
                control: electricsystem::PilotWiper::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pitch_needle<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::PitchNeedle {
                control: rawgaugevalues::PitchNeedle::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pitot<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electricsystem::PitotValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Pitot {
                control: electricsystem::Pitot::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pitot_tube_warning<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::PitotTubeWarning {
                control: warningpanel::PitotTubeWarning::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_quatre<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Quatre {
                control: rawgaugevalues::Quatre::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_alt_alarm_lamp<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarAltAlarmLamp {
                control: indicatorlights::RadarAltAlarmLamp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_alt_bug<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarAltBug {
                control: radaraltimeter::RadarAltBug::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_alt_flag_on_off<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarAltFlagOnOff {
                control: rawgaugevalues::RadarAltFlagOnOff::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_alt_flag_penne<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarAltFlagPenne {
                control: rawgaugevalues::RadarAltFlagPenne::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_alt_knob<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarAltKnob {
                control: rawgaugevalues::RadarAltKnob::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_alt_power<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarAltPower {
                control: radaraltimeter::RadarAltPower::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_alt_test<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(radaraltimeter::RadarAltTestValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarAltTest {
                control: radaraltimeter::RadarAltTest::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_alt_value<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarAltValue {
                control: gaugevalues::RadarAltValue::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_altimeter<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarAltimeter {
                control: rawgaugevalues::RadarAltimeter::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_altimeter_bug<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarAltimeterBug {
                control: rawgaugevalues::RadarAltimeterBug::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rearm_alter<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electricsystem::RearmAlterValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RearmAlter {
                control: electricsystem::RearmAlter::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rearm_gene<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electricsystem::RearmGeneValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RearmGene {
                control: electricsystem::RearmGene::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_red_lens_on_off<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(lights::RedLensOnOffValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RedLensOnOff {
                control: lights::RedLensOnOff::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_master_arm<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(weapons::RightMasterArmValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightMasterArm {
                control: weapons::RightMasterArm::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_right_master_arm_cover<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(weapons::RightMasterArmCoverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RightMasterArmCover {
                control: weapons::RightMasterArmCover::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rlt_idle_lamp<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RltIdleLamp {
                control: indicatorlights::RltIdleLamp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_roll_needle<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RollNeedle {
                control: rawgaugevalues::RollNeedle::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_roof_lamp<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RoofLamp {
                control: lights::RoofLamp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_roof_light<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RoofLight {
                control: rawgaugevalues::RoofLight::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rotor_brake<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rotors::RotorBrakeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RotorBrake {
                control: rotors::RotorBrake::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rotor_oil_pressure<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RotorOilPressure {
                control: warningpanel::RotorOilPressure::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rotor_rpm<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RotorRpm {
                control: rawgaugevalues::RotorRpm::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rotor_rpm_value<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RotorRpmValue {
                control: gaugevalues::RotorRpmValue::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_audio<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrAudio {
                control: rwr::RwrAudio::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_brightness<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrBrightness {
                control: rwr::RwrBrightness::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_light<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrLight {
                control: indicatorlights::RwrLight::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_marker<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rwr::RwrMarkerValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrMarker {
                control: rwr::RwrMarker::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_off_on_croc<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rwr::RwrOffOnCrocValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrOffOnCroc {
                control: rwr::RwrOffOnCroc::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_page<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(rwr::RwrPageValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrPage {
                control: rwr::RwrPage::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sand_filter<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electricsystem::SandFilterValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SandFilter {
                control: electricsystem::SandFilter::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sand_filter_lamp<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::SandFilterLamp {
                control: indicatorlights::SandFilterLamp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sand_filter_warning<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::SandFilterWarning {
                control: warningpanel::SandFilterWarning::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_standby_ha_adjust<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::StandbyHaAdjust {
                control: flightcontrols::StandbyHaAdjust::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_standby_ha_unlock<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flightcontrols::StandbyHaUnlockValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::StandbyHaUnlock {
                control: flightcontrols::StandbyHaUnlock::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_starter<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electricsystem::StarterValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Starter {
                control: electricsystem::Starter::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_stby_art_hor_curseur<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::StbyArtHorCurseur {
                control: rawgaugevalues::StbyArtHorCurseur::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_stby_art_hor_flag<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::StbyArtHorFlag {
                control: rawgaugevalues::StbyArtHorFlag::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_stby_art_hor_pitch<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::StbyArtHorPitch {
                control: rawgaugevalues::StbyArtHorPitch::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_stby_art_hor_roll<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::StbyArtHorRoll {
                control: rawgaugevalues::StbyArtHorRoll::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_stby_art_hor_w<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::StbyArtHorW {
                control: rawgaugevalues::StbyArtHorW::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_symbology_brightness_left<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(pe::SymbologyBrightnessLeftValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SymbologyBrightnessLeft {
                control: pe::SymbologyBrightnessLeft::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_symbology_brightness_right<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(pe::SymbologyBrightnessRightValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SymbologyBrightnessRight {
                control: pe::SymbologyBrightnessRight::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_temp_engine_oil<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TempEngineOil {
                control: rawgaugevalues::TempEngineOil::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_temp_external<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TempExternal {
                control: rawgaugevalues::TempExternal::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_toggle_vdo_vth<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(pe::ToggleVdoVthValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ToggleVdoVth {
                control: pe::ToggleVdoVth::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_torque_bug<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TorqueBug {
                control: rawgaugevalues::TorqueBug::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_torque_bug_adjust<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TorqueBugAdjust {
                control: torque::TorqueBugAdjust::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_torque_needle<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TorqueNeedle {
                control: rawgaugevalues::TorqueNeedle::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_torque_test<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(torque::TorqueTestValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::TorqueTest {
                control: torque::TorqueTest::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_torque_value<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TorqueValue {
                control: gaugevalues::TorqueValue::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_torque_warning_lamp<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::TorqueWarningLamp {
                control: indicatorlights::TorqueWarningLamp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_turbine_oil_pressure<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::TurbineOilPressure {
                control: warningpanel::TurbineOilPressure::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_turbine_rpm<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TurbineRpm {
                control: rawgaugevalues::TurbineRpm::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_turbine_rpm_value<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TurbineRpmValue {
                control: gaugevalues::TurbineRpmValue::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tv_brightness<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TvBrightness {
                control: tv::TvBrightness::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tv_brightness_value<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TvBrightnessValue {
                control: rawgaugevalues::TvBrightnessValue::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tv_contrast<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TvContrast {
                control: tv::TvContrast::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tv_cover<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(tv::TvCoverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::TvCover {
                control: tv::TvCover::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tv_lamp<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::TvLamp {
                control: indicatorlights::TvLamp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tv_on_off<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(tv::TvOnOffValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::TvOnOff {
                control: tv::TvOnOff::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_page_knob<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfPageKnob {
                control: uhfradio::UhfPageKnob::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_radio_button0<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhfradio::UhfRadioButton0Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfRadioButton0 {
                control: uhfradio::UhfRadioButton0::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_radio_button1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhfradio::UhfRadioButton1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfRadioButton1 {
                control: uhfradio::UhfRadioButton1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_radio_button2<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhfradio::UhfRadioButton2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfRadioButton2 {
                control: uhfradio::UhfRadioButton2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_radio_button3<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhfradio::UhfRadioButton3Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfRadioButton3 {
                control: uhfradio::UhfRadioButton3::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_radio_button4<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhfradio::UhfRadioButton4Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfRadioButton4 {
                control: uhfradio::UhfRadioButton4::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_radio_button5<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhfradio::UhfRadioButton5Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfRadioButton5 {
                control: uhfradio::UhfRadioButton5::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_radio_button6<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhfradio::UhfRadioButton6Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfRadioButton6 {
                control: uhfradio::UhfRadioButton6::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_radio_button7<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhfradio::UhfRadioButton7Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfRadioButton7 {
                control: uhfradio::UhfRadioButton7::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_radio_button8<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhfradio::UhfRadioButton8Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfRadioButton8 {
                control: uhfradio::UhfRadioButton8::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_radio_button9<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhfradio::UhfRadioButton9Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfRadioButton9 {
                control: uhfradio::UhfRadioButton9::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_radio_button_conf<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhfradio::UhfRadioButtonConfValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfRadioButtonConf {
                control: uhfradio::UhfRadioButtonConf::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_radio_button_drw<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhfradio::UhfRadioButtonDrwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfRadioButtonDrw {
                control: uhfradio::UhfRadioButtonDrw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_radio_button_vld<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhfradio::UhfRadioButtonVldValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfRadioButtonVld {
                control: uhfradio::UhfRadioButtonVld::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_radio_mode<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhfradio::UhfRadioModeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfRadioMode {
                control: uhfradio::UhfRadioMode::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_radio_volume1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfRadioVolume1 {
                control: intercom::UhfRadioVolume1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_radio_volume2<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfRadioVolume2 {
                control: intercom::UhfRadioVolume2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_unassigned_switch<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electricsystem::UnassignedSwitchValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UnassignedSwitch {
                control: electricsystem::UnassignedSwitch::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uv_lighting<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UvLighting {
                control: lights::UvLighting::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_variometer_vvi<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VariometerVvi {
                control: rawgaugevalues::VariometerVvi::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf_am_radio_volume1<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfAmRadioVolume1 {
                control: intercom::VhfAmRadioVolume1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf_am_radio_volume2<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfAmRadioVolume2 {
                control: intercom::VhfAmRadioVolume2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_volt_meter<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VoltMeter {
                control: rawgaugevalues::VoltMeter::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_voltmeter_test<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electricsystem::VoltmeterTestValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::VoltmeterTest {
                control: electricsystem::VoltmeterTest::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_warning_panel_test<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electricsystem::WarningPanelTestValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::WarningPanelTest {
                control: electricsystem::WarningPanelTest::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_weapon_launch_seq<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(weapons::WeaponLaunchSeqValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::WeaponLaunchSeq {
                control: weapons::WeaponLaunchSeq::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_weapons_master_arm<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(weapons::WeaponsMasterArmValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::WeaponsMasterArm {
                control: weapons::WeaponsMasterArm::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_weapons_panel1_disp_bright<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::WeaponsPanel1DispBright {
                control: weapons::WeaponsPanel1DispBright::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_weapons_panel1_off_on_stby<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(weapons::WeaponsPanel1OffOnStbyValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::WeaponsPanel1OffOnStby {
                control: weapons::WeaponsPanel1OffOnStby::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_wiper_once<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electricsystem::WiperOnceValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::WiperOnce {
                control: electricsystem::WiperOnce::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_wipers_copilot<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::WipersCopilot {
                control: rawgaugevalues::WipersCopilot::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_wipers_pilot<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::WipersPilot {
                control: rawgaugevalues::WipersPilot::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_wp2_left_lamp<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Wp2LeftLamp {
                control: indicatorlights::Wp2LeftLamp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_wp2_right_lamp<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Wp2RightLamp {
                control: indicatorlights::Wp2RightLamp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_yaw_needle<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::YawNeedle {
                control: rawgaugevalues::YawNeedle::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn build(self) -> Sa342m<'a, MAX_CONTROLS, MAX_PACKET_DATA> {
        Sa342m {
            parser: ProtocolParser::new(),
            controls: self.controls,
            saw_frame: false,
        }
    }
}

impl<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> Default
    for Sa342mBuilder<'a, MAX_CONTROLS, MAX_PACKET_DATA>
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

pub struct Sa342m<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> {
    parser: ProtocolParser<MAX_PACKET_DATA>,
    controls: Vec<RegisteredControl<'a>, MAX_CONTROLS>,
    saw_frame: bool,
}

impl<const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize>
    Sa342m<'_, MAX_CONTROLS, MAX_PACKET_DATA>
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
