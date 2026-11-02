//! Generated from model-definitions/UH-1H.json.
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
    pub enum AdfBandValue {
        Select1,
        Select2,
        Select3,
    }
    pub type AdfBandValueCallback<'a> = alloc::boxed::Box<dyn FnMut(AdfBandValue) + 'a>;
    impl core::convert::TryFrom<u16> for AdfBandValue {
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

    /// ADF Band
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct AdfBand(Selector<5372, 192, 6>);
    impl AdfBand {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdfBand {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdfBand {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AdfBfoSwValue {
        Select1,
        Select2,
    }
    pub type AdfBfoSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(AdfBfoSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for AdfBfoSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// BFO Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AdfBfoSw(Selector<5372, 8, 3>);
    impl AdfBfoSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdfBfoSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdfBfoSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADF Frequency
    /// Output: gauge position
    pub struct AdfFreq(AnalogGauge<5158, 65535, 0>);
    impl AdfFreq {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdfFreq {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdfFreq {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADF Gain
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct AdfGain(LimitedDial<5374, 65535, 0>);
    impl AdfGain {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
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
    pub enum AdfLoopLrValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
    }
    pub type AdfLoopLrValueCallback<'a> = alloc::boxed::Box<dyn FnMut(AdfLoopLrValue) + 'a>;
    impl core::convert::TryFrom<u16> for AdfLoopLrValue {
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

    /// ADF LOOP Left Low Speed / Left High Speed /Center / Right Low Speed / Right High Speed
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct AdfLoopLr(Selector<5372, 7, 0>);
    impl AdfLoopLr {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdfLoopLr {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdfLoopLr {
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

    /// ADF Mode OFF / ADF / ANT / LOOP
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct AdfMode(Selector<5372, 48, 4>);
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

    /// ADF Signal Level
    /// Output: gauge position
    pub struct AdfSignal(AnalogGauge<5160, 65535, 0>);
    impl AdfSignal {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdfSignal {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdfSignal {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADF Tune
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct AdfTune(AnalogDial<5376, 65535, 0>);
    impl AdfTune {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdfTune {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdfTune {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod altimeteroperator {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// 100 Foot Counter
    /// Output: gauge position
    pub struct Oalt100(AnalogGauge<5186, 65535, 0>);
    impl Oalt100 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Oalt100 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Oalt100 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// 1000 Foot Counter
    /// Output: gauge position
    pub struct Oalt1000(AnalogGauge<5184, 65535, 0>);
    impl Oalt1000 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Oalt1000 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Oalt1000 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// 10000 Foot Counter
    /// Output: gauge position
    pub struct Oalt10000(AnalogGauge<5182, 65535, 0>);
    impl Oalt10000 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Oalt10000 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Oalt10000 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Code Off Flag
    /// Output: gauge position
    pub struct OaltCodeOffFlag(AnalogGauge<5194, 65535, 0>);
    impl OaltCodeOffFlag {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for OaltCodeOffFlag {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for OaltCodeOffFlag {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Pressure Drum Counter 0
    /// Output: gauge position
    pub struct OaltPress0(AnalogGauge<5192, 65535, 0>);
    impl OaltPress0 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for OaltPress0 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for OaltPress0 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Pressure Drum Counter 1
    /// Output: gauge position
    pub struct OaltPress1(AnalogGauge<5190, 65535, 0>);
    impl OaltPress1 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for OaltPress1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for OaltPress1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Pressure Drum Counter 2
    /// Output: gauge position
    pub struct OaltPress2(AnalogGauge<5188, 65535, 0>);
    impl OaltPress2 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for OaltPress2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for OaltPress2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Altimeter Pointer
    /// Output: gauge position
    pub struct OaltPtr(AnalogGauge<5180, 65535, 0>);
    impl OaltPtr {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for OaltPtr {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for OaltPtr {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod altimeterpilot {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// 100 Foot Counter
    /// Output: gauge position
    pub struct Palt100(AnalogGauge<5200, 65535, 0>);
    impl Palt100 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Palt100 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Palt100 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// 1000 Foot Counter
    /// Output: gauge position
    pub struct Palt1000(AnalogGauge<5198, 65535, 0>);
    impl Palt1000 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Palt1000 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Palt1000 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// 10000 Foot Counter
    /// Output: gauge position
    pub struct Palt10000(AnalogGauge<5196, 65535, 0>);
    impl Palt10000 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Palt10000 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Palt10000 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Pressure Indicator
    /// Output: gauge position
    pub struct PaltPress(AnalogGauge<5202, 65535, 0>);
    impl PaltPress {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for PaltPress {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for PaltPress {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod armamentpanel {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// ARMED Indicator (red)
    /// Output: 0 if light is off, 1 if light is on
    pub struct ArmedInd(Led<5142, 16, 4>);
    impl ArmedInd {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ArmedInd {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ArmedInd {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GunSelValue {
        Select1,
        Select2,
        Select3,
    }
    pub type GunSelValueCallback<'a> = alloc::boxed::Box<dyn FnMut(GunSelValue) + 'a>;
    impl core::convert::TryFrom<u16> for GunSelValue {
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

    /// Gun Selector LEFT / BOTH / RIGHT
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct GunSel(Selector<5378, 24576, 13>);
    impl GunSel {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for GunSel {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for GunSel {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum JtsnBtnValue {
        Select1,
        Select2,
    }
    pub type JtsnBtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(JtsnBtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for JtsnBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Jettison Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct JtsnBtn(Selector<5398, 64, 6>);
    impl JtsnBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for JtsnBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for JtsnBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum JtsnCoverValue {
        Select1,
        Select2,
    }
    pub type JtsnCoverValueCallback<'a> = alloc::boxed::Box<dyn FnMut(JtsnCoverValue) + 'a>;
    impl core::convert::TryFrom<u16> for JtsnCoverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Jettison Switch Cover
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct JtsnCover(Selector<5398, 32, 5>);
    impl JtsnCover {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for JtsnCover {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for JtsnCover {
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

    /// Master Arm OFF - SAFE - ARMED
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct MasterArmSw(Selector<5378, 6144, 11>);
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

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RocketPairValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
        Select6,
        Select7,
        Select8,
    }
    pub type RocketPairValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RocketPairValue) + 'a>;
    impl core::convert::TryFrom<u16> for RocketPairValue {
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

    /// Rocket Pair
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct RocketPair(Selector<5398, 28, 2>);
    impl RocketPair {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RocketPair {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RocketPair {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RocketResetValue {
        Select1,
        Select2,
    }
    pub type RocketResetValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RocketResetValue) + 'a>;
    impl core::convert::TryFrom<u16> for RocketResetValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Rocket Reset
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RocketReset(Selector<5378, 32768, 15>);
    impl RocketReset {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RocketReset {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RocketReset {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RocketSelValue {
        Select1,
        Select2,
        Select3,
    }
    pub type RocketSelValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RocketSelValue) + 'a>;
    impl core::convert::TryFrom<u16> for RocketSelValue {
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

    /// 7.62 / 2.75 / 40
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct RocketSel(Selector<5398, 3, 0>);
    impl RocketSel {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RocketSel {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RocketSel {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// SAFE Indicator (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct SafeInd(Led<5142, 32, 5>);
    impl SafeInd {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for SafeInd {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SafeInd {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// X130 Armed Lamp (blue)
    /// Output: 0 if light is off, 1 if light is on
    pub struct X130Armed(Led<5406, 4096, 12>);
    impl X130Armed {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for X130Armed {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for X130Armed {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod cautionlightspanel {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// ENGINE OIL PRESS (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct ClA0(Led<5140, 1, 0>);
    impl ClA0 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ClA0 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClA0 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// AUX FUEL LOW (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct ClA1(Led<5140, 1024, 10>);
    impl ClA1 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ClA1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClA1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ENGINE ICING (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct ClB0(Led<5140, 2, 1>);
    impl ClB0 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ClB0 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClB0 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// XMSN OIL PRESS (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct ClB1(Led<5140, 2048, 11>);
    impl ClB1 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ClB1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClB1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ENGINE ICE JET (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct ClC0(Led<5140, 4, 2>);
    impl ClC0 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ClC0 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClC0 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// XMSN OIL HOT (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct ClC1(Led<5140, 4096, 12>);
    impl ClC1 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ClC1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClC1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ENGINE CHIP DET (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct ClD0(Led<5140, 8, 3>);
    impl ClD0 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ClD0 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClD0 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// HYD PRESSURE (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct ClD1(Led<5140, 8192, 13>);
    impl ClD1 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ClD1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClD1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// LEFT FUEL BOOST (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct ClE0(Led<5140, 16, 4>);
    impl ClE0 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ClE0 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClE0 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ENGINE INLET AIR (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct ClE1(Led<5140, 16384, 14>);
    impl ClE1 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ClE1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClE1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RIGHT FUEL BOOST (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct ClF0(Led<5140, 32, 5>);
    impl ClF0 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ClF0 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClF0 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// INST INVERTER (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct ClF1(Led<5140, 32768, 15>);
    impl ClF1 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ClF1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClF1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ENG FUEL PUMP (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct ClG0(Led<5140, 64, 6>);
    impl ClG0 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ClG0 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClG0 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// DC GENERATOR (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct ClG1(Led<5142, 1, 0>);
    impl ClG1 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ClG1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClG1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// 20 MIN FUEL (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct ClH0(Led<5140, 128, 7>);
    impl ClH0 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ClH0 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClH0 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// EXTERNAL POWER (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct ClH1(Led<5142, 2, 1>);
    impl ClH1 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ClH1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClH1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FUEL FILTER (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct ClI0(Led<5140, 256, 8>);
    impl ClI0 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ClI0 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClI0 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// CHIP DETECTOR (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct ClI1(Led<5142, 4, 2>);
    impl ClI1 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ClI1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClI1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// GOV EMER (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct ClJ0(Led<5140, 512, 9>);
    impl ClJ0 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ClJ0 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClJ0 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// IFF (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct ClJ1(Led<5142, 8, 3>);
    impl ClJ1 {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for ClJ1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClJ1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ClpBrightDimSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type ClpBrightDimSwValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(ClpBrightDimSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for ClpBrightDimSwValue {
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

    /// Bright/Dim Rocker Switch
    /// Input: set the switch position -- 0 = held left/down, 1 = centered, 2 = held right/up
    /// Output: selector position
    pub struct ClpBrightDimSw(Selector<5304, 384, 7>);
    impl ClpBrightDimSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ClpBrightDimSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClpBrightDimSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ClpResetTestSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type ClpResetTestSwValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(ClpResetTestSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for ClpResetTestSwValue {
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

    /// Test/Reset Rocker Switch
    /// Input: set the switch position -- 0 = held left/down, 1 = centered, 2 = held right/up
    /// Output: selector position
    pub struct ClpResetTestSw(Selector<5304, 96, 5>);
    impl ClpResetTestSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ClpResetTestSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClpResetTestSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod centerconsole {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ChipDetSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type ChipDetSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(ChipDetSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for ChipDetSwValue {
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

    /// Chip Detector Rocker Switch
    /// Input: set the switch position -- 0 = held left/down, 1 = centered, 2 = held right/up
    /// Output: selector position
    pub struct ChipDetSw(Selector<5310, 192, 6>);
    impl ChipDetSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ChipDetSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ChipDetSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ForceTrimSwValue {
        Select1,
        Select2,
    }
    pub type ForceTrimSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(ForceTrimSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for ForceTrimSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Force Trim Enable Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ForceTrimSw(Selector<5310, 1024, 10>);
    impl ForceTrimSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ForceTrimSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ForceTrimSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum HydContSwValue {
        Select1,
        Select2,
    }
    pub type HydContSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(HydContSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for HydContSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Hydraulic Control Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct HydContSw(Selector<5310, 512, 9>);
    impl HydContSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HydContSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HydContSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod circuitbreakers {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Cb28vValue {
        Select1,
        Select2,
    }
    pub type Cb28vValueCallback<'a> = alloc::boxed::Box<dyn FnMut(Cb28vValue) + 'a>;
    impl core::convert::TryFrom<u16> for Cb28vValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// 28V Trans (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct Cb28v(Selector<5302, 1024, 10>);
    impl Cb28v {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Cb28v {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Cb28v {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbAirValveValue {
        Select1,
        Select2,
    }
    pub type CbAirValveValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbAirValveValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbAirValveValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Cabin Heater Air Valve (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbAirValve(Selector<5298, 1024, 10>);
    impl CbAirValve {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbAirValve {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbAirValve {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbAmRadioValue {
        Select1,
        Select2,
    }
    pub type CbAmRadioValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbAmRadioValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbAmRadioValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// VHF AM Radio
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbAmRadio(Selector<5296, 32, 5>);
    impl CbAmRadio {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbAmRadio {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbAmRadio {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbAntiIceValue {
        Select1,
        Select2,
    }
    pub type CbAntiIceValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbAntiIceValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbAntiIceValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Engine anti-ice (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbAntiIce(Selector<5298, 8192, 13>);
    impl CbAntiIce {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbAntiIce {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbAntiIce {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbAnticollValue {
        Select1,
        Select2,
    }
    pub type CbAnticollValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbAnticollValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbAnticollValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Anticollision light (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbAnticoll(Selector<5298, 1, 0>);
    impl CbAnticoll {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbAnticoll {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbAnticoll {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbArc102Value {
        Select1,
        Select2,
    }
    pub type CbArc102ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbArc102Value) + 'a>;
    impl core::convert::TryFrom<u16> for CbArc102Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ARC-102 HF Static Inverter (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbArc102(Selector<5294, 32768, 15>);
    impl CbArc102 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbArc102 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbArc102 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbCableValue {
        Select1,
        Select2,
    }
    pub type CbCableValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbCableValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbCableValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Rescoue hoist cable cutter (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbCable(Selector<5296, 256, 8>);
    impl CbCable {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbCable {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbCable {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbCautionLtsValue {
        Select1,
        Select2,
    }
    pub type CbCautionLtsValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbCautionLtsValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbCautionLtsValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Caution lights (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbCautionLts(Selector<5298, 32, 5>);
    impl CbCautionLts {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbCautionLts {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbCautionLts {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbCockpitLtsValue {
        Select1,
        Select2,
    }
    pub type CbCockpitLtsValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbCockpitLtsValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbCockpitLtsValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Cockpit lights (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbCockpitLts(Selector<5298, 16, 4>);
    impl CbCockpitLts {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbCockpitLts {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbCockpitLts {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbConsoleLtsValue {
        Select1,
        Select2,
    }
    pub type CbConsoleLtsValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbConsoleLtsValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbConsoleLtsValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Console lights (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbConsoleLts(Selector<5298, 64, 6>);
    impl CbConsoleLts {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbConsoleLts {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbConsoleLts {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbCpltAttd1Value {
        Select1,
        Select2,
    }
    pub type CbCpltAttd1ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbCpltAttd1Value) + 'a>;
    impl core::convert::TryFrom<u16> for CbCpltAttd1Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Copilot ATTD1 (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbCpltAttd1(Selector<5302, 64, 6>);
    impl CbCpltAttd1 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbCpltAttd1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbCpltAttd1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbCpltAttd2Value {
        Select1,
        Select2,
    }
    pub type CbCpltAttd2ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbCpltAttd2Value) + 'a>;
    impl core::convert::TryFrom<u16> for CbCpltAttd2Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Coilot ATTD2 (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbCpltAttd2(Selector<5302, 128, 7>);
    impl CbCpltAttd2 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbCpltAttd2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbCpltAttd2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbCrsindValue {
        Select1,
        Select2,
    }
    pub type CbCrsindValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbCrsindValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbCrsindValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Course Ind (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbCrsind(Selector<5304, 1, 0>);
    impl CbCrsind {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbCrsind {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbCrsind {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbDomeLtsValue {
        Select1,
        Select2,
    }
    pub type CbDomeLtsValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbDomeLtsValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbDomeLtsValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Dome lights (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbDomeLts(Selector<5298, 8, 3>);
    impl CbDomeLts {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbDomeLts {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbDomeLts {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbEngValue {
        Select1,
        Select2,
    }
    pub type CbEngValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbEngValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbEngValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pressure Eng (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbEng(Selector<5302, 32768, 15>);
    impl CbEng {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbEng {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbEng {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbFailRelayValue {
        Select1,
        Select2,
    }
    pub type CbFailRelayValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbFailRelayValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbFailRelayValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Fail Relay (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbFailRelay(Selector<5302, 2048, 11>);
    impl CbFailRelay {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbFailRelay {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbFailRelay {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbFireValue {
        Select1,
        Select2,
    }
    pub type CbFireValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbFireValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbFireValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Fire detector (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbFire(Selector<5298, 16384, 14>);
    impl CbFire {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbFire {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbFire {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbFm2RadioValue {
        Select1,
        Select2,
    }
    pub type CbFm2RadioValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbFm2RadioValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbFm2RadioValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// FM 2 Radio (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbFm2Radio(Selector<5296, 16, 4>);
    impl CbFm2Radio {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbFm2Radio {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbFm2Radio {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbFmRadioValue {
        Select1,
        Select2,
    }
    pub type CbFmRadioValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbFmRadioValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbFmRadioValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// FM Radio
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbFmRadio(Selector<5296, 4, 2>);
    impl CbFmRadio {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbFmRadio {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbFmRadio {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbFuelValue {
        Select1,
        Select2,
    }
    pub type CbFuelValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbFuelValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbFuelValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pressure Fuel (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbFuel(Selector<5302, 4096, 12>);
    impl CbFuel {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbFuel {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbFuel {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbFuelQtyValue {
        Select1,
        Select2,
    }
    pub type CbFuelQtyValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbFuelQtyValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbFuelQtyValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Fuel Quantity (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbFuelQty(Selector<5302, 512, 9>);
    impl CbFuelQty {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbFuelQty {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbFuelQty {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbFuelTransValue {
        Select1,
        Select2,
    }
    pub type CbFuelTransValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbFuelTransValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbFuelTransValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Fuel TRANS (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbFuelTrans(Selector<5300, 16384, 14>);
    impl CbFuelTrans {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbFuelTrans {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbFuelTrans {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbFuelValvesValue {
        Select1,
        Select2,
    }
    pub type CbFuelValvesValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbFuelValvesValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbFuelValvesValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Fuel valves (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbFuelValves(Selector<5300, 32768, 15>);
    impl CbFuelValves {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbFuelValves {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbFuelValves {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbFusLtsValue {
        Select1,
        Select2,
    }
    pub type CbFusLtsValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbFusLtsValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbFusLtsValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Fuselage lights (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbFusLts(Selector<5298, 2, 1>);
    impl CbFusLts {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbFusLts {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbFusLts {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbGenBusValue {
        Select1,
        Select2,
    }
    pub type CbGenBusValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbGenBusValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbGenBusValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Generator & Bus Reset (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbGenBus(Selector<5300, 512, 9>);
    impl CbGenBus {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbGenBus {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbGenBus {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbGovValue {
        Select1,
        Select2,
    }
    pub type CbGovValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbGovValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbGovValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Governor Control (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbGov(Selector<5300, 2048, 11>);
    impl CbGov {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbGov {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbGov {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbGyroValue {
        Select1,
        Select2,
    }
    pub type CbGyroValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbGyroValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbGyroValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Gyro Compass (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbGyro(Selector<5302, 256, 8>);
    impl CbGyro {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbGyro {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbGyro {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbHeatedBlanket1Value {
        Select1,
        Select2,
    }
    pub type CbHeatedBlanket1ValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(CbHeatedBlanket1Value) + 'a>;
    impl core::convert::TryFrom<u16> for CbHeatedBlanket1Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Heated blanket 1 (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbHeatedBlanket1(Selector<5302, 1, 0>);
    impl CbHeatedBlanket1 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbHeatedBlanket1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbHeatedBlanket1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbHeatedBlanket2Value {
        Select1,
        Select2,
    }
    pub type CbHeatedBlanket2ValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(CbHeatedBlanket2Value) + 'a>;
    impl core::convert::TryFrom<u16> for CbHeatedBlanket2Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Heated blanket 2 (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbHeatedBlanket2(Selector<5302, 2, 1>);
    impl CbHeatedBlanket2 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbHeatedBlanket2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbHeatedBlanket2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbHfAntValue {
        Select1,
        Select2,
    }
    pub type CbHfAntValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbHfAntValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbHfAntValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// HF ANT COUPLR (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbHfAnt(Selector<5296, 1, 0>);
    impl CbHfAnt {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbHfAnt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbHfAnt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbHfArc102Value {
        Select1,
        Select2,
    }
    pub type CbHfArc102ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbHfArc102Value) + 'a>;
    impl core::convert::TryFrom<u16> for CbHfArc102Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// HF ARC-102 (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbHfArc102(Selector<5296, 2, 1>);
    impl CbHfArc102 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbHfArc102 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbHfArc102 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbHookValue {
        Select1,
        Select2,
    }
    pub type CbHookValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbHookValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbHookValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Cargo hook release (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbHook(Selector<5300, 16, 4>);
    impl CbHook {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbHook {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbHook {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbHydctlValue {
        Select1,
        Select2,
    }
    pub type CbHydctlValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbHydctlValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbHydctlValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// HYD Control (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbHydctl(Selector<5300, 4, 2>);
    impl CbHydctl {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbHydctl {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbHydctl {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbIdleStopValue {
        Select1,
        Select2,
    }
    pub type CbIdleStopValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbIdleStopValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbIdleStopValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// IDLE Stop release (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbIdleStop(Selector<5300, 4096, 12>);
    impl CbIdleStop {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbIdleStop {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbIdleStop {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbIffApx1Value {
        Select1,
        Select2,
    }
    pub type CbIffApx1ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbIffApx1Value) + 'a>;
    impl core::convert::TryFrom<u16> for CbIffApx1Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// IFF APX 1 (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbIffApx1(Selector<5294, 128, 7>);
    impl CbIffApx1 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbIffApx1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbIffApx1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbIffApx2Value {
        Select1,
        Select2,
    }
    pub type CbIffApx2ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbIffApx2Value) + 'a>;
    impl core::convert::TryFrom<u16> for CbIffApx2Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// IFF APX 2 (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbIffApx2(Selector<5294, 256, 8>);
    impl CbIffApx2 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbIffApx2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbIffApx2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbIgnValue {
        Select1,
        Select2,
    }
    pub type CbIgnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbIgnValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbIgnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Ignition system (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbIgn(Selector<5302, 8, 3>);
    impl CbIgn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbIgn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbIgn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbInstLtsValue {
        Select1,
        Select2,
    }
    pub type CbInstLtsValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbInstLtsValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbInstLtsValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// INST Panel lights (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbInstLts(Selector<5298, 128, 7>);
    impl CbInstLts {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbInstLts {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbInstLts {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbInstSecLtsValue {
        Select1,
        Select2,
    }
    pub type CbInstSecLtsValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbInstSecLtsValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbInstSecLtsValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// INST SEC lights (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbInstSecLts(Selector<5298, 256, 8>);
    impl CbInstSecLts {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbInstSecLts {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbInstSecLts {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbIntCpltValue {
        Select1,
        Select2,
    }
    pub type CbIntCpltValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbIntCpltValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbIntCpltValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Intercom Copilot (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbIntCplt(Selector<5294, 8192, 13>);
    impl CbIntCplt {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbIntCplt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbIntCplt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbIntPltValue {
        Select1,
        Select2,
    }
    pub type CbIntPltValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbIntPltValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbIntPltValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Intercom Pilot
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbIntPlt(Selector<5294, 16384, 14>);
    impl CbIntPlt {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbIntPlt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbIntPlt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbInvtrValue {
        Select1,
        Select2,
    }
    pub type CbInvtrValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbInvtrValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbInvtrValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Inverter CTRL (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbInvtr(Selector<5300, 128, 7>);
    impl CbInvtr {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbInvtr {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbInvtr {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbJtsnValue {
        Select1,
        Select2,
    }
    pub type CbJtsnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbJtsnValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbJtsnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// EXT Stores jettison (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbJtsn(Selector<5300, 32, 5>);
    impl CbJtsn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbJtsn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbJtsn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbKy20Value {
        Select1,
        Select2,
    }
    pub type CbKy20ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbKy20Value) + 'a>;
    impl core::convert::TryFrom<u16> for CbKy20Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// KY-28 Secure Radio (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbKy20(Selector<5296, 2048, 11>);
    impl CbKy20 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbKy20 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbKy20 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbLdgltsValue {
        Select1,
        Select2,
    }
    pub type CbLdgltsValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbLdgltsValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbLdgltsValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Landing Lights power (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbLdglts(Selector<5296, 16384, 14>);
    impl CbLdglts {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbLdglts {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbLdglts {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbLfNavValue {
        Select1,
        Select2,
    }
    pub type CbLfNavValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbLfNavValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbLfNavValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// LF Nav. (ARN-83)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbLfNav(Selector<5294, 4096, 12>);
    impl CbLfNav {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbLfNav {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbLfNav {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbLhFuelValue {
        Select1,
        Select2,
    }
    pub type CbLhFuelValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbLhFuelValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbLhFuelValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// LH fuel boost pump (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbLhFuel(Selector<5298, 32768, 15>);
    impl CbLhFuel {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbLhFuel {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbLhFuel {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbMainInvtrValue {
        Select1,
        Select2,
    }
    pub type CbMainInvtrValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbMainInvtrValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbMainInvtrValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Main Inverter PWR (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbMainInvtr(Selector<5300, 256, 8>);
    impl CbMainInvtr {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbMainInvtr {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbMainInvtr {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbMarkerBeaconValue {
        Select1,
        Select2,
    }
    pub type CbMarkerBeaconValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(CbMarkerBeaconValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbMarkerBeaconValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Marker Beacon
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbMarkerBeacon(Selector<5294, 1024, 10>);
    impl CbMarkerBeacon {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbMarkerBeacon {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbMarkerBeacon {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbNavLtsValue {
        Select1,
        Select2,
    }
    pub type CbNavLtsValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbNavLtsValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbNavLtsValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Navigation lights (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbNavLts(Selector<5298, 4, 2>);
    impl CbNavLts {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbNavLts {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbNavLts {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbOutletValveValue {
        Select1,
        Select2,
    }
    pub type CbOutletValveValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbOutletValveValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbOutletValveValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Cabin Heater Outlet Valve (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbOutletValve(Selector<5298, 512, 9>);
    impl CbOutletValve {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbOutletValve {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbOutletValve {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbPitotTubeValue {
        Select1,
        Select2,
    }
    pub type CbPitotTubeValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbPitotTubeValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbPitotTubeValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pitot tube (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbPitotTube(Selector<5296, 64, 6>);
    impl CbPitotTube {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbPitotTube {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbPitotTube {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbPltAttd1Value {
        Select1,
        Select2,
    }
    pub type CbPltAttd1ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbPltAttd1Value) + 'a>;
    impl core::convert::TryFrom<u16> for CbPltAttd1Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pilot ATTD1 (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbPltAttd1(Selector<5302, 16, 4>);
    impl CbPltAttd1 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbPltAttd1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbPltAttd1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbPltAttd2Value {
        Select1,
        Select2,
    }
    pub type CbPltAttd2ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbPltAttd2Value) + 'a>;
    impl core::convert::TryFrom<u16> for CbPltAttd2Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pilot ATTD2 (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbPltAttd2(Selector<5302, 32, 5>);
    impl CbPltAttd2 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbPltAttd2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbPltAttd2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbPorxWarnValue {
        Select1,
        Select2,
    }
    pub type CbPorxWarnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbPorxWarnValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbPorxWarnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Prox. warn. (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbPorxWarn(Selector<5294, 512, 9>);
    impl CbPorxWarn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbPorxWarn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbPorxWarn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbRescueContValue {
        Select1,
        Select2,
    }
    pub type CbRescueContValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbRescueContValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbRescueContValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Rescue hoist CTL (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbRescueCont(Selector<5296, 128, 7>);
    impl CbRescueCont {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbRescueCont {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbRescueCont {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbRescuePwrValue {
        Select1,
        Select2,
    }
    pub type CbRescuePwrValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbRescuePwrValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbRescuePwrValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Rescue Hoist Power (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbRescuePwr(Selector<5298, 2048, 11>);
    impl CbRescuePwr {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbRescuePwr {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbRescuePwr {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbRhFuelValue {
        Select1,
        Select2,
    }
    pub type CbRhFuelValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbRhFuelValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbRhFuelValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// RH fuel boost pumps (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbRhFuel(Selector<5300, 8192, 13>);
    impl CbRhFuel {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbRhFuel {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbRhFuel {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbRpmWarnValue {
        Select1,
        Select2,
    }
    pub type CbRpmWarnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbRpmWarnValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbRpmWarnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// RPM Warning System (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbRpmWarn(Selector<5298, 4096, 12>);
    impl CbRpmWarn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbRpmWarn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbRpmWarn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbSlCtlValue {
        Select1,
        Select2,
    }
    pub type CbSlCtlValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbSlCtlValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbSlCtlValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Landing and Search Lights Control (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbSlCtl(Selector<5296, 32768, 15>);
    impl CbSlCtl {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbSlCtl {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbSlCtl {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbSlPwrValue {
        Select1,
        Select2,
    }
    pub type CbSlPwrValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbSlPwrValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbSlPwrValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Search Light power (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbSlPwr(Selector<5296, 8192, 13>);
    impl CbSlPwr {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbSlPwr {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbSlPwr {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbSpareInvtrValue {
        Select1,
        Select2,
    }
    pub type CbSpareInvtrValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbSpareInvtrValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbSpareInvtrValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Spare Inverter (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbSpareInvtr(Selector<5300, 64, 6>);
    impl CbSpareInvtr {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbSpareInvtr {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbSpareInvtr {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbStarterValue {
        Select1,
        Select2,
    }
    pub type CbStarterValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbStarterValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbStarterValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Starter Relay (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbStarter(Selector<5296, 4096, 12>);
    impl CbStarter {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbStarter {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbStarter {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbStbyGenValue {
        Select1,
        Select2,
    }
    pub type CbStbyGenValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbStbyGenValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbStbyGenValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// STBY Generator Field (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbStbyGen(Selector<5300, 1024, 10>);
    impl CbStbyGen {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbStbyGen {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbStbyGen {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbTempValue {
        Select1,
        Select2,
    }
    pub type CbTempValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbTempValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbTempValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// TEMP indicator (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbTemp(Selector<5300, 2, 1>);
    impl CbTemp {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbTemp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbTemp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbTorqValue {
        Select1,
        Select2,
    }
    pub type CbTorqValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbTorqValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbTorqValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pressure Torque (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbTorq(Selector<5302, 8192, 13>);
    impl CbTorq {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbTorq {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbTorq {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbTrimValue {
        Select1,
        Select2,
    }
    pub type CbTrimValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbTrimValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbTrimValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Force Trim (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbTrim(Selector<5300, 8, 3>);
    impl CbTrim {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbTrim {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbTrim {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbTurnSlipValue {
        Select1,
        Select2,
    }
    pub type CbTurnSlipValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbTurnSlipValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbTurnSlipValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Turn & Slip Indicator
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbTurnSlip(Selector<5300, 1, 0>);
    impl CbTurnSlip {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbTurnSlip {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbTurnSlip {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbUhfRadioValue {
        Select1,
        Select2,
    }
    pub type CbUhfRadioValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbUhfRadioValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbUhfRadioValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// UHF Radio
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbUhfRadio(Selector<5296, 8, 3>);
    impl CbUhfRadio {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbUhfRadio {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbUhfRadio {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbVhfNavValue {
        Select1,
        Select2,
    }
    pub type CbVhfNavValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbVhfNavValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbVhfNavValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// VHF Nav. (ARN-82)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbVhfNav(Selector<5294, 2048, 11>);
    impl CbVhfNav {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbVhfNav {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbVhfNav {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbVmValue {
        Select1,
        Select2,
    }
    pub type CbVmValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbVmValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbVmValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Voltmeter Non Ess Bus (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbVm(Selector<5302, 4, 2>);
    impl CbVm {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbVm {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbVm {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbWiperCpltValue {
        Select1,
        Select2,
    }
    pub type CbWiperCpltValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbWiperCpltValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbWiperCpltValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Wind wiper CPLT
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbWiperCplt(Selector<5296, 512, 9>);
    impl CbWiperCplt {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbWiperCplt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbWiperCplt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbWiperPltValue {
        Select1,
        Select2,
    }
    pub type CbWiperPltValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbWiperPltValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbWiperPltValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Wind wiper PLT
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbWiperPlt(Selector<5296, 1024, 10>);
    impl CbWiperPlt {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbWiperPlt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbWiperPlt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CbXmsnValue {
        Select1,
        Select2,
    }
    pub type CbXmsnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CbXmsnValue) + 'a>;
    impl core::convert::TryFrom<u16> for CbXmsnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pressure XMSN (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CbXmsn(Selector<5302, 16384, 14>);
    impl CbXmsn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CbXmsn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CbXmsn {
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

    /// Adjustment Dial
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct ClockAdjust(AnalogDial<5316, 65535, 0>);
    impl ClockAdjust {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ClockAdjust {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClockAdjust {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Adjustment Dial Pull
    /// Input: toggle switch state
    /// Output: selector position
    pub struct ClockAdjustPull(Selector<5308, 32768, 15>);
    impl ClockAdjustPull {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ClockAdjustPull {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClockAdjustPull {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod collective {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GovRpmSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type GovRpmSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(GovRpmSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for GovRpmSwValue {
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

    /// Governor RPM Rocker Switch
    /// Input: set the switch position -- 0 = held left/down, 1 = centered, 2 = held right/up
    /// Output: selector position
    pub struct GovRpmSw(Selector<5310, 6144, 11>);
    impl GovRpmSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for GovRpmSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for GovRpmSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GovRpmSw2Value {
        Select1,
        Select2,
        Select3,
    }
    pub type GovRpmSw2ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(GovRpmSw2Value) + 'a>;
    impl core::convert::TryFrom<u16> for GovRpmSw2Value {
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

    /// Copilot Governor RPM Rocker Switch
    /// Input: set the switch position -- 0 = held left/down, 1 = centered, 2 = held right/up
    /// Output: selector position
    pub struct GovRpmSw2(Selector<5432, 3, 0>);
    impl GovRpmSw2 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for GovRpmSw2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for GovRpmSw2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LdgLightSwValue {
        Select1,
        Select2,
    }
    pub type LdgLightSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(LdgLightSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for LdgLightSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Landing Lights Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LdgLightSw(Selector<5372, 32768, 15>);
    impl LdgLightSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LdgLightSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LdgLightSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LdgLightSw2Value {
        Select1,
        Select2,
    }
    pub type LdgLightSw2ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(LdgLightSw2Value) + 'a>;
    impl core::convert::TryFrom<u16> for LdgLightSw2Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Copilot Landing Lights Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LdgLightSw2(Selector<5432, 4, 2>);
    impl LdgLightSw2 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LdgLightSw2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LdgLightSw2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LdgLtCtrlValue {
        Select1,
        Select2,
        Select3,
    }
    pub type LdgLtCtrlValueCallback<'a> = alloc::boxed::Box<dyn FnMut(LdgLtCtrlValue) + 'a>;
    impl core::convert::TryFrom<u16> for LdgLtCtrlValue {
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

    /// Landing Lights Control Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct LdgLtCtrl(Selector<5378, 12, 2>);
    impl LdgLtCtrl {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LdgLtCtrl {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LdgLtCtrl {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LdgLtCtrl2Value {
        Select1,
        Select2,
        Select3,
    }
    pub type LdgLtCtrl2ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(LdgLtCtrl2Value) + 'a>;
    impl core::convert::TryFrom<u16> for LdgLtCtrl2Value {
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

    /// Copilot Landing Lights Control Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct LdgLtCtrl2(Selector<5432, 96, 5>);
    impl LdgLtCtrl2 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LdgLtCtrl2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LdgLtCtrl2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SearchLightSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type SearchLightSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(SearchLightSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for SearchLightSwValue {
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

    /// Search Light STOW / OFF / ON
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct SearchLightSw(Selector<5378, 3, 0>);
    impl SearchLightSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SearchLightSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SearchLightSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SearchLightSw2Value {
        Select1,
        Select2,
        Select3,
    }
    pub type SearchLightSw2ValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(SearchLightSw2Value) + 'a>;
    impl core::convert::TryFrom<u16> for SearchLightSw2Value {
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

    /// Copilot Search Light STOW / OFF / ON
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct SearchLightSw2(Selector<5432, 24, 3>);
    impl SearchLightSw2 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SearchLightSw2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SearchLightSw2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Throttle Axis
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct Throttle(LimitedDial<5318, 65535, 0>);
    impl Throttle {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Throttle {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Throttle {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Copilot Throttle Axis
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct Throttle2(LimitedDial<5430, 65535, 0>);
    impl Throttle2 {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Throttle2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Throttle2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ThrottleStopValue {
        Select1,
        Select2,
    }
    pub type ThrottleStopValueCallback<'a> = alloc::boxed::Box<dyn FnMut(ThrottleStopValue) + 'a>;
    impl core::convert::TryFrom<u16> for ThrottleStopValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Throttle Stop Switch, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ThrottleStop(Selector<5310, 8, 3>);
    impl ThrottleStop {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ThrottleStop {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ThrottleStop {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum ThrottleStop2Value {
        Select1,
        Select2,
    }
    pub type ThrottleStop2ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(ThrottleStop2Value) + 'a>;
    impl core::convert::TryFrom<u16> for ThrottleStop2Value {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Copilot Throttle Stop Switch, ON/OFF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct ThrottleStop2(Selector<5418, 32768, 15>);
    impl ThrottleStop2 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ThrottleStop2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ThrottleStop2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod copilotrmi {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// RMI Course Pointer 1
    /// Output: gauge position
    pub struct RmiCrs1(AnalogGauge<5174, 65535, 0>);
    impl RmiCrs1 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RmiCrs1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RmiCrs1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RMI Course Pointer 2
    /// Output: gauge position
    pub struct RmiCrs2(AnalogGauge<5176, 65535, 0>);
    impl RmiCrs2 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RmiCrs2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RmiCrs2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RMI Heading
    /// Output: gauge position
    pub struct RmiHdg(AnalogGauge<5178, 65535, 0>);
    impl RmiHdg {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RmiHdg {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RmiHdg {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod countermeasures {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CmArmSwValue {
        Select1,
        Select2,
    }
    pub type CmArmSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CmArmSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for CmArmSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// SAFE / ARMED Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CmArmSw(Selector<5406, 16, 4>);
    impl CmArmSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CmArmSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CmArmSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CmArmedTestValue {
        Select1,
        Select2,
    }
    pub type CmArmedTestValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CmArmedTestValue) + 'a>;
    impl core::convert::TryFrom<u16> for CmArmedTestValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Armed Lamp Test
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CmArmedTest(Selector<5406, 128, 7>);
    impl CmArmedTest {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CmArmedTest {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CmArmedTest {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Chaff Counter Display (Number)
    /// Output: Chaff Counter Display (Number)
    pub struct CmChaffcntDisplay(Metadata<5418, 32256, 9>);
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

    /// Chaff Counter (String)
    /// Output: Chaff Counter (String)
    pub struct CmChaffcntDisplayStr(Display<5410, 2>);
    impl CmChaffcntDisplayStr {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for CmChaffcntDisplayStr {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CmChaffcntDisplayStr {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CmChaffcntResetValue {
        Select1,
        Select2,
    }
    pub type CmChaffcntResetValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(CmChaffcntResetValue) + 'a>;
    impl core::convert::TryFrom<u16> for CmChaffcntResetValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Chaff Counter Reset Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CmChaffcntReset(Selector<5406, 512, 9>);
    impl CmChaffcntReset {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CmChaffcntReset {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CmChaffcntReset {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CmFlareBtnValue {
        Select1,
        Select2,
    }
    pub type CmFlareBtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CmFlareBtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for CmFlareBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Flare Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CmFlareBtn(Selector<5406, 64, 6>);
    impl CmFlareBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CmFlareBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CmFlareBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Flare Counter Display (Number)
    /// Output: Flare Counter Display (Number)
    pub struct CmFlarecntDisplay(Metadata<5418, 504, 3>);
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

    /// Flare Counter (String)
    /// Output: Flare Counter (String)
    pub struct CmFlarecntDisplayStr(Display<5408, 2>);
    impl CmFlarecntDisplayStr {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for CmFlarecntDisplayStr {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CmFlarecntDisplayStr {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CmFlarecntResetValue {
        Select1,
        Select2,
    }
    pub type CmFlarecntResetValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(CmFlarecntResetValue) + 'a>;
    impl core::convert::TryFrom<u16> for CmFlarecntResetValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Flare Counter Reset Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CmFlarecntReset(Selector<5406, 256, 8>);
    impl CmFlarecntReset {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CmFlarecntReset {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CmFlarecntReset {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CmManPgrmSwValue {
        Select1,
        Select2,
    }
    pub type CmManPgrmSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CmManPgrmSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for CmManPgrmSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// MAN / PGRM Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CmManPgrmSw(Selector<5406, 32, 5>);
    impl CmManPgrmSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CmManPgrmSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CmManPgrmSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CmRippleCoverValue {
        Select1,
        Select2,
    }
    pub type CmRippleCoverValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CmRippleCoverValue) + 'a>;
    impl core::convert::TryFrom<u16> for CmRippleCoverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Ripple Fire Cover
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CmRippleCover(Selector<5406, 4, 2>);
    impl CmRippleCover {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CmRippleCover {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CmRippleCover {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CmRippleSwValue {
        Select1,
        Select2,
    }
    pub type CmRippleSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CmRippleSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for CmRippleSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Ripple Fire Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CmRippleSw(Selector<5406, 8, 3>);
    impl CmRippleSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CmRippleSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CmRippleSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod courseindicator {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Rotating Course Card
    /// Output: gauge position
    pub struct CiCourseCard(AnalogGauge<5152, 65535, 0>);
    impl CiCourseCard {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CiCourseCard {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CiCourseCard {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// From Marker
    /// Output: gauge position
    pub struct CiFrom(AnalogGauge<5150, 65535, 0>);
    impl CiFrom {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CiFrom {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CiFrom {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Horizontal Bar
    /// Output: gauge position
    pub struct CiHbar(AnalogGauge<5146, 65535, 0>);
    impl CiHbar {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CiHbar {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CiHbar {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Horizontal OFF Warning Flag
    /// Output: gauge position
    pub struct CiHoff(AnalogGauge<5156, 65535, 0>);
    impl CiHoff {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CiHoff {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CiHoff {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// To Marker
    /// Output: gauge position
    pub struct CiTo(AnalogGauge<5148, 65535, 0>);
    impl CiTo {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CiTo {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CiTo {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Vertical Bar
    /// Output: gauge position
    pub struct CiVbar(AnalogGauge<5144, 65535, 0>);
    impl CiVbar {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CiVbar {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CiVbar {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Vertical OFF Warning Flag
    /// Output: gauge position
    pub struct CiVoff(AnalogGauge<5154, 65535, 0>);
    impl CiVoff {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CiVoff {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CiVoff {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod cyclic {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CargoRelCpValue {
        Select1,
        Select2,
    }
    pub type CargoRelCpValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CargoRelCpValue) + 'a>;
    impl core::convert::TryFrom<u16> for CargoRelCpValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Cargo Release CoPilot
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CargoRelCp(Selector<5432, 256, 8>);
    impl CargoRelCp {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CargoRelCp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CargoRelCp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CargoRelPValue {
        Select1,
        Select2,
    }
    pub type CargoRelPValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CargoRelPValue) + 'a>;
    impl core::convert::TryFrom<u16> for CargoRelPValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Cargo Release Pilot
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CargoRelP(Selector<5432, 128, 7>);
    impl CargoRelP {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CargoRelP {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CargoRelP {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RadioIcsSwValue {
        Select1,
        Select2,
    }
    pub type RadioIcsSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RadioIcsSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for RadioIcsSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Radio/ICS Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RadioIcsSw(Selector<5330, 32768, 15>);
    impl RadioIcsSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RadioIcsSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadioIcsSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum TrimCpltValue {
        Select1,
        Select2,
    }
    pub type TrimCpltValueCallback<'a> = alloc::boxed::Box<dyn FnMut(TrimCpltValue) + 'a>;
    impl core::convert::TryFrom<u16> for TrimCpltValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Force Trim (Copilot Side)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct TrimCplt(Selector<5406, 2, 1>);
    impl TrimCplt {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TrimCplt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TrimCplt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum TrimPltValue {
        Select1,
        Select2,
    }
    pub type TrimPltValueCallback<'a> = alloc::boxed::Box<dyn FnMut(TrimPltValue) + 'a>;
    impl core::convert::TryFrom<u16> for TrimPltValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Force Trim (Pilot Side)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct TrimPlt(Selector<5406, 1, 0>);
    impl TrimPlt {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TrimPlt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TrimPlt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod doors {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DoorLPtrValue {
        Select1,
        Select2,
    }
    pub type DoorLPtrValueCallback<'a> = alloc::boxed::Box<dyn FnMut(DoorLPtrValue) + 'a>;
    impl core::convert::TryFrom<u16> for DoorLPtrValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Door Left
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct DoorLPtr(Selector<5406, 8192, 13>);
    impl DoorLPtr {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for DoorLPtr {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for DoorLPtr {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DoorRPtrValue {
        Select1,
        Select2,
    }
    pub type DoorRPtrValueCallback<'a> = alloc::boxed::Box<dyn FnMut(DoorRPtrValue) + 'a>;
    impl core::convert::TryFrom<u16> for DoorRPtrValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Door Right
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct DoorRPtr(Selector<5406, 16384, 14>);
    impl DoorRPtr {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for DoorRPtr {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for DoorRPtr {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod electricalsystem {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AcVmSrcValue {
        Select1,
        Select2,
        Select3,
    }
    pub type AcVmSrcValueCallback<'a> = alloc::boxed::Box<dyn FnMut(AcVmSrcValue) + 'a>;
    impl core::convert::TryFrom<u16> for AcVmSrcValue {
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

    /// AC Voltmeter Source
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct AcVmSrc(Selector<5294, 24, 3>);
    impl AcVmSrc {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AcVmSrc {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AcVmSrc {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum BatSwValue {
        Select1,
        Select2,
    }
    pub type BatSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(BatSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for BatSwValue {
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
    pub struct BatSw(Selector<5142, 8192, 13>);
    impl BatSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for BatSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for BatSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DcVmSrcValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
    }
    pub type DcVmSrcValueCallback<'a> = alloc::boxed::Box<dyn FnMut(DcVmSrcValue) + 'a>;
    impl core::convert::TryFrom<u16> for DcVmSrcValue {
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

    /// DC Voltmeter Source
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct DcVmSrc(Selector<5294, 7, 0>);
    impl DcVmSrc {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for DcVmSrc {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for DcVmSrc {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum InverterSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type InverterSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(InverterSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for InverterSwValue {
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

    /// Inverter Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct InverterSw(Selector<5294, 96, 5>);
    impl InverterSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for InverterSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for InverterSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NonEssBusSwValue {
        Select1,
        Select2,
    }
    pub type NonEssBusSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(NonEssBusSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for NonEssBusSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// NON ESS BUS Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct NonEssBusSw(Selector<5142, 32768, 15>);
    impl NonEssBusSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for NonEssBusSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for NonEssBusSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum StarterGenSwValue {
        Select1,
        Select2,
    }
    pub type StarterGenSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(StarterGenSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for StarterGenSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Starter/Stdby GEN
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct StarterGenSw(Selector<5142, 16384, 14>);
    impl StarterGenSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for StarterGenSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for StarterGenSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod enginepanel {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EmerGovSwValue {
        Select1,
        Select2,
    }
    pub type EmerGovSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(EmerGovSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for EmerGovSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Emergency Govenor Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct EmerGovSw(Selector<5310, 256, 8>);
    impl EmerGovSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EmerGovSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EmerGovSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum EngDeiceValue {
        Select1,
        Select2,
    }
    pub type EngDeiceValueCallback<'a> = alloc::boxed::Box<dyn FnMut(EngDeiceValue) + 'a>;
    impl core::convert::TryFrom<u16> for EngDeiceValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// De-Ice Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct EngDeice(Selector<5310, 16, 4>);
    impl EngDeice {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EngDeice {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EngDeice {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum LowRpmAudioValue {
        Select1,
        Select2,
    }
    pub type LowRpmAudioValueCallback<'a> = alloc::boxed::Box<dyn FnMut(LowRpmAudioValue) + 'a>;
    impl core::convert::TryFrom<u16> for LowRpmAudioValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Low RPM Audio Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct LowRpmAudio(Selector<5310, 32, 5>);
    impl LowRpmAudio {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for LowRpmAudio {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LowRpmAudio {
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
    pub struct ExtPositionLightLeft(Metadata<5406, 32768, 15>);
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
    pub struct ExtPositionLightRight(Metadata<5418, 1, 0>);
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
    pub struct ExtStrobe(Metadata<5418, 2, 1>);
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

    /// Weight ON Skids
    /// Output: Weight ON Skids
    pub struct ExtWowSkid(Metadata<5418, 4, 2>);
    impl ExtWowSkid {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ExtWowSkid {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ExtWowSkid {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod flexsight {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SightArmPltValue {
        Select1,
        Select2,
    }
    pub type SightArmPltValueCallback<'a> = alloc::boxed::Box<dyn FnMut(SightArmPltValue) + 'a>;
    impl core::convert::TryFrom<u16> for SightArmPltValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pilot Sight Armed / Safe
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct SightArmPlt(Selector<5398, 512, 9>);
    impl SightArmPlt {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SightArmPlt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SightArmPlt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Pilot Sight Elevation
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct SightElevPlt(LimitedDial<5404, 65535, 0>);
    impl SightElevPlt {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SightElevPlt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SightElevPlt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Copilot Sighting System Intensity Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct SightIntenCplt(LimitedDial<5400, 65535, 0>);
    impl SightIntenCplt {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SightIntenCplt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SightIntenCplt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Pilot Sight Intensity Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct SightIntenPlt(LimitedDial<5402, 65535, 0>);
    impl SightIntenPlt {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SightIntenPlt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SightIntenPlt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SightLampSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type SightLampSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(SightLampSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for SightLampSwValue {
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

    /// Sighting Station Lamp Switch BACKUP / OFF / MAIN
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct SightLampSw(Selector<5398, 384, 7>);
    impl SightLampSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SightLampSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SightLampSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum SightPwrPltValue {
        Select1,
        Select2,
    }
    pub type SightPwrPltValueCallback<'a> = alloc::boxed::Box<dyn FnMut(SightPwrPltValue) + 'a>;
    impl core::convert::TryFrom<u16> for SightPwrPltValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Pilot Sight Off / On
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct SightPwrPlt(Selector<5398, 1024, 10>);
    impl SightPwrPlt {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SightPwrPlt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SightPwrPlt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod frontdash {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// AC Voltage
    /// Output: gauge position
    pub struct AcV(AnalogGauge<5230, 65535, 0>);
    impl AcV {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AcV {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AcV {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AdfVorSwValue {
        Select1,
        Select2,
    }
    pub type AdfVorSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(AdfVorSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for AdfVorSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// ADF / VOR Switch: VOR / ADF
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AdfVorSw(Selector<5378, 512, 9>);
    impl AdfVorSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdfVorSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdfVorSw {
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
    pub struct AdiPitchTrim(LimitedDial<5324, 65535, 0>);
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

    /// Copilot ADI Pitch Trim Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct AdiPitchTrimCplt(LimitedDial<5320, 65535, 0>);
    impl AdiPitchTrimCplt {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdiPitchTrimCplt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdiPitchTrimCplt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// ADI Roll Trim Knob
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct AdiRollTrim(LimitedDial<5322, 65535, 0>);
    impl AdiRollTrim {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdiRollTrim {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdiRollTrim {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Copilot Altimeter Pressure Adjustment
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct AltAdjCplt(AnalogDial<5326, 65535, 0>);
    impl AltAdjCplt {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AltAdjCplt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AltAdjCplt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Pilot Altimeter Pressure Adjustment
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct AltAdjPlt(AnalogDial<5328, 65535, 0>);
    impl AltAdjPlt {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AltAdjPlt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AltAdjPlt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum BeaconSensingSwValue {
        Select1,
        Select2,
    }
    pub type BeaconSensingSwValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(BeaconSensingSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for BeaconSensingSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Marker Beacon Sensing LOW / HIGH
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct BeaconSensingSw(Selector<5356, 32768, 15>);
    impl BeaconSensingSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for BeaconSensingSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for BeaconSensingSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Marker Beacon Volume
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct BeaconVol(LimitedDial<5370, 65535, 0>);
    impl BeaconVol {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for BeaconVol {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for BeaconVol {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Clock Hours
    /// Output: gauge position
    pub struct ClockH(AnalogGauge<5240, 65535, 0>);
    impl ClockH {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ClockH {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClockH {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Clock Seconds
    /// Output: gauge position
    pub struct ClockM(AnalogGauge<5242, 65535, 0>);
    impl ClockM {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ClockM {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClockM {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Clock Seconds
    /// Output: gauge position
    pub struct ClockS(AnalogGauge<5244, 65535, 0>);
    impl ClockS {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ClockS {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ClockS {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CpltAdiCageValue {
        Select1,
        Select2,
    }
    pub type CpltAdiCageValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CpltAdiCageValue) + 'a>;
    impl core::convert::TryFrom<u16> for CpltAdiCageValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Copilot ADI Cage
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CpltAdiCage(Selector<5310, 16384, 14>);
    impl CpltAdiCage {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CpltAdiCage {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CpltAdiCage {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// CDI CRS Selector Knob
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct CrsKnob(AnalogDial<5396, 65535, 0>);
    impl CrsKnob {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CrsKnob {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CrsKnob {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// DC Voltage
    /// Output: gauge position
    pub struct DcV(AnalogGauge<5228, 65535, 0>);
    impl DcV {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for DcV {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for DcV {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Engine Oil Pressure
    /// Output: gauge position
    pub struct EngOilPress(AnalogGauge<5204, 65535, 0>);
    impl EngOilPress {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EngOilPress {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EngOilPress {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Engine Oil Temperature
    /// Output: gauge position
    pub struct EngOilTemp(AnalogGauge<5206, 65535, 0>);
    impl EngOilTemp {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EngOilTemp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EngOilTemp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Engine RPM
    /// Output: gauge position
    pub struct EngRpm(AnalogGauge<5218, 65535, 0>);
    impl EngRpm {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for EngRpm {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for EngRpm {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Exhaust Temperature
    /// Output: gauge position
    pub struct ExhaustTemp(AnalogGauge<5216, 65535, 0>);
    impl ExhaustTemp {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ExhaustTemp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ExhaustTemp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// FIRE Warning Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct FireInd(Led<5142, 64, 6>);
    impl FireInd {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for FireInd {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FireInd {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FireTestBtnValue {
        Select1,
        Select2,
    }
    pub type FireTestBtnValueCallback<'a> = alloc::boxed::Box<dyn FnMut(FireTestBtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for FireTestBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Fire Test Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct FireTestBtn(Selector<5310, 8192, 13>);
    impl FireTestBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FireTestBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FireTestBtn {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Fuel Pressure
    /// Output: gauge position
    pub struct FuelPress(AnalogGauge<5236, 65535, 0>);
    impl FuelPress {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FuelPress {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FuelPress {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Fuel Quantity
    /// Output: gauge position
    pub struct FuelQty(AnalogGauge<5238, 65535, 0>);
    impl FuelQty {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for FuelQty {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for FuelQty {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Gas Producer RPM
    /// Output: gauge position
    pub struct GasRpm(AnalogGauge<5224, 65535, 0>);
    impl GasRpm {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for GasRpm {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for GasRpm {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Gas Producer RPM Tenths
    /// Output: gauge position
    pub struct GasRpmT(AnalogGauge<5222, 65535, 0>);
    impl GasRpmT {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for GasRpmT {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for GasRpmT {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum GyroModeSwValue {
        Select1,
        Select2,
    }
    pub type GyroModeSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(GyroModeSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for GyroModeSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// DG / Slave Gyro Mode: MAG / DG
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct GyroModeSw(Selector<5378, 1024, 10>);
    impl GyroModeSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for GyroModeSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for GyroModeSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// HDG SET Knob
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct HdgSet(AnalogDial<5392, 65535, 0>);
    impl HdgSet {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HdgSet {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HdgSet {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Compass Synchronizing
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct HdgSync(AnalogDial<5394, 65535, 0>);
    impl HdgSync {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for HdgSync {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for HdgSync {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Low RPM Warning Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct LowRpmInd(Led<5142, 128, 7>);
    impl LowRpmInd {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for LowRpmInd {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for LowRpmInd {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Main Generator Load
    /// Output: gauge position
    pub struct MainGenLoad(AnalogGauge<5232, 65535, 0>);
    impl MainGenLoad {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MainGenLoad {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MainGenLoad {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// MASTER CAUTION Light (yellow)
    /// Output: 0 if light is off, 1 if light is on
    pub struct MasterCautionInd(Led<5142, 256, 8>);
    impl MasterCautionInd {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for MasterCautionInd {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MasterCautionInd {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Radar Altimeter High Setting
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct RadarAltHi(AnalogDial<5414, 65535, 0>);
    impl RadarAltHi {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RadarAltHi {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadarAltHi {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Radar Altimeter Low Setting
    /// Input: turn the dial left or right
    /// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
    pub struct RadarAltLo(AnalogDial<5412, 65535, 0>);
    impl RadarAltLo {
        pub const fn new() -> Self {
            Self(AnalogDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RadarAltLo {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadarAltLo {
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
    pub struct RadarAltTest(Selector<5406, 2048, 11>);
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

    /// Rotor RPM
    /// Output: gauge position
    pub struct RotorRpm(AnalogGauge<5220, 65535, 0>);
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

    /// Standby Compass Bank
    /// Output: gauge position
    pub struct SbyCompassBank(AnalogGauge<5438, 65535, 0>);
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
    pub struct SbyCompassHdg(AnalogGauge<5434, 65535, 0>);
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
    pub struct SbyCompassPitch(AnalogGauge<5436, 65535, 0>);
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

    /// Side Slip
    /// Output: gauge position
    pub struct Sideslip(AnalogGauge<5248, 65535, 0>);
    impl Sideslip {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for Sideslip {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for Sideslip {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// STBY Generator Load
    /// Output: gauge position
    pub struct StbyGenLoad(AnalogGauge<5234, 65535, 0>);
    impl StbyGenLoad {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for StbyGenLoad {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for StbyGenLoad {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Torque Pressure
    /// Output: gauge position
    pub struct TorquePress(AnalogGauge<5226, 65535, 0>);
    impl TorquePress {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TorquePress {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TorquePress {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Transmission Oil Pressure
    /// Output: gauge position
    pub struct TransOilPress(AnalogGauge<5208, 65535, 0>);
    impl TransOilPress {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TransOilPress {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TransOilPress {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Transmission Oil Temperature
    /// Output: gauge position
    pub struct TransOilTemp(AnalogGauge<5210, 65535, 0>);
    impl TransOilTemp {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TransOilTemp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TransOilTemp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Turn Indicator
    /// Output: gauge position
    pub struct TurnPtr(AnalogGauge<5246, 65535, 0>);
    impl TurnPtr {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TurnPtr {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TurnPtr {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Vertical Velocity Indicator (Operator)
    /// Output: gauge position
    pub struct VviO(AnalogGauge<5252, 65535, 0>);
    impl VviO {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for VviO {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for VviO {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Vertical Velocity Indicator (Pilot)
    /// Output: gauge position
    pub struct VviP(AnalogGauge<5250, 65535, 0>);
    impl VviP {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for VviP {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for VviP {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod frontdashadioperator {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Off Warning Flag
    /// Output: gauge position
    pub struct OadiOff(AnalogGauge<5264, 65535, 0>);
    impl OadiOff {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for OadiOff {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for OadiOff {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Pitch
    /// Output: gauge position
    pub struct OadiPitch(AnalogGauge<5262, 65535, 0>);
    impl OadiPitch {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for OadiPitch {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for OadiPitch {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Roll
    /// Output: gauge position
    pub struct OadiRoll(AnalogGauge<5260, 65535, 0>);
    impl OadiRoll {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for OadiRoll {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for OadiRoll {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod frontdashadipilot {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Off Warning Flag
    /// Output: gauge position
    pub struct PadiOff(AnalogGauge<5258, 65535, 0>);
    impl PadiOff {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for PadiOff {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for PadiOff {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Pitch
    /// Output: gauge position
    pub struct PadiPitch(AnalogGauge<5256, 65535, 0>);
    impl PadiPitch {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for PadiPitch {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for PadiPitch {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Roll
    /// Output: gauge position
    pub struct PadiRoll(AnalogGauge<5254, 65535, 0>);
    impl PadiRoll {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for PadiRoll {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for PadiRoll {
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
    pub enum MainFuelSwValue {
        Select1,
        Select2,
    }
    pub type MainFuelSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MainFuelSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for MainFuelSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Main Fuel Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MainFuelSw(Selector<5304, 512, 9>);
    impl MainFuelSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MainFuelSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MainFuelSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum TestFuelGaugeBtnValue {
        Select1,
        Select2,
    }
    pub type TestFuelGaugeBtnValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(TestFuelGaugeBtnValue) + 'a>;
    impl core::convert::TryFrom<u16> for TestFuelGaugeBtnValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Test Fuel Gauge Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct TestFuelGaugeBtn(Selector<5304, 1024, 10>);
    impl TestFuelGaugeBtn {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for TestFuelGaugeBtn {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for TestFuelGaugeBtn {
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

    /// AC GEN 1 Current
    /// Output: gauge position
    pub struct AcGen1Current(AnalogGauge<5134, 65535, 0>);
    impl AcGen1Current {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AcGen1Current {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AcGen1Current {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// AC GEN 2 Current
    /// Output: gauge position
    pub struct AcGen2Current(AnalogGauge<5136, 65535, 0>);
    impl AcGen2Current {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AcGen2Current {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AcGen2Current {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// AC Voltage
    /// Output: gauge position
    pub struct AcVoltage(AnalogGauge<5122, 65535, 0>);
    impl AcVoltage {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AcVoltage {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AcVoltage {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Attitude Pitch Shift
    /// Output: gauge position
    pub struct AdiPitchShift(AnalogGauge<5266, 65535, 0>);
    impl AdiPitchShift {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AdiPitchShift {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AdiPitchShift {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Anti-Ice Amperemeter
    /// Output: gauge position
    pub struct AntiIceAmp(AnalogGauge<5138, 65535, 0>);
    impl AntiIceAmp {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AntiIceAmp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AntiIceAmp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// DC Battery 1 Current
    /// Output: gauge position
    pub struct DcBat1Current(AnalogGauge<5124, 65535, 0>);
    impl DcBat1Current {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for DcBat1Current {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for DcBat1Current {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// DC Battery 2 Current
    /// Output: gauge position
    pub struct DcBat2Current(AnalogGauge<5126, 65535, 0>);
    impl DcBat2Current {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for DcBat2Current {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for DcBat2Current {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// DC Voltage
    /// Output: gauge position
    pub struct DcVoltage(AnalogGauge<5120, 65535, 0>);
    impl DcVoltage {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for DcVoltage {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for DcVoltage {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// DC VU 1 Current
    /// Output: gauge position
    pub struct DcVu1Current(AnalogGauge<5128, 65535, 0>);
    impl DcVu1Current {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for DcVu1Current {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for DcVu1Current {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// DC VU 2 Current
    /// Output: gauge position
    pub struct DcVu2Current(AnalogGauge<5130, 65535, 0>);
    impl DcVu2Current {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for DcVu2Current {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for DcVu2Current {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// DC VU 3 Current
    /// Output: gauge position
    pub struct DcVu3Current(AnalogGauge<5132, 65535, 0>);
    impl DcVu3Current {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for DcVu3Current {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for DcVu3Current {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// IAS Front
    /// Output: gauge position
    pub struct IasNose(AnalogGauge<5212, 65535, 0>);
    impl IasNose {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IasNose {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IasNose {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// IAS Roof
    /// Output: gauge position
    pub struct IasRoof(AnalogGauge<5214, 65535, 0>);
    impl IasRoof {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IasRoof {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IasRoof {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Marker Beacon
    /// Output: gauge position
    pub struct MarkerBeacon(AnalogGauge<5268, 65535, 0>);
    impl MarkerBeacon {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MarkerBeacon {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MarkerBeacon {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// RamTemp
    /// Output: gauge position
    pub struct RamTemp(AnalogGauge<5280, 65535, 0>);
    impl RamTemp {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RamTemp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RamTemp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// SIGHTS_FOR_CIVIL / controllers.Civil_Heli
    /// Output: gauge position
    pub struct SightsForCivil(AnalogGauge<5416, 65535, 0>);
    impl SightsForCivil {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for SightsForCivil {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for SightsForCivil {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod gmc {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// GMC Course Pointer 1
    /// Output: gauge position
    pub struct GmcCrs1(AnalogGauge<5162, 65535, 0>);
    impl GmcCrs1 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for GmcCrs1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for GmcCrs1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// GMC Course Pointer 2
    /// Output: gauge position
    pub struct GmcCrs2(AnalogGauge<5164, 65535, 0>);
    impl GmcCrs2 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for GmcCrs2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for GmcCrs2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// GMC Heading
    /// Output: gauge position
    pub struct GmcHdg(AnalogGauge<5168, 65535, 0>);
    impl GmcHdg {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for GmcHdg {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for GmcHdg {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// GMC Heading Marker
    /// Output: gauge position
    pub struct GmcHdgMarker(AnalogGauge<5166, 65535, 0>);
    impl GmcHdgMarker {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for GmcHdgMarker {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for GmcHdgMarker {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod gmcannunciator {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Output: gauge position
    pub struct GmcAnnunciator(AnalogGauge<5170, 65535, 0>);
    impl GmcAnnunciator {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for GmcAnnunciator {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for GmcAnnunciator {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod gmcpowerfail {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Output: gauge position
    pub struct GmcPwrfail(AnalogGauge<5172, 65535, 0>);
    impl GmcPwrfail {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for GmcPwrfail {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for GmcPwrfail {
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

    /// IFF Code: ZERO - B - A - (HOLD)
    /// Input: switch to previous or next state
    /// Output: selector position
    pub struct IffCode(Selector<5304, 6144, 11>);
    impl IffCode {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IffCode {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffCode {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffMasterValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
    }
    pub type IffMasterValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IffMasterValue) + 'a>;
    impl core::convert::TryFrom<u16> for IffMasterValue {
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

    /// IFF Master
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct IffMaster(Selector<5304, 57344, 13>);
    impl IffMaster {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IffMaster {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffMaster {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffMicIdentValue {
        Select1,
        Select2,
        Select3,
    }
    pub type IffMicIdentValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IffMicIdentValue) + 'a>;
    impl core::convert::TryFrom<u16> for IffMicIdentValue {
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

    /// Ident/Mic Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct IffMicIdent(Selector<5306, 12288, 12>);
    impl IffMicIdent {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IffMicIdent {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffMicIdent {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffMode1Wheel1Value {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
        Select6,
        Select7,
        Select8,
    }
    pub type IffMode1Wheel1ValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(IffMode1Wheel1Value) + 'a>;
    impl core::convert::TryFrom<u16> for IffMode1Wheel1Value {
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

    /// Mode-1 Wheel 1
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct IffMode1Wheel1(Selector<5308, 7, 0>);
    impl IffMode1Wheel1 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IffMode1Wheel1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffMode1Wheel1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffMode1Wheel2Value {
        Select1,
        Select2,
        Select3,
        Select4,
    }
    pub type IffMode1Wheel2ValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(IffMode1Wheel2Value) + 'a>;
    impl core::convert::TryFrom<u16> for IffMode1Wheel2Value {
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

    /// Mode-1 Wheel 2
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct IffMode1Wheel2(Selector<5308, 24, 3>);
    impl IffMode1Wheel2 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IffMode1Wheel2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffMode1Wheel2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffMode3aWheel1Value {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
        Select6,
        Select7,
        Select8,
    }
    pub type IffMode3aWheel1ValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(IffMode3aWheel1Value) + 'a>;
    impl core::convert::TryFrom<u16> for IffMode3aWheel1Value {
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

    /// Mode-3A Wheel 1
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct IffMode3aWheel1(Selector<5308, 224, 5>);
    impl IffMode3aWheel1 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IffMode3aWheel1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffMode3aWheel1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffMode3aWheel2Value {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
        Select6,
        Select7,
        Select8,
    }
    pub type IffMode3aWheel2ValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(IffMode3aWheel2Value) + 'a>;
    impl core::convert::TryFrom<u16> for IffMode3aWheel2Value {
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

    /// Mode-3A Wheel 2
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct IffMode3aWheel2(Selector<5308, 1792, 8>);
    impl IffMode3aWheel2 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IffMode3aWheel2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffMode3aWheel2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffMode3aWheel3Value {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
        Select6,
        Select7,
        Select8,
    }
    pub type IffMode3aWheel3ValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(IffMode3aWheel3Value) + 'a>;
    impl core::convert::TryFrom<u16> for IffMode3aWheel3Value {
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

    /// Mode-3A Wheel 3
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct IffMode3aWheel3(Selector<5308, 14336, 11>);
    impl IffMode3aWheel3 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IffMode3aWheel3 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffMode3aWheel3 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffMode3aWheel4Value {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
        Select6,
        Select7,
        Select8,
    }
    pub type IffMode3aWheel4ValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(IffMode3aWheel4Value) + 'a>;
    impl core::convert::TryFrom<u16> for IffMode3aWheel4Value {
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

    /// Mode-3A Wheel 4
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct IffMode3aWheel4(Selector<5310, 7, 0>);
    impl IffMode3aWheel4 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IffMode3aWheel4 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffMode3aWheel4 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffOnOutValue {
        Select1,
        Select2,
    }
    pub type IffOnOutValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IffOnOutValue) + 'a>;
    impl core::convert::TryFrom<u16> for IffOnOutValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// IFF On/Out
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IffOnOut(Selector<5306, 16384, 14>);
    impl IffOnOut {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IffOnOut {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffOnOut {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffOutAudioLightValue {
        Select1,
        Select2,
        Select3,
    }
    pub type IffOutAudioLightValueCallback<'a> =
        alloc::boxed::Box<dyn FnMut(IffOutAudioLightValue) + 'a>;
    impl core::convert::TryFrom<u16> for IffOutAudioLightValue {
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

    /// IFF Out: LIGHT - OFF - AUDIO
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct IffOutAudioLight(Selector<5306, 3, 0>);
    impl IffOutAudioLight {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IffOutAudioLight {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffOutAudioLight {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffRadtestValue {
        Select1,
        Select2,
        Select3,
    }
    pub type IffRadtestValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IffRadtestValue) + 'a>;
    impl core::convert::TryFrom<u16> for IffRadtestValue {
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

    /// RAD Test/Mon
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct IffRadtest(Selector<5306, 3072, 10>);
    impl IffRadtest {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IffRadtest {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffRadtest {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// IFF Reply Dim
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct IffReplyDim(LimitedDial<5312, 65535, 0>);
    impl IffReplyDim {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IffReplyDim {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffReplyDim {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// IFF Reply Indicator Lamp (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct IffReplyInd(Led<5142, 512, 9>);
    impl IffReplyInd {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for IffReplyInd {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffReplyInd {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffReplyTestValue {
        Select1,
        Select2,
    }
    pub type IffReplyTestValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IffReplyTestValue) + 'a>;
    impl core::convert::TryFrom<u16> for IffReplyTestValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// REPLY Push to Test
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IffReplyTest(Selector<5306, 32768, 15>);
    impl IffReplyTest {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IffReplyTest {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffReplyTest {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// TEST Reply Dim
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct IffTestDim(LimitedDial<5314, 65535, 0>);
    impl IffTestDim {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IffTestDim {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffTestDim {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// IFF Test Indicator Lamp (green)
    /// Output: 0 if light is off, 1 if light is on
    pub struct IffTestInd(Led<5142, 1024, 10>);
    impl IffTestInd {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for IffTestInd {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffTestInd {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffTestM1Value {
        Select1,
        Select2,
        Select3,
    }
    pub type IffTestM1ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IffTestM1Value) + 'a>;
    impl core::convert::TryFrom<u16> for IffTestM1Value {
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

    /// Test M-1
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct IffTestM1(Selector<5306, 12, 2>);
    impl IffTestM1 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IffTestM1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffTestM1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffTestM2Value {
        Select1,
        Select2,
        Select3,
    }
    pub type IffTestM2ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IffTestM2Value) + 'a>;
    impl core::convert::TryFrom<u16> for IffTestM2Value {
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

    /// Test M-2
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct IffTestM2(Selector<5306, 48, 4>);
    impl IffTestM2 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IffTestM2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffTestM2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffTestM3Value {
        Select1,
        Select2,
        Select3,
    }
    pub type IffTestM3ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IffTestM3Value) + 'a>;
    impl core::convert::TryFrom<u16> for IffTestM3Value {
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

    /// Test M-3
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct IffTestM3(Selector<5306, 192, 6>);
    impl IffTestM3 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IffTestM3 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffTestM3 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffTestM4Value {
        Select1,
        Select2,
        Select3,
    }
    pub type IffTestM4ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IffTestM4Value) + 'a>;
    impl core::convert::TryFrom<u16> for IffTestM4Value {
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

    /// Test M-4
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct IffTestM4(Selector<5306, 768, 8>);
    impl IffTestM4 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IffTestM4 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffTestM4 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IffTestTestValue {
        Select1,
        Select2,
    }
    pub type IffTestTestValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IffTestTestValue) + 'a>;
    impl core::convert::TryFrom<u16> for IffTestTestValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// TEST Push to Test
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IffTestTest(Selector<5308, 16384, 14>);
    impl IffTestTest {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IffTestTest {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IffTestTest {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod intercompanel {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IntIntSwValue {
        Select1,
        Select2,
    }
    pub type IntIntSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IntIntSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for IntIntSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// INT Receiver Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IntIntSw(Selector<5330, 8192, 13>);
    impl IntIntSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IntIntSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IntIntSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IntModeValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
        Select6,
    }
    pub type IntModeValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IntModeValue) + 'a>;
    impl core::convert::TryFrom<u16> for IntModeValue {
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

    /// Intercom Mode: PVT - INT - 1 - 2 - 3 - 4
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct IntMode(Selector<5340, 1792, 8>);
    impl IntMode {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IntMode {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IntMode {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IntNavSwValue {
        Select1,
        Select2,
    }
    pub type IntNavSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IntNavSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for IntNavSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// NAV Receiver Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IntNavSw(Selector<5330, 16384, 14>);
    impl IntNavSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IntNavSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IntNavSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IntRcvr1SwValue {
        Select1,
        Select2,
    }
    pub type IntRcvr1SwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IntRcvr1SwValue) + 'a>;
    impl core::convert::TryFrom<u16> for IntRcvr1SwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Receiver 1 Switch (VHF AM)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IntRcvr1Sw(Selector<5330, 512, 9>);
    impl IntRcvr1Sw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IntRcvr1Sw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IntRcvr1Sw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IntRcvr2SwValue {
        Select1,
        Select2,
    }
    pub type IntRcvr2SwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IntRcvr2SwValue) + 'a>;
    impl core::convert::TryFrom<u16> for IntRcvr2SwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Receiver 2 Switch (UHF)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IntRcvr2Sw(Selector<5330, 1024, 10>);
    impl IntRcvr2Sw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IntRcvr2Sw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IntRcvr2Sw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IntRcvr3SwValue {
        Select1,
        Select2,
    }
    pub type IntRcvr3SwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IntRcvr3SwValue) + 'a>;
    impl core::convert::TryFrom<u16> for IntRcvr3SwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Receiver 3 Switch (VHF FM)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IntRcvr3Sw(Selector<5330, 2048, 11>);
    impl IntRcvr3Sw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IntRcvr3Sw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IntRcvr3Sw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum IntRecvr4SwValue {
        Select1,
        Select2,
    }
    pub type IntRecvr4SwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(IntRecvr4SwValue) + 'a>;
    impl core::convert::TryFrom<u16> for IntRecvr4SwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Receiver 4 Switch (No Function)
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct IntRecvr4Sw(Selector<5330, 4096, 12>);
    impl IntRecvr4Sw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IntRecvr4Sw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IntRecvr4Sw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Intercom Volume
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct IntVol(LimitedDial<5342, 65535, 0>);
    impl IntVol {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for IntVol {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for IntVol {
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

    /// Console Light
    /// Output: gauge position
    pub struct ConsoleLight(AnalogGauge<5274, 65535, 0>);
    impl ConsoleLight {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for ConsoleLight {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for ConsoleLight {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Dome Light
    /// Output: gauge position
    pub struct DomeLight(AnalogGauge<5276, 65535, 0>);
    impl DomeLight {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for DomeLight {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for DomeLight {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Dome Light Green
    /// Output: gauge position
    pub struct DomeLightGreen(AnalogGauge<5278, 65535, 0>);
    impl DomeLightGreen {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for DomeLightGreen {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for DomeLightGreen {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod overheadpanel {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum AnticollLtsSwValue {
        Select1,
        Select2,
    }
    pub type AnticollLtsSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(AnticollLtsSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for AnticollLtsSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Anticollision Lights OFF / ON
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct AnticollLtsSw(Selector<5372, 16384, 14>);
    impl AnticollLtsSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for AnticollLtsSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for AnticollLtsSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum BleedAirSwValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
    }
    pub type BleedAirSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(BleedAirSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for BleedAirSwValue {
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

    /// Bleed Air Dial
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct BleedAirSw(Selector<5378, 448, 6>);
    impl BleedAirSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for BleedAirSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for BleedAirSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Overhead Console Panel Lights Brightness
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct BrtConsole(LimitedDial<5380, 65535, 0>);
    impl BrtConsole {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for BrtConsole {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for BrtConsole {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Copilot Instrument Lights Brightness
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct BrtCopilot(LimitedDial<5388, 65535, 0>);
    impl BrtCopilot {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for BrtCopilot {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for BrtCopilot {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Engine Instrument Lights Brightness
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct BrtEngine(LimitedDial<5386, 65535, 0>);
    impl BrtEngine {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for BrtEngine {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for BrtEngine {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Pedestal Lights Brightness
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct BrtPed(LimitedDial<5382, 65535, 0>);
    impl BrtPed {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for BrtPed {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for BrtPed {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Pilot Instrument Lights Brightness
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct BrtPilot(LimitedDial<5390, 65535, 0>);
    impl BrtPilot {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for BrtPilot {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for BrtPilot {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Secondary Instrument Lights Brightness
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct BrtSec(LimitedDial<5384, 65535, 0>);
    impl BrtSec {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for BrtSec {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for BrtSec {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum CargoRelSafeValue {
        Select1,
        Select2,
    }
    pub type CargoRelSafeValueCallback<'a> = alloc::boxed::Box<dyn FnMut(CargoRelSafeValue) + 'a>;
    impl core::convert::TryFrom<u16> for CargoRelSafeValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Cargo Safety OFF / ARM
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct CargoRelSafe(Selector<5432, 512, 9>);
    impl CargoRelSafe {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for CargoRelSafe {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for CargoRelSafe {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum DomeLightSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type DomeLightSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(DomeLightSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for DomeLightSwValue {
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

    /// Dome Light Switch WHITE / OFF / GREEN
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct DomeLightSw(Selector<5378, 48, 4>);
    impl DomeLightSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for DomeLightSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for DomeLightSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MainGenCoverValue {
        Select1,
        Select2,
    }
    pub type MainGenCoverValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MainGenCoverValue) + 'a>;
    impl core::convert::TryFrom<u16> for MainGenCoverValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Main Generator Switch Cover
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct MainGenCover(Selector<5304, 16, 4>);
    impl MainGenCover {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MainGenCover {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MainGenCover {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum MainGenSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type MainGenSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(MainGenSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for MainGenSwValue {
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

    /// Main Generator RESET/OFF/ON
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct MainGenSw(Selector<5304, 12, 2>);
    impl MainGenSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for MainGenSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for MainGenSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NavLtsSwValue {
        Select1,
        Select2,
        Select3,
    }
    pub type NavLtsSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(NavLtsSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for NavLtsSwValue {
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

    /// Navigation Lights STEADY - OFF - FLASH
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct NavLtsSw(Selector<5372, 6144, 11>);
    impl NavLtsSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for NavLtsSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for NavLtsSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum NvgPosLtsValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
        Select6,
    }
    pub type NvgPosLtsValueCallback<'a> = alloc::boxed::Box<dyn FnMut(NvgPosLtsValue) + 'a>;
    impl core::convert::TryFrom<u16> for NvgPosLtsValue {
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

    /// NVG Position Lights
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct NvgPosLts(Selector<5372, 1792, 8>);
    impl NvgPosLts {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for NvgPosLts {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for NvgPosLts {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum PitotHeatValue {
        Select1,
        Select2,
    }
    pub type PitotHeatValueCallback<'a> = alloc::boxed::Box<dyn FnMut(PitotHeatValue) + 'a>;
    impl core::convert::TryFrom<u16> for PitotHeatValue {
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
    pub struct PitotHeat(Selector<5304, 2, 1>);
    impl PitotHeat {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for PitotHeat {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for PitotHeat {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum PosLtsSwValue {
        Select1,
        Select2,
    }
    pub type PosLtsSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(PosLtsSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for PosLtsSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Position LIghts DIM / BRT
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct PosLtsSw(Selector<5372, 8192, 13>);
    impl PosLtsSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for PosLtsSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for PosLtsSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum RadarAltPwrValue {
        Select1,
        Select2,
    }
    pub type RadarAltPwrValueCallback<'a> = alloc::boxed::Box<dyn FnMut(RadarAltPwrValue) + 'a>;
    impl core::convert::TryFrom<u16> for RadarAltPwrValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Radar Altimeter Power
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct RadarAltPwr(Selector<5406, 1024, 10>);
    impl RadarAltPwr {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RadarAltPwr {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RadarAltPwr {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum WiperSelValue {
        Select1,
        Select2,
        Select3,
    }
    pub type WiperSelValueCallback<'a> = alloc::boxed::Box<dyn FnMut(WiperSelValue) + 'a>;
    impl core::convert::TryFrom<u16> for WiperSelValue {
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

    /// Wiper PILOT / BOTH / OPERATOR
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct WiperSel(Selector<5398, 6144, 11>);
    impl WiperSel {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for WiperSel {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for WiperSel {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Wiper Speed PARK - STOP - SLOW - MED - HIGH
    /// Input: switch to previous or next state
    /// Output: selector position
    pub struct WiperSpd(Selector<5398, 57344, 13>);
    impl WiperSpd {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for WiperSpd {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for WiperSpd {
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

    /// Radar Altimeter 1.Digit
    /// Output: gauge position
    pub struct RaltDigit1(AnalogGauge<5422, 65535, 0>);
    impl RaltDigit1 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RaltDigit1 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RaltDigit1 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Radar Altimeter 2.Digit
    /// Output: gauge position
    pub struct RaltDigit2(AnalogGauge<5424, 65535, 0>);
    impl RaltDigit2 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RaltDigit2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RaltDigit2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Radar Altimeter 3.Digit
    /// Output: gauge position
    pub struct RaltDigit3(AnalogGauge<5426, 65535, 0>);
    impl RaltDigit3 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RaltDigit3 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RaltDigit3 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Radar Altimeter 4.Digit
    /// Output: gauge position
    pub struct RaltDigit4(AnalogGauge<5428, 65535, 0>);
    impl RaltDigit4 {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RaltDigit4 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RaltDigit4 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Radar Altitude Display (Number)
    /// Output: Radar Altitude Display (Number)
    pub struct RaltDisplay(Metadata<5420, 65535, 0>);
    impl RaltDisplay {
        pub const fn new() -> Self {
            Self(Metadata::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RaltDisplay {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RaltDisplay {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Display (String)
    /// Output: Display (String)
    pub struct RaltDisplayStr(Display<5290, 4>);
    impl RaltDisplayStr {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for RaltDisplayStr {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RaltDisplayStr {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// HI Index
    /// Output: gauge position
    pub struct RaltHiIdx(AnalogGauge<5288, 65535, 0>);
    impl RaltHiIdx {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RaltHiIdx {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RaltHiIdx {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// HI Lamp (blue)
    /// Output: 0 if light is off, 1 if light is on
    pub struct RaltHiLamp(Led<5142, 4096, 12>);
    impl RaltHiLamp {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RaltHiLamp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RaltHiLamp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// LO Index
    /// Output: gauge position
    pub struct RaltLoIdx(AnalogGauge<5286, 65535, 0>);
    impl RaltLoIdx {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RaltLoIdx {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RaltLoIdx {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// LO Lamp (blue)
    /// Output: 0 if light is off, 1 if light is on
    pub struct RaltLoLamp(Led<5142, 2048, 11>);
    impl RaltLoLamp {
        pub const fn new() -> Self {
            Self(Led::new())
        }
        pub const fn is_on(&self) -> bool {
            self.0.is_on()
        }
    }
    impl Default for RaltLoLamp {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RaltLoLamp {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Needle
    /// Output: gauge position
    pub struct RaltNeedle(AnalogGauge<5282, 65535, 0>);
    impl RaltNeedle {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RaltNeedle {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RaltNeedle {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Off Warning Flag
    /// Output: gauge position
    pub struct RaltOff(AnalogGauge<5284, 65535, 0>);
    impl RaltOff {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for RaltOff {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for RaltOff {
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

    /// UHF Frequency
    /// Output: UHF Frequency
    pub struct UhfFreq(Display<5346, 6>);
    impl UhfFreq {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
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
    pub enum UhfFunctionValue {
        Select1,
        Select2,
        Select3,
        Select4,
    }
    pub type UhfFunctionValueCallback<'a> = alloc::boxed::Box<dyn FnMut(UhfFunctionValue) + 'a>;
    impl core::convert::TryFrom<u16> for UhfFunctionValue {
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

    /// Function Dial
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct UhfFunction(Selector<5352, 12, 2>);
    impl UhfFunction {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfFunction {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfFunction {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum UhfModeValue {
        Select1,
        Select2,
        Select3,
    }
    pub type UhfModeValueCallback<'a> = alloc::boxed::Box<dyn FnMut(UhfModeValue) + 'a>;
    impl core::convert::TryFrom<u16> for UhfModeValue {
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

    /// Frequency Mode Select
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct UhfMode(Selector<5352, 3, 0>);
    impl UhfMode {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfMode {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfMode {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Preset Channel Selector
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct UhfPresetInt(Selector<5340, 63488, 11>);
    impl UhfPresetInt {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for UhfPresetInt {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfPresetInt {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Preset Channel Selector
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: possible values: "1" "2" "3" "4" "5" "6" "7" "8" "9" "10" "11" "12" "13" "14" "15" "16" "17" "18" "19" "20"
    pub struct UhfPresetStr(Display<5344, 2>);
    impl UhfPresetStr {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for UhfPresetStr {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for UhfPresetStr {
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

    /// Squelch Switch
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct UhfSquelchSw(Selector<5352, 16, 4>);
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

    /// Volume Control
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct UhfVol(LimitedDial<5354, 65535, 0>);
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

pub mod vhfcommradio {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// VHF Frequency
    /// Output: VHF Frequency
    pub struct VhfcommFreq(Display<5334, 7>);
    impl VhfcommFreq {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for VhfcommFreq {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for VhfcommFreq {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// VHF KHz Selector
    /// Input: switch to previous or next state
    /// Output: selector position
    pub struct VhfcommKhz(Selector<5330, 480, 5>);
    impl VhfcommKhz {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for VhfcommKhz {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for VhfcommKhz {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// VHF MHz Selector
    /// Input: switch to previous or next state
    /// Output: selector position
    pub struct VhfcommMhz(Selector<5330, 30, 1>);
    impl VhfcommMhz {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for VhfcommMhz {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for VhfcommMhz {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum VhfcommPwrValue {
        Select1,
        Select2,
    }
    pub type VhfcommPwrValueCallback<'a> = alloc::boxed::Box<dyn FnMut(VhfcommPwrValue) + 'a>;
    impl core::convert::TryFrom<u16> for VhfcommPwrValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Power
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct VhfcommPwr(Selector<5330, 1, 0>);
    impl VhfcommPwr {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for VhfcommPwr {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for VhfcommPwr {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum VhfcommTestSwValue {
        Select1,
        Select2,
    }
    pub type VhfcommTestSwValueCallback<'a> = alloc::boxed::Box<dyn FnMut(VhfcommTestSwValue) + 'a>;
    impl core::convert::TryFrom<u16> for VhfcommTestSwValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Communication Test Button
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct VhfcommTestSw(Selector<5310, 32768, 15>);
    impl VhfcommTestSw {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for VhfcommTestSw {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for VhfcommTestSw {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Volume Control (step size less than 8192 may not work)
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct VhfcommVol(LimitedDial<5332, 65535, 0>);
    impl VhfcommVol {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for VhfcommVol {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for VhfcommVol {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod vhffmradio {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum VhffmFreq1IntValue {
        Select1,
        Select2,
        Select3,
        Select4,
        Select5,
    }
    pub type VhffmFreq1IntValueCallback<'a> = alloc::boxed::Box<dyn FnMut(VhffmFreq1IntValue) + 'a>;
    impl core::convert::TryFrom<u16> for VhffmFreq1IntValue {
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

    /// Frequency Tens of MHz
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct VhffmFreq1Int(Selector<5352, 224, 5>);
    impl VhffmFreq1Int {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for VhffmFreq1Int {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for VhffmFreq1Int {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Frequency Tens of MHz
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: possible values: "3" "4" "5" "6" "7"
    pub struct VhffmFreq1Str(Display<5356, 1>);
    impl VhffmFreq1Str {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for VhffmFreq1Str {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for VhffmFreq1Str {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum VhffmFreq2Value {
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
    pub type VhffmFreq2ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(VhffmFreq2Value) + 'a>;
    impl core::convert::TryFrom<u16> for VhffmFreq2Value {
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

    /// Frequency 1 MHz
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct VhffmFreq2(Selector<5352, 3840, 8>);
    impl VhffmFreq2 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for VhffmFreq2 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for VhffmFreq2 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum VhffmFreq3Value {
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
    pub type VhffmFreq3ValueCallback<'a> = alloc::boxed::Box<dyn FnMut(VhffmFreq3Value) + 'a>;
    impl core::convert::TryFrom<u16> for VhffmFreq3Value {
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

    /// Frequency Decimals MHz
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct VhffmFreq3(Selector<5352, 61440, 12>);
    impl VhffmFreq3 {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for VhffmFreq3 {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for VhffmFreq3 {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum VhffmFreq4IntValue {
        Select1,
        Select2,
    }
    pub type VhffmFreq4IntValueCallback<'a> = alloc::boxed::Box<dyn FnMut(VhffmFreq4IntValue) + 'a>;
    impl core::convert::TryFrom<u16> for VhffmFreq4IntValue {
        type Error = u16;

        fn try_from(value: u16) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(Self::Select1),
                1 => Ok(Self::Select2),
                value => Err(value),
            }
        }
    }

    /// Frequency Hundreds of MHz
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: selector position
    pub struct VhffmFreq4Int(Selector<5356, 256, 8>);
    impl VhffmFreq4Int {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for VhffmFreq4Int {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for VhffmFreq4Int {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Frequency Hundreds of MHz
    /// Input: switch to previous or next state
    /// Input: set position
    /// Input: Toggle switch state
    /// Output: possible values: "0" "5"
    pub struct VhffmFreq4Str(Display<5358, 1>);
    impl VhffmFreq4Str {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for VhffmFreq4Str {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for VhffmFreq4Str {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum VhffmModeValue {
        Select1,
        Select2,
        Select3,
        Select4,
    }
    pub type VhffmModeValueCallback<'a> = alloc::boxed::Box<dyn FnMut(VhffmModeValue) + 'a>;
    impl core::convert::TryFrom<u16> for VhffmModeValue {
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

    /// Mode Dial: OFF / T/R / RETRAN / HOME
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct VhffmMode(Selector<5356, 1536, 9>);
    impl VhffmMode {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for VhffmMode {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for VhffmMode {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum VhffmSquelchValue {
        Select1,
        Select2,
        Select3,
    }
    pub type VhffmSquelchValueCallback<'a> = alloc::boxed::Box<dyn FnMut(VhffmSquelchValue) + 'a>;
    impl core::convert::TryFrom<u16> for VhffmSquelchValue {
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

    /// Squelch Mode: DIS / CARR / TONE
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct VhffmSquelch(Selector<5356, 6144, 11>);
    impl VhffmSquelch {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for VhffmSquelch {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for VhffmSquelch {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Volume Control
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct VhffmVol(LimitedDial<5360, 65535, 0>);
    impl VhffmVol {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for VhffmVol {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for VhffmVol {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod vhfnavradio {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// VHF NAV Frequency
    /// Output: VHF NAV Frequency
    pub struct VhfnavFreq(Display<5362, 6>);
    impl VhfnavFreq {
        pub const fn new() -> Self {
            Self(Display::new())
        }
        pub fn value(&self) -> &[u8] {
            self.0.value()
        }
    }
    impl Default for VhfnavFreq {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for VhfnavFreq {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// VHF NAV KHz Selector
    /// Input: switch to previous or next state
    /// Output: selector position
    pub struct VhfnavKhz(Selector<5358, 61440, 12>);
    impl VhfnavKhz {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for VhfnavKhz {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for VhfnavKhz {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// VHF NAV MHz Selector
    /// Input: switch to previous or next state
    /// Output: selector position
    pub struct VhfnavMhz(Selector<5358, 3840, 8>);
    impl VhfnavMhz {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for VhfnavMhz {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for VhfnavMhz {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum VhfnavPwrValue {
        Select1,
        Select2,
        Select3,
    }
    pub type VhfnavPwrValueCallback<'a> = alloc::boxed::Box<dyn FnMut(VhfnavPwrValue) + 'a>;
    impl core::convert::TryFrom<u16> for VhfnavPwrValue {
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

    /// VHF NAV Off / On / Test
    /// Input: switch to previous or next state
    /// Input: set position
    /// Output: selector position
    pub struct VhfnavPwr(Selector<5356, 24576, 13>);
    impl VhfnavPwr {
        pub const fn new() -> Self {
            Self(Selector::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for VhfnavPwr {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for VhfnavPwr {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Volume Control (step size less than 8192 may not work)
    /// Input: set the position of the dial
    /// Input: turn the dial left or right
    /// Output: position of the potentiometer
    pub struct VhfnavVol(LimitedDial<5368, 65535, 0>);
    impl VhfnavVol {
        pub const fn new() -> Self {
            Self(LimitedDial::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for VhfnavVol {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for VhfnavVol {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub mod windshieldwipers {
    use super::*;

    pub type IntegerCallback<'a> = alloc::boxed::Box<dyn FnMut(u16) + 'a>;
    pub type LedCallback<'a> = alloc::boxed::Box<dyn FnMut(bool) + 'a>;
    pub type DisplayCallback<'a> = alloc::boxed::Box<dyn FnMut(&[u8]) + 'a>;

    /// Windshield Wiper (Operator)
    /// Output: gauge position
    pub struct WiperO(AnalogGauge<5272, 65535, 0>);
    impl WiperO {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for WiperO {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for WiperO {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }

    /// Windshield Wiper (Pilot)
    /// Output: gauge position
    pub struct WiperP(AnalogGauge<5270, 65535, 0>);
    impl WiperP {
        pub const fn new() -> Self {
            Self(AnalogGauge::new())
        }
        pub const fn value(&self) -> u16 {
            self.0.value()
        }
    }
    impl Default for WiperP {
        fn default() -> Self {
            Self::new()
        }
    }
    impl Control for WiperP {
        fn accept(&mut self, packet: Packet<'_>) {
            self.0.accept(packet);
        }
        fn commit(&mut self) -> bool {
            self.0.commit()
        }
    }
}

pub enum RegisteredControl<'a> {
    AcGen1Current {
        control: gauges::AcGen1Current,
        callback: IntegerCallback<'a>,
    },
    AcGen2Current {
        control: gauges::AcGen2Current,
        callback: IntegerCallback<'a>,
    },
    AcV {
        control: frontdash::AcV,
        callback: IntegerCallback<'a>,
    },
    AcVmSrc {
        control: electricalsystem::AcVmSrc,
        callback: electricalsystem::AcVmSrcValueCallback<'a>,
    },
    AcVoltage {
        control: gauges::AcVoltage,
        callback: IntegerCallback<'a>,
    },
    AdfBand {
        control: adf::AdfBand,
        callback: adf::AdfBandValueCallback<'a>,
    },
    AdfBfoSw {
        control: adf::AdfBfoSw,
        callback: adf::AdfBfoSwValueCallback<'a>,
    },
    AdfFreq {
        control: adf::AdfFreq,
        callback: IntegerCallback<'a>,
    },
    AdfGain {
        control: adf::AdfGain,
        callback: IntegerCallback<'a>,
    },
    AdfLoopLr {
        control: adf::AdfLoopLr,
        callback: adf::AdfLoopLrValueCallback<'a>,
    },
    AdfMode {
        control: adf::AdfMode,
        callback: adf::AdfModeValueCallback<'a>,
    },
    AdfSignal {
        control: adf::AdfSignal,
        callback: IntegerCallback<'a>,
    },
    AdfTune {
        control: adf::AdfTune,
        callback: IntegerCallback<'a>,
    },
    AdfVorSw {
        control: frontdash::AdfVorSw,
        callback: frontdash::AdfVorSwValueCallback<'a>,
    },
    AdiPitchShift {
        control: gauges::AdiPitchShift,
        callback: IntegerCallback<'a>,
    },
    AdiPitchTrim {
        control: frontdash::AdiPitchTrim,
        callback: IntegerCallback<'a>,
    },
    AdiPitchTrimCplt {
        control: frontdash::AdiPitchTrimCplt,
        callback: IntegerCallback<'a>,
    },
    AdiRollTrim {
        control: frontdash::AdiRollTrim,
        callback: IntegerCallback<'a>,
    },
    AltAdjCplt {
        control: frontdash::AltAdjCplt,
        callback: IntegerCallback<'a>,
    },
    AltAdjPlt {
        control: frontdash::AltAdjPlt,
        callback: IntegerCallback<'a>,
    },
    AntiIceAmp {
        control: gauges::AntiIceAmp,
        callback: IntegerCallback<'a>,
    },
    AnticollLtsSw {
        control: overheadpanel::AnticollLtsSw,
        callback: overheadpanel::AnticollLtsSwValueCallback<'a>,
    },
    ArmedInd {
        control: armamentpanel::ArmedInd,
        callback: LedCallback<'a>,
    },
    BatSw {
        control: electricalsystem::BatSw,
        callback: electricalsystem::BatSwValueCallback<'a>,
    },
    BeaconSensingSw {
        control: frontdash::BeaconSensingSw,
        callback: frontdash::BeaconSensingSwValueCallback<'a>,
    },
    BeaconVol {
        control: frontdash::BeaconVol,
        callback: IntegerCallback<'a>,
    },
    BleedAirSw {
        control: overheadpanel::BleedAirSw,
        callback: overheadpanel::BleedAirSwValueCallback<'a>,
    },
    BrtConsole {
        control: overheadpanel::BrtConsole,
        callback: IntegerCallback<'a>,
    },
    BrtCopilot {
        control: overheadpanel::BrtCopilot,
        callback: IntegerCallback<'a>,
    },
    BrtEngine {
        control: overheadpanel::BrtEngine,
        callback: IntegerCallback<'a>,
    },
    BrtPed {
        control: overheadpanel::BrtPed,
        callback: IntegerCallback<'a>,
    },
    BrtPilot {
        control: overheadpanel::BrtPilot,
        callback: IntegerCallback<'a>,
    },
    BrtSec {
        control: overheadpanel::BrtSec,
        callback: IntegerCallback<'a>,
    },
    CargoRelCp {
        control: cyclic::CargoRelCp,
        callback: cyclic::CargoRelCpValueCallback<'a>,
    },
    CargoRelP {
        control: cyclic::CargoRelP,
        callback: cyclic::CargoRelPValueCallback<'a>,
    },
    CargoRelSafe {
        control: overheadpanel::CargoRelSafe,
        callback: overheadpanel::CargoRelSafeValueCallback<'a>,
    },
    Cb28v {
        control: circuitbreakers::Cb28v,
        callback: circuitbreakers::Cb28vValueCallback<'a>,
    },
    CbAirValve {
        control: circuitbreakers::CbAirValve,
        callback: circuitbreakers::CbAirValveValueCallback<'a>,
    },
    CbAmRadio {
        control: circuitbreakers::CbAmRadio,
        callback: circuitbreakers::CbAmRadioValueCallback<'a>,
    },
    CbAntiIce {
        control: circuitbreakers::CbAntiIce,
        callback: circuitbreakers::CbAntiIceValueCallback<'a>,
    },
    CbAnticoll {
        control: circuitbreakers::CbAnticoll,
        callback: circuitbreakers::CbAnticollValueCallback<'a>,
    },
    CbArc102 {
        control: circuitbreakers::CbArc102,
        callback: circuitbreakers::CbArc102ValueCallback<'a>,
    },
    CbCable {
        control: circuitbreakers::CbCable,
        callback: circuitbreakers::CbCableValueCallback<'a>,
    },
    CbCautionLts {
        control: circuitbreakers::CbCautionLts,
        callback: circuitbreakers::CbCautionLtsValueCallback<'a>,
    },
    CbCockpitLts {
        control: circuitbreakers::CbCockpitLts,
        callback: circuitbreakers::CbCockpitLtsValueCallback<'a>,
    },
    CbConsoleLts {
        control: circuitbreakers::CbConsoleLts,
        callback: circuitbreakers::CbConsoleLtsValueCallback<'a>,
    },
    CbCpltAttd1 {
        control: circuitbreakers::CbCpltAttd1,
        callback: circuitbreakers::CbCpltAttd1ValueCallback<'a>,
    },
    CbCpltAttd2 {
        control: circuitbreakers::CbCpltAttd2,
        callback: circuitbreakers::CbCpltAttd2ValueCallback<'a>,
    },
    CbCrsind {
        control: circuitbreakers::CbCrsind,
        callback: circuitbreakers::CbCrsindValueCallback<'a>,
    },
    CbDomeLts {
        control: circuitbreakers::CbDomeLts,
        callback: circuitbreakers::CbDomeLtsValueCallback<'a>,
    },
    CbEng {
        control: circuitbreakers::CbEng,
        callback: circuitbreakers::CbEngValueCallback<'a>,
    },
    CbFailRelay {
        control: circuitbreakers::CbFailRelay,
        callback: circuitbreakers::CbFailRelayValueCallback<'a>,
    },
    CbFire {
        control: circuitbreakers::CbFire,
        callback: circuitbreakers::CbFireValueCallback<'a>,
    },
    CbFm2Radio {
        control: circuitbreakers::CbFm2Radio,
        callback: circuitbreakers::CbFm2RadioValueCallback<'a>,
    },
    CbFmRadio {
        control: circuitbreakers::CbFmRadio,
        callback: circuitbreakers::CbFmRadioValueCallback<'a>,
    },
    CbFuel {
        control: circuitbreakers::CbFuel,
        callback: circuitbreakers::CbFuelValueCallback<'a>,
    },
    CbFuelQty {
        control: circuitbreakers::CbFuelQty,
        callback: circuitbreakers::CbFuelQtyValueCallback<'a>,
    },
    CbFuelTrans {
        control: circuitbreakers::CbFuelTrans,
        callback: circuitbreakers::CbFuelTransValueCallback<'a>,
    },
    CbFuelValves {
        control: circuitbreakers::CbFuelValves,
        callback: circuitbreakers::CbFuelValvesValueCallback<'a>,
    },
    CbFusLts {
        control: circuitbreakers::CbFusLts,
        callback: circuitbreakers::CbFusLtsValueCallback<'a>,
    },
    CbGenBus {
        control: circuitbreakers::CbGenBus,
        callback: circuitbreakers::CbGenBusValueCallback<'a>,
    },
    CbGov {
        control: circuitbreakers::CbGov,
        callback: circuitbreakers::CbGovValueCallback<'a>,
    },
    CbGyro {
        control: circuitbreakers::CbGyro,
        callback: circuitbreakers::CbGyroValueCallback<'a>,
    },
    CbHeatedBlanket1 {
        control: circuitbreakers::CbHeatedBlanket1,
        callback: circuitbreakers::CbHeatedBlanket1ValueCallback<'a>,
    },
    CbHeatedBlanket2 {
        control: circuitbreakers::CbHeatedBlanket2,
        callback: circuitbreakers::CbHeatedBlanket2ValueCallback<'a>,
    },
    CbHfAnt {
        control: circuitbreakers::CbHfAnt,
        callback: circuitbreakers::CbHfAntValueCallback<'a>,
    },
    CbHfArc102 {
        control: circuitbreakers::CbHfArc102,
        callback: circuitbreakers::CbHfArc102ValueCallback<'a>,
    },
    CbHook {
        control: circuitbreakers::CbHook,
        callback: circuitbreakers::CbHookValueCallback<'a>,
    },
    CbHydctl {
        control: circuitbreakers::CbHydctl,
        callback: circuitbreakers::CbHydctlValueCallback<'a>,
    },
    CbIdleStop {
        control: circuitbreakers::CbIdleStop,
        callback: circuitbreakers::CbIdleStopValueCallback<'a>,
    },
    CbIffApx1 {
        control: circuitbreakers::CbIffApx1,
        callback: circuitbreakers::CbIffApx1ValueCallback<'a>,
    },
    CbIffApx2 {
        control: circuitbreakers::CbIffApx2,
        callback: circuitbreakers::CbIffApx2ValueCallback<'a>,
    },
    CbIgn {
        control: circuitbreakers::CbIgn,
        callback: circuitbreakers::CbIgnValueCallback<'a>,
    },
    CbInstLts {
        control: circuitbreakers::CbInstLts,
        callback: circuitbreakers::CbInstLtsValueCallback<'a>,
    },
    CbInstSecLts {
        control: circuitbreakers::CbInstSecLts,
        callback: circuitbreakers::CbInstSecLtsValueCallback<'a>,
    },
    CbIntCplt {
        control: circuitbreakers::CbIntCplt,
        callback: circuitbreakers::CbIntCpltValueCallback<'a>,
    },
    CbIntPlt {
        control: circuitbreakers::CbIntPlt,
        callback: circuitbreakers::CbIntPltValueCallback<'a>,
    },
    CbInvtr {
        control: circuitbreakers::CbInvtr,
        callback: circuitbreakers::CbInvtrValueCallback<'a>,
    },
    CbJtsn {
        control: circuitbreakers::CbJtsn,
        callback: circuitbreakers::CbJtsnValueCallback<'a>,
    },
    CbKy20 {
        control: circuitbreakers::CbKy20,
        callback: circuitbreakers::CbKy20ValueCallback<'a>,
    },
    CbLdglts {
        control: circuitbreakers::CbLdglts,
        callback: circuitbreakers::CbLdgltsValueCallback<'a>,
    },
    CbLfNav {
        control: circuitbreakers::CbLfNav,
        callback: circuitbreakers::CbLfNavValueCallback<'a>,
    },
    CbLhFuel {
        control: circuitbreakers::CbLhFuel,
        callback: circuitbreakers::CbLhFuelValueCallback<'a>,
    },
    CbMainInvtr {
        control: circuitbreakers::CbMainInvtr,
        callback: circuitbreakers::CbMainInvtrValueCallback<'a>,
    },
    CbMarkerBeacon {
        control: circuitbreakers::CbMarkerBeacon,
        callback: circuitbreakers::CbMarkerBeaconValueCallback<'a>,
    },
    CbNavLts {
        control: circuitbreakers::CbNavLts,
        callback: circuitbreakers::CbNavLtsValueCallback<'a>,
    },
    CbOutletValve {
        control: circuitbreakers::CbOutletValve,
        callback: circuitbreakers::CbOutletValveValueCallback<'a>,
    },
    CbPitotTube {
        control: circuitbreakers::CbPitotTube,
        callback: circuitbreakers::CbPitotTubeValueCallback<'a>,
    },
    CbPltAttd1 {
        control: circuitbreakers::CbPltAttd1,
        callback: circuitbreakers::CbPltAttd1ValueCallback<'a>,
    },
    CbPltAttd2 {
        control: circuitbreakers::CbPltAttd2,
        callback: circuitbreakers::CbPltAttd2ValueCallback<'a>,
    },
    CbPorxWarn {
        control: circuitbreakers::CbPorxWarn,
        callback: circuitbreakers::CbPorxWarnValueCallback<'a>,
    },
    CbRescueCont {
        control: circuitbreakers::CbRescueCont,
        callback: circuitbreakers::CbRescueContValueCallback<'a>,
    },
    CbRescuePwr {
        control: circuitbreakers::CbRescuePwr,
        callback: circuitbreakers::CbRescuePwrValueCallback<'a>,
    },
    CbRhFuel {
        control: circuitbreakers::CbRhFuel,
        callback: circuitbreakers::CbRhFuelValueCallback<'a>,
    },
    CbRpmWarn {
        control: circuitbreakers::CbRpmWarn,
        callback: circuitbreakers::CbRpmWarnValueCallback<'a>,
    },
    CbSlCtl {
        control: circuitbreakers::CbSlCtl,
        callback: circuitbreakers::CbSlCtlValueCallback<'a>,
    },
    CbSlPwr {
        control: circuitbreakers::CbSlPwr,
        callback: circuitbreakers::CbSlPwrValueCallback<'a>,
    },
    CbSpareInvtr {
        control: circuitbreakers::CbSpareInvtr,
        callback: circuitbreakers::CbSpareInvtrValueCallback<'a>,
    },
    CbStarter {
        control: circuitbreakers::CbStarter,
        callback: circuitbreakers::CbStarterValueCallback<'a>,
    },
    CbStbyGen {
        control: circuitbreakers::CbStbyGen,
        callback: circuitbreakers::CbStbyGenValueCallback<'a>,
    },
    CbTemp {
        control: circuitbreakers::CbTemp,
        callback: circuitbreakers::CbTempValueCallback<'a>,
    },
    CbTorq {
        control: circuitbreakers::CbTorq,
        callback: circuitbreakers::CbTorqValueCallback<'a>,
    },
    CbTrim {
        control: circuitbreakers::CbTrim,
        callback: circuitbreakers::CbTrimValueCallback<'a>,
    },
    CbTurnSlip {
        control: circuitbreakers::CbTurnSlip,
        callback: circuitbreakers::CbTurnSlipValueCallback<'a>,
    },
    CbUhfRadio {
        control: circuitbreakers::CbUhfRadio,
        callback: circuitbreakers::CbUhfRadioValueCallback<'a>,
    },
    CbVhfNav {
        control: circuitbreakers::CbVhfNav,
        callback: circuitbreakers::CbVhfNavValueCallback<'a>,
    },
    CbVm {
        control: circuitbreakers::CbVm,
        callback: circuitbreakers::CbVmValueCallback<'a>,
    },
    CbWiperCplt {
        control: circuitbreakers::CbWiperCplt,
        callback: circuitbreakers::CbWiperCpltValueCallback<'a>,
    },
    CbWiperPlt {
        control: circuitbreakers::CbWiperPlt,
        callback: circuitbreakers::CbWiperPltValueCallback<'a>,
    },
    CbXmsn {
        control: circuitbreakers::CbXmsn,
        callback: circuitbreakers::CbXmsnValueCallback<'a>,
    },
    ChipDetSw {
        control: centerconsole::ChipDetSw,
        callback: centerconsole::ChipDetSwValueCallback<'a>,
    },
    CiCourseCard {
        control: courseindicator::CiCourseCard,
        callback: IntegerCallback<'a>,
    },
    CiFrom {
        control: courseindicator::CiFrom,
        callback: IntegerCallback<'a>,
    },
    CiHbar {
        control: courseindicator::CiHbar,
        callback: IntegerCallback<'a>,
    },
    CiHoff {
        control: courseindicator::CiHoff,
        callback: IntegerCallback<'a>,
    },
    CiTo {
        control: courseindicator::CiTo,
        callback: IntegerCallback<'a>,
    },
    CiVbar {
        control: courseindicator::CiVbar,
        callback: IntegerCallback<'a>,
    },
    CiVoff {
        control: courseindicator::CiVoff,
        callback: IntegerCallback<'a>,
    },
    ClA0 {
        control: cautionlightspanel::ClA0,
        callback: LedCallback<'a>,
    },
    ClA1 {
        control: cautionlightspanel::ClA1,
        callback: LedCallback<'a>,
    },
    ClB0 {
        control: cautionlightspanel::ClB0,
        callback: LedCallback<'a>,
    },
    ClB1 {
        control: cautionlightspanel::ClB1,
        callback: LedCallback<'a>,
    },
    ClC0 {
        control: cautionlightspanel::ClC0,
        callback: LedCallback<'a>,
    },
    ClC1 {
        control: cautionlightspanel::ClC1,
        callback: LedCallback<'a>,
    },
    ClD0 {
        control: cautionlightspanel::ClD0,
        callback: LedCallback<'a>,
    },
    ClD1 {
        control: cautionlightspanel::ClD1,
        callback: LedCallback<'a>,
    },
    ClE0 {
        control: cautionlightspanel::ClE0,
        callback: LedCallback<'a>,
    },
    ClE1 {
        control: cautionlightspanel::ClE1,
        callback: LedCallback<'a>,
    },
    ClF0 {
        control: cautionlightspanel::ClF0,
        callback: LedCallback<'a>,
    },
    ClF1 {
        control: cautionlightspanel::ClF1,
        callback: LedCallback<'a>,
    },
    ClG0 {
        control: cautionlightspanel::ClG0,
        callback: LedCallback<'a>,
    },
    ClG1 {
        control: cautionlightspanel::ClG1,
        callback: LedCallback<'a>,
    },
    ClH0 {
        control: cautionlightspanel::ClH0,
        callback: LedCallback<'a>,
    },
    ClH1 {
        control: cautionlightspanel::ClH1,
        callback: LedCallback<'a>,
    },
    ClI0 {
        control: cautionlightspanel::ClI0,
        callback: LedCallback<'a>,
    },
    ClI1 {
        control: cautionlightspanel::ClI1,
        callback: LedCallback<'a>,
    },
    ClJ0 {
        control: cautionlightspanel::ClJ0,
        callback: LedCallback<'a>,
    },
    ClJ1 {
        control: cautionlightspanel::ClJ1,
        callback: LedCallback<'a>,
    },
    ClockAdjust {
        control: clock::ClockAdjust,
        callback: IntegerCallback<'a>,
    },
    ClockAdjustPull {
        control: clock::ClockAdjustPull,
        callback: IntegerCallback<'a>,
    },
    ClockH {
        control: frontdash::ClockH,
        callback: IntegerCallback<'a>,
    },
    ClockM {
        control: frontdash::ClockM,
        callback: IntegerCallback<'a>,
    },
    ClockS {
        control: frontdash::ClockS,
        callback: IntegerCallback<'a>,
    },
    ClpBrightDimSw {
        control: cautionlightspanel::ClpBrightDimSw,
        callback: cautionlightspanel::ClpBrightDimSwValueCallback<'a>,
    },
    ClpResetTestSw {
        control: cautionlightspanel::ClpResetTestSw,
        callback: cautionlightspanel::ClpResetTestSwValueCallback<'a>,
    },
    CmArmSw {
        control: countermeasures::CmArmSw,
        callback: countermeasures::CmArmSwValueCallback<'a>,
    },
    CmArmedTest {
        control: countermeasures::CmArmedTest,
        callback: countermeasures::CmArmedTestValueCallback<'a>,
    },
    CmChaffcntDisplay {
        control: countermeasures::CmChaffcntDisplay,
        callback: IntegerCallback<'a>,
    },
    CmChaffcntDisplayStr {
        control: countermeasures::CmChaffcntDisplayStr,
        callback: DisplayCallback<'a>,
    },
    CmChaffcntReset {
        control: countermeasures::CmChaffcntReset,
        callback: countermeasures::CmChaffcntResetValueCallback<'a>,
    },
    CmFlareBtn {
        control: countermeasures::CmFlareBtn,
        callback: countermeasures::CmFlareBtnValueCallback<'a>,
    },
    CmFlarecntDisplay {
        control: countermeasures::CmFlarecntDisplay,
        callback: IntegerCallback<'a>,
    },
    CmFlarecntDisplayStr {
        control: countermeasures::CmFlarecntDisplayStr,
        callback: DisplayCallback<'a>,
    },
    CmFlarecntReset {
        control: countermeasures::CmFlarecntReset,
        callback: countermeasures::CmFlarecntResetValueCallback<'a>,
    },
    CmManPgrmSw {
        control: countermeasures::CmManPgrmSw,
        callback: countermeasures::CmManPgrmSwValueCallback<'a>,
    },
    CmRippleCover {
        control: countermeasures::CmRippleCover,
        callback: countermeasures::CmRippleCoverValueCallback<'a>,
    },
    CmRippleSw {
        control: countermeasures::CmRippleSw,
        callback: countermeasures::CmRippleSwValueCallback<'a>,
    },
    ConsoleLight {
        control: lights::ConsoleLight,
        callback: IntegerCallback<'a>,
    },
    CpltAdiCage {
        control: frontdash::CpltAdiCage,
        callback: frontdash::CpltAdiCageValueCallback<'a>,
    },
    CrsKnob {
        control: frontdash::CrsKnob,
        callback: IntegerCallback<'a>,
    },
    DcBat1Current {
        control: gauges::DcBat1Current,
        callback: IntegerCallback<'a>,
    },
    DcBat2Current {
        control: gauges::DcBat2Current,
        callback: IntegerCallback<'a>,
    },
    DcV {
        control: frontdash::DcV,
        callback: IntegerCallback<'a>,
    },
    DcVmSrc {
        control: electricalsystem::DcVmSrc,
        callback: electricalsystem::DcVmSrcValueCallback<'a>,
    },
    DcVoltage {
        control: gauges::DcVoltage,
        callback: IntegerCallback<'a>,
    },
    DcVu1Current {
        control: gauges::DcVu1Current,
        callback: IntegerCallback<'a>,
    },
    DcVu2Current {
        control: gauges::DcVu2Current,
        callback: IntegerCallback<'a>,
    },
    DcVu3Current {
        control: gauges::DcVu3Current,
        callback: IntegerCallback<'a>,
    },
    DomeLight {
        control: lights::DomeLight,
        callback: IntegerCallback<'a>,
    },
    DomeLightGreen {
        control: lights::DomeLightGreen,
        callback: IntegerCallback<'a>,
    },
    DomeLightSw {
        control: overheadpanel::DomeLightSw,
        callback: overheadpanel::DomeLightSwValueCallback<'a>,
    },
    DoorLPtr {
        control: doors::DoorLPtr,
        callback: doors::DoorLPtrValueCallback<'a>,
    },
    DoorRPtr {
        control: doors::DoorRPtr,
        callback: doors::DoorRPtrValueCallback<'a>,
    },
    EmerGovSw {
        control: enginepanel::EmerGovSw,
        callback: enginepanel::EmerGovSwValueCallback<'a>,
    },
    EngDeice {
        control: enginepanel::EngDeice,
        callback: enginepanel::EngDeiceValueCallback<'a>,
    },
    EngOilPress {
        control: frontdash::EngOilPress,
        callback: IntegerCallback<'a>,
    },
    EngOilTemp {
        control: frontdash::EngOilTemp,
        callback: IntegerCallback<'a>,
    },
    EngRpm {
        control: frontdash::EngRpm,
        callback: IntegerCallback<'a>,
    },
    ExhaustTemp {
        control: frontdash::ExhaustTemp,
        callback: IntegerCallback<'a>,
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
    ExtWowSkid {
        control: externalaircraftmodel::ExtWowSkid,
        callback: IntegerCallback<'a>,
    },
    FireInd {
        control: frontdash::FireInd,
        callback: LedCallback<'a>,
    },
    FireTestBtn {
        control: frontdash::FireTestBtn,
        callback: frontdash::FireTestBtnValueCallback<'a>,
    },
    ForceTrimSw {
        control: centerconsole::ForceTrimSw,
        callback: centerconsole::ForceTrimSwValueCallback<'a>,
    },
    FuelPress {
        control: frontdash::FuelPress,
        callback: IntegerCallback<'a>,
    },
    FuelQty {
        control: frontdash::FuelQty,
        callback: IntegerCallback<'a>,
    },
    GasRpm {
        control: frontdash::GasRpm,
        callback: IntegerCallback<'a>,
    },
    GasRpmT {
        control: frontdash::GasRpmT,
        callback: IntegerCallback<'a>,
    },
    GmcAnnunciator {
        control: gmcannunciator::GmcAnnunciator,
        callback: IntegerCallback<'a>,
    },
    GmcCrs1 {
        control: gmc::GmcCrs1,
        callback: IntegerCallback<'a>,
    },
    GmcCrs2 {
        control: gmc::GmcCrs2,
        callback: IntegerCallback<'a>,
    },
    GmcHdg {
        control: gmc::GmcHdg,
        callback: IntegerCallback<'a>,
    },
    GmcHdgMarker {
        control: gmc::GmcHdgMarker,
        callback: IntegerCallback<'a>,
    },
    GmcPwrfail {
        control: gmcpowerfail::GmcPwrfail,
        callback: IntegerCallback<'a>,
    },
    GovRpmSw {
        control: collective::GovRpmSw,
        callback: collective::GovRpmSwValueCallback<'a>,
    },
    GovRpmSw2 {
        control: collective::GovRpmSw2,
        callback: collective::GovRpmSw2ValueCallback<'a>,
    },
    GunSel {
        control: armamentpanel::GunSel,
        callback: armamentpanel::GunSelValueCallback<'a>,
    },
    GyroModeSw {
        control: frontdash::GyroModeSw,
        callback: frontdash::GyroModeSwValueCallback<'a>,
    },
    HdgSet {
        control: frontdash::HdgSet,
        callback: IntegerCallback<'a>,
    },
    HdgSync {
        control: frontdash::HdgSync,
        callback: IntegerCallback<'a>,
    },
    HydContSw {
        control: centerconsole::HydContSw,
        callback: centerconsole::HydContSwValueCallback<'a>,
    },
    IasNose {
        control: gauges::IasNose,
        callback: IntegerCallback<'a>,
    },
    IasRoof {
        control: gauges::IasRoof,
        callback: IntegerCallback<'a>,
    },
    IffCode {
        control: iff::IffCode,
        callback: IntegerCallback<'a>,
    },
    IffMaster {
        control: iff::IffMaster,
        callback: iff::IffMasterValueCallback<'a>,
    },
    IffMicIdent {
        control: iff::IffMicIdent,
        callback: iff::IffMicIdentValueCallback<'a>,
    },
    IffMode1Wheel1 {
        control: iff::IffMode1Wheel1,
        callback: iff::IffMode1Wheel1ValueCallback<'a>,
    },
    IffMode1Wheel2 {
        control: iff::IffMode1Wheel2,
        callback: iff::IffMode1Wheel2ValueCallback<'a>,
    },
    IffMode3aWheel1 {
        control: iff::IffMode3aWheel1,
        callback: iff::IffMode3aWheel1ValueCallback<'a>,
    },
    IffMode3aWheel2 {
        control: iff::IffMode3aWheel2,
        callback: iff::IffMode3aWheel2ValueCallback<'a>,
    },
    IffMode3aWheel3 {
        control: iff::IffMode3aWheel3,
        callback: iff::IffMode3aWheel3ValueCallback<'a>,
    },
    IffMode3aWheel4 {
        control: iff::IffMode3aWheel4,
        callback: iff::IffMode3aWheel4ValueCallback<'a>,
    },
    IffOnOut {
        control: iff::IffOnOut,
        callback: iff::IffOnOutValueCallback<'a>,
    },
    IffOutAudioLight {
        control: iff::IffOutAudioLight,
        callback: iff::IffOutAudioLightValueCallback<'a>,
    },
    IffRadtest {
        control: iff::IffRadtest,
        callback: iff::IffRadtestValueCallback<'a>,
    },
    IffReplyDim {
        control: iff::IffReplyDim,
        callback: IntegerCallback<'a>,
    },
    IffReplyInd {
        control: iff::IffReplyInd,
        callback: LedCallback<'a>,
    },
    IffReplyTest {
        control: iff::IffReplyTest,
        callback: iff::IffReplyTestValueCallback<'a>,
    },
    IffTestDim {
        control: iff::IffTestDim,
        callback: IntegerCallback<'a>,
    },
    IffTestInd {
        control: iff::IffTestInd,
        callback: LedCallback<'a>,
    },
    IffTestM1 {
        control: iff::IffTestM1,
        callback: iff::IffTestM1ValueCallback<'a>,
    },
    IffTestM2 {
        control: iff::IffTestM2,
        callback: iff::IffTestM2ValueCallback<'a>,
    },
    IffTestM3 {
        control: iff::IffTestM3,
        callback: iff::IffTestM3ValueCallback<'a>,
    },
    IffTestM4 {
        control: iff::IffTestM4,
        callback: iff::IffTestM4ValueCallback<'a>,
    },
    IffTestTest {
        control: iff::IffTestTest,
        callback: iff::IffTestTestValueCallback<'a>,
    },
    IntIntSw {
        control: intercompanel::IntIntSw,
        callback: intercompanel::IntIntSwValueCallback<'a>,
    },
    IntMode {
        control: intercompanel::IntMode,
        callback: intercompanel::IntModeValueCallback<'a>,
    },
    IntNavSw {
        control: intercompanel::IntNavSw,
        callback: intercompanel::IntNavSwValueCallback<'a>,
    },
    IntRcvr1Sw {
        control: intercompanel::IntRcvr1Sw,
        callback: intercompanel::IntRcvr1SwValueCallback<'a>,
    },
    IntRcvr2Sw {
        control: intercompanel::IntRcvr2Sw,
        callback: intercompanel::IntRcvr2SwValueCallback<'a>,
    },
    IntRcvr3Sw {
        control: intercompanel::IntRcvr3Sw,
        callback: intercompanel::IntRcvr3SwValueCallback<'a>,
    },
    IntRecvr4Sw {
        control: intercompanel::IntRecvr4Sw,
        callback: intercompanel::IntRecvr4SwValueCallback<'a>,
    },
    IntVol {
        control: intercompanel::IntVol,
        callback: IntegerCallback<'a>,
    },
    InverterSw {
        control: electricalsystem::InverterSw,
        callback: electricalsystem::InverterSwValueCallback<'a>,
    },
    JtsnBtn {
        control: armamentpanel::JtsnBtn,
        callback: armamentpanel::JtsnBtnValueCallback<'a>,
    },
    JtsnCover {
        control: armamentpanel::JtsnCover,
        callback: armamentpanel::JtsnCoverValueCallback<'a>,
    },
    LdgLightSw {
        control: collective::LdgLightSw,
        callback: collective::LdgLightSwValueCallback<'a>,
    },
    LdgLightSw2 {
        control: collective::LdgLightSw2,
        callback: collective::LdgLightSw2ValueCallback<'a>,
    },
    LdgLtCtrl {
        control: collective::LdgLtCtrl,
        callback: collective::LdgLtCtrlValueCallback<'a>,
    },
    LdgLtCtrl2 {
        control: collective::LdgLtCtrl2,
        callback: collective::LdgLtCtrl2ValueCallback<'a>,
    },
    LowRpmAudio {
        control: enginepanel::LowRpmAudio,
        callback: enginepanel::LowRpmAudioValueCallback<'a>,
    },
    LowRpmInd {
        control: frontdash::LowRpmInd,
        callback: LedCallback<'a>,
    },
    MainFuelSw {
        control: fuelsystem::MainFuelSw,
        callback: fuelsystem::MainFuelSwValueCallback<'a>,
    },
    MainGenCover {
        control: overheadpanel::MainGenCover,
        callback: overheadpanel::MainGenCoverValueCallback<'a>,
    },
    MainGenLoad {
        control: frontdash::MainGenLoad,
        callback: IntegerCallback<'a>,
    },
    MainGenSw {
        control: overheadpanel::MainGenSw,
        callback: overheadpanel::MainGenSwValueCallback<'a>,
    },
    MarkerBeacon {
        control: gauges::MarkerBeacon,
        callback: IntegerCallback<'a>,
    },
    MasterArmSw {
        control: armamentpanel::MasterArmSw,
        callback: armamentpanel::MasterArmSwValueCallback<'a>,
    },
    MasterCautionInd {
        control: frontdash::MasterCautionInd,
        callback: LedCallback<'a>,
    },
    NavLtsSw {
        control: overheadpanel::NavLtsSw,
        callback: overheadpanel::NavLtsSwValueCallback<'a>,
    },
    NonEssBusSw {
        control: electricalsystem::NonEssBusSw,
        callback: electricalsystem::NonEssBusSwValueCallback<'a>,
    },
    NvgPosLts {
        control: overheadpanel::NvgPosLts,
        callback: overheadpanel::NvgPosLtsValueCallback<'a>,
    },
    OadiOff {
        control: frontdashadioperator::OadiOff,
        callback: IntegerCallback<'a>,
    },
    OadiPitch {
        control: frontdashadioperator::OadiPitch,
        callback: IntegerCallback<'a>,
    },
    OadiRoll {
        control: frontdashadioperator::OadiRoll,
        callback: IntegerCallback<'a>,
    },
    Oalt100 {
        control: altimeteroperator::Oalt100,
        callback: IntegerCallback<'a>,
    },
    Oalt1000 {
        control: altimeteroperator::Oalt1000,
        callback: IntegerCallback<'a>,
    },
    Oalt10000 {
        control: altimeteroperator::Oalt10000,
        callback: IntegerCallback<'a>,
    },
    OaltCodeOffFlag {
        control: altimeteroperator::OaltCodeOffFlag,
        callback: IntegerCallback<'a>,
    },
    OaltPress0 {
        control: altimeteroperator::OaltPress0,
        callback: IntegerCallback<'a>,
    },
    OaltPress1 {
        control: altimeteroperator::OaltPress1,
        callback: IntegerCallback<'a>,
    },
    OaltPress2 {
        control: altimeteroperator::OaltPress2,
        callback: IntegerCallback<'a>,
    },
    OaltPtr {
        control: altimeteroperator::OaltPtr,
        callback: IntegerCallback<'a>,
    },
    PadiOff {
        control: frontdashadipilot::PadiOff,
        callback: IntegerCallback<'a>,
    },
    PadiPitch {
        control: frontdashadipilot::PadiPitch,
        callback: IntegerCallback<'a>,
    },
    PadiRoll {
        control: frontdashadipilot::PadiRoll,
        callback: IntegerCallback<'a>,
    },
    Palt100 {
        control: altimeterpilot::Palt100,
        callback: IntegerCallback<'a>,
    },
    Palt1000 {
        control: altimeterpilot::Palt1000,
        callback: IntegerCallback<'a>,
    },
    Palt10000 {
        control: altimeterpilot::Palt10000,
        callback: IntegerCallback<'a>,
    },
    PaltPress {
        control: altimeterpilot::PaltPress,
        callback: IntegerCallback<'a>,
    },
    PitotHeat {
        control: overheadpanel::PitotHeat,
        callback: overheadpanel::PitotHeatValueCallback<'a>,
    },
    PosLtsSw {
        control: overheadpanel::PosLtsSw,
        callback: overheadpanel::PosLtsSwValueCallback<'a>,
    },
    RadarAltHi {
        control: frontdash::RadarAltHi,
        callback: IntegerCallback<'a>,
    },
    RadarAltLo {
        control: frontdash::RadarAltLo,
        callback: IntegerCallback<'a>,
    },
    RadarAltPwr {
        control: overheadpanel::RadarAltPwr,
        callback: overheadpanel::RadarAltPwrValueCallback<'a>,
    },
    RadarAltTest {
        control: frontdash::RadarAltTest,
        callback: frontdash::RadarAltTestValueCallback<'a>,
    },
    RadioIcsSw {
        control: cyclic::RadioIcsSw,
        callback: cyclic::RadioIcsSwValueCallback<'a>,
    },
    RaltDigit1 {
        control: radaraltimeter::RaltDigit1,
        callback: IntegerCallback<'a>,
    },
    RaltDigit2 {
        control: radaraltimeter::RaltDigit2,
        callback: IntegerCallback<'a>,
    },
    RaltDigit3 {
        control: radaraltimeter::RaltDigit3,
        callback: IntegerCallback<'a>,
    },
    RaltDigit4 {
        control: radaraltimeter::RaltDigit4,
        callback: IntegerCallback<'a>,
    },
    RaltDisplay {
        control: radaraltimeter::RaltDisplay,
        callback: IntegerCallback<'a>,
    },
    RaltDisplayStr {
        control: radaraltimeter::RaltDisplayStr,
        callback: DisplayCallback<'a>,
    },
    RaltHiIdx {
        control: radaraltimeter::RaltHiIdx,
        callback: IntegerCallback<'a>,
    },
    RaltHiLamp {
        control: radaraltimeter::RaltHiLamp,
        callback: LedCallback<'a>,
    },
    RaltLoIdx {
        control: radaraltimeter::RaltLoIdx,
        callback: IntegerCallback<'a>,
    },
    RaltLoLamp {
        control: radaraltimeter::RaltLoLamp,
        callback: LedCallback<'a>,
    },
    RaltNeedle {
        control: radaraltimeter::RaltNeedle,
        callback: IntegerCallback<'a>,
    },
    RaltOff {
        control: radaraltimeter::RaltOff,
        callback: IntegerCallback<'a>,
    },
    RamTemp {
        control: gauges::RamTemp,
        callback: IntegerCallback<'a>,
    },
    RmiCrs1 {
        control: copilotrmi::RmiCrs1,
        callback: IntegerCallback<'a>,
    },
    RmiCrs2 {
        control: copilotrmi::RmiCrs2,
        callback: IntegerCallback<'a>,
    },
    RmiHdg {
        control: copilotrmi::RmiHdg,
        callback: IntegerCallback<'a>,
    },
    RocketPair {
        control: armamentpanel::RocketPair,
        callback: armamentpanel::RocketPairValueCallback<'a>,
    },
    RocketReset {
        control: armamentpanel::RocketReset,
        callback: armamentpanel::RocketResetValueCallback<'a>,
    },
    RocketSel {
        control: armamentpanel::RocketSel,
        callback: armamentpanel::RocketSelValueCallback<'a>,
    },
    RotorRpm {
        control: frontdash::RotorRpm,
        callback: IntegerCallback<'a>,
    },
    SafeInd {
        control: armamentpanel::SafeInd,
        callback: LedCallback<'a>,
    },
    SbyCompassBank {
        control: frontdash::SbyCompassBank,
        callback: IntegerCallback<'a>,
    },
    SbyCompassHdg {
        control: frontdash::SbyCompassHdg,
        callback: IntegerCallback<'a>,
    },
    SbyCompassPitch {
        control: frontdash::SbyCompassPitch,
        callback: IntegerCallback<'a>,
    },
    SearchLightSw {
        control: collective::SearchLightSw,
        callback: collective::SearchLightSwValueCallback<'a>,
    },
    SearchLightSw2 {
        control: collective::SearchLightSw2,
        callback: collective::SearchLightSw2ValueCallback<'a>,
    },
    Sideslip {
        control: frontdash::Sideslip,
        callback: IntegerCallback<'a>,
    },
    SightArmPlt {
        control: flexsight::SightArmPlt,
        callback: flexsight::SightArmPltValueCallback<'a>,
    },
    SightElevPlt {
        control: flexsight::SightElevPlt,
        callback: IntegerCallback<'a>,
    },
    SightIntenCplt {
        control: flexsight::SightIntenCplt,
        callback: IntegerCallback<'a>,
    },
    SightIntenPlt {
        control: flexsight::SightIntenPlt,
        callback: IntegerCallback<'a>,
    },
    SightLampSw {
        control: flexsight::SightLampSw,
        callback: flexsight::SightLampSwValueCallback<'a>,
    },
    SightPwrPlt {
        control: flexsight::SightPwrPlt,
        callback: flexsight::SightPwrPltValueCallback<'a>,
    },
    SightsForCivil {
        control: gauges::SightsForCivil,
        callback: IntegerCallback<'a>,
    },
    StarterGenSw {
        control: electricalsystem::StarterGenSw,
        callback: electricalsystem::StarterGenSwValueCallback<'a>,
    },
    StbyGenLoad {
        control: frontdash::StbyGenLoad,
        callback: IntegerCallback<'a>,
    },
    TestFuelGaugeBtn {
        control: fuelsystem::TestFuelGaugeBtn,
        callback: fuelsystem::TestFuelGaugeBtnValueCallback<'a>,
    },
    Throttle {
        control: collective::Throttle,
        callback: IntegerCallback<'a>,
    },
    Throttle2 {
        control: collective::Throttle2,
        callback: IntegerCallback<'a>,
    },
    ThrottleStop {
        control: collective::ThrottleStop,
        callback: collective::ThrottleStopValueCallback<'a>,
    },
    ThrottleStop2 {
        control: collective::ThrottleStop2,
        callback: collective::ThrottleStop2ValueCallback<'a>,
    },
    TorquePress {
        control: frontdash::TorquePress,
        callback: IntegerCallback<'a>,
    },
    TransOilPress {
        control: frontdash::TransOilPress,
        callback: IntegerCallback<'a>,
    },
    TransOilTemp {
        control: frontdash::TransOilTemp,
        callback: IntegerCallback<'a>,
    },
    TrimCplt {
        control: cyclic::TrimCplt,
        callback: cyclic::TrimCpltValueCallback<'a>,
    },
    TrimPlt {
        control: cyclic::TrimPlt,
        callback: cyclic::TrimPltValueCallback<'a>,
    },
    TurnPtr {
        control: frontdash::TurnPtr,
        callback: IntegerCallback<'a>,
    },
    UhfFreq {
        control: uhfradio::UhfFreq,
        callback: DisplayCallback<'a>,
    },
    UhfFunction {
        control: uhfradio::UhfFunction,
        callback: uhfradio::UhfFunctionValueCallback<'a>,
    },
    UhfMode {
        control: uhfradio::UhfMode,
        callback: uhfradio::UhfModeValueCallback<'a>,
    },
    UhfPresetInt {
        control: uhfradio::UhfPresetInt,
        callback: IntegerCallback<'a>,
    },
    UhfPresetStr {
        control: uhfradio::UhfPresetStr,
        callback: DisplayCallback<'a>,
    },
    UhfSquelchSw {
        control: uhfradio::UhfSquelchSw,
        callback: uhfradio::UhfSquelchSwValueCallback<'a>,
    },
    UhfVol {
        control: uhfradio::UhfVol,
        callback: IntegerCallback<'a>,
    },
    VhfcommFreq {
        control: vhfcommradio::VhfcommFreq,
        callback: DisplayCallback<'a>,
    },
    VhfcommKhz {
        control: vhfcommradio::VhfcommKhz,
        callback: IntegerCallback<'a>,
    },
    VhfcommMhz {
        control: vhfcommradio::VhfcommMhz,
        callback: IntegerCallback<'a>,
    },
    VhfcommPwr {
        control: vhfcommradio::VhfcommPwr,
        callback: vhfcommradio::VhfcommPwrValueCallback<'a>,
    },
    VhfcommTestSw {
        control: vhfcommradio::VhfcommTestSw,
        callback: vhfcommradio::VhfcommTestSwValueCallback<'a>,
    },
    VhfcommVol {
        control: vhfcommradio::VhfcommVol,
        callback: IntegerCallback<'a>,
    },
    VhffmFreq1Int {
        control: vhffmradio::VhffmFreq1Int,
        callback: vhffmradio::VhffmFreq1IntValueCallback<'a>,
    },
    VhffmFreq1Str {
        control: vhffmradio::VhffmFreq1Str,
        callback: DisplayCallback<'a>,
    },
    VhffmFreq2 {
        control: vhffmradio::VhffmFreq2,
        callback: vhffmradio::VhffmFreq2ValueCallback<'a>,
    },
    VhffmFreq3 {
        control: vhffmradio::VhffmFreq3,
        callback: vhffmradio::VhffmFreq3ValueCallback<'a>,
    },
    VhffmFreq4Int {
        control: vhffmradio::VhffmFreq4Int,
        callback: vhffmradio::VhffmFreq4IntValueCallback<'a>,
    },
    VhffmFreq4Str {
        control: vhffmradio::VhffmFreq4Str,
        callback: DisplayCallback<'a>,
    },
    VhffmMode {
        control: vhffmradio::VhffmMode,
        callback: vhffmradio::VhffmModeValueCallback<'a>,
    },
    VhffmSquelch {
        control: vhffmradio::VhffmSquelch,
        callback: vhffmradio::VhffmSquelchValueCallback<'a>,
    },
    VhffmVol {
        control: vhffmradio::VhffmVol,
        callback: IntegerCallback<'a>,
    },
    VhfnavFreq {
        control: vhfnavradio::VhfnavFreq,
        callback: DisplayCallback<'a>,
    },
    VhfnavKhz {
        control: vhfnavradio::VhfnavKhz,
        callback: IntegerCallback<'a>,
    },
    VhfnavMhz {
        control: vhfnavradio::VhfnavMhz,
        callback: IntegerCallback<'a>,
    },
    VhfnavPwr {
        control: vhfnavradio::VhfnavPwr,
        callback: vhfnavradio::VhfnavPwrValueCallback<'a>,
    },
    VhfnavVol {
        control: vhfnavradio::VhfnavVol,
        callback: IntegerCallback<'a>,
    },
    VviO {
        control: frontdash::VviO,
        callback: IntegerCallback<'a>,
    },
    VviP {
        control: frontdash::VviP,
        callback: IntegerCallback<'a>,
    },
    WiperO {
        control: windshieldwipers::WiperO,
        callback: IntegerCallback<'a>,
    },
    WiperP {
        control: windshieldwipers::WiperP,
        callback: IntegerCallback<'a>,
    },
    WiperSel {
        control: overheadpanel::WiperSel,
        callback: overheadpanel::WiperSelValueCallback<'a>,
    },
    WiperSpd {
        control: overheadpanel::WiperSpd,
        callback: IntegerCallback<'a>,
    },
    X130Armed {
        control: armamentpanel::X130Armed,
        callback: LedCallback<'a>,
    },
}

impl RegisteredControl<'_> {
    fn accept(&mut self, packet: Packet<'_>) {
        match self {
            Self::AcGen1Current { control, .. } => control.accept(packet),
            Self::AcGen2Current { control, .. } => control.accept(packet),
            Self::AcV { control, .. } => control.accept(packet),
            Self::AcVmSrc { control, .. } => control.accept(packet),
            Self::AcVoltage { control, .. } => control.accept(packet),
            Self::AdfBand { control, .. } => control.accept(packet),
            Self::AdfBfoSw { control, .. } => control.accept(packet),
            Self::AdfFreq { control, .. } => control.accept(packet),
            Self::AdfGain { control, .. } => control.accept(packet),
            Self::AdfLoopLr { control, .. } => control.accept(packet),
            Self::AdfMode { control, .. } => control.accept(packet),
            Self::AdfSignal { control, .. } => control.accept(packet),
            Self::AdfTune { control, .. } => control.accept(packet),
            Self::AdfVorSw { control, .. } => control.accept(packet),
            Self::AdiPitchShift { control, .. } => control.accept(packet),
            Self::AdiPitchTrim { control, .. } => control.accept(packet),
            Self::AdiPitchTrimCplt { control, .. } => control.accept(packet),
            Self::AdiRollTrim { control, .. } => control.accept(packet),
            Self::AltAdjCplt { control, .. } => control.accept(packet),
            Self::AltAdjPlt { control, .. } => control.accept(packet),
            Self::AntiIceAmp { control, .. } => control.accept(packet),
            Self::AnticollLtsSw { control, .. } => control.accept(packet),
            Self::ArmedInd { control, .. } => control.accept(packet),
            Self::BatSw { control, .. } => control.accept(packet),
            Self::BeaconSensingSw { control, .. } => control.accept(packet),
            Self::BeaconVol { control, .. } => control.accept(packet),
            Self::BleedAirSw { control, .. } => control.accept(packet),
            Self::BrtConsole { control, .. } => control.accept(packet),
            Self::BrtCopilot { control, .. } => control.accept(packet),
            Self::BrtEngine { control, .. } => control.accept(packet),
            Self::BrtPed { control, .. } => control.accept(packet),
            Self::BrtPilot { control, .. } => control.accept(packet),
            Self::BrtSec { control, .. } => control.accept(packet),
            Self::CargoRelCp { control, .. } => control.accept(packet),
            Self::CargoRelP { control, .. } => control.accept(packet),
            Self::CargoRelSafe { control, .. } => control.accept(packet),
            Self::Cb28v { control, .. } => control.accept(packet),
            Self::CbAirValve { control, .. } => control.accept(packet),
            Self::CbAmRadio { control, .. } => control.accept(packet),
            Self::CbAntiIce { control, .. } => control.accept(packet),
            Self::CbAnticoll { control, .. } => control.accept(packet),
            Self::CbArc102 { control, .. } => control.accept(packet),
            Self::CbCable { control, .. } => control.accept(packet),
            Self::CbCautionLts { control, .. } => control.accept(packet),
            Self::CbCockpitLts { control, .. } => control.accept(packet),
            Self::CbConsoleLts { control, .. } => control.accept(packet),
            Self::CbCpltAttd1 { control, .. } => control.accept(packet),
            Self::CbCpltAttd2 { control, .. } => control.accept(packet),
            Self::CbCrsind { control, .. } => control.accept(packet),
            Self::CbDomeLts { control, .. } => control.accept(packet),
            Self::CbEng { control, .. } => control.accept(packet),
            Self::CbFailRelay { control, .. } => control.accept(packet),
            Self::CbFire { control, .. } => control.accept(packet),
            Self::CbFm2Radio { control, .. } => control.accept(packet),
            Self::CbFmRadio { control, .. } => control.accept(packet),
            Self::CbFuel { control, .. } => control.accept(packet),
            Self::CbFuelQty { control, .. } => control.accept(packet),
            Self::CbFuelTrans { control, .. } => control.accept(packet),
            Self::CbFuelValves { control, .. } => control.accept(packet),
            Self::CbFusLts { control, .. } => control.accept(packet),
            Self::CbGenBus { control, .. } => control.accept(packet),
            Self::CbGov { control, .. } => control.accept(packet),
            Self::CbGyro { control, .. } => control.accept(packet),
            Self::CbHeatedBlanket1 { control, .. } => control.accept(packet),
            Self::CbHeatedBlanket2 { control, .. } => control.accept(packet),
            Self::CbHfAnt { control, .. } => control.accept(packet),
            Self::CbHfArc102 { control, .. } => control.accept(packet),
            Self::CbHook { control, .. } => control.accept(packet),
            Self::CbHydctl { control, .. } => control.accept(packet),
            Self::CbIdleStop { control, .. } => control.accept(packet),
            Self::CbIffApx1 { control, .. } => control.accept(packet),
            Self::CbIffApx2 { control, .. } => control.accept(packet),
            Self::CbIgn { control, .. } => control.accept(packet),
            Self::CbInstLts { control, .. } => control.accept(packet),
            Self::CbInstSecLts { control, .. } => control.accept(packet),
            Self::CbIntCplt { control, .. } => control.accept(packet),
            Self::CbIntPlt { control, .. } => control.accept(packet),
            Self::CbInvtr { control, .. } => control.accept(packet),
            Self::CbJtsn { control, .. } => control.accept(packet),
            Self::CbKy20 { control, .. } => control.accept(packet),
            Self::CbLdglts { control, .. } => control.accept(packet),
            Self::CbLfNav { control, .. } => control.accept(packet),
            Self::CbLhFuel { control, .. } => control.accept(packet),
            Self::CbMainInvtr { control, .. } => control.accept(packet),
            Self::CbMarkerBeacon { control, .. } => control.accept(packet),
            Self::CbNavLts { control, .. } => control.accept(packet),
            Self::CbOutletValve { control, .. } => control.accept(packet),
            Self::CbPitotTube { control, .. } => control.accept(packet),
            Self::CbPltAttd1 { control, .. } => control.accept(packet),
            Self::CbPltAttd2 { control, .. } => control.accept(packet),
            Self::CbPorxWarn { control, .. } => control.accept(packet),
            Self::CbRescueCont { control, .. } => control.accept(packet),
            Self::CbRescuePwr { control, .. } => control.accept(packet),
            Self::CbRhFuel { control, .. } => control.accept(packet),
            Self::CbRpmWarn { control, .. } => control.accept(packet),
            Self::CbSlCtl { control, .. } => control.accept(packet),
            Self::CbSlPwr { control, .. } => control.accept(packet),
            Self::CbSpareInvtr { control, .. } => control.accept(packet),
            Self::CbStarter { control, .. } => control.accept(packet),
            Self::CbStbyGen { control, .. } => control.accept(packet),
            Self::CbTemp { control, .. } => control.accept(packet),
            Self::CbTorq { control, .. } => control.accept(packet),
            Self::CbTrim { control, .. } => control.accept(packet),
            Self::CbTurnSlip { control, .. } => control.accept(packet),
            Self::CbUhfRadio { control, .. } => control.accept(packet),
            Self::CbVhfNav { control, .. } => control.accept(packet),
            Self::CbVm { control, .. } => control.accept(packet),
            Self::CbWiperCplt { control, .. } => control.accept(packet),
            Self::CbWiperPlt { control, .. } => control.accept(packet),
            Self::CbXmsn { control, .. } => control.accept(packet),
            Self::ChipDetSw { control, .. } => control.accept(packet),
            Self::CiCourseCard { control, .. } => control.accept(packet),
            Self::CiFrom { control, .. } => control.accept(packet),
            Self::CiHbar { control, .. } => control.accept(packet),
            Self::CiHoff { control, .. } => control.accept(packet),
            Self::CiTo { control, .. } => control.accept(packet),
            Self::CiVbar { control, .. } => control.accept(packet),
            Self::CiVoff { control, .. } => control.accept(packet),
            Self::ClA0 { control, .. } => control.accept(packet),
            Self::ClA1 { control, .. } => control.accept(packet),
            Self::ClB0 { control, .. } => control.accept(packet),
            Self::ClB1 { control, .. } => control.accept(packet),
            Self::ClC0 { control, .. } => control.accept(packet),
            Self::ClC1 { control, .. } => control.accept(packet),
            Self::ClD0 { control, .. } => control.accept(packet),
            Self::ClD1 { control, .. } => control.accept(packet),
            Self::ClE0 { control, .. } => control.accept(packet),
            Self::ClE1 { control, .. } => control.accept(packet),
            Self::ClF0 { control, .. } => control.accept(packet),
            Self::ClF1 { control, .. } => control.accept(packet),
            Self::ClG0 { control, .. } => control.accept(packet),
            Self::ClG1 { control, .. } => control.accept(packet),
            Self::ClH0 { control, .. } => control.accept(packet),
            Self::ClH1 { control, .. } => control.accept(packet),
            Self::ClI0 { control, .. } => control.accept(packet),
            Self::ClI1 { control, .. } => control.accept(packet),
            Self::ClJ0 { control, .. } => control.accept(packet),
            Self::ClJ1 { control, .. } => control.accept(packet),
            Self::ClockAdjust { control, .. } => control.accept(packet),
            Self::ClockAdjustPull { control, .. } => control.accept(packet),
            Self::ClockH { control, .. } => control.accept(packet),
            Self::ClockM { control, .. } => control.accept(packet),
            Self::ClockS { control, .. } => control.accept(packet),
            Self::ClpBrightDimSw { control, .. } => control.accept(packet),
            Self::ClpResetTestSw { control, .. } => control.accept(packet),
            Self::CmArmSw { control, .. } => control.accept(packet),
            Self::CmArmedTest { control, .. } => control.accept(packet),
            Self::CmChaffcntDisplay { control, .. } => control.accept(packet),
            Self::CmChaffcntDisplayStr { control, .. } => control.accept(packet),
            Self::CmChaffcntReset { control, .. } => control.accept(packet),
            Self::CmFlareBtn { control, .. } => control.accept(packet),
            Self::CmFlarecntDisplay { control, .. } => control.accept(packet),
            Self::CmFlarecntDisplayStr { control, .. } => control.accept(packet),
            Self::CmFlarecntReset { control, .. } => control.accept(packet),
            Self::CmManPgrmSw { control, .. } => control.accept(packet),
            Self::CmRippleCover { control, .. } => control.accept(packet),
            Self::CmRippleSw { control, .. } => control.accept(packet),
            Self::ConsoleLight { control, .. } => control.accept(packet),
            Self::CpltAdiCage { control, .. } => control.accept(packet),
            Self::CrsKnob { control, .. } => control.accept(packet),
            Self::DcBat1Current { control, .. } => control.accept(packet),
            Self::DcBat2Current { control, .. } => control.accept(packet),
            Self::DcV { control, .. } => control.accept(packet),
            Self::DcVmSrc { control, .. } => control.accept(packet),
            Self::DcVoltage { control, .. } => control.accept(packet),
            Self::DcVu1Current { control, .. } => control.accept(packet),
            Self::DcVu2Current { control, .. } => control.accept(packet),
            Self::DcVu3Current { control, .. } => control.accept(packet),
            Self::DomeLight { control, .. } => control.accept(packet),
            Self::DomeLightGreen { control, .. } => control.accept(packet),
            Self::DomeLightSw { control, .. } => control.accept(packet),
            Self::DoorLPtr { control, .. } => control.accept(packet),
            Self::DoorRPtr { control, .. } => control.accept(packet),
            Self::EmerGovSw { control, .. } => control.accept(packet),
            Self::EngDeice { control, .. } => control.accept(packet),
            Self::EngOilPress { control, .. } => control.accept(packet),
            Self::EngOilTemp { control, .. } => control.accept(packet),
            Self::EngRpm { control, .. } => control.accept(packet),
            Self::ExhaustTemp { control, .. } => control.accept(packet),
            Self::ExtPositionLightLeft { control, .. } => control.accept(packet),
            Self::ExtPositionLightRight { control, .. } => control.accept(packet),
            Self::ExtStrobe { control, .. } => control.accept(packet),
            Self::ExtWowSkid { control, .. } => control.accept(packet),
            Self::FireInd { control, .. } => control.accept(packet),
            Self::FireTestBtn { control, .. } => control.accept(packet),
            Self::ForceTrimSw { control, .. } => control.accept(packet),
            Self::FuelPress { control, .. } => control.accept(packet),
            Self::FuelQty { control, .. } => control.accept(packet),
            Self::GasRpm { control, .. } => control.accept(packet),
            Self::GasRpmT { control, .. } => control.accept(packet),
            Self::GmcAnnunciator { control, .. } => control.accept(packet),
            Self::GmcCrs1 { control, .. } => control.accept(packet),
            Self::GmcCrs2 { control, .. } => control.accept(packet),
            Self::GmcHdg { control, .. } => control.accept(packet),
            Self::GmcHdgMarker { control, .. } => control.accept(packet),
            Self::GmcPwrfail { control, .. } => control.accept(packet),
            Self::GovRpmSw { control, .. } => control.accept(packet),
            Self::GovRpmSw2 { control, .. } => control.accept(packet),
            Self::GunSel { control, .. } => control.accept(packet),
            Self::GyroModeSw { control, .. } => control.accept(packet),
            Self::HdgSet { control, .. } => control.accept(packet),
            Self::HdgSync { control, .. } => control.accept(packet),
            Self::HydContSw { control, .. } => control.accept(packet),
            Self::IasNose { control, .. } => control.accept(packet),
            Self::IasRoof { control, .. } => control.accept(packet),
            Self::IffCode { control, .. } => control.accept(packet),
            Self::IffMaster { control, .. } => control.accept(packet),
            Self::IffMicIdent { control, .. } => control.accept(packet),
            Self::IffMode1Wheel1 { control, .. } => control.accept(packet),
            Self::IffMode1Wheel2 { control, .. } => control.accept(packet),
            Self::IffMode3aWheel1 { control, .. } => control.accept(packet),
            Self::IffMode3aWheel2 { control, .. } => control.accept(packet),
            Self::IffMode3aWheel3 { control, .. } => control.accept(packet),
            Self::IffMode3aWheel4 { control, .. } => control.accept(packet),
            Self::IffOnOut { control, .. } => control.accept(packet),
            Self::IffOutAudioLight { control, .. } => control.accept(packet),
            Self::IffRadtest { control, .. } => control.accept(packet),
            Self::IffReplyDim { control, .. } => control.accept(packet),
            Self::IffReplyInd { control, .. } => control.accept(packet),
            Self::IffReplyTest { control, .. } => control.accept(packet),
            Self::IffTestDim { control, .. } => control.accept(packet),
            Self::IffTestInd { control, .. } => control.accept(packet),
            Self::IffTestM1 { control, .. } => control.accept(packet),
            Self::IffTestM2 { control, .. } => control.accept(packet),
            Self::IffTestM3 { control, .. } => control.accept(packet),
            Self::IffTestM4 { control, .. } => control.accept(packet),
            Self::IffTestTest { control, .. } => control.accept(packet),
            Self::IntIntSw { control, .. } => control.accept(packet),
            Self::IntMode { control, .. } => control.accept(packet),
            Self::IntNavSw { control, .. } => control.accept(packet),
            Self::IntRcvr1Sw { control, .. } => control.accept(packet),
            Self::IntRcvr2Sw { control, .. } => control.accept(packet),
            Self::IntRcvr3Sw { control, .. } => control.accept(packet),
            Self::IntRecvr4Sw { control, .. } => control.accept(packet),
            Self::IntVol { control, .. } => control.accept(packet),
            Self::InverterSw { control, .. } => control.accept(packet),
            Self::JtsnBtn { control, .. } => control.accept(packet),
            Self::JtsnCover { control, .. } => control.accept(packet),
            Self::LdgLightSw { control, .. } => control.accept(packet),
            Self::LdgLightSw2 { control, .. } => control.accept(packet),
            Self::LdgLtCtrl { control, .. } => control.accept(packet),
            Self::LdgLtCtrl2 { control, .. } => control.accept(packet),
            Self::LowRpmAudio { control, .. } => control.accept(packet),
            Self::LowRpmInd { control, .. } => control.accept(packet),
            Self::MainFuelSw { control, .. } => control.accept(packet),
            Self::MainGenCover { control, .. } => control.accept(packet),
            Self::MainGenLoad { control, .. } => control.accept(packet),
            Self::MainGenSw { control, .. } => control.accept(packet),
            Self::MarkerBeacon { control, .. } => control.accept(packet),
            Self::MasterArmSw { control, .. } => control.accept(packet),
            Self::MasterCautionInd { control, .. } => control.accept(packet),
            Self::NavLtsSw { control, .. } => control.accept(packet),
            Self::NonEssBusSw { control, .. } => control.accept(packet),
            Self::NvgPosLts { control, .. } => control.accept(packet),
            Self::OadiOff { control, .. } => control.accept(packet),
            Self::OadiPitch { control, .. } => control.accept(packet),
            Self::OadiRoll { control, .. } => control.accept(packet),
            Self::Oalt100 { control, .. } => control.accept(packet),
            Self::Oalt1000 { control, .. } => control.accept(packet),
            Self::Oalt10000 { control, .. } => control.accept(packet),
            Self::OaltCodeOffFlag { control, .. } => control.accept(packet),
            Self::OaltPress0 { control, .. } => control.accept(packet),
            Self::OaltPress1 { control, .. } => control.accept(packet),
            Self::OaltPress2 { control, .. } => control.accept(packet),
            Self::OaltPtr { control, .. } => control.accept(packet),
            Self::PadiOff { control, .. } => control.accept(packet),
            Self::PadiPitch { control, .. } => control.accept(packet),
            Self::PadiRoll { control, .. } => control.accept(packet),
            Self::Palt100 { control, .. } => control.accept(packet),
            Self::Palt1000 { control, .. } => control.accept(packet),
            Self::Palt10000 { control, .. } => control.accept(packet),
            Self::PaltPress { control, .. } => control.accept(packet),
            Self::PitotHeat { control, .. } => control.accept(packet),
            Self::PosLtsSw { control, .. } => control.accept(packet),
            Self::RadarAltHi { control, .. } => control.accept(packet),
            Self::RadarAltLo { control, .. } => control.accept(packet),
            Self::RadarAltPwr { control, .. } => control.accept(packet),
            Self::RadarAltTest { control, .. } => control.accept(packet),
            Self::RadioIcsSw { control, .. } => control.accept(packet),
            Self::RaltDigit1 { control, .. } => control.accept(packet),
            Self::RaltDigit2 { control, .. } => control.accept(packet),
            Self::RaltDigit3 { control, .. } => control.accept(packet),
            Self::RaltDigit4 { control, .. } => control.accept(packet),
            Self::RaltDisplay { control, .. } => control.accept(packet),
            Self::RaltDisplayStr { control, .. } => control.accept(packet),
            Self::RaltHiIdx { control, .. } => control.accept(packet),
            Self::RaltHiLamp { control, .. } => control.accept(packet),
            Self::RaltLoIdx { control, .. } => control.accept(packet),
            Self::RaltLoLamp { control, .. } => control.accept(packet),
            Self::RaltNeedle { control, .. } => control.accept(packet),
            Self::RaltOff { control, .. } => control.accept(packet),
            Self::RamTemp { control, .. } => control.accept(packet),
            Self::RmiCrs1 { control, .. } => control.accept(packet),
            Self::RmiCrs2 { control, .. } => control.accept(packet),
            Self::RmiHdg { control, .. } => control.accept(packet),
            Self::RocketPair { control, .. } => control.accept(packet),
            Self::RocketReset { control, .. } => control.accept(packet),
            Self::RocketSel { control, .. } => control.accept(packet),
            Self::RotorRpm { control, .. } => control.accept(packet),
            Self::SafeInd { control, .. } => control.accept(packet),
            Self::SbyCompassBank { control, .. } => control.accept(packet),
            Self::SbyCompassHdg { control, .. } => control.accept(packet),
            Self::SbyCompassPitch { control, .. } => control.accept(packet),
            Self::SearchLightSw { control, .. } => control.accept(packet),
            Self::SearchLightSw2 { control, .. } => control.accept(packet),
            Self::Sideslip { control, .. } => control.accept(packet),
            Self::SightArmPlt { control, .. } => control.accept(packet),
            Self::SightElevPlt { control, .. } => control.accept(packet),
            Self::SightIntenCplt { control, .. } => control.accept(packet),
            Self::SightIntenPlt { control, .. } => control.accept(packet),
            Self::SightLampSw { control, .. } => control.accept(packet),
            Self::SightPwrPlt { control, .. } => control.accept(packet),
            Self::SightsForCivil { control, .. } => control.accept(packet),
            Self::StarterGenSw { control, .. } => control.accept(packet),
            Self::StbyGenLoad { control, .. } => control.accept(packet),
            Self::TestFuelGaugeBtn { control, .. } => control.accept(packet),
            Self::Throttle { control, .. } => control.accept(packet),
            Self::Throttle2 { control, .. } => control.accept(packet),
            Self::ThrottleStop { control, .. } => control.accept(packet),
            Self::ThrottleStop2 { control, .. } => control.accept(packet),
            Self::TorquePress { control, .. } => control.accept(packet),
            Self::TransOilPress { control, .. } => control.accept(packet),
            Self::TransOilTemp { control, .. } => control.accept(packet),
            Self::TrimCplt { control, .. } => control.accept(packet),
            Self::TrimPlt { control, .. } => control.accept(packet),
            Self::TurnPtr { control, .. } => control.accept(packet),
            Self::UhfFreq { control, .. } => control.accept(packet),
            Self::UhfFunction { control, .. } => control.accept(packet),
            Self::UhfMode { control, .. } => control.accept(packet),
            Self::UhfPresetInt { control, .. } => control.accept(packet),
            Self::UhfPresetStr { control, .. } => control.accept(packet),
            Self::UhfSquelchSw { control, .. } => control.accept(packet),
            Self::UhfVol { control, .. } => control.accept(packet),
            Self::VhfcommFreq { control, .. } => control.accept(packet),
            Self::VhfcommKhz { control, .. } => control.accept(packet),
            Self::VhfcommMhz { control, .. } => control.accept(packet),
            Self::VhfcommPwr { control, .. } => control.accept(packet),
            Self::VhfcommTestSw { control, .. } => control.accept(packet),
            Self::VhfcommVol { control, .. } => control.accept(packet),
            Self::VhffmFreq1Int { control, .. } => control.accept(packet),
            Self::VhffmFreq1Str { control, .. } => control.accept(packet),
            Self::VhffmFreq2 { control, .. } => control.accept(packet),
            Self::VhffmFreq3 { control, .. } => control.accept(packet),
            Self::VhffmFreq4Int { control, .. } => control.accept(packet),
            Self::VhffmFreq4Str { control, .. } => control.accept(packet),
            Self::VhffmMode { control, .. } => control.accept(packet),
            Self::VhffmSquelch { control, .. } => control.accept(packet),
            Self::VhffmVol { control, .. } => control.accept(packet),
            Self::VhfnavFreq { control, .. } => control.accept(packet),
            Self::VhfnavKhz { control, .. } => control.accept(packet),
            Self::VhfnavMhz { control, .. } => control.accept(packet),
            Self::VhfnavPwr { control, .. } => control.accept(packet),
            Self::VhfnavVol { control, .. } => control.accept(packet),
            Self::VviO { control, .. } => control.accept(packet),
            Self::VviP { control, .. } => control.accept(packet),
            Self::WiperO { control, .. } => control.accept(packet),
            Self::WiperP { control, .. } => control.accept(packet),
            Self::WiperSel { control, .. } => control.accept(packet),
            Self::WiperSpd { control, .. } => control.accept(packet),
            Self::X130Armed { control, .. } => control.accept(packet),
        }
    }

    fn commit(&mut self) {
        match self {
            Self::AcGen1Current { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AcGen2Current { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AcV { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AcVmSrc { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <electricalsystem::AcVmSrcValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::AcVoltage { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdfBand { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <adf::AdfBandValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::AdfBfoSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <adf::AdfBfoSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::AdfFreq { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdfGain { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdfLoopLr { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <adf::AdfLoopLrValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
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
            Self::AdfSignal { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdfTune { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdfVorSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <frontdash::AdfVorSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AdiPitchShift { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdiPitchTrim { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdiPitchTrimCplt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdiRollTrim { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AltAdjCplt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AltAdjPlt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AntiIceAmp { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AnticollLtsSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <overheadpanel::AnticollLtsSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::ArmedInd { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::BatSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <electricalsystem::BatSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::BeaconSensingSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <frontdash::BeaconSensingSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::BeaconVol { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::BleedAirSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <overheadpanel::BleedAirSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::BrtConsole { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::BrtCopilot { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::BrtEngine { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::BrtPed { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::BrtPilot { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::BrtSec { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CargoRelCp { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <cyclic::CargoRelCpValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CargoRelP { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <cyclic::CargoRelPValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CargoRelSafe { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <overheadpanel::CargoRelSafeValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Cb28v { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::Cb28vValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbAirValve { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <circuitbreakers::CbAirValveValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CbAmRadio { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbAmRadioValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbAntiIce { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbAntiIceValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbAnticoll { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <circuitbreakers::CbAnticollValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CbArc102 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbArc102Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbCable { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbCableValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbCautionLts { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbCautionLtsValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::CbCockpitLts { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbCockpitLtsValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::CbConsoleLts { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbConsoleLtsValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::CbCpltAttd1 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <circuitbreakers::CbCpltAttd1Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CbCpltAttd2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <circuitbreakers::CbCpltAttd2Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CbCrsind { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbCrsindValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbDomeLts { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbDomeLtsValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbEng { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbEngValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbFailRelay { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <circuitbreakers::CbFailRelayValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CbFire { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbFireValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbFm2Radio { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <circuitbreakers::CbFm2RadioValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CbFmRadio { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbFmRadioValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbFuel { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbFuelValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbFuelQty { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbFuelQtyValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbFuelTrans { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <circuitbreakers::CbFuelTransValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CbFuelValves { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbFuelValvesValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::CbFusLts { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbFusLtsValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbGenBus { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbGenBusValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbGov { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbGovValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbGyro { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbGyroValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbHeatedBlanket1 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbHeatedBlanket1Value as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::CbHeatedBlanket2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbHeatedBlanket2Value as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::CbHfAnt { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbHfAntValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbHfArc102 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <circuitbreakers::CbHfArc102Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CbHook { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbHookValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbHydctl { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbHydctlValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbIdleStop { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <circuitbreakers::CbIdleStopValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CbIffApx1 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbIffApx1Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbIffApx2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbIffApx2Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbIgn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbIgnValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbInstLts { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbInstLtsValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbInstSecLts { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbInstSecLtsValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::CbIntCplt { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbIntCpltValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbIntPlt { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbIntPltValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbInvtr { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbInvtrValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbJtsn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbJtsnValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbKy20 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbKy20Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbLdglts { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbLdgltsValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbLfNav { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbLfNavValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbLhFuel { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbLhFuelValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbMainInvtr { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <circuitbreakers::CbMainInvtrValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CbMarkerBeacon { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbMarkerBeaconValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::CbNavLts { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbNavLtsValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbOutletValve { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbOutletValveValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::CbPitotTube { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <circuitbreakers::CbPitotTubeValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CbPltAttd1 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <circuitbreakers::CbPltAttd1Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CbPltAttd2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <circuitbreakers::CbPltAttd2Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CbPorxWarn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <circuitbreakers::CbPorxWarnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CbRescueCont { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbRescueContValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::CbRescuePwr { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <circuitbreakers::CbRescuePwrValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CbRhFuel { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbRhFuelValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbRpmWarn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbRpmWarnValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbSlCtl { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbSlCtlValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbSlPwr { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbSlPwrValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbSpareInvtr { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbSpareInvtrValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::CbStarter { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbStarterValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbStbyGen { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbStbyGenValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbTemp { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbTempValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbTorq { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbTorqValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbTrim { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbTrimValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbTurnSlip { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <circuitbreakers::CbTurnSlipValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CbUhfRadio { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <circuitbreakers::CbUhfRadioValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CbVhfNav { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbVhfNavValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbVm { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <circuitbreakers::CbVmValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CbWiperCplt { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <circuitbreakers::CbWiperCpltValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CbWiperPlt { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <circuitbreakers::CbWiperPltValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CbXmsn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <circuitbreakers::CbXmsnValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::ChipDetSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <centerconsole::ChipDetSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CiCourseCard { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CiFrom { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CiHbar { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CiHoff { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CiTo { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CiVbar { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CiVoff { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ClA0 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClA1 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClB0 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClB1 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClC0 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClC1 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClD0 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClD1 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClE0 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClE1 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClF0 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClF1 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClG0 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClG1 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClH0 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClH1 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClI0 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClI1 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClJ0 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClJ1 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClockAdjust { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ClockAdjustPull { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ClockH { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ClockM { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ClockS { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ClpBrightDimSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <cautionlightspanel::ClpBrightDimSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::ClpResetTestSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <cautionlightspanel::ClpResetTestSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::CmArmSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <countermeasures::CmArmSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CmArmedTest { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <countermeasures::CmArmedTestValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CmChaffcntDisplay { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CmChaffcntDisplayStr { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CmChaffcntReset { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <countermeasures::CmChaffcntResetValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::CmFlareBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <countermeasures::CmFlareBtnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CmFlarecntDisplay { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CmFlarecntDisplayStr { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CmFlarecntReset { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <countermeasures::CmFlarecntResetValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::CmManPgrmSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <countermeasures::CmManPgrmSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CmRippleCover { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <countermeasures::CmRippleCoverValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::CmRippleSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <countermeasures::CmRippleSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::ConsoleLight { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::CpltAdiCage { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <frontdash::CpltAdiCageValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CrsKnob { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DcBat1Current { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DcBat2Current { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DcV { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DcVmSrc { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <electricalsystem::DcVmSrcValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::DcVoltage { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DcVu1Current { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DcVu2Current { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DcVu3Current { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DomeLight { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DomeLightGreen { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DomeLightSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <overheadpanel::DomeLightSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::DoorLPtr { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <doors::DoorLPtrValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::DoorRPtr { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <doors::DoorRPtrValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::EmerGovSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <enginepanel::EmerGovSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::EngDeice { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <enginepanel::EngDeiceValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::EngOilPress { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::EngOilTemp { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::EngRpm { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ExhaustTemp { control, callback } => {
                if control.commit() {
                    callback(control.value());
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
            Self::ExtWowSkid { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FireInd { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::FireTestBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <frontdash::FireTestBtnValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::ForceTrimSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <centerconsole::ForceTrimSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FuelPress { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelQty { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::GasRpm { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::GasRpmT { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::GmcAnnunciator { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::GmcCrs1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::GmcCrs2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::GmcHdg { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::GmcHdgMarker { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::GmcPwrfail { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::GovRpmSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <collective::GovRpmSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::GovRpmSw2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <collective::GovRpmSw2Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::GunSel { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <armamentpanel::GunSelValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::GyroModeSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <frontdash::GyroModeSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::HdgSet { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HdgSync { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HydContSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <centerconsole::HydContSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IasNose { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IasRoof { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IffCode { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IffMaster { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::IffMasterValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IffMicIdent { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::IffMicIdentValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IffMode1Wheel1 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::IffMode1Wheel1Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IffMode1Wheel2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::IffMode1Wheel2Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IffMode3aWheel1 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::IffMode3aWheel1Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IffMode3aWheel2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::IffMode3aWheel2Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IffMode3aWheel3 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::IffMode3aWheel3Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IffMode3aWheel4 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::IffMode3aWheel4Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IffOnOut { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <iff::IffOnOutValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::IffOutAudioLight { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::IffOutAudioLightValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IffRadtest { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::IffRadtestValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IffReplyDim { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IffReplyInd { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::IffReplyTest { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::IffReplyTestValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IffTestDim { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::IffTestInd { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::IffTestM1 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::IffTestM1Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IffTestM2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::IffTestM2Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IffTestM3 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::IffTestM3Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IffTestM4 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::IffTestM4Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IffTestTest { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <iff::IffTestTestValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IntIntSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <intercompanel::IntIntSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IntMode { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <intercompanel::IntModeValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IntNavSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <intercompanel::IntNavSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IntRcvr1Sw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <intercompanel::IntRcvr1SwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IntRcvr2Sw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <intercompanel::IntRcvr2SwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IntRcvr3Sw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <intercompanel::IntRcvr3SwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IntRecvr4Sw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <intercompanel::IntRecvr4SwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IntVol { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::InverterSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <electricalsystem::InverterSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::JtsnBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <armamentpanel::JtsnBtnValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::JtsnCover { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <armamentpanel::JtsnCoverValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::LdgLightSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <collective::LdgLightSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::LdgLightSw2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <collective::LdgLightSw2Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::LdgLtCtrl { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <collective::LdgLtCtrlValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LdgLtCtrl2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <collective::LdgLtCtrl2Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::LowRpmAudio { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <enginepanel::LowRpmAudioValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::LowRpmInd { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::MainFuelSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <fuelsystem::MainFuelSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MainGenCover { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <overheadpanel::MainGenCoverValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::MainGenLoad { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MainGenSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <overheadpanel::MainGenSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MarkerBeacon { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MasterArmSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <armamentpanel::MasterArmSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MasterCautionInd { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::NavLtsSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <overheadpanel::NavLtsSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::NonEssBusSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <electricalsystem::NonEssBusSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::NvgPosLts { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <overheadpanel::NvgPosLtsValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::OadiOff { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::OadiPitch { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::OadiRoll { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Oalt100 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Oalt1000 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Oalt10000 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::OaltCodeOffFlag { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::OaltPress0 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::OaltPress1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::OaltPress2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::OaltPtr { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PadiOff { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PadiPitch { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PadiRoll { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Palt100 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Palt1000 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Palt10000 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PaltPress { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PitotHeat { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <overheadpanel::PitotHeatValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::PosLtsSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <overheadpanel::PosLtsSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RadarAltHi { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RadarAltLo { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RadarAltPwr { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <overheadpanel::RadarAltPwrValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RadarAltTest { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <frontdash::RadarAltTestValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RadioIcsSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <cyclic::RadioIcsSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RaltDigit1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RaltDigit2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RaltDigit3 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RaltDigit4 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RaltDisplay { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RaltDisplayStr { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RaltHiIdx { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RaltHiLamp { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RaltLoIdx { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RaltLoLamp { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RaltNeedle { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RaltOff { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RamTemp { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RmiCrs1 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RmiCrs2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RmiHdg { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RocketPair { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <armamentpanel::RocketPairValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RocketReset { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <armamentpanel::RocketResetValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RocketSel { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <armamentpanel::RocketSelValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RotorRpm { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SafeInd { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
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
            Self::SearchLightSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <collective::SearchLightSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::SearchLightSw2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <collective::SearchLightSw2Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Sideslip { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SightArmPlt { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <flexsight::SightArmPltValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::SightElevPlt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SightIntenCplt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SightIntenPlt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SightLampSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <flexsight::SightLampSwValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::SightPwrPlt { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <flexsight::SightPwrPltValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::SightsForCivil { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::StarterGenSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <electricalsystem::StarterGenSwValue as core::convert::TryFrom<u16>>::try_from(control.value()) { callback(value); }
                }
            }
            Self::StbyGenLoad { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TestFuelGaugeBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <fuelsystem::TestFuelGaugeBtnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Throttle { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Throttle2 { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ThrottleStop { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <collective::ThrottleStopValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::ThrottleStop2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <collective::ThrottleStop2Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::TorquePress { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TransOilPress { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TransOilTemp { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TrimCplt { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <cyclic::TrimCpltValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::TrimPlt { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <cyclic::TrimPltValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::TurnPtr { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UhfFreq { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UhfFunction { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <uhfradio::UhfFunctionValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfMode { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <uhfradio::UhfModeValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfPresetInt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UhfPresetStr { control, callback } => {
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
            Self::UhfVol { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VhfcommFreq { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VhfcommKhz { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VhfcommMhz { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VhfcommPwr { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <vhfcommradio::VhfcommPwrValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::VhfcommTestSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <vhfcommradio::VhfcommTestSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::VhfcommVol { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VhffmFreq1Int { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <vhffmradio::VhffmFreq1IntValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::VhffmFreq1Str { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VhffmFreq2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <vhffmradio::VhffmFreq2Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::VhffmFreq3 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <vhffmradio::VhffmFreq3Value as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::VhffmFreq4Int { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <vhffmradio::VhffmFreq4IntValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::VhffmFreq4Str { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VhffmMode { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <vhffmradio::VhffmModeValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::VhffmSquelch { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <vhffmradio::VhffmSquelchValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::VhffmVol { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VhfnavFreq { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VhfnavKhz { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VhfnavMhz { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VhfnavPwr { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <vhfnavradio::VhfnavPwrValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::VhfnavVol { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VviO { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VviP { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::WiperO { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::WiperP { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::WiperSel { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <overheadpanel::WiperSelValue as core::convert::TryFrom<
                        u16,
                    >>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::WiperSpd { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::X130Armed { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
        }
    }
}

pub struct Uh1hBuilder<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> {
    controls: Vec<RegisteredControl<'a>, MAX_CONTROLS>,
    _packet_capacity: PhantomData<ProtocolParser<MAX_PACKET_DATA>>,
}

impl<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize>
    Uh1hBuilder<'a, MAX_CONTROLS, MAX_PACKET_DATA>
{
    pub fn new() -> Self {
        Self {
            controls: Vec::new(),
            _packet_capacity: PhantomData,
        }
    }

    pub fn register_ac_gen1_current<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AcGen1Current {
                control: gauges::AcGen1Current::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ac_gen2_current<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AcGen2Current {
                control: gauges::AcGen2Current::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ac_v<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AcV {
                control: frontdash::AcV::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ac_vm_src<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electricalsystem::AcVmSrcValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AcVmSrc {
                control: electricalsystem::AcVmSrc::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ac_voltage<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AcVoltage {
                control: gauges::AcVoltage::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_band<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(adf::AdfBandValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfBand {
                control: adf::AdfBand::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_bfo_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(adf::AdfBfoSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfBfoSw {
                control: adf::AdfBfoSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_freq<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfFreq {
                control: adf::AdfFreq::new(),
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

    pub fn register_adf_loop_lr<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(adf::AdfLoopLrValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfLoopLr {
                control: adf::AdfLoopLr::new(),
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

    pub fn register_adf_signal<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfSignal {
                control: adf::AdfSignal::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_tune<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfTune {
                control: adf::AdfTune::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_vor_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(frontdash::AdfVorSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfVorSw {
                control: frontdash::AdfVorSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_pitch_shift<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiPitchShift {
                control: gauges::AdiPitchShift::new(),
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
                control: frontdash::AdiPitchTrim::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_pitch_trim_cplt<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiPitchTrimCplt {
                control: frontdash::AdiPitchTrimCplt::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_roll_trim<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiRollTrim {
                control: frontdash::AdiRollTrim::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_adj_cplt<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltAdjCplt {
                control: frontdash::AltAdjCplt::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_adj_plt<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltAdjPlt {
                control: frontdash::AltAdjPlt::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_anti_ice_amp<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AntiIceAmp {
                control: gauges::AntiIceAmp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_anticoll_lts_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(overheadpanel::AnticollLtsSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AnticollLtsSw {
                control: overheadpanel::AnticollLtsSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_armed_ind<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ArmedInd {
                control: armamentpanel::ArmedInd::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_bat_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electricalsystem::BatSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::BatSw {
                control: electricalsystem::BatSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_beacon_sensing_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(frontdash::BeaconSensingSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::BeaconSensingSw {
                control: frontdash::BeaconSensingSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_beacon_vol<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BeaconVol {
                control: frontdash::BeaconVol::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_bleed_air_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(overheadpanel::BleedAirSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::BleedAirSw {
                control: overheadpanel::BleedAirSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_brt_console<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BrtConsole {
                control: overheadpanel::BrtConsole::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_brt_copilot<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BrtCopilot {
                control: overheadpanel::BrtCopilot::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_brt_engine<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BrtEngine {
                control: overheadpanel::BrtEngine::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_brt_ped<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BrtPed {
                control: overheadpanel::BrtPed::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_brt_pilot<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BrtPilot {
                control: overheadpanel::BrtPilot::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_brt_sec<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BrtSec {
                control: overheadpanel::BrtSec::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cargo_rel_cp<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(cyclic::CargoRelCpValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CargoRelCp {
                control: cyclic::CargoRelCp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cargo_rel_p<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(cyclic::CargoRelPValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CargoRelP {
                control: cyclic::CargoRelP::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cargo_rel_safe<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(overheadpanel::CargoRelSafeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CargoRelSafe {
                control: overheadpanel::CargoRelSafe::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb28v<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::Cb28vValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Cb28v {
                control: circuitbreakers::Cb28v::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_air_valve<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbAirValveValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbAirValve {
                control: circuitbreakers::CbAirValve::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_am_radio<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbAmRadioValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbAmRadio {
                control: circuitbreakers::CbAmRadio::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_anti_ice<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbAntiIceValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbAntiIce {
                control: circuitbreakers::CbAntiIce::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_anticoll<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbAnticollValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbAnticoll {
                control: circuitbreakers::CbAnticoll::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_arc102<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbArc102Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbArc102 {
                control: circuitbreakers::CbArc102::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_cable<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbCableValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbCable {
                control: circuitbreakers::CbCable::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_caution_lts<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbCautionLtsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbCautionLts {
                control: circuitbreakers::CbCautionLts::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_cockpit_lts<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbCockpitLtsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbCockpitLts {
                control: circuitbreakers::CbCockpitLts::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_console_lts<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbConsoleLtsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbConsoleLts {
                control: circuitbreakers::CbConsoleLts::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_cplt_attd1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbCpltAttd1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbCpltAttd1 {
                control: circuitbreakers::CbCpltAttd1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_cplt_attd2<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbCpltAttd2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbCpltAttd2 {
                control: circuitbreakers::CbCpltAttd2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_crsind<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbCrsindValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbCrsind {
                control: circuitbreakers::CbCrsind::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_dome_lts<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbDomeLtsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbDomeLts {
                control: circuitbreakers::CbDomeLts::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_eng<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbEngValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbEng {
                control: circuitbreakers::CbEng::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_fail_relay<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbFailRelayValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbFailRelay {
                control: circuitbreakers::CbFailRelay::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_fire<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbFireValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbFire {
                control: circuitbreakers::CbFire::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_fm2_radio<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbFm2RadioValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbFm2Radio {
                control: circuitbreakers::CbFm2Radio::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_fm_radio<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbFmRadioValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbFmRadio {
                control: circuitbreakers::CbFmRadio::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_fuel<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbFuelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbFuel {
                control: circuitbreakers::CbFuel::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_fuel_qty<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbFuelQtyValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbFuelQty {
                control: circuitbreakers::CbFuelQty::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_fuel_trans<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbFuelTransValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbFuelTrans {
                control: circuitbreakers::CbFuelTrans::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_fuel_valves<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbFuelValvesValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbFuelValves {
                control: circuitbreakers::CbFuelValves::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_fus_lts<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbFusLtsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbFusLts {
                control: circuitbreakers::CbFusLts::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_gen_bus<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbGenBusValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbGenBus {
                control: circuitbreakers::CbGenBus::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_gov<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbGovValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbGov {
                control: circuitbreakers::CbGov::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_gyro<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbGyroValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbGyro {
                control: circuitbreakers::CbGyro::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_heated_blanket1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbHeatedBlanket1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbHeatedBlanket1 {
                control: circuitbreakers::CbHeatedBlanket1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_heated_blanket2<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbHeatedBlanket2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbHeatedBlanket2 {
                control: circuitbreakers::CbHeatedBlanket2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_hf_ant<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbHfAntValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbHfAnt {
                control: circuitbreakers::CbHfAnt::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_hf_arc102<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbHfArc102Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbHfArc102 {
                control: circuitbreakers::CbHfArc102::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_hook<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbHookValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbHook {
                control: circuitbreakers::CbHook::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_hydctl<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbHydctlValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbHydctl {
                control: circuitbreakers::CbHydctl::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_idle_stop<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbIdleStopValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbIdleStop {
                control: circuitbreakers::CbIdleStop::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_iff_apx1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbIffApx1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbIffApx1 {
                control: circuitbreakers::CbIffApx1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_iff_apx2<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbIffApx2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbIffApx2 {
                control: circuitbreakers::CbIffApx2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_ign<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbIgnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbIgn {
                control: circuitbreakers::CbIgn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_inst_lts<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbInstLtsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbInstLts {
                control: circuitbreakers::CbInstLts::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_inst_sec_lts<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbInstSecLtsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbInstSecLts {
                control: circuitbreakers::CbInstSecLts::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_int_cplt<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbIntCpltValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbIntCplt {
                control: circuitbreakers::CbIntCplt::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_int_plt<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbIntPltValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbIntPlt {
                control: circuitbreakers::CbIntPlt::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_invtr<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbInvtrValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbInvtr {
                control: circuitbreakers::CbInvtr::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_jtsn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbJtsnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbJtsn {
                control: circuitbreakers::CbJtsn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_ky20<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbKy20Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbKy20 {
                control: circuitbreakers::CbKy20::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_ldglts<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbLdgltsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbLdglts {
                control: circuitbreakers::CbLdglts::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_lf_nav<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbLfNavValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbLfNav {
                control: circuitbreakers::CbLfNav::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_lh_fuel<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbLhFuelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbLhFuel {
                control: circuitbreakers::CbLhFuel::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_main_invtr<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbMainInvtrValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbMainInvtr {
                control: circuitbreakers::CbMainInvtr::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_marker_beacon<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbMarkerBeaconValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbMarkerBeacon {
                control: circuitbreakers::CbMarkerBeacon::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_nav_lts<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbNavLtsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbNavLts {
                control: circuitbreakers::CbNavLts::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_outlet_valve<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbOutletValveValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbOutletValve {
                control: circuitbreakers::CbOutletValve::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_pitot_tube<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbPitotTubeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbPitotTube {
                control: circuitbreakers::CbPitotTube::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_plt_attd1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbPltAttd1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbPltAttd1 {
                control: circuitbreakers::CbPltAttd1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_plt_attd2<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbPltAttd2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbPltAttd2 {
                control: circuitbreakers::CbPltAttd2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_porx_warn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbPorxWarnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbPorxWarn {
                control: circuitbreakers::CbPorxWarn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_rescue_cont<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbRescueContValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbRescueCont {
                control: circuitbreakers::CbRescueCont::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_rescue_pwr<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbRescuePwrValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbRescuePwr {
                control: circuitbreakers::CbRescuePwr::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_rh_fuel<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbRhFuelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbRhFuel {
                control: circuitbreakers::CbRhFuel::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_rpm_warn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbRpmWarnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbRpmWarn {
                control: circuitbreakers::CbRpmWarn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_sl_ctl<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbSlCtlValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbSlCtl {
                control: circuitbreakers::CbSlCtl::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_sl_pwr<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbSlPwrValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbSlPwr {
                control: circuitbreakers::CbSlPwr::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_spare_invtr<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbSpareInvtrValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbSpareInvtr {
                control: circuitbreakers::CbSpareInvtr::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_starter<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbStarterValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbStarter {
                control: circuitbreakers::CbStarter::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_stby_gen<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbStbyGenValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbStbyGen {
                control: circuitbreakers::CbStbyGen::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_temp<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbTempValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbTemp {
                control: circuitbreakers::CbTemp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_torq<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbTorqValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbTorq {
                control: circuitbreakers::CbTorq::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_trim<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbTrimValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbTrim {
                control: circuitbreakers::CbTrim::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_turn_slip<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbTurnSlipValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbTurnSlip {
                control: circuitbreakers::CbTurnSlip::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_uhf_radio<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbUhfRadioValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbUhfRadio {
                control: circuitbreakers::CbUhfRadio::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_vhf_nav<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbVhfNavValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbVhfNav {
                control: circuitbreakers::CbVhfNav::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_vm<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbVmValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbVm {
                control: circuitbreakers::CbVm::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_wiper_cplt<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbWiperCpltValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbWiperCplt {
                control: circuitbreakers::CbWiperCplt::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_wiper_plt<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbWiperPltValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbWiperPlt {
                control: circuitbreakers::CbWiperPlt::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_xmsn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(circuitbreakers::CbXmsnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbXmsn {
                control: circuitbreakers::CbXmsn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_chip_det_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(centerconsole::ChipDetSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ChipDetSw {
                control: centerconsole::ChipDetSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ci_course_card<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CiCourseCard {
                control: courseindicator::CiCourseCard::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ci_from<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CiFrom {
                control: courseindicator::CiFrom::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ci_hbar<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CiHbar {
                control: courseindicator::CiHbar::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ci_hoff<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CiHoff {
                control: courseindicator::CiHoff::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ci_to<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CiTo {
                control: courseindicator::CiTo::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ci_vbar<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CiVbar {
                control: courseindicator::CiVbar::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ci_voff<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CiVoff {
                control: courseindicator::CiVoff::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_a0<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClA0 {
                control: cautionlightspanel::ClA0::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_a1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClA1 {
                control: cautionlightspanel::ClA1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_b0<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClB0 {
                control: cautionlightspanel::ClB0::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_b1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClB1 {
                control: cautionlightspanel::ClB1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_c0<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClC0 {
                control: cautionlightspanel::ClC0::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_c1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClC1 {
                control: cautionlightspanel::ClC1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_d0<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClD0 {
                control: cautionlightspanel::ClD0::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_d1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClD1 {
                control: cautionlightspanel::ClD1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_e0<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClE0 {
                control: cautionlightspanel::ClE0::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_e1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClE1 {
                control: cautionlightspanel::ClE1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_f0<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClF0 {
                control: cautionlightspanel::ClF0::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_f1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClF1 {
                control: cautionlightspanel::ClF1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_g0<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClG0 {
                control: cautionlightspanel::ClG0::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_g1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClG1 {
                control: cautionlightspanel::ClG1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_h0<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClH0 {
                control: cautionlightspanel::ClH0::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_h1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClH1 {
                control: cautionlightspanel::ClH1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_i0<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClI0 {
                control: cautionlightspanel::ClI0::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_i1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClI1 {
                control: cautionlightspanel::ClI1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_j0<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClJ0 {
                control: cautionlightspanel::ClJ0::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_j1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClJ1 {
                control: cautionlightspanel::ClJ1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_adjust<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockAdjust {
                control: clock::ClockAdjust::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_adjust_pull<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockAdjustPull {
                control: clock::ClockAdjustPull::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_h<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockH {
                control: frontdash::ClockH::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_m<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockM {
                control: frontdash::ClockM::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_s<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockS {
                control: frontdash::ClockS::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_bright_dim_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(cautionlightspanel::ClpBrightDimSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpBrightDimSw {
                control: cautionlightspanel::ClpBrightDimSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_reset_test_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(cautionlightspanel::ClpResetTestSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpResetTestSw {
                control: cautionlightspanel::ClpResetTestSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cm_arm_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(countermeasures::CmArmSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmArmSw {
                control: countermeasures::CmArmSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cm_armed_test<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(countermeasures::CmArmedTestValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmArmedTest {
                control: countermeasures::CmArmedTest::new(),
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
                control: countermeasures::CmChaffcntDisplay::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cm_chaffcnt_display_str<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmChaffcntDisplayStr {
                control: countermeasures::CmChaffcntDisplayStr::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cm_chaffcnt_reset<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(countermeasures::CmChaffcntResetValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmChaffcntReset {
                control: countermeasures::CmChaffcntReset::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cm_flare_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(countermeasures::CmFlareBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmFlareBtn {
                control: countermeasures::CmFlareBtn::new(),
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
                control: countermeasures::CmFlarecntDisplay::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cm_flarecnt_display_str<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmFlarecntDisplayStr {
                control: countermeasures::CmFlarecntDisplayStr::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cm_flarecnt_reset<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(countermeasures::CmFlarecntResetValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmFlarecntReset {
                control: countermeasures::CmFlarecntReset::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cm_man_pgrm_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(countermeasures::CmManPgrmSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmManPgrmSw {
                control: countermeasures::CmManPgrmSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cm_ripple_cover<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(countermeasures::CmRippleCoverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmRippleCover {
                control: countermeasures::CmRippleCover::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cm_ripple_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(countermeasures::CmRippleSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmRippleSw {
                control: countermeasures::CmRippleSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_console_light<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ConsoleLight {
                control: lights::ConsoleLight::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cplt_adi_cage<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(frontdash::CpltAdiCageValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CpltAdiCage {
                control: frontdash::CpltAdiCage::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_crs_knob<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CrsKnob {
                control: frontdash::CrsKnob::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dc_bat1_current<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DcBat1Current {
                control: gauges::DcBat1Current::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dc_bat2_current<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DcBat2Current {
                control: gauges::DcBat2Current::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dc_v<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DcV {
                control: frontdash::DcV::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dc_vm_src<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electricalsystem::DcVmSrcValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::DcVmSrc {
                control: electricalsystem::DcVmSrc::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dc_voltage<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DcVoltage {
                control: gauges::DcVoltage::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dc_vu1_current<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DcVu1Current {
                control: gauges::DcVu1Current::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dc_vu2_current<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DcVu2Current {
                control: gauges::DcVu2Current::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dc_vu3_current<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DcVu3Current {
                control: gauges::DcVu3Current::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dome_light<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DomeLight {
                control: lights::DomeLight::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dome_light_green<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DomeLightGreen {
                control: lights::DomeLightGreen::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dome_light_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(overheadpanel::DomeLightSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::DomeLightSw {
                control: overheadpanel::DomeLightSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_door_l_ptr<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(doors::DoorLPtrValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::DoorLPtr {
                control: doors::DoorLPtr::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_door_r_ptr<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(doors::DoorRPtrValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::DoorRPtr {
                control: doors::DoorRPtr::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_emer_gov_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginepanel::EmerGovSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EmerGovSw {
                control: enginepanel::EmerGovSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_eng_deice<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginepanel::EngDeiceValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngDeice {
                control: enginepanel::EngDeice::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_eng_oil_press<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngOilPress {
                control: frontdash::EngOilPress::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_eng_oil_temp<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngOilTemp {
                control: frontdash::EngOilTemp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_eng_rpm<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngRpm {
                control: frontdash::EngRpm::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_exhaust_temp<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExhaustTemp {
                control: frontdash::ExhaustTemp::new(),
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

    pub fn register_ext_wow_skid<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtWowSkid {
                control: externalaircraftmodel::ExtWowSkid::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fire_ind<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::FireInd {
                control: frontdash::FireInd::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fire_test_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(frontdash::FireTestBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FireTestBtn {
                control: frontdash::FireTestBtn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_force_trim_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(centerconsole::ForceTrimSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ForceTrimSw {
                control: centerconsole::ForceTrimSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_press<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelPress {
                control: frontdash::FuelPress::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_qty<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelQty {
                control: frontdash::FuelQty::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gas_rpm<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::GasRpm {
                control: frontdash::GasRpm::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gas_rpm_t<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::GasRpmT {
                control: frontdash::GasRpmT::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gmc_annunciator<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::GmcAnnunciator {
                control: gmcannunciator::GmcAnnunciator::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gmc_crs1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::GmcCrs1 {
                control: gmc::GmcCrs1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gmc_crs2<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::GmcCrs2 {
                control: gmc::GmcCrs2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gmc_hdg<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::GmcHdg {
                control: gmc::GmcHdg::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gmc_hdg_marker<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::GmcHdgMarker {
                control: gmc::GmcHdgMarker::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gmc_pwrfail<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::GmcPwrfail {
                control: gmcpowerfail::GmcPwrfail::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gov_rpm_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(collective::GovRpmSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::GovRpmSw {
                control: collective::GovRpmSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gov_rpm_sw2<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(collective::GovRpmSw2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::GovRpmSw2 {
                control: collective::GovRpmSw2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gun_sel<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(armamentpanel::GunSelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::GunSel {
                control: armamentpanel::GunSel::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gyro_mode_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(frontdash::GyroModeSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::GyroModeSw {
                control: frontdash::GyroModeSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hdg_set<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HdgSet {
                control: frontdash::HdgSet::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hdg_sync<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HdgSync {
                control: frontdash::HdgSync::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hyd_cont_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(centerconsole::HydContSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HydContSw {
                control: centerconsole::HydContSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ias_nose<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IasNose {
                control: gauges::IasNose::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ias_roof<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IasRoof {
                control: gauges::IasRoof::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_code<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffCode {
                control: iff::IffCode::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_master<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::IffMasterValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffMaster {
                control: iff::IffMaster::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_mic_ident<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::IffMicIdentValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffMicIdent {
                control: iff::IffMicIdent::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_mode1_wheel1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::IffMode1Wheel1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffMode1Wheel1 {
                control: iff::IffMode1Wheel1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_mode1_wheel2<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::IffMode1Wheel2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffMode1Wheel2 {
                control: iff::IffMode1Wheel2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_mode3a_wheel1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::IffMode3aWheel1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffMode3aWheel1 {
                control: iff::IffMode3aWheel1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_mode3a_wheel2<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::IffMode3aWheel2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffMode3aWheel2 {
                control: iff::IffMode3aWheel2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_mode3a_wheel3<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::IffMode3aWheel3Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffMode3aWheel3 {
                control: iff::IffMode3aWheel3::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_mode3a_wheel4<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::IffMode3aWheel4Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffMode3aWheel4 {
                control: iff::IffMode3aWheel4::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_on_out<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::IffOnOutValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffOnOut {
                control: iff::IffOnOut::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_out_audio_light<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::IffOutAudioLightValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffOutAudioLight {
                control: iff::IffOutAudioLight::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_radtest<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::IffRadtestValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffRadtest {
                control: iff::IffRadtest::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_reply_dim<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffReplyDim {
                control: iff::IffReplyDim::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_reply_ind<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffReplyInd {
                control: iff::IffReplyInd::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_reply_test<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::IffReplyTestValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffReplyTest {
                control: iff::IffReplyTest::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_test_dim<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffTestDim {
                control: iff::IffTestDim::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_test_ind<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffTestInd {
                control: iff::IffTestInd::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_test_m1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::IffTestM1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffTestM1 {
                control: iff::IffTestM1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_test_m2<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::IffTestM2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffTestM2 {
                control: iff::IffTestM2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_test_m3<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::IffTestM3Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffTestM3 {
                control: iff::IffTestM3::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_test_m4<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::IffTestM4Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffTestM4 {
                control: iff::IffTestM4::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_test_test<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(iff::IffTestTestValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffTestTest {
                control: iff::IffTestTest::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_int_int_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(intercompanel::IntIntSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IntIntSw {
                control: intercompanel::IntIntSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_int_mode<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(intercompanel::IntModeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IntMode {
                control: intercompanel::IntMode::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_int_nav_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(intercompanel::IntNavSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IntNavSw {
                control: intercompanel::IntNavSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_int_rcvr1_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(intercompanel::IntRcvr1SwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IntRcvr1Sw {
                control: intercompanel::IntRcvr1Sw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_int_rcvr2_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(intercompanel::IntRcvr2SwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IntRcvr2Sw {
                control: intercompanel::IntRcvr2Sw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_int_rcvr3_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(intercompanel::IntRcvr3SwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IntRcvr3Sw {
                control: intercompanel::IntRcvr3Sw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_int_recvr4_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(intercompanel::IntRecvr4SwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IntRecvr4Sw {
                control: intercompanel::IntRecvr4Sw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_int_vol<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IntVol {
                control: intercompanel::IntVol::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_inverter_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electricalsystem::InverterSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::InverterSw {
                control: electricalsystem::InverterSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_jtsn_btn<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(armamentpanel::JtsnBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::JtsnBtn {
                control: armamentpanel::JtsnBtn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_jtsn_cover<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(armamentpanel::JtsnCoverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::JtsnCover {
                control: armamentpanel::JtsnCover::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ldg_light_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(collective::LdgLightSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LdgLightSw {
                control: collective::LdgLightSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ldg_light_sw2<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(collective::LdgLightSw2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::LdgLightSw2 {
                control: collective::LdgLightSw2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ldg_lt_ctrl<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(collective::LdgLtCtrlValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LdgLtCtrl {
                control: collective::LdgLtCtrl::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ldg_lt_ctrl2<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(collective::LdgLtCtrl2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::LdgLtCtrl2 {
                control: collective::LdgLtCtrl2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_low_rpm_audio<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(enginepanel::LowRpmAudioValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LowRpmAudio {
                control: enginepanel::LowRpmAudio::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_low_rpm_ind<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LowRpmInd {
                control: frontdash::LowRpmInd::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_main_fuel_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fuelsystem::MainFuelSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MainFuelSw {
                control: fuelsystem::MainFuelSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_main_gen_cover<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(overheadpanel::MainGenCoverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MainGenCover {
                control: overheadpanel::MainGenCover::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_main_gen_load<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MainGenLoad {
                control: frontdash::MainGenLoad::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_main_gen_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(overheadpanel::MainGenSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MainGenSw {
                control: overheadpanel::MainGenSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_marker_beacon<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MarkerBeacon {
                control: gauges::MarkerBeacon::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_master_arm_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(armamentpanel::MasterArmSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MasterArmSw {
                control: armamentpanel::MasterArmSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_master_caution_ind<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::MasterCautionInd {
                control: frontdash::MasterCautionInd::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nav_lts_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(overheadpanel::NavLtsSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::NavLtsSw {
                control: overheadpanel::NavLtsSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_non_ess_bus_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electricalsystem::NonEssBusSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::NonEssBusSw {
                control: electricalsystem::NonEssBusSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nvg_pos_lts<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(overheadpanel::NvgPosLtsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::NvgPosLts {
                control: overheadpanel::NvgPosLts::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oadi_off<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OadiOff {
                control: frontdashadioperator::OadiOff::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oadi_pitch<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OadiPitch {
                control: frontdashadioperator::OadiPitch::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oadi_roll<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OadiRoll {
                control: frontdashadioperator::OadiRoll::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oalt100<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Oalt100 {
                control: altimeteroperator::Oalt100::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oalt1000<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Oalt1000 {
                control: altimeteroperator::Oalt1000::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oalt10000<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Oalt10000 {
                control: altimeteroperator::Oalt10000::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oalt_code_off_flag<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OaltCodeOffFlag {
                control: altimeteroperator::OaltCodeOffFlag::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oalt_press0<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OaltPress0 {
                control: altimeteroperator::OaltPress0::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oalt_press1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OaltPress1 {
                control: altimeteroperator::OaltPress1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oalt_press2<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OaltPress2 {
                control: altimeteroperator::OaltPress2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oalt_ptr<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OaltPtr {
                control: altimeteroperator::OaltPtr::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_padi_off<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::PadiOff {
                control: frontdashadipilot::PadiOff::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_padi_pitch<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::PadiPitch {
                control: frontdashadipilot::PadiPitch::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_padi_roll<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::PadiRoll {
                control: frontdashadipilot::PadiRoll::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_palt100<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Palt100 {
                control: altimeterpilot::Palt100::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_palt1000<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Palt1000 {
                control: altimeterpilot::Palt1000::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_palt10000<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Palt10000 {
                control: altimeterpilot::Palt10000::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_palt_press<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::PaltPress {
                control: altimeterpilot::PaltPress::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pitot_heat<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(overheadpanel::PitotHeatValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::PitotHeat {
                control: overheadpanel::PitotHeat::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pos_lts_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(overheadpanel::PosLtsSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::PosLtsSw {
                control: overheadpanel::PosLtsSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_alt_hi<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarAltHi {
                control: frontdash::RadarAltHi::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_alt_lo<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarAltLo {
                control: frontdash::RadarAltLo::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_alt_pwr<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(overheadpanel::RadarAltPwrValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarAltPwr {
                control: overheadpanel::RadarAltPwr::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_alt_test<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(frontdash::RadarAltTestValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarAltTest {
                control: frontdash::RadarAltTest::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radio_ics_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(cyclic::RadioIcsSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadioIcsSw {
                control: cyclic::RadioIcsSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ralt_digit1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RaltDigit1 {
                control: radaraltimeter::RaltDigit1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ralt_digit2<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RaltDigit2 {
                control: radaraltimeter::RaltDigit2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ralt_digit3<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RaltDigit3 {
                control: radaraltimeter::RaltDigit3::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ralt_digit4<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RaltDigit4 {
                control: radaraltimeter::RaltDigit4::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ralt_display<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RaltDisplay {
                control: radaraltimeter::RaltDisplay::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ralt_display_str<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::RaltDisplayStr {
                control: radaraltimeter::RaltDisplayStr::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ralt_hi_idx<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RaltHiIdx {
                control: radaraltimeter::RaltHiIdx::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ralt_hi_lamp<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RaltHiLamp {
                control: radaraltimeter::RaltHiLamp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ralt_lo_idx<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RaltLoIdx {
                control: radaraltimeter::RaltLoIdx::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ralt_lo_lamp<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RaltLoLamp {
                control: radaraltimeter::RaltLoLamp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ralt_needle<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RaltNeedle {
                control: radaraltimeter::RaltNeedle::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ralt_off<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RaltOff {
                control: radaraltimeter::RaltOff::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ram_temp<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RamTemp {
                control: gauges::RamTemp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rmi_crs1<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RmiCrs1 {
                control: copilotrmi::RmiCrs1::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rmi_crs2<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RmiCrs2 {
                control: copilotrmi::RmiCrs2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rmi_hdg<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RmiHdg {
                control: copilotrmi::RmiHdg::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rocket_pair<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(armamentpanel::RocketPairValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RocketPair {
                control: armamentpanel::RocketPair::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rocket_reset<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(armamentpanel::RocketResetValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RocketReset {
                control: armamentpanel::RocketReset::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rocket_sel<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(armamentpanel::RocketSelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RocketSel {
                control: armamentpanel::RocketSel::new(),
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
                control: frontdash::RotorRpm::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_safe_ind<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::SafeInd {
                control: armamentpanel::SafeInd::new(),
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
                control: frontdash::SbyCompassBank::new(),
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
                control: frontdash::SbyCompassHdg::new(),
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
                control: frontdash::SbyCompassPitch::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_search_light_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(collective::SearchLightSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SearchLightSw {
                control: collective::SearchLightSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_search_light_sw2<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(collective::SearchLightSw2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::SearchLightSw2 {
                control: collective::SearchLightSw2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sideslip<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Sideslip {
                control: frontdash::Sideslip::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sight_arm_plt<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flexsight::SightArmPltValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SightArmPlt {
                control: flexsight::SightArmPlt::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sight_elev_plt<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SightElevPlt {
                control: flexsight::SightElevPlt::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sight_inten_cplt<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SightIntenCplt {
                control: flexsight::SightIntenCplt::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sight_inten_plt<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SightIntenPlt {
                control: flexsight::SightIntenPlt::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sight_lamp_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flexsight::SightLampSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SightLampSw {
                control: flexsight::SightLampSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sight_pwr_plt<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(flexsight::SightPwrPltValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SightPwrPlt {
                control: flexsight::SightPwrPlt::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sights_for_civil<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SightsForCivil {
                control: gauges::SightsForCivil::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_starter_gen_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(electricalsystem::StarterGenSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::StarterGenSw {
                control: electricalsystem::StarterGenSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_stby_gen_load<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::StbyGenLoad {
                control: frontdash::StbyGenLoad::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_test_fuel_gauge_btn<F>(
        &mut self,
        callback: F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(fuelsystem::TestFuelGaugeBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::TestFuelGaugeBtn {
                control: fuelsystem::TestFuelGaugeBtn::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_throttle<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Throttle {
                control: collective::Throttle::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_throttle2<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Throttle2 {
                control: collective::Throttle2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_throttle_stop<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(collective::ThrottleStopValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ThrottleStop {
                control: collective::ThrottleStop::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_throttle_stop2<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(collective::ThrottleStop2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::ThrottleStop2 {
                control: collective::ThrottleStop2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_torque_press<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TorquePress {
                control: frontdash::TorquePress::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_trans_oil_press<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TransOilPress {
                control: frontdash::TransOilPress::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_trans_oil_temp<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TransOilTemp {
                control: frontdash::TransOilTemp::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_trim_cplt<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(cyclic::TrimCpltValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::TrimCplt {
                control: cyclic::TrimCplt::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_trim_plt<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(cyclic::TrimPltValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::TrimPlt {
                control: cyclic::TrimPlt::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_turn_ptr<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TurnPtr {
                control: frontdash::TurnPtr::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_freq<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfFreq {
                control: uhfradio::UhfFreq::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_function<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhfradio::UhfFunctionValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfFunction {
                control: uhfradio::UhfFunction::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_mode<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(uhfradio::UhfModeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfMode {
                control: uhfradio::UhfMode::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_preset_int<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfPresetInt {
                control: uhfradio::UhfPresetInt::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_preset_str<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfPresetStr {
                control: uhfradio::UhfPresetStr::new(),
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

    pub fn register_vhfcomm_freq<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfcommFreq {
                control: vhfcommradio::VhfcommFreq::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhfcomm_khz<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfcommKhz {
                control: vhfcommradio::VhfcommKhz::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhfcomm_mhz<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfcommMhz {
                control: vhfcommradio::VhfcommMhz::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhfcomm_pwr<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(vhfcommradio::VhfcommPwrValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfcommPwr {
                control: vhfcommradio::VhfcommPwr::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhfcomm_test_sw<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(vhfcommradio::VhfcommTestSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfcommTestSw {
                control: vhfcommradio::VhfcommTestSw::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhfcomm_vol<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfcommVol {
                control: vhfcommradio::VhfcommVol::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhffm_freq1_int<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(vhffmradio::VhffmFreq1IntValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhffmFreq1Int {
                control: vhffmradio::VhffmFreq1Int::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhffm_freq1_str<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhffmFreq1Str {
                control: vhffmradio::VhffmFreq1Str::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhffm_freq2<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(vhffmradio::VhffmFreq2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhffmFreq2 {
                control: vhffmradio::VhffmFreq2::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhffm_freq3<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(vhffmradio::VhffmFreq3Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhffmFreq3 {
                control: vhffmradio::VhffmFreq3::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhffm_freq4_int<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(vhffmradio::VhffmFreq4IntValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhffmFreq4Int {
                control: vhffmradio::VhffmFreq4Int::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhffm_freq4_str<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhffmFreq4Str {
                control: vhffmradio::VhffmFreq4Str::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhffm_mode<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(vhffmradio::VhffmModeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhffmMode {
                control: vhffmradio::VhffmMode::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhffm_squelch<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(vhffmradio::VhffmSquelchValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhffmSquelch {
                control: vhffmradio::VhffmSquelch::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhffm_vol<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhffmVol {
                control: vhffmradio::VhffmVol::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhfnav_freq<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfnavFreq {
                control: vhfnavradio::VhfnavFreq::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhfnav_khz<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfnavKhz {
                control: vhfnavradio::VhfnavKhz::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhfnav_mhz<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfnavMhz {
                control: vhfnavradio::VhfnavMhz::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhfnav_pwr<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(vhfnavradio::VhfnavPwrValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfnavPwr {
                control: vhfnavradio::VhfnavPwr::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhfnav_vol<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfnavVol {
                control: vhfnavradio::VhfnavVol::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vvi_o<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VviO {
                control: frontdash::VviO::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vvi_p<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VviP {
                control: frontdash::VviP::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_wiper_o<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::WiperO {
                control: windshieldwipers::WiperO::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_wiper_p<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::WiperP {
                control: windshieldwipers::WiperP::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_wiper_sel<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(overheadpanel::WiperSelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::WiperSel {
                control: overheadpanel::WiperSel::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_wiper_spd<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::WiperSpd {
                control: overheadpanel::WiperSpd::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_x130_armed<F>(&mut self, callback: F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::X130Armed {
                control: armamentpanel::X130Armed::new(),
                callback: alloc::boxed::Box::new(callback),
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn build(self) -> Uh1h<'a, MAX_CONTROLS, MAX_PACKET_DATA> {
        Uh1h {
            parser: ProtocolParser::new(),
            controls: self.controls,
            saw_frame: false,
        }
    }
}

impl<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> Default
    for Uh1hBuilder<'a, MAX_CONTROLS, MAX_PACKET_DATA>
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

pub struct Uh1h<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> {
    parser: ProtocolParser<MAX_PACKET_DATA>,
    controls: Vec<RegisteredControl<'a>, MAX_CONTROLS>,
    saw_frame: bool,
}

impl<const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize>
    Uh1h<'_, MAX_CONTROLS, MAX_PACKET_DATA>
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
