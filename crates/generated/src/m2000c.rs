//! Generated from model-definitions/M-2000C.json.
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AaRadSttSelBtnValue {
    Select1,
    Select2,
}
pub type AaRadSttSelBtnValueCallback<'a> = &'a mut dyn FnMut(AaRadSttSelBtnValue);
impl core::convert::TryFrom<u16> for AaRadSttSelBtnValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - A/A Radar STT Selector Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct AaRadSttSelBtn(Selector<29440, 512, 9>);
impl AaRadSttSelBtn {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AaRadSttSelBtn {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AaRadSttSelBtn {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AarSwValue {
    Select1,
    Select2,
    Select3,
}
pub type AarSwValueCallback<'a> = &'a mut dyn FnMut(AarSwValue);
impl core::convert::TryFrom<u16> for AarSwValue {
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

/// I - Air Refuel Transfer Switch
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct AarSw(Selector<29276, 384, 7>);
impl AarSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AarSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AarSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AbEmerCutoffSwValue {
    Select1,
    Select2,
}
pub type AbEmerCutoffSwValueCallback<'a> = &'a mut dyn FnMut(AbEmerCutoffSwValue);
impl core::convert::TryFrom<u16> for AbEmerCutoffSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - A/B Emergency Cutoff Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct AbEmerCutoffSw(Selector<29352, 4, 2>);
impl AbEmerCutoffSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AbEmerCutoffSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AbEmerCutoffSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AbEmerCutoffSwCovValue {
    Select1,
    Select2,
}
pub type AbEmerCutoffSwCovValueCallback<'a> = &'a mut dyn FnMut(AbEmerCutoffSwCovValue);
impl core::convert::TryFrom<u16> for AbEmerCutoffSwCovValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - A/B Emergency Cutoff Switch Cover
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct AbEmerCutoffSwCov(Selector<29352, 2, 1>);
impl AbEmerCutoffSwCov {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AbEmerCutoffSwCov {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AbEmerCutoffSwCov {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - ADI - Bille
/// Output: gauge position
pub struct AdiBille(AnalogGauge<29200, 65535, 0>);
impl AdiBille {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AdiBille {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AdiBille {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdiBklSwValue {
    Select1,
    Select2,
}
pub type AdiBklSwValueCallback<'a> = &'a mut dyn FnMut(AdiBklSwValue);
impl core::convert::TryFrom<u16> for AdiBklSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - ADI - Backlight Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct AdiBklSw(Selector<29184, 2, 1>);
impl AdiBklSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AdiBklSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AdiBklSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdiCageLevValue {
    Select1,
    Select2,
}
pub type AdiCageLevValueCallback<'a> = &'a mut dyn FnMut(AdiCageLevValue);
impl core::convert::TryFrom<u16> for AdiCageLevValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - ADI - Cage Lever
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct AdiCageLev(Selector<29184, 1, 0>);
impl AdiCageLev {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AdiCageLev {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AdiCageLev {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - ADI - Compass Position
/// Output: gauge position
pub struct AdiCompas(AnalogGauge<29190, 65535, 0>);
impl AdiCompas {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AdiCompas {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AdiCompas {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - ADI - Flag
/// Output: gauge position
pub struct AdiFlag(AnalogGauge<29192, 65535, 0>);
impl AdiFlag {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AdiFlag {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AdiFlag {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - ADI - Horizontal ILS Needle
/// Output: gauge position
pub struct AdiIlsH(AnalogGauge<29198, 65535, 0>);
impl AdiIlsH {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AdiIlsH {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AdiIlsH {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - ADI - Vertical ILS Needle
/// Output: gauge position
pub struct AdiIlsV(AnalogGauge<29196, 65535, 0>);
impl AdiIlsV {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AdiIlsV {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AdiIlsV {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - ADI - Voyant Ambre sous
/// Output: gauge position
pub struct AdiLight(AnalogGauge<29194, 65535, 0>);
impl AdiLight {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AdiLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AdiLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - ADI - Pitch Position
/// Output: gauge position
pub struct AdiPitch(AnalogGauge<29186, 65535, 0>);
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

/// O - ADI - Roll Position
/// Output: gauge position
pub struct AdiRoll(AnalogGauge<29188, 65535, 0>);
impl AdiRoll {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AdiRoll {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AdiRoll {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AgGunRetSwValue {
    Select1,
    Select2,
}
pub type AgGunRetSwValueCallback<'a> = &'a mut dyn FnMut(AgGunRetSwValue);
impl core::convert::TryFrom<u16> for AgGunRetSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - HUD - A/G Gun Reticle Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct AgGunRetSw(Selector<29492, 32768, 15>);
impl AgGunRetSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AgGunRetSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AgGunRetSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AgRadAModeSwValue {
    Select1,
    Select2,
}
pub type AgRadAModeSwValueCallback<'a> = &'a mut dyn FnMut(AgRadAModeSwValue);
impl core::convert::TryFrom<u16> for AgRadAModeSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - A/G Radar A Mode Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct AgRadAModeSw(Selector<29438, 4096, 12>);
impl AgRadAModeSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AgRadAModeSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AgRadAModeSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AgRadDecModeSwValue {
    Select1,
    Select2,
}
pub type AgRadDecModeSwValueCallback<'a> = &'a mut dyn FnMut(AgRadDecModeSwValue);
impl core::convert::TryFrom<u16> for AgRadDecModeSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - A/G Radar DEC Mode Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct AgRadDecModeSw(Selector<29438, 8192, 13>);
impl AgRadDecModeSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AgRadDecModeSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AgRadDecModeSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AgRadVisuModeSwValue {
    Select1,
    Select2,
}
pub type AgRadVisuModeSwValueCallback<'a> = &'a mut dyn FnMut(AgRadVisuModeSwValue);
impl core::convert::TryFrom<u16> for AgRadVisuModeSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - A/G Radar VISU Mode Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct AgRadVisuModeSw(Selector<29438, 16384, 14>);
impl AgRadVisuModeSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AgRadVisuModeSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AgRadVisuModeSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Air Refueling Light (yellow)
/// Output: 0 if light is off, 1 if light is on
pub struct AirRefuelL(Led<29500, 32768, 15>);
impl AirRefuelL {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for AirRefuelL {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AirRefuelL {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alt100FtSelValue {
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
pub type Alt100FtSelValueCallback<'a> = &'a mut dyn FnMut(Alt100FtSelValue);
impl core::convert::TryFrom<u16> for Alt100FtSelValue {
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

/// I - Altitude 100 ft Selector
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct Alt100FtSel(Selector<29238, 240, 4>);
impl Alt100FtSel {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Alt100FtSel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Alt100FtSel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alt10kFtSelValue {
    Select1,
    Select2,
    Select3,
    Select4,
    Select5,
    Select6,
}
pub type Alt10kFtSelValueCallback<'a> = &'a mut dyn FnMut(Alt10kFtSelValue);
impl core::convert::TryFrom<u16> for Alt10kFtSelValue {
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

/// I - BCD - Altitude 10000 ft Selector
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct Alt10kFtSel(Selector<29184, 14336, 11>);
impl Alt10kFtSel {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Alt10kFtSel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Alt10kFtSel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alt1SwValue {
    Select1,
    Select2,
}
pub type Alt1SwValueCallback<'a> = &'a mut dyn FnMut(Alt1SwValue);
impl core::convert::TryFrom<u16> for Alt1SwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Alternator 1 Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Alt1Sw(Selector<29252, 8192, 13>);
impl Alt1Sw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Alt1Sw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Alt1Sw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alt1kFtSelValue {
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
pub type Alt1kFtSelValueCallback<'a> = &'a mut dyn FnMut(Alt1kFtSelValue);
impl core::convert::TryFrom<u16> for Alt1kFtSelValue {
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

/// I - Altitude 1000 ft Selector
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct Alt1kFtSel(Selector<29238, 15, 0>);
impl Alt1kFtSel {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Alt1kFtSel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Alt1kFtSel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alt2SwValue {
    Select1,
    Select2,
}
pub type Alt2SwValueCallback<'a> = &'a mut dyn FnMut(Alt2SwValue);
impl core::convert::TryFrom<u16> for Alt2SwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Alternator 2 Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Alt2Sw(Selector<29252, 16384, 14>);
impl Alt2Sw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Alt2Sw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Alt2Sw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - ALT - x100 Baro Press display
/// Output: gauge position
pub struct AltBaroCents(AnalogGauge<29214, 65535, 0>);
impl AltBaroCents {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AltBaroCents {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AltBaroCents {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - ALT - x1 Baro Press display
/// Output: gauge position
pub struct AltBaroOnes(AnalogGauge<29218, 65535, 0>);
impl AltBaroOnes {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AltBaroOnes {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AltBaroOnes {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - ALT - x10 Baro Press display
/// Output: gauge position
pub struct AltBaroTens(AnalogGauge<29216, 65535, 0>);
impl AltBaroTens {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AltBaroTens {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AltBaroTens {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - ALT - x1000 Baro Press display
/// Output: gauge position
pub struct AltBaroThous(AnalogGauge<29212, 65535, 0>);
impl AltBaroThous {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AltBaroThous {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AltBaroThous {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - ALT - x100 ft display
/// Output: gauge position
pub struct AltCents(AnalogGauge<29210, 65535, 0>);
impl AltCents {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AltCents {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AltCents {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AltHoldBtnValue {
    Select1,
    Select2,
}
pub type AltHoldBtnValueCallback<'a> = &'a mut dyn FnMut(AltHoldBtnValue);
impl core::convert::TryFrom<u16> for AltHoldBtnValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Altitude Hold Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct AltHoldBtn(Selector<29184, 256, 8>);
impl AltHoldBtn {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AltHoldBtn {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AltHoldBtn {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - ALT - x10 000 ft display
/// Output: gauge position
pub struct AltKcents(AnalogGauge<29206, 65535, 0>);
impl AltKcents {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AltKcents {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AltKcents {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - ALT - Needle
/// Output: gauge position
pub struct AltNeedle(AnalogGauge<29204, 65535, 0>);
impl AltNeedle {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AltNeedle {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AltNeedle {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - ALT - x1000 ft display
/// Output: gauge position
pub struct AltThous(AnalogGauge<29208, 65535, 0>);
impl AltThous {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AltThous {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AltThous {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AmplisSelKnobValue {
    Select1,
    Select2,
}
pub type AmplisSelKnobValueCallback<'a> = &'a mut dyn FnMut(AmplisSelKnobValue);
impl core::convert::TryFrom<u16> for AmplisSelKnobValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - AMPLIS Selector Knob
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct AmplisSelKnob(Selector<29184, 64, 6>);
impl AmplisSelKnob {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AmplisSelKnob {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AmplisSelKnob {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AntiColLgtSwValue {
    Select1,
    Select2,
}
pub type AntiColLgtSwValueCallback<'a> = &'a mut dyn FnMut(AntiColLgtSwValue);
impl core::convert::TryFrom<u16> for AntiColLgtSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Anti-Collision Lights Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct AntiColLgtSw(Selector<29276, 4, 2>);
impl AntiColLgtSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AntiColLgtSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AntiColLgtSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AntiSkidSwValue {
    Select1,
    Select2,
}
pub type AntiSkidSwValueCallback<'a> = &'a mut dyn FnMut(AntiSkidSwValue);
impl core::convert::TryFrom<u16> for AntiSkidSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Anti-Skid Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct AntiSkidSw(Selector<29352, 1024, 10>);
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
pub enum AntiSkidSwCovValue {
    Select1,
    Select2,
}
pub type AntiSkidSwCovValueCallback<'a> = &'a mut dyn FnMut(AntiSkidSwCovValue);
impl core::convert::TryFrom<u16> for AntiSkidSwCovValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Anti-Skid Switch Cover
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct AntiSkidSwCov(Selector<29352, 512, 9>);
impl AntiSkidSwCov {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AntiSkidSwCov {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AntiSkidSwCov {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - AOA - Flag
/// Output: gauge position
pub struct AoaFlag(AnalogGauge<29222, 65535, 0>);
impl AoaFlag {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AoaFlag {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AoaFlag {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - AOA - Indicator
/// Output: gauge position
pub struct AoaPos(AnalogGauge<29220, 65535, 0>);
impl AoaPos {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AoaPos {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AoaPos {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - AP - Alt Hold Amber Light
/// Output: 0 if light is off, 1 if light is on
pub struct ApAltAmbre(Led<29238, 1024, 10>);
impl ApAltAmbre {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ApAltAmbre {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ApAltAmbre {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - AP - Alt Hold Green Light
/// Output: 0 if light is off, 1 if light is on
pub struct ApAltVert(Led<29238, 512, 9>);
impl ApAltVert {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ApAltVert {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ApAltVert {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - AP - G Amber Light
/// Output: 0 if light is off, 1 if light is on
pub struct ApGAmbre(Led<29240, 4, 2>);
impl ApGAmbre {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ApGAmbre {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ApGAmbre {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - AP - G Green Light
/// Output: 0 if light is off, 1 if light is on
pub struct ApGVert(Led<29240, 2, 1>);
impl ApGVert {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ApGVert {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ApGVert {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - AP - L Amber Light
/// Output: 0 if light is off, 1 if light is on
pub struct ApLAmbre(Led<29240, 1, 0>);
impl ApLAmbre {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ApLAmbre {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ApLAmbre {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - AP - L Green Light
/// Output: 0 if light is off, 1 if light is on
pub struct ApLVert(Led<29238, 32768, 15>);
impl ApLVert {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ApLVert {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ApLVert {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApLgtTestBtnValue {
    Select1,
    Select2,
}
pub type ApLgtTestBtnValueCallback<'a> = &'a mut dyn FnMut(ApLgtTestBtnValue);
impl core::convert::TryFrom<u16> for ApLgtTestBtnValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Autopilot Lights Test Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct ApLgtTestBtn(Selector<29184, 16384, 14>);
impl ApLgtTestBtn {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ApLgtTestBtn {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ApLgtTestBtn {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - AP - Master Amber Light
/// Output: 0 if light is off, 1 if light is on
pub struct ApMasterAmbre(Led<29184, 32768, 15>);
impl ApMasterAmbre {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ApMasterAmbre {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ApMasterAmbre {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApMasterBtnValue {
    Select1,
    Select2,
}
pub type ApMasterBtnValueCallback<'a> = &'a mut dyn FnMut(ApMasterBtnValue);
impl core::convert::TryFrom<u16> for ApMasterBtnValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Autopilot Master Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct ApMasterBtn(Selector<29184, 128, 7>);
impl ApMasterBtn {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ApMasterBtn {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ApMasterBtn {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - AP - Master Green Light
/// Output: 0 if light is off, 1 if light is on
pub struct ApMasterVert(Led<29238, 256, 8>);
impl ApMasterVert {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ApMasterVert {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ApMasterVert {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - AP - Free Amber Light
/// Output: 0 if light is off, 1 if light is on
pub struct ApNcAmbre(Led<29238, 16384, 14>);
impl ApNcAmbre {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ApNcAmbre {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ApNcAmbre {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - AP - Free Green Light
/// Output: 0 if light is off, 1 if light is on
pub struct ApNcVert(Led<29238, 8192, 13>);
impl ApNcVert {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ApNcVert {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ApNcVert {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - AP - Sel Alt Hold Amber Light
/// Output: 0 if light is off, 1 if light is on
pub struct ApSelectAltAmbre(Led<29238, 4096, 12>);
impl ApSelectAltAmbre {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ApSelectAltAmbre {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ApSelectAltAmbre {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - AP - Sel Alt Hold Green Light
/// Output: 0 if light is off, 1 if light is on
pub struct ApSelectAltVert(Led<29238, 2048, 11>);
impl ApSelectAltVert {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ApSelectAltVert {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ApSelectAltVert {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApTestSwValue {
    Select1,
    Select2,
}
pub type ApTestSwValueCallback<'a> = &'a mut dyn FnMut(ApTestSwValue);
impl core::convert::TryFrom<u16> for ApTestSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - TEST - Autopilot Test Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct ApTestSw(Selector<29468, 1024, 10>);
impl ApTestSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ApTestSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ApTestSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ApTestSwCovValue {
    Select1,
    Select2,
}
pub type ApTestSwCovValueCallback<'a> = &'a mut dyn FnMut(ApTestSwCovValue);
impl core::convert::TryFrom<u16> for ApTestSwCovValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - TEST - Autopilot Test Switch Cover
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct ApTestSwCov(Selector<29468, 512, 9>);
impl ApTestSwCov {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ApTestSwCov {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ApTestSwCov {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AppHoldBtnValue {
    Select1,
    Select2,
}
pub type AppHoldBtnValueCallback<'a> = &'a mut dyn FnMut(AppHoldBtnValue);
impl core::convert::TryFrom<u16> for AppHoldBtnValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Approach Hold Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct AppHoldBtn(Selector<29184, 1024, 10>);
impl AppHoldBtn {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AppHoldBtn {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AppHoldBtn {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioWarnResetValue {
    Select1,
    Select2,
}
pub type AudioWarnResetValueCallback<'a> = &'a mut dyn FnMut(AudioWarnResetValue);
impl core::convert::TryFrom<u16> for AudioWarnResetValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - AL - Audio Warning Reset Button (PANNEL)
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct AudioWarnReset(Selector<29354, 1024, 10>);
impl AudioWarnReset {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AudioWarnReset {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AudioWarnReset {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioWarnSwValue {
    Select1,
    Select2,
}
pub type AudioWarnSwValueCallback<'a> = &'a mut dyn FnMut(AudioWarnSwValue);
impl core::convert::TryFrom<u16> for AudioWarnSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - AL - Audio Warning Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct AudioWarnSw(Selector<29442, 4096, 12>);
impl AudioWarnSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AudioWarnSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AudioWarnSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AutoManBtnValue {
    Select1,
    Select2,
}
pub type AutoManBtnValueCallback<'a> = &'a mut dyn FnMut(AutoManBtnValue);
impl core::convert::TryFrom<u16> for AutoManBtnValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PPA - AUTO/MAN Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct AutoManBtn(Selector<29400, 32768, 15>);
impl AutoManBtn {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AutoManBtn {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AutoManBtn {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuxGunsightValue {
    Select1,
    Select2,
}
pub type AuxGunsightValueCallback<'a> = &'a mut dyn FnMut(AuxGunsightValue);
impl core::convert::TryFrom<u16> for AuxGunsightValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - HUD - Auxiliary Gunsight
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct AuxGunsight(Selector<29492, 16384, 14>);
impl AuxGunsight {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AuxGunsight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AuxGunsight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// I - HUD - Auxiliary Gunsight Deflection
/// Input: turn the dial left or right
/// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
pub struct AuxGunsightDefl(AnalogDial<29496, 65535, 0>);
impl AuxGunsightDefl {
    pub const fn new() -> Self {
        Self(AnalogDial::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for AuxGunsightDefl {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for AuxGunsightDefl {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// I - ALT - Barometric Pressure Calibration
/// Input: turn the dial left or right
/// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
pub struct BaroPressCalib(AnalogDial<29202, 65535, 0>);
impl BaroPressCalib {
    pub const fn new() -> Self {
        Self(AnalogDial::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for BaroPressCalib {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for BaroPressCalib {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BeWpSelValue {
    Select1,
    Select2,
    Select3,
}
pub type BeWpSelValueCallback<'a> = &'a mut dyn FnMut(BeWpSelValue);
impl core::convert::TryFrom<u16> for BeWpSelValue {
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

/// I - VTB - Bullseye Waypoint Selector (N)
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct BeWpSel(Selector<29492, 3, 0>);
impl BeWpSel {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for BeWpSel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for BeWpSel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BingoFuel100KgSelValue {
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
pub type BingoFuel100KgSelValueCallback<'a> = &'a mut dyn FnMut(BingoFuel100KgSelValue);
impl core::convert::TryFrom<u16> for BingoFuel100KgSelValue {
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

/// I - Bingo Fuel 100 kg Selector
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct BingoFuel100KgSel(Selector<29280, 15, 0>);
impl BingoFuel100KgSel {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for BingoFuel100KgSel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for BingoFuel100KgSel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BingoFuel1kKgSelValue {
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
pub type BingoFuel1kKgSelValueCallback<'a> = &'a mut dyn FnMut(BingoFuel1kKgSelValue);
impl core::convert::TryFrom<u16> for BingoFuel1kKgSelValue {
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

/// I - Bingo Fuel 1000 kg Selector
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct BingoFuel1kKgSel(Selector<29276, 30720, 11>);
impl BingoFuel1kKgSel {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for BingoFuel1kKgSel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for BingoFuel1kKgSel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BombFuzeSelSwValue {
    Select1,
    Select2,
    Select3,
}
pub type BombFuzeSelSwValueCallback<'a> = &'a mut dyn FnMut(BombFuzeSelSwValue);
impl core::convert::TryFrom<u16> for BombFuzeSelSwValue {
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

/// I - PPA - Bomb Fuze Selector Switch
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct BombFuzeSelSw(Selector<29400, 12288, 12>);
impl BombFuzeSelSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for BombFuzeSelSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for BombFuzeSelSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BombRelIntSelSwValue {
    Select1,
    Select2,
    Select3,
}
pub type BombRelIntSelSwValueCallback<'a> = &'a mut dyn FnMut(BombRelIntSelSwValue);
impl core::convert::TryFrom<u16> for BombRelIntSelSwValue {
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

/// I - PPA - Bomb Release Interval Selector Switch
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct BombRelIntSelSw(Selector<29408, 768, 8>);
impl BombRelIntSelSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for BombRelIntSelSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for BombRelIntSelSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BombRelQtySelSwValue {
    Select1,
    Select2,
    Select3,
}
pub type BombRelQtySelSwValueCallback<'a> = &'a mut dyn FnMut(BombRelQtySelSwValue);
impl core::convert::TryFrom<u16> for BombRelQtySelSwValue {
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

/// I - PPA - Bomb Release Quantity Selector Switch
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct BombRelQtySelSw(Selector<29404, 24576, 13>);
impl BombRelQtySelSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for BombRelQtySelSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for BombRelQtySelSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrPwrSwValue {
    Select1,
    Select2,
    Select3,
}
pub type BrPwrSwValueCallback<'a> = &'a mut dyn FnMut(BrPwrSwValue);
impl core::convert::TryFrom<u16> for BrPwrSwValue {
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

/// I - BR Power Switch
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct BrPwrSw(Selector<29274, 24, 3>);
impl BrPwrSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for BrPwrSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for BrPwrSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// I - Caution/Advisory Lights Rheostat
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct CautAdvLgt(LimitedDial<29346, 65535, 0>);
impl CautAdvLgt {
    pub const fn new() -> Self {
        Self(LimitedDial::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CautAdvLgt {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CautAdvLgt {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - ECS - C Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClimC(Led<29262, 128, 7>);
impl ClimC {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClimC {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClimC {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - ECS - F Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClimF(Led<29262, 256, 8>);
impl ClimF {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClimF {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClimF {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLOCK - Hours Needle
/// Output: gauge position
pub struct ClkH(AnalogGauge<29256, 65535, 0>);
impl ClkH {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ClkH {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClkH {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLOCK - Minutes Needle
/// Output: gauge position
pub struct ClkM(AnalogGauge<29258, 65535, 0>);
impl ClkM {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ClkM {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClkM {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLOCK - Seconds Needle
/// Output: gauge position
pub struct ClkS(AnalogGauge<29260, 65535, 0>);
impl ClkS {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ClkS {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClkS {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - 5mn O2 Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct Clp5mnO2(Led<29250, 1024, 10>);
impl Clp5mnO2 {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Clp5mnO2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Clp5mnO2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - ALT.1 Indicator Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpAlt1(Led<29240, 64, 6>);
impl ClpAlt1 {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpAlt1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpAlt1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - ALT.2 Indicator Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpAlt2(Led<29240, 128, 7>);
impl ClpAlt2 {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpAlt2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpAlt2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - ANEMO Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpAnemo(Led<29250, 4096, 12>);
impl ClpAnemo {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpAnemo {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpAnemo {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - BATT Indicator Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpBatt(Led<29240, 16, 4>);
impl ClpBatt {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpBatt {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpBatt {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - BECS Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpBecs(Led<29252, 16, 4>);
impl ClpBecs {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpBecs {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpBecs {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - BINGO Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpBingo(Led<29250, 64, 6>);
impl ClpBingo {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpBingo {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpBingo {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - BP Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpBp(Led<29240, 8192, 13>);
impl ClpBp {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpBp {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpBp {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - BP D Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpBpD(Led<29240, 32768, 15>);
impl ClpBpD {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpBpD {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpBpD {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - BP G Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpBpG(Led<29240, 16384, 14>);
impl ClpBpG {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpBpG {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpBpG {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - CALC Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpCalc(Led<29240, 1024, 10>);
impl ClpCalc {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpCalc {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpCalc {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - CC Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpCc(Led<29250, 8192, 13>);
impl ClpCc {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpCc {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpCc {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - CONDIT Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpCondit(Led<29250, 32768, 15>);
impl ClpCondit {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpCondit {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpCondit {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - CONF Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpConf(Led<29252, 1, 0>);
impl ClpConf {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpConf {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpConf {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - DECOL Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpDecol(Led<29252, 512, 9>);
impl ClpDecol {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpDecol {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpDecol {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - DOM Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpDom(Led<29252, 8, 3>);
impl ClpDom {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpDom {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpDom {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - DSV Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpDsv(Led<29250, 16384, 14>);
impl ClpDsv {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpDsv {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpDsv {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - EP Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpEp(Led<29250, 32, 5>);
impl ClpEp {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpEp {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpEp {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - GAIN Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpGain(Led<29252, 128, 7>);
impl ClpGain {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpGain {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpGain {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - HUILE Indicator Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpHuile(Led<29240, 256, 8>);
impl ClpHuile {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpHuile {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpHuile {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - HYD.1 Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpHyd1(Led<29250, 4, 2>);
impl ClpHyd1 {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpHyd1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpHyd1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - HYD.2 Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpHyd2(Led<29250, 8, 3>);
impl ClpHyd2 {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpHyd2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpHyd2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - HYD.S Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpHydS(Led<29250, 16, 4>);
impl ClpHydS {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpHydS {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpHydS {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - INCIDENCE Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpIncidence(Led<29252, 64, 6>);
impl ClpIncidence {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpIncidence {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpIncidence {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - MAN Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpMan(Led<29252, 4, 2>);
impl ClpMan {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpMan {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpMan {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - NIVEAU Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpNiveau(Led<29250, 2, 1>);
impl ClpNiveau {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpNiveau {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpNiveau {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - O2 HA Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpO2Ha(Led<29250, 2048, 11>);
impl ClpO2Ha {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpO2Ha {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpO2Ha {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - P.CAB Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpPCab(Led<29250, 128, 7>);
impl ClpPCab {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpPCab {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpPCab {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - PA Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpPa(Led<29252, 2, 1>);
impl ClpPa {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpPa {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpPa {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - PARK Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpPark(Led<29252, 1024, 10>);
impl ClpPark {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpPark {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpPark {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - PELLES Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpPelles(Led<29240, 4096, 12>);
impl ClpPelles {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpPelles {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpPelles {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - REG O2 Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpRegO2(Led<29250, 512, 9>);
impl ClpRegO2 {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpRegO2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpRegO2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - RPM Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpRpm(Led<29252, 256, 8>);
impl ClpRpm {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpRpm {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpRpm {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - SOURIS Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpSouris(Led<29240, 2048, 11>);
impl ClpSouris {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpSouris {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpSouris {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - T7 Indicator Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpT7(Led<29240, 512, 9>);
impl ClpT7 {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpT7 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpT7 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - TEMP Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpTemp(Led<29250, 256, 8>);
impl ClpTemp {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpTemp {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpTemp {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - TRANSF Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpTransf(Led<29250, 1, 0>);
impl ClpTransf {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpTransf {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpTransf {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - TR Indicator Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpTrn(Led<29240, 32, 5>);
impl ClpTrn {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpTrn {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpTrn {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CLP - U.S EL Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ClpUsEl(Led<29252, 32, 5>);
impl ClpUsEl {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ClpUsEl {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ClpUsEl {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CnpyHand1Value {
    Select1,
    Select2,
}
pub type CnpyHand1ValueCallback<'a> = &'a mut dyn FnMut(CnpyHand1Value);
impl core::convert::TryFrom<u16> for CnpyHand1Value {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Canopy Handle
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct CnpyHand1(Selector<29366, 256, 8>);
impl CnpyHand1 {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CnpyHand1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CnpyHand1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CnpyHand2Value {
    Select1,
    Select2,
}
pub type CnpyHand2ValueCallback<'a> = &'a mut dyn FnMut(CnpyHand2Value);
impl core::convert::TryFrom<u16> for CnpyHand2Value {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Canopy Handle
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct CnpyHand2(Selector<29366, 512, 9>);
impl CnpyHand2 {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CnpyHand2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CnpyHand2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CnpyJettValue {
    Select1,
    Select2,
}
pub type CnpyJettValueCallback<'a> = &'a mut dyn FnMut(CnpyJettValue);
impl core::convert::TryFrom<u16> for CnpyJettValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Canopy Jettison
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct CnpyJett(Selector<29366, 64, 6>);
impl CnpyJett {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CnpyJett {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CnpyJett {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CnpyLkNLowLevValue {
    Select1,
    Select2,
    Select3,
}
pub type CnpyLkNLowLevValueCallback<'a> = &'a mut dyn FnMut(CnpyLkNLowLevValue);
impl core::convert::TryFrom<u16> for CnpyLkNLowLevValue {
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

/// I - Canopy Lock/Neutral/Lower Lever
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct CnpyLkNLowLev(Selector<29366, 24, 3>);
impl CnpyLkNLowLev {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CnpyLkNLowLev {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CnpyLkNLowLev {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CnpyRestValue {
    Select1,
    Select2,
    Select3,
}
pub type CnpyRestValueCallback<'a> = &'a mut dyn FnMut(CnpyRestValue);
impl core::convert::TryFrom<u16> for CnpyRestValue {
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

/// I - Canopy Rest
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct CnpyRest(Selector<29366, 6144, 11>);
impl CnpyRest {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CnpyRest {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CnpyRest {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CocClockValue {
    Select1,
    Select2,
}
pub type CocClockValueCallback<'a> = &'a mut dyn FnMut(CocClockValue);
impl core::convert::TryFrom<u16> for CocClockValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - CLOCK - Cockpit Clock Position
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct CocClock(Selector<29252, 32768, 15>);
impl CocClock {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for CocClock {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for CocClock {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConesOpSwValue {
    Select1,
    Select2,
}
pub type ConesOpSwValueCallback<'a> = &'a mut dyn FnMut(ConesOpSwValue);
impl core::convert::TryFrom<u16> for ConesOpSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Intake Cones Operation Switch (SOURIS)
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct ConesOpSw(Selector<29184, 8, 3>);
impl ConesOpSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ConesOpSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ConesOpSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CONF - A Light
/// Output: 0 if light is off, 1 if light is on
pub struct ConfA(Led<29254, 4, 2>);
impl ConfA {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ConfA {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ConfA {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CONF - Aux Landing Gear Light
/// Output: 0 if light is off, 1 if light is on
pub struct ConfAux(Led<29254, 1024, 10>);
impl ConfAux {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ConfAux {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ConfAux {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CONF - CROSS Light
/// Output: 0 if light is off, 1 if light is on
pub struct ConfCross(Led<29254, 64, 6>);
impl ConfCross {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ConfCross {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ConfCross {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CONF - Right Landing Gear Light
/// Output: 0 if light is off, 1 if light is on
pub struct ConfD(Led<29254, 2048, 11>);
impl ConfD {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ConfD {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ConfD {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CONF - DIRAV Light
/// Output: 0 if light is off, 1 if light is on
pub struct ConfDirav(Led<29254, 16, 4>);
impl ConfDirav {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ConfDirav {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ConfDirav {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CONF - F Light
/// Output: 0 if light is off, 1 if light is on
pub struct ConfF(Led<29254, 8, 3>);
impl ConfF {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ConfF {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ConfF {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CONF - FREIN Light
/// Output: 0 if light is off, 1 if light is on
pub struct ConfFrein(Led<29254, 32, 5>);
impl ConfFrein {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ConfFrein {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ConfFrein {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CONF - Left Landing Gear Light
/// Output: 0 if light is off, 1 if light is on
pub struct ConfG(Led<29254, 512, 9>);
impl ConfG {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ConfG {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ConfG {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CONF - Red Landing Gear Light
/// Output: 0 if light is off, 1 if light is on
pub struct ConfRouge(Led<29254, 256, 8>);
impl ConfRouge {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ConfRouge {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ConfRouge {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CONF - SPAD Light
/// Output: 0 if light is off, 1 if light is on
pub struct ConfSpad(Led<29254, 128, 7>);
impl ConfSpad {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ConfSpad {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ConfSpad {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// I - Console Panel Lights Knob
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct ConsPanelLgtKnob(LimitedDial<29344, 65535, 0>);
impl ConsPanelLgtKnob {
    pub const fn new() -> Self {
        Self(LimitedDial::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ConsPanelLgtKnob {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ConsPanelLgtKnob {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum D2mPwrSwValue {
    Select1,
    Select2,
    Select3,
}
pub type D2mPwrSwValueCallback<'a> = &'a mut dyn FnMut(D2mPwrSwValue);
impl core::convert::TryFrom<u16> for D2mPwrSwValue {
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

/// I - D2M Power Switch
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct D2mPwrSw(Selector<29274, 384, 7>);
impl D2mPwrSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for D2mPwrSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for D2mPwrSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// I - Dashboard Panel Lights Knob
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct DashPanelLgtKnob(LimitedDial<29340, 65535, 0>);
impl DashPanelLgtKnob {
    pub const fn new() -> Self {
        Self(LimitedDial::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for DashPanelLgtKnob {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for DashPanelLgtKnob {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// I - Dashboard U.V. Lights Knob
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct DashUVLgtKnob(LimitedDial<29338, 65535, 0>);
impl DashUVLgtKnob {
    pub const fn new() -> Self {
        Self(LimitedDial::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for DashUVLgtKnob {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for DashUVLgtKnob {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecoyRelModeSwValue {
    Select1,
    Select2,
    Select3,
}
pub type DecoyRelModeSwValueCallback<'a> = &'a mut dyn FnMut(DecoyRelModeSwValue);
impl core::convert::TryFrom<u16> for DecoyRelModeSwValue {
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

/// I - Decoy Release Mode Switch
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct DecoyRelModeSw(Selector<29274, 1536, 9>);
impl DecoyRelModeSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for DecoyRelModeSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for DecoyRelModeSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DecoyRelPgmKnobValue {
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
pub type DecoyRelPgmKnobValueCallback<'a> = &'a mut dyn FnMut(DecoyRelPgmKnobValue);
impl core::convert::TryFrom<u16> for DecoyRelPgmKnobValue {
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

/// I - Decoy Release Program Knob
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct DecoyRelPgmKnob(Selector<29274, 30720, 11>);
impl DecoyRelPgmKnob {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for DecoyRelPgmKnob {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for DecoyRelPgmKnob {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - AL - DEMARRAGE Light
/// Output: 0 if light is off, 1 if light is on
pub struct Dem(Led<29366, 1, 0>);
impl Dem {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Dem {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Dem {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DragChuteLevValue {
    Select1,
    Select2,
}
pub type DragChuteLevValueCallback<'a> = &'a mut dyn FnMut(DragChuteLevValue);
impl core::convert::TryFrom<u16> for DragChuteLevValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Drag Chute Lever
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct DragChuteLev(Selector<29366, 128, 7>);
impl DragChuteLev {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for DragChuteLev {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for DragChuteLev {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EcmBoxBltSwValue {
    Select1,
    Select2,
}
pub type EcmBoxBltSwValueCallback<'a> = &'a mut dyn FnMut(EcmBoxBltSwValue);
impl core::convert::TryFrom<u16> for EcmBoxBltSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - ECM Box Clear Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct EcmBoxBltSw(Selector<29254, 4096, 12>);
impl EcmBoxBltSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EcmBoxBltSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EcmBoxBltSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - ECM Box EM EM Light (yellow)
/// Output: 0 if light is off, 1 if light is on
pub struct EcmBoxEm(Led<29254, 16384, 14>);
impl EcmBoxEm {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for EcmBoxEm {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EcmBoxEm {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - ECM Box EO Light (yellow)
/// Output: 0 if light is off, 1 if light is on
pub struct EcmBoxEo(Led<29262, 1, 0>);
impl EcmBoxEo {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for EcmBoxEo {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EcmBoxEo {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - ECM Box IR Light (yellow)
/// Output: 0 if light is off, 1 if light is on
pub struct EcmBoxIr(Led<29254, 32768, 15>);
impl EcmBoxIr {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for EcmBoxIr {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EcmBoxIr {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - ECM Box LL Light (green)
/// Output: 0 if light is off, 1 if light is on
pub struct EcmBoxLl(Led<29254, 8192, 13>);
impl EcmBoxLl {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for EcmBoxLl {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EcmBoxLl {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EcsAirExchSwValue {
    Select1,
    Select2,
}
pub type EcsAirExchSwValueCallback<'a> = &'a mut dyn FnMut(EcsAirExchSwValue);
impl core::convert::TryFrom<u16> for EcsAirExchSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - ECS Air Exchange Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct EcsAirExchSw(Selector<29262, 32, 5>);
impl EcsAirExchSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EcsAirExchSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EcsAirExchSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EcsCBtnValue {
    Select1,
    Select2,
}
pub type EcsCBtnValueCallback<'a> = &'a mut dyn FnMut(EcsCBtnValue);
impl core::convert::TryFrom<u16> for EcsCBtnValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - ECS C Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct EcsCBtn(Selector<29262, 4, 2>);
impl EcsCBtn {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EcsCBtn {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EcsCBtn {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EcsCondSwValue {
    Select1,
    Select2,
}
pub type EcsCondSwValueCallback<'a> = &'a mut dyn FnMut(EcsCondSwValue);
impl core::convert::TryFrom<u16> for EcsCondSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - ECS Cond Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct EcsCondSw(Selector<29262, 16, 4>);
impl EcsCondSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EcsCondSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EcsCondSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EcsDefogSwValue {
    Select1,
    Select2,
}
pub type EcsDefogSwValueCallback<'a> = &'a mut dyn FnMut(EcsDefogSwValue);
impl core::convert::TryFrom<u16> for EcsDefogSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - ECS Defog Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct EcsDefogSw(Selector<29262, 64, 6>);
impl EcsDefogSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EcsDefogSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EcsDefogSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EcsFBtnValue {
    Select1,
    Select2,
}
pub type EcsFBtnValueCallback<'a> = &'a mut dyn FnMut(EcsFBtnValue);
impl core::convert::TryFrom<u16> for EcsFBtnValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - ECS F Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct EcsFBtn(Selector<29262, 8, 3>);
impl EcsFBtn {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EcsFBtn {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EcsFBtn {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EcsMainModeSwValue {
    Select1,
    Select2,
}
pub type EcsMainModeSwValueCallback<'a> = &'a mut dyn FnMut(EcsMainModeSwValue);
impl core::convert::TryFrom<u16> for EcsMainModeSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - ECS Main Mode Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct EcsMainModeSw(Selector<29262, 2, 1>);
impl EcsMainModeSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EcsMainModeSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EcsMainModeSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// I - ECS Temperature Select Knob
/// Input: turn the dial left or right
/// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
pub struct EcsTempSelectKnob(AnalogDial<29264, 65535, 0>);
impl EcsTempSelectKnob {
    pub const fn new() -> Self {
        Self(AnalogDial::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EcsTempSelectKnob {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EcsTempSelectKnob {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - TEST - ELEC Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct ElecFail(Led<29468, 4096, 12>);
impl ElecFail {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for ElecFail {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ElecFail {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElecPwrTransfSwValue {
    Select1,
    Select2,
}
pub type ElecPwrTransfSwValueCallback<'a> = &'a mut dyn FnMut(ElecPwrTransfSwValue);
impl core::convert::TryFrom<u16> for ElecPwrTransfSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Electric Power Transfer Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct ElecPwrTransfSw(Selector<29252, 4096, 12>);
impl ElecPwrTransfSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ElecPwrTransfSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ElecPwrTransfSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmerCompassValue {
    Select1,
    Select2,
}
pub type EmerCompassValueCallback<'a> = &'a mut dyn FnMut(EmerCompassValue);
impl core::convert::TryFrom<u16> for EmerCompassValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Emergency Compass
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct EmerCompass(Selector<29456, 2, 1>);
impl EmerCompass {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EmerCompass {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EmerCompass {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmerHydPumpSwValue {
    Select1,
    Select2,
}
pub type EmerHydPumpSwValueCallback<'a> = &'a mut dyn FnMut(EmerHydPumpSwValue);
impl core::convert::TryFrom<u16> for EmerHydPumpSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Emergency Hydraulic Pump Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct EmerHydPumpSw(Selector<29442, 8192, 13>);
impl EmerHydPumpSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EmerHydPumpSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EmerHydPumpSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmerJettValue {
    Select1,
    Select2,
}
pub type EmerJettValueCallback<'a> = &'a mut dyn FnMut(EmerJettValue);
impl core::convert::TryFrom<u16> for EmerJettValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Emergency Jettison Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct EmerJett(Selector<29354, 64, 6>);
impl EmerJett {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EmerJett {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EmerJett {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmerThrottleCovValue {
    Select1,
    Select2,
}
pub type EmerThrottleCovValueCallback<'a> = &'a mut dyn FnMut(EmerThrottleCovValue);
impl core::convert::TryFrom<u16> for EmerThrottleCovValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Emergency Throttle Cover
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct EmerThrottleCov(Selector<29334, 32768, 15>);
impl EmerThrottleCov {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EmerThrottleCov {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EmerThrottleCov {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// I - Emergency Throttle Handle
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct EmerThrottleHand(LimitedDial<29350, 65535, 0>);
impl EmerThrottleHand {
    pub const fn new() -> Self {
        Self(LimitedDial::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EmerThrottleHand {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EmerThrottleHand {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EngEmerCtrlCovValue {
    Select1,
    Select2,
}
pub type EngEmerCtrlCovValueCallback<'a> = &'a mut dyn FnMut(EngEmerCtrlCovValue);
impl core::convert::TryFrom<u16> for EngEmerCtrlCovValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Engine Emergency Control Cover
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct EngEmerCtrlCov(Selector<29352, 32, 5>);
impl EngEmerCtrlCov {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EngEmerCtrlCov {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EngEmerCtrlCov {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EngEmerCtrlSwValue {
    Select1,
    Select2,
}
pub type EngEmerCtrlSwValueCallback<'a> = &'a mut dyn FnMut(EngEmerCtrlSwValue);
impl core::convert::TryFrom<u16> for EngEmerCtrlSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Engine Emergency Control Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct EngEmerCtrlSw(Selector<29352, 64, 6>);
impl EngEmerCtrlSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EngEmerCtrlSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EngEmerCtrlSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EngFuelLPumpSwValue {
    Select1,
    Select2,
}
pub type EngFuelLPumpSwValueCallback<'a> = &'a mut dyn FnMut(EngFuelLPumpSwValue);
impl core::convert::TryFrom<u16> for EngFuelLPumpSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Engine Fuel Left Boost Pump Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct EngFuelLPumpSw(Selector<29262, 4096, 12>);
impl EngFuelLPumpSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EngFuelLPumpSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EngFuelLPumpSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EngFuelRPumpSwValue {
    Select1,
    Select2,
}
pub type EngFuelRPumpSwValueCallback<'a> = &'a mut dyn FnMut(EngFuelRPumpSwValue);
impl core::convert::TryFrom<u16> for EngFuelRPumpSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Engine Fuel Right Boost Pump Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct EngFuelRPumpSw(Selector<29262, 8192, 13>);
impl EngFuelRPumpSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EngFuelRPumpSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EngFuelRPumpSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EngInFlgtStartSwValue {
    Select1,
    Select2,
}
pub type EngInFlgtStartSwValueCallback<'a> = &'a mut dyn FnMut(EngInFlgtStartSwValue);
impl core::convert::TryFrom<u16> for EngInFlgtStartSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Engine In-Flight Start Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct EngInFlgtStartSw(Selector<29352, 1, 0>);
impl EngInFlgtStartSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EngInFlgtStartSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EngInFlgtStartSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EngShutdownBtnValue {
    Select1,
    Select2,
}
pub type EngShutdownBtnValueCallback<'a> = &'a mut dyn FnMut(EngShutdownBtnValue);
impl core::convert::TryFrom<u16> for EngShutdownBtnValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Engine Shutdown Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct EngShutdownBtn(Selector<29262, 512, 9>);
impl EngShutdownBtn {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EngShutdownBtn {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EngShutdownBtn {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EngStartSwValue {
    Select1,
    Select2,
}
pub type EngStartSwValueCallback<'a> = &'a mut dyn FnMut(EngStartSwValue);
impl core::convert::TryFrom<u16> for EngStartSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Engine Start Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct EngStartSw(Selector<29262, 16384, 14>);
impl EngStartSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EngStartSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EngStartSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EngStartSwCovValue {
    Select1,
    Select2,
}
pub type EngStartSwCovValueCallback<'a> = &'a mut dyn FnMut(EngStartSwCovValue);
impl core::convert::TryFrom<u16> for EngStartSwCovValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Engine Start Switch Cover
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct EngStartSwCov(Selector<29262, 1024, 10>);
impl EngStartSwCov {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EngStartSwCov {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EngStartSwCov {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EwModeSelSwValue {
    Select1,
    Select2,
    Select3,
}
pub type EwModeSelSwValueCallback<'a> = &'a mut dyn FnMut(EwModeSelSwValue);
impl core::convert::TryFrom<u16> for EwModeSelSwValue {
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

/// I - EW Mode Selector Switch
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct EwModeSelSw(Selector<29276, 3, 0>);
impl EwModeSelSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for EwModeSelSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for EwModeSelSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Aft Formation Lights
/// Output: Aft Formation Lights
pub struct ExtFormationLightsAft(Metadata<29528, 65535, 0>);
impl ExtFormationLightsAft {
    pub const fn new() -> Self {
        Self(Metadata::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ExtFormationLightsAft {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ExtFormationLightsAft {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Front Formation Lights
/// Output: Front Formation Lights
pub struct ExtFormationLightsFront(Metadata<29526, 65535, 0>);
impl ExtFormationLightsFront {
    pub const fn new() -> Self {
        Self(Metadata::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ExtFormationLightsFront {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ExtFormationLightsFront {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Left Position Light (red)
/// Output: Left Position Light (red)
pub struct ExtPositionLightLeft(Metadata<29524, 1, 0>);
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
pub struct ExtPositionLightRight(Metadata<29524, 2, 1>);
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
pub struct ExtSpeedBrakeLeft(Metadata<29522, 65535, 0>);
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
pub struct ExtSpeedBrakeRight(Metadata<29520, 65535, 0>);
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

/// Bottom Strobe Light
/// Output: Bottom Strobe Light
pub struct ExtStrobeBottom(Metadata<29524, 8, 3>);
impl ExtStrobeBottom {
    pub const fn new() -> Self {
        Self(Metadata::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ExtStrobeBottom {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ExtStrobeBottom {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Top Strobe Light
/// Output: Top Strobe Light
pub struct ExtStrobeTop(Metadata<29524, 4, 2>);
impl ExtStrobeTop {
    pub const fn new() -> Self {
        Self(Metadata::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ExtStrobeTop {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ExtStrobeTop {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FbwCh5SwValue {
    Select1,
    Select2,
}
pub type FbwCh5SwValueCallback<'a> = &'a mut dyn FnMut(FbwCh5SwValue);
impl core::convert::TryFrom<u16> for FbwCh5SwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - FBW - Channel 5 Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct FbwCh5Sw(Selector<29352, 256, 8>);
impl FbwCh5Sw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FbwCh5Sw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FbwCh5Sw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FbwCh5SwCovValue {
    Select1,
    Select2,
}
pub type FbwCh5SwCovValueCallback<'a> = &'a mut dyn FnMut(FbwCh5SwCovValue);
impl core::convert::TryFrom<u16> for FbwCh5SwCovValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - FBW - Channel 5 Switch Cover
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct FbwCh5SwCov(Selector<29352, 128, 7>);
impl FbwCh5SwCov {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FbwCh5SwCov {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FbwCh5SwCov {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FbwGLimSwValue {
    Select1,
    Select2,
}
pub type FbwGLimSwValueCallback<'a> = &'a mut dyn FnMut(FbwGLimSwValue);
impl core::convert::TryFrom<u16> for FbwGLimSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - FBW - G-Limiter Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct FbwGLimSw(Selector<29354, 8, 3>);
impl FbwGLimSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FbwGLimSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FbwGLimSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FbwGainModeSwValue {
    Select1,
    Select2,
}
pub type FbwGainModeSwValueCallback<'a> = &'a mut dyn FnMut(FbwGainModeSwValue);
impl core::convert::TryFrom<u16> for FbwGainModeSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - FBW - Gain Mode Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct FbwGainModeSw(Selector<29354, 4, 2>);
impl FbwGainModeSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FbwGainModeSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FbwGainModeSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FbwGainModeSwCovValue {
    Select1,
    Select2,
}
pub type FbwGainModeSwCovValueCallback<'a> = &'a mut dyn FnMut(FbwGainModeSwCovValue);
impl core::convert::TryFrom<u16> for FbwGainModeSwCovValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - FBW - Gain Mode Switch Cover
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct FbwGainModeSwCov(Selector<29354, 2, 1>);
impl FbwGainModeSwCov {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FbwGainModeSwCov {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FbwGainModeSwCov {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FbwResetBtnValue {
    Select1,
    Select2,
}
pub type FbwResetBtnValueCallback<'a> = &'a mut dyn FnMut(FbwResetBtnValue);
impl core::convert::TryFrom<u16> for FbwResetBtnValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - FBW - Reset Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct FbwResetBtn(Selector<29354, 16, 4>);
impl FbwResetBtn {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FbwResetBtn {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FbwResetBtn {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FbwSpinModeSwValue {
    Select1,
    Select2,
}
pub type FbwSpinModeSwValueCallback<'a> = &'a mut dyn FnMut(FbwSpinModeSwValue);
impl core::convert::TryFrom<u16> for FbwSpinModeSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - FBW - Spin Mode Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct FbwSpinModeSw(Selector<29352, 8192, 13>);
impl FbwSpinModeSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FbwSpinModeSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FbwSpinModeSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FbwTestSwValue {
    Select1,
    Select2,
    Select3,
}
pub type FbwTestSwValueCallback<'a> = &'a mut dyn FnMut(FbwTestSwValue);
impl core::convert::TryFrom<u16> for FbwTestSwValue {
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

/// I - TEST - FBW Test Switch
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct FbwTestSw(Selector<29468, 384, 7>);
impl FbwTestSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FbwTestSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FbwTestSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FbwTestSwCovValue {
    Select1,
    Select2,
}
pub type FbwTestSwCovValueCallback<'a> = &'a mut dyn FnMut(FbwTestSwCovValue);
impl core::convert::TryFrom<u16> for FbwTestSwCovValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - TEST - FBW Test Switch Cover
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct FbwTestSwCov(Selector<29468, 2048, 11>);
impl FbwTestSwCov {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FbwTestSwCov {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FbwTestSwCov {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - AL - FEU PC Light
/// Output: 0 if light is off, 1 if light is on
pub struct FeuPc(Led<29354, 32768, 15>);
impl FeuPc {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for FeuPc {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FeuPc {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - AL - FEU SEC Light
/// Output: 0 if light is off, 1 if light is on
pub struct FeuSec(Led<29354, 16384, 14>);
impl FeuSec {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for FeuSec {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FeuSec {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FiringModeSelValue {
    Select1,
    Select2,
}
pub type FiringModeSelValueCallback<'a> = &'a mut dyn FnMut(FiringModeSelValue);
impl core::convert::TryFrom<u16> for FiringModeSelValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PPA - 530D/Rockets/Guns Firing Mode Selector Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct FiringModeSel(Selector<29404, 32768, 15>);
impl FiringModeSel {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FiringModeSel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FiringModeSel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormLgtSwValue {
    Select1,
    Select2,
}
pub type FormLgtSwValueCallback<'a> = &'a mut dyn FnMut(FormLgtSwValue);
impl core::convert::TryFrom<u16> for FormLgtSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Formation Lights Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct FormLgtSw(Selector<29276, 16, 4>);
impl FormLgtSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FormLgtSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FormLgtSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - FUEL - Right AV Light
/// Output: 0 if light is off, 1 if light is on
pub struct FuelAvD(Led<29280, 2048, 11>);
impl FuelAvD {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for FuelAvD {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelAvD {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - FUEL - Left AV Light
/// Output: 0 if light is off, 1 if light is on
pub struct FuelAvG(Led<29280, 1024, 10>);
impl FuelAvG {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for FuelAvG {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelAvG {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FuelCrofSwValue {
    Select1,
    Select2,
}
pub type FuelCrofSwValueCallback<'a> = &'a mut dyn FnMut(FuelCrofSwValue);
impl core::convert::TryFrom<u16> for FuelCrofSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Fuel Crossfeeed Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct FuelCrofSw(Selector<29276, 1024, 10>);
impl FuelCrofSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FuelCrofSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelCrofSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FuelCutoffSwValue {
    Select1,
    Select2,
}
pub type FuelCutoffSwValueCallback<'a> = &'a mut dyn FnMut(FuelCutoffSwValue);
impl core::convert::TryFrom<u16> for FuelCutoffSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Fuel Cutoff Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct FuelCutoffSw(Selector<29274, 1, 0>);
impl FuelCutoffSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FuelCutoffSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelCutoffSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FuelCutoffSwCovValue {
    Select1,
    Select2,
}
pub type FuelCutoffSwCovValueCallback<'a> = &'a mut dyn FnMut(FuelCutoffSwCovValue);
impl core::convert::TryFrom<u16> for FuelCutoffSwCovValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Fuel Cutoff Switch Cover
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct FuelCutoffSwCov(Selector<29262, 32768, 15>);
impl FuelCutoffSwCov {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FuelCutoffSwCov {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelCutoffSwCov {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - FUEL - x100 DETOT display
/// Output: gauge position
pub struct FuelDetotCents(AnalogGauge<29290, 65535, 0>);
impl FuelDetotCents {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FuelDetotCents {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelDetotCents {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FuelDetotSwValue {
    Select1,
    Select2,
}
pub type FuelDetotSwValueCallback<'a> = &'a mut dyn FnMut(FuelDetotSwValue);
impl core::convert::TryFrom<u16> for FuelDetotSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - DETOT Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct FuelDetotSw(Selector<29276, 512, 9>);
impl FuelDetotSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FuelDetotSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelDetotSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - FUEL - x10 DETOT display
/// Output: gauge position
pub struct FuelDetotTens(AnalogGauge<29292, 65535, 0>);
impl FuelDetotTens {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FuelDetotTens {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelDetotTens {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - FUEL - x1000 DETOT display
/// Output: gauge position
pub struct FuelDetotThous(AnalogGauge<29288, 65535, 0>);
impl FuelDetotThous {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FuelDetotThous {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelDetotThous {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FuelDmpSwValue {
    Select1,
    Select2,
}
pub type FuelDmpSwValueCallback<'a> = &'a mut dyn FnMut(FuelDmpSwValue);
impl core::convert::TryFrom<u16> for FuelDmpSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - FUEL - Dump Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct FuelDmpSw(Selector<29280, 16, 4>);
impl FuelDmpSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FuelDmpSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelDmpSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FuelDmpSwCovValue {
    Select1,
    Select2,
}
pub type FuelDmpSwCovValueCallback<'a> = &'a mut dyn FnMut(FuelDmpSwCovValue);
impl core::convert::TryFrom<u16> for FuelDmpSwCovValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - FUEL - Dump Switch Cover
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct FuelDmpSwCov(Selector<29276, 32768, 15>);
impl FuelDmpSwCov {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FuelDmpSwCov {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelDmpSwCov {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - FUEL - Fuel Flow Display
/// Output: O - FUEL - Fuel Flow Display
pub struct FuelFlow(Display<29298, 3>);
impl FuelFlow {
    pub const fn new() -> Self {
        Self(Display::new())
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for FuelFlow {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelFlow {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - FUEL - x100 JAUGE display
/// Output: gauge position
pub struct FuelJaugCents(AnalogGauge<29284, 65535, 0>);
impl FuelJaugCents {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FuelJaugCents {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelJaugCents {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - FUEL - RIGHT JAUGE display
/// Output: gauge position
pub struct FuelJaugD(AnalogGauge<29296, 65535, 0>);
impl FuelJaugD {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FuelJaugD {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelJaugD {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - FUEL - LEFT JAUGE display
/// Output: gauge position
pub struct FuelJaugG(AnalogGauge<29294, 65535, 0>);
impl FuelJaugG {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FuelJaugG {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelJaugG {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - FUEL - x10 JAUGE display
/// Output: gauge position
pub struct FuelJaugTens(AnalogGauge<29286, 65535, 0>);
impl FuelJaugTens {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FuelJaugTens {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelJaugTens {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - FUEL - x1000 JAUGE display
/// Output: gauge position
pub struct FuelJaugThous(AnalogGauge<29282, 65535, 0>);
impl FuelJaugThous {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FuelJaugThous {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelJaugThous {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - FUEL - Right RL Light
/// Output: 0 if light is off, 1 if light is on
pub struct FuelRlD(Led<29280, 512, 9>);
impl FuelRlD {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for FuelRlD {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelRlD {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - FUEL - Left RL Light
/// Output: 0 if light is off, 1 if light is on
pub struct FuelRlG(Led<29280, 128, 7>);
impl FuelRlG {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for FuelRlG {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelRlG {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - FUEL - Center RL Light
/// Output: 0 if light is off, 1 if light is on
pub struct FuelRlM(Led<29280, 256, 8>);
impl FuelRlM {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for FuelRlM {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelRlM {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FuelTransfSwValue {
    Select1,
    Select2,
    Select3,
}
pub type FuelTransfSwValueCallback<'a> = &'a mut dyn FnMut(FuelTransfSwValue);
impl core::convert::TryFrom<u16> for FuelTransfSwValue {
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

/// I - Transfer Switch
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct FuelTransfSw(Selector<29280, 96, 5>);
impl FuelTransfSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for FuelTransfSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelTransfSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - FUEL - Right V Light
/// Output: 0 if light is off, 1 if light is on
pub struct FuelVD(Led<29280, 8192, 13>);
impl FuelVD {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for FuelVD {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelVD {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - FUEL - Left V Light
/// Output: 0 if light is off, 1 if light is on
pub struct FuelVG(Led<29280, 4096, 12>);
impl FuelVG {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for FuelVG {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for FuelVG {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - ACC - G Needle
/// Output: gauge position
pub struct GmeterNeedle(AnalogGauge<29302, 65535, 0>);
impl GmeterNeedle {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for GmeterNeedle {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for GmeterNeedle {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GunArmSwValue {
    Select1,
    Select2,
}
pub type GunArmSwValueCallback<'a> = &'a mut dyn FnMut(GunArmSwValue);
impl core::convert::TryFrom<u16> for GunArmSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Gun Arm/Safe Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct GunArmSw(Selector<29354, 128, 7>);
impl GunArmSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for GunArmSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for GunArmSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GunModeBtnValue {
    Select1,
    Select2,
}
pub type GunModeBtnValueCallback<'a> = &'a mut dyn FnMut(GunModeBtnValue);
impl core::convert::TryFrom<u16> for GunModeBtnValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PCA - Gun Mode Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct GunModeBtn(Selector<29374, 32, 5>);
impl GunModeBtn {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for GunModeBtn {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for GunModeBtn {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCA - Gun Mode Button KL1 Light
/// Output: 0 if light is off, 1 if light is on
pub struct GunModeKl1(Led<29376, 4, 2>);
impl GunModeKl1 {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for GunModeKl1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for GunModeKl1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCA - Gun Mode Button KL2 Light
/// Output: 0 if light is off, 1 if light is on
pub struct GunModeKl2(Led<29376, 8, 3>);
impl GunModeKl2 {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for GunModeKl2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for GunModeKl2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - HSI - Compass
/// Output: gauge position
pub struct HsiCompas(AnalogGauge<29320, 65535, 0>);
impl HsiCompas {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HsiCompas {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HsiCompas {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - HSI - COURSE (Double Needle)
/// Output: gauge position
pub struct HsiDNeedle(AnalogGauge<29308, 65535, 0>);
impl HsiDNeedle {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HsiDNeedle {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HsiDNeedle {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - HSI x100 Distance display
/// Output: gauge position
pub struct HsiDistCents(AnalogGauge<29312, 65535, 0>);
impl HsiDistCents {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HsiDistCents {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HsiDistCents {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - HSI x.1 Distance display
/// Output: gauge position
pub struct HsiDistDec(AnalogGauge<29318, 65535, 0>);
impl HsiDistDec {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HsiDistDec {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HsiDistDec {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - HSI x1 Distance display
/// Output: gauge position
pub struct HsiDistOnes(AnalogGauge<29316, 65535, 0>);
impl HsiDistOnes {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HsiDistOnes {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HsiDistOnes {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - HSI x10 Distance display
/// Output: gauge position
pub struct HsiDistTens(AnalogGauge<29314, 65535, 0>);
impl HsiDistTens {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HsiDistTens {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HsiDistTens {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - HSI - Right Flag
/// Output: gauge position
pub struct HsiFlagD(AnalogGauge<29326, 65535, 0>);
impl HsiFlagD {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HsiFlagD {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HsiFlagD {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - HSI - Distance Flag
/// Output: gauge position
pub struct HsiFlagDist(AnalogGauge<29322, 65535, 0>);
impl HsiFlagDist {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HsiFlagDist {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HsiFlagDist {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - HSI - Left Flag
/// Output: gauge position
pub struct HsiFlagG(AnalogGauge<29324, 65535, 0>);
impl HsiFlagG {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HsiFlagG {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HsiFlagG {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - HSI - Center Flag
/// Output: gauge position
pub struct HsiFlagM(AnalogGauge<29328, 65535, 0>);
impl HsiFlagM {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HsiFlagM {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HsiFlagM {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - HSI - AP Heading (Green Arrow)
/// Output: gauge position
pub struct HsiHdg(AnalogGauge<29306, 65535, 0>);
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HsiModeSelSwValue {
    Select1,
    Select2,
    Select3,
    Select4,
    Select5,
    Select6,
}
pub type HsiModeSelSwValueCallback<'a> = &'a mut dyn FnMut(HsiModeSelSwValue);
impl core::convert::TryFrom<u16> for HsiModeSelSwValue {
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

/// I - HSI - Mode Selector Switch
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct HsiModeSelSw(Selector<29300, 1792, 8>);
impl HsiModeSelSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HsiModeSelSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HsiModeSelSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - HSI - VAD (Simple Needle)
/// Output: gauge position
pub struct HsiNeedle(AnalogGauge<29310, 65535, 0>);
impl HsiNeedle {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HsiNeedle {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HsiNeedle {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// I - HSI - VAD Selector
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct HsiVadSel(LimitedDial<29304, 65535, 0>);
impl HsiVadSel {
    pub const fn new() -> Self {
        Self(LimitedDial::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HsiVadSel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HsiVadSel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HudClrBtnValue {
    Select1,
    Select2,
}
pub type HudClrBtnValueCallback<'a> = &'a mut dyn FnMut(HudClrBtnValue);
impl core::convert::TryFrom<u16> for HudClrBtnValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - HUD - EFF Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct HudClrBtn(Selector<29500, 1, 0>);
impl HudClrBtn {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HudClrBtn {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HudClrBtn {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HudDecSwValue {
    Select1,
    Select2,
}
pub type HudDecSwValueCallback<'a> = &'a mut dyn FnMut(HudDecSwValue);
impl core::convert::TryFrom<u16> for HudDecSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - HUD - Declutter Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct HudDecSw(Selector<29488, 32768, 15>);
impl HudDecSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HudDecSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HudDecSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HudPwrSwValue {
    Select1,
    Select2,
    Select3,
}
pub type HudPwrSwValueCallback<'a> = &'a mut dyn FnMut(HudPwrSwValue);
impl core::convert::TryFrom<u16> for HudPwrSwValue {
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

/// I - HUD - Power Switch
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct HudPwrSw(Selector<29500, 6, 1>);
impl HudPwrSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HudPwrSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HudPwrSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HudRaltSelSwValue {
    Select1,
    Select2,
    Select3,
}
pub type HudRaltSelSwValueCallback<'a> = &'a mut dyn FnMut(HudRaltSelSwValue);
impl core::convert::TryFrom<u16> for HudRaltSelSwValue {
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

/// I - HUD - Altimeter Selector Switch
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct HudRaltSelSw(Selector<29500, 24, 3>);
impl HudRaltSelSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HudRaltSelSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HudRaltSelSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - HUD - Recording Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct HudRec(Led<29500, 128, 7>);
impl HudRec {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for HudRec {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HudRec {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - HYD - Right Hyd Needle (Circuit #2 - Emer)
/// Output: gauge position
pub struct HydDNeedle(AnalogGauge<29372, 65535, 0>);
impl HydDNeedle {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HydDNeedle {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HydDNeedle {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - TEST - HYD Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct HydFail(Led<29468, 8192, 13>);
impl HydFail {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for HydFail {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HydFail {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - HYD - Left Hyd Needle (Circuit #1 - Main)
/// Output: gauge position
pub struct HydGNeedle(AnalogGauge<29370, 65535, 0>);
impl HydGNeedle {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HydGNeedle {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HydGNeedle {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HydSysSelValue {
    Select1,
    Select2,
}
pub type HydSysSelValueCallback<'a> = &'a mut dyn FnMut(HydSysSelValue);
impl core::convert::TryFrom<u16> for HydSysSelValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Hydraulic System Selector
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct HydSysSel(Selector<29366, 2, 1>);
impl HydSysSel {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for HydSysSel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for HydSysSel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdentPwrSwValue {
    Select1,
    Select2,
}
pub type IdentPwrSwValueCallback<'a> = &'a mut dyn FnMut(IdentPwrSwValue);
impl core::convert::TryFrom<u16> for IdentPwrSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - IFF_REP - Ident Power Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct IdentPwrSw(Selector<29300, 32768, 15>);
impl IdentPwrSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for IdentPwrSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for IdentPwrSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - IFF_REP - Fault Light
/// Output: 0 if light is off, 1 if light is on
pub struct IffFault(Led<29334, 16384, 14>);
impl IffFault {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for IffFault {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for IffFault {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - IFF Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct IffInd(Led<29354, 512, 9>);
impl IffInd {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for IffInd {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for IffInd {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IffM4SwValue {
    Select1,
    Select2,
}
pub type IffM4SwValueCallback<'a> = &'a mut dyn FnMut(IffM4SwValue);
impl core::convert::TryFrom<u16> for IffM4SwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - IFF_REP - ON/OUT MODE 4 Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct IffM4Sw(Selector<29334, 4096, 12>);
impl IffM4Sw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for IffM4Sw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for IffM4Sw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - IFF_REP - Red Light
/// Output: 0 if light is off, 1 if light is on
pub struct IffRed(Led<29334, 8192, 13>);
impl IffRed {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for IffRed {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for IffRed {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InsAuxHdHorValue {
    Select1,
    Select2,
    Select3,
}
pub type InsAuxHdHorValueCallback<'a> = &'a mut dyn FnMut(InsAuxHdHorValue);
impl core::convert::TryFrom<u16> for InsAuxHdHorValue {
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

/// I - INS Auxiliary Heading/Horizon
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct InsAuxHdHor(Selector<29456, 12, 2>);
impl InsAuxHdHor {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for InsAuxHdHor {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for InsAuxHdHor {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InsBtn0Value {
    Select1,
    Select2,
}
pub type InsBtn0ValueCallback<'a> = &'a mut dyn FnMut(InsBtn0Value);
impl core::convert::TryFrom<u16> for InsBtn0Value {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PCN - Button 0
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct InsBtn0(Selector<29388, 8192, 13>);
impl InsBtn0 {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for InsBtn0 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for InsBtn0 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InsBtn1Value {
    Select1,
    Select2,
}
pub type InsBtn1ValueCallback<'a> = &'a mut dyn FnMut(InsBtn1Value);
impl core::convert::TryFrom<u16> for InsBtn1Value {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PCN - Button 1
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct InsBtn1(Selector<29384, 4096, 12>);
impl InsBtn1 {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for InsBtn1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for InsBtn1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InsBtn2Value {
    Select1,
    Select2,
}
pub type InsBtn2ValueCallback<'a> = &'a mut dyn FnMut(InsBtn2Value);
impl core::convert::TryFrom<u16> for InsBtn2Value {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PCN - Button 2
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct InsBtn2(Selector<29384, 8192, 13>);
impl InsBtn2 {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for InsBtn2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for InsBtn2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InsBtn3Value {
    Select1,
    Select2,
}
pub type InsBtn3ValueCallback<'a> = &'a mut dyn FnMut(InsBtn3Value);
impl core::convert::TryFrom<u16> for InsBtn3Value {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PCN - Button 3
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct InsBtn3(Selector<29384, 16384, 14>);
impl InsBtn3 {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for InsBtn3 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for InsBtn3 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InsBtn4Value {
    Select1,
    Select2,
}
pub type InsBtn4ValueCallback<'a> = &'a mut dyn FnMut(InsBtn4Value);
impl core::convert::TryFrom<u16> for InsBtn4Value {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PCN - Button 4
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct InsBtn4(Selector<29384, 32768, 15>);
impl InsBtn4 {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for InsBtn4 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for InsBtn4 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InsBtn5Value {
    Select1,
    Select2,
}
pub type InsBtn5ValueCallback<'a> = &'a mut dyn FnMut(InsBtn5Value);
impl core::convert::TryFrom<u16> for InsBtn5Value {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PCN - Button 5
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct InsBtn5(Selector<29388, 256, 8>);
impl InsBtn5 {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for InsBtn5 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for InsBtn5 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InsBtn6Value {
    Select1,
    Select2,
}
pub type InsBtn6ValueCallback<'a> = &'a mut dyn FnMut(InsBtn6Value);
impl core::convert::TryFrom<u16> for InsBtn6Value {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PCN - Button 6
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct InsBtn6(Selector<29388, 512, 9>);
impl InsBtn6 {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for InsBtn6 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for InsBtn6 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InsBtn7Value {
    Select1,
    Select2,
}
pub type InsBtn7ValueCallback<'a> = &'a mut dyn FnMut(InsBtn7Value);
impl core::convert::TryFrom<u16> for InsBtn7Value {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PCN - Button 7
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct InsBtn7(Selector<29388, 1024, 10>);
impl InsBtn7 {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for InsBtn7 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for InsBtn7 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InsBtn8Value {
    Select1,
    Select2,
}
pub type InsBtn8ValueCallback<'a> = &'a mut dyn FnMut(InsBtn8Value);
impl core::convert::TryFrom<u16> for InsBtn8Value {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PCN - Button 8
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct InsBtn8(Selector<29388, 2048, 11>);
impl InsBtn8 {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for InsBtn8 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for InsBtn8 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InsBtn9Value {
    Select1,
    Select2,
}
pub type InsBtn9ValueCallback<'a> = &'a mut dyn FnMut(InsBtn9Value);
impl core::convert::TryFrom<u16> for InsBtn9Value {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PCN - Button 9
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct InsBtn9(Selector<29388, 4096, 12>);
impl InsBtn9 {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for InsBtn9 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for InsBtn9 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InsClrBtnValue {
    Select1,
    Select2,
}
pub type InsClrBtnValueCallback<'a> = &'a mut dyn FnMut(InsClrBtnValue);
impl core::convert::TryFrom<u16> for InsClrBtnValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PCN - EFF Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct InsClrBtn(Selector<29388, 16384, 14>);
impl InsClrBtn {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for InsClrBtn {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for InsClrBtn {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InsDestSwValue {
    Select1,
    Select2,
}
pub type InsDestSwValueCallback<'a> = &'a mut dyn FnMut(InsDestSwValue);
impl core::convert::TryFrom<u16> for InsDestSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PCN - DEST Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct InsDestSw(Selector<29376, 32768, 15>);
impl InsDestSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for InsDestSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for InsDestSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCN - DEST Button Light
/// Output: 0 if light is off, 1 if light is on
pub struct InsDestSwLight(Led<29396, 512, 9>);
impl InsDestSwLight {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for InsDestSwLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for InsDestSwLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCN - EFF Button Light
/// Output: 0 if light is off, 1 if light is on
pub struct InsEffSwLight(Led<29396, 4096, 12>);
impl InsEffSwLight {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for InsEffSwLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for InsEffSwLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InsEnterBtnValue {
    Select1,
    Select2,
}
pub type InsEnterBtnValueCallback<'a> = &'a mut dyn FnMut(InsEnterBtnValue);
impl core::convert::TryFrom<u16> for InsEnterBtnValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PCN - INS Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct InsEnterBtn(Selector<29388, 32768, 15>);
impl InsEnterBtn {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for InsEnterBtn {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for InsEnterBtn {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCN - INS Button Light
/// Output: 0 if light is off, 1 if light is on
pub struct InsInsSwLight(Led<29396, 8192, 13>);
impl InsInsSwLight {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for InsInsSwLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for InsInsSwLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InsModeSelValue {
    Select1,
    Select2,
    Select3,
    Select4,
    Select5,
    Select6,
    Select7,
    Select8,
}
pub type InsModeSelValueCallback<'a> = &'a mut dyn FnMut(InsModeSelValue);
impl core::convert::TryFrom<u16> for InsModeSelValue {
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

/// I - PSM - INS Mode Selector
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct InsModeSel(Selector<29412, 57344, 13>);
impl InsModeSel {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for InsModeSel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for InsModeSel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InsNextWpBtnValue {
    Select1,
    Select2,
}
pub type InsNextWpBtnValueCallback<'a> = &'a mut dyn FnMut(InsNextWpBtnValue);
impl core::convert::TryFrom<u16> for InsNextWpBtnValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PCN - INS Next Waypoint Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct InsNextWpBtn(Selector<29392, 256, 8>);
impl InsNextWpBtn {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for InsNextWpBtn {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for InsNextWpBtn {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InsOpalModeValue {
    Select1,
    Select2,
    Select3,
    Select4,
    Select5,
}
pub type InsOpalModeValueCallback<'a> = &'a mut dyn FnMut(InsOpalModeValue);
impl core::convert::TryFrom<u16> for InsOpalModeValue {
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

/// I - PSM - INS Operational Mode
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct InsOpalMode(Selector<29412, 7168, 10>);
impl InsOpalMode {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for InsOpalMode {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for InsOpalMode {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

///  I - PCN - INS Parameter Selector - Rotary Variant
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct InsParamSel(Selector<29380, 61440, 12>);
impl InsParamSel {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for InsParamSel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for InsParamSel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InsPrepSwValue {
    Select1,
    Select2,
}
pub type InsPrepSwValueCallback<'a> = &'a mut dyn FnMut(InsPrepSwValue);
impl core::convert::TryFrom<u16> for InsPrepSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PCN - PREP Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct InsPrepSw(Selector<29376, 16384, 14>);
impl InsPrepSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for InsPrepSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for InsPrepSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCN - PREP Button Light
/// Output: 0 if light is off, 1 if light is on
pub struct InsPrepSwLight(Led<29396, 256, 8>);
impl InsPrepSwLight {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for InsPrepSwLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for InsPrepSwLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InsPrevWpBtnValue {
    Select1,
    Select2,
}
pub type InsPrevWpBtnValueCallback<'a> = &'a mut dyn FnMut(InsPrevWpBtnValue);
impl core::convert::TryFrom<u16> for InsPrevWpBtnValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PCN - INS Previous Waypoint Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct InsPrevWpBtn(Selector<29392, 512, 9>);
impl InsPrevWpBtn {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for InsPrevWpBtn {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for InsPrevWpBtn {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InsUpdateValue {
    Select1,
    Select2,
}
pub type InsUpdateValueCallback<'a> = &'a mut dyn FnMut(InsUpdateValue);
impl core::convert::TryFrom<u16> for InsUpdateValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PCN - INS Update (REC)
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct InsUpdate(Selector<29384, 512, 9>);
impl InsUpdate {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for InsUpdate {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for InsUpdate {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntakeSlatsOpSwValue {
    Select1,
    Select2,
}
pub type IntakeSlatsOpSwValueCallback<'a> = &'a mut dyn FnMut(IntakeSlatsOpSwValue);
impl core::convert::TryFrom<u16> for IntakeSlatsOpSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Intake Slats Operation Switch (PELLES)
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct IntakeSlatsOpSw(Selector<29184, 4, 2>);
impl IntakeSlatsOpSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for IntakeSlatsOpSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for IntakeSlatsOpSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - LDG - Landing Gear Lever Light
/// Output: 0 if light is off, 1 if light is on
pub struct LandingGearLeverLight(Led<29354, 32, 5>);
impl LandingGearLeverLight {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for LandingGearLeverLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LandingGearLeverLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LdgEmerRelLevValue {
    Select1,
    Select2,
}
pub type LdgEmerRelLevValueCallback<'a> = &'a mut dyn FnMut(LdgEmerRelLevValue);
impl core::convert::TryFrom<u16> for LdgEmerRelLevValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - LDG - Landing Gear Emergency Release Lever
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct LdgEmerRelLev(Selector<29354, 1, 0>);
impl LdgEmerRelLev {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for LdgEmerRelLev {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LdgEmerRelLev {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LdgLevValue {
    Select1,
    Select2,
}
pub type LdgLevValueCallback<'a> = &'a mut dyn FnMut(LdgLevValue);
impl core::convert::TryFrom<u16> for LdgLevValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - LDG - Landing Gear Lever
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct LdgLev(Selector<29352, 16384, 14>);
impl LdgLev {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for LdgLev {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LdgLev {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LdgLgtSwValue {
    Select1,
    Select2,
    Select3,
}
pub type LdgLgtSwValueCallback<'a> = &'a mut dyn FnMut(LdgLgtSwValue);
impl core::convert::TryFrom<u16> for LdgLgtSwValue {
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

/// I - Landing Lights Switch
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct LdgLgtSw(Selector<29276, 96, 5>);
impl LdgLgtSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for LdgLgtSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LdgLgtSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// I - Light Brightnes Control/Test
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct LgtBrightCtrlTest(LimitedDial<29336, 65535, 0>);
impl LgtBrightCtrlTest {
    pub const fn new() -> Self {
        Self(LimitedDial::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for LgtBrightCtrlTest {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LgtBrightCtrlTest {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LgtTestSwValue {
    Select1,
    Select2,
    Select3,
}
pub type LgtTestSwValueCallback<'a> = &'a mut dyn FnMut(LgtTestSwValue);
impl core::convert::TryFrom<u16> for LgtTestSwValue {
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

/// I - Lights Test Switch
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct LgtTestSw(Selector<29254, 3, 0>);
impl LgtTestSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for LgtTestSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LgtTestSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - LIM Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct LimInd(Led<29354, 256, 8>);
impl LimInd {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for LimInd {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LimInd {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoxDilLevValue {
    Select1,
    Select2,
}
pub type LoxDilLevValueCallback<'a> = &'a mut dyn FnMut(LoxDilLevValue);
impl core::convert::TryFrom<u16> for LoxDilLevValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - LOX Dilution Lever
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct LoxDilLev(Selector<29456, 32, 5>);
impl LoxDilLev {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for LoxDilLev {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LoxDilLev {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoxEmerSupValue {
    Select1,
    Select2,
}
pub type LoxEmerSupValueCallback<'a> = &'a mut dyn FnMut(LoxEmerSupValue);
impl core::convert::TryFrom<u16> for LoxEmerSupValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - LOX Emergency Supply
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct LoxEmerSup(Selector<29456, 64, 6>);
impl LoxEmerSup {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for LoxEmerSup {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for LoxEmerSup {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MagicStandbyValue {
    Select1,
    Select2,
}
pub type MagicStandbyValueCallback<'a> = &'a mut dyn FnMut(MagicStandbyValue);
impl core::convert::TryFrom<u16> for MagicStandbyValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PPA - MAGIC Standby Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MagicStandby(Selector<29404, 256, 8>);
impl MagicStandby {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MagicStandby {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MagicStandby {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// I - MAGIC Tone Volume Knob
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct MagicToneVolKnob(LimitedDial<29228, 65535, 0>);
impl MagicToneVolKnob {
    pub const fn new() -> Self {
        Self(LimitedDial::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MagicToneVolKnob {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MagicToneVolKnob {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MagnetoSwValue {
    Select1,
    Select2,
}
pub type MagnetoSwValueCallback<'a> = &'a mut dyn FnMut(MagnetoSwValue);
impl core::convert::TryFrom<u16> for MagnetoSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Magneto Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MagnetoSw(Selector<29352, 4096, 12>);
impl MagnetoSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MagnetoSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MagnetoSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MainBattSwValue {
    Select1,
    Select2,
}
pub type MainBattSwValueCallback<'a> = &'a mut dyn FnMut(MainBattSwValue);
impl core::convert::TryFrom<u16> for MainBattSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Main Battery Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MainBattSw(Selector<29252, 2048, 11>);
impl MainBattSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MainBattSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MainBattSw {
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
}
pub type MasterArmSwValueCallback<'a> = &'a mut dyn FnMut(MasterArmSwValue);
impl core::convert::TryFrom<u16> for MasterArmSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PCA - Master Arm Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MasterArmSw(Selector<29366, 32768, 15>);
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

/// O - AL - Amber PANNE Button Light
/// Output: 0 if light is off, 1 if light is on
pub struct McAmbre(Led<29354, 2048, 11>);
impl McAmbre {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for McAmbre {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for McAmbre {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - AL - Red PANNE Button Light
/// Output: 0 if light is off, 1 if light is on
pub struct McRouge(Led<29354, 4096, 12>);
impl McRouge {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for McRouge {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for McRouge {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// I - HUD - Minimum Altitude Selector
/// Input: turn the dial left or right
/// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
pub struct MinAltSel(AnalogDial<29494, 65535, 0>);
impl MinAltSel {
    pub const fn new() -> Self {
        Self(AnalogDial::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MinAltSel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MinAltSel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MirrorToggleValue {
    Select1,
    Select2,
}
pub type MirrorToggleValueCallback<'a> = &'a mut dyn FnMut(MirrorToggleValue);
impl core::convert::TryFrom<u16> for MirrorToggleValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Mirror Rendering Toggle
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MirrorToggle(Selector<29366, 1024, 10>);
impl MirrorToggle {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MirrorToggle {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MirrorToggle {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MisSelSwValue {
    Select1,
    Select2,
    Select3,
}
pub type MisSelSwValueCallback<'a> = &'a mut dyn FnMut(MisSelSwValue);
impl core::convert::TryFrom<u16> for MisSelSwValue {
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

/// I - PPA - Missile Selector Switch
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct MisSelSw(Selector<29404, 1536, 9>);
impl MisSelSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MisSelSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MisSelSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode110SelValue {
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
pub type Mode110SelValueCallback<'a> = &'a mut dyn FnMut(Mode110SelValue);
impl core::convert::TryFrom<u16> for Mode110SelValue {
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

/// I - IFF_REP - Mode-1 Tens Selector
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct Mode110Sel(Selector<29332, 15, 0>);
impl Mode110Sel {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Mode110Sel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Mode110Sel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode11SelValue {
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
pub type Mode11SelValueCallback<'a> = &'a mut dyn FnMut(Mode11SelValue);
impl core::convert::TryFrom<u16> for Mode11SelValue {
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

/// I - IFF_REP - Mode-1 Ones Selector
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct Mode11Sel(Selector<29332, 240, 4>);
impl Mode11Sel {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Mode11Sel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Mode11Sel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode1SwValue {
    Select1,
    Select2,
}
pub type Mode1SwValueCallback<'a> = &'a mut dyn FnMut(Mode1SwValue);
impl core::convert::TryFrom<u16> for Mode1SwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - IFF_REP - Mode-1 Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Mode1Sw(Selector<29334, 256, 8>);
impl Mode1Sw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Mode1Sw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Mode1Sw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode2SwValue {
    Select1,
    Select2,
}
pub type Mode2SwValueCallback<'a> = &'a mut dyn FnMut(Mode2SwValue);
impl core::convert::TryFrom<u16> for Mode2SwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - IFF_REP - Mode-2 Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Mode2Sw(Selector<29334, 512, 9>);
impl Mode2Sw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Mode2Sw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Mode2Sw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode3a100SelValue {
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
pub type Mode3a100SelValueCallback<'a> = &'a mut dyn FnMut(Mode3a100SelValue);
impl core::convert::TryFrom<u16> for Mode3a100SelValue {
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

/// I - IFF_REP - Mode-3A Hundreds Selector
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct Mode3a100Sel(Selector<29332, 61440, 12>);
impl Mode3a100Sel {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Mode3a100Sel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Mode3a100Sel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode3a10SelValue {
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
pub type Mode3a10SelValueCallback<'a> = &'a mut dyn FnMut(Mode3a10SelValue);
impl core::convert::TryFrom<u16> for Mode3a10SelValue {
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

/// I - IFF_REP - Mode-3A Tens Selector
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct Mode3a10Sel(Selector<29334, 15, 0>);
impl Mode3a10Sel {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Mode3a10Sel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Mode3a10Sel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode3a1SelValue {
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
pub type Mode3a1SelValueCallback<'a> = &'a mut dyn FnMut(Mode3a1SelValue);
impl core::convert::TryFrom<u16> for Mode3a1SelValue {
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

/// I - IFF_REP - Mode-3A Ones Selector
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct Mode3a1Sel(Selector<29334, 240, 4>);
impl Mode3a1Sel {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Mode3a1Sel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Mode3a1Sel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode3a1kSelValue {
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
pub type Mode3a1kSelValueCallback<'a> = &'a mut dyn FnMut(Mode3a1kSelValue);
impl core::convert::TryFrom<u16> for Mode3a1kSelValue {
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

/// I - IFF_REP - Mode-3A Thousands Selector
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct Mode3a1kSel(Selector<29332, 3840, 8>);
impl Mode3a1kSel {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Mode3a1kSel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Mode3a1kSel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode3aSwValue {
    Select1,
    Select2,
}
pub type Mode3aSwValueCallback<'a> = &'a mut dyn FnMut(Mode3aSwValue);
impl core::convert::TryFrom<u16> for Mode3aSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - IFF_REP - Mode-3A Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Mode3aSw(Selector<29334, 1024, 10>);
impl Mode3aSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Mode3aSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Mode3aSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModeCSwValue {
    Select1,
    Select2,
}
pub type ModeCSwValueCallback<'a> = &'a mut dyn FnMut(ModeCSwValue);
impl core::convert::TryFrom<u16> for ModeCSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - IFF_REP - Mode-C Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct ModeCSw(Selector<29334, 2048, 11>);
impl ModeCSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ModeCSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ModeCSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MrkPosValue {
    Select1,
    Select2,
}
pub type MrkPosValueCallback<'a> = &'a mut dyn FnMut(MrkPosValue);
impl core::convert::TryFrom<u16> for MrkPosValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PCN - Mark Position (MRC)
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct MrkPos(Selector<29384, 2048, 11>);
impl MrkPos {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MrkPos {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MrkPos {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// I - Marker Signal Volume Knob
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct MrkSigVolKnob(LimitedDial<29232, 65535, 0>);
impl MrkSigVolKnob {
    pub const fn new() -> Self {
        Self(LimitedDial::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for MrkSigVolKnob {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for MrkSigVolKnob {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - ENG - RPM Needle
/// Output: gauge position
pub struct NRpm(AnalogGauge<29266, 65535, 0>);
impl NRpm {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for NRpm {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for NRpm {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NavLgtSwValue {
    Select1,
    Select2,
}
pub type NavLgtSwValueCallback<'a> = &'a mut dyn FnMut(NavLgtSwValue);
impl core::convert::TryFrom<u16> for NavLgtSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Navigation Lights Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct NavLgtSw(Selector<29276, 8, 3>);
impl NavLgtSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for NavLgtSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for NavLgtSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NvgHelmetMountValue {
    Select1,
    Select2,
}
pub type NvgHelmetMountValueCallback<'a> = &'a mut dyn FnMut(NvgHelmetMountValue);
impl core::convert::TryFrom<u16> for NvgHelmetMountValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - NVG - Mount/Unmount NVG on Helmet
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct NvgHelmetMount(Selector<29500, 256, 8>);
impl NvgHelmetMount {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for NvgHelmetMount {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for NvgHelmetMount {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NvgLightFilterSwValue {
    Select1,
    Select2,
}
pub type NvgLightFilterSwValueCallback<'a> = &'a mut dyn FnMut(NvgLightFilterSwValue);
impl core::convert::TryFrom<u16> for NvgLightFilterSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - NVG - NVG Lights Filter Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct NvgLightFilterSw(Selector<29500, 1024, 10>);
impl NvgLightFilterSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for NvgLightFilterSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for NvgLightFilterSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NvgStowValue {
    Select1,
    Select2,
}
pub type NvgStowValueCallback<'a> = &'a mut dyn FnMut(NvgStowValue);
impl core::convert::TryFrom<u16> for NvgStowValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - NVG - STOW/UNSTOW NVG
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct NvgStow(Selector<29500, 512, 9>);
impl NvgStow {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for NvgStow {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for NvgStow {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NwsIffInterrBtnValue {
    Select1,
    Select2,
}
pub type NwsIffInterrBtnValueCallback<'a> = &'a mut dyn FnMut(NwsIffInterrBtnValue);
impl core::convert::TryFrom<u16> for NwsIffInterrBtnValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Nose Wheel Steering / IFF Interrogation Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct NwsIffInterrBtn(Selector<29366, 4, 2>);
impl NwsIffInterrBtn {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for NwsIffInterrBtn {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for NwsIffInterrBtn {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OffsetWpTgtValue {
    Select1,
    Select2,
}
pub type OffsetWpTgtValueCallback<'a> = &'a mut dyn FnMut(OffsetWpTgtValue);
impl core::convert::TryFrom<u16> for OffsetWpTgtValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PCN - Offset Waypoint/Target (BAD)
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct OffsetWpTgt(Selector<29384, 256, 8>);
impl OffsetWpTgt {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for OffsetWpTgt {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for OffsetWpTgt {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - LOX - Oxygen Flow Indicator
/// Output: gauge position
pub struct OxyFlowInd(AnalogGauge<29460, 65535, 0>);
impl OxyFlowInd {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for OxyFlowInd {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for OxyFlowInd {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - LOX - Needle
/// Output: gauge position
pub struct OxyNeedle(AnalogGauge<29458, 65535, 0>);
impl OxyNeedle {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for OxyNeedle {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for OxyNeedle {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - AL - PC Light
/// Output: 0 if light is off, 1 if light is on
pub struct Pc(Led<29354, 8192, 13>);
impl Pc {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Pc {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Pc {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCA Bottom #1 Display
/// Output: O - PCA Bottom #1 Display
pub struct PcaBr1Disp(Display<29398, 3>);
impl PcaBr1Disp {
    pub const fn new() -> Self {
        Self(Display::new())
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for PcaBr1Disp {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcaBr1Disp {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCA Bottom #2 Display
/// Output: O - PCA Bottom #2 Display
pub struct PcaBr2Disp(Display<29402, 3>);
impl PcaBr2Disp {
    pub const fn new() -> Self {
        Self(Display::new())
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for PcaBr2Disp {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcaBr2Disp {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCA Bottom #3 Display
/// Output: O - PCA Bottom #3 Display
pub struct PcaBr3Disp(Display<29406, 3>);
impl PcaBr3Disp {
    pub const fn new() -> Self {
        Self(Display::new())
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for PcaBr3Disp {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcaBr3Disp {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCA Bottom #4 Display
/// Output: O - PCA Bottom #4 Display
pub struct PcaBr4Disp(Display<29410, 3>);
impl PcaBr4Disp {
    pub const fn new() -> Self {
        Self(Display::new())
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for PcaBr4Disp {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcaBr4Disp {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCA Bottom #5 Display
/// Output: O - PCA Bottom #5 Display
pub struct PcaBr5Disp(Display<29414, 3>);
impl PcaBr5Disp {
    pub const fn new() -> Self {
        Self(Display::new())
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for PcaBr5Disp {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcaBr5Disp {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PcaBtn1Value {
    Select1,
    Select2,
}
pub type PcaBtn1ValueCallback<'a> = &'a mut dyn FnMut(PcaBtn1Value);
impl core::convert::TryFrom<u16> for PcaBtn1Value {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PCA - Button 1
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct PcaBtn1(Selector<29374, 1, 0>);
impl PcaBtn1 {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for PcaBtn1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcaBtn1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PcaBtn2Value {
    Select1,
    Select2,
}
pub type PcaBtn2ValueCallback<'a> = &'a mut dyn FnMut(PcaBtn2Value);
impl core::convert::TryFrom<u16> for PcaBtn2Value {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PCA - Button 2
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct PcaBtn2(Selector<29374, 2, 1>);
impl PcaBtn2 {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for PcaBtn2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcaBtn2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PcaBtn3Value {
    Select1,
    Select2,
}
pub type PcaBtn3ValueCallback<'a> = &'a mut dyn FnMut(PcaBtn3Value);
impl core::convert::TryFrom<u16> for PcaBtn3Value {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PCA - Button 3
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct PcaBtn3(Selector<29374, 4, 2>);
impl PcaBtn3 {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for PcaBtn3 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcaBtn3 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PcaBtn4Value {
    Select1,
    Select2,
}
pub type PcaBtn4ValueCallback<'a> = &'a mut dyn FnMut(PcaBtn4Value);
impl core::convert::TryFrom<u16> for PcaBtn4Value {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PCA - Button 4
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct PcaBtn4(Selector<29374, 8, 3>);
impl PcaBtn4 {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for PcaBtn4 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcaBtn4 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PcaBtn5Value {
    Select1,
    Select2,
}
pub type PcaBtn5ValueCallback<'a> = &'a mut dyn FnMut(PcaBtn5Value);
impl core::convert::TryFrom<u16> for PcaBtn5Value {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PCA - Button 5
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct PcaBtn5(Selector<29374, 16, 4>);
impl PcaBtn5 {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for PcaBtn5 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcaBtn5 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCA - Button 1 S Light
/// Output: 0 if light is off, 1 if light is on
pub struct PcaButton1S(Led<29374, 8192, 13>);
impl PcaButton1S {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for PcaButton1S {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcaButton1S {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCA - Button 2 S Light
/// Output: 0 if light is off, 1 if light is on
pub struct PcaButton2S(Led<29374, 16384, 14>);
impl PcaButton2S {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for PcaButton2S {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcaButton2S {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCA - Button 3 S Light
/// Output: 0 if light is off, 1 if light is on
pub struct PcaButton3S(Led<29374, 32768, 15>);
impl PcaButton3S {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for PcaButton3S {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcaButton3S {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCA - Button 4 S Light
/// Output: 0 if light is off, 1 if light is on
pub struct PcaButton4S(Led<29376, 1, 0>);
impl PcaButton4S {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for PcaButton4S {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcaButton4S {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCA - Button 5 S Light
/// Output: 0 if light is off, 1 if light is on
pub struct PcaButton5S(Led<29376, 2, 1>);
impl PcaButton5S {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for PcaButton5S {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcaButton5S {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCA Upper #1 Display
/// Output: O - PCA Upper #1 Display
pub struct PcaUr1Disp(Display<29378, 3>);
impl PcaUr1Disp {
    pub const fn new() -> Self {
        Self(Display::new())
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for PcaUr1Disp {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcaUr1Disp {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCA Upper #2 Display
/// Output: O - PCA Upper #2 Display
pub struct PcaUr2Disp(Display<29382, 3>);
impl PcaUr2Disp {
    pub const fn new() -> Self {
        Self(Display::new())
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for PcaUr2Disp {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcaUr2Disp {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCA Upper #3 Display
/// Output: O - PCA Upper #3 Display
pub struct PcaUr3Disp(Display<29386, 3>);
impl PcaUr3Disp {
    pub const fn new() -> Self {
        Self(Display::new())
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for PcaUr3Disp {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcaUr3Disp {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCA Upper #4 Display
/// Output: O - PCA Upper #4 Display
pub struct PcaUr4Disp(Display<29390, 3>);
impl PcaUr4Disp {
    pub const fn new() -> Self {
        Self(Display::new())
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for PcaUr4Disp {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcaUr4Disp {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCA Upper #5 Display
/// Output: O - PCA Upper #5 Display
pub struct PcaUr5Disp(Display<29394, 3>);
impl PcaUr5Disp {
    pub const fn new() -> Self {
        Self(Display::new())
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for PcaUr5Disp {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcaUr5Disp {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCN - ALN Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct PcnAln(Led<29392, 2048, 11>);
impl PcnAln {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for PcnAln {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcnAln {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCN - BAD Button Light
/// Output: 0 if light is off, 1 if light is on
pub struct PcnBad(Led<29396, 1024, 10>);
impl PcnBad {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for PcnBad {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcnBad {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// PCN Digit Left Display (Multi Character)
/// Output: PCN Digit Left Display (Multi Character)
pub struct PcnDis2dl(Display<29444, 2>);
impl PcnDis2dl {
    pub const fn new() -> Self {
        Self(Display::new())
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for PcnDis2dl {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcnDis2dl {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// PCN Digit Right Display (Multi Character)
/// Output: PCN Digit Right Display (Multi Character)
pub struct PcnDis2dr(Display<29446, 2>);
impl PcnDis2dr {
    pub const fn new() -> Self {
        Self(Display::new())
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for PcnDis2dr {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcnDis2dr {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// PCN Digit Left Display
/// Output: PCN Digit Left Display
pub struct PcnDisDl(Display<29440, 1>);
impl PcnDisDl {
    pub const fn new() -> Self {
        Self(Display::new())
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for PcnDisDl {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcnDisDl {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// PCN Digit Right Display
/// Output: PCN Digit Right Display
pub struct PcnDisDr(Display<29442, 1>);
impl PcnDisDr {
    pub const fn new() -> Self {
        Self(Display::new())
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for PcnDisDr {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcnDisDr {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCN - DEST Display
/// Output: O - PCN - DEST Display
pub struct PcnDispDest(Display<29418, 2>);
impl PcnDispDest {
    pub const fn new() -> Self {
        Self(Display::new())
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for PcnDispDest {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcnDispDest {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCN - Left Display
/// Output: O - PCN - Left Display
pub struct PcnDispL(Display<29420, 8>);
impl PcnDispL {
    pub const fn new() -> Self {
        Self(Display::new())
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for PcnDispL {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcnDispL {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCN - PREP Display
/// Output: O - PCN - PREP Display
pub struct PcnDispPrep(Display<29428, 2>);
impl PcnDispPrep {
    pub const fn new() -> Self {
        Self(Display::new())
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for PcnDispPrep {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcnDispPrep {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCN - Right Display
/// Output: O - PCN - Right Display
pub struct PcnDispR(Display<29430, 9>);
impl PcnDispR {
    pub const fn new() -> Self {
        Self(Display::new())
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for PcnDispR {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcnDispR {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCN - ENC Button Light
/// Output: 0 if light is off, 1 if light is on
pub struct PcnEncSwLight(Led<29400, 256, 8>);
impl PcnEncSwLight {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for PcnEncSwLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcnEncSwLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCN - M91 Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct PcnM91Light(Led<29400, 512, 9>);
impl PcnM91Light {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for PcnM91Light {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcnM91Light {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCN - M92 Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct PcnM92Light(Led<29400, 1024, 10>);
impl PcnM92Light {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for PcnM92Light {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcnM92Light {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCN - M93 Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct PcnM93Light(Led<29400, 2048, 11>);
impl PcnM93Light {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for PcnM93Light {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcnM93Light {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCN - MIP Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct PcnMip(Led<29392, 4096, 12>);
impl PcnMip {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for PcnMip {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcnMip {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCN - MRQ Button Light
/// Output: 0 if light is off, 1 if light is on
pub struct PcnMrqSwLight(Led<29396, 32768, 15>);
impl PcnMrqSwLight {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for PcnMrqSwLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcnMrqSwLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCN - N.DEG Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct PcnNdeg(Led<29392, 8192, 13>);
impl PcnNdeg {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for PcnNdeg {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcnNdeg {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCN - PRET Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct PcnPret(Led<29392, 1024, 10>);
impl PcnPret {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for PcnPret {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcnPret {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCN - REC Button Light
/// Output: 0 if light is off, 1 if light is on
pub struct PcnRec(Led<29396, 2048, 11>);
impl PcnRec {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for PcnRec {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcnRec {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCN - SEC Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct PcnSec(Led<29392, 16384, 14>);
impl PcnSec {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for PcnSec {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcnSec {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCN - UNI Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct PcnUni(Led<29392, 32768, 15>);
impl PcnUni {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for PcnUni {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcnUni {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCN - VAL Button Light
/// Output: 0 if light is off, 1 if light is on
pub struct PcnValSwLight(Led<29396, 16384, 14>);
impl PcnValSwLight {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for PcnValSwLight {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PcnValSwLight {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PedalAdjustLevValue {
    Select1,
    Select2,
}
pub type PedalAdjustLevValueCallback<'a> = &'a mut dyn FnMut(PedalAdjustLevValue);
impl core::convert::TryFrom<u16> for PedalAdjustLevValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Pedal Adjustment Lever
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct PedalAdjustLev(Selector<29366, 32, 5>);
impl PedalAdjustLev {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for PedalAdjustLev {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PedalAdjustLev {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PitotHeatCovValue {
    Select1,
    Select2,
}
pub type PitotHeatCovValueCallback<'a> = &'a mut dyn FnMut(PitotHeatCovValue);
impl core::convert::TryFrom<u16> for PitotHeatCovValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Pitot Heat Cover
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct PitotHeatCov(Selector<29442, 16384, 14>);
impl PitotHeatCov {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for PitotHeatCov {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PitotHeatCov {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PitotHeatSwValue {
    Select1,
    Select2,
}
pub type PitotHeatSwValueCallback<'a> = &'a mut dyn FnMut(PitotHeatSwValue);
impl core::convert::TryFrom<u16> for PitotHeatSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Pitot Heat Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct PitotHeatSw(Selector<29442, 32768, 15>);
impl PitotHeatSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for PitotHeatSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PitotHeatSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PkgBrakeLevValue {
    Select1,
    Select2,
}
pub type PkgBrakeLevValueCallback<'a> = &'a mut dyn FnMut(PkgBrakeLevValue);
impl core::convert::TryFrom<u16> for PkgBrakeLevValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Parking Brake Lever
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct PkgBrakeLev(Selector<29456, 1, 0>);
impl PkgBrakeLev {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for PkgBrakeLev {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PkgBrakeLev {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PoliceLgtSwValue {
    Select1,
    Select2,
}
pub type PoliceLgtSwValueCallback<'a> = &'a mut dyn FnMut(PoliceLgtSwValue);
impl core::convert::TryFrom<u16> for PoliceLgtSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Police Lights Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct PoliceLgtSw(Selector<29274, 32768, 15>);
impl PoliceLgtSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for PoliceLgtSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PoliceLgtSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PPA - AUTO Light
/// Output: 0 if light is off, 1 if light is on
pub struct PpaAuto(Led<29408, 4096, 12>);
impl PpaAuto {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for PpaAuto {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PpaAuto {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PPA - Gun/Rockets Mode PAR Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct PpaGunRocketPap(Led<29412, 256, 8>);
impl PpaGunRocketPap {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for PpaGunRocketPap {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PpaGunRocketPap {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PPA - Gun/Rockets Mode TOT Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct PpaGunRocketTot(Led<29412, 512, 9>);
impl PpaGunRocketTot {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for PpaGunRocketTot {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PpaGunRocketTot {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PPA Interval Display
/// Output: O - PPA Interval Display
pub struct PpaIntDisp(Display<29450, 2>);
impl PpaIntDisp {
    pub const fn new() -> Self {
        Self(Display::new())
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for PpaIntDisp {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PpaIntDisp {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PPA - MAGIC MAG Light
/// Output: 0 if light is off, 1 if light is on
pub struct PpaMagicMis(Led<29408, 32768, 15>);
impl PpaMagicMis {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for PpaMagicMis {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PpaMagicMis {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PPA - MAGIC Light
/// Output: 0 if light is off, 1 if light is on
pub struct PpaMagicP(Led<29408, 16384, 14>);
impl PpaMagicP {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for PpaMagicP {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PpaMagicP {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PPA - MAN Light
/// Output: 0 if light is off, 1 if light is on
pub struct PpaMan(Led<29408, 8192, 13>);
impl PpaMan {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for PpaMan {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PpaMan {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PPA Quantity Display
/// Output: O - PPA Quantity Display
pub struct PpaQtyDisp(Display<29448, 2>);
impl PpaQtyDisp {
    pub const fn new() -> Self {
        Self(Display::new())
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for PpaQtyDisp {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PpaQtyDisp {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PPA - S530 MIS Light
/// Output: 0 if light is off, 1 if light is on
pub struct PpaS530Mis(Led<29408, 2048, 11>);
impl PpaS530Mis {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for PpaS530Mis {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PpaS530Mis {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PPA - S530 P Light
/// Output: 0 if light is off, 1 if light is on
pub struct PpaS530P(Led<29408, 1024, 10>);
impl PpaS530P {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for PpaS530P {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PpaS530P {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PpaTestSwValue {
    Select1,
    Select2,
    Select3,
}
pub type PpaTestSwValueCallback<'a> = &'a mut dyn FnMut(PpaTestSwValue);
impl core::convert::TryFrom<u16> for PpaTestSwValue {
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

/// I - PPA - Test/Pres Switch
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct PpaTestSw(Selector<29404, 6144, 11>);
impl PpaTestSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for PpaTestSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PpaTestSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - CAB - Cabine Press Needle
/// Output: gauge position
pub struct PressNeedle(AnalogGauge<29368, 65535, 0>);
impl PressNeedle {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for PressNeedle {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PressNeedle {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PSV - Rudder Position display
/// Output: gauge position
pub struct PsvDerive(AnalogGauge<29360, 65535, 0>);
impl PsvDerive {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for PsvDerive {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PsvDerive {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PSV - Right Ext Elevon Position display
/// Output: gauge position
pub struct PsvElDExt(AnalogGauge<29362, 65535, 0>);
impl PsvElDExt {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for PsvElDExt {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PsvElDExt {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PSV - Right Int Elevon Position display
/// Output: gauge position
pub struct PsvElDInt(AnalogGauge<29364, 65535, 0>);
impl PsvElDInt {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for PsvElDInt {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PsvElDInt {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PSV - Left Ext Elevon Position display
/// Output: gauge position
pub struct PsvElGExt(AnalogGauge<29356, 65535, 0>);
impl PsvElGExt {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for PsvElGExt {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PsvElGExt {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PSV - Left Int Elevon Position display
/// Output: gauge position
pub struct PsvElGInt(AnalogGauge<29358, 65535, 0>);
impl PsvElGInt {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for PsvElGInt {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for PsvElGInt {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QraSwValue {
    Select1,
    Select2,
}
pub type QraSwValueCallback<'a> = &'a mut dyn FnMut(QraSwValue);
impl core::convert::TryFrom<u16> for QraSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Alert Network (QRA)
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct QraSw(Selector<29456, 16, 4>);
impl QraSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for QraSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for QraSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - RAD - A Button Light
/// Output: 0 if light is off, 1 if light is on
pub struct RadA(Led<29442, 256, 8>);
impl RadA {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for RadA {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadA {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RadAzSelValue {
    Select1,
    Select2,
    Select3,
}
pub type RadAzSelValueCallback<'a> = &'a mut dyn FnMut(RadAzSelValue);
impl core::convert::TryFrom<u16> for RadAzSelValue {
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

/// I - Radar Azimuth Selector
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct RadAzSel(Selector<29416, 49152, 14>);
impl RadAzSel {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RadAzSel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadAzSel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// I - Radar Contrast Switch
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct RadContrastSw(LimitedDial<29452, 65535, 0>);
impl RadContrastSw {
    pub const fn new() -> Self {
        Self(LimitedDial::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RadContrastSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadContrastSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - RAD - DEC Button Light
/// Output: 0 if light is off, 1 if light is on
pub struct RadDec(Led<29442, 512, 9>);
impl RadDec {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for RadDec {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadDec {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RadDopRejSwValue {
    Select1,
    Select2,
}
pub type RadDopRejSwValueCallback<'a> = &'a mut dyn FnMut(RadDopRejSwValue);
impl core::convert::TryFrom<u16> for RadDopRejSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Radar Doppler Reject Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct RadDopRejSw(Selector<29438, 2048, 11>);
impl RadDopRejSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RadDopRejSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadDopRejSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// I - Radar Gain Dial
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct RadGainDial(LimitedDial<29454, 65535, 0>);
impl RadGainDial {
    pub const fn new() -> Self {
        Self(LimitedDial::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RadGainDial {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadGainDial {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RadGridSelSwValue {
    Select1,
    Select2,
}
pub type RadGridSelSwValueCallback<'a> = &'a mut dyn FnMut(RadGridSelSwValue);
impl core::convert::TryFrom<u16> for RadGridSelSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Radar Grid Selector Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct RadGridSelSw(Selector<29438, 32768, 15>);
impl RadGridSelSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RadGridSelSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadGridSelSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RadIffCode1SelValue {
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
pub type RadIffCode1SelValueCallback<'a> = &'a mut dyn FnMut(RadIffCode1SelValue);
impl core::convert::TryFrom<u16> for RadIffCode1SelValue {
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

/// I - IFF_INT -  Code-1 Selector
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct RadIffCode1Sel(Selector<29330, 61440, 12>);
impl RadIffCode1Sel {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RadIffCode1Sel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadIffCode1Sel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RadIffCode2SelValue {
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
pub type RadIffCode2SelValueCallback<'a> = &'a mut dyn FnMut(RadIffCode2SelValue);
impl core::convert::TryFrom<u16> for RadIffCode2SelValue {
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

/// I - IFF_INT -  Code-2 Selector
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct RadIffCode2Sel(Selector<29330, 3840, 8>);
impl RadIffCode2Sel {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RadIffCode2Sel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadIffCode2Sel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RadIffCode3SelValue {
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
pub type RadIffCode3SelValueCallback<'a> = &'a mut dyn FnMut(RadIffCode3SelValue);
impl core::convert::TryFrom<u16> for RadIffCode3SelValue {
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

/// I - IFF_INT -  Code-3 Selector
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct RadIffCode3Sel(Selector<29330, 240, 4>);
impl RadIffCode3Sel {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RadIffCode3Sel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadIffCode3Sel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RadIffCode4SelValue {
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
pub type RadIffCode4SelValueCallback<'a> = &'a mut dyn FnMut(RadIffCode4SelValue);
impl core::convert::TryFrom<u16> for RadIffCode4SelValue {
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

/// I - IFF_INT -  Code-4 Selector
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct RadIffCode4Sel(Selector<29330, 15, 0>);
impl RadIffCode4Sel {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RadIffCode4Sel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadIffCode4Sel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RadIffLRSelValue {
    Select1,
    Select2,
}
pub type RadIffLRSelValueCallback<'a> = &'a mut dyn FnMut(RadIffLRSelValue);
impl core::convert::TryFrom<u16> for RadIffLRSelValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - IFF_INT -  L/R Selector
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct RadIffLRSel(Selector<29300, 16384, 14>);
impl RadIffLRSel {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RadIffLRSel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadIffLRSel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RadIffModeSwValue {
    Select1,
    Select2,
    Select3,
    Select4,
    Select5,
    Select6,
}
pub type RadIffModeSwValueCallback<'a> = &'a mut dyn FnMut(RadIffModeSwValue);
impl core::convert::TryFrom<u16> for RadIffModeSwValue {
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

/// I - IFF_INT -  Mode Switch
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct RadIffModeSw(Selector<29300, 14336, 11>);
impl RadIffModeSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RadIffModeSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadIffModeSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RadIffPowSwValue {
    Select1,
    Select2,
    Select3,
}
pub type RadIffPowSwValueCallback<'a> = &'a mut dyn FnMut(RadIffPowSwValue);
impl core::convert::TryFrom<u16> for RadIffPowSwValue {
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

/// I - IFF_INT -  Power Switch
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct RadIffPowSw(Selector<29280, 49152, 14>);
impl RadIffPowSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RadIffPowSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadIffPowSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RadIllumSwValue {
    Select1,
    Select2,
    Select3,
}
pub type RadIllumSwValueCallback<'a> = &'a mut dyn FnMut(RadIllumSwValue);
impl core::convert::TryFrom<u16> for RadIllumSwValue {
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

/// I - Radar Illumination Switch
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct RadIllumSw(Selector<29416, 768, 8>);
impl RadIllumSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RadIllumSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadIllumSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RadPrfSwValue {
    Select1,
    Select2,
    Select3,
}
pub type RadPrfSwValueCallback<'a> = &'a mut dyn FnMut(RadPrfSwValue);
impl core::convert::TryFrom<u16> for RadPrfSwValue {
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

/// I - Radar PRF Switch
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct RadPrfSw(Selector<29440, 6144, 11>);
impl RadPrfSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RadPrfSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadPrfSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - RAD - PSIC Button Light
/// Output: 0 if light is off, 1 if light is on
pub struct RadPsic(Led<29442, 2048, 11>);
impl RadPsic {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for RadPsic {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadPsic {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RadPwrSelValue {
    Select1,
    Select2,
    Select3,
    Select4,
}
pub type RadPwrSelValueCallback<'a> = &'a mut dyn FnMut(RadPwrSelValue);
impl core::convert::TryFrom<u16> for RadPwrSelValue {
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

/// I - Radar Power Selector
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct RadPwrSel(Selector<29416, 3072, 10>);
impl RadPwrSel {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RadPwrSel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadPwrSel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RadRaltPwrSwValue {
    Select1,
    Select2,
    Select3,
}
pub type RadRaltPwrSwValueCallback<'a> = &'a mut dyn FnMut(RadRaltPwrSwValue);
impl core::convert::TryFrom<u16> for RadRaltPwrSwValue {
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

/// I - HUD - Radar Altimeter Power Switch
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct RadRaltPwrSw(Selector<29500, 96, 5>);
impl RadRaltPwrSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RadRaltPwrSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadRaltPwrSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RadRearmBtnValue {
    Select1,
    Select2,
}
pub type RadRearmBtnValueCallback<'a> = &'a mut dyn FnMut(RadRearmBtnValue);
impl core::convert::TryFrom<u16> for RadRearmBtnValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Radar Rearm Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct RadRearmBtn(Selector<29438, 1024, 10>);
impl RadRearmBtn {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RadRearmBtn {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadRearmBtn {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RadRngSelSwValue {
    Select1,
    Select2,
    Select3,
}
pub type RadRngSelSwValueCallback<'a> = &'a mut dyn FnMut(RadRngSelSwValue);
impl core::convert::TryFrom<u16> for RadRngSelSwValue {
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

/// I - Radar Range Selector Switch
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct RadRngSelSw(Selector<29440, 24576, 13>);
impl RadRngSelSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RadRngSelSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadRngSelSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RadScanLinesSelValue {
    Select1,
    Select2,
    Select3,
}
pub type RadScanLinesSelValueCallback<'a> = &'a mut dyn FnMut(RadScanLinesSelValue);
impl core::convert::TryFrom<u16> for RadScanLinesSelValue {
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

/// I - Radar Scan Lines Selector
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct RadScanLinesSel(Selector<29416, 12288, 12>);
impl RadScanLinesSel {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RadScanLinesSel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadScanLinesSel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RadTestBtnValue {
    Select1,
    Select2,
}
pub type RadTestBtnValueCallback<'a> = &'a mut dyn FnMut(RadTestBtnValue);
impl core::convert::TryFrom<u16> for RadTestBtnValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Radar Test Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct RadTestBtn(Selector<29438, 512, 9>);
impl RadTestBtn {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RadTestBtn {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadTestBtn {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - RAD - VAL Button Light
/// Output: 0 if light is off, 1 if light is on
pub struct RadVal(Led<29440, 32768, 15>);
impl RadVal {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for RadVal {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadVal {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - RAD - VISU Button Light
/// Output: 0 if light is off, 1 if light is on
pub struct RadVisu(Led<29442, 1024, 10>);
impl RadVisu {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for RadVisu {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadVisu {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RadWowEmAutSwValue {
    Select1,
    Select2,
}
pub type RadWowEmAutSwValueCallback<'a> = &'a mut dyn FnMut(RadWowEmAutSwValue);
impl core::convert::TryFrom<u16> for RadWowEmAutSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Radar WOW Emitter Authorize Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct RadWowEmAutSw(Selector<29438, 256, 8>);
impl RadWowEmAutSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RadWowEmAutSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RadWowEmAutSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// I - Red Flood Lights Knob
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct RedFloodLgtKnob(LimitedDial<29342, 65535, 0>);
impl RedFloodLgtKnob {
    pub const fn new() -> Self {
        Self(LimitedDial::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RedFloodLgtKnob {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RedFloodLgtKnob {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// I - Refuel Lights Brightness Knob
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct RefuelLgtBrightKnob(LimitedDial<29278, 65535, 0>);
impl RefuelLgtBrightKnob {
    pub const fn new() -> Self {
        Self(LimitedDial::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RefuelLgtBrightKnob {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RefuelLgtBrightKnob {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - ENG - x1 RPM display
/// Output: gauge position
pub struct RpmOnes(AnalogGauge<29272, 65535, 0>);
impl RpmOnes {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RpmOnes {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RpmOnes {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - ENG - x10 RPM display
/// Output: gauge position
pub struct RpmTens(AnalogGauge<29270, 65535, 0>);
impl RpmTens {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RpmTens {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RpmTens {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - RWR - BR Indicator Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct RwrBr(Led<29456, 256, 8>);
impl RwrBr {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for RwrBr {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RwrBr {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - RWR - D2M Indicator Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct RwrD2m(Led<29456, 1024, 10>);
impl RwrD2m {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for RwrD2m {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RwrD2m {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - RWR - DA Indicator Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct RwrDa(Led<29456, 512, 9>);
impl RwrDa {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for RwrDa {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RwrDa {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// I - RWR Light Brightnes Control
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct RwrLgtBrightCtrl(LimitedDial<29462, 65535, 0>);
impl RwrLgtBrightCtrl {
    pub const fn new() -> Self {
        Self(LimitedDial::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RwrLgtBrightCtrl {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RwrLgtBrightCtrl {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - RWR - LL Indicator Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct RwrLl(Led<29456, 2048, 11>);
impl RwrLl {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for RwrLl {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RwrLl {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RwrPwrSwValue {
    Select1,
    Select2,
    Select3,
}
pub type RwrPwrSwValueCallback<'a> = &'a mut dyn FnMut(RwrPwrSwValue);
impl core::convert::TryFrom<u16> for RwrPwrSwValue {
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

/// I - RWR Power Switch
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct RwrPwrSw(Selector<29274, 96, 5>);
impl RwrPwrSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for RwrPwrSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RwrPwrSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - RWR - V Indicator Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct RwrV(Led<29456, 128, 7>);
impl RwrV {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for RwrV {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for RwrV {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum S530StandbyValue {
    Select1,
    Select2,
}
pub type S530StandbyValueCallback<'a> = &'a mut dyn FnMut(S530StandbyValue);
impl core::convert::TryFrom<u16> for S530StandbyValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PPA - S530 Standby Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct S530Standby(Selector<29400, 16384, 14>);
impl S530Standby {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for S530Standby {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for S530Standby {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SafeGndSwValue {
    Select1,
    Select2,
}
pub type SafeGndSwValueCallback<'a> = &'a mut dyn FnMut(SafeGndSwValue);
impl core::convert::TryFrom<u16> for SafeGndSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - LDG - Safety Ground Override
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct SafeGndSw(Selector<29352, 32768, 15>);
impl SafeGndSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for SafeGndSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SafeGndSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SbAdiCageSwValue {
    Select1,
    Select2,
}
pub type SbAdiCageSwValueCallback<'a> = &'a mut dyn FnMut(SbAdiCageSwValue);
impl core::convert::TryFrom<u16> for SbAdiCageSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - BKADI - UNCAGE
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct SbAdiCageSw(Selector<29240, 8, 3>);
impl SbAdiCageSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for SbAdiCageSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SbAdiCageSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - BKADI - Flag
/// Output: gauge position
pub struct SbAdiFlag(AnalogGauge<29248, 65535, 0>);
impl SbAdiFlag {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for SbAdiFlag {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SbAdiFlag {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - BKADI - Pitch Position
/// Output: gauge position
pub struct SbAdiPitch(AnalogGauge<29246, 65535, 0>);
impl SbAdiPitch {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for SbAdiPitch {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SbAdiPitch {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - BKADI - Roll Position
/// Output: gauge position
pub struct SbAdiRoll(AnalogGauge<29244, 65535, 0>);
impl SbAdiRoll {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for SbAdiRoll {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SbAdiRoll {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// I - BKADI - Set
/// Input: turn the dial left or right
/// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
pub struct SbAdiRot(AnalogDial<29242, 65535, 0>);
impl SbAdiRot {
    pub const fn new() -> Self {
        Self(AnalogDial::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for SbAdiRot {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SbAdiRot {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SeatAdjustSwValue {
    Select1,
    Select2,
    Select3,
}
pub type SeatAdjustSwValueCallback<'a> = &'a mut dyn FnMut(SeatAdjustSwValue);
impl core::convert::TryFrom<u16> for SeatAdjustSwValue {
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

/// I - Seat Adjustment Switch
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct SeatAdjustSw(Selector<29366, 24576, 13>);
impl SeatAdjustSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for SeatAdjustSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SeatAdjustSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecHuileSwValue {
    Select1,
    Select2,
}
pub type SecHuileSwValueCallback<'a> = &'a mut dyn FnMut(SecHuileSwValue);
impl core::convert::TryFrom<u16> for SecHuileSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Secours Huile Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct SecHuileSw(Selector<29352, 16, 4>);
impl SecHuileSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for SecHuileSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SecHuileSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecHuileSwCoverValue {
    Select1,
    Select2,
}
pub type SecHuileSwCoverValueCallback<'a> = &'a mut dyn FnMut(SecHuileSwCoverValue);
impl core::convert::TryFrom<u16> for SecHuileSwCoverValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Secours Huile Switch Cover
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct SecHuileSwCover(Selector<29352, 8, 3>);
impl SecHuileSwCover {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for SecHuileSwCover {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SecHuileSwCover {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelAltHoldBtnValue {
    Select1,
    Select2,
}
pub type SelAltHoldBtnValueCallback<'a> = &'a mut dyn FnMut(SelAltHoldBtnValue);
impl core::convert::TryFrom<u16> for SelAltHoldBtnValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Selected Altitude Hold Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct SelAltHoldBtn(Selector<29184, 512, 9>);
impl SelAltHoldBtn {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for SelAltHoldBtn {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SelAltHoldBtn {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelJettSwValue {
    Select1,
    Select2,
}
pub type SelJettSwValueCallback<'a> = &'a mut dyn FnMut(SelJettSwValue);
impl core::convert::TryFrom<u16> for SelJettSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PCA - Selective Jettison Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct SelJettSw(Selector<29374, 128, 7>);
impl SelJettSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for SelJettSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SelJettSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SelJettSwCovValue {
    Select1,
    Select2,
}
pub type SelJettSwCovValueCallback<'a> = &'a mut dyn FnMut(SelJettSwCovValue);
impl core::convert::TryFrom<u16> for SelJettSwCovValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PCA - Selective Jettison Switch Cover
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct SelJettSwCov(Selector<29374, 64, 6>);
impl SelJettSwCov {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for SelJettSwCov {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SelJettSwCov {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SerpamRecSwValue {
    Select1,
    Select2,
}
pub type SerpamRecSwValueCallback<'a> = &'a mut dyn FnMut(SerpamRecSwValue);
impl core::convert::TryFrom<u16> for SerpamRecSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - SERPAM Recorder Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct SerpamRecSw(Selector<29352, 2048, 11>);
impl SerpamRecSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for SerpamRecSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SerpamRecSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlatsOpSwValue {
    Select1,
    Select2,
    Select3,
}
pub type SlatsOpSwValueCallback<'a> = &'a mut dyn FnMut(SlatsOpSwValue);
impl core::convert::TryFrom<u16> for SlatsOpSwValue {
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

/// I - Slats Operation Switch (BECS)
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct SlatsOpSw(Selector<29184, 48, 4>);
impl SlatsOpSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for SlatsOpSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SlatsOpSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - SPEED - Speed Needle
/// Output: gauge position
pub struct SpeedKtsNeed(AnalogGauge<29464, 65535, 0>);
impl SpeedKtsNeed {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for SpeedKtsNeed {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SpeedKtsNeed {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - SPEED - Mach
/// Output: gauge position
pub struct SpeedMach(AnalogGauge<29466, 65535, 0>);
impl SpeedMach {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for SpeedMach {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for SpeedMach {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StartFuelPumpSwValue {
    Select1,
    Select2,
}
pub type StartFuelPumpSwValueCallback<'a> = &'a mut dyn FnMut(StartFuelPumpSwValue);
impl core::convert::TryFrom<u16> for StartFuelPumpSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Starter Fuel Pump Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct StartFuelPumpSw(Selector<29262, 2048, 11>);
impl StartFuelPumpSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for StartFuelPumpSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for StartFuelPumpSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - ENG - Tt7 Needle
/// Output: gauge position
pub struct T7Needle(AnalogGauge<29268, 65535, 0>);
impl T7Needle {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for T7Needle {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for T7Needle {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// I - TACAN Channel 10 Selector
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct TacCh10Sel(Selector<29456, 61440, 12>);
impl TacCh10Sel {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for TacCh10Sel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for TacCh10Sel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TacCh1SelValue {
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
pub type TacCh1SelValueCallback<'a> = &'a mut dyn FnMut(TacCh1SelValue);
impl core::convert::TryFrom<u16> for TacCh1SelValue {
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

/// I - TACAN Channel 1 Selector
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct TacCh1Sel(Selector<29468, 120, 3>);
impl TacCh1Sel {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for TacCh1Sel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for TacCh1Sel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TacModeSelValue {
    Select1,
    Select2,
    Select3,
    Select4,
}
pub type TacModeSelValueCallback<'a> = &'a mut dyn FnMut(TacModeSelValue);
impl core::convert::TryFrom<u16> for TacModeSelValue {
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

/// I - TACAN Mode Select
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct TacModeSel(Selector<29468, 3, 0>);
impl TacModeSel {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for TacModeSel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for TacModeSel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// I - TACAN Volume Knob
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct TacVolKnob(LimitedDial<29226, 65535, 0>);
impl TacVolKnob {
    pub const fn new() -> Self {
        Self(LimitedDial::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for TacVolKnob {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for TacVolKnob {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TacXYSelValue {
    Select1,
    Select2,
}
pub type TacXYSelValueCallback<'a> = &'a mut dyn FnMut(TacXYSelValue);
impl core::convert::TryFrom<u16> for TacXYSelValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - TACAN X/Y Select
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct TacXYSel(Selector<29468, 4, 2>);
impl TacXYSel {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for TacXYSel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for TacXYSel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// I - TB APP Volume Knob
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct TbAppVolKnob(LimitedDial<29230, 65535, 0>);
impl TbAppVolKnob {
    pub const fn new() -> Self {
        Self(LimitedDial::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for TbAppVolKnob {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for TbAppVolKnob {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TdcModeSwValue {
    Select1,
    Select2,
}
pub type TdcModeSwValueCallback<'a> = &'a mut dyn FnMut(TdcModeSwValue);
impl core::convert::TryFrom<u16> for TdcModeSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - TDC Mode Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct TdcModeSw(Selector<29440, 1024, 10>);
impl TdcModeSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for TdcModeSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for TdcModeSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - TEST - Red Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct TestRouge(Led<29468, 16384, 14>);
impl TestRouge {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for TestRouge {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for TestRouge {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - TEST - Green Indicator Light
/// Output: 0 if light is off, 1 if light is on
pub struct TestVert(Led<29468, 32768, 15>);
impl TestVert {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for TestVert {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for TestVert {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TgtAgeValue {
    Select1,
    Select2,
    Select3,
}
pub type TgtAgeValueCallback<'a> = &'a mut dyn FnMut(TgtAgeValue);
impl core::convert::TryFrom<u16> for TgtAgeValue {
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

/// I - VTB - Target Age (T)
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct TgtAge(Selector<29492, 12288, 12>);
impl TgtAge {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for TgtAge {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for TgtAge {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TgtAltValue {
    Select1,
    Select2,
    Select3,
}
pub type TgtAltValueCallback<'a> = &'a mut dyn FnMut(TgtAltValue);
impl core::convert::TryFrom<u16> for TgtAltValue {
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

/// I - VTB - Target Altitude (Z)
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct TgtAlt(Selector<29492, 768, 8>);
impl TgtAlt {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for TgtAlt {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for TgtAlt {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TgtBearBeValue {
    Select1,
    Select2,
    Select3,
}
pub type TgtBearBeValueCallback<'a> = &'a mut dyn FnMut(TgtBearBeValue);
impl core::convert::TryFrom<u16> for TgtBearBeValue {
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

/// I - VTB - Target Bearing from Bullseye (Theta)
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct TgtBearBe(Selector<29492, 48, 4>);
impl TgtBearBe {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for TgtBearBe {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for TgtBearBe {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TgtDataMnlEntryValue {
    Select1,
    Select2,
    Select3,
}
pub type TgtDataMnlEntryValueCallback<'a> = &'a mut dyn FnMut(TgtDataMnlEntryValue);
impl core::convert::TryFrom<u16> for TgtDataMnlEntryValue {
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

/// I - VTB - Target Data Manual Entry Begin/End (DEB/FIN)
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct TgtDataMnlEntry(Selector<29490, 49152, 14>);
impl TgtDataMnlEntry {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for TgtDataMnlEntry {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for TgtDataMnlEntry {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TgtHdValue {
    Select1,
    Select2,
    Select3,
}
pub type TgtHdValueCallback<'a> = &'a mut dyn FnMut(TgtHdValue);
impl core::convert::TryFrom<u16> for TgtHdValue {
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

/// I - VTB - Target Heading ( C )
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct TgtHd(Selector<29492, 192, 6>);
impl TgtHd {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for TgtHd {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for TgtHd {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TgtMachNumValue {
    Select1,
    Select2,
    Select3,
}
pub type TgtMachNumValueCallback<'a> = &'a mut dyn FnMut(TgtMachNumValue);
impl core::convert::TryFrom<u16> for TgtMachNumValue {
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

/// I - VTB - Target Mach Number (M)
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct TgtMachNum(Selector<29492, 3072, 10>);
impl TgtMachNum {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for TgtMachNum {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for TgtMachNum {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TgtMemTimeSelSwValue {
    Select1,
    Select2,
}
pub type TgtMemTimeSelSwValueCallback<'a> = &'a mut dyn FnMut(TgtMemTimeSelSwValue);
impl core::convert::TryFrom<u16> for TgtMemTimeSelSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - Target Memory Time Selector Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct TgtMemTimeSelSw(Selector<29440, 256, 8>);
impl TgtMemTimeSelSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for TgtMemTimeSelSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for TgtMemTimeSelSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TgtRngBeValue {
    Select1,
    Select2,
    Select3,
}
pub type TgtRngBeValueCallback<'a> = &'a mut dyn FnMut(TgtRngBeValue);
impl core::convert::TryFrom<u16> for TgtRngBeValue {
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

/// I - VTB - Target Range from Bullseye (Rho)
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct TgtRngBe(Selector<29492, 12, 2>);
impl TgtRngBe {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for TgtRngBe {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for TgtRngBe {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// I - HUD - Target Wingspan Knob
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct TgtWingspanKnob(LimitedDial<29498, 65535, 0>);
impl TgtWingspanKnob {
    pub const fn new() -> Self {
        Self(LimitedDial::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for TgtWingspanKnob {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for TgtWingspanKnob {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UhfCdeSwValue {
    Select1,
    Select2,
}
pub type UhfCdeSwValueCallback<'a> = &'a mut dyn FnMut(UhfCdeSwValue);
impl core::convert::TryFrom<u16> for UhfCdeSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - UHF - CDE Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UhfCdeSw(Selector<29472, 1024, 10>);
impl UhfCdeSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UhfCdeSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UhfCdeSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UhfEA2SwValue {
    Select1,
    Select2,
    Select3,
}
pub type UhfEA2SwValueCallback<'a> = &'a mut dyn FnMut(UhfEA2SwValue);
impl core::convert::TryFrom<u16> for UhfEA2SwValue {
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

/// I - UHF - E+A2 Switch
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct UhfEA2Sw(Selector<29472, 12288, 12>);
impl UhfEA2Sw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UhfEA2Sw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UhfEA2Sw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - UHF - Frequency Report Display
/// Output: O - UHF - Frequency Report Display
pub struct UhfFrequency(Display<29484, 5>);
impl UhfFrequency {
    pub const fn new() -> Self {
        Self(Display::new())
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for UhfFrequency {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UhfFrequency {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UhfModeSwValue {
    Select1,
    Select2,
    Select3,
    Select4,
    Select5,
}
pub type UhfModeSwValueCallback<'a> = &'a mut dyn FnMut(UhfModeSwValue);
impl core::convert::TryFrom<u16> for UhfModeSwValue {
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

/// I - UHF - Mode Selector
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct UhfModeSw(Selector<29472, 224, 5>);
impl UhfModeSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UhfModeSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UhfModeSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - UHF - PRESET Display
/// Output: gauge position
pub struct UhfPreset(AnalogGauge<29482, 65535, 0>);
impl UhfPreset {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UhfPreset {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UhfPreset {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Preset Knob UHF
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct UhfPresetKnobInt(Selector<29478, 7936, 8>);
impl UhfPresetKnobInt {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UhfPresetKnobInt {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UhfPresetKnobInt {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// Preset Knob UHF
/// Input: switch to previous or next state
/// Input: set position
/// Output: possible values: " 1" " 1" " 2" " 3" " 4" " 5" " 6" " 7" " 8" " 9" "10" "11" "12" "13" "14" "15" "16" "17" "18" "19" "20"
pub struct UhfPresetKnobStr(Display<29480, 2>);
impl UhfPresetKnobStr {
    pub const fn new() -> Self {
        Self(Display::new())
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for UhfPresetKnobStr {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UhfPresetKnobStr {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UhfPwr5w25wSwValue {
    Select1,
    Select2,
}
pub type UhfPwr5w25wSwValueCallback<'a> = &'a mut dyn FnMut(UhfPwr5w25wSwValue);
impl core::convert::TryFrom<u16> for UhfPwr5w25wSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - UHF - Power 5W/25W Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UhfPwr5w25wSw(Selector<29472, 256, 8>);
impl UhfPwr5w25wSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UhfPwr5w25wSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UhfPwr5w25wSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// I - UHF - Radio Volume Knob
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct UhfRadioVolKnob(LimitedDial<29234, 65535, 0>);
impl UhfRadioVolKnob {
    pub const fn new() -> Self {
        Self(LimitedDial::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UhfRadioVolKnob {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UhfRadioVolKnob {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UHF Repeater COM1 Light (green)
/// Output: 0 if light is off, 1 if light is on
pub struct UhfRepL1(Led<29500, 2048, 11>);
impl UhfRepL1 {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for UhfRepL1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UhfRepL1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UHF Repeater COM2 Light (green)
/// Output: 0 if light is off, 1 if light is on
pub struct UhfRepL2(Led<29500, 4096, 12>);
impl UhfRepL2 {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for UhfRepL2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UhfRepL2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UHF Secure Channel Light (green)
/// Output: 0 if light is off, 1 if light is on
pub struct UhfSecureL(Led<29500, 16384, 14>);
impl UhfSecureL {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for UhfSecureL {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UhfSecureL {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UhfSilSwValue {
    Select1,
    Select2,
}
pub type UhfSilSwValueCallback<'a> = &'a mut dyn FnMut(UhfSilSwValue);
impl core::convert::TryFrom<u16> for UhfSilSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - UHF - SIL Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UhfSilSw(Selector<29472, 512, 9>);
impl UhfSilSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UhfSilSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UhfSilSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// UHF TEST Light (yellow)
/// Output: 0 if light is off, 1 if light is on
pub struct UhfTestL(Led<29500, 8192, 13>);
impl UhfTestL {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for UhfTestL {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UhfTestL {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UhfTestSwValue {
    Select1,
    Select2,
}
pub type UhfTestSwValueCallback<'a> = &'a mut dyn FnMut(UhfTestSwValue);
impl core::convert::TryFrom<u16> for UhfTestSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - UHF - TEST Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UhfTestSw(Selector<29472, 2048, 11>);
impl UhfTestSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for UhfTestSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for UhfTestSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValDataEntryValue {
    Select1,
    Select2,
}
pub type ValDataEntryValueCallback<'a> = &'a mut dyn FnMut(ValDataEntryValue);
impl core::convert::TryFrom<u16> for ValDataEntryValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PCN - Validate Data Entry (VAL)
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct ValDataEntry(Selector<29384, 1024, 10>);
impl ValDataEntry {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for ValDataEntry {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for ValDataEntry {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - VVI - Needle
/// Output: gauge position
pub struct VarioNeedle(AnalogGauge<29502, 65535, 0>);
impl VarioNeedle {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for VarioNeedle {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VarioNeedle {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VentSelSwValue {
    Select1,
    Select2,
    Select3,
}
pub type VentSelSwValueCallback<'a> = &'a mut dyn FnMut(VentSelSwValue);
impl core::convert::TryFrom<u16> for VentSelSwValue {
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

/// I - Ventilation Selector Switch
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct VentSelSw(Selector<29274, 6, 1>);
impl VentSelSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for VentSelSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VentSelSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vhf0Value {
    Select1,
    Select2,
}
pub type Vhf0ValueCallback<'a> = &'a mut dyn FnMut(Vhf0Value);
impl core::convert::TryFrom<u16> for Vhf0Value {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - VHF - 0 Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Vhf0(Selector<29472, 8, 3>);
impl Vhf0 {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Vhf0 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Vhf0 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vhf1ReadValue {
    Select1,
    Select2,
}
pub type Vhf1ReadValueCallback<'a> = &'a mut dyn FnMut(Vhf1ReadValue);
impl core::convert::TryFrom<u16> for Vhf1ReadValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - VHF - 1/READ Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Vhf1Read(Selector<29470, 1024, 10>);
impl Vhf1Read {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Vhf1Read {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Vhf1Read {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vhf2SqlValue {
    Select1,
    Select2,
}
pub type Vhf2SqlValueCallback<'a> = &'a mut dyn FnMut(Vhf2SqlValue);
impl core::convert::TryFrom<u16> for Vhf2SqlValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - VHF - 2/SQL Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Vhf2Sql(Selector<29470, 2048, 11>);
impl Vhf2Sql {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Vhf2Sql {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Vhf2Sql {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vhf3GrValue {
    Select1,
    Select2,
}
pub type Vhf3GrValueCallback<'a> = &'a mut dyn FnMut(Vhf3GrValue);
impl core::convert::TryFrom<u16> for Vhf3GrValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - VHF - 3/GR Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Vhf3Gr(Selector<29470, 4096, 12>);
impl Vhf3Gr {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Vhf3Gr {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Vhf3Gr {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vhf4Value {
    Select1,
    Select2,
}
pub type Vhf4ValueCallback<'a> = &'a mut dyn FnMut(Vhf4Value);
impl core::convert::TryFrom<u16> for Vhf4Value {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - VHF - 4 Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Vhf4(Selector<29470, 8192, 13>);
impl Vhf4 {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Vhf4 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Vhf4 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vhf520LowValue {
    Select1,
    Select2,
}
pub type Vhf520LowValueCallback<'a> = &'a mut dyn FnMut(Vhf520LowValue);
impl core::convert::TryFrom<u16> for Vhf520LowValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - VHF - 5/20/LOW Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Vhf520Low(Selector<29470, 16384, 14>);
impl Vhf520Low {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Vhf520Low {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Vhf520Low {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vhf6ToneValue {
    Select1,
    Select2,
}
pub type Vhf6ToneValueCallback<'a> = &'a mut dyn FnMut(Vhf6ToneValue);
impl core::convert::TryFrom<u16> for Vhf6ToneValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - VHF - 6/TONE Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Vhf6Tone(Selector<29470, 32768, 15>);
impl Vhf6Tone {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Vhf6Tone {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Vhf6Tone {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vhf7Value {
    Select1,
    Select2,
}
pub type Vhf7ValueCallback<'a> = &'a mut dyn FnMut(Vhf7Value);
impl core::convert::TryFrom<u16> for Vhf7Value {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - VHF - 7 Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Vhf7(Selector<29472, 1, 0>);
impl Vhf7 {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Vhf7 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Vhf7 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vhf8TodValue {
    Select1,
    Select2,
}
pub type Vhf8TodValueCallback<'a> = &'a mut dyn FnMut(Vhf8TodValue);
impl core::convert::TryFrom<u16> for Vhf8TodValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - VHF - 8/TOD Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Vhf8Tod(Selector<29472, 2, 1>);
impl Vhf8Tod {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Vhf8Tod {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Vhf8Tod {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vhf9ZeroValue {
    Select1,
    Select2,
}
pub type Vhf9ZeroValueCallback<'a> = &'a mut dyn FnMut(Vhf9ZeroValue);
impl core::convert::TryFrom<u16> for Vhf9ZeroValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - VHF - 9/ZERO Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct Vhf9Zero(Selector<29472, 4, 2>);
impl Vhf9Zero {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Vhf9Zero {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Vhf9Zero {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// I - VHF - Channel Selector
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct VhfChSel(Selector<29470, 248, 3>);
impl VhfChSel {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for VhfChSel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VhfChSel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VhfConfValue {
    Select1,
    Select2,
}
pub type VhfConfValueCallback<'a> = &'a mut dyn FnMut(VhfConfValue);
impl core::convert::TryFrom<u16> for VhfConfValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - VHF - CONF Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct VhfConf(Selector<29472, 16, 4>);
impl VhfConf {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for VhfConf {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VhfConf {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - VHF - Frequency Report Display
/// Output: O - VHF - Frequency Report Display
pub struct VhfFrequency(Display<29474, 5>);
impl VhfFrequency {
    pub const fn new() -> Self {
        Self(Display::new())
    }
    pub fn value(&self) -> &[u8] {
        self.0.value()
    }
}
impl Default for VhfFrequency {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VhfFrequency {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VhfMemClrValue {
    Select1,
    Select2,
}
pub type VhfMemClrValueCallback<'a> = &'a mut dyn FnMut(VhfMemClrValue);
impl core::convert::TryFrom<u16> for VhfMemClrValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - VHF - MEM/CLR Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct VhfMemClr(Selector<29470, 256, 8>);
impl VhfMemClr {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for VhfMemClr {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VhfMemClr {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VhfModeValue {
    Select1,
    Select2,
    Select3,
    Select4,
    Select5,
    Select6,
    Select7,
}
pub type VhfModeValueCallback<'a> = &'a mut dyn FnMut(VhfModeValue);
impl core::convert::TryFrom<u16> for VhfModeValue {
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

/// I - VHF - MODE Switch
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct VhfMode(Selector<29470, 7, 0>);
impl VhfMode {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for VhfMode {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VhfMode {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VhfVldXfrValue {
    Select1,
    Select2,
}
pub type VhfVldXfrValueCallback<'a> = &'a mut dyn FnMut(VhfVldXfrValue);
impl core::convert::TryFrom<u16> for VhfVldXfrValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - VHF -  VLD/XFR Button
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct VhfVldXfr(Selector<29470, 512, 9>);
impl VhfVldXfr {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for VhfVldXfr {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VhfVldXfr {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// VOR/ILS Drum 0.01
/// Output: gauge position
pub struct Vorils001Drum(AnalogGauge<29512, 65535, 0>);
impl Vorils001Drum {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Vorils001Drum {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Vorils001Drum {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// VOR/ILS Drum 0.1
/// Output: gauge position
pub struct Vorils01Drum(AnalogGauge<29510, 65535, 0>);
impl Vorils01Drum {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Vorils01Drum {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Vorils01Drum {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// VOR/ILS Drum 100
/// Output: gauge position
pub struct Vorils100Drum(AnalogGauge<29504, 65535, 0>);
impl Vorils100Drum {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Vorils100Drum {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Vorils100Drum {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// VOR/ILS Drum 10
/// Output: gauge position
pub struct Vorils10Drum(AnalogGauge<29506, 65535, 0>);
impl Vorils10Drum {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Vorils10Drum {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Vorils10Drum {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// VOR/ILS Drum 1
/// Output: gauge position
pub struct Vorils1Drum(AnalogGauge<29508, 65535, 0>);
impl Vorils1Drum {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Vorils1Drum {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Vorils1Drum {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// I - VOR/ILS Frequency Change Decimal
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct VorilsFreqDecimal(Selector<29490, 31, 0>);
impl VorilsFreqDecimal {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for VorilsFreqDecimal {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VorilsFreqDecimal {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// I - VOR/ILS Frequency Change Whole
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct VorilsFreqWhole(Selector<29488, 3840, 8>);
impl VorilsFreqWhole {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for VorilsFreqWhole {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VorilsFreqWhole {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VorilsPwrDialValue {
    Select1,
    Select2,
}
pub type VorilsPwrDialValueCallback<'a> = &'a mut dyn FnMut(VorilsPwrDialValue);
impl core::convert::TryFrom<u16> for VorilsPwrDialValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - VOR/ILS Power Dial
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct VorilsPwrDial(Selector<29472, 16384, 14>);
impl VorilsPwrDial {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for VorilsPwrDial {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VorilsPwrDial {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VorilsTestDialValue {
    Select1,
    Select2,
}
pub type VorilsTestDialValueCallback<'a> = &'a mut dyn FnMut(VorilsTestDialValue);
impl core::convert::TryFrom<u16> for VorilsTestDialValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - VOR/ILS Test Dial
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct VorilsTestDial(Selector<29472, 32768, 15>);
impl VorilsTestDial {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for VorilsTestDial {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VorilsTestDial {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// I - VOR/ILS Volume Knob
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct VorilsVolKnob(LimitedDial<29224, 65535, 0>);
impl VorilsVolKnob {
    pub const fn new() -> Self {
        Self(LimitedDial::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for VorilsVolKnob {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VorilsVolKnob {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// TACAN 10 Drum
/// Output: gauge position
pub struct Vtac10Drum(AnalogGauge<29516, 65535, 0>);
impl Vtac10Drum {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Vtac10Drum {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Vtac10Drum {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// TACAN 1 Drum
/// Output: gauge position
pub struct Vtac1Drum(AnalogGauge<29518, 65535, 0>);
impl Vtac1Drum {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for Vtac1Drum {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Vtac1Drum {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// TACAN X/Y Drum
/// Output: gauge position
pub struct VtacXYDrum(AnalogGauge<29514, 65535, 0>);
impl VtacXYDrum {
    pub const fn new() -> Self {
        Self(AnalogGauge::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for VtacXYDrum {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VtacXYDrum {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VtbDecValue {
    Select1,
    Select2,
}
pub type VtbDecValueCallback<'a> = &'a mut dyn FnMut(VtbDecValue);
impl core::convert::TryFrom<u16> for VtbDecValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - VTB - Declutter (ALLEG)
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct VtbDec(Selector<29478, 16384, 14>);
impl VtbDec {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for VtbDec {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VtbDec {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VtbDispBrightValue {
    Select1,
    Select2,
    Select3,
    Select4,
    Select5,
    Select6,
    Select7,
    Select8,
}
pub type VtbDispBrightValueCallback<'a> = &'a mut dyn FnMut(VtbDispBrightValue);
impl core::convert::TryFrom<u16> for VtbDispBrightValue {
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

/// I - VTB - Display Brightness (LUM CAV)
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct VtbDispBright(Selector<29490, 14336, 11>);
impl VtbDispBright {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for VtbDispBright {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VtbDispBright {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VtbDispContrastValue {
    Select1,
    Select2,
    Select3,
    Select4,
    Select5,
    Select6,
    Select7,
    Select8,
}
pub type VtbDispContrastValueCallback<'a> = &'a mut dyn FnMut(VtbDispContrastValue);
impl core::convert::TryFrom<u16> for VtbDispContrastValue {
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

/// I - VTB - Display Contrast (CONTRAST)
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct VtbDispContrast(Selector<29490, 1792, 8>);
impl VtbDispContrast {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for VtbDispContrast {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VtbDispContrast {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VtbIconsBrightValue {
    Select1,
    Select2,
    Select3,
    Select4,
    Select5,
    Select6,
    Select7,
    Select8,
}
pub type VtbIconsBrightValueCallback<'a> = &'a mut dyn FnMut(VtbIconsBrightValue);
impl core::convert::TryFrom<u16> for VtbIconsBrightValue {
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

/// I - VTB - Icons and Rulers Brightness (MRQ)
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct VtbIconsBright(Selector<29488, 28672, 12>);
impl VtbIconsBright {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for VtbIconsBright {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VtbIconsBright {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VtbOrSelValue {
    Select1,
    Select2,
}
pub type VtbOrSelValueCallback<'a> = &'a mut dyn FnMut(VtbOrSelValue);
impl core::convert::TryFrom<u16> for VtbOrSelValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - VTB - Orientation Selector (Inop) (CADR)
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct VtbOrSel(Selector<29478, 32768, 15>);
impl VtbOrSel {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for VtbOrSel {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VtbOrSel {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VtbPwrSwValue {
    Select1,
    Select2,
}
pub type VtbPwrSwValueCallback<'a> = &'a mut dyn FnMut(VtbPwrSwValue);
impl core::convert::TryFrom<u16> for VtbPwrSwValue {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - VTB - Power Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct VtbPwrSw(Selector<29478, 8192, 13>);
impl VtbPwrSw {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for VtbPwrSw {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VtbPwrSw {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VtbVideoBrightValue {
    Select1,
    Select2,
    Select3,
    Select4,
    Select5,
    Select6,
    Select7,
    Select8,
}
pub type VtbVideoBrightValueCallback<'a> = &'a mut dyn FnMut(VtbVideoBrightValue);
impl core::convert::TryFrom<u16> for VtbVideoBrightValue {
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

/// I - VTB - Video Brightness (LUM GEN)
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct VtbVideoBright(Selector<29490, 224, 5>);
impl VtbVideoBright {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for VtbVideoBright {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VtbVideoBright {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// I - U/VHF - Radio Volume Knob
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct VuhfRadioVolKnob(LimitedDial<29236, 65535, 0>);
impl VuhfRadioVolKnob {
    pub const fn new() -> Self {
        Self(LimitedDial::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for VuhfRadioVolKnob {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for VuhfRadioVolKnob {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCA - Weapons Button 1 R Light
/// Output: 0 if light is off, 1 if light is on
pub struct Weapon1R(Led<29376, 32, 5>);
impl Weapon1R {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Weapon1R {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Weapon1R {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCA - Weapons Button 1 S Light
/// Output: 0 if light is off, 1 if light is on
pub struct Weapon1S(Led<29376, 16, 4>);
impl Weapon1S {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Weapon1S {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Weapon1S {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCA - Weapons Button 2 R Light
/// Output: 0 if light is off, 1 if light is on
pub struct Weapon2R(Led<29376, 128, 7>);
impl Weapon2R {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Weapon2R {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Weapon2R {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCA - Weapons Button 2 S Light
/// Output: 0 if light is off, 1 if light is on
pub struct Weapon2S(Led<29376, 64, 6>);
impl Weapon2S {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Weapon2S {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Weapon2S {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCA - Weapons Button 3 R Light
/// Output: 0 if light is off, 1 if light is on
pub struct Weapon3R(Led<29376, 512, 9>);
impl Weapon3R {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Weapon3R {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Weapon3R {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCA - Weapons Button 3 S Light
/// Output: 0 if light is off, 1 if light is on
pub struct Weapon3S(Led<29376, 256, 8>);
impl Weapon3S {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Weapon3S {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Weapon3S {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCA - Weapons Button 4 R Light
/// Output: 0 if light is off, 1 if light is on
pub struct Weapon4R(Led<29376, 2048, 11>);
impl Weapon4R {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Weapon4R {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Weapon4R {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCA - Weapons Button 4 S Light
/// Output: 0 if light is off, 1 if light is on
pub struct Weapon4S(Led<29376, 1024, 10>);
impl Weapon4S {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Weapon4S {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Weapon4S {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCA - Weapons Button 5 R Light
/// Output: 0 if light is off, 1 if light is on
pub struct Weapon5R(Led<29376, 8192, 13>);
impl Weapon5R {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Weapon5R {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Weapon5R {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// O - PCA - Weapons Button 5 S Light
/// Output: 0 if light is off, 1 if light is on
pub struct Weapon5S(Led<29376, 4096, 12>);
impl Weapon5S {
    pub const fn new() -> Self {
        Self(Led::new())
    }
    pub const fn is_on(&self) -> bool {
        self.0.is_on()
    }
}
impl Default for Weapon5S {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for Weapon5S {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

/// I - White Flood Lights Knob
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct WhFloodLgtKnob(LimitedDial<29348, 65535, 0>);
impl WhFloodLgtKnob {
    pub const fn new() -> Self {
        Self(LimitedDial::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for WhFloodLgtKnob {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for WhFloodLgtKnob {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WpsSelBtn1Value {
    Select1,
    Select2,
}
pub type WpsSelBtn1ValueCallback<'a> = &'a mut dyn FnMut(WpsSelBtn1Value);
impl core::convert::TryFrom<u16> for WpsSelBtn1Value {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PCA - Weapons Selector Button 1
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct WpsSelBtn1(Selector<29374, 256, 8>);
impl WpsSelBtn1 {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for WpsSelBtn1 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for WpsSelBtn1 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WpsSelBtn2Value {
    Select1,
    Select2,
}
pub type WpsSelBtn2ValueCallback<'a> = &'a mut dyn FnMut(WpsSelBtn2Value);
impl core::convert::TryFrom<u16> for WpsSelBtn2Value {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PCA - Weapons Selector Button 2
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct WpsSelBtn2(Selector<29374, 512, 9>);
impl WpsSelBtn2 {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for WpsSelBtn2 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for WpsSelBtn2 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WpsSelBtn3Value {
    Select1,
    Select2,
}
pub type WpsSelBtn3ValueCallback<'a> = &'a mut dyn FnMut(WpsSelBtn3Value);
impl core::convert::TryFrom<u16> for WpsSelBtn3Value {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PCA - Weapons Selector Button 3
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct WpsSelBtn3(Selector<29374, 1024, 10>);
impl WpsSelBtn3 {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for WpsSelBtn3 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for WpsSelBtn3 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WpsSelBtn4Value {
    Select1,
    Select2,
}
pub type WpsSelBtn4ValueCallback<'a> = &'a mut dyn FnMut(WpsSelBtn4Value);
impl core::convert::TryFrom<u16> for WpsSelBtn4Value {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PCA - Weapons Selector Button 4
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct WpsSelBtn4(Selector<29374, 2048, 11>);
impl WpsSelBtn4 {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for WpsSelBtn4 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for WpsSelBtn4 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WpsSelBtn5Value {
    Select1,
    Select2,
}
pub type WpsSelBtn5ValueCallback<'a> = &'a mut dyn FnMut(WpsSelBtn5Value);
impl core::convert::TryFrom<u16> for WpsSelBtn5Value {
    type Error = u16;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Select1),
            1 => Ok(Self::Select2),
            value => Err(value),
        }
    }
}

/// I - PCA - Weapons Selector Button 5
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct WpsSelBtn5(Selector<29374, 4096, 12>);
impl WpsSelBtn5 {
    pub const fn new() -> Self {
        Self(Selector::new())
    }
    pub const fn value(&self) -> u16 {
        self.0.value()
    }
}
impl Default for WpsSelBtn5 {
    fn default() -> Self {
        Self::new()
    }
}
impl Control for WpsSelBtn5 {
    fn accept(&mut self, packet: Packet<'_>) {
        self.0.accept(packet);
    }
    fn commit(&mut self) -> bool {
        self.0.commit()
    }
}

pub enum RegisteredControl<'a> {
    AaRadSttSelBtn {
        control: AaRadSttSelBtn,
        callback: AaRadSttSelBtnValueCallback<'a>,
    },
    AarSw {
        control: AarSw,
        callback: AarSwValueCallback<'a>,
    },
    AbEmerCutoffSw {
        control: AbEmerCutoffSw,
        callback: AbEmerCutoffSwValueCallback<'a>,
    },
    AbEmerCutoffSwCov {
        control: AbEmerCutoffSwCov,
        callback: AbEmerCutoffSwCovValueCallback<'a>,
    },
    AdiBille {
        control: AdiBille,
        callback: IntegerCallback<'a>,
    },
    AdiBklSw {
        control: AdiBklSw,
        callback: AdiBklSwValueCallback<'a>,
    },
    AdiCageLev {
        control: AdiCageLev,
        callback: AdiCageLevValueCallback<'a>,
    },
    AdiCompas {
        control: AdiCompas,
        callback: IntegerCallback<'a>,
    },
    AdiFlag {
        control: AdiFlag,
        callback: IntegerCallback<'a>,
    },
    AdiIlsH {
        control: AdiIlsH,
        callback: IntegerCallback<'a>,
    },
    AdiIlsV {
        control: AdiIlsV,
        callback: IntegerCallback<'a>,
    },
    AdiLight {
        control: AdiLight,
        callback: IntegerCallback<'a>,
    },
    AdiPitch {
        control: AdiPitch,
        callback: IntegerCallback<'a>,
    },
    AdiRoll {
        control: AdiRoll,
        callback: IntegerCallback<'a>,
    },
    AgGunRetSw {
        control: AgGunRetSw,
        callback: AgGunRetSwValueCallback<'a>,
    },
    AgRadAModeSw {
        control: AgRadAModeSw,
        callback: AgRadAModeSwValueCallback<'a>,
    },
    AgRadDecModeSw {
        control: AgRadDecModeSw,
        callback: AgRadDecModeSwValueCallback<'a>,
    },
    AgRadVisuModeSw {
        control: AgRadVisuModeSw,
        callback: AgRadVisuModeSwValueCallback<'a>,
    },
    AirRefuelL {
        control: AirRefuelL,
        callback: LedCallback<'a>,
    },
    Alt100FtSel {
        control: Alt100FtSel,
        callback: Alt100FtSelValueCallback<'a>,
    },
    Alt10kFtSel {
        control: Alt10kFtSel,
        callback: Alt10kFtSelValueCallback<'a>,
    },
    Alt1Sw {
        control: Alt1Sw,
        callback: Alt1SwValueCallback<'a>,
    },
    Alt1kFtSel {
        control: Alt1kFtSel,
        callback: Alt1kFtSelValueCallback<'a>,
    },
    Alt2Sw {
        control: Alt2Sw,
        callback: Alt2SwValueCallback<'a>,
    },
    AltBaroCents {
        control: AltBaroCents,
        callback: IntegerCallback<'a>,
    },
    AltBaroOnes {
        control: AltBaroOnes,
        callback: IntegerCallback<'a>,
    },
    AltBaroTens {
        control: AltBaroTens,
        callback: IntegerCallback<'a>,
    },
    AltBaroThous {
        control: AltBaroThous,
        callback: IntegerCallback<'a>,
    },
    AltCents {
        control: AltCents,
        callback: IntegerCallback<'a>,
    },
    AltHoldBtn {
        control: AltHoldBtn,
        callback: AltHoldBtnValueCallback<'a>,
    },
    AltKcents {
        control: AltKcents,
        callback: IntegerCallback<'a>,
    },
    AltNeedle {
        control: AltNeedle,
        callback: IntegerCallback<'a>,
    },
    AltThous {
        control: AltThous,
        callback: IntegerCallback<'a>,
    },
    AmplisSelKnob {
        control: AmplisSelKnob,
        callback: AmplisSelKnobValueCallback<'a>,
    },
    AntiColLgtSw {
        control: AntiColLgtSw,
        callback: AntiColLgtSwValueCallback<'a>,
    },
    AntiSkidSw {
        control: AntiSkidSw,
        callback: AntiSkidSwValueCallback<'a>,
    },
    AntiSkidSwCov {
        control: AntiSkidSwCov,
        callback: AntiSkidSwCovValueCallback<'a>,
    },
    AoaFlag {
        control: AoaFlag,
        callback: IntegerCallback<'a>,
    },
    AoaPos {
        control: AoaPos,
        callback: IntegerCallback<'a>,
    },
    ApAltAmbre {
        control: ApAltAmbre,
        callback: LedCallback<'a>,
    },
    ApAltVert {
        control: ApAltVert,
        callback: LedCallback<'a>,
    },
    ApGAmbre {
        control: ApGAmbre,
        callback: LedCallback<'a>,
    },
    ApGVert {
        control: ApGVert,
        callback: LedCallback<'a>,
    },
    ApLAmbre {
        control: ApLAmbre,
        callback: LedCallback<'a>,
    },
    ApLVert {
        control: ApLVert,
        callback: LedCallback<'a>,
    },
    ApLgtTestBtn {
        control: ApLgtTestBtn,
        callback: ApLgtTestBtnValueCallback<'a>,
    },
    ApMasterAmbre {
        control: ApMasterAmbre,
        callback: LedCallback<'a>,
    },
    ApMasterBtn {
        control: ApMasterBtn,
        callback: ApMasterBtnValueCallback<'a>,
    },
    ApMasterVert {
        control: ApMasterVert,
        callback: LedCallback<'a>,
    },
    ApNcAmbre {
        control: ApNcAmbre,
        callback: LedCallback<'a>,
    },
    ApNcVert {
        control: ApNcVert,
        callback: LedCallback<'a>,
    },
    ApSelectAltAmbre {
        control: ApSelectAltAmbre,
        callback: LedCallback<'a>,
    },
    ApSelectAltVert {
        control: ApSelectAltVert,
        callback: LedCallback<'a>,
    },
    ApTestSw {
        control: ApTestSw,
        callback: ApTestSwValueCallback<'a>,
    },
    ApTestSwCov {
        control: ApTestSwCov,
        callback: ApTestSwCovValueCallback<'a>,
    },
    AppHoldBtn {
        control: AppHoldBtn,
        callback: AppHoldBtnValueCallback<'a>,
    },
    AudioWarnReset {
        control: AudioWarnReset,
        callback: AudioWarnResetValueCallback<'a>,
    },
    AudioWarnSw {
        control: AudioWarnSw,
        callback: AudioWarnSwValueCallback<'a>,
    },
    AutoManBtn {
        control: AutoManBtn,
        callback: AutoManBtnValueCallback<'a>,
    },
    AuxGunsight {
        control: AuxGunsight,
        callback: AuxGunsightValueCallback<'a>,
    },
    AuxGunsightDefl {
        control: AuxGunsightDefl,
        callback: IntegerCallback<'a>,
    },
    BaroPressCalib {
        control: BaroPressCalib,
        callback: IntegerCallback<'a>,
    },
    BeWpSel {
        control: BeWpSel,
        callback: BeWpSelValueCallback<'a>,
    },
    BingoFuel100KgSel {
        control: BingoFuel100KgSel,
        callback: BingoFuel100KgSelValueCallback<'a>,
    },
    BingoFuel1kKgSel {
        control: BingoFuel1kKgSel,
        callback: BingoFuel1kKgSelValueCallback<'a>,
    },
    BombFuzeSelSw {
        control: BombFuzeSelSw,
        callback: BombFuzeSelSwValueCallback<'a>,
    },
    BombRelIntSelSw {
        control: BombRelIntSelSw,
        callback: BombRelIntSelSwValueCallback<'a>,
    },
    BombRelQtySelSw {
        control: BombRelQtySelSw,
        callback: BombRelQtySelSwValueCallback<'a>,
    },
    BrPwrSw {
        control: BrPwrSw,
        callback: BrPwrSwValueCallback<'a>,
    },
    CautAdvLgt {
        control: CautAdvLgt,
        callback: IntegerCallback<'a>,
    },
    ClimC {
        control: ClimC,
        callback: LedCallback<'a>,
    },
    ClimF {
        control: ClimF,
        callback: LedCallback<'a>,
    },
    ClkH {
        control: ClkH,
        callback: IntegerCallback<'a>,
    },
    ClkM {
        control: ClkM,
        callback: IntegerCallback<'a>,
    },
    ClkS {
        control: ClkS,
        callback: IntegerCallback<'a>,
    },
    Clp5mnO2 {
        control: Clp5mnO2,
        callback: LedCallback<'a>,
    },
    ClpAlt1 {
        control: ClpAlt1,
        callback: LedCallback<'a>,
    },
    ClpAlt2 {
        control: ClpAlt2,
        callback: LedCallback<'a>,
    },
    ClpAnemo {
        control: ClpAnemo,
        callback: LedCallback<'a>,
    },
    ClpBatt {
        control: ClpBatt,
        callback: LedCallback<'a>,
    },
    ClpBecs {
        control: ClpBecs,
        callback: LedCallback<'a>,
    },
    ClpBingo {
        control: ClpBingo,
        callback: LedCallback<'a>,
    },
    ClpBp {
        control: ClpBp,
        callback: LedCallback<'a>,
    },
    ClpBpD {
        control: ClpBpD,
        callback: LedCallback<'a>,
    },
    ClpBpG {
        control: ClpBpG,
        callback: LedCallback<'a>,
    },
    ClpCalc {
        control: ClpCalc,
        callback: LedCallback<'a>,
    },
    ClpCc {
        control: ClpCc,
        callback: LedCallback<'a>,
    },
    ClpCondit {
        control: ClpCondit,
        callback: LedCallback<'a>,
    },
    ClpConf {
        control: ClpConf,
        callback: LedCallback<'a>,
    },
    ClpDecol {
        control: ClpDecol,
        callback: LedCallback<'a>,
    },
    ClpDom {
        control: ClpDom,
        callback: LedCallback<'a>,
    },
    ClpDsv {
        control: ClpDsv,
        callback: LedCallback<'a>,
    },
    ClpEp {
        control: ClpEp,
        callback: LedCallback<'a>,
    },
    ClpGain {
        control: ClpGain,
        callback: LedCallback<'a>,
    },
    ClpHuile {
        control: ClpHuile,
        callback: LedCallback<'a>,
    },
    ClpHyd1 {
        control: ClpHyd1,
        callback: LedCallback<'a>,
    },
    ClpHyd2 {
        control: ClpHyd2,
        callback: LedCallback<'a>,
    },
    ClpHydS {
        control: ClpHydS,
        callback: LedCallback<'a>,
    },
    ClpIncidence {
        control: ClpIncidence,
        callback: LedCallback<'a>,
    },
    ClpMan {
        control: ClpMan,
        callback: LedCallback<'a>,
    },
    ClpNiveau {
        control: ClpNiveau,
        callback: LedCallback<'a>,
    },
    ClpO2Ha {
        control: ClpO2Ha,
        callback: LedCallback<'a>,
    },
    ClpPCab {
        control: ClpPCab,
        callback: LedCallback<'a>,
    },
    ClpPa {
        control: ClpPa,
        callback: LedCallback<'a>,
    },
    ClpPark {
        control: ClpPark,
        callback: LedCallback<'a>,
    },
    ClpPelles {
        control: ClpPelles,
        callback: LedCallback<'a>,
    },
    ClpRegO2 {
        control: ClpRegO2,
        callback: LedCallback<'a>,
    },
    ClpRpm {
        control: ClpRpm,
        callback: LedCallback<'a>,
    },
    ClpSouris {
        control: ClpSouris,
        callback: LedCallback<'a>,
    },
    ClpT7 {
        control: ClpT7,
        callback: LedCallback<'a>,
    },
    ClpTemp {
        control: ClpTemp,
        callback: LedCallback<'a>,
    },
    ClpTransf {
        control: ClpTransf,
        callback: LedCallback<'a>,
    },
    ClpTrn {
        control: ClpTrn,
        callback: LedCallback<'a>,
    },
    ClpUsEl {
        control: ClpUsEl,
        callback: LedCallback<'a>,
    },
    CnpyHand1 {
        control: CnpyHand1,
        callback: CnpyHand1ValueCallback<'a>,
    },
    CnpyHand2 {
        control: CnpyHand2,
        callback: CnpyHand2ValueCallback<'a>,
    },
    CnpyJett {
        control: CnpyJett,
        callback: CnpyJettValueCallback<'a>,
    },
    CnpyLkNLowLev {
        control: CnpyLkNLowLev,
        callback: CnpyLkNLowLevValueCallback<'a>,
    },
    CnpyRest {
        control: CnpyRest,
        callback: CnpyRestValueCallback<'a>,
    },
    CocClock {
        control: CocClock,
        callback: CocClockValueCallback<'a>,
    },
    ConesOpSw {
        control: ConesOpSw,
        callback: ConesOpSwValueCallback<'a>,
    },
    ConfA {
        control: ConfA,
        callback: LedCallback<'a>,
    },
    ConfAux {
        control: ConfAux,
        callback: LedCallback<'a>,
    },
    ConfCross {
        control: ConfCross,
        callback: LedCallback<'a>,
    },
    ConfD {
        control: ConfD,
        callback: LedCallback<'a>,
    },
    ConfDirav {
        control: ConfDirav,
        callback: LedCallback<'a>,
    },
    ConfF {
        control: ConfF,
        callback: LedCallback<'a>,
    },
    ConfFrein {
        control: ConfFrein,
        callback: LedCallback<'a>,
    },
    ConfG {
        control: ConfG,
        callback: LedCallback<'a>,
    },
    ConfRouge {
        control: ConfRouge,
        callback: LedCallback<'a>,
    },
    ConfSpad {
        control: ConfSpad,
        callback: LedCallback<'a>,
    },
    ConsPanelLgtKnob {
        control: ConsPanelLgtKnob,
        callback: IntegerCallback<'a>,
    },
    D2mPwrSw {
        control: D2mPwrSw,
        callback: D2mPwrSwValueCallback<'a>,
    },
    DashPanelLgtKnob {
        control: DashPanelLgtKnob,
        callback: IntegerCallback<'a>,
    },
    DashUVLgtKnob {
        control: DashUVLgtKnob,
        callback: IntegerCallback<'a>,
    },
    DecoyRelModeSw {
        control: DecoyRelModeSw,
        callback: DecoyRelModeSwValueCallback<'a>,
    },
    DecoyRelPgmKnob {
        control: DecoyRelPgmKnob,
        callback: DecoyRelPgmKnobValueCallback<'a>,
    },
    Dem {
        control: Dem,
        callback: LedCallback<'a>,
    },
    DragChuteLev {
        control: DragChuteLev,
        callback: DragChuteLevValueCallback<'a>,
    },
    EcmBoxBltSw {
        control: EcmBoxBltSw,
        callback: EcmBoxBltSwValueCallback<'a>,
    },
    EcmBoxEm {
        control: EcmBoxEm,
        callback: LedCallback<'a>,
    },
    EcmBoxEo {
        control: EcmBoxEo,
        callback: LedCallback<'a>,
    },
    EcmBoxIr {
        control: EcmBoxIr,
        callback: LedCallback<'a>,
    },
    EcmBoxLl {
        control: EcmBoxLl,
        callback: LedCallback<'a>,
    },
    EcsAirExchSw {
        control: EcsAirExchSw,
        callback: EcsAirExchSwValueCallback<'a>,
    },
    EcsCBtn {
        control: EcsCBtn,
        callback: EcsCBtnValueCallback<'a>,
    },
    EcsCondSw {
        control: EcsCondSw,
        callback: EcsCondSwValueCallback<'a>,
    },
    EcsDefogSw {
        control: EcsDefogSw,
        callback: EcsDefogSwValueCallback<'a>,
    },
    EcsFBtn {
        control: EcsFBtn,
        callback: EcsFBtnValueCallback<'a>,
    },
    EcsMainModeSw {
        control: EcsMainModeSw,
        callback: EcsMainModeSwValueCallback<'a>,
    },
    EcsTempSelectKnob {
        control: EcsTempSelectKnob,
        callback: IntegerCallback<'a>,
    },
    ElecFail {
        control: ElecFail,
        callback: LedCallback<'a>,
    },
    ElecPwrTransfSw {
        control: ElecPwrTransfSw,
        callback: ElecPwrTransfSwValueCallback<'a>,
    },
    EmerCompass {
        control: EmerCompass,
        callback: EmerCompassValueCallback<'a>,
    },
    EmerHydPumpSw {
        control: EmerHydPumpSw,
        callback: EmerHydPumpSwValueCallback<'a>,
    },
    EmerJett {
        control: EmerJett,
        callback: EmerJettValueCallback<'a>,
    },
    EmerThrottleCov {
        control: EmerThrottleCov,
        callback: EmerThrottleCovValueCallback<'a>,
    },
    EmerThrottleHand {
        control: EmerThrottleHand,
        callback: IntegerCallback<'a>,
    },
    EngEmerCtrlCov {
        control: EngEmerCtrlCov,
        callback: EngEmerCtrlCovValueCallback<'a>,
    },
    EngEmerCtrlSw {
        control: EngEmerCtrlSw,
        callback: EngEmerCtrlSwValueCallback<'a>,
    },
    EngFuelLPumpSw {
        control: EngFuelLPumpSw,
        callback: EngFuelLPumpSwValueCallback<'a>,
    },
    EngFuelRPumpSw {
        control: EngFuelRPumpSw,
        callback: EngFuelRPumpSwValueCallback<'a>,
    },
    EngInFlgtStartSw {
        control: EngInFlgtStartSw,
        callback: EngInFlgtStartSwValueCallback<'a>,
    },
    EngShutdownBtn {
        control: EngShutdownBtn,
        callback: EngShutdownBtnValueCallback<'a>,
    },
    EngStartSw {
        control: EngStartSw,
        callback: EngStartSwValueCallback<'a>,
    },
    EngStartSwCov {
        control: EngStartSwCov,
        callback: EngStartSwCovValueCallback<'a>,
    },
    EwModeSelSw {
        control: EwModeSelSw,
        callback: EwModeSelSwValueCallback<'a>,
    },
    ExtFormationLightsAft {
        control: ExtFormationLightsAft,
        callback: IntegerCallback<'a>,
    },
    ExtFormationLightsFront {
        control: ExtFormationLightsFront,
        callback: IntegerCallback<'a>,
    },
    ExtPositionLightLeft {
        control: ExtPositionLightLeft,
        callback: IntegerCallback<'a>,
    },
    ExtPositionLightRight {
        control: ExtPositionLightRight,
        callback: IntegerCallback<'a>,
    },
    ExtSpeedBrakeLeft {
        control: ExtSpeedBrakeLeft,
        callback: IntegerCallback<'a>,
    },
    ExtSpeedBrakeRight {
        control: ExtSpeedBrakeRight,
        callback: IntegerCallback<'a>,
    },
    ExtStrobeBottom {
        control: ExtStrobeBottom,
        callback: IntegerCallback<'a>,
    },
    ExtStrobeTop {
        control: ExtStrobeTop,
        callback: IntegerCallback<'a>,
    },
    FbwCh5Sw {
        control: FbwCh5Sw,
        callback: FbwCh5SwValueCallback<'a>,
    },
    FbwCh5SwCov {
        control: FbwCh5SwCov,
        callback: FbwCh5SwCovValueCallback<'a>,
    },
    FbwGLimSw {
        control: FbwGLimSw,
        callback: FbwGLimSwValueCallback<'a>,
    },
    FbwGainModeSw {
        control: FbwGainModeSw,
        callback: FbwGainModeSwValueCallback<'a>,
    },
    FbwGainModeSwCov {
        control: FbwGainModeSwCov,
        callback: FbwGainModeSwCovValueCallback<'a>,
    },
    FbwResetBtn {
        control: FbwResetBtn,
        callback: FbwResetBtnValueCallback<'a>,
    },
    FbwSpinModeSw {
        control: FbwSpinModeSw,
        callback: FbwSpinModeSwValueCallback<'a>,
    },
    FbwTestSw {
        control: FbwTestSw,
        callback: FbwTestSwValueCallback<'a>,
    },
    FbwTestSwCov {
        control: FbwTestSwCov,
        callback: FbwTestSwCovValueCallback<'a>,
    },
    FeuPc {
        control: FeuPc,
        callback: LedCallback<'a>,
    },
    FeuSec {
        control: FeuSec,
        callback: LedCallback<'a>,
    },
    FiringModeSel {
        control: FiringModeSel,
        callback: FiringModeSelValueCallback<'a>,
    },
    FormLgtSw {
        control: FormLgtSw,
        callback: FormLgtSwValueCallback<'a>,
    },
    FuelAvD {
        control: FuelAvD,
        callback: LedCallback<'a>,
    },
    FuelAvG {
        control: FuelAvG,
        callback: LedCallback<'a>,
    },
    FuelCrofSw {
        control: FuelCrofSw,
        callback: FuelCrofSwValueCallback<'a>,
    },
    FuelCutoffSw {
        control: FuelCutoffSw,
        callback: FuelCutoffSwValueCallback<'a>,
    },
    FuelCutoffSwCov {
        control: FuelCutoffSwCov,
        callback: FuelCutoffSwCovValueCallback<'a>,
    },
    FuelDetotCents {
        control: FuelDetotCents,
        callback: IntegerCallback<'a>,
    },
    FuelDetotSw {
        control: FuelDetotSw,
        callback: FuelDetotSwValueCallback<'a>,
    },
    FuelDetotTens {
        control: FuelDetotTens,
        callback: IntegerCallback<'a>,
    },
    FuelDetotThous {
        control: FuelDetotThous,
        callback: IntegerCallback<'a>,
    },
    FuelDmpSw {
        control: FuelDmpSw,
        callback: FuelDmpSwValueCallback<'a>,
    },
    FuelDmpSwCov {
        control: FuelDmpSwCov,
        callback: FuelDmpSwCovValueCallback<'a>,
    },
    FuelFlow {
        control: FuelFlow,
        callback: DisplayCallback<'a>,
    },
    FuelJaugCents {
        control: FuelJaugCents,
        callback: IntegerCallback<'a>,
    },
    FuelJaugD {
        control: FuelJaugD,
        callback: IntegerCallback<'a>,
    },
    FuelJaugG {
        control: FuelJaugG,
        callback: IntegerCallback<'a>,
    },
    FuelJaugTens {
        control: FuelJaugTens,
        callback: IntegerCallback<'a>,
    },
    FuelJaugThous {
        control: FuelJaugThous,
        callback: IntegerCallback<'a>,
    },
    FuelRlD {
        control: FuelRlD,
        callback: LedCallback<'a>,
    },
    FuelRlG {
        control: FuelRlG,
        callback: LedCallback<'a>,
    },
    FuelRlM {
        control: FuelRlM,
        callback: LedCallback<'a>,
    },
    FuelTransfSw {
        control: FuelTransfSw,
        callback: FuelTransfSwValueCallback<'a>,
    },
    FuelVD {
        control: FuelVD,
        callback: LedCallback<'a>,
    },
    FuelVG {
        control: FuelVG,
        callback: LedCallback<'a>,
    },
    GmeterNeedle {
        control: GmeterNeedle,
        callback: IntegerCallback<'a>,
    },
    GunArmSw {
        control: GunArmSw,
        callback: GunArmSwValueCallback<'a>,
    },
    GunModeBtn {
        control: GunModeBtn,
        callback: GunModeBtnValueCallback<'a>,
    },
    GunModeKl1 {
        control: GunModeKl1,
        callback: LedCallback<'a>,
    },
    GunModeKl2 {
        control: GunModeKl2,
        callback: LedCallback<'a>,
    },
    HsiCompas {
        control: HsiCompas,
        callback: IntegerCallback<'a>,
    },
    HsiDNeedle {
        control: HsiDNeedle,
        callback: IntegerCallback<'a>,
    },
    HsiDistCents {
        control: HsiDistCents,
        callback: IntegerCallback<'a>,
    },
    HsiDistDec {
        control: HsiDistDec,
        callback: IntegerCallback<'a>,
    },
    HsiDistOnes {
        control: HsiDistOnes,
        callback: IntegerCallback<'a>,
    },
    HsiDistTens {
        control: HsiDistTens,
        callback: IntegerCallback<'a>,
    },
    HsiFlagD {
        control: HsiFlagD,
        callback: IntegerCallback<'a>,
    },
    HsiFlagDist {
        control: HsiFlagDist,
        callback: IntegerCallback<'a>,
    },
    HsiFlagG {
        control: HsiFlagG,
        callback: IntegerCallback<'a>,
    },
    HsiFlagM {
        control: HsiFlagM,
        callback: IntegerCallback<'a>,
    },
    HsiHdg {
        control: HsiHdg,
        callback: IntegerCallback<'a>,
    },
    HsiModeSelSw {
        control: HsiModeSelSw,
        callback: HsiModeSelSwValueCallback<'a>,
    },
    HsiNeedle {
        control: HsiNeedle,
        callback: IntegerCallback<'a>,
    },
    HsiVadSel {
        control: HsiVadSel,
        callback: IntegerCallback<'a>,
    },
    HudClrBtn {
        control: HudClrBtn,
        callback: HudClrBtnValueCallback<'a>,
    },
    HudDecSw {
        control: HudDecSw,
        callback: HudDecSwValueCallback<'a>,
    },
    HudPwrSw {
        control: HudPwrSw,
        callback: HudPwrSwValueCallback<'a>,
    },
    HudRaltSelSw {
        control: HudRaltSelSw,
        callback: HudRaltSelSwValueCallback<'a>,
    },
    HudRec {
        control: HudRec,
        callback: LedCallback<'a>,
    },
    HydDNeedle {
        control: HydDNeedle,
        callback: IntegerCallback<'a>,
    },
    HydFail {
        control: HydFail,
        callback: LedCallback<'a>,
    },
    HydGNeedle {
        control: HydGNeedle,
        callback: IntegerCallback<'a>,
    },
    HydSysSel {
        control: HydSysSel,
        callback: HydSysSelValueCallback<'a>,
    },
    IdentPwrSw {
        control: IdentPwrSw,
        callback: IdentPwrSwValueCallback<'a>,
    },
    IffFault {
        control: IffFault,
        callback: LedCallback<'a>,
    },
    IffInd {
        control: IffInd,
        callback: LedCallback<'a>,
    },
    IffM4Sw {
        control: IffM4Sw,
        callback: IffM4SwValueCallback<'a>,
    },
    IffRed {
        control: IffRed,
        callback: LedCallback<'a>,
    },
    InsAuxHdHor {
        control: InsAuxHdHor,
        callback: InsAuxHdHorValueCallback<'a>,
    },
    InsBtn0 {
        control: InsBtn0,
        callback: InsBtn0ValueCallback<'a>,
    },
    InsBtn1 {
        control: InsBtn1,
        callback: InsBtn1ValueCallback<'a>,
    },
    InsBtn2 {
        control: InsBtn2,
        callback: InsBtn2ValueCallback<'a>,
    },
    InsBtn3 {
        control: InsBtn3,
        callback: InsBtn3ValueCallback<'a>,
    },
    InsBtn4 {
        control: InsBtn4,
        callback: InsBtn4ValueCallback<'a>,
    },
    InsBtn5 {
        control: InsBtn5,
        callback: InsBtn5ValueCallback<'a>,
    },
    InsBtn6 {
        control: InsBtn6,
        callback: InsBtn6ValueCallback<'a>,
    },
    InsBtn7 {
        control: InsBtn7,
        callback: InsBtn7ValueCallback<'a>,
    },
    InsBtn8 {
        control: InsBtn8,
        callback: InsBtn8ValueCallback<'a>,
    },
    InsBtn9 {
        control: InsBtn9,
        callback: InsBtn9ValueCallback<'a>,
    },
    InsClrBtn {
        control: InsClrBtn,
        callback: InsClrBtnValueCallback<'a>,
    },
    InsDestSw {
        control: InsDestSw,
        callback: InsDestSwValueCallback<'a>,
    },
    InsDestSwLight {
        control: InsDestSwLight,
        callback: LedCallback<'a>,
    },
    InsEffSwLight {
        control: InsEffSwLight,
        callback: LedCallback<'a>,
    },
    InsEnterBtn {
        control: InsEnterBtn,
        callback: InsEnterBtnValueCallback<'a>,
    },
    InsInsSwLight {
        control: InsInsSwLight,
        callback: LedCallback<'a>,
    },
    InsModeSel {
        control: InsModeSel,
        callback: InsModeSelValueCallback<'a>,
    },
    InsNextWpBtn {
        control: InsNextWpBtn,
        callback: InsNextWpBtnValueCallback<'a>,
    },
    InsOpalMode {
        control: InsOpalMode,
        callback: InsOpalModeValueCallback<'a>,
    },
    InsParamSel {
        control: InsParamSel,
        callback: IntegerCallback<'a>,
    },
    InsPrepSw {
        control: InsPrepSw,
        callback: InsPrepSwValueCallback<'a>,
    },
    InsPrepSwLight {
        control: InsPrepSwLight,
        callback: LedCallback<'a>,
    },
    InsPrevWpBtn {
        control: InsPrevWpBtn,
        callback: InsPrevWpBtnValueCallback<'a>,
    },
    InsUpdate {
        control: InsUpdate,
        callback: InsUpdateValueCallback<'a>,
    },
    IntakeSlatsOpSw {
        control: IntakeSlatsOpSw,
        callback: IntakeSlatsOpSwValueCallback<'a>,
    },
    LandingGearLeverLight {
        control: LandingGearLeverLight,
        callback: LedCallback<'a>,
    },
    LdgEmerRelLev {
        control: LdgEmerRelLev,
        callback: LdgEmerRelLevValueCallback<'a>,
    },
    LdgLev {
        control: LdgLev,
        callback: LdgLevValueCallback<'a>,
    },
    LdgLgtSw {
        control: LdgLgtSw,
        callback: LdgLgtSwValueCallback<'a>,
    },
    LgtBrightCtrlTest {
        control: LgtBrightCtrlTest,
        callback: IntegerCallback<'a>,
    },
    LgtTestSw {
        control: LgtTestSw,
        callback: LgtTestSwValueCallback<'a>,
    },
    LimInd {
        control: LimInd,
        callback: LedCallback<'a>,
    },
    LoxDilLev {
        control: LoxDilLev,
        callback: LoxDilLevValueCallback<'a>,
    },
    LoxEmerSup {
        control: LoxEmerSup,
        callback: LoxEmerSupValueCallback<'a>,
    },
    MagicStandby {
        control: MagicStandby,
        callback: MagicStandbyValueCallback<'a>,
    },
    MagicToneVolKnob {
        control: MagicToneVolKnob,
        callback: IntegerCallback<'a>,
    },
    MagnetoSw {
        control: MagnetoSw,
        callback: MagnetoSwValueCallback<'a>,
    },
    MainBattSw {
        control: MainBattSw,
        callback: MainBattSwValueCallback<'a>,
    },
    MasterArmSw {
        control: MasterArmSw,
        callback: MasterArmSwValueCallback<'a>,
    },
    McAmbre {
        control: McAmbre,
        callback: LedCallback<'a>,
    },
    McRouge {
        control: McRouge,
        callback: LedCallback<'a>,
    },
    MinAltSel {
        control: MinAltSel,
        callback: IntegerCallback<'a>,
    },
    MirrorToggle {
        control: MirrorToggle,
        callback: MirrorToggleValueCallback<'a>,
    },
    MisSelSw {
        control: MisSelSw,
        callback: MisSelSwValueCallback<'a>,
    },
    Mode110Sel {
        control: Mode110Sel,
        callback: Mode110SelValueCallback<'a>,
    },
    Mode11Sel {
        control: Mode11Sel,
        callback: Mode11SelValueCallback<'a>,
    },
    Mode1Sw {
        control: Mode1Sw,
        callback: Mode1SwValueCallback<'a>,
    },
    Mode2Sw {
        control: Mode2Sw,
        callback: Mode2SwValueCallback<'a>,
    },
    Mode3a100Sel {
        control: Mode3a100Sel,
        callback: Mode3a100SelValueCallback<'a>,
    },
    Mode3a10Sel {
        control: Mode3a10Sel,
        callback: Mode3a10SelValueCallback<'a>,
    },
    Mode3a1Sel {
        control: Mode3a1Sel,
        callback: Mode3a1SelValueCallback<'a>,
    },
    Mode3a1kSel {
        control: Mode3a1kSel,
        callback: Mode3a1kSelValueCallback<'a>,
    },
    Mode3aSw {
        control: Mode3aSw,
        callback: Mode3aSwValueCallback<'a>,
    },
    ModeCSw {
        control: ModeCSw,
        callback: ModeCSwValueCallback<'a>,
    },
    MrkPos {
        control: MrkPos,
        callback: MrkPosValueCallback<'a>,
    },
    MrkSigVolKnob {
        control: MrkSigVolKnob,
        callback: IntegerCallback<'a>,
    },
    NRpm {
        control: NRpm,
        callback: IntegerCallback<'a>,
    },
    NavLgtSw {
        control: NavLgtSw,
        callback: NavLgtSwValueCallback<'a>,
    },
    NvgHelmetMount {
        control: NvgHelmetMount,
        callback: NvgHelmetMountValueCallback<'a>,
    },
    NvgLightFilterSw {
        control: NvgLightFilterSw,
        callback: NvgLightFilterSwValueCallback<'a>,
    },
    NvgStow {
        control: NvgStow,
        callback: NvgStowValueCallback<'a>,
    },
    NwsIffInterrBtn {
        control: NwsIffInterrBtn,
        callback: NwsIffInterrBtnValueCallback<'a>,
    },
    OffsetWpTgt {
        control: OffsetWpTgt,
        callback: OffsetWpTgtValueCallback<'a>,
    },
    OxyFlowInd {
        control: OxyFlowInd,
        callback: IntegerCallback<'a>,
    },
    OxyNeedle {
        control: OxyNeedle,
        callback: IntegerCallback<'a>,
    },
    Pc {
        control: Pc,
        callback: LedCallback<'a>,
    },
    PcaBr1Disp {
        control: PcaBr1Disp,
        callback: DisplayCallback<'a>,
    },
    PcaBr2Disp {
        control: PcaBr2Disp,
        callback: DisplayCallback<'a>,
    },
    PcaBr3Disp {
        control: PcaBr3Disp,
        callback: DisplayCallback<'a>,
    },
    PcaBr4Disp {
        control: PcaBr4Disp,
        callback: DisplayCallback<'a>,
    },
    PcaBr5Disp {
        control: PcaBr5Disp,
        callback: DisplayCallback<'a>,
    },
    PcaBtn1 {
        control: PcaBtn1,
        callback: PcaBtn1ValueCallback<'a>,
    },
    PcaBtn2 {
        control: PcaBtn2,
        callback: PcaBtn2ValueCallback<'a>,
    },
    PcaBtn3 {
        control: PcaBtn3,
        callback: PcaBtn3ValueCallback<'a>,
    },
    PcaBtn4 {
        control: PcaBtn4,
        callback: PcaBtn4ValueCallback<'a>,
    },
    PcaBtn5 {
        control: PcaBtn5,
        callback: PcaBtn5ValueCallback<'a>,
    },
    PcaButton1S {
        control: PcaButton1S,
        callback: LedCallback<'a>,
    },
    PcaButton2S {
        control: PcaButton2S,
        callback: LedCallback<'a>,
    },
    PcaButton3S {
        control: PcaButton3S,
        callback: LedCallback<'a>,
    },
    PcaButton4S {
        control: PcaButton4S,
        callback: LedCallback<'a>,
    },
    PcaButton5S {
        control: PcaButton5S,
        callback: LedCallback<'a>,
    },
    PcaUr1Disp {
        control: PcaUr1Disp,
        callback: DisplayCallback<'a>,
    },
    PcaUr2Disp {
        control: PcaUr2Disp,
        callback: DisplayCallback<'a>,
    },
    PcaUr3Disp {
        control: PcaUr3Disp,
        callback: DisplayCallback<'a>,
    },
    PcaUr4Disp {
        control: PcaUr4Disp,
        callback: DisplayCallback<'a>,
    },
    PcaUr5Disp {
        control: PcaUr5Disp,
        callback: DisplayCallback<'a>,
    },
    PcnAln {
        control: PcnAln,
        callback: LedCallback<'a>,
    },
    PcnBad {
        control: PcnBad,
        callback: LedCallback<'a>,
    },
    PcnDis2dl {
        control: PcnDis2dl,
        callback: DisplayCallback<'a>,
    },
    PcnDis2dr {
        control: PcnDis2dr,
        callback: DisplayCallback<'a>,
    },
    PcnDisDl {
        control: PcnDisDl,
        callback: DisplayCallback<'a>,
    },
    PcnDisDr {
        control: PcnDisDr,
        callback: DisplayCallback<'a>,
    },
    PcnDispDest {
        control: PcnDispDest,
        callback: DisplayCallback<'a>,
    },
    PcnDispL {
        control: PcnDispL,
        callback: DisplayCallback<'a>,
    },
    PcnDispPrep {
        control: PcnDispPrep,
        callback: DisplayCallback<'a>,
    },
    PcnDispR {
        control: PcnDispR,
        callback: DisplayCallback<'a>,
    },
    PcnEncSwLight {
        control: PcnEncSwLight,
        callback: LedCallback<'a>,
    },
    PcnM91Light {
        control: PcnM91Light,
        callback: LedCallback<'a>,
    },
    PcnM92Light {
        control: PcnM92Light,
        callback: LedCallback<'a>,
    },
    PcnM93Light {
        control: PcnM93Light,
        callback: LedCallback<'a>,
    },
    PcnMip {
        control: PcnMip,
        callback: LedCallback<'a>,
    },
    PcnMrqSwLight {
        control: PcnMrqSwLight,
        callback: LedCallback<'a>,
    },
    PcnNdeg {
        control: PcnNdeg,
        callback: LedCallback<'a>,
    },
    PcnPret {
        control: PcnPret,
        callback: LedCallback<'a>,
    },
    PcnRec {
        control: PcnRec,
        callback: LedCallback<'a>,
    },
    PcnSec {
        control: PcnSec,
        callback: LedCallback<'a>,
    },
    PcnUni {
        control: PcnUni,
        callback: LedCallback<'a>,
    },
    PcnValSwLight {
        control: PcnValSwLight,
        callback: LedCallback<'a>,
    },
    PedalAdjustLev {
        control: PedalAdjustLev,
        callback: PedalAdjustLevValueCallback<'a>,
    },
    PitotHeatCov {
        control: PitotHeatCov,
        callback: PitotHeatCovValueCallback<'a>,
    },
    PitotHeatSw {
        control: PitotHeatSw,
        callback: PitotHeatSwValueCallback<'a>,
    },
    PkgBrakeLev {
        control: PkgBrakeLev,
        callback: PkgBrakeLevValueCallback<'a>,
    },
    PoliceLgtSw {
        control: PoliceLgtSw,
        callback: PoliceLgtSwValueCallback<'a>,
    },
    PpaAuto {
        control: PpaAuto,
        callback: LedCallback<'a>,
    },
    PpaGunRocketPap {
        control: PpaGunRocketPap,
        callback: LedCallback<'a>,
    },
    PpaGunRocketTot {
        control: PpaGunRocketTot,
        callback: LedCallback<'a>,
    },
    PpaIntDisp {
        control: PpaIntDisp,
        callback: DisplayCallback<'a>,
    },
    PpaMagicMis {
        control: PpaMagicMis,
        callback: LedCallback<'a>,
    },
    PpaMagicP {
        control: PpaMagicP,
        callback: LedCallback<'a>,
    },
    PpaMan {
        control: PpaMan,
        callback: LedCallback<'a>,
    },
    PpaQtyDisp {
        control: PpaQtyDisp,
        callback: DisplayCallback<'a>,
    },
    PpaS530Mis {
        control: PpaS530Mis,
        callback: LedCallback<'a>,
    },
    PpaS530P {
        control: PpaS530P,
        callback: LedCallback<'a>,
    },
    PpaTestSw {
        control: PpaTestSw,
        callback: PpaTestSwValueCallback<'a>,
    },
    PressNeedle {
        control: PressNeedle,
        callback: IntegerCallback<'a>,
    },
    PsvDerive {
        control: PsvDerive,
        callback: IntegerCallback<'a>,
    },
    PsvElDExt {
        control: PsvElDExt,
        callback: IntegerCallback<'a>,
    },
    PsvElDInt {
        control: PsvElDInt,
        callback: IntegerCallback<'a>,
    },
    PsvElGExt {
        control: PsvElGExt,
        callback: IntegerCallback<'a>,
    },
    PsvElGInt {
        control: PsvElGInt,
        callback: IntegerCallback<'a>,
    },
    QraSw {
        control: QraSw,
        callback: QraSwValueCallback<'a>,
    },
    RadA {
        control: RadA,
        callback: LedCallback<'a>,
    },
    RadAzSel {
        control: RadAzSel,
        callback: RadAzSelValueCallback<'a>,
    },
    RadContrastSw {
        control: RadContrastSw,
        callback: IntegerCallback<'a>,
    },
    RadDec {
        control: RadDec,
        callback: LedCallback<'a>,
    },
    RadDopRejSw {
        control: RadDopRejSw,
        callback: RadDopRejSwValueCallback<'a>,
    },
    RadGainDial {
        control: RadGainDial,
        callback: IntegerCallback<'a>,
    },
    RadGridSelSw {
        control: RadGridSelSw,
        callback: RadGridSelSwValueCallback<'a>,
    },
    RadIffCode1Sel {
        control: RadIffCode1Sel,
        callback: RadIffCode1SelValueCallback<'a>,
    },
    RadIffCode2Sel {
        control: RadIffCode2Sel,
        callback: RadIffCode2SelValueCallback<'a>,
    },
    RadIffCode3Sel {
        control: RadIffCode3Sel,
        callback: RadIffCode3SelValueCallback<'a>,
    },
    RadIffCode4Sel {
        control: RadIffCode4Sel,
        callback: RadIffCode4SelValueCallback<'a>,
    },
    RadIffLRSel {
        control: RadIffLRSel,
        callback: RadIffLRSelValueCallback<'a>,
    },
    RadIffModeSw {
        control: RadIffModeSw,
        callback: RadIffModeSwValueCallback<'a>,
    },
    RadIffPowSw {
        control: RadIffPowSw,
        callback: RadIffPowSwValueCallback<'a>,
    },
    RadIllumSw {
        control: RadIllumSw,
        callback: RadIllumSwValueCallback<'a>,
    },
    RadPrfSw {
        control: RadPrfSw,
        callback: RadPrfSwValueCallback<'a>,
    },
    RadPsic {
        control: RadPsic,
        callback: LedCallback<'a>,
    },
    RadPwrSel {
        control: RadPwrSel,
        callback: RadPwrSelValueCallback<'a>,
    },
    RadRaltPwrSw {
        control: RadRaltPwrSw,
        callback: RadRaltPwrSwValueCallback<'a>,
    },
    RadRearmBtn {
        control: RadRearmBtn,
        callback: RadRearmBtnValueCallback<'a>,
    },
    RadRngSelSw {
        control: RadRngSelSw,
        callback: RadRngSelSwValueCallback<'a>,
    },
    RadScanLinesSel {
        control: RadScanLinesSel,
        callback: RadScanLinesSelValueCallback<'a>,
    },
    RadTestBtn {
        control: RadTestBtn,
        callback: RadTestBtnValueCallback<'a>,
    },
    RadVal {
        control: RadVal,
        callback: LedCallback<'a>,
    },
    RadVisu {
        control: RadVisu,
        callback: LedCallback<'a>,
    },
    RadWowEmAutSw {
        control: RadWowEmAutSw,
        callback: RadWowEmAutSwValueCallback<'a>,
    },
    RedFloodLgtKnob {
        control: RedFloodLgtKnob,
        callback: IntegerCallback<'a>,
    },
    RefuelLgtBrightKnob {
        control: RefuelLgtBrightKnob,
        callback: IntegerCallback<'a>,
    },
    RpmOnes {
        control: RpmOnes,
        callback: IntegerCallback<'a>,
    },
    RpmTens {
        control: RpmTens,
        callback: IntegerCallback<'a>,
    },
    RwrBr {
        control: RwrBr,
        callback: LedCallback<'a>,
    },
    RwrD2m {
        control: RwrD2m,
        callback: LedCallback<'a>,
    },
    RwrDa {
        control: RwrDa,
        callback: LedCallback<'a>,
    },
    RwrLgtBrightCtrl {
        control: RwrLgtBrightCtrl,
        callback: IntegerCallback<'a>,
    },
    RwrLl {
        control: RwrLl,
        callback: LedCallback<'a>,
    },
    RwrPwrSw {
        control: RwrPwrSw,
        callback: RwrPwrSwValueCallback<'a>,
    },
    RwrV {
        control: RwrV,
        callback: LedCallback<'a>,
    },
    S530Standby {
        control: S530Standby,
        callback: S530StandbyValueCallback<'a>,
    },
    SafeGndSw {
        control: SafeGndSw,
        callback: SafeGndSwValueCallback<'a>,
    },
    SbAdiCageSw {
        control: SbAdiCageSw,
        callback: SbAdiCageSwValueCallback<'a>,
    },
    SbAdiFlag {
        control: SbAdiFlag,
        callback: IntegerCallback<'a>,
    },
    SbAdiPitch {
        control: SbAdiPitch,
        callback: IntegerCallback<'a>,
    },
    SbAdiRoll {
        control: SbAdiRoll,
        callback: IntegerCallback<'a>,
    },
    SbAdiRot {
        control: SbAdiRot,
        callback: IntegerCallback<'a>,
    },
    SeatAdjustSw {
        control: SeatAdjustSw,
        callback: SeatAdjustSwValueCallback<'a>,
    },
    SecHuileSw {
        control: SecHuileSw,
        callback: SecHuileSwValueCallback<'a>,
    },
    SecHuileSwCover {
        control: SecHuileSwCover,
        callback: SecHuileSwCoverValueCallback<'a>,
    },
    SelAltHoldBtn {
        control: SelAltHoldBtn,
        callback: SelAltHoldBtnValueCallback<'a>,
    },
    SelJettSw {
        control: SelJettSw,
        callback: SelJettSwValueCallback<'a>,
    },
    SelJettSwCov {
        control: SelJettSwCov,
        callback: SelJettSwCovValueCallback<'a>,
    },
    SerpamRecSw {
        control: SerpamRecSw,
        callback: SerpamRecSwValueCallback<'a>,
    },
    SlatsOpSw {
        control: SlatsOpSw,
        callback: SlatsOpSwValueCallback<'a>,
    },
    SpeedKtsNeed {
        control: SpeedKtsNeed,
        callback: IntegerCallback<'a>,
    },
    SpeedMach {
        control: SpeedMach,
        callback: IntegerCallback<'a>,
    },
    StartFuelPumpSw {
        control: StartFuelPumpSw,
        callback: StartFuelPumpSwValueCallback<'a>,
    },
    T7Needle {
        control: T7Needle,
        callback: IntegerCallback<'a>,
    },
    TacCh10Sel {
        control: TacCh10Sel,
        callback: IntegerCallback<'a>,
    },
    TacCh1Sel {
        control: TacCh1Sel,
        callback: TacCh1SelValueCallback<'a>,
    },
    TacModeSel {
        control: TacModeSel,
        callback: TacModeSelValueCallback<'a>,
    },
    TacVolKnob {
        control: TacVolKnob,
        callback: IntegerCallback<'a>,
    },
    TacXYSel {
        control: TacXYSel,
        callback: TacXYSelValueCallback<'a>,
    },
    TbAppVolKnob {
        control: TbAppVolKnob,
        callback: IntegerCallback<'a>,
    },
    TdcModeSw {
        control: TdcModeSw,
        callback: TdcModeSwValueCallback<'a>,
    },
    TestRouge {
        control: TestRouge,
        callback: LedCallback<'a>,
    },
    TestVert {
        control: TestVert,
        callback: LedCallback<'a>,
    },
    TgtAge {
        control: TgtAge,
        callback: TgtAgeValueCallback<'a>,
    },
    TgtAlt {
        control: TgtAlt,
        callback: TgtAltValueCallback<'a>,
    },
    TgtBearBe {
        control: TgtBearBe,
        callback: TgtBearBeValueCallback<'a>,
    },
    TgtDataMnlEntry {
        control: TgtDataMnlEntry,
        callback: TgtDataMnlEntryValueCallback<'a>,
    },
    TgtHd {
        control: TgtHd,
        callback: TgtHdValueCallback<'a>,
    },
    TgtMachNum {
        control: TgtMachNum,
        callback: TgtMachNumValueCallback<'a>,
    },
    TgtMemTimeSelSw {
        control: TgtMemTimeSelSw,
        callback: TgtMemTimeSelSwValueCallback<'a>,
    },
    TgtRngBe {
        control: TgtRngBe,
        callback: TgtRngBeValueCallback<'a>,
    },
    TgtWingspanKnob {
        control: TgtWingspanKnob,
        callback: IntegerCallback<'a>,
    },
    UhfCdeSw {
        control: UhfCdeSw,
        callback: UhfCdeSwValueCallback<'a>,
    },
    UhfEA2Sw {
        control: UhfEA2Sw,
        callback: UhfEA2SwValueCallback<'a>,
    },
    UhfFrequency {
        control: UhfFrequency,
        callback: DisplayCallback<'a>,
    },
    UhfModeSw {
        control: UhfModeSw,
        callback: UhfModeSwValueCallback<'a>,
    },
    UhfPreset {
        control: UhfPreset,
        callback: IntegerCallback<'a>,
    },
    UhfPresetKnobInt {
        control: UhfPresetKnobInt,
        callback: IntegerCallback<'a>,
    },
    UhfPresetKnobStr {
        control: UhfPresetKnobStr,
        callback: DisplayCallback<'a>,
    },
    UhfPwr5w25wSw {
        control: UhfPwr5w25wSw,
        callback: UhfPwr5w25wSwValueCallback<'a>,
    },
    UhfRadioVolKnob {
        control: UhfRadioVolKnob,
        callback: IntegerCallback<'a>,
    },
    UhfRepL1 {
        control: UhfRepL1,
        callback: LedCallback<'a>,
    },
    UhfRepL2 {
        control: UhfRepL2,
        callback: LedCallback<'a>,
    },
    UhfSecureL {
        control: UhfSecureL,
        callback: LedCallback<'a>,
    },
    UhfSilSw {
        control: UhfSilSw,
        callback: UhfSilSwValueCallback<'a>,
    },
    UhfTestL {
        control: UhfTestL,
        callback: LedCallback<'a>,
    },
    UhfTestSw {
        control: UhfTestSw,
        callback: UhfTestSwValueCallback<'a>,
    },
    ValDataEntry {
        control: ValDataEntry,
        callback: ValDataEntryValueCallback<'a>,
    },
    VarioNeedle {
        control: VarioNeedle,
        callback: IntegerCallback<'a>,
    },
    VentSelSw {
        control: VentSelSw,
        callback: VentSelSwValueCallback<'a>,
    },
    Vhf0 {
        control: Vhf0,
        callback: Vhf0ValueCallback<'a>,
    },
    Vhf1Read {
        control: Vhf1Read,
        callback: Vhf1ReadValueCallback<'a>,
    },
    Vhf2Sql {
        control: Vhf2Sql,
        callback: Vhf2SqlValueCallback<'a>,
    },
    Vhf3Gr {
        control: Vhf3Gr,
        callback: Vhf3GrValueCallback<'a>,
    },
    Vhf4 {
        control: Vhf4,
        callback: Vhf4ValueCallback<'a>,
    },
    Vhf520Low {
        control: Vhf520Low,
        callback: Vhf520LowValueCallback<'a>,
    },
    Vhf6Tone {
        control: Vhf6Tone,
        callback: Vhf6ToneValueCallback<'a>,
    },
    Vhf7 {
        control: Vhf7,
        callback: Vhf7ValueCallback<'a>,
    },
    Vhf8Tod {
        control: Vhf8Tod,
        callback: Vhf8TodValueCallback<'a>,
    },
    Vhf9Zero {
        control: Vhf9Zero,
        callback: Vhf9ZeroValueCallback<'a>,
    },
    VhfChSel {
        control: VhfChSel,
        callback: IntegerCallback<'a>,
    },
    VhfConf {
        control: VhfConf,
        callback: VhfConfValueCallback<'a>,
    },
    VhfFrequency {
        control: VhfFrequency,
        callback: DisplayCallback<'a>,
    },
    VhfMemClr {
        control: VhfMemClr,
        callback: VhfMemClrValueCallback<'a>,
    },
    VhfMode {
        control: VhfMode,
        callback: VhfModeValueCallback<'a>,
    },
    VhfVldXfr {
        control: VhfVldXfr,
        callback: VhfVldXfrValueCallback<'a>,
    },
    Vorils001Drum {
        control: Vorils001Drum,
        callback: IntegerCallback<'a>,
    },
    Vorils01Drum {
        control: Vorils01Drum,
        callback: IntegerCallback<'a>,
    },
    Vorils100Drum {
        control: Vorils100Drum,
        callback: IntegerCallback<'a>,
    },
    Vorils10Drum {
        control: Vorils10Drum,
        callback: IntegerCallback<'a>,
    },
    Vorils1Drum {
        control: Vorils1Drum,
        callback: IntegerCallback<'a>,
    },
    VorilsFreqDecimal {
        control: VorilsFreqDecimal,
        callback: IntegerCallback<'a>,
    },
    VorilsFreqWhole {
        control: VorilsFreqWhole,
        callback: IntegerCallback<'a>,
    },
    VorilsPwrDial {
        control: VorilsPwrDial,
        callback: VorilsPwrDialValueCallback<'a>,
    },
    VorilsTestDial {
        control: VorilsTestDial,
        callback: VorilsTestDialValueCallback<'a>,
    },
    VorilsVolKnob {
        control: VorilsVolKnob,
        callback: IntegerCallback<'a>,
    },
    Vtac10Drum {
        control: Vtac10Drum,
        callback: IntegerCallback<'a>,
    },
    Vtac1Drum {
        control: Vtac1Drum,
        callback: IntegerCallback<'a>,
    },
    VtacXYDrum {
        control: VtacXYDrum,
        callback: IntegerCallback<'a>,
    },
    VtbDec {
        control: VtbDec,
        callback: VtbDecValueCallback<'a>,
    },
    VtbDispBright {
        control: VtbDispBright,
        callback: VtbDispBrightValueCallback<'a>,
    },
    VtbDispContrast {
        control: VtbDispContrast,
        callback: VtbDispContrastValueCallback<'a>,
    },
    VtbIconsBright {
        control: VtbIconsBright,
        callback: VtbIconsBrightValueCallback<'a>,
    },
    VtbOrSel {
        control: VtbOrSel,
        callback: VtbOrSelValueCallback<'a>,
    },
    VtbPwrSw {
        control: VtbPwrSw,
        callback: VtbPwrSwValueCallback<'a>,
    },
    VtbVideoBright {
        control: VtbVideoBright,
        callback: VtbVideoBrightValueCallback<'a>,
    },
    VuhfRadioVolKnob {
        control: VuhfRadioVolKnob,
        callback: IntegerCallback<'a>,
    },
    Weapon1R {
        control: Weapon1R,
        callback: LedCallback<'a>,
    },
    Weapon1S {
        control: Weapon1S,
        callback: LedCallback<'a>,
    },
    Weapon2R {
        control: Weapon2R,
        callback: LedCallback<'a>,
    },
    Weapon2S {
        control: Weapon2S,
        callback: LedCallback<'a>,
    },
    Weapon3R {
        control: Weapon3R,
        callback: LedCallback<'a>,
    },
    Weapon3S {
        control: Weapon3S,
        callback: LedCallback<'a>,
    },
    Weapon4R {
        control: Weapon4R,
        callback: LedCallback<'a>,
    },
    Weapon4S {
        control: Weapon4S,
        callback: LedCallback<'a>,
    },
    Weapon5R {
        control: Weapon5R,
        callback: LedCallback<'a>,
    },
    Weapon5S {
        control: Weapon5S,
        callback: LedCallback<'a>,
    },
    WhFloodLgtKnob {
        control: WhFloodLgtKnob,
        callback: IntegerCallback<'a>,
    },
    WpsSelBtn1 {
        control: WpsSelBtn1,
        callback: WpsSelBtn1ValueCallback<'a>,
    },
    WpsSelBtn2 {
        control: WpsSelBtn2,
        callback: WpsSelBtn2ValueCallback<'a>,
    },
    WpsSelBtn3 {
        control: WpsSelBtn3,
        callback: WpsSelBtn3ValueCallback<'a>,
    },
    WpsSelBtn4 {
        control: WpsSelBtn4,
        callback: WpsSelBtn4ValueCallback<'a>,
    },
    WpsSelBtn5 {
        control: WpsSelBtn5,
        callback: WpsSelBtn5ValueCallback<'a>,
    },
}

impl RegisteredControl<'_> {
    fn accept(&mut self, packet: Packet<'_>) {
        match self {
            Self::AaRadSttSelBtn { control, .. } => control.accept(packet),
            Self::AarSw { control, .. } => control.accept(packet),
            Self::AbEmerCutoffSw { control, .. } => control.accept(packet),
            Self::AbEmerCutoffSwCov { control, .. } => control.accept(packet),
            Self::AdiBille { control, .. } => control.accept(packet),
            Self::AdiBklSw { control, .. } => control.accept(packet),
            Self::AdiCageLev { control, .. } => control.accept(packet),
            Self::AdiCompas { control, .. } => control.accept(packet),
            Self::AdiFlag { control, .. } => control.accept(packet),
            Self::AdiIlsH { control, .. } => control.accept(packet),
            Self::AdiIlsV { control, .. } => control.accept(packet),
            Self::AdiLight { control, .. } => control.accept(packet),
            Self::AdiPitch { control, .. } => control.accept(packet),
            Self::AdiRoll { control, .. } => control.accept(packet),
            Self::AgGunRetSw { control, .. } => control.accept(packet),
            Self::AgRadAModeSw { control, .. } => control.accept(packet),
            Self::AgRadDecModeSw { control, .. } => control.accept(packet),
            Self::AgRadVisuModeSw { control, .. } => control.accept(packet),
            Self::AirRefuelL { control, .. } => control.accept(packet),
            Self::Alt100FtSel { control, .. } => control.accept(packet),
            Self::Alt10kFtSel { control, .. } => control.accept(packet),
            Self::Alt1Sw { control, .. } => control.accept(packet),
            Self::Alt1kFtSel { control, .. } => control.accept(packet),
            Self::Alt2Sw { control, .. } => control.accept(packet),
            Self::AltBaroCents { control, .. } => control.accept(packet),
            Self::AltBaroOnes { control, .. } => control.accept(packet),
            Self::AltBaroTens { control, .. } => control.accept(packet),
            Self::AltBaroThous { control, .. } => control.accept(packet),
            Self::AltCents { control, .. } => control.accept(packet),
            Self::AltHoldBtn { control, .. } => control.accept(packet),
            Self::AltKcents { control, .. } => control.accept(packet),
            Self::AltNeedle { control, .. } => control.accept(packet),
            Self::AltThous { control, .. } => control.accept(packet),
            Self::AmplisSelKnob { control, .. } => control.accept(packet),
            Self::AntiColLgtSw { control, .. } => control.accept(packet),
            Self::AntiSkidSw { control, .. } => control.accept(packet),
            Self::AntiSkidSwCov { control, .. } => control.accept(packet),
            Self::AoaFlag { control, .. } => control.accept(packet),
            Self::AoaPos { control, .. } => control.accept(packet),
            Self::ApAltAmbre { control, .. } => control.accept(packet),
            Self::ApAltVert { control, .. } => control.accept(packet),
            Self::ApGAmbre { control, .. } => control.accept(packet),
            Self::ApGVert { control, .. } => control.accept(packet),
            Self::ApLAmbre { control, .. } => control.accept(packet),
            Self::ApLVert { control, .. } => control.accept(packet),
            Self::ApLgtTestBtn { control, .. } => control.accept(packet),
            Self::ApMasterAmbre { control, .. } => control.accept(packet),
            Self::ApMasterBtn { control, .. } => control.accept(packet),
            Self::ApMasterVert { control, .. } => control.accept(packet),
            Self::ApNcAmbre { control, .. } => control.accept(packet),
            Self::ApNcVert { control, .. } => control.accept(packet),
            Self::ApSelectAltAmbre { control, .. } => control.accept(packet),
            Self::ApSelectAltVert { control, .. } => control.accept(packet),
            Self::ApTestSw { control, .. } => control.accept(packet),
            Self::ApTestSwCov { control, .. } => control.accept(packet),
            Self::AppHoldBtn { control, .. } => control.accept(packet),
            Self::AudioWarnReset { control, .. } => control.accept(packet),
            Self::AudioWarnSw { control, .. } => control.accept(packet),
            Self::AutoManBtn { control, .. } => control.accept(packet),
            Self::AuxGunsight { control, .. } => control.accept(packet),
            Self::AuxGunsightDefl { control, .. } => control.accept(packet),
            Self::BaroPressCalib { control, .. } => control.accept(packet),
            Self::BeWpSel { control, .. } => control.accept(packet),
            Self::BingoFuel100KgSel { control, .. } => control.accept(packet),
            Self::BingoFuel1kKgSel { control, .. } => control.accept(packet),
            Self::BombFuzeSelSw { control, .. } => control.accept(packet),
            Self::BombRelIntSelSw { control, .. } => control.accept(packet),
            Self::BombRelQtySelSw { control, .. } => control.accept(packet),
            Self::BrPwrSw { control, .. } => control.accept(packet),
            Self::CautAdvLgt { control, .. } => control.accept(packet),
            Self::ClimC { control, .. } => control.accept(packet),
            Self::ClimF { control, .. } => control.accept(packet),
            Self::ClkH { control, .. } => control.accept(packet),
            Self::ClkM { control, .. } => control.accept(packet),
            Self::ClkS { control, .. } => control.accept(packet),
            Self::Clp5mnO2 { control, .. } => control.accept(packet),
            Self::ClpAlt1 { control, .. } => control.accept(packet),
            Self::ClpAlt2 { control, .. } => control.accept(packet),
            Self::ClpAnemo { control, .. } => control.accept(packet),
            Self::ClpBatt { control, .. } => control.accept(packet),
            Self::ClpBecs { control, .. } => control.accept(packet),
            Self::ClpBingo { control, .. } => control.accept(packet),
            Self::ClpBp { control, .. } => control.accept(packet),
            Self::ClpBpD { control, .. } => control.accept(packet),
            Self::ClpBpG { control, .. } => control.accept(packet),
            Self::ClpCalc { control, .. } => control.accept(packet),
            Self::ClpCc { control, .. } => control.accept(packet),
            Self::ClpCondit { control, .. } => control.accept(packet),
            Self::ClpConf { control, .. } => control.accept(packet),
            Self::ClpDecol { control, .. } => control.accept(packet),
            Self::ClpDom { control, .. } => control.accept(packet),
            Self::ClpDsv { control, .. } => control.accept(packet),
            Self::ClpEp { control, .. } => control.accept(packet),
            Self::ClpGain { control, .. } => control.accept(packet),
            Self::ClpHuile { control, .. } => control.accept(packet),
            Self::ClpHyd1 { control, .. } => control.accept(packet),
            Self::ClpHyd2 { control, .. } => control.accept(packet),
            Self::ClpHydS { control, .. } => control.accept(packet),
            Self::ClpIncidence { control, .. } => control.accept(packet),
            Self::ClpMan { control, .. } => control.accept(packet),
            Self::ClpNiveau { control, .. } => control.accept(packet),
            Self::ClpO2Ha { control, .. } => control.accept(packet),
            Self::ClpPCab { control, .. } => control.accept(packet),
            Self::ClpPa { control, .. } => control.accept(packet),
            Self::ClpPark { control, .. } => control.accept(packet),
            Self::ClpPelles { control, .. } => control.accept(packet),
            Self::ClpRegO2 { control, .. } => control.accept(packet),
            Self::ClpRpm { control, .. } => control.accept(packet),
            Self::ClpSouris { control, .. } => control.accept(packet),
            Self::ClpT7 { control, .. } => control.accept(packet),
            Self::ClpTemp { control, .. } => control.accept(packet),
            Self::ClpTransf { control, .. } => control.accept(packet),
            Self::ClpTrn { control, .. } => control.accept(packet),
            Self::ClpUsEl { control, .. } => control.accept(packet),
            Self::CnpyHand1 { control, .. } => control.accept(packet),
            Self::CnpyHand2 { control, .. } => control.accept(packet),
            Self::CnpyJett { control, .. } => control.accept(packet),
            Self::CnpyLkNLowLev { control, .. } => control.accept(packet),
            Self::CnpyRest { control, .. } => control.accept(packet),
            Self::CocClock { control, .. } => control.accept(packet),
            Self::ConesOpSw { control, .. } => control.accept(packet),
            Self::ConfA { control, .. } => control.accept(packet),
            Self::ConfAux { control, .. } => control.accept(packet),
            Self::ConfCross { control, .. } => control.accept(packet),
            Self::ConfD { control, .. } => control.accept(packet),
            Self::ConfDirav { control, .. } => control.accept(packet),
            Self::ConfF { control, .. } => control.accept(packet),
            Self::ConfFrein { control, .. } => control.accept(packet),
            Self::ConfG { control, .. } => control.accept(packet),
            Self::ConfRouge { control, .. } => control.accept(packet),
            Self::ConfSpad { control, .. } => control.accept(packet),
            Self::ConsPanelLgtKnob { control, .. } => control.accept(packet),
            Self::D2mPwrSw { control, .. } => control.accept(packet),
            Self::DashPanelLgtKnob { control, .. } => control.accept(packet),
            Self::DashUVLgtKnob { control, .. } => control.accept(packet),
            Self::DecoyRelModeSw { control, .. } => control.accept(packet),
            Self::DecoyRelPgmKnob { control, .. } => control.accept(packet),
            Self::Dem { control, .. } => control.accept(packet),
            Self::DragChuteLev { control, .. } => control.accept(packet),
            Self::EcmBoxBltSw { control, .. } => control.accept(packet),
            Self::EcmBoxEm { control, .. } => control.accept(packet),
            Self::EcmBoxEo { control, .. } => control.accept(packet),
            Self::EcmBoxIr { control, .. } => control.accept(packet),
            Self::EcmBoxLl { control, .. } => control.accept(packet),
            Self::EcsAirExchSw { control, .. } => control.accept(packet),
            Self::EcsCBtn { control, .. } => control.accept(packet),
            Self::EcsCondSw { control, .. } => control.accept(packet),
            Self::EcsDefogSw { control, .. } => control.accept(packet),
            Self::EcsFBtn { control, .. } => control.accept(packet),
            Self::EcsMainModeSw { control, .. } => control.accept(packet),
            Self::EcsTempSelectKnob { control, .. } => control.accept(packet),
            Self::ElecFail { control, .. } => control.accept(packet),
            Self::ElecPwrTransfSw { control, .. } => control.accept(packet),
            Self::EmerCompass { control, .. } => control.accept(packet),
            Self::EmerHydPumpSw { control, .. } => control.accept(packet),
            Self::EmerJett { control, .. } => control.accept(packet),
            Self::EmerThrottleCov { control, .. } => control.accept(packet),
            Self::EmerThrottleHand { control, .. } => control.accept(packet),
            Self::EngEmerCtrlCov { control, .. } => control.accept(packet),
            Self::EngEmerCtrlSw { control, .. } => control.accept(packet),
            Self::EngFuelLPumpSw { control, .. } => control.accept(packet),
            Self::EngFuelRPumpSw { control, .. } => control.accept(packet),
            Self::EngInFlgtStartSw { control, .. } => control.accept(packet),
            Self::EngShutdownBtn { control, .. } => control.accept(packet),
            Self::EngStartSw { control, .. } => control.accept(packet),
            Self::EngStartSwCov { control, .. } => control.accept(packet),
            Self::EwModeSelSw { control, .. } => control.accept(packet),
            Self::ExtFormationLightsAft { control, .. } => control.accept(packet),
            Self::ExtFormationLightsFront { control, .. } => control.accept(packet),
            Self::ExtPositionLightLeft { control, .. } => control.accept(packet),
            Self::ExtPositionLightRight { control, .. } => control.accept(packet),
            Self::ExtSpeedBrakeLeft { control, .. } => control.accept(packet),
            Self::ExtSpeedBrakeRight { control, .. } => control.accept(packet),
            Self::ExtStrobeBottom { control, .. } => control.accept(packet),
            Self::ExtStrobeTop { control, .. } => control.accept(packet),
            Self::FbwCh5Sw { control, .. } => control.accept(packet),
            Self::FbwCh5SwCov { control, .. } => control.accept(packet),
            Self::FbwGLimSw { control, .. } => control.accept(packet),
            Self::FbwGainModeSw { control, .. } => control.accept(packet),
            Self::FbwGainModeSwCov { control, .. } => control.accept(packet),
            Self::FbwResetBtn { control, .. } => control.accept(packet),
            Self::FbwSpinModeSw { control, .. } => control.accept(packet),
            Self::FbwTestSw { control, .. } => control.accept(packet),
            Self::FbwTestSwCov { control, .. } => control.accept(packet),
            Self::FeuPc { control, .. } => control.accept(packet),
            Self::FeuSec { control, .. } => control.accept(packet),
            Self::FiringModeSel { control, .. } => control.accept(packet),
            Self::FormLgtSw { control, .. } => control.accept(packet),
            Self::FuelAvD { control, .. } => control.accept(packet),
            Self::FuelAvG { control, .. } => control.accept(packet),
            Self::FuelCrofSw { control, .. } => control.accept(packet),
            Self::FuelCutoffSw { control, .. } => control.accept(packet),
            Self::FuelCutoffSwCov { control, .. } => control.accept(packet),
            Self::FuelDetotCents { control, .. } => control.accept(packet),
            Self::FuelDetotSw { control, .. } => control.accept(packet),
            Self::FuelDetotTens { control, .. } => control.accept(packet),
            Self::FuelDetotThous { control, .. } => control.accept(packet),
            Self::FuelDmpSw { control, .. } => control.accept(packet),
            Self::FuelDmpSwCov { control, .. } => control.accept(packet),
            Self::FuelFlow { control, .. } => control.accept(packet),
            Self::FuelJaugCents { control, .. } => control.accept(packet),
            Self::FuelJaugD { control, .. } => control.accept(packet),
            Self::FuelJaugG { control, .. } => control.accept(packet),
            Self::FuelJaugTens { control, .. } => control.accept(packet),
            Self::FuelJaugThous { control, .. } => control.accept(packet),
            Self::FuelRlD { control, .. } => control.accept(packet),
            Self::FuelRlG { control, .. } => control.accept(packet),
            Self::FuelRlM { control, .. } => control.accept(packet),
            Self::FuelTransfSw { control, .. } => control.accept(packet),
            Self::FuelVD { control, .. } => control.accept(packet),
            Self::FuelVG { control, .. } => control.accept(packet),
            Self::GmeterNeedle { control, .. } => control.accept(packet),
            Self::GunArmSw { control, .. } => control.accept(packet),
            Self::GunModeBtn { control, .. } => control.accept(packet),
            Self::GunModeKl1 { control, .. } => control.accept(packet),
            Self::GunModeKl2 { control, .. } => control.accept(packet),
            Self::HsiCompas { control, .. } => control.accept(packet),
            Self::HsiDNeedle { control, .. } => control.accept(packet),
            Self::HsiDistCents { control, .. } => control.accept(packet),
            Self::HsiDistDec { control, .. } => control.accept(packet),
            Self::HsiDistOnes { control, .. } => control.accept(packet),
            Self::HsiDistTens { control, .. } => control.accept(packet),
            Self::HsiFlagD { control, .. } => control.accept(packet),
            Self::HsiFlagDist { control, .. } => control.accept(packet),
            Self::HsiFlagG { control, .. } => control.accept(packet),
            Self::HsiFlagM { control, .. } => control.accept(packet),
            Self::HsiHdg { control, .. } => control.accept(packet),
            Self::HsiModeSelSw { control, .. } => control.accept(packet),
            Self::HsiNeedle { control, .. } => control.accept(packet),
            Self::HsiVadSel { control, .. } => control.accept(packet),
            Self::HudClrBtn { control, .. } => control.accept(packet),
            Self::HudDecSw { control, .. } => control.accept(packet),
            Self::HudPwrSw { control, .. } => control.accept(packet),
            Self::HudRaltSelSw { control, .. } => control.accept(packet),
            Self::HudRec { control, .. } => control.accept(packet),
            Self::HydDNeedle { control, .. } => control.accept(packet),
            Self::HydFail { control, .. } => control.accept(packet),
            Self::HydGNeedle { control, .. } => control.accept(packet),
            Self::HydSysSel { control, .. } => control.accept(packet),
            Self::IdentPwrSw { control, .. } => control.accept(packet),
            Self::IffFault { control, .. } => control.accept(packet),
            Self::IffInd { control, .. } => control.accept(packet),
            Self::IffM4Sw { control, .. } => control.accept(packet),
            Self::IffRed { control, .. } => control.accept(packet),
            Self::InsAuxHdHor { control, .. } => control.accept(packet),
            Self::InsBtn0 { control, .. } => control.accept(packet),
            Self::InsBtn1 { control, .. } => control.accept(packet),
            Self::InsBtn2 { control, .. } => control.accept(packet),
            Self::InsBtn3 { control, .. } => control.accept(packet),
            Self::InsBtn4 { control, .. } => control.accept(packet),
            Self::InsBtn5 { control, .. } => control.accept(packet),
            Self::InsBtn6 { control, .. } => control.accept(packet),
            Self::InsBtn7 { control, .. } => control.accept(packet),
            Self::InsBtn8 { control, .. } => control.accept(packet),
            Self::InsBtn9 { control, .. } => control.accept(packet),
            Self::InsClrBtn { control, .. } => control.accept(packet),
            Self::InsDestSw { control, .. } => control.accept(packet),
            Self::InsDestSwLight { control, .. } => control.accept(packet),
            Self::InsEffSwLight { control, .. } => control.accept(packet),
            Self::InsEnterBtn { control, .. } => control.accept(packet),
            Self::InsInsSwLight { control, .. } => control.accept(packet),
            Self::InsModeSel { control, .. } => control.accept(packet),
            Self::InsNextWpBtn { control, .. } => control.accept(packet),
            Self::InsOpalMode { control, .. } => control.accept(packet),
            Self::InsParamSel { control, .. } => control.accept(packet),
            Self::InsPrepSw { control, .. } => control.accept(packet),
            Self::InsPrepSwLight { control, .. } => control.accept(packet),
            Self::InsPrevWpBtn { control, .. } => control.accept(packet),
            Self::InsUpdate { control, .. } => control.accept(packet),
            Self::IntakeSlatsOpSw { control, .. } => control.accept(packet),
            Self::LandingGearLeverLight { control, .. } => control.accept(packet),
            Self::LdgEmerRelLev { control, .. } => control.accept(packet),
            Self::LdgLev { control, .. } => control.accept(packet),
            Self::LdgLgtSw { control, .. } => control.accept(packet),
            Self::LgtBrightCtrlTest { control, .. } => control.accept(packet),
            Self::LgtTestSw { control, .. } => control.accept(packet),
            Self::LimInd { control, .. } => control.accept(packet),
            Self::LoxDilLev { control, .. } => control.accept(packet),
            Self::LoxEmerSup { control, .. } => control.accept(packet),
            Self::MagicStandby { control, .. } => control.accept(packet),
            Self::MagicToneVolKnob { control, .. } => control.accept(packet),
            Self::MagnetoSw { control, .. } => control.accept(packet),
            Self::MainBattSw { control, .. } => control.accept(packet),
            Self::MasterArmSw { control, .. } => control.accept(packet),
            Self::McAmbre { control, .. } => control.accept(packet),
            Self::McRouge { control, .. } => control.accept(packet),
            Self::MinAltSel { control, .. } => control.accept(packet),
            Self::MirrorToggle { control, .. } => control.accept(packet),
            Self::MisSelSw { control, .. } => control.accept(packet),
            Self::Mode110Sel { control, .. } => control.accept(packet),
            Self::Mode11Sel { control, .. } => control.accept(packet),
            Self::Mode1Sw { control, .. } => control.accept(packet),
            Self::Mode2Sw { control, .. } => control.accept(packet),
            Self::Mode3a100Sel { control, .. } => control.accept(packet),
            Self::Mode3a10Sel { control, .. } => control.accept(packet),
            Self::Mode3a1Sel { control, .. } => control.accept(packet),
            Self::Mode3a1kSel { control, .. } => control.accept(packet),
            Self::Mode3aSw { control, .. } => control.accept(packet),
            Self::ModeCSw { control, .. } => control.accept(packet),
            Self::MrkPos { control, .. } => control.accept(packet),
            Self::MrkSigVolKnob { control, .. } => control.accept(packet),
            Self::NRpm { control, .. } => control.accept(packet),
            Self::NavLgtSw { control, .. } => control.accept(packet),
            Self::NvgHelmetMount { control, .. } => control.accept(packet),
            Self::NvgLightFilterSw { control, .. } => control.accept(packet),
            Self::NvgStow { control, .. } => control.accept(packet),
            Self::NwsIffInterrBtn { control, .. } => control.accept(packet),
            Self::OffsetWpTgt { control, .. } => control.accept(packet),
            Self::OxyFlowInd { control, .. } => control.accept(packet),
            Self::OxyNeedle { control, .. } => control.accept(packet),
            Self::Pc { control, .. } => control.accept(packet),
            Self::PcaBr1Disp { control, .. } => control.accept(packet),
            Self::PcaBr2Disp { control, .. } => control.accept(packet),
            Self::PcaBr3Disp { control, .. } => control.accept(packet),
            Self::PcaBr4Disp { control, .. } => control.accept(packet),
            Self::PcaBr5Disp { control, .. } => control.accept(packet),
            Self::PcaBtn1 { control, .. } => control.accept(packet),
            Self::PcaBtn2 { control, .. } => control.accept(packet),
            Self::PcaBtn3 { control, .. } => control.accept(packet),
            Self::PcaBtn4 { control, .. } => control.accept(packet),
            Self::PcaBtn5 { control, .. } => control.accept(packet),
            Self::PcaButton1S { control, .. } => control.accept(packet),
            Self::PcaButton2S { control, .. } => control.accept(packet),
            Self::PcaButton3S { control, .. } => control.accept(packet),
            Self::PcaButton4S { control, .. } => control.accept(packet),
            Self::PcaButton5S { control, .. } => control.accept(packet),
            Self::PcaUr1Disp { control, .. } => control.accept(packet),
            Self::PcaUr2Disp { control, .. } => control.accept(packet),
            Self::PcaUr3Disp { control, .. } => control.accept(packet),
            Self::PcaUr4Disp { control, .. } => control.accept(packet),
            Self::PcaUr5Disp { control, .. } => control.accept(packet),
            Self::PcnAln { control, .. } => control.accept(packet),
            Self::PcnBad { control, .. } => control.accept(packet),
            Self::PcnDis2dl { control, .. } => control.accept(packet),
            Self::PcnDis2dr { control, .. } => control.accept(packet),
            Self::PcnDisDl { control, .. } => control.accept(packet),
            Self::PcnDisDr { control, .. } => control.accept(packet),
            Self::PcnDispDest { control, .. } => control.accept(packet),
            Self::PcnDispL { control, .. } => control.accept(packet),
            Self::PcnDispPrep { control, .. } => control.accept(packet),
            Self::PcnDispR { control, .. } => control.accept(packet),
            Self::PcnEncSwLight { control, .. } => control.accept(packet),
            Self::PcnM91Light { control, .. } => control.accept(packet),
            Self::PcnM92Light { control, .. } => control.accept(packet),
            Self::PcnM93Light { control, .. } => control.accept(packet),
            Self::PcnMip { control, .. } => control.accept(packet),
            Self::PcnMrqSwLight { control, .. } => control.accept(packet),
            Self::PcnNdeg { control, .. } => control.accept(packet),
            Self::PcnPret { control, .. } => control.accept(packet),
            Self::PcnRec { control, .. } => control.accept(packet),
            Self::PcnSec { control, .. } => control.accept(packet),
            Self::PcnUni { control, .. } => control.accept(packet),
            Self::PcnValSwLight { control, .. } => control.accept(packet),
            Self::PedalAdjustLev { control, .. } => control.accept(packet),
            Self::PitotHeatCov { control, .. } => control.accept(packet),
            Self::PitotHeatSw { control, .. } => control.accept(packet),
            Self::PkgBrakeLev { control, .. } => control.accept(packet),
            Self::PoliceLgtSw { control, .. } => control.accept(packet),
            Self::PpaAuto { control, .. } => control.accept(packet),
            Self::PpaGunRocketPap { control, .. } => control.accept(packet),
            Self::PpaGunRocketTot { control, .. } => control.accept(packet),
            Self::PpaIntDisp { control, .. } => control.accept(packet),
            Self::PpaMagicMis { control, .. } => control.accept(packet),
            Self::PpaMagicP { control, .. } => control.accept(packet),
            Self::PpaMan { control, .. } => control.accept(packet),
            Self::PpaQtyDisp { control, .. } => control.accept(packet),
            Self::PpaS530Mis { control, .. } => control.accept(packet),
            Self::PpaS530P { control, .. } => control.accept(packet),
            Self::PpaTestSw { control, .. } => control.accept(packet),
            Self::PressNeedle { control, .. } => control.accept(packet),
            Self::PsvDerive { control, .. } => control.accept(packet),
            Self::PsvElDExt { control, .. } => control.accept(packet),
            Self::PsvElDInt { control, .. } => control.accept(packet),
            Self::PsvElGExt { control, .. } => control.accept(packet),
            Self::PsvElGInt { control, .. } => control.accept(packet),
            Self::QraSw { control, .. } => control.accept(packet),
            Self::RadA { control, .. } => control.accept(packet),
            Self::RadAzSel { control, .. } => control.accept(packet),
            Self::RadContrastSw { control, .. } => control.accept(packet),
            Self::RadDec { control, .. } => control.accept(packet),
            Self::RadDopRejSw { control, .. } => control.accept(packet),
            Self::RadGainDial { control, .. } => control.accept(packet),
            Self::RadGridSelSw { control, .. } => control.accept(packet),
            Self::RadIffCode1Sel { control, .. } => control.accept(packet),
            Self::RadIffCode2Sel { control, .. } => control.accept(packet),
            Self::RadIffCode3Sel { control, .. } => control.accept(packet),
            Self::RadIffCode4Sel { control, .. } => control.accept(packet),
            Self::RadIffLRSel { control, .. } => control.accept(packet),
            Self::RadIffModeSw { control, .. } => control.accept(packet),
            Self::RadIffPowSw { control, .. } => control.accept(packet),
            Self::RadIllumSw { control, .. } => control.accept(packet),
            Self::RadPrfSw { control, .. } => control.accept(packet),
            Self::RadPsic { control, .. } => control.accept(packet),
            Self::RadPwrSel { control, .. } => control.accept(packet),
            Self::RadRaltPwrSw { control, .. } => control.accept(packet),
            Self::RadRearmBtn { control, .. } => control.accept(packet),
            Self::RadRngSelSw { control, .. } => control.accept(packet),
            Self::RadScanLinesSel { control, .. } => control.accept(packet),
            Self::RadTestBtn { control, .. } => control.accept(packet),
            Self::RadVal { control, .. } => control.accept(packet),
            Self::RadVisu { control, .. } => control.accept(packet),
            Self::RadWowEmAutSw { control, .. } => control.accept(packet),
            Self::RedFloodLgtKnob { control, .. } => control.accept(packet),
            Self::RefuelLgtBrightKnob { control, .. } => control.accept(packet),
            Self::RpmOnes { control, .. } => control.accept(packet),
            Self::RpmTens { control, .. } => control.accept(packet),
            Self::RwrBr { control, .. } => control.accept(packet),
            Self::RwrD2m { control, .. } => control.accept(packet),
            Self::RwrDa { control, .. } => control.accept(packet),
            Self::RwrLgtBrightCtrl { control, .. } => control.accept(packet),
            Self::RwrLl { control, .. } => control.accept(packet),
            Self::RwrPwrSw { control, .. } => control.accept(packet),
            Self::RwrV { control, .. } => control.accept(packet),
            Self::S530Standby { control, .. } => control.accept(packet),
            Self::SafeGndSw { control, .. } => control.accept(packet),
            Self::SbAdiCageSw { control, .. } => control.accept(packet),
            Self::SbAdiFlag { control, .. } => control.accept(packet),
            Self::SbAdiPitch { control, .. } => control.accept(packet),
            Self::SbAdiRoll { control, .. } => control.accept(packet),
            Self::SbAdiRot { control, .. } => control.accept(packet),
            Self::SeatAdjustSw { control, .. } => control.accept(packet),
            Self::SecHuileSw { control, .. } => control.accept(packet),
            Self::SecHuileSwCover { control, .. } => control.accept(packet),
            Self::SelAltHoldBtn { control, .. } => control.accept(packet),
            Self::SelJettSw { control, .. } => control.accept(packet),
            Self::SelJettSwCov { control, .. } => control.accept(packet),
            Self::SerpamRecSw { control, .. } => control.accept(packet),
            Self::SlatsOpSw { control, .. } => control.accept(packet),
            Self::SpeedKtsNeed { control, .. } => control.accept(packet),
            Self::SpeedMach { control, .. } => control.accept(packet),
            Self::StartFuelPumpSw { control, .. } => control.accept(packet),
            Self::T7Needle { control, .. } => control.accept(packet),
            Self::TacCh10Sel { control, .. } => control.accept(packet),
            Self::TacCh1Sel { control, .. } => control.accept(packet),
            Self::TacModeSel { control, .. } => control.accept(packet),
            Self::TacVolKnob { control, .. } => control.accept(packet),
            Self::TacXYSel { control, .. } => control.accept(packet),
            Self::TbAppVolKnob { control, .. } => control.accept(packet),
            Self::TdcModeSw { control, .. } => control.accept(packet),
            Self::TestRouge { control, .. } => control.accept(packet),
            Self::TestVert { control, .. } => control.accept(packet),
            Self::TgtAge { control, .. } => control.accept(packet),
            Self::TgtAlt { control, .. } => control.accept(packet),
            Self::TgtBearBe { control, .. } => control.accept(packet),
            Self::TgtDataMnlEntry { control, .. } => control.accept(packet),
            Self::TgtHd { control, .. } => control.accept(packet),
            Self::TgtMachNum { control, .. } => control.accept(packet),
            Self::TgtMemTimeSelSw { control, .. } => control.accept(packet),
            Self::TgtRngBe { control, .. } => control.accept(packet),
            Self::TgtWingspanKnob { control, .. } => control.accept(packet),
            Self::UhfCdeSw { control, .. } => control.accept(packet),
            Self::UhfEA2Sw { control, .. } => control.accept(packet),
            Self::UhfFrequency { control, .. } => control.accept(packet),
            Self::UhfModeSw { control, .. } => control.accept(packet),
            Self::UhfPreset { control, .. } => control.accept(packet),
            Self::UhfPresetKnobInt { control, .. } => control.accept(packet),
            Self::UhfPresetKnobStr { control, .. } => control.accept(packet),
            Self::UhfPwr5w25wSw { control, .. } => control.accept(packet),
            Self::UhfRadioVolKnob { control, .. } => control.accept(packet),
            Self::UhfRepL1 { control, .. } => control.accept(packet),
            Self::UhfRepL2 { control, .. } => control.accept(packet),
            Self::UhfSecureL { control, .. } => control.accept(packet),
            Self::UhfSilSw { control, .. } => control.accept(packet),
            Self::UhfTestL { control, .. } => control.accept(packet),
            Self::UhfTestSw { control, .. } => control.accept(packet),
            Self::ValDataEntry { control, .. } => control.accept(packet),
            Self::VarioNeedle { control, .. } => control.accept(packet),
            Self::VentSelSw { control, .. } => control.accept(packet),
            Self::Vhf0 { control, .. } => control.accept(packet),
            Self::Vhf1Read { control, .. } => control.accept(packet),
            Self::Vhf2Sql { control, .. } => control.accept(packet),
            Self::Vhf3Gr { control, .. } => control.accept(packet),
            Self::Vhf4 { control, .. } => control.accept(packet),
            Self::Vhf520Low { control, .. } => control.accept(packet),
            Self::Vhf6Tone { control, .. } => control.accept(packet),
            Self::Vhf7 { control, .. } => control.accept(packet),
            Self::Vhf8Tod { control, .. } => control.accept(packet),
            Self::Vhf9Zero { control, .. } => control.accept(packet),
            Self::VhfChSel { control, .. } => control.accept(packet),
            Self::VhfConf { control, .. } => control.accept(packet),
            Self::VhfFrequency { control, .. } => control.accept(packet),
            Self::VhfMemClr { control, .. } => control.accept(packet),
            Self::VhfMode { control, .. } => control.accept(packet),
            Self::VhfVldXfr { control, .. } => control.accept(packet),
            Self::Vorils001Drum { control, .. } => control.accept(packet),
            Self::Vorils01Drum { control, .. } => control.accept(packet),
            Self::Vorils100Drum { control, .. } => control.accept(packet),
            Self::Vorils10Drum { control, .. } => control.accept(packet),
            Self::Vorils1Drum { control, .. } => control.accept(packet),
            Self::VorilsFreqDecimal { control, .. } => control.accept(packet),
            Self::VorilsFreqWhole { control, .. } => control.accept(packet),
            Self::VorilsPwrDial { control, .. } => control.accept(packet),
            Self::VorilsTestDial { control, .. } => control.accept(packet),
            Self::VorilsVolKnob { control, .. } => control.accept(packet),
            Self::Vtac10Drum { control, .. } => control.accept(packet),
            Self::Vtac1Drum { control, .. } => control.accept(packet),
            Self::VtacXYDrum { control, .. } => control.accept(packet),
            Self::VtbDec { control, .. } => control.accept(packet),
            Self::VtbDispBright { control, .. } => control.accept(packet),
            Self::VtbDispContrast { control, .. } => control.accept(packet),
            Self::VtbIconsBright { control, .. } => control.accept(packet),
            Self::VtbOrSel { control, .. } => control.accept(packet),
            Self::VtbPwrSw { control, .. } => control.accept(packet),
            Self::VtbVideoBright { control, .. } => control.accept(packet),
            Self::VuhfRadioVolKnob { control, .. } => control.accept(packet),
            Self::Weapon1R { control, .. } => control.accept(packet),
            Self::Weapon1S { control, .. } => control.accept(packet),
            Self::Weapon2R { control, .. } => control.accept(packet),
            Self::Weapon2S { control, .. } => control.accept(packet),
            Self::Weapon3R { control, .. } => control.accept(packet),
            Self::Weapon3S { control, .. } => control.accept(packet),
            Self::Weapon4R { control, .. } => control.accept(packet),
            Self::Weapon4S { control, .. } => control.accept(packet),
            Self::Weapon5R { control, .. } => control.accept(packet),
            Self::Weapon5S { control, .. } => control.accept(packet),
            Self::WhFloodLgtKnob { control, .. } => control.accept(packet),
            Self::WpsSelBtn1 { control, .. } => control.accept(packet),
            Self::WpsSelBtn2 { control, .. } => control.accept(packet),
            Self::WpsSelBtn3 { control, .. } => control.accept(packet),
            Self::WpsSelBtn4 { control, .. } => control.accept(packet),
            Self::WpsSelBtn5 { control, .. } => control.accept(packet),
        }
    }

    fn commit(&mut self) {
        match self {
            Self::AaRadSttSelBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <AaRadSttSelBtnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AarSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <AarSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::AbEmerCutoffSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <AbEmerCutoffSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AbEmerCutoffSwCov { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <AbEmerCutoffSwCovValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AdiBille { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdiBklSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <AdiBklSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::AdiCageLev { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <AdiCageLevValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::AdiCompas { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdiFlag { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdiIlsH { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdiIlsV { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdiLight { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdiPitch { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AdiRoll { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AgGunRetSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <AgGunRetSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::AgRadAModeSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <AgRadAModeSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::AgRadDecModeSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <AgRadDecModeSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AgRadVisuModeSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <AgRadVisuModeSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AirRefuelL { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Alt100FtSel { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <Alt100FtSelValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Alt10kFtSel { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <Alt10kFtSelValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Alt1Sw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <Alt1SwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Alt1kFtSel { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <Alt1kFtSelValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Alt2Sw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <Alt2SwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::AltBaroCents { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AltBaroOnes { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AltBaroTens { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AltBaroThous { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AltCents { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AltHoldBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <AltHoldBtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::AltKcents { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AltNeedle { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AltThous { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AmplisSelKnob { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <AmplisSelKnobValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::AntiColLgtSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <AntiColLgtSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::AntiSkidSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <AntiSkidSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::AntiSkidSwCov { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <AntiSkidSwCovValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::AoaFlag { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::AoaPos { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ApAltAmbre { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ApAltVert { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ApGAmbre { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ApGVert { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ApLAmbre { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ApLVert { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ApLgtTestBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <ApLgtTestBtnValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::ApMasterAmbre { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ApMasterBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ApMasterBtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::ApMasterVert { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ApNcAmbre { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ApNcVert { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ApSelectAltAmbre { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ApSelectAltVert { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ApTestSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ApTestSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::ApTestSwCov { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ApTestSwCovValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::AppHoldBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <AppHoldBtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::AudioWarnReset { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <AudioWarnResetValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::AudioWarnSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <AudioWarnSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::AutoManBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <AutoManBtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::AuxGunsight { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <AuxGunsightValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::AuxGunsightDefl { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::BaroPressCalib { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::BeWpSel { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <BeWpSelValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::BingoFuel100KgSel { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <BingoFuel100KgSelValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::BingoFuel1kKgSel { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <BingoFuel1kKgSelValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::BombFuzeSelSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <BombFuzeSelSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::BombRelIntSelSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <BombRelIntSelSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::BombRelQtySelSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <BombRelQtySelSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::BrPwrSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <BrPwrSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CautAdvLgt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ClimC { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClimF { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClkH { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ClkM { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ClkS { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Clp5mnO2 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpAlt1 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpAlt2 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpAnemo { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpBatt { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpBecs { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpBingo { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpBp { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpBpD { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpBpG { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpCalc { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpCc { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpCondit { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpConf { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpDecol { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpDom { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpDsv { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpEp { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpGain { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpHuile { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpHyd1 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpHyd2 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpHydS { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpIncidence { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpMan { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpNiveau { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpO2Ha { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpPCab { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpPa { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpPark { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpPelles { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpRegO2 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpRpm { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpSouris { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpT7 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpTemp { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpTransf { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpTrn { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ClpUsEl { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::CnpyHand1 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CnpyHand1Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CnpyHand2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CnpyHand2Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CnpyJett { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CnpyJettValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CnpyLkNLowLev { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <CnpyLkNLowLevValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::CnpyRest { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CnpyRestValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CocClock { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CocClockValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::ConesOpSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ConesOpSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::ConfA { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ConfAux { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ConfCross { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ConfD { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ConfDirav { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ConfF { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ConfFrein { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ConfG { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ConfRouge { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ConfSpad { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ConsPanelLgtKnob { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::D2mPwrSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <D2mPwrSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::DashPanelLgtKnob { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DashUVLgtKnob { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::DecoyRelModeSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <DecoyRelModeSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::DecoyRelPgmKnob { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <DecoyRelPgmKnobValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::Dem { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::DragChuteLev { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <DragChuteLevValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::EcmBoxBltSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <EcmBoxBltSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::EcmBoxEm { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::EcmBoxEo { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::EcmBoxIr { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::EcmBoxLl { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::EcsAirExchSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <EcsAirExchSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::EcsCBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <EcsCBtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::EcsCondSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <EcsCondSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::EcsDefogSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <EcsDefogSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::EcsFBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <EcsFBtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::EcsMainModeSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <EcsMainModeSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::EcsTempSelectKnob { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ElecFail { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::ElecPwrTransfSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ElecPwrTransfSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::EmerCompass { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <EmerCompassValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::EmerHydPumpSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <EmerHydPumpSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::EmerJett { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <EmerJettValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::EmerThrottleCov { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <EmerThrottleCovValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::EmerThrottleHand { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::EngEmerCtrlCov { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <EngEmerCtrlCovValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::EngEmerCtrlSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <EngEmerCtrlSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::EngFuelLPumpSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <EngFuelLPumpSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::EngFuelRPumpSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <EngFuelRPumpSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::EngInFlgtStartSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <EngInFlgtStartSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::EngShutdownBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <EngShutdownBtnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::EngStartSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <EngStartSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::EngStartSwCov { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <EngStartSwCovValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::EwModeSelSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <EwModeSelSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::ExtFormationLightsAft { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ExtFormationLightsFront { control, callback } => {
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
            Self::ExtStrobeBottom { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::ExtStrobeTop { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FbwCh5Sw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <FbwCh5SwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FbwCh5SwCov { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <FbwCh5SwCovValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FbwGLimSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <FbwGLimSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FbwGainModeSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <FbwGainModeSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::FbwGainModeSwCov { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <FbwGainModeSwCovValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::FbwResetBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <FbwResetBtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FbwSpinModeSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <FbwSpinModeSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::FbwTestSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <FbwTestSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FbwTestSwCov { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <FbwTestSwCovValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::FeuPc { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::FeuSec { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::FiringModeSel { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <FiringModeSelValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::FormLgtSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <FormLgtSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FuelAvD { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::FuelAvG { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::FuelCrofSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <FuelCrofSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FuelCutoffSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <FuelCutoffSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::FuelCutoffSwCov { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <FuelCutoffSwCovValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::FuelDetotCents { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelDetotSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <FuelDetotSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FuelDetotTens { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelDetotThous { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelDmpSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <FuelDmpSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FuelDmpSwCov { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <FuelDmpSwCovValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::FuelFlow { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelJaugCents { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelJaugD { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelJaugG { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelJaugTens { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelJaugThous { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::FuelRlD { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::FuelRlG { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::FuelRlM { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::FuelTransfSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <FuelTransfSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::FuelVD { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::FuelVG { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::GmeterNeedle { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::GunArmSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <GunArmSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::GunModeBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <GunModeBtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::GunModeKl1 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::GunModeKl2 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::HsiCompas { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HsiDNeedle { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HsiDistCents { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HsiDistDec { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HsiDistOnes { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HsiDistTens { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HsiFlagD { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HsiFlagDist { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HsiFlagG { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HsiFlagM { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HsiHdg { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HsiModeSelSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <HsiModeSelSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::HsiNeedle { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HsiVadSel { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HudClrBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <HudClrBtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::HudDecSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <HudDecSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::HudPwrSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <HudPwrSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::HudRaltSelSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <HudRaltSelSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::HudRec { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::HydDNeedle { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HydFail { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::HydGNeedle { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::HydSysSel { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <HydSysSelValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IdentPwrSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IdentPwrSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IffFault { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::IffInd { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::IffM4Sw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IffM4SwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IffRed { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::InsAuxHdHor { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <InsAuxHdHorValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::InsBtn0 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <InsBtn0Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::InsBtn1 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <InsBtn1Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::InsBtn2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <InsBtn2Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::InsBtn3 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <InsBtn3Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::InsBtn4 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <InsBtn4Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::InsBtn5 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <InsBtn5Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::InsBtn6 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <InsBtn6Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::InsBtn7 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <InsBtn7Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::InsBtn8 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <InsBtn8Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::InsBtn9 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <InsBtn9Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::InsClrBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <InsClrBtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::InsDestSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <InsDestSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::InsDestSwLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::InsEffSwLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::InsEnterBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <InsEnterBtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::InsInsSwLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::InsModeSel { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <InsModeSelValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::InsNextWpBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <InsNextWpBtnValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::InsOpalMode { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <InsOpalModeValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::InsParamSel { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::InsPrepSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <InsPrepSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::InsPrepSwLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::InsPrevWpBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <InsPrevWpBtnValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::InsUpdate { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <InsUpdateValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IntakeSlatsOpSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IntakeSlatsOpSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LandingGearLeverLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LdgEmerRelLev { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <LdgEmerRelLevValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::LdgLev { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <LdgLevValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::LdgLgtSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <LdgLgtSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::LgtBrightCtrlTest { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::LgtTestSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <LgtTestSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::LimInd { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::LoxDilLev { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <LoxDilLevValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::LoxEmerSup { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <LoxEmerSupValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MagicStandby { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <MagicStandbyValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::MagicToneVolKnob { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MagnetoSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MagnetoSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MainBattSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MainBattSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MasterArmSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MasterArmSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::McAmbre { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::McRouge { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::MinAltSel { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::MirrorToggle { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <MirrorToggleValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::MisSelSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MisSelSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Mode110Sel { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <Mode110SelValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Mode11Sel { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <Mode11SelValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Mode1Sw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <Mode1SwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Mode2Sw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <Mode2SwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Mode3a100Sel { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <Mode3a100SelValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::Mode3a10Sel { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <Mode3a10SelValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Mode3a1Sel { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <Mode3a1SelValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Mode3a1kSel { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <Mode3a1kSelValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Mode3aSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <Mode3aSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::ModeCSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ModeCSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MrkPos { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MrkPosValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MrkSigVolKnob { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::NRpm { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::NavLgtSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <NavLgtSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::NvgHelmetMount { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <NvgHelmetMountValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::NvgLightFilterSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <NvgLightFilterSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::NvgStow { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <NvgStowValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::NwsIffInterrBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <NwsIffInterrBtnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::OffsetWpTgt { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <OffsetWpTgtValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::OxyFlowInd { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::OxyNeedle { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Pc { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::PcaBr1Disp { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PcaBr2Disp { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PcaBr3Disp { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PcaBr4Disp { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PcaBr5Disp { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PcaBtn1 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <PcaBtn1Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::PcaBtn2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <PcaBtn2Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::PcaBtn3 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <PcaBtn3Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::PcaBtn4 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <PcaBtn4Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::PcaBtn5 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <PcaBtn5Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::PcaButton1S { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::PcaButton2S { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::PcaButton3S { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::PcaButton4S { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::PcaButton5S { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::PcaUr1Disp { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PcaUr2Disp { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PcaUr3Disp { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PcaUr4Disp { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PcaUr5Disp { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PcnAln { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::PcnBad { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::PcnDis2dl { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PcnDis2dr { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PcnDisDl { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PcnDisDr { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PcnDispDest { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PcnDispL { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PcnDispPrep { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PcnDispR { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PcnEncSwLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::PcnM91Light { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::PcnM92Light { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::PcnM93Light { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::PcnMip { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::PcnMrqSwLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::PcnNdeg { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::PcnPret { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::PcnRec { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::PcnSec { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::PcnUni { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::PcnValSwLight { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::PedalAdjustLev { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <PedalAdjustLevValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::PitotHeatCov { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <PitotHeatCovValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::PitotHeatSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <PitotHeatSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::PkgBrakeLev { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <PkgBrakeLevValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::PoliceLgtSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <PoliceLgtSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::PpaAuto { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::PpaGunRocketPap { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::PpaGunRocketTot { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::PpaIntDisp { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PpaMagicMis { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::PpaMagicP { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::PpaMan { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::PpaQtyDisp { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PpaS530Mis { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::PpaS530P { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::PpaTestSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <PpaTestSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::PressNeedle { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PsvDerive { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PsvElDExt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PsvElDInt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PsvElGExt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::PsvElGInt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::QraSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <QraSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RadA { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RadAzSel { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <RadAzSelValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RadContrastSw { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RadDec { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RadDopRejSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <RadDopRejSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RadGainDial { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RadGridSelSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <RadGridSelSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::RadIffCode1Sel { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <RadIffCode1SelValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RadIffCode2Sel { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <RadIffCode2SelValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RadIffCode3Sel { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <RadIffCode3SelValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RadIffCode4Sel { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <RadIffCode4SelValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RadIffLRSel { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <RadIffLRSelValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RadIffModeSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <RadIffModeSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::RadIffPowSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <RadIffPowSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RadIllumSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <RadIllumSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RadPrfSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <RadPrfSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RadPsic { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RadPwrSel { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <RadPwrSelValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RadRaltPwrSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <RadRaltPwrSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::RadRearmBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <RadRearmBtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RadRngSelSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <RadRngSelSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RadScanLinesSel { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <RadScanLinesSelValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::RadTestBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <RadTestBtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RadVal { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RadVisu { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RadWowEmAutSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <RadWowEmAutSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::RedFloodLgtKnob { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RefuelLgtBrightKnob { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RpmOnes { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RpmTens { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RwrBr { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RwrD2m { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RwrDa { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RwrLgtBrightCtrl { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::RwrLl { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::RwrPwrSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <RwrPwrSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RwrV { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::S530Standby { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <S530StandbyValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::SafeGndSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <SafeGndSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::SbAdiCageSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <SbAdiCageSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::SbAdiFlag { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SbAdiPitch { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SbAdiRoll { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SbAdiRot { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SeatAdjustSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <SeatAdjustSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::SecHuileSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <SecHuileSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::SecHuileSwCover { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <SecHuileSwCoverValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::SelAltHoldBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <SelAltHoldBtnValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::SelJettSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <SelJettSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::SelJettSwCov { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <SelJettSwCovValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::SerpamRecSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <SerpamRecSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::SlatsOpSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <SlatsOpSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::SpeedKtsNeed { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::SpeedMach { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::StartFuelPumpSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <StartFuelPumpSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::T7Needle { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TacCh10Sel { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TacCh1Sel { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <TacCh1SelValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::TacModeSel { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <TacModeSelValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::TacVolKnob { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TacXYSel { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <TacXYSelValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::TbAppVolKnob { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::TdcModeSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <TdcModeSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::TestRouge { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::TestVert { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::TgtAge { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <TgtAgeValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::TgtAlt { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <TgtAltValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::TgtBearBe { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <TgtBearBeValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::TgtDataMnlEntry { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <TgtDataMnlEntryValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::TgtHd { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <TgtHdValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::TgtMachNum { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <TgtMachNumValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::TgtMemTimeSelSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <TgtMemTimeSelSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::TgtRngBe { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <TgtRngBeValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::TgtWingspanKnob { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UhfCdeSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <UhfCdeSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfEA2Sw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <UhfEA2SwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfFrequency { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UhfModeSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <UhfModeSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfPreset { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UhfPresetKnobInt { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UhfPresetKnobStr { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UhfPwr5w25wSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <UhfPwr5w25wSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::UhfRadioVolKnob { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::UhfRepL1 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::UhfRepL2 { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::UhfSecureL { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::UhfSilSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <UhfSilSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfTestL { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::UhfTestSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <UhfTestSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::ValDataEntry { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <ValDataEntryValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::VarioNeedle { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VentSelSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <VentSelSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Vhf0 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <Vhf0Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Vhf1Read { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <Vhf1ReadValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Vhf2Sql { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <Vhf2SqlValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Vhf3Gr { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <Vhf3GrValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Vhf4 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <Vhf4Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Vhf520Low { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <Vhf520LowValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Vhf6Tone { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <Vhf6ToneValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Vhf7 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <Vhf7Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Vhf8Tod { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <Vhf8TodValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Vhf9Zero { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <Vhf9ZeroValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::VhfChSel { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VhfConf { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <VhfConfValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::VhfFrequency { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VhfMemClr { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <VhfMemClrValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::VhfMode { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <VhfModeValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::VhfVldXfr { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <VhfVldXfrValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Vorils001Drum { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Vorils01Drum { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Vorils100Drum { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Vorils10Drum { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Vorils1Drum { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VorilsFreqDecimal { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VorilsFreqWhole { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VorilsPwrDial { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <VorilsPwrDialValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::VorilsTestDial { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <VorilsTestDialValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::VorilsVolKnob { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Vtac10Drum { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Vtac1Drum { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VtacXYDrum { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::VtbDec { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <VtbDecValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::VtbDispBright { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <VtbDispBrightValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::VtbDispContrast { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <VtbDispContrastValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::VtbIconsBright { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <VtbIconsBrightValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::VtbOrSel { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <VtbOrSelValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::VtbPwrSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <VtbPwrSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::VtbVideoBright { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <VtbVideoBrightValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::VuhfRadioVolKnob { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::Weapon1R { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Weapon1S { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Weapon2R { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Weapon2S { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Weapon3R { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Weapon3S { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Weapon4R { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Weapon4S { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Weapon5R { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::Weapon5S { control, callback } => {
                if control.commit() {
                    callback(control.is_on());
                }
            }
            Self::WhFloodLgtKnob { control, callback } => {
                if control.commit() {
                    callback(control.value());
                }
            }
            Self::WpsSelBtn1 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <WpsSelBtn1Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::WpsSelBtn2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <WpsSelBtn2Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::WpsSelBtn3 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <WpsSelBtn3Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::WpsSelBtn4 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <WpsSelBtn4Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::WpsSelBtn5 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <WpsSelBtn5Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
        }
    }
}

pub struct M2000cBuilder<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> {
    controls: Vec<RegisteredControl<'a>, MAX_CONTROLS>,
    _packet_capacity: PhantomData<ProtocolParser<MAX_PACKET_DATA>>,
}

impl<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize>
    M2000cBuilder<'a, MAX_CONTROLS, MAX_PACKET_DATA>
{
    pub fn new() -> Self {
        Self {
            controls: Vec::new(),
            _packet_capacity: PhantomData,
        }
    }

    pub fn register_aa_rad_stt_sel_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(AaRadSttSelBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AaRadSttSelBtn {
                control: AaRadSttSelBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_aar_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(AarSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AarSw {
                control: AarSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ab_emer_cutoff_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(AbEmerCutoffSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AbEmerCutoffSw {
                control: AbEmerCutoffSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ab_emer_cutoff_sw_cov<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(AbEmerCutoffSwCovValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AbEmerCutoffSwCov {
                control: AbEmerCutoffSwCov::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_bille<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiBille {
                control: AdiBille::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_bkl_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(AdiBklSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiBklSw {
                control: AdiBklSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_cage_lev<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(AdiCageLevValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiCageLev {
                control: AdiCageLev::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_compas<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiCompas {
                control: AdiCompas::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_flag<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiFlag {
                control: AdiFlag::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_ils_h<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiIlsH {
                control: AdiIlsH::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_ils_v<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiIlsV {
                control: AdiIlsV::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_light<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiLight {
                control: AdiLight::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_pitch<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiPitch {
                control: AdiPitch::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_roll<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiRoll {
                control: AdiRoll::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ag_gun_ret_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(AgGunRetSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AgGunRetSw {
                control: AgGunRetSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ag_rad_a_mode_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(AgRadAModeSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AgRadAModeSw {
                control: AgRadAModeSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ag_rad_dec_mode_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(AgRadDecModeSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AgRadDecModeSw {
                control: AgRadDecModeSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ag_rad_visu_mode_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(AgRadVisuModeSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AgRadVisuModeSw {
                control: AgRadVisuModeSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_air_refuel_l<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::AirRefuelL {
                control: AirRefuelL::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt100_ft_sel<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Alt100FtSelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Alt100FtSel {
                control: Alt100FtSel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt10k_ft_sel<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Alt10kFtSelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Alt10kFtSel {
                control: Alt10kFtSel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt1_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Alt1SwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Alt1Sw {
                control: Alt1Sw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt1k_ft_sel<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Alt1kFtSelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Alt1kFtSel {
                control: Alt1kFtSel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt2_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Alt2SwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Alt2Sw {
                control: Alt2Sw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_baro_cents<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltBaroCents {
                control: AltBaroCents::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_baro_ones<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltBaroOnes {
                control: AltBaroOnes::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_baro_tens<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltBaroTens {
                control: AltBaroTens::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_baro_thous<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltBaroThous {
                control: AltBaroThous::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_cents<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltCents {
                control: AltCents::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_hold_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(AltHoldBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltHoldBtn {
                control: AltHoldBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_kcents<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltKcents {
                control: AltKcents::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_needle<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltNeedle {
                control: AltNeedle::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_thous<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltThous {
                control: AltThous::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_amplis_sel_knob<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(AmplisSelKnobValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AmplisSelKnob {
                control: AmplisSelKnob::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_anti_col_lgt_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(AntiColLgtSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AntiColLgtSw {
                control: AntiColLgtSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_anti_skid_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(AntiSkidSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AntiSkidSw {
                control: AntiSkidSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_anti_skid_sw_cov<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(AntiSkidSwCovValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AntiSkidSwCov {
                control: AntiSkidSwCov::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_aoa_flag<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AoaFlag {
                control: AoaFlag::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_aoa_pos<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AoaPos {
                control: AoaPos::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ap_alt_ambre<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ApAltAmbre {
                control: ApAltAmbre::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ap_alt_vert<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ApAltVert {
                control: ApAltVert::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ap_g_ambre<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ApGAmbre {
                control: ApGAmbre::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ap_g_vert<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ApGVert {
                control: ApGVert::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ap_l_ambre<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ApLAmbre {
                control: ApLAmbre::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ap_l_vert<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ApLVert {
                control: ApLVert::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ap_lgt_test_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ApLgtTestBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ApLgtTestBtn {
                control: ApLgtTestBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ap_master_ambre<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ApMasterAmbre {
                control: ApMasterAmbre::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ap_master_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ApMasterBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ApMasterBtn {
                control: ApMasterBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ap_master_vert<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ApMasterVert {
                control: ApMasterVert::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ap_nc_ambre<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ApNcAmbre {
                control: ApNcAmbre::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ap_nc_vert<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ApNcVert {
                control: ApNcVert::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ap_select_alt_ambre<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ApSelectAltAmbre {
                control: ApSelectAltAmbre::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ap_select_alt_vert<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ApSelectAltVert {
                control: ApSelectAltVert::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ap_test_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ApTestSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ApTestSw {
                control: ApTestSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ap_test_sw_cov<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ApTestSwCovValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ApTestSwCov {
                control: ApTestSwCov::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_app_hold_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(AppHoldBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AppHoldBtn {
                control: AppHoldBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_audio_warn_reset<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(AudioWarnResetValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AudioWarnReset {
                control: AudioWarnReset::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_audio_warn_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(AudioWarnSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AudioWarnSw {
                control: AudioWarnSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_auto_man_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(AutoManBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AutoManBtn {
                control: AutoManBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_aux_gunsight<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(AuxGunsightValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AuxGunsight {
                control: AuxGunsight::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_aux_gunsight_defl<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AuxGunsightDefl {
                control: AuxGunsightDefl::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_baro_press_calib<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BaroPressCalib {
                control: BaroPressCalib::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_be_wp_sel<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(BeWpSelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::BeWpSel {
                control: BeWpSel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_bingo_fuel100_kg_sel<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(BingoFuel100KgSelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::BingoFuel100KgSel {
                control: BingoFuel100KgSel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_bingo_fuel1k_kg_sel<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(BingoFuel1kKgSelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::BingoFuel1kKgSel {
                control: BingoFuel1kKgSel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_bomb_fuze_sel_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(BombFuzeSelSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::BombFuzeSelSw {
                control: BombFuzeSelSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_bomb_rel_int_sel_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(BombRelIntSelSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::BombRelIntSelSw {
                control: BombRelIntSelSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_bomb_rel_qty_sel_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(BombRelQtySelSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::BombRelQtySelSw {
                control: BombRelQtySelSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_br_pwr_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(BrPwrSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::BrPwrSw {
                control: BrPwrSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_caut_adv_lgt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CautAdvLgt {
                control: CautAdvLgt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clim_c<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClimC {
                control: ClimC::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clim_f<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClimF {
                control: ClimF::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clk_h<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClkH {
                control: ClkH::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clk_m<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClkM {
                control: ClkM::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clk_s<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClkS {
                control: ClkS::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp5mn_o2<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Clp5mnO2 {
                control: Clp5mnO2::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_alt1<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpAlt1 {
                control: ClpAlt1::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_alt2<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpAlt2 {
                control: ClpAlt2::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_anemo<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpAnemo {
                control: ClpAnemo::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_batt<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpBatt {
                control: ClpBatt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_becs<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpBecs {
                control: ClpBecs::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_bingo<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpBingo {
                control: ClpBingo::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_bp<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpBp {
                control: ClpBp::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_bp_d<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpBpD {
                control: ClpBpD::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_bp_g<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpBpG {
                control: ClpBpG::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_calc<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpCalc {
                control: ClpCalc::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_cc<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpCc {
                control: ClpCc::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_condit<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpCondit {
                control: ClpCondit::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_conf<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpConf {
                control: ClpConf::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_decol<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpDecol {
                control: ClpDecol::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_dom<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpDom {
                control: ClpDom::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_dsv<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpDsv {
                control: ClpDsv::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_ep<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpEp {
                control: ClpEp::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_gain<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpGain {
                control: ClpGain::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_huile<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpHuile {
                control: ClpHuile::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_hyd1<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpHyd1 {
                control: ClpHyd1::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_hyd2<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpHyd2 {
                control: ClpHyd2::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_hyd_s<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpHydS {
                control: ClpHydS::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_incidence<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpIncidence {
                control: ClpIncidence::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_man<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpMan {
                control: ClpMan::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_niveau<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpNiveau {
                control: ClpNiveau::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_o2_ha<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpO2Ha {
                control: ClpO2Ha::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_p_cab<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpPCab {
                control: ClpPCab::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_pa<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpPa {
                control: ClpPa::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_park<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpPark {
                control: ClpPark::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_pelles<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpPelles {
                control: ClpPelles::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_reg_o2<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpRegO2 {
                control: ClpRegO2::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_rpm<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpRpm {
                control: ClpRpm::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_souris<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpSouris {
                control: ClpSouris::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_t7<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpT7 {
                control: ClpT7::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_temp<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpTemp {
                control: ClpTemp::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_transf<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpTransf {
                control: ClpTransf::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_trn<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpTrn {
                control: ClpTrn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_us_el<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpUsEl {
                control: ClpUsEl::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cnpy_hand1<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CnpyHand1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::CnpyHand1 {
                control: CnpyHand1::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cnpy_hand2<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CnpyHand2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::CnpyHand2 {
                control: CnpyHand2::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cnpy_jett<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CnpyJettValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CnpyJett {
                control: CnpyJett::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cnpy_lk_n_low_lev<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CnpyLkNLowLevValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CnpyLkNLowLev {
                control: CnpyLkNLowLev::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cnpy_rest<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CnpyRestValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CnpyRest {
                control: CnpyRest::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_coc_clock<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CocClockValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CocClock {
                control: CocClock::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cones_op_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ConesOpSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ConesOpSw {
                control: ConesOpSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_conf_a<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ConfA {
                control: ConfA::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_conf_aux<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ConfAux {
                control: ConfAux::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_conf_cross<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ConfCross {
                control: ConfCross::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_conf_d<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ConfD {
                control: ConfD::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_conf_dirav<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ConfDirav {
                control: ConfDirav::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_conf_f<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ConfF {
                control: ConfF::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_conf_frein<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ConfFrein {
                control: ConfFrein::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_conf_g<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ConfG {
                control: ConfG::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_conf_rouge<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ConfRouge {
                control: ConfRouge::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_conf_spad<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ConfSpad {
                control: ConfSpad::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cons_panel_lgt_knob<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ConsPanelLgtKnob {
                control: ConsPanelLgtKnob::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_d2m_pwr_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(D2mPwrSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::D2mPwrSw {
                control: D2mPwrSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dash_panel_lgt_knob<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DashPanelLgtKnob {
                control: DashPanelLgtKnob::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dash_u_v_lgt_knob<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DashUVLgtKnob {
                control: DashUVLgtKnob::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_decoy_rel_mode_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(DecoyRelModeSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::DecoyRelModeSw {
                control: DecoyRelModeSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_decoy_rel_pgm_knob<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(DecoyRelPgmKnobValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::DecoyRelPgmKnob {
                control: DecoyRelPgmKnob::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dem<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Dem {
                control: Dem::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_drag_chute_lev<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(DragChuteLevValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::DragChuteLev {
                control: DragChuteLev::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecm_box_blt_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EcmBoxBltSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EcmBoxBltSw {
                control: EcmBoxBltSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecm_box_em<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::EcmBoxEm {
                control: EcmBoxEm::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecm_box_eo<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::EcmBoxEo {
                control: EcmBoxEo::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecm_box_ir<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::EcmBoxIr {
                control: EcmBoxIr::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecm_box_ll<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::EcmBoxLl {
                control: EcmBoxLl::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecs_air_exch_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EcsAirExchSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EcsAirExchSw {
                control: EcsAirExchSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecs_c_btn<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EcsCBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EcsCBtn {
                control: EcsCBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecs_cond_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EcsCondSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EcsCondSw {
                control: EcsCondSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecs_defog_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EcsDefogSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EcsDefogSw {
                control: EcsDefogSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecs_f_btn<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EcsFBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EcsFBtn {
                control: EcsFBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecs_main_mode_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EcsMainModeSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EcsMainModeSw {
                control: EcsMainModeSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecs_temp_select_knob<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EcsTempSelectKnob {
                control: EcsTempSelectKnob::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_elec_fail<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ElecFail {
                control: ElecFail::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_elec_pwr_transf_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ElecPwrTransfSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ElecPwrTransfSw {
                control: ElecPwrTransfSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_emer_compass<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EmerCompassValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EmerCompass {
                control: EmerCompass::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_emer_hyd_pump_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EmerHydPumpSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EmerHydPumpSw {
                control: EmerHydPumpSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_emer_jett<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EmerJettValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EmerJett {
                control: EmerJett::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_emer_throttle_cov<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EmerThrottleCovValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EmerThrottleCov {
                control: EmerThrottleCov::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_emer_throttle_hand<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EmerThrottleHand {
                control: EmerThrottleHand::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_eng_emer_ctrl_cov<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EngEmerCtrlCovValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngEmerCtrlCov {
                control: EngEmerCtrlCov::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_eng_emer_ctrl_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EngEmerCtrlSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngEmerCtrlSw {
                control: EngEmerCtrlSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_eng_fuel_l_pump_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EngFuelLPumpSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngFuelLPumpSw {
                control: EngFuelLPumpSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_eng_fuel_r_pump_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EngFuelRPumpSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngFuelRPumpSw {
                control: EngFuelRPumpSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_eng_in_flgt_start_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EngInFlgtStartSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngInFlgtStartSw {
                control: EngInFlgtStartSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_eng_shutdown_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EngShutdownBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngShutdownBtn {
                control: EngShutdownBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_eng_start_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EngStartSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngStartSw {
                control: EngStartSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_eng_start_sw_cov<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EngStartSwCovValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngStartSwCov {
                control: EngStartSwCov::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ew_mode_sel_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EwModeSelSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EwModeSelSw {
                control: EwModeSelSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_formation_lights_aft<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtFormationLightsAft {
                control: ExtFormationLightsAft::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_formation_lights_front<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtFormationLightsFront {
                control: ExtFormationLightsFront::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
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
                control: ExtPositionLightLeft::new(),
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
                control: ExtPositionLightRight::new(),
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
                control: ExtSpeedBrakeLeft::new(),
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
                control: ExtSpeedBrakeRight::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_strobe_bottom<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtStrobeBottom {
                control: ExtStrobeBottom::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_strobe_top<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtStrobeTop {
                control: ExtStrobeTop::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fbw_ch5_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(FbwCh5SwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FbwCh5Sw {
                control: FbwCh5Sw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fbw_ch5_sw_cov<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(FbwCh5SwCovValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FbwCh5SwCov {
                control: FbwCh5SwCov::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fbw_g_lim_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(FbwGLimSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FbwGLimSw {
                control: FbwGLimSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fbw_gain_mode_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(FbwGainModeSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FbwGainModeSw {
                control: FbwGainModeSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fbw_gain_mode_sw_cov<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(FbwGainModeSwCovValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FbwGainModeSwCov {
                control: FbwGainModeSwCov::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fbw_reset_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(FbwResetBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FbwResetBtn {
                control: FbwResetBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fbw_spin_mode_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(FbwSpinModeSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FbwSpinModeSw {
                control: FbwSpinModeSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fbw_test_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(FbwTestSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FbwTestSw {
                control: FbwTestSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fbw_test_sw_cov<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(FbwTestSwCovValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FbwTestSwCov {
                control: FbwTestSwCov::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_feu_pc<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::FeuPc {
                control: FeuPc::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_feu_sec<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::FeuSec {
                control: FeuSec::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_firing_mode_sel<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(FiringModeSelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FiringModeSel {
                control: FiringModeSel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_form_lgt_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(FormLgtSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FormLgtSw {
                control: FormLgtSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_av_d<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelAvD {
                control: FuelAvD::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_av_g<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelAvG {
                control: FuelAvG::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_crof_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(FuelCrofSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelCrofSw {
                control: FuelCrofSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_cutoff_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(FuelCutoffSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelCutoffSw {
                control: FuelCutoffSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_cutoff_sw_cov<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(FuelCutoffSwCovValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelCutoffSwCov {
                control: FuelCutoffSwCov::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_detot_cents<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelDetotCents {
                control: FuelDetotCents::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_detot_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(FuelDetotSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelDetotSw {
                control: FuelDetotSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_detot_tens<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelDetotTens {
                control: FuelDetotTens::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_detot_thous<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelDetotThous {
                control: FuelDetotThous::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_dmp_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(FuelDmpSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelDmpSw {
                control: FuelDmpSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_dmp_sw_cov<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(FuelDmpSwCovValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelDmpSwCov {
                control: FuelDmpSwCov::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_flow<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelFlow {
                control: FuelFlow::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_jaug_cents<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelJaugCents {
                control: FuelJaugCents::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_jaug_d<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelJaugD {
                control: FuelJaugD::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_jaug_g<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelJaugG {
                control: FuelJaugG::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_jaug_tens<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelJaugTens {
                control: FuelJaugTens::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_jaug_thous<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelJaugThous {
                control: FuelJaugThous::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_rl_d<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelRlD {
                control: FuelRlD::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_rl_g<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelRlG {
                control: FuelRlG::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_rl_m<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelRlM {
                control: FuelRlM::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_transf_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(FuelTransfSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelTransfSw {
                control: FuelTransfSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_v_d<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelVD {
                control: FuelVD::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_v_g<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelVG {
                control: FuelVG::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gmeter_needle<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::GmeterNeedle {
                control: GmeterNeedle::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gun_arm_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(GunArmSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::GunArmSw {
                control: GunArmSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gun_mode_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(GunModeBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::GunModeBtn {
                control: GunModeBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gun_mode_kl1<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::GunModeKl1 {
                control: GunModeKl1::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gun_mode_kl2<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::GunModeKl2 {
                control: GunModeKl2::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hsi_compas<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HsiCompas {
                control: HsiCompas::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hsi_d_needle<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HsiDNeedle {
                control: HsiDNeedle::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hsi_dist_cents<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HsiDistCents {
                control: HsiDistCents::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hsi_dist_dec<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HsiDistDec {
                control: HsiDistDec::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hsi_dist_ones<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HsiDistOnes {
                control: HsiDistOnes::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hsi_dist_tens<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HsiDistTens {
                control: HsiDistTens::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hsi_flag_d<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HsiFlagD {
                control: HsiFlagD::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hsi_flag_dist<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HsiFlagDist {
                control: HsiFlagDist::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hsi_flag_g<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HsiFlagG {
                control: HsiFlagG::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hsi_flag_m<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HsiFlagM {
                control: HsiFlagM::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hsi_hdg<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HsiHdg {
                control: HsiHdg::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hsi_mode_sel_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(HsiModeSelSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HsiModeSelSw {
                control: HsiModeSelSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hsi_needle<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HsiNeedle {
                control: HsiNeedle::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hsi_vad_sel<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HsiVadSel {
                control: HsiVadSel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_clr_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(HudClrBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudClrBtn {
                control: HudClrBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_dec_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(HudDecSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudDecSw {
                control: HudDecSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_pwr_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(HudPwrSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudPwrSw {
                control: HudPwrSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_ralt_sel_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(HudRaltSelSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudRaltSelSw {
                control: HudRaltSelSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_rec<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudRec {
                control: HudRec::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hyd_d_needle<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HydDNeedle {
                control: HydDNeedle::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hyd_fail<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::HydFail {
                control: HydFail::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hyd_g_needle<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HydGNeedle {
                control: HydGNeedle::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hyd_sys_sel<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(HydSysSelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HydSysSel {
                control: HydSysSel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ident_pwr_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IdentPwrSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IdentPwrSw {
                control: IdentPwrSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_fault<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffFault {
                control: IffFault::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_ind<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffInd {
                control: IffInd::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_m4_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IffM4SwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffM4Sw {
                control: IffM4Sw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_red<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffRed {
                control: IffRed::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ins_aux_hd_hor<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(InsAuxHdHorValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::InsAuxHdHor {
                control: InsAuxHdHor::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ins_btn0<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(InsBtn0Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::InsBtn0 {
                control: InsBtn0::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ins_btn1<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(InsBtn1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::InsBtn1 {
                control: InsBtn1::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ins_btn2<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(InsBtn2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::InsBtn2 {
                control: InsBtn2::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ins_btn3<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(InsBtn3Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::InsBtn3 {
                control: InsBtn3::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ins_btn4<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(InsBtn4Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::InsBtn4 {
                control: InsBtn4::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ins_btn5<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(InsBtn5Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::InsBtn5 {
                control: InsBtn5::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ins_btn6<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(InsBtn6Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::InsBtn6 {
                control: InsBtn6::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ins_btn7<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(InsBtn7Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::InsBtn7 {
                control: InsBtn7::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ins_btn8<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(InsBtn8Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::InsBtn8 {
                control: InsBtn8::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ins_btn9<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(InsBtn9Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::InsBtn9 {
                control: InsBtn9::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ins_clr_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(InsClrBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::InsClrBtn {
                control: InsClrBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ins_dest_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(InsDestSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::InsDestSw {
                control: InsDestSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ins_dest_sw_light<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::InsDestSwLight {
                control: InsDestSwLight::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ins_eff_sw_light<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::InsEffSwLight {
                control: InsEffSwLight::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ins_enter_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(InsEnterBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::InsEnterBtn {
                control: InsEnterBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ins_ins_sw_light<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::InsInsSwLight {
                control: InsInsSwLight::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ins_mode_sel<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(InsModeSelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::InsModeSel {
                control: InsModeSel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ins_next_wp_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(InsNextWpBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::InsNextWpBtn {
                control: InsNextWpBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ins_opal_mode<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(InsOpalModeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::InsOpalMode {
                control: InsOpalMode::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ins_param_sel<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::InsParamSel {
                control: InsParamSel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ins_prep_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(InsPrepSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::InsPrepSw {
                control: InsPrepSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ins_prep_sw_light<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::InsPrepSwLight {
                control: InsPrepSwLight::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ins_prev_wp_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(InsPrevWpBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::InsPrevWpBtn {
                control: InsPrevWpBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ins_update<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(InsUpdateValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::InsUpdate {
                control: InsUpdate::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_intake_slats_op_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IntakeSlatsOpSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IntakeSlatsOpSw {
                control: IntakeSlatsOpSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_landing_gear_lever_light<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LandingGearLeverLight {
                control: LandingGearLeverLight::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ldg_emer_rel_lev<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(LdgEmerRelLevValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LdgEmerRelLev {
                control: LdgEmerRelLev::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ldg_lev<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(LdgLevValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LdgLev {
                control: LdgLev::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ldg_lgt_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(LdgLgtSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LdgLgtSw {
                control: LdgLgtSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lgt_bright_ctrl_test<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::LgtBrightCtrlTest {
                control: LgtBrightCtrlTest::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lgt_test_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(LgtTestSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LgtTestSw {
                control: LgtTestSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lim_ind<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LimInd {
                control: LimInd::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lox_dil_lev<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(LoxDilLevValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LoxDilLev {
                control: LoxDilLev::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_lox_emer_sup<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(LoxEmerSupValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LoxEmerSup {
                control: LoxEmerSup::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_magic_standby<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MagicStandbyValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MagicStandby {
                control: MagicStandby::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_magic_tone_vol_knob<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MagicToneVolKnob {
                control: MagicToneVolKnob::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_magneto_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MagnetoSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MagnetoSw {
                control: MagnetoSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_main_batt_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MainBattSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MainBattSw {
                control: MainBattSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_master_arm_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MasterArmSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MasterArmSw {
                control: MasterArmSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mc_ambre<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::McAmbre {
                control: McAmbre::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mc_rouge<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::McRouge {
                control: McRouge::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_min_alt_sel<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MinAltSel {
                control: MinAltSel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mirror_toggle<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MirrorToggleValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MirrorToggle {
                control: MirrorToggle::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mis_sel_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MisSelSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MisSelSw {
                control: MisSelSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mode110_sel<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Mode110SelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Mode110Sel {
                control: Mode110Sel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mode11_sel<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Mode11SelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Mode11Sel {
                control: Mode11Sel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mode1_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Mode1SwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Mode1Sw {
                control: Mode1Sw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mode2_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Mode2SwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Mode2Sw {
                control: Mode2Sw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mode3a100_sel<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Mode3a100SelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Mode3a100Sel {
                control: Mode3a100Sel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mode3a10_sel<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Mode3a10SelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Mode3a10Sel {
                control: Mode3a10Sel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mode3a1_sel<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Mode3a1SelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Mode3a1Sel {
                control: Mode3a1Sel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mode3a1k_sel<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Mode3a1kSelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Mode3a1kSel {
                control: Mode3a1kSel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mode3a_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Mode3aSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Mode3aSw {
                control: Mode3aSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mode_c_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ModeCSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ModeCSw {
                control: ModeCSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mrk_pos<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MrkPosValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MrkPos {
                control: MrkPos::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mrk_sig_vol_knob<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MrkSigVolKnob {
                control: MrkSigVolKnob::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_n_rpm<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::NRpm {
                control: NRpm::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nav_lgt_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(NavLgtSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::NavLgtSw {
                control: NavLgtSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nvg_helmet_mount<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(NvgHelmetMountValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::NvgHelmetMount {
                control: NvgHelmetMount::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nvg_light_filter_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(NvgLightFilterSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::NvgLightFilterSw {
                control: NvgLightFilterSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nvg_stow<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(NvgStowValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::NvgStow {
                control: NvgStow::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nws_iff_interr_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(NwsIffInterrBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::NwsIffInterrBtn {
                control: NwsIffInterrBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_offset_wp_tgt<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(OffsetWpTgtValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::OffsetWpTgt {
                control: OffsetWpTgt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oxy_flow_ind<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OxyFlowInd {
                control: OxyFlowInd::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oxy_needle<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OxyNeedle {
                control: OxyNeedle::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pc<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Pc {
                control: Pc::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pca_br1_disp<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcaBr1Disp {
                control: PcaBr1Disp::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pca_br2_disp<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcaBr2Disp {
                control: PcaBr2Disp::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pca_br3_disp<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcaBr3Disp {
                control: PcaBr3Disp::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pca_br4_disp<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcaBr4Disp {
                control: PcaBr4Disp::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pca_br5_disp<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcaBr5Disp {
                control: PcaBr5Disp::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pca_btn1<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(PcaBtn1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcaBtn1 {
                control: PcaBtn1::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pca_btn2<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(PcaBtn2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcaBtn2 {
                control: PcaBtn2::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pca_btn3<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(PcaBtn3Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcaBtn3 {
                control: PcaBtn3::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pca_btn4<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(PcaBtn4Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcaBtn4 {
                control: PcaBtn4::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pca_btn5<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(PcaBtn5Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcaBtn5 {
                control: PcaBtn5::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pca_button1_s<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcaButton1S {
                control: PcaButton1S::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pca_button2_s<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcaButton2S {
                control: PcaButton2S::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pca_button3_s<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcaButton3S {
                control: PcaButton3S::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pca_button4_s<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcaButton4S {
                control: PcaButton4S::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pca_button5_s<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcaButton5S {
                control: PcaButton5S::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pca_ur1_disp<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcaUr1Disp {
                control: PcaUr1Disp::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pca_ur2_disp<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcaUr2Disp {
                control: PcaUr2Disp::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pca_ur3_disp<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcaUr3Disp {
                control: PcaUr3Disp::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pca_ur4_disp<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcaUr4Disp {
                control: PcaUr4Disp::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pca_ur5_disp<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcaUr5Disp {
                control: PcaUr5Disp::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pcn_aln<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcnAln {
                control: PcnAln::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pcn_bad<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcnBad {
                control: PcnBad::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pcn_dis2dl<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcnDis2dl {
                control: PcnDis2dl::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pcn_dis2dr<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcnDis2dr {
                control: PcnDis2dr::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pcn_dis_dl<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcnDisDl {
                control: PcnDisDl::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pcn_dis_dr<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcnDisDr {
                control: PcnDisDr::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pcn_disp_dest<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcnDispDest {
                control: PcnDispDest::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pcn_disp_l<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcnDispL {
                control: PcnDispL::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pcn_disp_prep<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcnDispPrep {
                control: PcnDispPrep::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pcn_disp_r<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcnDispR {
                control: PcnDispR::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pcn_enc_sw_light<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcnEncSwLight {
                control: PcnEncSwLight::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pcn_m91_light<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcnM91Light {
                control: PcnM91Light::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pcn_m92_light<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcnM92Light {
                control: PcnM92Light::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pcn_m93_light<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcnM93Light {
                control: PcnM93Light::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pcn_mip<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcnMip {
                control: PcnMip::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pcn_mrq_sw_light<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcnMrqSwLight {
                control: PcnMrqSwLight::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pcn_ndeg<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcnNdeg {
                control: PcnNdeg::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pcn_pret<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcnPret {
                control: PcnPret::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pcn_rec<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcnRec {
                control: PcnRec::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pcn_sec<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcnSec {
                control: PcnSec::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pcn_uni<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcnUni {
                control: PcnUni::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pcn_val_sw_light<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::PcnValSwLight {
                control: PcnValSwLight::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pedal_adjust_lev<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(PedalAdjustLevValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::PedalAdjustLev {
                control: PedalAdjustLev::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pitot_heat_cov<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(PitotHeatCovValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::PitotHeatCov {
                control: PitotHeatCov::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pitot_heat_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(PitotHeatSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::PitotHeatSw {
                control: PitotHeatSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pkg_brake_lev<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(PkgBrakeLevValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::PkgBrakeLev {
                control: PkgBrakeLev::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_police_lgt_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(PoliceLgtSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::PoliceLgtSw {
                control: PoliceLgtSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ppa_auto<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::PpaAuto {
                control: PpaAuto::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ppa_gun_rocket_pap<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::PpaGunRocketPap {
                control: PpaGunRocketPap::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ppa_gun_rocket_tot<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::PpaGunRocketTot {
                control: PpaGunRocketTot::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ppa_int_disp<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::PpaIntDisp {
                control: PpaIntDisp::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ppa_magic_mis<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::PpaMagicMis {
                control: PpaMagicMis::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ppa_magic_p<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::PpaMagicP {
                control: PpaMagicP::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ppa_man<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::PpaMan {
                control: PpaMan::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ppa_qty_disp<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::PpaQtyDisp {
                control: PpaQtyDisp::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ppa_s530_mis<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::PpaS530Mis {
                control: PpaS530Mis::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ppa_s530_p<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::PpaS530P {
                control: PpaS530P::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ppa_test_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(PpaTestSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::PpaTestSw {
                control: PpaTestSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_press_needle<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::PressNeedle {
                control: PressNeedle::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_psv_derive<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::PsvDerive {
                control: PsvDerive::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_psv_el_d_ext<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::PsvElDExt {
                control: PsvElDExt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_psv_el_d_int<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::PsvElDInt {
                control: PsvElDInt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_psv_el_g_ext<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::PsvElGExt {
                control: PsvElGExt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_psv_el_g_int<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::PsvElGInt {
                control: PsvElGInt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_qra_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(QraSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::QraSw {
                control: QraSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rad_a<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadA {
                control: RadA::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rad_az_sel<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RadAzSelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadAzSel {
                control: RadAzSel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rad_contrast_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadContrastSw {
                control: RadContrastSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rad_dec<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadDec {
                control: RadDec::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rad_dop_rej_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RadDopRejSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadDopRejSw {
                control: RadDopRejSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rad_gain_dial<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadGainDial {
                control: RadGainDial::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rad_grid_sel_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RadGridSelSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadGridSelSw {
                control: RadGridSelSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rad_iff_code1_sel<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RadIffCode1SelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadIffCode1Sel {
                control: RadIffCode1Sel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rad_iff_code2_sel<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RadIffCode2SelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadIffCode2Sel {
                control: RadIffCode2Sel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rad_iff_code3_sel<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RadIffCode3SelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadIffCode3Sel {
                control: RadIffCode3Sel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rad_iff_code4_sel<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RadIffCode4SelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadIffCode4Sel {
                control: RadIffCode4Sel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rad_iff_l_r_sel<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RadIffLRSelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadIffLRSel {
                control: RadIffLRSel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rad_iff_mode_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RadIffModeSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadIffModeSw {
                control: RadIffModeSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rad_iff_pow_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RadIffPowSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadIffPowSw {
                control: RadIffPowSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rad_illum_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RadIllumSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadIllumSw {
                control: RadIllumSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rad_prf_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RadPrfSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadPrfSw {
                control: RadPrfSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rad_psic<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadPsic {
                control: RadPsic::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rad_pwr_sel<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RadPwrSelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadPwrSel {
                control: RadPwrSel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rad_ralt_pwr_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RadRaltPwrSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadRaltPwrSw {
                control: RadRaltPwrSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rad_rearm_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RadRearmBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadRearmBtn {
                control: RadRearmBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rad_rng_sel_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RadRngSelSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadRngSelSw {
                control: RadRngSelSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rad_scan_lines_sel<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RadScanLinesSelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadScanLinesSel {
                control: RadScanLinesSel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rad_test_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RadTestBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadTestBtn {
                control: RadTestBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rad_val<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadVal {
                control: RadVal::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rad_visu<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadVisu {
                control: RadVisu::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rad_wow_em_aut_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RadWowEmAutSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadWowEmAutSw {
                control: RadWowEmAutSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_red_flood_lgt_knob<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RedFloodLgtKnob {
                control: RedFloodLgtKnob::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_refuel_lgt_bright_knob<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RefuelLgtBrightKnob {
                control: RefuelLgtBrightKnob::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rpm_ones<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RpmOnes {
                control: RpmOnes::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rpm_tens<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RpmTens {
                control: RpmTens::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_br<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrBr {
                control: RwrBr::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_d2m<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrD2m {
                control: RwrD2m::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_da<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrDa {
                control: RwrDa::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_lgt_bright_ctrl<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrLgtBrightCtrl {
                control: RwrLgtBrightCtrl::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_ll<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrLl {
                control: RwrLl::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_pwr_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RwrPwrSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrPwrSw {
                control: RwrPwrSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_v<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrV {
                control: RwrV::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_s530_standby<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(S530StandbyValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::S530Standby {
                control: S530Standby::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_safe_gnd_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(SafeGndSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SafeGndSw {
                control: SafeGndSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sb_adi_cage_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(SbAdiCageSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SbAdiCageSw {
                control: SbAdiCageSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sb_adi_flag<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SbAdiFlag {
                control: SbAdiFlag::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sb_adi_pitch<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SbAdiPitch {
                control: SbAdiPitch::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sb_adi_roll<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SbAdiRoll {
                control: SbAdiRoll::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sb_adi_rot<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SbAdiRot {
                control: SbAdiRot::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_seat_adjust_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(SeatAdjustSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SeatAdjustSw {
                control: SeatAdjustSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sec_huile_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(SecHuileSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SecHuileSw {
                control: SecHuileSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sec_huile_sw_cover<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(SecHuileSwCoverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SecHuileSwCover {
                control: SecHuileSwCover::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sel_alt_hold_btn<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(SelAltHoldBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SelAltHoldBtn {
                control: SelAltHoldBtn::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sel_jett_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(SelJettSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SelJettSw {
                control: SelJettSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sel_jett_sw_cov<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(SelJettSwCovValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SelJettSwCov {
                control: SelJettSwCov::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_serpam_rec_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(SerpamRecSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SerpamRecSw {
                control: SerpamRecSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_slats_op_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(SlatsOpSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SlatsOpSw {
                control: SlatsOpSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_speed_kts_need<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SpeedKtsNeed {
                control: SpeedKtsNeed::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_speed_mach<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SpeedMach {
                control: SpeedMach::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_start_fuel_pump_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(StartFuelPumpSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::StartFuelPumpSw {
                control: StartFuelPumpSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_t7_needle<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::T7Needle {
                control: T7Needle::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tac_ch10_sel<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TacCh10Sel {
                control: TacCh10Sel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tac_ch1_sel<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(TacCh1SelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::TacCh1Sel {
                control: TacCh1Sel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tac_mode_sel<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(TacModeSelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::TacModeSel {
                control: TacModeSel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tac_vol_knob<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TacVolKnob {
                control: TacVolKnob::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tac_x_y_sel<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(TacXYSelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::TacXYSel {
                control: TacXYSel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tb_app_vol_knob<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TbAppVolKnob {
                control: TbAppVolKnob::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tdc_mode_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(TdcModeSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::TdcModeSw {
                control: TdcModeSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_test_rouge<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::TestRouge {
                control: TestRouge::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_test_vert<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::TestVert {
                control: TestVert::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tgt_age<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(TgtAgeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::TgtAge {
                control: TgtAge::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tgt_alt<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(TgtAltValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::TgtAlt {
                control: TgtAlt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tgt_bear_be<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(TgtBearBeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::TgtBearBe {
                control: TgtBearBe::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tgt_data_mnl_entry<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(TgtDataMnlEntryValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::TgtDataMnlEntry {
                control: TgtDataMnlEntry::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tgt_hd<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(TgtHdValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::TgtHd {
                control: TgtHd::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tgt_mach_num<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(TgtMachNumValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::TgtMachNum {
                control: TgtMachNum::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tgt_mem_time_sel_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(TgtMemTimeSelSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::TgtMemTimeSelSw {
                control: TgtMemTimeSelSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tgt_rng_be<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(TgtRngBeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::TgtRngBe {
                control: TgtRngBe::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tgt_wingspan_knob<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TgtWingspanKnob {
                control: TgtWingspanKnob::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_cde_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(UhfCdeSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfCdeSw {
                control: UhfCdeSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_e_a2_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(UhfEA2SwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfEA2Sw {
                control: UhfEA2Sw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_frequency<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfFrequency {
                control: UhfFrequency::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_mode_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(UhfModeSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfModeSw {
                control: UhfModeSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_preset<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfPreset {
                control: UhfPreset::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_preset_knob_int<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfPresetKnobInt {
                control: UhfPresetKnobInt::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_preset_knob_str<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfPresetKnobStr {
                control: UhfPresetKnobStr::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_pwr5w25w_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(UhfPwr5w25wSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfPwr5w25wSw {
                control: UhfPwr5w25wSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_radio_vol_knob<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfRadioVolKnob {
                control: UhfRadioVolKnob::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_rep_l1<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfRepL1 {
                control: UhfRepL1::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_rep_l2<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfRepL2 {
                control: UhfRepL2::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_secure_l<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfSecureL {
                control: UhfSecureL::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_sil_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(UhfSilSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfSilSw {
                control: UhfSilSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_test_l<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfTestL {
                control: UhfTestL::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_test_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(UhfTestSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfTestSw {
                control: UhfTestSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_val_data_entry<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ValDataEntryValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ValDataEntry {
                control: ValDataEntry::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vario_needle<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VarioNeedle {
                control: VarioNeedle::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vent_sel_sw<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(VentSelSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::VentSelSw {
                control: VentSelSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf0<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Vhf0Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::Vhf0 {
                control: Vhf0::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf1_read<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Vhf1ReadValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Vhf1Read {
                control: Vhf1Read::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf2_sql<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Vhf2SqlValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Vhf2Sql {
                control: Vhf2Sql::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf3_gr<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Vhf3GrValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Vhf3Gr {
                control: Vhf3Gr::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf4<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Vhf4Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::Vhf4 {
                control: Vhf4::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf520_low<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Vhf520LowValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Vhf520Low {
                control: Vhf520Low::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf6_tone<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Vhf6ToneValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Vhf6Tone {
                control: Vhf6Tone::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf7<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Vhf7Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::Vhf7 {
                control: Vhf7::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf8_tod<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Vhf8TodValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Vhf8Tod {
                control: Vhf8Tod::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf9_zero<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Vhf9ZeroValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Vhf9Zero {
                control: Vhf9Zero::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf_ch_sel<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfChSel {
                control: VhfChSel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf_conf<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(VhfConfValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfConf {
                control: VhfConf::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf_frequency<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfFrequency {
                control: VhfFrequency::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf_mem_clr<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(VhfMemClrValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfMemClr {
                control: VhfMemClr::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf_mode<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(VhfModeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfMode {
                control: VhfMode::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhf_vld_xfr<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(VhfVldXfrValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfVldXfr {
                control: VhfVldXfr::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vorils001_drum<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Vorils001Drum {
                control: Vorils001Drum::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vorils01_drum<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Vorils01Drum {
                control: Vorils01Drum::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vorils100_drum<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Vorils100Drum {
                control: Vorils100Drum::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vorils10_drum<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Vorils10Drum {
                control: Vorils10Drum::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vorils1_drum<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Vorils1Drum {
                control: Vorils1Drum::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vorils_freq_decimal<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VorilsFreqDecimal {
                control: VorilsFreqDecimal::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vorils_freq_whole<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VorilsFreqWhole {
                control: VorilsFreqWhole::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vorils_pwr_dial<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(VorilsPwrDialValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::VorilsPwrDial {
                control: VorilsPwrDial::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vorils_test_dial<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(VorilsTestDialValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::VorilsTestDial {
                control: VorilsTestDial::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vorils_vol_knob<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VorilsVolKnob {
                control: VorilsVolKnob::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vtac10_drum<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Vtac10Drum {
                control: Vtac10Drum::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vtac1_drum<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Vtac1Drum {
                control: Vtac1Drum::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vtac_x_y_drum<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VtacXYDrum {
                control: VtacXYDrum::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vtb_dec<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(VtbDecValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::VtbDec {
                control: VtbDec::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vtb_disp_bright<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(VtbDispBrightValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::VtbDispBright {
                control: VtbDispBright::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vtb_disp_contrast<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(VtbDispContrastValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::VtbDispContrast {
                control: VtbDispContrast::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vtb_icons_bright<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(VtbIconsBrightValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::VtbIconsBright {
                control: VtbIconsBright::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vtb_or_sel<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(VtbOrSelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::VtbOrSel {
                control: VtbOrSel::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vtb_pwr_sw<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(VtbPwrSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::VtbPwrSw {
                control: VtbPwrSw::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vtb_video_bright<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(VtbVideoBrightValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::VtbVideoBright {
                control: VtbVideoBright::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vuhf_radio_vol_knob<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VuhfRadioVolKnob {
                control: VuhfRadioVolKnob::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_weapon1_r<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Weapon1R {
                control: Weapon1R::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_weapon1_s<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Weapon1S {
                control: Weapon1S::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_weapon2_r<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Weapon2R {
                control: Weapon2R::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_weapon2_s<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Weapon2S {
                control: Weapon2S::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_weapon3_r<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Weapon3R {
                control: Weapon3R::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_weapon3_s<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Weapon3S {
                control: Weapon3S::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_weapon4_r<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Weapon4R {
                control: Weapon4R::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_weapon4_s<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Weapon4S {
                control: Weapon4S::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_weapon5_r<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Weapon5R {
                control: Weapon5R::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_weapon5_s<F>(&mut self, callback: &'a mut F) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::Weapon5S {
                control: Weapon5S::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_wh_flood_lgt_knob<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::WhFloodLgtKnob {
                control: WhFloodLgtKnob::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_wps_sel_btn1<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(WpsSelBtn1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::WpsSelBtn1 {
                control: WpsSelBtn1::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_wps_sel_btn2<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(WpsSelBtn2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::WpsSelBtn2 {
                control: WpsSelBtn2::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_wps_sel_btn3<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(WpsSelBtn3Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::WpsSelBtn3 {
                control: WpsSelBtn3::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_wps_sel_btn4<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(WpsSelBtn4Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::WpsSelBtn4 {
                control: WpsSelBtn4::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_wps_sel_btn5<F>(
        &mut self,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(WpsSelBtn5Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::WpsSelBtn5 {
                control: WpsSelBtn5::new(),
                callback,
            })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn build(self) -> M2000c<'a, MAX_CONTROLS, MAX_PACKET_DATA> {
        M2000c {
            parser: ProtocolParser::new(),
            controls: self.controls,
            saw_frame: false,
        }
    }
}

impl<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> Default
    for M2000cBuilder<'a, MAX_CONTROLS, MAX_PACKET_DATA>
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

pub struct M2000c<'a, const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize> {
    parser: ProtocolParser<MAX_PACKET_DATA>,
    controls: Vec<RegisteredControl<'a>, MAX_CONTROLS>,
    saw_frame: bool,
}

impl<const MAX_CONTROLS: usize, const MAX_PACKET_DATA: usize>
    M2000c<'_, MAX_CONTROLS, MAX_PACKET_DATA>
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
