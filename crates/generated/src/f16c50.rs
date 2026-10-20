//! Generated from model-definitions/F-16C_50.json.
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

/// AB RESET Switch, AB RESET/NORM/ENG DATA
/// Input: set the switch position
/// Output: switch position -- 0 = Down, 1 = Mid ,  2 = UP
pub struct AbResetSw(Selector);
impl AbResetSw {
    pub const fn new() -> Self {
        Self(Selector::new(17440, 6144, 11))
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

/// ADI AUX Flag
/// Output: gauge position
pub struct AdiAuxFlag(AnalogGauge);
impl AdiAuxFlag {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17592, 65535, 0))
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
pub struct AdiBank(AnalogGauge);
impl AdiBank {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17586, 65535, 0))
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
pub struct AdiBubble(AnalogGauge);
impl AdiBubble {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17604, 65535, 0))
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
pub struct AdiGsBar(AnalogGauge);
impl AdiGsBar {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17598, 65535, 0))
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
pub struct AdiGsFlag(AnalogGauge);
impl AdiGsFlag {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17594, 65535, 0))
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
pub struct AdiGsPointer(AnalogGauge);
impl AdiGsPointer {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17602, 65535, 0))
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
pub struct AdiLocBar(AnalogGauge);
impl AdiLocBar {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17596, 65535, 0))
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
pub struct AdiLocFlag(AnalogGauge);
impl AdiLocFlag {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17590, 65535, 0))
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
pub struct AdiOffFlag(AnalogGauge);
impl AdiOffFlag {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17588, 65535, 0))
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
pub struct AdiPitch(AnalogGauge);
impl AdiPitch {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17584, 65535, 0))
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
pub struct AdiPitchTrim(LimitedDial);
impl AdiPitchTrim {
    pub const fn new() -> Self {
        Self(LimitedDial::new(17514, 65535, 0))
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
pub struct AdiTurnrate(AnalogGauge);
impl AdiTurnrate {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17600, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdvModeSwValue {
    Select1,
    Select2,
}
pub type AdvModeSwValueCallback<'a> = &'a mut dyn FnMut(AdvModeSwValue);
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
pub struct AdvModeSw(Selector);
impl AdvModeSw {
    pub const fn new() -> Self {
        Self(Selector::new(17408, 4096, 12))
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

/// AERIAL REFUELING Light Knob
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct AirRefuelLightKnb(LimitedDial);
impl AirRefuelLightKnb {
    pub const fn new() -> Self {
        Self(LimitedDial::new(17420, 65535, 0))
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
pub enum AirRefuelSwValue {
    Select1,
    Select2,
}
pub type AirRefuelSwValueCallback<'a> = &'a mut dyn FnMut(AirRefuelSwValue);
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
pub struct AirRefuelSw(Selector);
impl AirRefuelSw {
    pub const fn new() -> Self {
        Self(Selector::new(17436, 4096, 12))
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
pub enum AirSourceKnbValue {
    Select1,
    Select2,
    Select3,
    Select4,
    Select5,
}
pub type AirSourceKnbValueCallback<'a> = &'a mut dyn FnMut(AirSourceKnbValue);
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
pub struct AirSourceKnb(Selector);
impl AirSourceKnb {
    pub const fn new() -> Self {
        Self(Selector::new(17438, 28672, 12))
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

/// Airspeed Indicator
/// Output: gauge position
pub struct Airspeed(AnalogGauge);
impl Airspeed {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17564, 65535, 0))
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
pub struct AirspeedSetKnb(AnalogDial);
impl AirspeedSetKnb {
    pub const fn new() -> Self {
        Self(AnalogDial::new(17508, 65535, 0))
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

/// Altimeter 10000 ft count
/// Output: gauge position
pub struct Alt10000FtCnt(AnalogGauge);
impl Alt10000FtCnt {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17548, 65535, 0))
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
pub struct Alt1000FtCnt(AnalogGauge);
impl Alt1000FtCnt {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17550, 65535, 0))
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
pub struct Alt100FtCnt(AnalogGauge);
impl Alt100FtCnt {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17552, 65535, 0))
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
pub struct Alt100FtPtr(AnalogGauge);
impl Alt100FtPtr {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17546, 65535, 0))
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
pub struct AltBaroSetKnb(AnalogDial);
impl AltBaroSetKnb {
    pub const fn new() -> Self {
        Self(AnalogDial::new(17510, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AltFlapsSwValue {
    Select1,
    Select2,
}
pub type AltFlapsSwValueCallback<'a> = &'a mut dyn FnMut(AltFlapsSwValue);
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
pub struct AltFlapsSw(Selector);
impl AltFlapsSw {
    pub const fn new() -> Self {
        Self(Selector::new(17408, 2, 1))
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

/// Altimeter Mode Lever, ELEC/OFF/PNEU
/// Input: set the switch position
/// Output: switch position -- 0 = Down, 1 = Mid ,  2 = UP
pub struct AltModeLv(Selector);
impl AltModeLv {
    pub const fn new() -> Self {
        Self(Selector::new(17506, 768, 8))
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
pub struct AltPneuFlag(AnalogGauge);
impl AltPneuFlag {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17562, 65535, 0))
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
pub struct AltPressureDrum0Cnt(AnalogGauge);
impl AltPressureDrum0Cnt {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17554, 65535, 0))
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
pub struct AltPressureDrum1Cnt(AnalogGauge);
impl AltPressureDrum1Cnt {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17556, 65535, 0))
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
pub struct AltPressureDrum2Cnt(AnalogGauge);
impl AltPressureDrum2Cnt {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17558, 65535, 0))
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
pub struct AltPressureDrum3Cnt(AnalogGauge);
impl AltPressureDrum3Cnt {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17560, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AltRelBtnValue {
    Select1,
    Select2,
}
pub type AltRelBtnValueCallback<'a> = &'a mut dyn FnMut(AltRelBtnValue);
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
pub struct AltRelBtn(Selector);
impl AltRelBtn {
    pub const fn new() -> Self {
        Self(Selector::new(17444, 512, 9))
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
pub type AntiCollLightKnbValueCallback<'a> = &'a mut dyn FnMut(AntiCollLightKnbValue);
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
pub struct AntiCollLightKnb(Selector);
impl AntiCollLightKnb {
    pub const fn new() -> Self {
        Self(Selector::new(17416, 7, 0))
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

/// ANTI-SKID Switch, PARKING BRAKE/ANTI-SKID/OFF
/// Input: set the switch position
/// Output: switch position -- 0 = Down, 1 = Mid ,  2 = UP
pub struct AntiSkidSw(Selector);
impl AntiSkidSw {
    pub const fn new() -> Self {
        Self(Selector::new(17438, 384, 7))
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

/// AOA Indexer Dimming Lever
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct AoaIndexBrtKnb(LimitedDial);
impl AoaIndexBrtKnb {
    pub const fn new() -> Self {
        Self(LimitedDial::new(17432, 65535, 0))
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

/// AoA Value
/// Output: gauge position
pub struct AoaValue(AnalogGauge);
impl AoaValue {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17544, 65535, 0))
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

/// Autopilot PITCH Switch, ATT HOLD/ A/P OFF/ ALT HOLD
/// Input: set the switch position
/// Output: switch position -- 0 = Down, 1 = Mid ,  2 = UP
pub struct ApPitchSw(Selector);
impl ApPitchSw {
    pub const fn new() -> Self {
        Self(Selector::new(17408, 768, 8))
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
pub type ApRollSwValueCallback<'a> = &'a mut dyn FnMut(ApRollSwValue);
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
pub struct ApRollSw(Selector);
impl ApRollSw {
    pub const fn new() -> Self {
        Self(Selector::new(17408, 3072, 10))
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

/// AR Status Indicator Dimming Lever
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct ArStatusBrtKnb(LimitedDial);
impl ArStatusBrtKnb {
    pub const fn new() -> Self {
        Self(LimitedDial::new(17434, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BitSwValue {
    Select1,
    Select2,
}
pub type BitSwValueCallback<'a> = &'a mut dyn FnMut(BitSwValue);
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
pub struct BitSw(Selector);
impl BitSw {
    pub const fn new() -> Self {
        Self(Selector::new(17408, 4, 2))
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
pub enum BrakeChanSwValue {
    Select1,
    Select2,
}
pub type BrakeChanSwValueCallback<'a> = &'a mut dyn FnMut(BrakeChanSwValue);
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
pub struct BrakeChanSw(Selector);
impl BrakeChanSw {
    pub const fn new() -> Self {
        Self(Selector::new(17438, 64, 6))
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
pub enum CanopyHandleValue {
    Select1,
    Select2,
}
pub type CanopyHandleValueCallback<'a> = &'a mut dyn FnMut(CanopyHandleValue);
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
pub struct CanopyHandle(Selector);
impl CanopyHandle {
    pub const fn new() -> Self {
        Self(Selector::new(17506, 32768, 15))
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
pub type CanopyJettThandleValueCallback<'a> = &'a mut dyn FnMut(CanopyJettThandleValue);
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
pub struct CanopyJettThandle(Selector);
impl CanopyJettThandle {
    pub const fn new() -> Self {
        Self(Selector::new(17522, 4, 2))
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
pub struct CanopySw(Selector);
impl CanopySw {
    pub const fn new() -> Self {
        Self(Selector::new(17522, 48, 4))
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
pub struct CanopyValue(AnalogGauge);
impl CanopyValue {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17540, 65535, 0))
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

/// Current Time Hours
/// Output: gauge position
pub struct ClockCurrtimeH(AnalogGauge);
impl ClockCurrtimeH {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17656, 65535, 0))
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
pub struct ClockCurrtimeMs(AnalogGauge);
impl ClockCurrtimeMs {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17658, 65535, 0))
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
pub type ClockElapsedValueCallback<'a> = &'a mut dyn FnMut(ClockElapsedValue);
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
pub struct ClockElapsed(Selector);
impl ClockElapsed {
    pub const fn new() -> Self {
        Self(Selector::new(17506, 16384, 14))
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
pub struct ClockElapsedTimeM(AnalogGauge);
impl ClockElapsedTimeM {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17660, 65535, 0))
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
pub struct ClockElapsedTimeSec(AnalogGauge);
impl ClockElapsedTimeSec {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17662, 65535, 0))
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
pub type ClockSetValueCallback<'a> = &'a mut dyn FnMut(ClockSetValue);
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
pub struct ClockSet(Selector);
impl ClockSet {
    pub const fn new() -> Self {
        Self(Selector::new(17506, 8192, 13))
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
pub struct ClockWind(AnalogDial);
impl ClockWind {
    pub const fn new() -> Self {
        Self(AnalogDial::new(17520, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cmds01ExpCatSwValue {
    Select1,
    Select2,
}
pub type Cmds01ExpCatSwValueCallback<'a> = &'a mut dyn FnMut(Cmds01ExpCatSwValue);
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
pub struct Cmds01ExpCatSw(Selector);
impl Cmds01ExpCatSw {
    pub const fn new() -> Self {
        Self(Selector::new(17498, 32, 5))
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
pub type Cmds02ExpCatSwValueCallback<'a> = &'a mut dyn FnMut(Cmds02ExpCatSwValue);
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
pub struct Cmds02ExpCatSw(Selector);
impl Cmds02ExpCatSw {
    pub const fn new() -> Self {
        Self(Selector::new(17498, 64, 6))
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
pub struct CmdsChAmount(Display<4>);
impl CmdsChAmount {
    pub const fn new() -> Self {
        Self(Display::<4>::new(17822, 4))
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
pub type CmdsChExpCatSwValueCallback<'a> = &'a mut dyn FnMut(CmdsChExpCatSwValue);
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
pub struct CmdsChExpCatSw(Selector);
impl CmdsChExpCatSw {
    pub const fn new() -> Self {
        Self(Selector::new(17498, 128, 7))
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
pub type CmdsDispenseBtnValueCallback<'a> = &'a mut dyn FnMut(CmdsDispenseBtnValue);
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
pub struct CmdsDispenseBtn(Selector);
impl CmdsDispenseBtn {
    pub const fn new() -> Self {
        Self(Selector::new(17498, 1, 0))
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
pub struct CmdsFlAmount(Display<4>);
impl CmdsFlAmount {
    pub const fn new() -> Self {
        Self(Display::<4>::new(17826, 4))
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
pub type CmdsFlExpCatSwValueCallback<'a> = &'a mut dyn FnMut(CmdsFlExpCatSwValue);
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
pub struct CmdsFlExpCatSw(Selector);
impl CmdsFlExpCatSw {
    pub const fn new() -> Self {
        Self(Selector::new(17498, 256, 8))
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
pub type CmdsJettSwValueCallback<'a> = &'a mut dyn FnMut(CmdsJettSwValue);
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
pub struct CmdsJettSw(Selector);
impl CmdsJettSw {
    pub const fn new() -> Self {
        Self(Selector::new(17498, 16, 4))
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
pub type CmdsJmrSourcheSwValueCallback<'a> = &'a mut dyn FnMut(CmdsJmrSourcheSwValue);
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
pub struct CmdsJmrSourcheSw(Selector);
impl CmdsJmrSourcheSw {
    pub const fn new() -> Self {
        Self(Selector::new(17498, 4, 2))
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
pub type CmdsModeKnbValueCallback<'a> = &'a mut dyn FnMut(CmdsModeKnbValue);
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
pub struct CmdsModeKnb(Selector);
impl CmdsModeKnb {
    pub const fn new() -> Self {
        Self(Selector::new(17498, 28672, 12))
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
pub type CmdsMwsSourcheSwValueCallback<'a> = &'a mut dyn FnMut(CmdsMwsSourcheSwValue);
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
pub struct CmdsMwsSourcheSw(Selector);
impl CmdsMwsSourcheSw {
    pub const fn new() -> Self {
        Self(Selector::new(17498, 8, 3))
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
pub struct CmdsO1Amount(Display<4>);
impl CmdsO1Amount {
    pub const fn new() -> Self {
        Self(Display::<4>::new(17814, 4))
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
pub struct CmdsO2Amount(Display<4>);
impl CmdsO2Amount {
    pub const fn new() -> Self {
        Self(Display::<4>::new(17818, 4))
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
pub type CmdsProgKnbValueCallback<'a> = &'a mut dyn FnMut(CmdsProgKnbValue);
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
pub struct CmdsProgKnb(Selector);
impl CmdsProgKnb {
    pub const fn new() -> Self {
        Self(Selector::new(17498, 3584, 9))
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
pub type CmdsPwrSourcheSwValueCallback<'a> = &'a mut dyn FnMut(CmdsPwrSourcheSwValue);
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
pub struct CmdsPwrSourcheSw(Selector);
impl CmdsPwrSourcheSw {
    pub const fn new() -> Self {
        Self(Selector::new(17498, 2, 1))
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

/// Cabin Pressure Altimeter
/// Output: gauge position
pub struct CockpitAlititude(AnalogGauge);
impl CockpitAlititude {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17642, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Comm1ModeKnbValue {
    Select1,
    Select2,
    Select3,
}
pub type Comm1ModeKnbValueCallback<'a> = &'a mut dyn FnMut(Comm1ModeKnbValue);
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
pub struct Comm1ModeKnb(Selector);
impl Comm1ModeKnb {
    pub const fn new() -> Self {
        Self(Selector::new(17458, 49152, 14))
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
pub struct Comm1PwrKnb(LimitedDial);
impl Comm1PwrKnb {
    pub const fn new() -> Self {
        Self(LimitedDial::new(17462, 65535, 0))
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
pub type Comm2ModeKnbValueCallback<'a> = &'a mut dyn FnMut(Comm2ModeKnbValue);
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
pub struct Comm2ModeKnb(Selector);
impl Comm2ModeKnb {
    pub const fn new() -> Self {
        Self(Selector::new(17460, 3, 0))
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
pub struct Comm2PwrKnb(LimitedDial);
impl Comm2PwrKnb {
    pub const fn new() -> Self {
        Self(LimitedDial::new(17464, 65535, 0))
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

/// DED Display Line 1
/// Output: DED Display Line 1
pub struct DedLine1(Display<29>);
impl DedLine1 {
    pub const fn new() -> Self {
        Self(Display::<29>::new(17664, 29))
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
pub struct DedLine2(Display<29>);
impl DedLine2 {
    pub const fn new() -> Self {
        Self(Display::<29>::new(17694, 29))
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
pub struct DedLine3(Display<29>);
impl DedLine3 {
    pub const fn new() -> Self {
        Self(Display::<29>::new(17724, 29))
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
pub struct DedLine4(Display<29>);
impl DedLine4 {
    pub const fn new() -> Self {
        Self(Display::<29>::new(17754, 29))
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
pub struct DedLine5(Display<29>);
impl DedLine5 {
    pub const fn new() -> Self {
        Self(Display::<29>::new(17784, 29))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DefogLeverValue {
    Select1,
    Select2,
}
pub type DefogLeverValueCallback<'a> = &'a mut dyn FnMut(DefogLeverValue);
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
pub struct DefogLever(Selector);
impl DefogLever {
    pub const fn new() -> Self {
        Self(Selector::new(17438, 32768, 15))
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
pub enum DigiBakSwValue {
    Select1,
    Select2,
}
pub type DigiBakSwValueCallback<'a> = &'a mut dyn FnMut(DigiBakSwValue);
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
pub struct DigiBakSw(Selector);
impl DigiBakSw {
    pub const fn new() -> Self {
        Self(Selector::new(17408, 1, 0))
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
pub enum DlSwValue {
    Select1,
    Select2,
}
pub type DlSwValueCallback<'a> = &'a mut dyn FnMut(DlSwValue);
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
pub struct DlSw(Selector);
impl DlSw {
    pub const fn new() -> Self {
        Self(Selector::new(17444, 16, 4))
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
pub enum DnLockBtnValue {
    Select1,
    Select2,
}
pub type DnLockBtnValueCallback<'a> = &'a mut dyn FnMut(DnLockBtnValue);
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
pub struct DnLockBtn(Selector);
impl DnLockBtn {
    pub const fn new() -> Self {
        Self(Selector::new(17438, 8, 3))
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
pub enum Ecm1BtnValue {
    Select1,
    Select2,
}
pub type Ecm1BtnValueCallback<'a> = &'a mut dyn FnMut(Ecm1BtnValue);
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
pub struct Ecm1Btn(Selector);
impl Ecm1Btn {
    pub const fn new() -> Self {
        Self(Selector::new(17692, 512, 9))
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
pub type Ecm2BtnValueCallback<'a> = &'a mut dyn FnMut(Ecm2BtnValue);
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
pub struct Ecm2Btn(Selector);
impl Ecm2Btn {
    pub const fn new() -> Self {
        Self(Selector::new(17692, 1024, 10))
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
pub type Ecm3BtnValueCallback<'a> = &'a mut dyn FnMut(Ecm3BtnValue);
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
pub struct Ecm3Btn(Selector);
impl Ecm3Btn {
    pub const fn new() -> Self {
        Self(Selector::new(17692, 2048, 11))
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
pub type Ecm4BtnValueCallback<'a> = &'a mut dyn FnMut(Ecm4BtnValue);
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
pub struct Ecm4Btn(Selector);
impl Ecm4Btn {
    pub const fn new() -> Self {
        Self(Selector::new(17692, 4096, 12))
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
pub type Ecm5BtnValueCallback<'a> = &'a mut dyn FnMut(Ecm5BtnValue);
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
pub struct Ecm5Btn(Selector);
impl Ecm5Btn {
    pub const fn new() -> Self {
        Self(Selector::new(17692, 8192, 13))
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
pub type Ecm6BtnValueCallback<'a> = &'a mut dyn FnMut(Ecm6BtnValue);
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
pub struct Ecm6Btn(Selector);
impl Ecm6Btn {
    pub const fn new() -> Self {
        Self(Selector::new(17692, 16384, 14))
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
pub type EcmBitBtnValueCallback<'a> = &'a mut dyn FnMut(EcmBitBtnValue);
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
pub struct EcmBitBtn(Selector);
impl EcmBitBtn {
    pub const fn new() -> Self {
        Self(Selector::new(17692, 256, 8))
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

/// ECM DIM Rotary Knob
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct EcmDimKnb(LimitedDial);
impl EcmDimKnb {
    pub const fn new() -> Self {
        Self(LimitedDial::new(17842, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EcmFrmBtnValue {
    Select1,
    Select2,
}
pub type EcmFrmBtnValueCallback<'a> = &'a mut dyn FnMut(EcmFrmBtnValue);
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
pub struct EcmFrmBtn(Selector);
impl EcmFrmBtn {
    pub const fn new() -> Self {
        Self(Selector::new(17692, 32768, 15))
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
pub type EcmPwSwValueCallback<'a> = &'a mut dyn FnMut(EcmPwSwValue);
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
pub struct EcmPwSw(Selector);
impl EcmPwSw {
    pub const fn new() -> Self {
        Self(Selector::new(17538, 6144, 11))
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
pub type EcmResetBtnValueCallback<'a> = &'a mut dyn FnMut(EcmResetBtnValue);
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
pub struct EcmResetBtn(Selector);
impl EcmResetBtn {
    pub const fn new() -> Self {
        Self(Selector::new(17538, 32768, 15))
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
pub type EcmSplBtnValueCallback<'a> = &'a mut dyn FnMut(EcmSplBtnValue);
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
pub struct EcmSplBtn(Selector);
impl EcmSplBtn {
    pub const fn new() -> Self {
        Self(Selector::new(17722, 256, 8))
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
pub type EcmXmitSwValueCallback<'a> = &'a mut dyn FnMut(EcmXmitSwValue);
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
pub struct EcmXmitSw(Selector);
impl EcmXmitSw {
    pub const fn new() -> Self {
        Self(Selector::new(17538, 24576, 13))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EhsiCrsSetValue {
    Select1,
    Select2,
}
pub type EhsiCrsSetValueCallback<'a> = &'a mut dyn FnMut(EhsiCrsSetValue);
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
pub struct EhsiCrsSet(Selector);
impl EhsiCrsSet {
    pub const fn new() -> Self {
        Self(Selector::new(17506, 1024, 10))
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
pub struct EhsiCrsSetKnb(AnalogDial);
impl EhsiCrsSetKnb {
    pub const fn new() -> Self {
        Self(AnalogDial::new(17516, 65535, 0))
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
pub type EhsiHdgSetBtnValueCallback<'a> = &'a mut dyn FnMut(EhsiHdgSetBtnValue);
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
pub struct EhsiHdgSetBtn(Selector);
impl EhsiHdgSetBtn {
    pub const fn new() -> Self {
        Self(Selector::new(17506, 2048, 11))
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
pub struct EhsiHdgSetKnb(AnalogDial);
impl EhsiHdgSetKnb {
    pub const fn new() -> Self {
        Self(AnalogDial::new(17518, 65535, 0))
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
pub type EhsiModeValueCallback<'a> = &'a mut dyn FnMut(EhsiModeValue);
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
pub struct EhsiMode(Selector);
impl EhsiMode {
    pub const fn new() -> Self {
        Self(Selector::new(17506, 4096, 12))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ElecCautionValue {
    Select1,
    Select2,
}
pub type ElecCautionValueCallback<'a> = &'a mut dyn FnMut(ElecCautionValue);
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
pub struct ElecCaution(Selector);
impl ElecCaution {
    pub const fn new() -> Self {
        Self(Selector::new(17416, 32768, 15))
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
pub enum EmergStroreJettValue {
    Select1,
    Select2,
}
pub type EmergStroreJettValueCallback<'a> = &'a mut dyn FnMut(EmergStroreJettValue);
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
pub struct EmergStroreJett(Selector);
impl EmergStroreJett {
    pub const fn new() -> Self {
        Self(Selector::new(17444, 128, 7))
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
pub enum EngAntiIceValue {
    Select1,
    Select2,
    Select3,
}
pub type EngAntiIceValueCallback<'a> = &'a mut dyn FnMut(EngAntiIceValue);
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
pub struct EngAntiIce(Selector);
impl EngAntiIce {
    pub const fn new() -> Self {
        Self(Selector::new(17440, 48, 4))
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
pub type EngContSwValueCallback<'a> = &'a mut dyn FnMut(EngContSwValue);
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
pub struct EngContSw(Selector);
impl EngContSw {
    pub const fn new() -> Self {
        Self(Selector::new(17440, 512, 9))
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
pub type EngContSwCoverValueCallback<'a> = &'a mut dyn FnMut(EngContSwCoverValue);
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
pub struct EngContSwCover(Selector);
impl EngContSwCover {
    pub const fn new() -> Self {
        Self(Selector::new(17440, 256, 8))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EngineFeedKnbValue {
    Select1,
    Select2,
    Select3,
    Select4,
}
pub type EngineFeedKnbValueCallback<'a> = &'a mut dyn FnMut(EngineFeedKnbValue);
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
pub struct EngineFeedKnb(Selector);
impl EngineFeedKnb {
    pub const fn new() -> Self {
        Self(Selector::new(17436, 3072, 10))
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

/// Engine FTIT Indicator
/// Output: gauge position
pub struct EngineFtit(AnalogGauge);
impl EngineFtit {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17622, 65535, 0))
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
pub struct EngineNozzlePosition(AnalogGauge);
impl EngineNozzlePosition {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17618, 65535, 0))
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
pub struct EngineOilPressure(AnalogGauge);
impl EngineOilPressure {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17616, 65535, 0))
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
pub struct EngineTachometer(AnalogGauge);
impl EngineTachometer {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17620, 65535, 0))
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
pub enum EpuGenTestSwValue {
    Select1,
    Select2,
}
pub type EpuGenTestSwValueCallback<'a> = &'a mut dyn FnMut(EpuGenTestSwValue);
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
pub struct EpuGenTestSw(Selector);
impl EpuGenTestSw {
    pub const fn new() -> Self {
        Self(Selector::new(17436, 4, 2))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EpuSwValue {
    Select1,
    Select2,
    Select3,
}
pub type EpuSwValueCallback<'a> = &'a mut dyn FnMut(EpuSwValue);
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
pub struct EpuSw(Selector);
impl EpuSw {
    pub const fn new() -> Self {
        Self(Selector::new(17440, 12, 2))
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
pub type EpuSwCoverOffValueCallback<'a> = &'a mut dyn FnMut(EpuSwCoverOffValue);
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
pub struct EpuSwCoverOff(Selector);
impl EpuSwCoverOff {
    pub const fn new() -> Self {
        Self(Selector::new(17440, 2, 1))
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
pub type EpuSwCoverOnValueCallback<'a> = &'a mut dyn FnMut(EpuSwCoverOnValue);
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
pub struct EpuSwCoverOn(Selector);
impl EpuSwCoverOn {
    pub const fn new() -> Self {
        Self(Selector::new(17440, 1, 0))
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

/// Formation Lights
/// Output: Formation Lights
pub struct ExtFormationLights(Metadata);
impl ExtFormationLights {
    pub const fn new() -> Self {
        Self(Metadata::new(17848, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExtFuelTransSwValue {
    Select1,
    Select2,
}
pub type ExtFuelTransSwValueCallback<'a> = &'a mut dyn FnMut(ExtFuelTransSwValue);
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
pub struct ExtFuelTransSw(Selector);
impl ExtFuelTransSw {
    pub const fn new() -> Self {
        Self(Selector::new(17436, 8192, 13))
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

/// Fuselage Position Lights (L-red; R-green;Back-white)
/// Output: Fuselage Position Lights (L-red; R-green;Back-white)
pub struct ExtPositionLightFuselage(Metadata);
impl ExtPositionLightFuselage {
    pub const fn new() -> Self {
        Self(Metadata::new(17722, 1024, 10))
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
pub struct ExtPositionLightsWing(Metadata);
impl ExtPositionLightsWing {
    pub const fn new() -> Self {
        Self(Metadata::new(17722, 512, 9))
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
pub struct ExtSpeedBrakeLeft(Metadata);
impl ExtSpeedBrakeLeft {
    pub const fn new() -> Self {
        Self(Metadata::new(17846, 65535, 0))
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
pub struct ExtSpeedBrakeRight(Metadata);
impl ExtSpeedBrakeRight {
    pub const fn new() -> Self {
        Self(Metadata::new(17844, 65535, 0))
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
pub struct ExtStrobeTail(Metadata);
impl ExtStrobeTail {
    pub const fn new() -> Self {
        Self(Metadata::new(17722, 2048, 11))
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
pub struct ExtTailLight(Metadata);
impl ExtTailLight {
    pub const fn new() -> Self {
        Self(Metadata::new(17850, 65535, 0))
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
pub struct ExtWowLeft(Metadata);
impl ExtWowLeft {
    pub const fn new() -> Self {
        Self(Metadata::new(17722, 16384, 14))
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
pub struct ExtWowNose(Metadata);
impl ExtWowNose {
    pub const fn new() -> Self {
        Self(Metadata::new(17722, 4096, 12))
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
pub struct ExtWowRight(Metadata);
impl ExtWowRight {
    pub const fn new() -> Self {
        Self(Metadata::new(17722, 8192, 13))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FAckBtnValue {
    Select1,
    Select2,
}
pub type FAckBtnValueCallback<'a> = &'a mut dyn FnMut(FAckBtnValue);
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
pub struct FAckBtn(Selector);
impl FAckBtn {
    pub const fn new() -> Self {
        Self(Selector::new(17456, 1024, 10))
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
pub enum FcrPwrSwValue {
    Select1,
    Select2,
}
pub type FcrPwrSwValueCallback<'a> = &'a mut dyn FnMut(FcrPwrSwValue);
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
pub struct FcrPwrSw(Selector);
impl FcrPwrSw {
    pub const fn new() -> Self {
        Self(Selector::new(17442, 64, 6))
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
pub enum FireOheatDetectBtnValue {
    Select1,
    Select2,
}
pub type FireOheatDetectBtnValueCallback<'a> = &'a mut dyn FnMut(FireOheatDetectBtnValue);
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
pub struct FireOheatDetectBtn(Selector);
impl FireOheatDetectBtn {
    pub const fn new() -> Self {
        Self(Selector::new(17440, 8192, 13))
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

/// FLCS PWR TEST Switch, MAINT/NORM/TEST
/// Input: set the switch position
/// Output: switch position -- 0 = Down, 1 = Mid ,  2 = UP
pub struct FlcsPwrTestSw(Selector);
impl FlcsPwrTestSw {
    pub const fn new() -> Self {
        Self(Selector::new(17436, 96, 5))
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
pub enum FlcsResetSwValue {
    Select1,
    Select2,
}
pub type FlcsResetSwValueCallback<'a> = &'a mut dyn FnMut(FlcsResetSwValue);
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
pub struct FlcsResetSw(Selector);
impl FlcsResetSw {
    pub const fn new() -> Self {
        Self(Selector::new(17408, 8, 3))
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

/// FLOOD CONSOLES BRT Knob
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct FloodConsolesBrtKnb(LimitedDial);
impl FloodConsolesBrtKnb {
    pub const fn new() -> Self {
        Self(LimitedDial::new(17428, 65535, 0))
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
pub struct FloodInstPnlBrtKnb(LimitedDial);
impl FloodInstPnlBrtKnb {
    pub const fn new() -> Self {
        Self(LimitedDial::new(17430, 65535, 0))
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

/// Flow Indicator Gauge
/// Output: gauge position
pub struct FlowIndicator(AnalogGauge);
impl FlowIndicator {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17646, 65535, 0))
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
pub struct FlowIndicatorLight(Led);
impl FlowIndicatorLight {
    pub const fn new() -> Self {
        Self(Led::new(17538, 512, 9))
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

/// FORM Light Knob
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct FormLightKnb(LimitedDial);
impl FormLightKnb {
    pub const fn new() -> Self {
        Self(LimitedDial::new(17418, 65535, 0))
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

/// Fuel Quantity Indicator AL
/// Output: gauge position
pub struct FuelAl(AnalogGauge);
impl FuelAl {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17632, 65535, 0))
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
pub struct FuelFr(AnalogGauge);
impl FuelFr {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17634, 65535, 0))
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
pub type FuelMasterCvValueCallback<'a> = &'a mut dyn FnMut(FuelMasterCvValue);
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
pub struct FuelMasterCv(Selector);
impl FuelMasterCv {
    pub const fn new() -> Self {
        Self(Selector::new(17436, 256, 8))
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
pub type FuelMasterSwValueCallback<'a> = &'a mut dyn FnMut(FuelMasterSwValue);
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
pub struct FuelMasterSw(Selector);
impl FuelMasterSw {
    pub const fn new() -> Self {
        Self(Selector::new(17436, 128, 7))
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
pub type FuelQtySelKnbValueCallback<'a> = &'a mut dyn FnMut(FuelQtySelKnbValue);
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
pub struct FuelQtySelKnb(Selector);
impl FuelQtySelKnb {
    pub const fn new() -> Self {
        Self(Selector::new(17438, 7, 0))
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
pub struct FuelQtySelTKnb(Selector);
impl FuelQtySelTKnb {
    pub const fn new() -> Self {
        Self(Selector::new(17436, 16384, 14))
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
pub struct Fuelflowcounter100(AnalogGauge);
impl Fuelflowcounter100 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17630, 65535, 0))
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
pub struct Fuelflowcounter10k(AnalogGauge);
impl Fuelflowcounter10k {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17626, 65535, 0))
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
pub struct Fuelflowcounter1k(AnalogGauge);
impl Fuelflowcounter1k {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17628, 65535, 0))
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
pub struct Fueltotalizer100(AnalogGauge);
impl Fueltotalizer100 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17640, 65535, 0))
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
pub struct Fueltotalizer10k(AnalogGauge);
impl Fueltotalizer10k {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17636, 65535, 0))
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
pub struct Fueltotalizer1k(AnalogGauge);
impl Fueltotalizer1k {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17638, 65535, 0))
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
pub enum GearHandleValue {
    Select1,
    Select2,
}
pub type GearHandleValueCallback<'a> = &'a mut dyn FnMut(GearHandleValue);
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
pub struct GearHandle(Selector);
impl GearHandle {
    pub const fn new() -> Self {
        Self(Selector::new(17436, 32768, 15))
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
pub enum GndJettEnableSwValue {
    Select1,
    Select2,
}
pub type GndJettEnableSwValueCallback<'a> = &'a mut dyn FnMut(GndJettEnableSwValue);
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
pub struct GndJettEnableSw(Selector);
impl GndJettEnableSw {
    pub const fn new() -> Self {
        Self(Selector::new(17444, 256, 8))
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
pub enum GpsSwValue {
    Select1,
    Select2,
}
pub type GpsSwValueCallback<'a> = &'a mut dyn FnMut(GpsSwValue);
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
pub struct GpsSw(Selector);
impl GpsSw {
    pub const fn new() -> Self {
        Self(Selector::new(17442, 8192, 13))
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
pub enum HdptSwLValue {
    Select1,
    Select2,
}
pub type HdptSwLValueCallback<'a> = &'a mut dyn FnMut(HdptSwLValue);
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
pub struct HdptSwL(Selector);
impl HdptSwL {
    pub const fn new() -> Self {
        Self(Selector::new(17442, 16, 4))
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
pub type HdptSwRValueCallback<'a> = &'a mut dyn FnMut(HdptSwRValue);
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
pub struct HdptSwR(Selector);
impl HdptSwR {
    pub const fn new() -> Self {
        Self(Selector::new(17442, 32, 5))
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
pub enum HideStickValue {
    Select1,
    Select2,
}
pub type HideStickValueCallback<'a> = &'a mut dyn FnMut(HideStickValue);
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
pub struct HideStick(Selector);
impl HideStick {
    pub const fn new() -> Self {
        Self(Selector::new(17522, 64, 6))
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

/// HMCS SYMBOLOGY INT Knob
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct HmcsIntKnb(LimitedDial);
impl HmcsIntKnb {
    pub const fn new() -> Self {
        Self(LimitedDial::new(17492, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HookSwValue {
    Select1,
    Select2,
}
pub type HookSwValueCallback<'a> = &'a mut dyn FnMut(HookSwValue);
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
pub struct HookSw(Selector);
impl HookSw {
    pub const fn new() -> Self {
        Self(Selector::new(17438, 16, 4))
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
pub type HornSilenceBtnValueCallback<'a> = &'a mut dyn FnMut(HornSilenceBtnValue);
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
pub struct HornSilenceBtn(Selector);
impl HornSilenceBtn {
    pub const fn new() -> Self {
        Self(Selector::new(17438, 32, 5))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HotMicSwValue {
    Select1,
    Select2,
    Select3,
}
pub type HotMicSwValueCallback<'a> = &'a mut dyn FnMut(HotMicSwValue);
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
pub struct HotMicSw(Selector);
impl HotMicSw {
    pub const fn new() -> Self {
        Self(Selector::new(17460, 12, 2))
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
pub enum HudAltSwValue {
    Select1,
    Select2,
    Select3,
}
pub type HudAltSwValueCallback<'a> = &'a mut dyn FnMut(HudAltSwValue);
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
pub struct HudAltSw(Selector);
impl HudAltSw {
    pub const fn new() -> Self {
        Self(Selector::new(17458, 768, 8))
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
pub type HudBrtSwValueCallback<'a> = &'a mut dyn FnMut(HudBrtSwValue);
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
pub struct HudBrtSw(Selector);
impl HudBrtSw {
    pub const fn new() -> Self {
        Self(Selector::new(17458, 3072, 10))
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
pub type HudDedDataSwValueCallback<'a> = &'a mut dyn FnMut(HudDedDataSwValue);
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
pub struct HudDedDataSw(Selector);
impl HudDedDataSw {
    pub const fn new() -> Self {
        Self(Selector::new(17458, 12, 2))
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
pub type HudDepressRetSwValueCallback<'a> = &'a mut dyn FnMut(HudDepressRetSwValue);
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
pub struct HudDepressRetSw(Selector);
impl HudDepressRetSw {
    pub const fn new() -> Self {
        Self(Selector::new(17458, 48, 4))
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
pub type HudFpMarkerSwValueCallback<'a> = &'a mut dyn FnMut(HudFpMarkerSwValue);
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
pub struct HudFpMarkerSw(Selector);
impl HudFpMarkerSw {
    pub const fn new() -> Self {
        Self(Selector::new(17458, 3, 0))
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
pub type HudScalesSwValueCallback<'a> = &'a mut dyn FnMut(HudScalesSwValue);
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
pub struct HudScalesSw(Selector);
impl HudScalesSw {
    pub const fn new() -> Self {
        Self(Selector::new(17456, 49152, 14))
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
pub type HudSpeedSwValueCallback<'a> = &'a mut dyn FnMut(HudSpeedSwValue);
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
pub struct HudSpeedSw(Selector);
impl HudSpeedSw {
    pub const fn new() -> Self {
        Self(Selector::new(17458, 192, 6))
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
pub type HudTestSwValueCallback<'a> = &'a mut dyn FnMut(HudTestSwValue);
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
pub struct HudTestSw(Selector);
impl HudTestSw {
    pub const fn new() -> Self {
        Self(Selector::new(17458, 12288, 12))
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

/// Hydrazin Volume Indicator
/// Output: gauge position
pub struct HydrazinVolume(AnalogGauge);
impl HydrazinVolume {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17624, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IcpAaModeBtnValue {
    Select1,
    Select2,
}
pub type IcpAaModeBtnValueCallback<'a> = &'a mut dyn FnMut(IcpAaModeBtnValue);
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
pub struct IcpAaModeBtn(Selector);
impl IcpAaModeBtn {
    pub const fn new() -> Self {
        Self(Selector::new(17446, 512, 9))
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
pub type IcpAgModeBtnValueCallback<'a> = &'a mut dyn FnMut(IcpAgModeBtnValue);
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
pub struct IcpAgModeBtn(Selector);
impl IcpAgModeBtn {
    pub const fn new() -> Self {
        Self(Selector::new(17446, 1024, 10))
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
pub type IcpBtn0ValueCallback<'a> = &'a mut dyn FnMut(IcpBtn0Value);
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
pub struct IcpBtn0(Selector);
impl IcpBtn0 {
    pub const fn new() -> Self {
        Self(Selector::new(17446, 16, 4))
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
pub type IcpBtn1ValueCallback<'a> = &'a mut dyn FnMut(IcpBtn1Value);
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
pub struct IcpBtn1(Selector);
impl IcpBtn1 {
    pub const fn new() -> Self {
        Self(Selector::new(17444, 2048, 11))
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
pub type IcpBtn2ValueCallback<'a> = &'a mut dyn FnMut(IcpBtn2Value);
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
pub struct IcpBtn2(Selector);
impl IcpBtn2 {
    pub const fn new() -> Self {
        Self(Selector::new(17444, 4096, 12))
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
pub type IcpBtn3ValueCallback<'a> = &'a mut dyn FnMut(IcpBtn3Value);
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
pub struct IcpBtn3(Selector);
impl IcpBtn3 {
    pub const fn new() -> Self {
        Self(Selector::new(17444, 8192, 13))
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
pub type IcpBtn4ValueCallback<'a> = &'a mut dyn FnMut(IcpBtn4Value);
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
pub struct IcpBtn4(Selector);
impl IcpBtn4 {
    pub const fn new() -> Self {
        Self(Selector::new(17444, 16384, 14))
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
pub type IcpBtn5ValueCallback<'a> = &'a mut dyn FnMut(IcpBtn5Value);
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
pub struct IcpBtn5(Selector);
impl IcpBtn5 {
    pub const fn new() -> Self {
        Self(Selector::new(17444, 32768, 15))
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
pub type IcpBtn6ValueCallback<'a> = &'a mut dyn FnMut(IcpBtn6Value);
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
pub struct IcpBtn6(Selector);
impl IcpBtn6 {
    pub const fn new() -> Self {
        Self(Selector::new(17446, 1, 0))
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
pub type IcpBtn7ValueCallback<'a> = &'a mut dyn FnMut(IcpBtn7Value);
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
pub struct IcpBtn7(Selector);
impl IcpBtn7 {
    pub const fn new() -> Self {
        Self(Selector::new(17446, 2, 1))
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
pub type IcpBtn8ValueCallback<'a> = &'a mut dyn FnMut(IcpBtn8Value);
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
pub struct IcpBtn8(Selector);
impl IcpBtn8 {
    pub const fn new() -> Self {
        Self(Selector::new(17446, 4, 2))
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
pub type IcpBtn9ValueCallback<'a> = &'a mut dyn FnMut(IcpBtn9Value);
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
pub struct IcpBtn9(Selector);
impl IcpBtn9 {
    pub const fn new() -> Self {
        Self(Selector::new(17446, 8, 3))
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
pub type IcpCom1BtnValueCallback<'a> = &'a mut dyn FnMut(IcpCom1BtnValue);
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
pub struct IcpCom1Btn(Selector);
impl IcpCom1Btn {
    pub const fn new() -> Self {
        Self(Selector::new(17446, 32, 5))
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
pub type IcpCom2BtnValueCallback<'a> = &'a mut dyn FnMut(IcpCom2BtnValue);
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
pub struct IcpCom2Btn(Selector);
impl IcpCom2Btn {
    pub const fn new() -> Self {
        Self(Selector::new(17446, 64, 6))
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
pub struct IcpDataRtnSeqSw(Selector);
impl IcpDataRtnSeqSw {
    pub const fn new() -> Self {
        Self(Selector::new(17456, 192, 6))
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
pub struct IcpDataUpDnSw(Selector);
impl IcpDataUpDnSw {
    pub const fn new() -> Self {
        Self(Selector::new(17456, 768, 8))
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
pub struct IcpDedSw(Selector);
impl IcpDedSw {
    pub const fn new() -> Self {
        Self(Selector::new(17456, 3, 0))
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
pub struct IcpDriftSw(Selector);
impl IcpDriftSw {
    pub const fn new() -> Self {
        Self(Selector::new(17456, 48, 4))
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
pub type IcpEntrBtnValueCallback<'a> = &'a mut dyn FnMut(IcpEntrBtnValue);
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
pub struct IcpEntrBtn(Selector);
impl IcpEntrBtn {
    pub const fn new() -> Self {
        Self(Selector::new(17446, 4096, 12))
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
pub type IcpFlirGainSwValueCallback<'a> = &'a mut dyn FnMut(IcpFlirGainSwValue);
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
pub struct IcpFlirGainSw(Selector);
impl IcpFlirGainSw {
    pub const fn new() -> Self {
        Self(Selector::new(17446, 49152, 14))
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
pub struct IcpFlirSw(Selector);
impl IcpFlirSw {
    pub const fn new() -> Self {
        Self(Selector::new(17456, 12, 2))
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
pub struct IcpHudBrtKnb(LimitedDial);
impl IcpHudBrtKnb {
    pub const fn new() -> Self {
        Self(LimitedDial::new(17454, 65535, 0))
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
pub type IcpIffBtnValueCallback<'a> = &'a mut dyn FnMut(IcpIffBtnValue);
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
pub struct IcpIffBtn(Selector);
impl IcpIffBtn {
    pub const fn new() -> Self {
        Self(Selector::new(17446, 128, 7))
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
pub type IcpListBtnValueCallback<'a> = &'a mut dyn FnMut(IcpListBtnValue);
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
pub struct IcpListBtn(Selector);
impl IcpListBtn {
    pub const fn new() -> Self {
        Self(Selector::new(17446, 256, 8))
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
pub struct IcpRasterBrtKnb(LimitedDial);
impl IcpRasterBrtKnb {
    pub const fn new() -> Self {
        Self(LimitedDial::new(17452, 65535, 0))
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
pub struct IcpRasterContrKnb(LimitedDial);
impl IcpRasterContrKnb {
    pub const fn new() -> Self {
        Self(LimitedDial::new(17450, 65535, 0))
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
pub type IcpRclBtnValueCallback<'a> = &'a mut dyn FnMut(IcpRclBtnValue);
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
pub struct IcpRclBtn(Selector);
impl IcpRclBtn {
    pub const fn new() -> Self {
        Self(Selector::new(17446, 2048, 11))
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
pub struct IcpReticleDepressKnb(LimitedDial);
impl IcpReticleDepressKnb {
    pub const fn new() -> Self {
        Self(LimitedDial::new(17448, 65535, 0))
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
pub type IcpWxBtnValueCallback<'a> = &'a mut dyn FnMut(IcpWxBtnValue);
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
pub struct IcpWxBtn(Selector);
impl IcpWxBtn {
    pub const fn new() -> Self {
        Self(Selector::new(17446, 8192, 13))
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
pub enum IffAntSelSwValue {
    Select1,
    Select2,
    Select3,
}
pub type IffAntSelSwValueCallback<'a> = &'a mut dyn FnMut(IffAntSelSwValue);
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
pub struct IffAntSelSw(Selector);
impl IffAntSelSw {
    pub const fn new() -> Self {
        Self(Selector::new(17460, 96, 5))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IffCIKnbValue {
    Select1,
    Select2,
}
pub type IffCIKnbValueCallback<'a> = &'a mut dyn FnMut(IffCIKnbValue);
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
pub struct IffCIKnb(Selector);
impl IffCIKnb {
    pub const fn new() -> Self {
        Self(Selector::new(17482, 128, 7))
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
pub struct IffCodeDrumDigit1(AnalogGauge);
impl IffCodeDrumDigit1 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17648, 65535, 0))
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
pub struct IffCodeDrumDigit2(AnalogGauge);
impl IffCodeDrumDigit2 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17650, 65535, 0))
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
pub struct IffCodeDrumDigit3(AnalogGauge);
impl IffCodeDrumDigit3 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17652, 65535, 0))
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
pub struct IffCodeDrumDigit4(AnalogGauge);
impl IffCodeDrumDigit4 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17654, 65535, 0))
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
pub type IffEnableSwValueCallback<'a> = &'a mut dyn FnMut(IffEnableSwValue);
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
pub struct IffEnableSw(Selector);
impl IffEnableSw {
    pub const fn new() -> Self {
        Self(Selector::new(17486, 3, 0))
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
pub enum IffIdBtnValue {
    Select1,
    Select2,
}
pub type IffIdBtnValueCallback<'a> = &'a mut dyn FnMut(IffIdBtnValue);
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
pub struct IffIdBtn(Selector);
impl IffIdBtn {
    pub const fn new() -> Self {
        Self(Selector::new(17456, 2048, 11))
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
pub enum IffM1Sel1Value {
    Select1,
    Select2,
    Select3,
}
pub type IffM1Sel1ValueCallback<'a> = &'a mut dyn FnMut(IffM1Sel1Value);
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
pub struct IffM1Sel1(Selector);
impl IffM1Sel1 {
    pub const fn new() -> Self {
        Self(Selector::new(17486, 12, 2))
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
pub type IffM1Sel2ValueCallback<'a> = &'a mut dyn FnMut(IffM1Sel2Value);
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
pub struct IffM1Sel2(Selector);
impl IffM1Sel2 {
    pub const fn new() -> Self {
        Self(Selector::new(17486, 48, 4))
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
pub type IffM3Sel1ValueCallback<'a> = &'a mut dyn FnMut(IffM3Sel1Value);
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
pub struct IffM3Sel1(Selector);
impl IffM3Sel1 {
    pub const fn new() -> Self {
        Self(Selector::new(17486, 192, 6))
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
pub type IffM3Sel2ValueCallback<'a> = &'a mut dyn FnMut(IffM3Sel2Value);
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
pub struct IffM3Sel2(Selector);
impl IffM3Sel2 {
    pub const fn new() -> Self {
        Self(Selector::new(17486, 768, 8))
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
pub type IffM4CodeSwValueCallback<'a> = &'a mut dyn FnMut(IffM4CodeSwValue);
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
pub struct IffM4CodeSw(Selector);
impl IffM4CodeSw {
    pub const fn new() -> Self {
        Self(Selector::new(17482, 6144, 11))
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
pub type IffM4MonitorSwValueCallback<'a> = &'a mut dyn FnMut(IffM4MonitorSwValue);
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
pub struct IffM4MonitorSw(Selector);
impl IffM4MonitorSw {
    pub const fn new() -> Self {
        Self(Selector::new(17482, 32768, 15))
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
pub type IffM4ReplySwValueCallback<'a> = &'a mut dyn FnMut(IffM4ReplySwValue);
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
pub struct IffM4ReplySw(Selector);
impl IffM4ReplySw {
    pub const fn new() -> Self {
        Self(Selector::new(17482, 24576, 13))
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
pub type IffMasterKnbValueCallback<'a> = &'a mut dyn FnMut(IffMasterKnbValue);
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
pub struct IffMasterKnb(Selector);
impl IffMasterKnb {
    pub const fn new() -> Self {
        Self(Selector::new(17482, 1792, 8))
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

/// ILS Power Knob
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct IlsPwrKnb(LimitedDial);
impl IlsPwrKnb {
    pub const fn new() -> Self {
        Self(LimitedDial::new(17478, 65535, 0))
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
pub type InsKnbValueCallback<'a> = &'a mut dyn FnMut(InsKnbValue);
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
pub struct InsKnb(Selector);
impl InsKnb {
    pub const fn new() -> Self {
        Self(Selector::new(17444, 7, 0))
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

/// INTERCOM Knob
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct IntercomKnb(LimitedDial);
impl IntercomKnb {
    pub const fn new() -> Self {
        Self(LimitedDial::new(17474, 65535, 0))
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

/// JFS Switch, START 1/OFF/START 2
/// Input: set the switch position
/// Output: switch position -- 0 = Down, 1 = Mid ,  2 = UP
pub struct JfsSw(Selector);
impl JfsSw {
    pub const fn new() -> Self {
        Self(Selector::new(17440, 192, 6))
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
pub type Ky58FillKnbValueCallback<'a> = &'a mut dyn FnMut(Ky58FillKnbValue);
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
pub struct Ky58FillKnb(Selector);
impl Ky58FillKnb {
    pub const fn new() -> Self {
        Self(Selector::new(17486, 28672, 12))
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
pub type Ky58ModeKnbValueCallback<'a> = &'a mut dyn FnMut(Ky58ModeKnbValue);
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
pub struct Ky58ModeKnb(Selector);
impl Ky58ModeKnb {
    pub const fn new() -> Self {
        Self(Selector::new(17486, 3072, 10))
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
pub type Ky58PwrKnbValueCallback<'a> = &'a mut dyn FnMut(Ky58PwrKnbValue);
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
pub struct Ky58PwrKnb(Selector);
impl Ky58PwrKnb {
    pub const fn new() -> Self {
        Self(Selector::new(17490, 3, 0))
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
pub struct Ky58VolKnb(LimitedDial);
impl Ky58VolKnb {
    pub const fn new() -> Self {
        Self(LimitedDial::new(17488, 65535, 0))
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
pub enum LandTaxiLightSwValue {
    Select1,
    Select2,
    Select3,
}
pub type LandTaxiLightSwValueCallback<'a> = &'a mut dyn FnMut(LandTaxiLightSwValue);
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
pub struct LandTaxiLightSw(Selector);
impl LandTaxiLightSw {
    pub const fn new() -> Self {
        Self(Selector::new(17416, 3072, 10))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LaserArmSwValue {
    Select1,
    Select2,
}
pub type LaserArmSwValueCallback<'a> = &'a mut dyn FnMut(LaserArmSwValue);
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
pub struct LaserArmSw(Selector);
impl LaserArmSw {
    pub const fn new() -> Self {
        Self(Selector::new(17444, 1024, 10))
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
pub enum LeFlapsSwValue {
    Select1,
    Select2,
}
pub type LeFlapsSwValueCallback<'a> = &'a mut dyn FnMut(LeFlapsSwValue);
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
pub struct LeFlapsSw(Selector);
impl LeFlapsSw {
    pub const fn new() -> Self {
        Self(Selector::new(17408, 16, 4))
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

/// ACFT BATT FAIL Light (yellow)
/// Output: 0 if light is off, 1 if light is on
pub struct LightAcftBattFail(Led);
impl LightAcftBattFail {
    pub const fn new() -> Self {
        Self(Led::new(17530, 256, 8))
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
pub struct LightActive(Led);
impl LightActive {
    pub const fn new() -> Self {
        Self(Led::new(17530, 512, 9))
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
pub struct LightAftFuelLow(Led);
impl LightAftFuelLow {
    pub const fn new() -> Self {
        Self(Led::new(17526, 8, 3))
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
pub struct LightAir(Led);
impl LightAir {
    pub const fn new() -> Self {
        Self(Led::new(17528, 16384, 14))
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
pub struct LightAntiSkid(Led);
impl LightAntiSkid {
    pub const fn new() -> Self {
        Self(Led::new(17524, 4, 2))
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
pub struct LightAoaDn(Led);
impl LightAoaDn {
    pub const fn new() -> Self {
        Self(Led::new(17528, 16, 4))
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
pub struct LightAoaMid(Led);
impl LightAoaMid {
    pub const fn new() -> Self {
        Self(Led::new(17528, 8, 3))
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
pub struct LightAoaUp(Led);
impl LightAoaUp {
    pub const fn new() -> Self {
        Self(Led::new(17528, 4, 2))
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
pub struct LightArNws(Led);
impl LightArNws {
    pub const fn new() -> Self {
        Self(Led::new(17528, 1024, 10))
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
pub struct LightAtfNot(Led);
impl LightAtfNot {
    pub const fn new() -> Self {
        Self(Led::new(17524, 2048, 11))
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
pub struct LightAvionicsFault(Led);
impl LightAvionicsFault {
    pub const fn new() -> Self {
        Self(Led::new(17522, 512, 9))
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
pub struct LightBuc(Led);
impl LightBuc {
    pub const fn new() -> Self {
        Self(Led::new(17526, 1, 0))
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
pub struct LightCabinPress(Led);
impl LightCabinPress {
    pub const fn new() -> Self {
        Self(Led::new(17524, 16384, 14))
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
pub struct LightCadc(Led);
impl LightCadc {
    pub const fn new() -> Self {
        Self(Led::new(17524, 8, 3))
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
pub struct LightCanopy(Led);
impl LightCanopy {
    pub const fn new() -> Self {
        Self(Led::new(17528, 1, 0))
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
pub struct LightCaution1(Led);
impl LightCaution1 {
    pub const fn new() -> Self {
        Self(Led::new(17524, 8192, 13))
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
pub struct LightCaution2(Led);
impl LightCaution2 {
    pub const fn new() -> Self {
        Self(Led::new(17526, 2, 1))
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
pub struct LightCaution3(Led);
impl LightCaution3 {
    pub const fn new() -> Self {
        Self(Led::new(17526, 4, 2))
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
pub struct LightCaution4(Led);
impl LightCaution4 {
    pub const fn new() -> Self {
        Self(Led::new(17526, 16, 4))
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
pub struct LightCaution5(Led);
impl LightCaution5 {
    pub const fn new() -> Self {
        Self(Led::new(17526, 32, 5))
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
pub struct LightCaution6(Led);
impl LightCaution6 {
    pub const fn new() -> Self {
        Self(Led::new(17526, 64, 6))
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
pub struct LightCmdsDisp(Led);
impl LightCmdsDisp {
    pub const fn new() -> Self {
        Self(Led::new(17534, 4, 2))
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
pub struct LightCmdsGo(Led);
impl LightCmdsGo {
    pub const fn new() -> Self {
        Self(Led::new(17534, 2, 1))
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
pub struct LightCmdsNoGo(Led);
impl LightCmdsNoGo {
    pub const fn new() -> Self {
        Self(Led::new(17534, 1, 0))
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
pub struct LightCmdsRdy(Led);
impl LightCmdsRdy {
    pub const fn new() -> Self {
        Self(Led::new(17534, 8, 3))
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
pub struct LightConsles(Led);
impl LightConsles {
    pub const fn new() -> Self {
        Self(Led::new(17534, 16, 4))
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
pub struct LightConslesFlood(Led);
impl LightConslesFlood {
    pub const fn new() -> Self {
        Self(Led::new(17534, 64, 6))
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
pub struct LightDbuOn(Led);
impl LightDbuOn {
    pub const fn new() -> Self {
        Self(Led::new(17526, 16384, 14))
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
pub struct LightDisc(Led);
impl LightDisc {
    pub const fn new() -> Self {
        Self(Led::new(17528, 2048, 11))
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
pub struct LightEcm1A(Led);
impl LightEcm1A {
    pub const fn new() -> Self {
        Self(Led::new(17534, 512, 9))
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
pub struct LightEcm1F(Led);
impl LightEcm1F {
    pub const fn new() -> Self {
        Self(Led::new(17534, 1024, 10))
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
pub struct LightEcm1S(Led);
impl LightEcm1S {
    pub const fn new() -> Self {
        Self(Led::new(17534, 256, 8))
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
pub struct LightEcm1T(Led);
impl LightEcm1T {
    pub const fn new() -> Self {
        Self(Led::new(17534, 2048, 11))
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
pub struct LightEcm2A(Led);
impl LightEcm2A {
    pub const fn new() -> Self {
        Self(Led::new(17534, 8192, 13))
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
pub struct LightEcm2F(Led);
impl LightEcm2F {
    pub const fn new() -> Self {
        Self(Led::new(17534, 16384, 14))
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
pub struct LightEcm2S(Led);
impl LightEcm2S {
    pub const fn new() -> Self {
        Self(Led::new(17534, 4096, 12))
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
pub struct LightEcm2T(Led);
impl LightEcm2T {
    pub const fn new() -> Self {
        Self(Led::new(17534, 32768, 15))
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
pub struct LightEcm3A(Led);
impl LightEcm3A {
    pub const fn new() -> Self {
        Self(Led::new(17536, 2, 1))
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
pub struct LightEcm3F(Led);
impl LightEcm3F {
    pub const fn new() -> Self {
        Self(Led::new(17536, 4, 2))
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
pub struct LightEcm3S(Led);
impl LightEcm3S {
    pub const fn new() -> Self {
        Self(Led::new(17536, 1, 0))
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
pub struct LightEcm3T(Led);
impl LightEcm3T {
    pub const fn new() -> Self {
        Self(Led::new(17536, 8, 3))
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
pub struct LightEcm4A(Led);
impl LightEcm4A {
    pub const fn new() -> Self {
        Self(Led::new(17536, 32, 5))
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
pub struct LightEcm4F(Led);
impl LightEcm4F {
    pub const fn new() -> Self {
        Self(Led::new(17536, 64, 6))
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
pub struct LightEcm4S(Led);
impl LightEcm4S {
    pub const fn new() -> Self {
        Self(Led::new(17536, 16, 4))
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
pub struct LightEcm4T(Led);
impl LightEcm4T {
    pub const fn new() -> Self {
        Self(Led::new(17536, 128, 7))
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
pub struct LightEcm5A(Led);
impl LightEcm5A {
    pub const fn new() -> Self {
        Self(Led::new(17536, 512, 9))
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
pub struct LightEcm5F(Led);
impl LightEcm5F {
    pub const fn new() -> Self {
        Self(Led::new(17536, 1024, 10))
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
pub struct LightEcm5S(Led);
impl LightEcm5S {
    pub const fn new() -> Self {
        Self(Led::new(17536, 256, 8))
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
pub struct LightEcm5T(Led);
impl LightEcm5T {
    pub const fn new() -> Self {
        Self(Led::new(17536, 2048, 11))
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
pub struct LightEcmA(Led);
impl LightEcmA {
    pub const fn new() -> Self {
        Self(Led::new(17536, 8192, 13))
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
pub struct LightEcmF(Led);
impl LightEcmF {
    pub const fn new() -> Self {
        Self(Led::new(17536, 16384, 14))
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
pub struct LightEcmFrmA(Led);
impl LightEcmFrmA {
    pub const fn new() -> Self {
        Self(Led::new(17538, 2, 1))
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
pub struct LightEcmFrmF(Led);
impl LightEcmFrmF {
    pub const fn new() -> Self {
        Self(Led::new(17538, 4, 2))
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
pub struct LightEcmFrmS(Led);
impl LightEcmFrmS {
    pub const fn new() -> Self {
        Self(Led::new(17538, 1, 0))
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
pub struct LightEcmFrmT(Led);
impl LightEcmFrmT {
    pub const fn new() -> Self {
        Self(Led::new(17538, 8, 3))
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
pub struct LightEcmS(Led);
impl LightEcmS {
    pub const fn new() -> Self {
        Self(Led::new(17536, 4096, 12))
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
pub struct LightEcmSplA(Led);
impl LightEcmSplA {
    pub const fn new() -> Self {
        Self(Led::new(17538, 32, 5))
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
pub struct LightEcmSplF(Led);
impl LightEcmSplF {
    pub const fn new() -> Self {
        Self(Led::new(17538, 64, 6))
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
pub struct LightEcmSplS(Led);
impl LightEcmSplS {
    pub const fn new() -> Self {
        Self(Led::new(17538, 16, 4))
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
pub struct LightEcmSplT(Led);
impl LightEcmSplT {
    pub const fn new() -> Self {
        Self(Led::new(17538, 128, 7))
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
pub struct LightEcmT(Led);
impl LightEcmT {
    pub const fn new() -> Self {
        Self(Led::new(17536, 32768, 15))
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
pub struct LightEdge(Led);
impl LightEdge {
    pub const fn new() -> Self {
        Self(Led::new(17526, 256, 8))
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
pub struct LightEec(Led);
impl LightEec {
    pub const fn new() -> Self {
        Self(Led::new(17524, 4096, 12))
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
pub struct LightElec(Led);
impl LightElec {
    pub const fn new() -> Self {
        Self(Led::new(17530, 8, 3))
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
pub struct LightElecSys(Led);
impl LightElecSys {
    pub const fn new() -> Self {
        Self(Led::new(17522, 2048, 11))
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
pub struct LightEngFire(Led);
impl LightEngFire {
    pub const fn new() -> Self {
        Self(Led::new(17526, 1024, 10))
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
pub struct LightEngine(Led);
impl LightEngine {
    pub const fn new() -> Self {
        Self(Led::new(17526, 2048, 11))
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
pub struct LightEngineFault(Led);
impl LightEngineFault {
    pub const fn new() -> Self {
        Self(Led::new(17522, 256, 8))
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
pub struct LightEpu(Led);
impl LightEpu {
    pub const fn new() -> Self {
        Self(Led::new(17528, 32768, 15))
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
pub struct LightEpuGen(Led);
impl LightEpuGen {
    pub const fn new() -> Self {
        Self(Led::new(17530, 16, 4))
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
pub struct LightEpuPmg(Led);
impl LightEpuPmg {
    pub const fn new() -> Self {
        Self(Led::new(17530, 32, 5))
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
pub struct LightEquipHot(Led);
impl LightEquipHot {
    pub const fn new() -> Self {
        Self(Led::new(17522, 8192, 13))
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
pub struct LightFlFail(Led);
impl LightFlFail {
    pub const fn new() -> Self {
        Self(Led::new(17530, 4096, 12))
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
pub struct LightFlRun(Led);
impl LightFlRun {
    pub const fn new() -> Self {
        Self(Led::new(17530, 2048, 11))
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
pub struct LightFlcs(Led);
impl LightFlcs {
    pub const fn new() -> Self {
        Self(Led::new(17526, 8192, 13))
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
pub struct LightFlcsFault(Led);
impl LightFlcsFault {
    pub const fn new() -> Self {
        Self(Led::new(17522, 128, 7))
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
pub struct LightFlcsPmg(Led);
impl LightFlcsPmg {
    pub const fn new() -> Self {
        Self(Led::new(17530, 1, 0))
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
pub struct LightFlcsPwrA(Led);
impl LightFlcsPwrA {
    pub const fn new() -> Self {
        Self(Led::new(17530, 8192, 13))
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
pub struct LightFlcsPwrB(Led);
impl LightFlcsPwrB {
    pub const fn new() -> Self {
        Self(Led::new(17530, 16384, 14))
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
pub struct LightFlcsPwrC(Led);
impl LightFlcsPwrC {
    pub const fn new() -> Self {
        Self(Led::new(17530, 32768, 15))
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
pub struct LightFlcsPwrD(Led);
impl LightFlcsPwrD {
    pub const fn new() -> Self {
        Self(Led::new(17532, 1, 0))
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
pub struct LightFlcsRly(Led);
impl LightFlcsRly {
    pub const fn new() -> Self {
        Self(Led::new(17530, 128, 7))
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
pub struct LightFuelOilHot(Led);
impl LightFuelOilHot {
    pub const fn new() -> Self {
        Self(Led::new(17524, 1, 0))
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
pub struct LightFwdFuelLow(Led);
impl LightFwdFuelLow {
    pub const fn new() -> Self {
        Self(Led::new(17524, 32768, 15))
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
pub struct LightGearL(Led);
impl LightGearL {
    pub const fn new() -> Self {
        Self(Led::new(17528, 64, 6))
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
pub struct LightGearN(Led);
impl LightGearN {
    pub const fn new() -> Self {
        Self(Led::new(17528, 32, 5))
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
pub struct LightGearR(Led);
impl LightGearR {
    pub const fn new() -> Self {
        Self(Led::new(17528, 128, 7))
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
pub struct LightGearWarn(Led);
impl LightGearWarn {
    pub const fn new() -> Self {
        Self(Led::new(17528, 256, 8))
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
pub struct LightHook(Led);
impl LightHook {
    pub const fn new() -> Self {
        Self(Led::new(17524, 64, 6))
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
pub struct LightHydOilPress(Led);
impl LightHydOilPress {
    pub const fn new() -> Self {
        Self(Led::new(17526, 4096, 12))
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
pub struct LightHydrazn(Led);
impl LightHydrazn {
    pub const fn new() -> Self {
        Self(Led::new(17528, 8192, 13))
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
pub struct LightIff(Led);
impl LightIff {
    pub const fn new() -> Self {
        Self(Led::new(17524, 32, 5))
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
pub struct LightInletIcing(Led);
impl LightInletIcing {
    pub const fn new() -> Self {
        Self(Led::new(17524, 16, 4))
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
pub struct LightInstPnl(Led);
impl LightInstPnl {
    pub const fn new() -> Self {
        Self(Led::new(17534, 32, 5))
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
pub struct LightInstPnlFlood(Led);
impl LightInstPnlFlood {
    pub const fn new() -> Self {
        Self(Led::new(17534, 128, 7))
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
pub struct LightJfsRun(Led);
impl LightJfsRun {
    pub const fn new() -> Self {
        Self(Led::new(17528, 4096, 12))
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
pub struct LightMainGen(Led);
impl LightMainGen {
    pub const fn new() -> Self {
        Self(Led::new(17530, 2, 1))
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
pub struct LightMarkerBeacon(Led);
impl LightMarkerBeacon {
    pub const fn new() -> Self {
        Self(Led::new(17538, 256, 8))
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
pub struct LightMasterCaution(Led);
impl LightMasterCaution {
    pub const fn new() -> Self {
        Self(Led::new(17526, 128, 7))
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
pub struct LightNuclear(Led);
impl LightNuclear {
    pub const fn new() -> Self {
        Self(Led::new(17524, 512, 9))
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
pub struct LightNwsFail(Led);
impl LightNwsFail {
    pub const fn new() -> Self {
        Self(Led::new(17522, 16384, 14))
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
pub struct LightObogs(Led);
impl LightObogs {
    pub const fn new() -> Self {
        Self(Led::new(17524, 1024, 10))
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
pub struct LightOverheat(Led);
impl LightOverheat {
    pub const fn new() -> Self {
        Self(Led::new(17524, 256, 8))
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
pub struct LightOxyLow(Led);
impl LightOxyLow {
    pub const fn new() -> Self {
        Self(Led::new(17528, 2, 1))
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
pub struct LightProbeHeat(Led);
impl LightProbeHeat {
    pub const fn new() -> Self {
        Self(Led::new(17522, 32768, 15))
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
pub struct LightRadarAlt(Led);
impl LightRadarAlt {
    pub const fn new() -> Self {
        Self(Led::new(17524, 2, 1))
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
pub struct LightRdy(Led);
impl LightRdy {
    pub const fn new() -> Self {
        Self(Led::new(17528, 512, 9))
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
pub struct LightRwrActPower(Led);
impl LightRwrActPower {
    pub const fn new() -> Self {
        Self(Led::new(17532, 8, 3))
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
pub struct LightRwrActivity(Led);
impl LightRwrActivity {
    pub const fn new() -> Self {
        Self(Led::new(17532, 4, 2))
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
pub struct LightRwrAlt(Led);
impl LightRwrAlt {
    pub const fn new() -> Self {
        Self(Led::new(17532, 32, 5))
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
pub struct LightRwrAltLow(Led);
impl LightRwrAltLow {
    pub const fn new() -> Self {
        Self(Led::new(17532, 16, 4))
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
pub struct LightRwrHandoffH(Led);
impl LightRwrHandoffH {
    pub const fn new() -> Self {
        Self(Led::new(17532, 256, 8))
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
pub struct LightRwrHandoffUp(Led);
impl LightRwrHandoffUp {
    pub const fn new() -> Self {
        Self(Led::new(17532, 128, 7))
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
pub struct LightRwrModeOpen(Led);
impl LightRwrModeOpen {
    pub const fn new() -> Self {
        Self(Led::new(17532, 2048, 11))
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
pub struct LightRwrModePri(Led);
impl LightRwrModePri {
    pub const fn new() -> Self {
        Self(Led::new(17532, 1024, 10))
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
pub struct LightRwrMslLaunch(Led);
impl LightRwrMslLaunch {
    pub const fn new() -> Self {
        Self(Led::new(17532, 512, 9))
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
pub struct LightRwrPower(Led);
impl LightRwrPower {
    pub const fn new() -> Self {
        Self(Led::new(17532, 64, 6))
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
pub struct LightRwrSearch(Led);
impl LightRwrSearch {
    pub const fn new() -> Self {
        Self(Led::new(17532, 2, 1))
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
pub struct LightRwrShipUnk(Led);
impl LightRwrShipUnk {
    pub const fn new() -> Self {
        Self(Led::new(17532, 4096, 12))
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
pub struct LightRwrSystest(Led);
impl LightRwrSystest {
    pub const fn new() -> Self {
        Self(Led::new(17532, 8192, 13))
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
pub struct LightRwrTgtsepDn(Led);
impl LightRwrTgtsepDn {
    pub const fn new() -> Self {
        Self(Led::new(17532, 32768, 15))
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
pub struct LightRwrTgtsepUp(Led);
impl LightRwrTgtsepUp {
    pub const fn new() -> Self {
        Self(Led::new(17532, 16384, 14))
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
pub struct LightSeatNot(Led);
impl LightSeatNot {
    pub const fn new() -> Self {
        Self(Led::new(17522, 1024, 10))
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
pub struct LightSec(Led);
impl LightSec {
    pub const fn new() -> Self {
        Self(Led::new(17522, 4096, 12))
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
pub struct LightStby(Led);
impl LightStby {
    pub const fn new() -> Self {
        Self(Led::new(17530, 1024, 10))
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
pub struct LightStbyGen(Led);
impl LightStbyGen {
    pub const fn new() -> Self {
        Self(Led::new(17530, 4, 2))
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
pub struct LightStoresConfig(Led);
impl LightStoresConfig {
    pub const fn new() -> Self {
        Self(Led::new(17524, 128, 7))
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
pub struct LightTfFail(Led);
impl LightTfFail {
    pub const fn new() -> Self {
        Self(Led::new(17526, 512, 9))
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
pub struct LightToFlcs(Led);
impl LightToFlcs {
    pub const fn new() -> Self {
        Self(Led::new(17530, 64, 6))
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
pub struct LightToLdgConfig(Led);
impl LightToLdgConfig {
    pub const fn new() -> Self {
        Self(Led::new(17526, 32768, 15))
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

/// Mach Indicator
/// Output: gauge position
pub struct MachIndicator(AnalogGauge);
impl MachIndicator {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17570, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MainPwrSwValue {
    Select1,
    Select2,
    Select3,
}
pub type MainPwrSwValueCallback<'a> = &'a mut dyn FnMut(MainPwrSwValue);
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
pub struct MainPwrSw(Selector);
impl MainPwrSw {
    pub const fn new() -> Self {
        Self(Selector::new(17436, 3, 0))
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

/// MAL & IND LTS Switch, BRT/Center/DIM
/// Input: set the switch position
/// Output: switch position -- 0 = Down, 1 = Mid ,  2 = UP
pub struct MalIndLtsBrtSw(Selector);
impl MalIndLtsBrtSw {
    pub const fn new() -> Self {
        Self(Selector::new(17416, 24576, 13))
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
pub enum MalIndLtsTestValue {
    Select1,
    Select2,
}
pub type MalIndLtsTestValueCallback<'a> = &'a mut dyn FnMut(MalIndLtsTestValue);
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
pub struct MalIndLtsTest(Selector);
impl MalIndLtsTest {
    pub const fn new() -> Self {
        Self(Selector::new(17416, 4096, 12))
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
pub enum ManTfFlyupSwValue {
    Select1,
    Select2,
}
pub type ManTfFlyupSwValueCallback<'a> = &'a mut dyn FnMut(ManTfFlyupSwValue);
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
pub struct ManTfFlyupSw(Selector);
impl ManTfFlyupSw {
    pub const fn new() -> Self {
        Self(Selector::new(17408, 8192, 13))
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
pub type ManualPitchSwValueCallback<'a> = &'a mut dyn FnMut(ManualPitchSwValue);
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
pub struct ManualPitchSw(Selector);
impl ManualPitchSw {
    pub const fn new() -> Self {
        Self(Selector::new(17408, 64, 6))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MapSwValue {
    Select1,
    Select2,
}
pub type MapSwValueCallback<'a> = &'a mut dyn FnMut(MapSwValue);
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
pub struct MapSw(Selector);
impl MapSw {
    pub const fn new() -> Self {
        Self(Selector::new(17444, 8, 3))
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
pub enum MasterArmSwValue {
    Select1,
    Select2,
    Select3,
}
pub type MasterArmSwValueCallback<'a> = &'a mut dyn FnMut(MasterArmSwValue);
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
pub struct MasterArmSw(Selector);
impl MasterArmSw {
    pub const fn new() -> Self {
        Self(Selector::new(17444, 96, 5))
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
pub enum MasterCautionValue {
    Select1,
    Select2,
}
pub type MasterCautionValueCallback<'a> = &'a mut dyn FnMut(MasterCautionValue);
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
pub struct MasterCaution(Selector);
impl MasterCaution {
    pub const fn new() -> Self {
        Self(Selector::new(17408, 32768, 15))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MasterLightSwValue {
    Select1,
    Select2,
    Select3,
    Select4,
    Select5,
}
pub type MasterLightSwValueCallback<'a> = &'a mut dyn FnMut(MasterLightSwValue);
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
pub struct MasterLightSw(Selector);
impl MasterLightSw {
    pub const fn new() -> Self {
        Self(Selector::new(17416, 896, 7))
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

/// Max Airspeed
/// Output: gauge position
pub struct MaxAirspeed(AnalogGauge);
impl MaxAirspeed {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17566, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MaxPwrSwValue {
    Select1,
    Select2,
}
pub type MaxPwrSwValueCallback<'a> = &'a mut dyn FnMut(MaxPwrSwValue);
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
pub struct MaxPwrSw(Selector);
impl MaxPwrSw {
    pub const fn new() -> Self {
        Self(Selector::new(17440, 1024, 10))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MfdL1Value {
    Select1,
    Select2,
}
pub type MfdL1ValueCallback<'a> = &'a mut dyn FnMut(MfdL1Value);
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
pub struct MfdL1(Selector);
impl MfdL1 {
    pub const fn new() -> Self {
        Self(Selector::new(17498, 32768, 15))
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
pub type MfdL10ValueCallback<'a> = &'a mut dyn FnMut(MfdL10Value);
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
pub struct MfdL10(Selector);
impl MfdL10 {
    pub const fn new() -> Self {
        Self(Selector::new(17500, 256, 8))
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
pub type MfdL11ValueCallback<'a> = &'a mut dyn FnMut(MfdL11Value);
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
pub struct MfdL11(Selector);
impl MfdL11 {
    pub const fn new() -> Self {
        Self(Selector::new(17500, 512, 9))
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
pub type MfdL12ValueCallback<'a> = &'a mut dyn FnMut(MfdL12Value);
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
pub struct MfdL12(Selector);
impl MfdL12 {
    pub const fn new() -> Self {
        Self(Selector::new(17500, 1024, 10))
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
pub type MfdL13ValueCallback<'a> = &'a mut dyn FnMut(MfdL13Value);
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
pub struct MfdL13(Selector);
impl MfdL13 {
    pub const fn new() -> Self {
        Self(Selector::new(17500, 2048, 11))
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
pub type MfdL14ValueCallback<'a> = &'a mut dyn FnMut(MfdL14Value);
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
pub struct MfdL14(Selector);
impl MfdL14 {
    pub const fn new() -> Self {
        Self(Selector::new(17500, 4096, 12))
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
pub type MfdL15ValueCallback<'a> = &'a mut dyn FnMut(MfdL15Value);
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
pub struct MfdL15(Selector);
impl MfdL15 {
    pub const fn new() -> Self {
        Self(Selector::new(17500, 8192, 13))
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
pub type MfdL16ValueCallback<'a> = &'a mut dyn FnMut(MfdL16Value);
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
pub struct MfdL16(Selector);
impl MfdL16 {
    pub const fn new() -> Self {
        Self(Selector::new(17500, 16384, 14))
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
pub type MfdL17ValueCallback<'a> = &'a mut dyn FnMut(MfdL17Value);
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
pub struct MfdL17(Selector);
impl MfdL17 {
    pub const fn new() -> Self {
        Self(Selector::new(17500, 32768, 15))
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
pub type MfdL18ValueCallback<'a> = &'a mut dyn FnMut(MfdL18Value);
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
pub struct MfdL18(Selector);
impl MfdL18 {
    pub const fn new() -> Self {
        Self(Selector::new(17502, 1, 0))
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
pub type MfdL19ValueCallback<'a> = &'a mut dyn FnMut(MfdL19Value);
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
pub struct MfdL19(Selector);
impl MfdL19 {
    pub const fn new() -> Self {
        Self(Selector::new(17502, 2, 1))
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
pub type MfdL2ValueCallback<'a> = &'a mut dyn FnMut(MfdL2Value);
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
pub struct MfdL2(Selector);
impl MfdL2 {
    pub const fn new() -> Self {
        Self(Selector::new(17500, 1, 0))
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
pub type MfdL20ValueCallback<'a> = &'a mut dyn FnMut(MfdL20Value);
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
pub struct MfdL20(Selector);
impl MfdL20 {
    pub const fn new() -> Self {
        Self(Selector::new(17502, 4, 2))
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
pub type MfdL3ValueCallback<'a> = &'a mut dyn FnMut(MfdL3Value);
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
pub struct MfdL3(Selector);
impl MfdL3 {
    pub const fn new() -> Self {
        Self(Selector::new(17500, 2, 1))
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
pub type MfdL4ValueCallback<'a> = &'a mut dyn FnMut(MfdL4Value);
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
pub struct MfdL4(Selector);
impl MfdL4 {
    pub const fn new() -> Self {
        Self(Selector::new(17500, 4, 2))
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
pub type MfdL5ValueCallback<'a> = &'a mut dyn FnMut(MfdL5Value);
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
pub struct MfdL5(Selector);
impl MfdL5 {
    pub const fn new() -> Self {
        Self(Selector::new(17500, 8, 3))
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
pub type MfdL6ValueCallback<'a> = &'a mut dyn FnMut(MfdL6Value);
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
pub struct MfdL6(Selector);
impl MfdL6 {
    pub const fn new() -> Self {
        Self(Selector::new(17500, 16, 4))
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
pub type MfdL7ValueCallback<'a> = &'a mut dyn FnMut(MfdL7Value);
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
pub struct MfdL7(Selector);
impl MfdL7 {
    pub const fn new() -> Self {
        Self(Selector::new(17500, 32, 5))
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
pub type MfdL8ValueCallback<'a> = &'a mut dyn FnMut(MfdL8Value);
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
pub struct MfdL8(Selector);
impl MfdL8 {
    pub const fn new() -> Self {
        Self(Selector::new(17500, 64, 6))
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
pub type MfdL9ValueCallback<'a> = &'a mut dyn FnMut(MfdL9Value);
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
pub struct MfdL9(Selector);
impl MfdL9 {
    pub const fn new() -> Self {
        Self(Selector::new(17500, 128, 7))
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
pub type MfdLBrtSwValueCallback<'a> = &'a mut dyn FnMut(MfdLBrtSwValue);
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
pub struct MfdLBrtSw(Selector);
impl MfdLBrtSw {
    pub const fn new() -> Self {
        Self(Selector::new(17502, 1536, 9))
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
pub type MfdLConSwValueCallback<'a> = &'a mut dyn FnMut(MfdLConSwValue);
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
pub struct MfdLConSw(Selector);
impl MfdLConSw {
    pub const fn new() -> Self {
        Self(Selector::new(17502, 384, 7))
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
pub type MfdLGainSwValueCallback<'a> = &'a mut dyn FnMut(MfdLGainSwValue);
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
pub struct MfdLGainSw(Selector);
impl MfdLGainSw {
    pub const fn new() -> Self {
        Self(Selector::new(17502, 24, 3))
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
pub type MfdLSymSwValueCallback<'a> = &'a mut dyn FnMut(MfdLSymSwValue);
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
pub struct MfdLSymSw(Selector);
impl MfdLSymSw {
    pub const fn new() -> Self {
        Self(Selector::new(17502, 96, 5))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MfdR1Value {
    Select1,
    Select2,
}
pub type MfdR1ValueCallback<'a> = &'a mut dyn FnMut(MfdR1Value);
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
pub struct MfdR1(Selector);
impl MfdR1 {
    pub const fn new() -> Self {
        Self(Selector::new(17502, 2048, 11))
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
pub type MfdR10ValueCallback<'a> = &'a mut dyn FnMut(MfdR10Value);
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
pub struct MfdR10(Selector);
impl MfdR10 {
    pub const fn new() -> Self {
        Self(Selector::new(17504, 16, 4))
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
pub type MfdR11ValueCallback<'a> = &'a mut dyn FnMut(MfdR11Value);
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
pub struct MfdR11(Selector);
impl MfdR11 {
    pub const fn new() -> Self {
        Self(Selector::new(17504, 32, 5))
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
pub type MfdR12ValueCallback<'a> = &'a mut dyn FnMut(MfdR12Value);
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
pub struct MfdR12(Selector);
impl MfdR12 {
    pub const fn new() -> Self {
        Self(Selector::new(17504, 64, 6))
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
pub type MfdR13ValueCallback<'a> = &'a mut dyn FnMut(MfdR13Value);
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
pub struct MfdR13(Selector);
impl MfdR13 {
    pub const fn new() -> Self {
        Self(Selector::new(17504, 128, 7))
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
pub type MfdR14ValueCallback<'a> = &'a mut dyn FnMut(MfdR14Value);
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
pub struct MfdR14(Selector);
impl MfdR14 {
    pub const fn new() -> Self {
        Self(Selector::new(17504, 256, 8))
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
pub type MfdR15ValueCallback<'a> = &'a mut dyn FnMut(MfdR15Value);
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
pub struct MfdR15(Selector);
impl MfdR15 {
    pub const fn new() -> Self {
        Self(Selector::new(17504, 512, 9))
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
pub type MfdR16ValueCallback<'a> = &'a mut dyn FnMut(MfdR16Value);
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
pub struct MfdR16(Selector);
impl MfdR16 {
    pub const fn new() -> Self {
        Self(Selector::new(17504, 1024, 10))
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
pub type MfdR17ValueCallback<'a> = &'a mut dyn FnMut(MfdR17Value);
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
pub struct MfdR17(Selector);
impl MfdR17 {
    pub const fn new() -> Self {
        Self(Selector::new(17504, 2048, 11))
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
pub type MfdR18ValueCallback<'a> = &'a mut dyn FnMut(MfdR18Value);
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
pub struct MfdR18(Selector);
impl MfdR18 {
    pub const fn new() -> Self {
        Self(Selector::new(17504, 4096, 12))
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
pub type MfdR19ValueCallback<'a> = &'a mut dyn FnMut(MfdR19Value);
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
pub struct MfdR19(Selector);
impl MfdR19 {
    pub const fn new() -> Self {
        Self(Selector::new(17504, 8192, 13))
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
pub type MfdR2ValueCallback<'a> = &'a mut dyn FnMut(MfdR2Value);
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
pub struct MfdR2(Selector);
impl MfdR2 {
    pub const fn new() -> Self {
        Self(Selector::new(17502, 4096, 12))
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
pub type MfdR20ValueCallback<'a> = &'a mut dyn FnMut(MfdR20Value);
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
pub struct MfdR20(Selector);
impl MfdR20 {
    pub const fn new() -> Self {
        Self(Selector::new(17504, 16384, 14))
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
pub type MfdR3ValueCallback<'a> = &'a mut dyn FnMut(MfdR3Value);
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
pub struct MfdR3(Selector);
impl MfdR3 {
    pub const fn new() -> Self {
        Self(Selector::new(17502, 8192, 13))
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
pub type MfdR4ValueCallback<'a> = &'a mut dyn FnMut(MfdR4Value);
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
pub struct MfdR4(Selector);
impl MfdR4 {
    pub const fn new() -> Self {
        Self(Selector::new(17502, 16384, 14))
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
pub type MfdR5ValueCallback<'a> = &'a mut dyn FnMut(MfdR5Value);
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
pub struct MfdR5(Selector);
impl MfdR5 {
    pub const fn new() -> Self {
        Self(Selector::new(17502, 32768, 15))
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
pub type MfdR6ValueCallback<'a> = &'a mut dyn FnMut(MfdR6Value);
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
pub struct MfdR6(Selector);
impl MfdR6 {
    pub const fn new() -> Self {
        Self(Selector::new(17504, 1, 0))
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
pub type MfdR7ValueCallback<'a> = &'a mut dyn FnMut(MfdR7Value);
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
pub struct MfdR7(Selector);
impl MfdR7 {
    pub const fn new() -> Self {
        Self(Selector::new(17504, 2, 1))
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
pub type MfdR8ValueCallback<'a> = &'a mut dyn FnMut(MfdR8Value);
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
pub struct MfdR8(Selector);
impl MfdR8 {
    pub const fn new() -> Self {
        Self(Selector::new(17504, 4, 2))
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
pub type MfdR9ValueCallback<'a> = &'a mut dyn FnMut(MfdR9Value);
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
pub struct MfdR9(Selector);
impl MfdR9 {
    pub const fn new() -> Self {
        Self(Selector::new(17504, 8, 3))
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
pub type MfdRBrtSwValueCallback<'a> = &'a mut dyn FnMut(MfdRBrtSwValue);
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
pub struct MfdRBrtSw(Selector);
impl MfdRBrtSw {
    pub const fn new() -> Self {
        Self(Selector::new(17506, 192, 6))
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
pub type MfdRConSwValueCallback<'a> = &'a mut dyn FnMut(MfdRConSwValue);
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
pub struct MfdRConSw(Selector);
impl MfdRConSw {
    pub const fn new() -> Self {
        Self(Selector::new(17506, 48, 4))
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
pub type MfdRGainSwValueCallback<'a> = &'a mut dyn FnMut(MfdRGainSwValue);
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
pub struct MfdRGainSw(Selector);
impl MfdRGainSw {
    pub const fn new() -> Self {
        Self(Selector::new(17506, 3, 0))
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
pub type MfdRSymSwValueCallback<'a> = &'a mut dyn FnMut(MfdRSymSwValue);
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
pub struct MfdRSymSw(Selector);
impl MfdRSymSw {
    pub const fn new() -> Self {
        Self(Selector::new(17506, 12, 2))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MfdSwValue {
    Select1,
    Select2,
}
pub type MfdSwValueCallback<'a> = &'a mut dyn FnMut(MfdSwValue);
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
pub struct MfdSw(Selector);
impl MfdSw {
    pub const fn new() -> Self {
        Self(Selector::new(17442, 2048, 11))
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
pub type MidsLvtKnbValueCallback<'a> = &'a mut dyn FnMut(MidsLvtKnbValue);
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
pub struct MidsLvtKnb(Selector);
impl MidsLvtKnb {
    pub const fn new() -> Self {
        Self(Selector::new(17442, 49152, 14))
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
pub type MmcPwrSwValueCallback<'a> = &'a mut dyn FnMut(MmcPwrSwValue);
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
pub struct MmcPwrSw(Selector);
impl MmcPwrSw {
    pub const fn new() -> Self {
        Self(Selector::new(17442, 512, 9))
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

/// MSL Tone Knob
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct MslKnb(LimitedDial);
impl MslKnb {
    pub const fn new() -> Self {
        Self(LimitedDial::new(17468, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ObogsSwValue {
    Select1,
    Select2,
}
pub type ObogsSwValueCallback<'a> = &'a mut dyn FnMut(ObogsSwValue);
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
pub struct ObogsSw(Selector);
impl ObogsSw {
    pub const fn new() -> Self {
        Self(Selector::new(17442, 8, 3))
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
pub type OxyDiluterLvrValueCallback<'a> = &'a mut dyn FnMut(OxyDiluterLvrValue);
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
pub struct OxyDiluterLvr(Selector);
impl OxyDiluterLvr {
    pub const fn new() -> Self {
        Self(Selector::new(17442, 1, 0))
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
pub struct OxyEmergLvr(Selector);
impl OxyEmergLvr {
    pub const fn new() -> Self {
        Self(Selector::new(17442, 6, 1))
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
pub type OxySupplyLvrValueCallback<'a> = &'a mut dyn FnMut(OxySupplyLvrValue);
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
pub struct OxySupplyLvr(Selector);
impl OxySupplyLvr {
    pub const fn new() -> Self {
        Self(Selector::new(17440, 49152, 14))
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
pub struct OxygenPressure(AnalogGauge);
impl OxygenPressure {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17644, 65535, 0))
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

/// PITCH TRIM Wheel
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct PitchTrim(LimitedDial);
impl PitchTrim {
    pub const fn new() -> Self {
        Self(LimitedDial::new(17412, 65535, 0))
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

/// Pitch Trim Indicator
/// Output: gauge position
pub struct Pitchtrimind(AnalogGauge);
impl Pitchtrimind {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17608, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PlainCipherSwValue {
    Select1,
    Select2,
    Select3,
}
pub type PlainCipherSwValueCallback<'a> = &'a mut dyn FnMut(PlainCipherSwValue);
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
pub struct PlainCipherSw(Selector);
impl PlainCipherSw {
    pub const fn new() -> Self {
        Self(Selector::new(17490, 12, 2))
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
pub enum PosFlashLightSwValue {
    Select1,
    Select2,
}
pub type PosFlashLightSwValueCallback<'a> = &'a mut dyn FnMut(PosFlashLightSwValue);
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
pub struct PosFlashLightSw(Selector);
impl PosFlashLightSw {
    pub const fn new() -> Self {
        Self(Selector::new(17408, 16384, 14))
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
pub type PosFuselageLightSwValueCallback<'a> = &'a mut dyn FnMut(PosFuselageLightSwValue);
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
pub struct PosFuselageLightSw(Selector);
impl PosFuselageLightSw {
    pub const fn new() -> Self {
        Self(Selector::new(17416, 96, 5))
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
pub type PosWingTailLightSwValueCallback<'a> = &'a mut dyn FnMut(PosWingTailLightSwValue);
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
pub struct PosWingTailLightSw(Selector);
impl PosWingTailLightSw {
    pub const fn new() -> Self {
        Self(Selector::new(17416, 24, 3))
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

/// PRIMARY CONSOLES BRT Knob
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct PriConsolesBrtKnb(LimitedDial);
impl PriConsolesBrtKnb {
    pub const fn new() -> Self {
        Self(LimitedDial::new(17422, 65535, 0))
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
pub struct PriDataDisplayBrtKnb(LimitedDial);
impl PriDataDisplayBrtKnb {
    pub const fn new() -> Self {
        Self(LimitedDial::new(17426, 65535, 0))
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
pub struct PriInstPnlBrtKnb(LimitedDial);
impl PriInstPnlBrtKnb {
    pub const fn new() -> Self {
        Self(LimitedDial::new(17424, 65535, 0))
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

/// PROBE HEAT Switch, PROBE HEAT/OFF/TEST
/// Input: set the switch position
/// Output: switch position -- 0 = Down, 1 = Mid ,  2 = UP
pub struct ProbeHeatSw(Selector);
impl ProbeHeatSw {
    pub const fn new() -> Self {
        Self(Selector::new(17436, 24, 3))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RdrAltPwrSwValue {
    Select1,
    Select2,
    Select3,
}
pub type RdrAltPwrSwValueCallback<'a> = &'a mut dyn FnMut(RdrAltPwrSwValue);
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
pub struct RdrAltPwrSw(Selector);
impl RdrAltPwrSw {
    pub const fn new() -> Self {
        Self(Selector::new(17442, 384, 7))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RfSwValue {
    Select1,
    Select2,
    Select3,
}
pub type RfSwValueCallback<'a> = &'a mut dyn FnMut(RfSwValue);
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
pub struct RfSw(Selector);
impl RfSw {
    pub const fn new() -> Self {
        Self(Selector::new(17456, 12288, 12))
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

/// ROLL TRIM Wheel
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct RollTrim(LimitedDial);
impl RollTrim {
    pub const fn new() -> Self {
        Self(LimitedDial::new(17410, 65535, 0))
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

/// Roll Trim Indicator
/// Output: gauge position
pub struct Rolltrimind(AnalogGauge);
impl Rolltrimind {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17606, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RwrActPwrBtnValue {
    Select1,
    Select2,
}
pub type RwrActPwrBtnValueCallback<'a> = &'a mut dyn FnMut(RwrActPwrBtnValue);
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
pub struct RwrActPwrBtn(Selector);
impl RwrActPwrBtn {
    pub const fn new() -> Self {
        Self(Selector::new(17490, 8192, 13))
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
pub type RwrAltBtnValueCallback<'a> = &'a mut dyn FnMut(RwrAltBtnValue);
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
pub struct RwrAltBtn(Selector);
impl RwrAltBtn {
    pub const fn new() -> Self {
        Self(Selector::new(17490, 16384, 14))
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
pub type RwrHandoffBtnValueCallback<'a> = &'a mut dyn FnMut(RwrHandoffBtnValue);
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
pub struct RwrHandoffBtn(Selector);
impl RwrHandoffBtn {
    pub const fn new() -> Self {
        Self(Selector::new(17490, 64, 6))
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
pub struct RwrIndDimKnb(LimitedDial);
impl RwrIndDimKnb {
    pub const fn new() -> Self {
        Self(LimitedDial::new(17496, 65535, 0))
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
pub struct RwrIntensKnb(LimitedDial);
impl RwrIntensKnb {
    pub const fn new() -> Self {
        Self(LimitedDial::new(17494, 65535, 0))
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
pub type RwrLaunchBtnValueCallback<'a> = &'a mut dyn FnMut(RwrLaunchBtnValue);
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
pub struct RwrLaunchBtn(Selector);
impl RwrLaunchBtn {
    pub const fn new() -> Self {
        Self(Selector::new(17490, 128, 7))
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
pub type RwrModeBtnValueCallback<'a> = &'a mut dyn FnMut(RwrModeBtnValue);
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
pub struct RwrModeBtn(Selector);
impl RwrModeBtn {
    pub const fn new() -> Self {
        Self(Selector::new(17490, 256, 8))
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
pub type RwrPwrBtnValueCallback<'a> = &'a mut dyn FnMut(RwrPwrBtnValue);
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
pub struct RwrPwrBtn(Selector);
impl RwrPwrBtn {
    pub const fn new() -> Self {
        Self(Selector::new(17490, 32768, 15))
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
pub type RwrSearchBtnValueCallback<'a> = &'a mut dyn FnMut(RwrSearchBtnValue);
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
pub struct RwrSearchBtn(Selector);
impl RwrSearchBtn {
    pub const fn new() -> Self {
        Self(Selector::new(17490, 4096, 12))
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
pub type RwrSysTestBtnValueCallback<'a> = &'a mut dyn FnMut(RwrSysTestBtnValue);
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
pub struct RwrSysTestBtn(Selector);
impl RwrSysTestBtn {
    pub const fn new() -> Self {
        Self(Selector::new(17490, 1024, 10))
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
pub type RwrTBtnValueCallback<'a> = &'a mut dyn FnMut(RwrTBtnValue);
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
pub struct RwrTBtn(Selector);
impl RwrTBtn {
    pub const fn new() -> Self {
        Self(Selector::new(17490, 2048, 11))
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
pub type RwrUnknownShipBtnValueCallback<'a> = &'a mut dyn FnMut(RwrUnknownShipBtnValue);
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
pub struct RwrUnknownShipBtn(Selector);
impl RwrUnknownShipBtn {
    pub const fn new() -> Self {
        Self(Selector::new(17490, 512, 9))
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

/// SAI Aircraft Reference Symbol
/// Output: gauge position
pub struct SaiAircraftreferencesymbol(AnalogGauge);
impl SaiAircraftreferencesymbol {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17578, 65535, 0))
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
pub struct SaiBank(AnalogGauge);
impl SaiBank {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17574, 65535, 0))
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
pub struct SaiBankArrow(AnalogGauge);
impl SaiBankArrow {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17840, 65535, 0))
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
pub type SaiCageValueCallback<'a> = &'a mut dyn FnMut(SaiCageValue);
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
pub struct SaiCage(Selector);
impl SaiCage {
    pub const fn new() -> Self {
        Self(Selector::new(17504, 32768, 15))
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
pub struct SaiKnbArrow(AnalogGauge);
impl SaiKnbArrow {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17580, 65535, 0))
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
pub struct SaiOffFlag(AnalogGauge);
impl SaiOffFlag {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17576, 65535, 0))
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
pub struct SaiPitch(AnalogGauge);
impl SaiPitch {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17572, 65535, 0))
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
pub struct SaiPitchTrim(AnalogDial);
impl SaiPitchTrim {
    pub const fn new() -> Self {
        Self(AnalogDial::new(17512, 65535, 0))
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

/// SEAT ADJ Switch, UP/OFF/DOWN
/// Input: set the switch position
/// Output: switch position -- 0 = Down, 1 = Mid ,  2 = UP
pub struct SeatAdj(Selector);
impl SeatAdj {
    pub const fn new() -> Self {
        Self(Selector::new(17522, 3, 0))
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
pub type SeatEjectHandleValueCallback<'a> = &'a mut dyn FnMut(SeatEjectHandleValue);
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
pub struct SeatEjectHandle(Selector);
impl SeatEjectHandle {
    pub const fn new() -> Self {
        Self(Selector::new(17538, 1024, 10))
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
pub type SeatEjectSafeValueCallback<'a> = &'a mut dyn FnMut(SeatEjectSafeValue);
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
pub struct SeatEjectSafe(Selector);
impl SeatEjectSafe {
    pub const fn new() -> Self {
        Self(Selector::new(17522, 8, 3))
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
pub struct SeatHeight(AnalogGauge);
impl SeatHeight {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17542, 65535, 0))
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

/// SECURE VOICE Knob
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct SecVoiceKnb(LimitedDial);
impl SecVoiceKnb {
    pub const fn new() -> Self {
        Self(LimitedDial::new(17466, 65535, 0))
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

/// Set Airspeed
/// Output: gauge position
pub struct SetAirspeed(AnalogGauge);
impl SetAirspeed {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17568, 65535, 0))
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

/// Speed Brake Indicator
/// Output: gauge position
pub struct SpeedbrakeIndicator(AnalogGauge);
impl SpeedbrakeIndicator {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17610, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StStaSwValue {
    Select1,
    Select2,
}
pub type StStaSwValueCallback<'a> = &'a mut dyn FnMut(StStaSwValue);
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
pub struct StStaSw(Selector);
impl StStaSw {
    pub const fn new() -> Self {
        Self(Selector::new(17442, 1024, 10))
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
pub enum StoresConfigSwValue {
    Select1,
    Select2,
}
pub type StoresConfigSwValueCallback<'a> = &'a mut dyn FnMut(StoresConfigSwValue);
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
pub struct StoresConfigSw(Selector);
impl StoresConfigSw {
    pub const fn new() -> Self {
        Self(Selector::new(17408, 128, 7))
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

/// System A Pressure
/// Output: gauge position
pub struct SysaPressure(AnalogGauge);
impl SysaPressure {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17612, 65535, 0))
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
pub struct SysbPressure(AnalogGauge);
impl SysbPressure {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17614, 65535, 0))
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

/// TACAN Knob
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct TacanKnb(LimitedDial);
impl TacanKnb {
    pub const fn new() -> Self {
        Self(LimitedDial::new(17476, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TankIntertingSwValue {
    Select1,
    Select2,
}
pub type TankIntertingSwValueCallback<'a> = &'a mut dyn FnMut(TankIntertingSwValue);
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
pub struct TankIntertingSw(Selector);
impl TankIntertingSw {
    pub const fn new() -> Self {
        Self(Selector::new(17436, 512, 9))
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
pub type TempKnbValueCallback<'a> = &'a mut dyn FnMut(TempKnbValue);
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
pub struct TempKnb(Selector);
impl TempKnb {
    pub const fn new() -> Self {
        Self(Selector::new(17438, 3584, 9))
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

/// TF Tone Knob Knob
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct TfKnb(LimitedDial);
impl TfKnb {
    pub const fn new() -> Self {
        Self(LimitedDial::new(17470, 65535, 0))
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
pub struct ThreatKnb(LimitedDial);
impl ThreatKnb {
    pub const fn new() -> Self {
        Self(LimitedDial::new(17472, 65535, 0))
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
pub enum TrimApDiscSwValue {
    Select1,
    Select2,
}
pub type TrimApDiscSwValueCallback<'a> = &'a mut dyn FnMut(TrimApDiscSwValue);
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
pub struct TrimApDiscSw(Selector);
impl TrimApDiscSw {
    pub const fn new() -> Self {
        Self(Selector::new(17408, 32, 5))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UfcSwValue {
    Select1,
    Select2,
}
pub type UfcSwValueCallback<'a> = &'a mut dyn FnMut(UfcSwValue);
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
pub struct UfcSw(Selector);
impl UfcSw {
    pub const fn new() -> Self {
        Self(Selector::new(17442, 4096, 12))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UhfAntSelSwValue {
    Select1,
    Select2,
    Select3,
}
pub type UhfAntSelSwValueCallback<'a> = &'a mut dyn FnMut(UhfAntSelSwValue);
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
pub struct UhfAntSelSw(Selector);
impl UhfAntSelSw {
    pub const fn new() -> Self {
        Self(Selector::new(17460, 384, 7))
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

/// UHF CHAN Display
/// Output: UHF CHAN Display
pub struct UhfChanDisp(Display<2>);
impl UhfChanDisp {
    pub const fn new() -> Self {
        Self(Display::<2>::new(17830, 2))
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
pub struct UhfChanKnb(Selector);
impl UhfChanKnb {
    pub const fn new() -> Self {
        Self(Selector::new(17460, 15872, 9))
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
pub type UhfDoorValueCallback<'a> = &'a mut dyn FnMut(UhfDoorValue);
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
pub struct UhfDoor(Selector);
impl UhfDoor {
    pub const fn new() -> Self {
        Self(Selector::new(17482, 64, 6))
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
pub type UhfFreq0025KnbValueCallback<'a> = &'a mut dyn FnMut(UhfFreq0025KnbValue);
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
pub struct UhfFreq0025Knb(Selector);
impl UhfFreq0025Knb {
    pub const fn new() -> Self {
        Self(Selector::new(17480, 12288, 12))
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
pub type UhfFreq01KnbValueCallback<'a> = &'a mut dyn FnMut(UhfFreq01KnbValue);
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
pub struct UhfFreq01Knb(Selector);
impl UhfFreq01Knb {
    pub const fn new() -> Self {
        Self(Selector::new(17480, 3840, 8))
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
pub type UhfFreq100KnbValueCallback<'a> = &'a mut dyn FnMut(UhfFreq100KnbValue);
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
pub struct UhfFreq100Knb(Selector);
impl UhfFreq100Knb {
    pub const fn new() -> Self {
        Self(Selector::new(17460, 49152, 14))
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
pub type UhfFreq10KnbValueCallback<'a> = &'a mut dyn FnMut(UhfFreq10KnbValue);
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
pub struct UhfFreq10Knb(Selector);
impl UhfFreq10Knb {
    pub const fn new() -> Self {
        Self(Selector::new(17480, 15, 0))
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
pub type UhfFreq1KnbValueCallback<'a> = &'a mut dyn FnMut(UhfFreq1KnbValue);
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
pub struct UhfFreq1Knb(Selector);
impl UhfFreq1Knb {
    pub const fn new() -> Self {
        Self(Selector::new(17480, 240, 4))
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
pub struct UhfFreqDisp(Display<7>);
impl UhfFreqDisp {
    pub const fn new() -> Self {
        Self(Display::<7>::new(17832, 7))
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
pub type UhfFuncKnbValueCallback<'a> = &'a mut dyn FnMut(UhfFuncKnbValue);
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
pub struct UhfFuncKnb(Selector);
impl UhfFuncKnb {
    pub const fn new() -> Self {
        Self(Selector::new(17480, 49152, 14))
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
pub type UhfModeKnbValueCallback<'a> = &'a mut dyn FnMut(UhfModeKnbValue);
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
pub struct UhfModeKnb(Selector);
impl UhfModeKnb {
    pub const fn new() -> Self {
        Self(Selector::new(17482, 3, 0))
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
pub type UhfSquelchSwValueCallback<'a> = &'a mut dyn FnMut(UhfSquelchSwValue);
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
pub struct UhfSquelchSw(Selector);
impl UhfSquelchSw {
    pub const fn new() -> Self {
        Self(Selector::new(17482, 8, 3))
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
pub type UhfStatusBtnValueCallback<'a> = &'a mut dyn FnMut(UhfStatusBtnValue);
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
pub struct UhfStatusBtn(Selector);
impl UhfStatusBtn {
    pub const fn new() -> Self {
        Self(Selector::new(17482, 32, 5))
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
pub type UhfTestBtnValueCallback<'a> = &'a mut dyn FnMut(UhfTestBtnValue);
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
pub struct UhfTestBtn(Selector);
impl UhfTestBtn {
    pub const fn new() -> Self {
        Self(Selector::new(17482, 16, 4))
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
pub type UhfToneBtnValueCallback<'a> = &'a mut dyn FnMut(UhfToneBtnValue);
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
pub struct UhfToneBtn(Selector);
impl UhfToneBtn {
    pub const fn new() -> Self {
        Self(Selector::new(17482, 4, 2))
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
pub struct UhfVolKnb(LimitedDial);
impl UhfVolKnb {
    pub const fn new() -> Self {
        Self(LimitedDial::new(17484, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VmsInhibitSwValue {
    Select1,
    Select2,
}
pub type VmsInhibitSwValueCallback<'a> = &'a mut dyn FnMut(VmsInhibitSwValue);
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
pub struct VmsInhibitSw(Selector);
impl VmsInhibitSw {
    pub const fn new() -> Self {
        Self(Selector::new(17460, 16, 4))
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

/// VVI
/// Output: gauge position
pub struct Vvi(AnalogGauge);
impl Vvi {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(17582, 65535, 0))
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

/// YAW TRIM Knob
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct YawTrim(LimitedDial);
impl YawTrim {
    pub const fn new() -> Self {
        Self(LimitedDial::new(17414, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ZeroizeSwValue {
    Select1,
    Select2,
    Select3,
}
pub type ZeroizeSwValueCallback<'a> = &'a mut dyn FnMut(ZeroizeSwValue);
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
pub struct ZeroizeSw(Selector);
impl ZeroizeSw {
    pub const fn new() -> Self {
        Self(Selector::new(17490, 48, 4))
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
pub type ZeroizeSwCoverValueCallback<'a> = &'a mut dyn FnMut(ZeroizeSwCoverValue);
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
pub struct ZeroizeSwCover(Selector);
impl ZeroizeSwCover {
    pub const fn new() -> Self {
        Self(Selector::new(17486, 32768, 15))
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

pub enum RegisteredControl<'a> {
    AbResetSw {
        control: AbResetSw,
        callback: IntegerCallback<'a>,
    },
    AdiAuxFlag {
        control: AdiAuxFlag,
        callback: IntegerCallback<'a>,
    },
    AdiBank {
        control: AdiBank,
        callback: IntegerCallback<'a>,
    },
    AdiBubble {
        control: AdiBubble,
        callback: IntegerCallback<'a>,
    },
    AdiGsBar {
        control: AdiGsBar,
        callback: IntegerCallback<'a>,
    },
    AdiGsFlag {
        control: AdiGsFlag,
        callback: IntegerCallback<'a>,
    },
    AdiGsPointer {
        control: AdiGsPointer,
        callback: IntegerCallback<'a>,
    },
    AdiLocBar {
        control: AdiLocBar,
        callback: IntegerCallback<'a>,
    },
    AdiLocFlag {
        control: AdiLocFlag,
        callback: IntegerCallback<'a>,
    },
    AdiOffFlag {
        control: AdiOffFlag,
        callback: IntegerCallback<'a>,
    },
    AdiPitch {
        control: AdiPitch,
        callback: IntegerCallback<'a>,
    },
    AdiPitchTrim {
        control: AdiPitchTrim,
        callback: IntegerCallback<'a>,
    },
    AdiTurnrate {
        control: AdiTurnrate,
        callback: IntegerCallback<'a>,
    },
    AdvModeSw {
        control: AdvModeSw,
        callback: AdvModeSwValueCallback<'a>,
    },
    AirRefuelLightKnb {
        control: AirRefuelLightKnb,
        callback: IntegerCallback<'a>,
    },
    AirRefuelSw {
        control: AirRefuelSw,
        callback: AirRefuelSwValueCallback<'a>,
    },
    AirSourceKnb {
        control: AirSourceKnb,
        callback: AirSourceKnbValueCallback<'a>,
    },
    Airspeed {
        control: Airspeed,
        callback: IntegerCallback<'a>,
    },
    AirspeedSetKnb {
        control: AirspeedSetKnb,
        callback: IntegerCallback<'a>,
    },
    Alt10000FtCnt {
        control: Alt10000FtCnt,
        callback: IntegerCallback<'a>,
    },
    Alt1000FtCnt {
        control: Alt1000FtCnt,
        callback: IntegerCallback<'a>,
    },
    Alt100FtCnt {
        control: Alt100FtCnt,
        callback: IntegerCallback<'a>,
    },
    Alt100FtPtr {
        control: Alt100FtPtr,
        callback: IntegerCallback<'a>,
    },
    AltBaroSetKnb {
        control: AltBaroSetKnb,
        callback: IntegerCallback<'a>,
    },
    AltFlapsSw {
        control: AltFlapsSw,
        callback: AltFlapsSwValueCallback<'a>,
    },
    AltModeLv {
        control: AltModeLv,
        callback: IntegerCallback<'a>,
    },
    AltPneuFlag {
        control: AltPneuFlag,
        callback: IntegerCallback<'a>,
    },
    AltPressureDrum0Cnt {
        control: AltPressureDrum0Cnt,
        callback: IntegerCallback<'a>,
    },
    AltPressureDrum1Cnt {
        control: AltPressureDrum1Cnt,
        callback: IntegerCallback<'a>,
    },
    AltPressureDrum2Cnt {
        control: AltPressureDrum2Cnt,
        callback: IntegerCallback<'a>,
    },
    AltPressureDrum3Cnt {
        control: AltPressureDrum3Cnt,
        callback: IntegerCallback<'a>,
    },
    AltRelBtn {
        control: AltRelBtn,
        callback: AltRelBtnValueCallback<'a>,
    },
    AntiCollLightKnb {
        control: AntiCollLightKnb,
        callback: AntiCollLightKnbValueCallback<'a>,
    },
    AntiSkidSw {
        control: AntiSkidSw,
        callback: IntegerCallback<'a>,
    },
    AoaIndexBrtKnb {
        control: AoaIndexBrtKnb,
        callback: IntegerCallback<'a>,
    },
    AoaValue {
        control: AoaValue,
        callback: IntegerCallback<'a>,
    },
    ApPitchSw {
        control: ApPitchSw,
        callback: IntegerCallback<'a>,
    },
    ApRollSw {
        control: ApRollSw,
        callback: ApRollSwValueCallback<'a>,
    },
    ArStatusBrtKnb {
        control: ArStatusBrtKnb,
        callback: IntegerCallback<'a>,
    },
    BitSw {
        control: BitSw,
        callback: BitSwValueCallback<'a>,
    },
    BrakeChanSw {
        control: BrakeChanSw,
        callback: BrakeChanSwValueCallback<'a>,
    },
    CanopyHandle {
        control: CanopyHandle,
        callback: CanopyHandleValueCallback<'a>,
    },
    CanopyJettThandle {
        control: CanopyJettThandle,
        callback: CanopyJettThandleValueCallback<'a>,
    },
    CanopySw {
        control: CanopySw,
        callback: IntegerCallback<'a>,
    },
    CanopyValue {
        control: CanopyValue,
        callback: IntegerCallback<'a>,
    },
    ClockCurrtimeH {
        control: ClockCurrtimeH,
        callback: IntegerCallback<'a>,
    },
    ClockCurrtimeMs {
        control: ClockCurrtimeMs,
        callback: IntegerCallback<'a>,
    },
    ClockElapsed {
        control: ClockElapsed,
        callback: ClockElapsedValueCallback<'a>,
    },
    ClockElapsedTimeM {
        control: ClockElapsedTimeM,
        callback: IntegerCallback<'a>,
    },
    ClockElapsedTimeSec {
        control: ClockElapsedTimeSec,
        callback: IntegerCallback<'a>,
    },
    ClockSet {
        control: ClockSet,
        callback: ClockSetValueCallback<'a>,
    },
    ClockWind {
        control: ClockWind,
        callback: IntegerCallback<'a>,
    },
    Cmds01ExpCatSw {
        control: Cmds01ExpCatSw,
        callback: Cmds01ExpCatSwValueCallback<'a>,
    },
    Cmds02ExpCatSw {
        control: Cmds02ExpCatSw,
        callback: Cmds02ExpCatSwValueCallback<'a>,
    },
    CmdsChAmount {
        control: CmdsChAmount,
        callback: DisplayCallback<'a>,
    },
    CmdsChExpCatSw {
        control: CmdsChExpCatSw,
        callback: CmdsChExpCatSwValueCallback<'a>,
    },
    CmdsDispenseBtn {
        control: CmdsDispenseBtn,
        callback: CmdsDispenseBtnValueCallback<'a>,
    },
    CmdsFlAmount {
        control: CmdsFlAmount,
        callback: DisplayCallback<'a>,
    },
    CmdsFlExpCatSw {
        control: CmdsFlExpCatSw,
        callback: CmdsFlExpCatSwValueCallback<'a>,
    },
    CmdsJettSw {
        control: CmdsJettSw,
        callback: CmdsJettSwValueCallback<'a>,
    },
    CmdsJmrSourcheSw {
        control: CmdsJmrSourcheSw,
        callback: CmdsJmrSourcheSwValueCallback<'a>,
    },
    CmdsModeKnb {
        control: CmdsModeKnb,
        callback: CmdsModeKnbValueCallback<'a>,
    },
    CmdsMwsSourcheSw {
        control: CmdsMwsSourcheSw,
        callback: CmdsMwsSourcheSwValueCallback<'a>,
    },
    CmdsO1Amount {
        control: CmdsO1Amount,
        callback: DisplayCallback<'a>,
    },
    CmdsO2Amount {
        control: CmdsO2Amount,
        callback: DisplayCallback<'a>,
    },
    CmdsProgKnb {
        control: CmdsProgKnb,
        callback: CmdsProgKnbValueCallback<'a>,
    },
    CmdsPwrSourcheSw {
        control: CmdsPwrSourcheSw,
        callback: CmdsPwrSourcheSwValueCallback<'a>,
    },
    CockpitAlititude {
        control: CockpitAlititude,
        callback: IntegerCallback<'a>,
    },
    Comm1ModeKnb {
        control: Comm1ModeKnb,
        callback: Comm1ModeKnbValueCallback<'a>,
    },
    Comm1PwrKnb {
        control: Comm1PwrKnb,
        callback: IntegerCallback<'a>,
    },
    Comm2ModeKnb {
        control: Comm2ModeKnb,
        callback: Comm2ModeKnbValueCallback<'a>,
    },
    Comm2PwrKnb {
        control: Comm2PwrKnb,
        callback: IntegerCallback<'a>,
    },
    DedLine1 {
        control: DedLine1,
        callback: DisplayCallback<'a>,
    },
    DedLine2 {
        control: DedLine2,
        callback: DisplayCallback<'a>,
    },
    DedLine3 {
        control: DedLine3,
        callback: DisplayCallback<'a>,
    },
    DedLine4 {
        control: DedLine4,
        callback: DisplayCallback<'a>,
    },
    DedLine5 {
        control: DedLine5,
        callback: DisplayCallback<'a>,
    },
    DefogLever {
        control: DefogLever,
        callback: DefogLeverValueCallback<'a>,
    },
    DigiBakSw {
        control: DigiBakSw,
        callback: DigiBakSwValueCallback<'a>,
    },
    DlSw {
        control: DlSw,
        callback: DlSwValueCallback<'a>,
    },
    DnLockBtn {
        control: DnLockBtn,
        callback: DnLockBtnValueCallback<'a>,
    },
    Ecm1Btn {
        control: Ecm1Btn,
        callback: Ecm1BtnValueCallback<'a>,
    },
    Ecm2Btn {
        control: Ecm2Btn,
        callback: Ecm2BtnValueCallback<'a>,
    },
    Ecm3Btn {
        control: Ecm3Btn,
        callback: Ecm3BtnValueCallback<'a>,
    },
    Ecm4Btn {
        control: Ecm4Btn,
        callback: Ecm4BtnValueCallback<'a>,
    },
    Ecm5Btn {
        control: Ecm5Btn,
        callback: Ecm5BtnValueCallback<'a>,
    },
    Ecm6Btn {
        control: Ecm6Btn,
        callback: Ecm6BtnValueCallback<'a>,
    },
    EcmBitBtn {
        control: EcmBitBtn,
        callback: EcmBitBtnValueCallback<'a>,
    },
    EcmDimKnb {
        control: EcmDimKnb,
        callback: IntegerCallback<'a>,
    },
    EcmFrmBtn {
        control: EcmFrmBtn,
        callback: EcmFrmBtnValueCallback<'a>,
    },
    EcmPwSw {
        control: EcmPwSw,
        callback: EcmPwSwValueCallback<'a>,
    },
    EcmResetBtn {
        control: EcmResetBtn,
        callback: EcmResetBtnValueCallback<'a>,
    },
    EcmSplBtn {
        control: EcmSplBtn,
        callback: EcmSplBtnValueCallback<'a>,
    },
    EcmXmitSw {
        control: EcmXmitSw,
        callback: EcmXmitSwValueCallback<'a>,
    },
    EhsiCrsSet {
        control: EhsiCrsSet,
        callback: EhsiCrsSetValueCallback<'a>,
    },
    EhsiCrsSetKnb {
        control: EhsiCrsSetKnb,
        callback: IntegerCallback<'a>,
    },
    EhsiHdgSetBtn {
        control: EhsiHdgSetBtn,
        callback: EhsiHdgSetBtnValueCallback<'a>,
    },
    EhsiHdgSetKnb {
        control: EhsiHdgSetKnb,
        callback: IntegerCallback<'a>,
    },
    EhsiMode {
        control: EhsiMode,
        callback: EhsiModeValueCallback<'a>,
    },
    ElecCaution {
        control: ElecCaution,
        callback: ElecCautionValueCallback<'a>,
    },
    EmergStroreJett {
        control: EmergStroreJett,
        callback: EmergStroreJettValueCallback<'a>,
    },
    EngAntiIce {
        control: EngAntiIce,
        callback: EngAntiIceValueCallback<'a>,
    },
    EngContSw {
        control: EngContSw,
        callback: EngContSwValueCallback<'a>,
    },
    EngContSwCover {
        control: EngContSwCover,
        callback: EngContSwCoverValueCallback<'a>,
    },
    EngineFeedKnb {
        control: EngineFeedKnb,
        callback: EngineFeedKnbValueCallback<'a>,
    },
    EngineFtit {
        control: EngineFtit,
        callback: IntegerCallback<'a>,
    },
    EngineNozzlePosition {
        control: EngineNozzlePosition,
        callback: IntegerCallback<'a>,
    },
    EngineOilPressure {
        control: EngineOilPressure,
        callback: IntegerCallback<'a>,
    },
    EngineTachometer {
        control: EngineTachometer,
        callback: IntegerCallback<'a>,
    },
    EpuGenTestSw {
        control: EpuGenTestSw,
        callback: EpuGenTestSwValueCallback<'a>,
    },
    EpuSw {
        control: EpuSw,
        callback: EpuSwValueCallback<'a>,
    },
    EpuSwCoverOff {
        control: EpuSwCoverOff,
        callback: EpuSwCoverOffValueCallback<'a>,
    },
    EpuSwCoverOn {
        control: EpuSwCoverOn,
        callback: EpuSwCoverOnValueCallback<'a>,
    },
    ExtFormationLights {
        control: ExtFormationLights,
        callback: IntegerCallback<'a>,
    },
    ExtFuelTransSw {
        control: ExtFuelTransSw,
        callback: ExtFuelTransSwValueCallback<'a>,
    },
    ExtPositionLightFuselage {
        control: ExtPositionLightFuselage,
        callback: IntegerCallback<'a>,
    },
    ExtPositionLightsWing {
        control: ExtPositionLightsWing,
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
    ExtStrobeTail {
        control: ExtStrobeTail,
        callback: IntegerCallback<'a>,
    },
    ExtTailLight {
        control: ExtTailLight,
        callback: IntegerCallback<'a>,
    },
    ExtWowLeft {
        control: ExtWowLeft,
        callback: IntegerCallback<'a>,
    },
    ExtWowNose {
        control: ExtWowNose,
        callback: IntegerCallback<'a>,
    },
    ExtWowRight {
        control: ExtWowRight,
        callback: IntegerCallback<'a>,
    },
    FAckBtn {
        control: FAckBtn,
        callback: FAckBtnValueCallback<'a>,
    },
    FcrPwrSw {
        control: FcrPwrSw,
        callback: FcrPwrSwValueCallback<'a>,
    },
    FireOheatDetectBtn {
        control: FireOheatDetectBtn,
        callback: FireOheatDetectBtnValueCallback<'a>,
    },
    FlcsPwrTestSw {
        control: FlcsPwrTestSw,
        callback: IntegerCallback<'a>,
    },
    FlcsResetSw {
        control: FlcsResetSw,
        callback: FlcsResetSwValueCallback<'a>,
    },
    FloodConsolesBrtKnb {
        control: FloodConsolesBrtKnb,
        callback: IntegerCallback<'a>,
    },
    FloodInstPnlBrtKnb {
        control: FloodInstPnlBrtKnb,
        callback: IntegerCallback<'a>,
    },
    FlowIndicator {
        control: FlowIndicator,
        callback: IntegerCallback<'a>,
    },
    FlowIndicatorLight {
        control: FlowIndicatorLight,
        callback: LedCallback<'a>,
    },
    FormLightKnb {
        control: FormLightKnb,
        callback: IntegerCallback<'a>,
    },
    FuelAl {
        control: FuelAl,
        callback: IntegerCallback<'a>,
    },
    FuelFr {
        control: FuelFr,
        callback: IntegerCallback<'a>,
    },
    FuelMasterCv {
        control: FuelMasterCv,
        callback: FuelMasterCvValueCallback<'a>,
    },
    FuelMasterSw {
        control: FuelMasterSw,
        callback: FuelMasterSwValueCallback<'a>,
    },
    FuelQtySelKnb {
        control: FuelQtySelKnb,
        callback: FuelQtySelKnbValueCallback<'a>,
    },
    FuelQtySelTKnb {
        control: FuelQtySelTKnb,
        callback: IntegerCallback<'a>,
    },
    Fuelflowcounter100 {
        control: Fuelflowcounter100,
        callback: IntegerCallback<'a>,
    },
    Fuelflowcounter10k {
        control: Fuelflowcounter10k,
        callback: IntegerCallback<'a>,
    },
    Fuelflowcounter1k {
        control: Fuelflowcounter1k,
        callback: IntegerCallback<'a>,
    },
    Fueltotalizer100 {
        control: Fueltotalizer100,
        callback: IntegerCallback<'a>,
    },
    Fueltotalizer10k {
        control: Fueltotalizer10k,
        callback: IntegerCallback<'a>,
    },
    Fueltotalizer1k {
        control: Fueltotalizer1k,
        callback: IntegerCallback<'a>,
    },
    GearHandle {
        control: GearHandle,
        callback: GearHandleValueCallback<'a>,
    },
    GndJettEnableSw {
        control: GndJettEnableSw,
        callback: GndJettEnableSwValueCallback<'a>,
    },
    GpsSw {
        control: GpsSw,
        callback: GpsSwValueCallback<'a>,
    },
    HdptSwL {
        control: HdptSwL,
        callback: HdptSwLValueCallback<'a>,
    },
    HdptSwR {
        control: HdptSwR,
        callback: HdptSwRValueCallback<'a>,
    },
    HideStick {
        control: HideStick,
        callback: HideStickValueCallback<'a>,
    },
    HmcsIntKnb {
        control: HmcsIntKnb,
        callback: IntegerCallback<'a>,
    },
    HookSw {
        control: HookSw,
        callback: HookSwValueCallback<'a>,
    },
    HornSilenceBtn {
        control: HornSilenceBtn,
        callback: HornSilenceBtnValueCallback<'a>,
    },
    HotMicSw {
        control: HotMicSw,
        callback: HotMicSwValueCallback<'a>,
    },
    HudAltSw {
        control: HudAltSw,
        callback: HudAltSwValueCallback<'a>,
    },
    HudBrtSw {
        control: HudBrtSw,
        callback: HudBrtSwValueCallback<'a>,
    },
    HudDedDataSw {
        control: HudDedDataSw,
        callback: HudDedDataSwValueCallback<'a>,
    },
    HudDepressRetSw {
        control: HudDepressRetSw,
        callback: HudDepressRetSwValueCallback<'a>,
    },
    HudFpMarkerSw {
        control: HudFpMarkerSw,
        callback: HudFpMarkerSwValueCallback<'a>,
    },
    HudScalesSw {
        control: HudScalesSw,
        callback: HudScalesSwValueCallback<'a>,
    },
    HudSpeedSw {
        control: HudSpeedSw,
        callback: HudSpeedSwValueCallback<'a>,
    },
    HudTestSw {
        control: HudTestSw,
        callback: HudTestSwValueCallback<'a>,
    },
    HydrazinVolume {
        control: HydrazinVolume,
        callback: IntegerCallback<'a>,
    },
    IcpAaModeBtn {
        control: IcpAaModeBtn,
        callback: IcpAaModeBtnValueCallback<'a>,
    },
    IcpAgModeBtn {
        control: IcpAgModeBtn,
        callback: IcpAgModeBtnValueCallback<'a>,
    },
    IcpBtn0 {
        control: IcpBtn0,
        callback: IcpBtn0ValueCallback<'a>,
    },
    IcpBtn1 {
        control: IcpBtn1,
        callback: IcpBtn1ValueCallback<'a>,
    },
    IcpBtn2 {
        control: IcpBtn2,
        callback: IcpBtn2ValueCallback<'a>,
    },
    IcpBtn3 {
        control: IcpBtn3,
        callback: IcpBtn3ValueCallback<'a>,
    },
    IcpBtn4 {
        control: IcpBtn4,
        callback: IcpBtn4ValueCallback<'a>,
    },
    IcpBtn5 {
        control: IcpBtn5,
        callback: IcpBtn5ValueCallback<'a>,
    },
    IcpBtn6 {
        control: IcpBtn6,
        callback: IcpBtn6ValueCallback<'a>,
    },
    IcpBtn7 {
        control: IcpBtn7,
        callback: IcpBtn7ValueCallback<'a>,
    },
    IcpBtn8 {
        control: IcpBtn8,
        callback: IcpBtn8ValueCallback<'a>,
    },
    IcpBtn9 {
        control: IcpBtn9,
        callback: IcpBtn9ValueCallback<'a>,
    },
    IcpCom1Btn {
        control: IcpCom1Btn,
        callback: IcpCom1BtnValueCallback<'a>,
    },
    IcpCom2Btn {
        control: IcpCom2Btn,
        callback: IcpCom2BtnValueCallback<'a>,
    },
    IcpDataRtnSeqSw {
        control: IcpDataRtnSeqSw,
        callback: IntegerCallback<'a>,
    },
    IcpDataUpDnSw {
        control: IcpDataUpDnSw,
        callback: IntegerCallback<'a>,
    },
    IcpDedSw {
        control: IcpDedSw,
        callback: IntegerCallback<'a>,
    },
    IcpDriftSw {
        control: IcpDriftSw,
        callback: IntegerCallback<'a>,
    },
    IcpEntrBtn {
        control: IcpEntrBtn,
        callback: IcpEntrBtnValueCallback<'a>,
    },
    IcpFlirGainSw {
        control: IcpFlirGainSw,
        callback: IcpFlirGainSwValueCallback<'a>,
    },
    IcpFlirSw {
        control: IcpFlirSw,
        callback: IntegerCallback<'a>,
    },
    IcpHudBrtKnb {
        control: IcpHudBrtKnb,
        callback: IntegerCallback<'a>,
    },
    IcpIffBtn {
        control: IcpIffBtn,
        callback: IcpIffBtnValueCallback<'a>,
    },
    IcpListBtn {
        control: IcpListBtn,
        callback: IcpListBtnValueCallback<'a>,
    },
    IcpRasterBrtKnb {
        control: IcpRasterBrtKnb,
        callback: IntegerCallback<'a>,
    },
    IcpRasterContrKnb {
        control: IcpRasterContrKnb,
        callback: IntegerCallback<'a>,
    },
    IcpRclBtn {
        control: IcpRclBtn,
        callback: IcpRclBtnValueCallback<'a>,
    },
    IcpReticleDepressKnb {
        control: IcpReticleDepressKnb,
        callback: IntegerCallback<'a>,
    },
    IcpWxBtn {
        control: IcpWxBtn,
        callback: IcpWxBtnValueCallback<'a>,
    },
    IffAntSelSw {
        control: IffAntSelSw,
        callback: IffAntSelSwValueCallback<'a>,
    },
    IffCIKnb {
        control: IffCIKnb,
        callback: IffCIKnbValueCallback<'a>,
    },
    IffCodeDrumDigit1 {
        control: IffCodeDrumDigit1,
        callback: IntegerCallback<'a>,
    },
    IffCodeDrumDigit2 {
        control: IffCodeDrumDigit2,
        callback: IntegerCallback<'a>,
    },
    IffCodeDrumDigit3 {
        control: IffCodeDrumDigit3,
        callback: IntegerCallback<'a>,
    },
    IffCodeDrumDigit4 {
        control: IffCodeDrumDigit4,
        callback: IntegerCallback<'a>,
    },
    IffEnableSw {
        control: IffEnableSw,
        callback: IffEnableSwValueCallback<'a>,
    },
    IffIdBtn {
        control: IffIdBtn,
        callback: IffIdBtnValueCallback<'a>,
    },
    IffM1Sel1 {
        control: IffM1Sel1,
        callback: IffM1Sel1ValueCallback<'a>,
    },
    IffM1Sel2 {
        control: IffM1Sel2,
        callback: IffM1Sel2ValueCallback<'a>,
    },
    IffM3Sel1 {
        control: IffM3Sel1,
        callback: IffM3Sel1ValueCallback<'a>,
    },
    IffM3Sel2 {
        control: IffM3Sel2,
        callback: IffM3Sel2ValueCallback<'a>,
    },
    IffM4CodeSw {
        control: IffM4CodeSw,
        callback: IffM4CodeSwValueCallback<'a>,
    },
    IffM4MonitorSw {
        control: IffM4MonitorSw,
        callback: IffM4MonitorSwValueCallback<'a>,
    },
    IffM4ReplySw {
        control: IffM4ReplySw,
        callback: IffM4ReplySwValueCallback<'a>,
    },
    IffMasterKnb {
        control: IffMasterKnb,
        callback: IffMasterKnbValueCallback<'a>,
    },
    IlsPwrKnb {
        control: IlsPwrKnb,
        callback: IntegerCallback<'a>,
    },
    InsKnb {
        control: InsKnb,
        callback: InsKnbValueCallback<'a>,
    },
    IntercomKnb {
        control: IntercomKnb,
        callback: IntegerCallback<'a>,
    },
    JfsSw {
        control: JfsSw,
        callback: IntegerCallback<'a>,
    },
    Ky58FillKnb {
        control: Ky58FillKnb,
        callback: Ky58FillKnbValueCallback<'a>,
    },
    Ky58ModeKnb {
        control: Ky58ModeKnb,
        callback: Ky58ModeKnbValueCallback<'a>,
    },
    Ky58PwrKnb {
        control: Ky58PwrKnb,
        callback: Ky58PwrKnbValueCallback<'a>,
    },
    Ky58VolKnb {
        control: Ky58VolKnb,
        callback: IntegerCallback<'a>,
    },
    LandTaxiLightSw {
        control: LandTaxiLightSw,
        callback: LandTaxiLightSwValueCallback<'a>,
    },
    LaserArmSw {
        control: LaserArmSw,
        callback: LaserArmSwValueCallback<'a>,
    },
    LeFlapsSw {
        control: LeFlapsSw,
        callback: LeFlapsSwValueCallback<'a>,
    },
    LightAcftBattFail {
        control: LightAcftBattFail,
        callback: LedCallback<'a>,
    },
    LightActive {
        control: LightActive,
        callback: LedCallback<'a>,
    },
    LightAftFuelLow {
        control: LightAftFuelLow,
        callback: LedCallback<'a>,
    },
    LightAir {
        control: LightAir,
        callback: LedCallback<'a>,
    },
    LightAntiSkid {
        control: LightAntiSkid,
        callback: LedCallback<'a>,
    },
    LightAoaDn {
        control: LightAoaDn,
        callback: LedCallback<'a>,
    },
    LightAoaMid {
        control: LightAoaMid,
        callback: LedCallback<'a>,
    },
    LightAoaUp {
        control: LightAoaUp,
        callback: LedCallback<'a>,
    },
    LightArNws {
        control: LightArNws,
        callback: LedCallback<'a>,
    },
    LightAtfNot {
        control: LightAtfNot,
        callback: LedCallback<'a>,
    },
    LightAvionicsFault {
        control: LightAvionicsFault,
        callback: LedCallback<'a>,
    },
    LightBuc {
        control: LightBuc,
        callback: LedCallback<'a>,
    },
    LightCabinPress {
        control: LightCabinPress,
        callback: LedCallback<'a>,
    },
    LightCadc {
        control: LightCadc,
        callback: LedCallback<'a>,
    },
    LightCanopy {
        control: LightCanopy,
        callback: LedCallback<'a>,
    },
    LightCaution1 {
        control: LightCaution1,
        callback: LedCallback<'a>,
    },
    LightCaution2 {
        control: LightCaution2,
        callback: LedCallback<'a>,
    },
    LightCaution3 {
        control: LightCaution3,
        callback: LedCallback<'a>,
    },
    LightCaution4 {
        control: LightCaution4,
        callback: LedCallback<'a>,
    },
    LightCaution5 {
        control: LightCaution5,
        callback: LedCallback<'a>,
    },
    LightCaution6 {
        control: LightCaution6,
        callback: LedCallback<'a>,
    },
    LightCmdsDisp {
        control: LightCmdsDisp,
        callback: LedCallback<'a>,
    },
    LightCmdsGo {
        control: LightCmdsGo,
        callback: LedCallback<'a>,
    },
    LightCmdsNoGo {
        control: LightCmdsNoGo,
        callback: LedCallback<'a>,
    },
    LightCmdsRdy {
        control: LightCmdsRdy,
        callback: LedCallback<'a>,
    },
    LightConsles {
        control: LightConsles,
        callback: LedCallback<'a>,
    },
    LightConslesFlood {
        control: LightConslesFlood,
        callback: LedCallback<'a>,
    },
    LightDbuOn {
        control: LightDbuOn,
        callback: LedCallback<'a>,
    },
    LightDisc {
        control: LightDisc,
        callback: LedCallback<'a>,
    },
    LightEcm1A {
        control: LightEcm1A,
        callback: LedCallback<'a>,
    },
    LightEcm1F {
        control: LightEcm1F,
        callback: LedCallback<'a>,
    },
    LightEcm1S {
        control: LightEcm1S,
        callback: LedCallback<'a>,
    },
    LightEcm1T {
        control: LightEcm1T,
        callback: LedCallback<'a>,
    },
    LightEcm2A {
        control: LightEcm2A,
        callback: LedCallback<'a>,
    },
    LightEcm2F {
        control: LightEcm2F,
        callback: LedCallback<'a>,
    },
    LightEcm2S {
        control: LightEcm2S,
        callback: LedCallback<'a>,
    },
    LightEcm2T {
        control: LightEcm2T,
        callback: LedCallback<'a>,
    },
    LightEcm3A {
        control: LightEcm3A,
        callback: LedCallback<'a>,
    },
    LightEcm3F {
        control: LightEcm3F,
        callback: LedCallback<'a>,
    },
    LightEcm3S {
        control: LightEcm3S,
        callback: LedCallback<'a>,
    },
    LightEcm3T {
        control: LightEcm3T,
        callback: LedCallback<'a>,
    },
    LightEcm4A {
        control: LightEcm4A,
        callback: LedCallback<'a>,
    },
    LightEcm4F {
        control: LightEcm4F,
        callback: LedCallback<'a>,
    },
    LightEcm4S {
        control: LightEcm4S,
        callback: LedCallback<'a>,
    },
    LightEcm4T {
        control: LightEcm4T,
        callback: LedCallback<'a>,
    },
    LightEcm5A {
        control: LightEcm5A,
        callback: LedCallback<'a>,
    },
    LightEcm5F {
        control: LightEcm5F,
        callback: LedCallback<'a>,
    },
    LightEcm5S {
        control: LightEcm5S,
        callback: LedCallback<'a>,
    },
    LightEcm5T {
        control: LightEcm5T,
        callback: LedCallback<'a>,
    },
    LightEcmA {
        control: LightEcmA,
        callback: LedCallback<'a>,
    },
    LightEcmF {
        control: LightEcmF,
        callback: LedCallback<'a>,
    },
    LightEcmFrmA {
        control: LightEcmFrmA,
        callback: LedCallback<'a>,
    },
    LightEcmFrmF {
        control: LightEcmFrmF,
        callback: LedCallback<'a>,
    },
    LightEcmFrmS {
        control: LightEcmFrmS,
        callback: LedCallback<'a>,
    },
    LightEcmFrmT {
        control: LightEcmFrmT,
        callback: LedCallback<'a>,
    },
    LightEcmS {
        control: LightEcmS,
        callback: LedCallback<'a>,
    },
    LightEcmSplA {
        control: LightEcmSplA,
        callback: LedCallback<'a>,
    },
    LightEcmSplF {
        control: LightEcmSplF,
        callback: LedCallback<'a>,
    },
    LightEcmSplS {
        control: LightEcmSplS,
        callback: LedCallback<'a>,
    },
    LightEcmSplT {
        control: LightEcmSplT,
        callback: LedCallback<'a>,
    },
    LightEcmT {
        control: LightEcmT,
        callback: LedCallback<'a>,
    },
    LightEdge {
        control: LightEdge,
        callback: LedCallback<'a>,
    },
    LightEec {
        control: LightEec,
        callback: LedCallback<'a>,
    },
    LightElec {
        control: LightElec,
        callback: LedCallback<'a>,
    },
    LightElecSys {
        control: LightElecSys,
        callback: LedCallback<'a>,
    },
    LightEngFire {
        control: LightEngFire,
        callback: LedCallback<'a>,
    },
    LightEngine {
        control: LightEngine,
        callback: LedCallback<'a>,
    },
    LightEngineFault {
        control: LightEngineFault,
        callback: LedCallback<'a>,
    },
    LightEpu {
        control: LightEpu,
        callback: LedCallback<'a>,
    },
    LightEpuGen {
        control: LightEpuGen,
        callback: LedCallback<'a>,
    },
    LightEpuPmg {
        control: LightEpuPmg,
        callback: LedCallback<'a>,
    },
    LightEquipHot {
        control: LightEquipHot,
        callback: LedCallback<'a>,
    },
    LightFlFail {
        control: LightFlFail,
        callback: LedCallback<'a>,
    },
    LightFlRun {
        control: LightFlRun,
        callback: LedCallback<'a>,
    },
    LightFlcs {
        control: LightFlcs,
        callback: LedCallback<'a>,
    },
    LightFlcsFault {
        control: LightFlcsFault,
        callback: LedCallback<'a>,
    },
    LightFlcsPmg {
        control: LightFlcsPmg,
        callback: LedCallback<'a>,
    },
    LightFlcsPwrA {
        control: LightFlcsPwrA,
        callback: LedCallback<'a>,
    },
    LightFlcsPwrB {
        control: LightFlcsPwrB,
        callback: LedCallback<'a>,
    },
    LightFlcsPwrC {
        control: LightFlcsPwrC,
        callback: LedCallback<'a>,
    },
    LightFlcsPwrD {
        control: LightFlcsPwrD,
        callback: LedCallback<'a>,
    },
    LightFlcsRly {
        control: LightFlcsRly,
        callback: LedCallback<'a>,
    },
    LightFuelOilHot {
        control: LightFuelOilHot,
        callback: LedCallback<'a>,
    },
    LightFwdFuelLow {
        control: LightFwdFuelLow,
        callback: LedCallback<'a>,
    },
    LightGearL {
        control: LightGearL,
        callback: LedCallback<'a>,
    },
    LightGearN {
        control: LightGearN,
        callback: LedCallback<'a>,
    },
    LightGearR {
        control: LightGearR,
        callback: LedCallback<'a>,
    },
    LightGearWarn {
        control: LightGearWarn,
        callback: LedCallback<'a>,
    },
    LightHook {
        control: LightHook,
        callback: LedCallback<'a>,
    },
    LightHydOilPress {
        control: LightHydOilPress,
        callback: LedCallback<'a>,
    },
    LightHydrazn {
        control: LightHydrazn,
        callback: LedCallback<'a>,
    },
    LightIff {
        control: LightIff,
        callback: LedCallback<'a>,
    },
    LightInletIcing {
        control: LightInletIcing,
        callback: LedCallback<'a>,
    },
    LightInstPnl {
        control: LightInstPnl,
        callback: LedCallback<'a>,
    },
    LightInstPnlFlood {
        control: LightInstPnlFlood,
        callback: LedCallback<'a>,
    },
    LightJfsRun {
        control: LightJfsRun,
        callback: LedCallback<'a>,
    },
    LightMainGen {
        control: LightMainGen,
        callback: LedCallback<'a>,
    },
    LightMarkerBeacon {
        control: LightMarkerBeacon,
        callback: LedCallback<'a>,
    },
    LightMasterCaution {
        control: LightMasterCaution,
        callback: LedCallback<'a>,
    },
    LightNuclear {
        control: LightNuclear,
        callback: LedCallback<'a>,
    },
    LightNwsFail {
        control: LightNwsFail,
        callback: LedCallback<'a>,
    },
    LightObogs {
        control: LightObogs,
        callback: LedCallback<'a>,
    },
    LightOverheat {
        control: LightOverheat,
        callback: LedCallback<'a>,
    },
    LightOxyLow {
        control: LightOxyLow,
        callback: LedCallback<'a>,
    },
    LightProbeHeat {
        control: LightProbeHeat,
        callback: LedCallback<'a>,
    },
    LightRadarAlt {
        control: LightRadarAlt,
        callback: LedCallback<'a>,
    },
    LightRdy {
        control: LightRdy,
        callback: LedCallback<'a>,
    },
    LightRwrActPower {
        control: LightRwrActPower,
        callback: LedCallback<'a>,
    },
    LightRwrActivity {
        control: LightRwrActivity,
        callback: LedCallback<'a>,
    },
    LightRwrAlt {
        control: LightRwrAlt,
        callback: LedCallback<'a>,
    },
    LightRwrAltLow {
        control: LightRwrAltLow,
        callback: LedCallback<'a>,
    },
    LightRwrHandoffH {
        control: LightRwrHandoffH,
        callback: LedCallback<'a>,
    },
    LightRwrHandoffUp {
        control: LightRwrHandoffUp,
        callback: LedCallback<'a>,
    },
    LightRwrModeOpen {
        control: LightRwrModeOpen,
        callback: LedCallback<'a>,
    },
    LightRwrModePri {
        control: LightRwrModePri,
        callback: LedCallback<'a>,
    },
    LightRwrMslLaunch {
        control: LightRwrMslLaunch,
        callback: LedCallback<'a>,
    },
    LightRwrPower {
        control: LightRwrPower,
        callback: LedCallback<'a>,
    },
    LightRwrSearch {
        control: LightRwrSearch,
        callback: LedCallback<'a>,
    },
    LightRwrShipUnk {
        control: LightRwrShipUnk,
        callback: LedCallback<'a>,
    },
    LightRwrSystest {
        control: LightRwrSystest,
        callback: LedCallback<'a>,
    },
    LightRwrTgtsepDn {
        control: LightRwrTgtsepDn,
        callback: LedCallback<'a>,
    },
    LightRwrTgtsepUp {
        control: LightRwrTgtsepUp,
        callback: LedCallback<'a>,
    },
    LightSeatNot {
        control: LightSeatNot,
        callback: LedCallback<'a>,
    },
    LightSec {
        control: LightSec,
        callback: LedCallback<'a>,
    },
    LightStby {
        control: LightStby,
        callback: LedCallback<'a>,
    },
    LightStbyGen {
        control: LightStbyGen,
        callback: LedCallback<'a>,
    },
    LightStoresConfig {
        control: LightStoresConfig,
        callback: LedCallback<'a>,
    },
    LightTfFail {
        control: LightTfFail,
        callback: LedCallback<'a>,
    },
    LightToFlcs {
        control: LightToFlcs,
        callback: LedCallback<'a>,
    },
    LightToLdgConfig {
        control: LightToLdgConfig,
        callback: LedCallback<'a>,
    },
    MachIndicator {
        control: MachIndicator,
        callback: IntegerCallback<'a>,
    },
    MainPwrSw {
        control: MainPwrSw,
        callback: MainPwrSwValueCallback<'a>,
    },
    MalIndLtsBrtSw {
        control: MalIndLtsBrtSw,
        callback: IntegerCallback<'a>,
    },
    MalIndLtsTest {
        control: MalIndLtsTest,
        callback: MalIndLtsTestValueCallback<'a>,
    },
    ManTfFlyupSw {
        control: ManTfFlyupSw,
        callback: ManTfFlyupSwValueCallback<'a>,
    },
    ManualPitchSw {
        control: ManualPitchSw,
        callback: ManualPitchSwValueCallback<'a>,
    },
    MapSw {
        control: MapSw,
        callback: MapSwValueCallback<'a>,
    },
    MasterArmSw {
        control: MasterArmSw,
        callback: MasterArmSwValueCallback<'a>,
    },
    MasterCaution {
        control: MasterCaution,
        callback: MasterCautionValueCallback<'a>,
    },
    MasterLightSw {
        control: MasterLightSw,
        callback: MasterLightSwValueCallback<'a>,
    },
    MaxAirspeed {
        control: MaxAirspeed,
        callback: IntegerCallback<'a>,
    },
    MaxPwrSw {
        control: MaxPwrSw,
        callback: MaxPwrSwValueCallback<'a>,
    },
    MfdL1 {
        control: MfdL1,
        callback: MfdL1ValueCallback<'a>,
    },
    MfdL10 {
        control: MfdL10,
        callback: MfdL10ValueCallback<'a>,
    },
    MfdL11 {
        control: MfdL11,
        callback: MfdL11ValueCallback<'a>,
    },
    MfdL12 {
        control: MfdL12,
        callback: MfdL12ValueCallback<'a>,
    },
    MfdL13 {
        control: MfdL13,
        callback: MfdL13ValueCallback<'a>,
    },
    MfdL14 {
        control: MfdL14,
        callback: MfdL14ValueCallback<'a>,
    },
    MfdL15 {
        control: MfdL15,
        callback: MfdL15ValueCallback<'a>,
    },
    MfdL16 {
        control: MfdL16,
        callback: MfdL16ValueCallback<'a>,
    },
    MfdL17 {
        control: MfdL17,
        callback: MfdL17ValueCallback<'a>,
    },
    MfdL18 {
        control: MfdL18,
        callback: MfdL18ValueCallback<'a>,
    },
    MfdL19 {
        control: MfdL19,
        callback: MfdL19ValueCallback<'a>,
    },
    MfdL2 {
        control: MfdL2,
        callback: MfdL2ValueCallback<'a>,
    },
    MfdL20 {
        control: MfdL20,
        callback: MfdL20ValueCallback<'a>,
    },
    MfdL3 {
        control: MfdL3,
        callback: MfdL3ValueCallback<'a>,
    },
    MfdL4 {
        control: MfdL4,
        callback: MfdL4ValueCallback<'a>,
    },
    MfdL5 {
        control: MfdL5,
        callback: MfdL5ValueCallback<'a>,
    },
    MfdL6 {
        control: MfdL6,
        callback: MfdL6ValueCallback<'a>,
    },
    MfdL7 {
        control: MfdL7,
        callback: MfdL7ValueCallback<'a>,
    },
    MfdL8 {
        control: MfdL8,
        callback: MfdL8ValueCallback<'a>,
    },
    MfdL9 {
        control: MfdL9,
        callback: MfdL9ValueCallback<'a>,
    },
    MfdLBrtSw {
        control: MfdLBrtSw,
        callback: MfdLBrtSwValueCallback<'a>,
    },
    MfdLConSw {
        control: MfdLConSw,
        callback: MfdLConSwValueCallback<'a>,
    },
    MfdLGainSw {
        control: MfdLGainSw,
        callback: MfdLGainSwValueCallback<'a>,
    },
    MfdLSymSw {
        control: MfdLSymSw,
        callback: MfdLSymSwValueCallback<'a>,
    },
    MfdR1 {
        control: MfdR1,
        callback: MfdR1ValueCallback<'a>,
    },
    MfdR10 {
        control: MfdR10,
        callback: MfdR10ValueCallback<'a>,
    },
    MfdR11 {
        control: MfdR11,
        callback: MfdR11ValueCallback<'a>,
    },
    MfdR12 {
        control: MfdR12,
        callback: MfdR12ValueCallback<'a>,
    },
    MfdR13 {
        control: MfdR13,
        callback: MfdR13ValueCallback<'a>,
    },
    MfdR14 {
        control: MfdR14,
        callback: MfdR14ValueCallback<'a>,
    },
    MfdR15 {
        control: MfdR15,
        callback: MfdR15ValueCallback<'a>,
    },
    MfdR16 {
        control: MfdR16,
        callback: MfdR16ValueCallback<'a>,
    },
    MfdR17 {
        control: MfdR17,
        callback: MfdR17ValueCallback<'a>,
    },
    MfdR18 {
        control: MfdR18,
        callback: MfdR18ValueCallback<'a>,
    },
    MfdR19 {
        control: MfdR19,
        callback: MfdR19ValueCallback<'a>,
    },
    MfdR2 {
        control: MfdR2,
        callback: MfdR2ValueCallback<'a>,
    },
    MfdR20 {
        control: MfdR20,
        callback: MfdR20ValueCallback<'a>,
    },
    MfdR3 {
        control: MfdR3,
        callback: MfdR3ValueCallback<'a>,
    },
    MfdR4 {
        control: MfdR4,
        callback: MfdR4ValueCallback<'a>,
    },
    MfdR5 {
        control: MfdR5,
        callback: MfdR5ValueCallback<'a>,
    },
    MfdR6 {
        control: MfdR6,
        callback: MfdR6ValueCallback<'a>,
    },
    MfdR7 {
        control: MfdR7,
        callback: MfdR7ValueCallback<'a>,
    },
    MfdR8 {
        control: MfdR8,
        callback: MfdR8ValueCallback<'a>,
    },
    MfdR9 {
        control: MfdR9,
        callback: MfdR9ValueCallback<'a>,
    },
    MfdRBrtSw {
        control: MfdRBrtSw,
        callback: MfdRBrtSwValueCallback<'a>,
    },
    MfdRConSw {
        control: MfdRConSw,
        callback: MfdRConSwValueCallback<'a>,
    },
    MfdRGainSw {
        control: MfdRGainSw,
        callback: MfdRGainSwValueCallback<'a>,
    },
    MfdRSymSw {
        control: MfdRSymSw,
        callback: MfdRSymSwValueCallback<'a>,
    },
    MfdSw {
        control: MfdSw,
        callback: MfdSwValueCallback<'a>,
    },
    MidsLvtKnb {
        control: MidsLvtKnb,
        callback: MidsLvtKnbValueCallback<'a>,
    },
    MmcPwrSw {
        control: MmcPwrSw,
        callback: MmcPwrSwValueCallback<'a>,
    },
    MslKnb {
        control: MslKnb,
        callback: IntegerCallback<'a>,
    },
    ObogsSw {
        control: ObogsSw,
        callback: ObogsSwValueCallback<'a>,
    },
    OxyDiluterLvr {
        control: OxyDiluterLvr,
        callback: OxyDiluterLvrValueCallback<'a>,
    },
    OxyEmergLvr {
        control: OxyEmergLvr,
        callback: IntegerCallback<'a>,
    },
    OxySupplyLvr {
        control: OxySupplyLvr,
        callback: OxySupplyLvrValueCallback<'a>,
    },
    OxygenPressure {
        control: OxygenPressure,
        callback: IntegerCallback<'a>,
    },
    PitchTrim {
        control: PitchTrim,
        callback: IntegerCallback<'a>,
    },
    Pitchtrimind {
        control: Pitchtrimind,
        callback: IntegerCallback<'a>,
    },
    PlainCipherSw {
        control: PlainCipherSw,
        callback: PlainCipherSwValueCallback<'a>,
    },
    PosFlashLightSw {
        control: PosFlashLightSw,
        callback: PosFlashLightSwValueCallback<'a>,
    },
    PosFuselageLightSw {
        control: PosFuselageLightSw,
        callback: PosFuselageLightSwValueCallback<'a>,
    },
    PosWingTailLightSw {
        control: PosWingTailLightSw,
        callback: PosWingTailLightSwValueCallback<'a>,
    },
    PriConsolesBrtKnb {
        control: PriConsolesBrtKnb,
        callback: IntegerCallback<'a>,
    },
    PriDataDisplayBrtKnb {
        control: PriDataDisplayBrtKnb,
        callback: IntegerCallback<'a>,
    },
    PriInstPnlBrtKnb {
        control: PriInstPnlBrtKnb,
        callback: IntegerCallback<'a>,
    },
    ProbeHeatSw {
        control: ProbeHeatSw,
        callback: IntegerCallback<'a>,
    },
    RdrAltPwrSw {
        control: RdrAltPwrSw,
        callback: RdrAltPwrSwValueCallback<'a>,
    },
    RfSw {
        control: RfSw,
        callback: RfSwValueCallback<'a>,
    },
    RollTrim {
        control: RollTrim,
        callback: IntegerCallback<'a>,
    },
    Rolltrimind {
        control: Rolltrimind,
        callback: IntegerCallback<'a>,
    },
    RwrActPwrBtn {
        control: RwrActPwrBtn,
        callback: RwrActPwrBtnValueCallback<'a>,
    },
    RwrAltBtn {
        control: RwrAltBtn,
        callback: RwrAltBtnValueCallback<'a>,
    },
    RwrHandoffBtn {
        control: RwrHandoffBtn,
        callback: RwrHandoffBtnValueCallback<'a>,
    },
    RwrIndDimKnb {
        control: RwrIndDimKnb,
        callback: IntegerCallback<'a>,
    },
    RwrIntensKnb {
        control: RwrIntensKnb,
        callback: IntegerCallback<'a>,
    },
    RwrLaunchBtn {
        control: RwrLaunchBtn,
        callback: RwrLaunchBtnValueCallback<'a>,
    },
    RwrModeBtn {
        control: RwrModeBtn,
        callback: RwrModeBtnValueCallback<'a>,
    },
    RwrPwrBtn {
        control: RwrPwrBtn,
        callback: RwrPwrBtnValueCallback<'a>,
    },
    RwrSearchBtn {
        control: RwrSearchBtn,
        callback: RwrSearchBtnValueCallback<'a>,
    },
    RwrSysTestBtn {
        control: RwrSysTestBtn,
        callback: RwrSysTestBtnValueCallback<'a>,
    },
    RwrTBtn {
        control: RwrTBtn,
        callback: RwrTBtnValueCallback<'a>,
    },
    RwrUnknownShipBtn {
        control: RwrUnknownShipBtn,
        callback: RwrUnknownShipBtnValueCallback<'a>,
    },
    SaiAircraftreferencesymbol {
        control: SaiAircraftreferencesymbol,
        callback: IntegerCallback<'a>,
    },
    SaiBank {
        control: SaiBank,
        callback: IntegerCallback<'a>,
    },
    SaiBankArrow {
        control: SaiBankArrow,
        callback: IntegerCallback<'a>,
    },
    SaiCage {
        control: SaiCage,
        callback: SaiCageValueCallback<'a>,
    },
    SaiKnbArrow {
        control: SaiKnbArrow,
        callback: IntegerCallback<'a>,
    },
    SaiOffFlag {
        control: SaiOffFlag,
        callback: IntegerCallback<'a>,
    },
    SaiPitch {
        control: SaiPitch,
        callback: IntegerCallback<'a>,
    },
    SaiPitchTrim {
        control: SaiPitchTrim,
        callback: IntegerCallback<'a>,
    },
    SeatAdj {
        control: SeatAdj,
        callback: IntegerCallback<'a>,
    },
    SeatEjectHandle {
        control: SeatEjectHandle,
        callback: SeatEjectHandleValueCallback<'a>,
    },
    SeatEjectSafe {
        control: SeatEjectSafe,
        callback: SeatEjectSafeValueCallback<'a>,
    },
    SeatHeight {
        control: SeatHeight,
        callback: IntegerCallback<'a>,
    },
    SecVoiceKnb {
        control: SecVoiceKnb,
        callback: IntegerCallback<'a>,
    },
    SetAirspeed {
        control: SetAirspeed,
        callback: IntegerCallback<'a>,
    },
    SpeedbrakeIndicator {
        control: SpeedbrakeIndicator,
        callback: IntegerCallback<'a>,
    },
    StStaSw {
        control: StStaSw,
        callback: StStaSwValueCallback<'a>,
    },
    StoresConfigSw {
        control: StoresConfigSw,
        callback: StoresConfigSwValueCallback<'a>,
    },
    SysaPressure {
        control: SysaPressure,
        callback: IntegerCallback<'a>,
    },
    SysbPressure {
        control: SysbPressure,
        callback: IntegerCallback<'a>,
    },
    TacanKnb {
        control: TacanKnb,
        callback: IntegerCallback<'a>,
    },
    TankIntertingSw {
        control: TankIntertingSw,
        callback: TankIntertingSwValueCallback<'a>,
    },
    TempKnb {
        control: TempKnb,
        callback: TempKnbValueCallback<'a>,
    },
    TfKnb {
        control: TfKnb,
        callback: IntegerCallback<'a>,
    },
    ThreatKnb {
        control: ThreatKnb,
        callback: IntegerCallback<'a>,
    },
    TrimApDiscSw {
        control: TrimApDiscSw,
        callback: TrimApDiscSwValueCallback<'a>,
    },
    UfcSw {
        control: UfcSw,
        callback: UfcSwValueCallback<'a>,
    },
    UhfAntSelSw {
        control: UhfAntSelSw,
        callback: UhfAntSelSwValueCallback<'a>,
    },
    UhfChanDisp {
        control: UhfChanDisp,
        callback: DisplayCallback<'a>,
    },
    UhfChanKnb {
        control: UhfChanKnb,
        callback: IntegerCallback<'a>,
    },
    UhfDoor {
        control: UhfDoor,
        callback: UhfDoorValueCallback<'a>,
    },
    UhfFreq0025Knb {
        control: UhfFreq0025Knb,
        callback: UhfFreq0025KnbValueCallback<'a>,
    },
    UhfFreq01Knb {
        control: UhfFreq01Knb,
        callback: UhfFreq01KnbValueCallback<'a>,
    },
    UhfFreq100Knb {
        control: UhfFreq100Knb,
        callback: UhfFreq100KnbValueCallback<'a>,
    },
    UhfFreq10Knb {
        control: UhfFreq10Knb,
        callback: UhfFreq10KnbValueCallback<'a>,
    },
    UhfFreq1Knb {
        control: UhfFreq1Knb,
        callback: UhfFreq1KnbValueCallback<'a>,
    },
    UhfFreqDisp {
        control: UhfFreqDisp,
        callback: DisplayCallback<'a>,
    },
    UhfFuncKnb {
        control: UhfFuncKnb,
        callback: UhfFuncKnbValueCallback<'a>,
    },
    UhfModeKnb {
        control: UhfModeKnb,
        callback: UhfModeKnbValueCallback<'a>,
    },
    UhfSquelchSw {
        control: UhfSquelchSw,
        callback: UhfSquelchSwValueCallback<'a>,
    },
    UhfStatusBtn {
        control: UhfStatusBtn,
        callback: UhfStatusBtnValueCallback<'a>,
    },
    UhfTestBtn {
        control: UhfTestBtn,
        callback: UhfTestBtnValueCallback<'a>,
    },
    UhfToneBtn {
        control: UhfToneBtn,
        callback: UhfToneBtnValueCallback<'a>,
    },
    UhfVolKnb {
        control: UhfVolKnb,
        callback: IntegerCallback<'a>,
    },
    VmsInhibitSw {
        control: VmsInhibitSw,
        callback: VmsInhibitSwValueCallback<'a>,
    },
    Vvi {
        control: Vvi,
        callback: IntegerCallback<'a>,
    },
    YawTrim {
        control: YawTrim,
        callback: IntegerCallback<'a>,
    },
    ZeroizeSw {
        control: ZeroizeSw,
        callback: ZeroizeSwValueCallback<'a>,
    },
    ZeroizeSwCover {
        control: ZeroizeSwCover,
        callback: ZeroizeSwCoverValueCallback<'a>,
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
                        <AdvModeSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                    if let Ok(value) =
                        <AirRefuelSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::AirSourceKnb { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <AirSourceKnbValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
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
                        <AltFlapsSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                        <AltRelBtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::AntiCollLightKnb { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <AntiCollLightKnbValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
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
                    if let Ok(value) =
                        <ApRollSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                    if let Ok(value) =
                        <BitSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::BrakeChanSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <BrakeChanSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CanopyHandle { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <CanopyHandleValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::CanopyJettThandle { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CanopyJettThandleValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
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
                    if let Ok(value) = <ClockElapsedValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
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
                        <ClockSetValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                        <Cmds01ExpCatSwValue as core::convert::TryFrom<u16>>::try_from(
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
                        <Cmds02ExpCatSwValue as core::convert::TryFrom<u16>>::try_from(
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
                        <CmdsChExpCatSwValue as core::convert::TryFrom<u16>>::try_from(
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
                        <CmdsDispenseBtnValue as core::convert::TryFrom<u16>>::try_from(
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
                        <CmdsFlExpCatSwValue as core::convert::TryFrom<u16>>::try_from(
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
                        <CmdsJettSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CmdsJmrSourcheSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CmdsJmrSourcheSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CmdsModeKnb { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CmdsModeKnbValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CmdsMwsSourcheSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CmdsMwsSourcheSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
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
                        <CmdsProgKnbValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CmdsPwrSourcheSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CmdsPwrSourcheSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
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
                    if let Ok(value) = <Comm1ModeKnbValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
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
                    if let Ok(value) = <Comm2ModeKnbValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
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
                        <DefogLeverValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::DigiBakSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <DigiBakSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::DlSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <DlSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::DnLockBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <DnLockBtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Ecm1Btn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <Ecm1BtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Ecm2Btn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <Ecm2BtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Ecm3Btn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <Ecm3BtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Ecm4Btn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <Ecm4BtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Ecm5Btn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <Ecm5BtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Ecm6Btn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <Ecm6BtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::EcmBitBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <EcmBitBtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                        <EcmFrmBtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::EcmPwSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <EcmPwSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::EcmResetBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <EcmResetBtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::EcmSplBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <EcmSplBtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::EcmXmitSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <EcmXmitSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::EhsiCrsSet { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <EhsiCrsSetValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                    if let Ok(value) = <EhsiHdgSetBtnValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
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
                        <EhsiModeValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::ElecCaution { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ElecCautionValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::EmergStroreJett { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <EmergStroreJettValue as core::convert::TryFrom<u16>>::try_from(
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
                        <EngAntiIceValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::EngContSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <EngContSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::EngContSwCover { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <EngContSwCoverValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::EngineFeedKnb { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <EngineFeedKnbValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
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
                    if let Ok(value) = <EpuGenTestSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::EpuSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <EpuSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::EpuSwCoverOff { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <EpuSwCoverOffValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::EpuSwCoverOn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <EpuSwCoverOnValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
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
                    if let Ok(value) =
                        <ExtFuelTransSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
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
                    if let Ok(value) =
                        <FAckBtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FcrPwrSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <FcrPwrSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::FireOheatDetectBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <FireOheatDetectBtnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
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
                    if let Ok(value) =
                        <FlcsResetSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
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
                    if let Ok(value) = <FuelMasterCvValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::FuelMasterSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <FuelMasterSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::FuelQtySelKnb { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <FuelQtySelKnbValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
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
                    if let Ok(value) =
                        <GearHandleValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::GndJettEnableSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <GndJettEnableSwValue as core::convert::TryFrom<u16>>::try_from(
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
                        <GpsSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::HdptSwL { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <HdptSwLValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::HdptSwR { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <HdptSwRValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::HideStick { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <HideStickValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                        <HookSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::HornSilenceBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <HornSilenceBtnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::HotMicSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <HotMicSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::HudAltSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <HudAltSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::HudBrtSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <HudBrtSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::HudDedDataSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <HudDedDataSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::HudDepressRetSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <HudDepressRetSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::HudFpMarkerSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <HudFpMarkerSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::HudScalesSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <HudScalesSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::HudSpeedSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <HudSpeedSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::HudTestSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <HudTestSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                    if let Ok(value) = <IcpAaModeBtnValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::IcpAgModeBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <IcpAgModeBtnValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::IcpBtn0 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IcpBtn0Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IcpBtn1 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IcpBtn1Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IcpBtn2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IcpBtn2Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IcpBtn3 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IcpBtn3Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IcpBtn4 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IcpBtn4Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IcpBtn5 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IcpBtn5Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IcpBtn6 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IcpBtn6Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IcpBtn7 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IcpBtn7Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IcpBtn8 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IcpBtn8Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IcpBtn9 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IcpBtn9Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IcpCom1Btn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IcpCom1BtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IcpCom2Btn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IcpCom2BtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                        <IcpEntrBtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IcpFlirGainSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <IcpFlirGainSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
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
                        <IcpIffBtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IcpListBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IcpListBtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                        <IcpRclBtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                    if let Ok(value) =
                        <IcpWxBtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IffAntSelSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IffAntSelSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IffCIKnb { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IffCIKnbValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
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
                        <IffEnableSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IffIdBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IffIdBtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IffM1Sel1 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IffM1Sel1Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IffM1Sel2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IffM1Sel2Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IffM3Sel1 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IffM3Sel1Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IffM3Sel2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IffM3Sel2Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IffM4CodeSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IffM4CodeSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IffM4MonitorSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IffM4MonitorSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IffM4ReplySw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <IffM4ReplySwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::IffMasterKnb { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <IffMasterKnbValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
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
                        <InsKnbValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                        <Ky58FillKnbValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Ky58ModeKnb { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <Ky58ModeKnbValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::Ky58PwrKnb { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <Ky58PwrKnbValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                    if let Ok(value) =
                        <LandTaxiLightSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::LaserArmSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <LaserArmSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::LeFlapsSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <LeFlapsSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                    if let Ok(value) =
                        <MainPwrSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                    if let Ok(value) = <MalIndLtsTestValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::ManTfFlyupSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <ManTfFlyupSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::ManualPitchSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <ManualPitchSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::MapSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MapSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
            Self::MasterCaution { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <MasterCautionValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::MasterLightSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <MasterLightSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
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
                        <MaxPwrSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL1 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdL1Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL10 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdL10Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL11 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdL11Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL12 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdL12Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL13 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdL13Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL14 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdL14Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL15 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdL15Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL16 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdL16Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL17 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdL17Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL18 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdL18Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL19 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdL19Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdL2Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL20 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdL20Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL3 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdL3Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL4 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdL4Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL5 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdL5Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL6 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdL6Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL7 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdL7Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL8 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdL8Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdL9 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdL9Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdLBrtSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdLBrtSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdLConSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdLConSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdLGainSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdLGainSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdLSymSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdLSymSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR1 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdR1Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR10 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdR10Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR11 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdR11Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR12 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdR12Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR13 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdR13Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR14 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdR14Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR15 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdR15Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR16 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdR16Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR17 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdR17Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR18 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdR18Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR19 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdR19Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdR2Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR20 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdR20Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR3 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdR3Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR4 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdR4Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR5 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdR5Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR6 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdR6Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR7 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdR7Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR8 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdR8Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdR9 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdR9Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdRBrtSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdRBrtSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdRConSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdRConSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdRGainSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdRGainSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdRSymSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdRSymSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MfdSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MfdSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MidsLvtKnb { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MidsLvtKnbValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MmcPwrSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <MmcPwrSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                        <ObogsSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::OxyDiluterLvr { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <OxyDiluterLvrValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
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
                    if let Ok(value) = <OxySupplyLvrValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
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
                    if let Ok(value) = <PlainCipherSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::PosFlashLightSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <PosFlashLightSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::PosFuselageLightSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <PosFuselageLightSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::PosWingTailLightSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <PosWingTailLightSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
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
                    if let Ok(value) =
                        <RdrAltPwrSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RfSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <RfSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                    if let Ok(value) = <RwrActPwrBtnValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::RwrAltBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <RwrAltBtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RwrHandoffBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <RwrHandoffBtnValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
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
                    if let Ok(value) = <RwrLaunchBtnValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::RwrModeBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <RwrModeBtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RwrPwrBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <RwrPwrBtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RwrSearchBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <RwrSearchBtnValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::RwrSysTestBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <RwrSysTestBtnValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::RwrTBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <RwrTBtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RwrUnknownShipBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <RwrUnknownShipBtnValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
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
                    if let Ok(value) =
                        <SaiCageValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
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
                    if let Ok(value) =
                        <SeatEjectHandleValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::SeatEjectSafe { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <SeatEjectSafeValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
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
                        <StStaSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::StoresConfigSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <StoresConfigSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
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
                        <TankIntertingSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::TempKnb { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <TempKnbValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
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
                    if let Ok(value) = <TrimApDiscSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::UfcSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <UfcSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfAntSelSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <UhfAntSelSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                    if let Ok(value) =
                        <UhfDoorValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfFreq0025Knb { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <UhfFreq0025KnbValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfFreq01Knb { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <UhfFreq01KnbValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::UhfFreq100Knb { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <UhfFreq100KnbValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::UhfFreq10Knb { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <UhfFreq10KnbValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::UhfFreq1Knb { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <UhfFreq1KnbValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                        <UhfFuncKnbValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfModeKnb { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <UhfModeKnbValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfSquelchSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <UhfSquelchSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::UhfStatusBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <UhfStatusBtnValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::UhfTestBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <UhfTestBtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfToneBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <UhfToneBtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                    if let Ok(value) = <VmsInhibitSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
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
                        <ZeroizeSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::ZeroizeSwCover { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ZeroizeSwCoverValue as core::convert::TryFrom<u16>>::try_from(
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

    pub fn register_ab_reset_sw<F>(
        &mut self,
        control: AbResetSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AbResetSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_aux_flag<F>(
        &mut self,
        control: AdiAuxFlag,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiAuxFlag { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_bank<F>(
        &mut self,
        control: AdiBank,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiBank { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_bubble<F>(
        &mut self,
        control: AdiBubble,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiBubble { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_gs_bar<F>(
        &mut self,
        control: AdiGsBar,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiGsBar { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_gs_flag<F>(
        &mut self,
        control: AdiGsFlag,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiGsFlag { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_gs_pointer<F>(
        &mut self,
        control: AdiGsPointer,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiGsPointer { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_loc_bar<F>(
        &mut self,
        control: AdiLocBar,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiLocBar { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_loc_flag<F>(
        &mut self,
        control: AdiLocFlag,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiLocFlag { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_off_flag<F>(
        &mut self,
        control: AdiOffFlag,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiOffFlag { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_pitch<F>(
        &mut self,
        control: AdiPitch,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiPitch { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_pitch_trim<F>(
        &mut self,
        control: AdiPitchTrim,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiPitchTrim { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_turnrate<F>(
        &mut self,
        control: AdiTurnrate,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiTurnrate { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adv_mode_sw<F>(
        &mut self,
        control: AdvModeSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(AdvModeSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdvModeSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_air_refuel_light_knb<F>(
        &mut self,
        control: AirRefuelLightKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AirRefuelLightKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_air_refuel_sw<F>(
        &mut self,
        control: AirRefuelSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(AirRefuelSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AirRefuelSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_air_source_knb<F>(
        &mut self,
        control: AirSourceKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(AirSourceKnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AirSourceKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_airspeed<F>(
        &mut self,
        control: Airspeed,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Airspeed { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_airspeed_set_knb<F>(
        &mut self,
        control: AirspeedSetKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AirspeedSetKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt10000_ft_cnt<F>(
        &mut self,
        control: Alt10000FtCnt,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Alt10000FtCnt { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt1000_ft_cnt<F>(
        &mut self,
        control: Alt1000FtCnt,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Alt1000FtCnt { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt100_ft_cnt<F>(
        &mut self,
        control: Alt100FtCnt,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Alt100FtCnt { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt100_ft_ptr<F>(
        &mut self,
        control: Alt100FtPtr,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Alt100FtPtr { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_baro_set_knb<F>(
        &mut self,
        control: AltBaroSetKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltBaroSetKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_flaps_sw<F>(
        &mut self,
        control: AltFlapsSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(AltFlapsSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltFlapsSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_mode_lv<F>(
        &mut self,
        control: AltModeLv,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltModeLv { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_pneu_flag<F>(
        &mut self,
        control: AltPneuFlag,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltPneuFlag { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_pressure_drum0_cnt<F>(
        &mut self,
        control: AltPressureDrum0Cnt,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltPressureDrum0Cnt { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_pressure_drum1_cnt<F>(
        &mut self,
        control: AltPressureDrum1Cnt,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltPressureDrum1Cnt { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_pressure_drum2_cnt<F>(
        &mut self,
        control: AltPressureDrum2Cnt,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltPressureDrum2Cnt { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_pressure_drum3_cnt<F>(
        &mut self,
        control: AltPressureDrum3Cnt,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltPressureDrum3Cnt { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_rel_btn<F>(
        &mut self,
        control: AltRelBtn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(AltRelBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltRelBtn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_anti_coll_light_knb<F>(
        &mut self,
        control: AntiCollLightKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(AntiCollLightKnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AntiCollLightKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_anti_skid_sw<F>(
        &mut self,
        control: AntiSkidSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AntiSkidSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_aoa_index_brt_knb<F>(
        &mut self,
        control: AoaIndexBrtKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AoaIndexBrtKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_aoa_value<F>(
        &mut self,
        control: AoaValue,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AoaValue { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ap_pitch_sw<F>(
        &mut self,
        control: ApPitchSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ApPitchSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ap_roll_sw<F>(
        &mut self,
        control: ApRollSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ApRollSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ApRollSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ar_status_brt_knb<F>(
        &mut self,
        control: ArStatusBrtKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ArStatusBrtKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_bit_sw<F>(
        &mut self,
        control: BitSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(BitSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::BitSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_brake_chan_sw<F>(
        &mut self,
        control: BrakeChanSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(BrakeChanSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::BrakeChanSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_canopy_handle<F>(
        &mut self,
        control: CanopyHandle,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CanopyHandleValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CanopyHandle { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_canopy_jett_thandle<F>(
        &mut self,
        control: CanopyJettThandle,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CanopyJettThandleValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CanopyJettThandle { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_canopy_sw<F>(
        &mut self,
        control: CanopySw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CanopySw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_canopy_value<F>(
        &mut self,
        control: CanopyValue,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CanopyValue { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_currtime_h<F>(
        &mut self,
        control: ClockCurrtimeH,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockCurrtimeH { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_currtime_ms<F>(
        &mut self,
        control: ClockCurrtimeMs,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockCurrtimeMs { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_elapsed<F>(
        &mut self,
        control: ClockElapsed,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ClockElapsedValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockElapsed { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_elapsed_time_m<F>(
        &mut self,
        control: ClockElapsedTimeM,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockElapsedTimeM { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_elapsed_time_sec<F>(
        &mut self,
        control: ClockElapsedTimeSec,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockElapsedTimeSec { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_set<F>(
        &mut self,
        control: ClockSet,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ClockSetValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockSet { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_wind<F>(
        &mut self,
        control: ClockWind,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockWind { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cmds01_exp_cat_sw<F>(
        &mut self,
        control: Cmds01ExpCatSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Cmds01ExpCatSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Cmds01ExpCatSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cmds02_exp_cat_sw<F>(
        &mut self,
        control: Cmds02ExpCatSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Cmds02ExpCatSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Cmds02ExpCatSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cmds_ch_amount<F>(
        &mut self,
        control: CmdsChAmount,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmdsChAmount { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cmds_ch_exp_cat_sw<F>(
        &mut self,
        control: CmdsChExpCatSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CmdsChExpCatSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmdsChExpCatSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cmds_dispense_btn<F>(
        &mut self,
        control: CmdsDispenseBtn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CmdsDispenseBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmdsDispenseBtn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cmds_fl_amount<F>(
        &mut self,
        control: CmdsFlAmount,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmdsFlAmount { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cmds_fl_exp_cat_sw<F>(
        &mut self,
        control: CmdsFlExpCatSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CmdsFlExpCatSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmdsFlExpCatSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cmds_jett_sw<F>(
        &mut self,
        control: CmdsJettSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CmdsJettSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmdsJettSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cmds_jmr_sourche_sw<F>(
        &mut self,
        control: CmdsJmrSourcheSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CmdsJmrSourcheSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmdsJmrSourcheSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cmds_mode_knb<F>(
        &mut self,
        control: CmdsModeKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CmdsModeKnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmdsModeKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cmds_mws_sourche_sw<F>(
        &mut self,
        control: CmdsMwsSourcheSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CmdsMwsSourcheSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmdsMwsSourcheSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cmds_o1_amount<F>(
        &mut self,
        control: CmdsO1Amount,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmdsO1Amount { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cmds_o2_amount<F>(
        &mut self,
        control: CmdsO2Amount,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmdsO2Amount { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cmds_prog_knb<F>(
        &mut self,
        control: CmdsProgKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CmdsProgKnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmdsProgKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cmds_pwr_sourche_sw<F>(
        &mut self,
        control: CmdsPwrSourcheSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CmdsPwrSourcheSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmdsPwrSourcheSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cockpit_alititude<F>(
        &mut self,
        control: CockpitAlititude,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CockpitAlititude { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_comm1_mode_knb<F>(
        &mut self,
        control: Comm1ModeKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Comm1ModeKnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Comm1ModeKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_comm1_pwr_knb<F>(
        &mut self,
        control: Comm1PwrKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Comm1PwrKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_comm2_mode_knb<F>(
        &mut self,
        control: Comm2ModeKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Comm2ModeKnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Comm2ModeKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_comm2_pwr_knb<F>(
        &mut self,
        control: Comm2PwrKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Comm2PwrKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ded_line1<F>(
        &mut self,
        control: DedLine1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::DedLine1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ded_line2<F>(
        &mut self,
        control: DedLine2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::DedLine2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ded_line3<F>(
        &mut self,
        control: DedLine3,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::DedLine3 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ded_line4<F>(
        &mut self,
        control: DedLine4,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::DedLine4 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ded_line5<F>(
        &mut self,
        control: DedLine5,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::DedLine5 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_defog_lever<F>(
        &mut self,
        control: DefogLever,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(DefogLeverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::DefogLever { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_digi_bak_sw<F>(
        &mut self,
        control: DigiBakSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(DigiBakSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::DigiBakSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dl_sw<F>(
        &mut self,
        control: DlSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(DlSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::DlSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dn_lock_btn<F>(
        &mut self,
        control: DnLockBtn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(DnLockBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::DnLockBtn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecm1_btn<F>(
        &mut self,
        control: Ecm1Btn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Ecm1BtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ecm1Btn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecm2_btn<F>(
        &mut self,
        control: Ecm2Btn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Ecm2BtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ecm2Btn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecm3_btn<F>(
        &mut self,
        control: Ecm3Btn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Ecm3BtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ecm3Btn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecm4_btn<F>(
        &mut self,
        control: Ecm4Btn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Ecm4BtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ecm4Btn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecm5_btn<F>(
        &mut self,
        control: Ecm5Btn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Ecm5BtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ecm5Btn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecm6_btn<F>(
        &mut self,
        control: Ecm6Btn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Ecm6BtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ecm6Btn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecm_bit_btn<F>(
        &mut self,
        control: EcmBitBtn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EcmBitBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EcmBitBtn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecm_dim_knb<F>(
        &mut self,
        control: EcmDimKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EcmDimKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecm_frm_btn<F>(
        &mut self,
        control: EcmFrmBtn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EcmFrmBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EcmFrmBtn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecm_pw_sw<F>(
        &mut self,
        control: EcmPwSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EcmPwSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EcmPwSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecm_reset_btn<F>(
        &mut self,
        control: EcmResetBtn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EcmResetBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EcmResetBtn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecm_spl_btn<F>(
        &mut self,
        control: EcmSplBtn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EcmSplBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EcmSplBtn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ecm_xmit_sw<F>(
        &mut self,
        control: EcmXmitSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EcmXmitSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EcmXmitSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ehsi_crs_set<F>(
        &mut self,
        control: EhsiCrsSet,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EhsiCrsSetValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EhsiCrsSet { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ehsi_crs_set_knb<F>(
        &mut self,
        control: EhsiCrsSetKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EhsiCrsSetKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ehsi_hdg_set_btn<F>(
        &mut self,
        control: EhsiHdgSetBtn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EhsiHdgSetBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EhsiHdgSetBtn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ehsi_hdg_set_knb<F>(
        &mut self,
        control: EhsiHdgSetKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EhsiHdgSetKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ehsi_mode<F>(
        &mut self,
        control: EhsiMode,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EhsiModeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EhsiMode { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_elec_caution<F>(
        &mut self,
        control: ElecCaution,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ElecCautionValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ElecCaution { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_emerg_strore_jett<F>(
        &mut self,
        control: EmergStroreJett,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EmergStroreJettValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EmergStroreJett { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_eng_anti_ice<F>(
        &mut self,
        control: EngAntiIce,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EngAntiIceValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngAntiIce { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_eng_cont_sw<F>(
        &mut self,
        control: EngContSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EngContSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngContSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_eng_cont_sw_cover<F>(
        &mut self,
        control: EngContSwCover,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EngContSwCoverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngContSwCover { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_engine_feed_knb<F>(
        &mut self,
        control: EngineFeedKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EngineFeedKnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngineFeedKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_engine_ftit<F>(
        &mut self,
        control: EngineFtit,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngineFtit { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_engine_nozzle_position<F>(
        &mut self,
        control: EngineNozzlePosition,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngineNozzlePosition { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_engine_oil_pressure<F>(
        &mut self,
        control: EngineOilPressure,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngineOilPressure { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_engine_tachometer<F>(
        &mut self,
        control: EngineTachometer,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngineTachometer { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_epu_gen_test_sw<F>(
        &mut self,
        control: EpuGenTestSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EpuGenTestSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EpuGenTestSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_epu_sw<F>(
        &mut self,
        control: EpuSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EpuSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EpuSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_epu_sw_cover_off<F>(
        &mut self,
        control: EpuSwCoverOff,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EpuSwCoverOffValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EpuSwCoverOff { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_epu_sw_cover_on<F>(
        &mut self,
        control: EpuSwCoverOn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EpuSwCoverOnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EpuSwCoverOn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_formation_lights<F>(
        &mut self,
        control: ExtFormationLights,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtFormationLights { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_fuel_trans_sw<F>(
        &mut self,
        control: ExtFuelTransSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ExtFuelTransSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtFuelTransSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_position_light_fuselage<F>(
        &mut self,
        control: ExtPositionLightFuselage,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtPositionLightFuselage { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_position_lights_wing<F>(
        &mut self,
        control: ExtPositionLightsWing,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtPositionLightsWing { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_speed_brake_left<F>(
        &mut self,
        control: ExtSpeedBrakeLeft,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtSpeedBrakeLeft { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_speed_brake_right<F>(
        &mut self,
        control: ExtSpeedBrakeRight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtSpeedBrakeRight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_strobe_tail<F>(
        &mut self,
        control: ExtStrobeTail,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtStrobeTail { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_tail_light<F>(
        &mut self,
        control: ExtTailLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtTailLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_wow_left<F>(
        &mut self,
        control: ExtWowLeft,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtWowLeft { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_wow_nose<F>(
        &mut self,
        control: ExtWowNose,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtWowNose { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_wow_right<F>(
        &mut self,
        control: ExtWowRight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtWowRight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_f_ack_btn<F>(
        &mut self,
        control: FAckBtn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(FAckBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FAckBtn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fcr_pwr_sw<F>(
        &mut self,
        control: FcrPwrSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(FcrPwrSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FcrPwrSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fire_oheat_detect_btn<F>(
        &mut self,
        control: FireOheatDetectBtn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(FireOheatDetectBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FireOheatDetectBtn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flcs_pwr_test_sw<F>(
        &mut self,
        control: FlcsPwrTestSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlcsPwrTestSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flcs_reset_sw<F>(
        &mut self,
        control: FlcsResetSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(FlcsResetSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlcsResetSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flood_consoles_brt_knb<F>(
        &mut self,
        control: FloodConsolesBrtKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FloodConsolesBrtKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flood_inst_pnl_brt_knb<F>(
        &mut self,
        control: FloodInstPnlBrtKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FloodInstPnlBrtKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flow_indicator<F>(
        &mut self,
        control: FlowIndicator,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlowIndicator { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_flow_indicator_light<F>(
        &mut self,
        control: FlowIndicatorLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::FlowIndicatorLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_form_light_knb<F>(
        &mut self,
        control: FormLightKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FormLightKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_al<F>(
        &mut self,
        control: FuelAl,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelAl { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_fr<F>(
        &mut self,
        control: FuelFr,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelFr { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_master_cv<F>(
        &mut self,
        control: FuelMasterCv,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(FuelMasterCvValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelMasterCv { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_master_sw<F>(
        &mut self,
        control: FuelMasterSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(FuelMasterSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelMasterSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_qty_sel_knb<F>(
        &mut self,
        control: FuelQtySelKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(FuelQtySelKnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelQtySelKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_qty_sel_t_knb<F>(
        &mut self,
        control: FuelQtySelTKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelQtySelTKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuelflowcounter100<F>(
        &mut self,
        control: Fuelflowcounter100,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fuelflowcounter100 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuelflowcounter10k<F>(
        &mut self,
        control: Fuelflowcounter10k,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fuelflowcounter10k { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuelflowcounter1k<F>(
        &mut self,
        control: Fuelflowcounter1k,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fuelflowcounter1k { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fueltotalizer100<F>(
        &mut self,
        control: Fueltotalizer100,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fueltotalizer100 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fueltotalizer10k<F>(
        &mut self,
        control: Fueltotalizer10k,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fueltotalizer10k { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fueltotalizer1k<F>(
        &mut self,
        control: Fueltotalizer1k,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Fueltotalizer1k { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gear_handle<F>(
        &mut self,
        control: GearHandle,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(GearHandleValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::GearHandle { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gnd_jett_enable_sw<F>(
        &mut self,
        control: GndJettEnableSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(GndJettEnableSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::GndJettEnableSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gps_sw<F>(
        &mut self,
        control: GpsSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(GpsSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::GpsSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hdpt_sw_l<F>(
        &mut self,
        control: HdptSwL,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(HdptSwLValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HdptSwL { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hdpt_sw_r<F>(
        &mut self,
        control: HdptSwR,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(HdptSwRValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HdptSwR { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hide_stick<F>(
        &mut self,
        control: HideStick,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(HideStickValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HideStick { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hmcs_int_knb<F>(
        &mut self,
        control: HmcsIntKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HmcsIntKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hook_sw<F>(
        &mut self,
        control: HookSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(HookSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HookSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_horn_silence_btn<F>(
        &mut self,
        control: HornSilenceBtn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(HornSilenceBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HornSilenceBtn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hot_mic_sw<F>(
        &mut self,
        control: HotMicSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(HotMicSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HotMicSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_alt_sw<F>(
        &mut self,
        control: HudAltSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(HudAltSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudAltSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_brt_sw<F>(
        &mut self,
        control: HudBrtSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(HudBrtSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudBrtSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_ded_data_sw<F>(
        &mut self,
        control: HudDedDataSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(HudDedDataSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudDedDataSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_depress_ret_sw<F>(
        &mut self,
        control: HudDepressRetSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(HudDepressRetSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudDepressRetSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_fp_marker_sw<F>(
        &mut self,
        control: HudFpMarkerSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(HudFpMarkerSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudFpMarkerSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_scales_sw<F>(
        &mut self,
        control: HudScalesSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(HudScalesSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudScalesSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_speed_sw<F>(
        &mut self,
        control: HudSpeedSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(HudSpeedSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudSpeedSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hud_test_sw<F>(
        &mut self,
        control: HudTestSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(HudTestSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HudTestSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hydrazin_volume<F>(
        &mut self,
        control: HydrazinVolume,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HydrazinVolume { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_aa_mode_btn<F>(
        &mut self,
        control: IcpAaModeBtn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IcpAaModeBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpAaModeBtn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_ag_mode_btn<F>(
        &mut self,
        control: IcpAgModeBtn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IcpAgModeBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpAgModeBtn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_btn0<F>(
        &mut self,
        control: IcpBtn0,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IcpBtn0Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpBtn0 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_btn1<F>(
        &mut self,
        control: IcpBtn1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IcpBtn1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpBtn1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_btn2<F>(
        &mut self,
        control: IcpBtn2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IcpBtn2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpBtn2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_btn3<F>(
        &mut self,
        control: IcpBtn3,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IcpBtn3Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpBtn3 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_btn4<F>(
        &mut self,
        control: IcpBtn4,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IcpBtn4Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpBtn4 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_btn5<F>(
        &mut self,
        control: IcpBtn5,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IcpBtn5Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpBtn5 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_btn6<F>(
        &mut self,
        control: IcpBtn6,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IcpBtn6Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpBtn6 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_btn7<F>(
        &mut self,
        control: IcpBtn7,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IcpBtn7Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpBtn7 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_btn8<F>(
        &mut self,
        control: IcpBtn8,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IcpBtn8Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpBtn8 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_btn9<F>(
        &mut self,
        control: IcpBtn9,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IcpBtn9Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpBtn9 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_com1_btn<F>(
        &mut self,
        control: IcpCom1Btn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IcpCom1BtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpCom1Btn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_com2_btn<F>(
        &mut self,
        control: IcpCom2Btn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IcpCom2BtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpCom2Btn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_data_rtn_seq_sw<F>(
        &mut self,
        control: IcpDataRtnSeqSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpDataRtnSeqSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_data_up_dn_sw<F>(
        &mut self,
        control: IcpDataUpDnSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpDataUpDnSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_ded_sw<F>(
        &mut self,
        control: IcpDedSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpDedSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_drift_sw<F>(
        &mut self,
        control: IcpDriftSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpDriftSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_entr_btn<F>(
        &mut self,
        control: IcpEntrBtn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IcpEntrBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpEntrBtn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_flir_gain_sw<F>(
        &mut self,
        control: IcpFlirGainSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IcpFlirGainSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpFlirGainSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_flir_sw<F>(
        &mut self,
        control: IcpFlirSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpFlirSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_hud_brt_knb<F>(
        &mut self,
        control: IcpHudBrtKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpHudBrtKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_iff_btn<F>(
        &mut self,
        control: IcpIffBtn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IcpIffBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpIffBtn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_list_btn<F>(
        &mut self,
        control: IcpListBtn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IcpListBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpListBtn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_raster_brt_knb<F>(
        &mut self,
        control: IcpRasterBrtKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpRasterBrtKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_raster_contr_knb<F>(
        &mut self,
        control: IcpRasterContrKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpRasterContrKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_rcl_btn<F>(
        &mut self,
        control: IcpRclBtn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IcpRclBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpRclBtn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_reticle_depress_knb<F>(
        &mut self,
        control: IcpReticleDepressKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpReticleDepressKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_icp_wx_btn<F>(
        &mut self,
        control: IcpWxBtn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IcpWxBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IcpWxBtn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_ant_sel_sw<F>(
        &mut self,
        control: IffAntSelSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IffAntSelSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffAntSelSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_c_i_knb<F>(
        &mut self,
        control: IffCIKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IffCIKnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffCIKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_code_drum_digit1<F>(
        &mut self,
        control: IffCodeDrumDigit1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffCodeDrumDigit1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_code_drum_digit2<F>(
        &mut self,
        control: IffCodeDrumDigit2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffCodeDrumDigit2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_code_drum_digit3<F>(
        &mut self,
        control: IffCodeDrumDigit3,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffCodeDrumDigit3 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_code_drum_digit4<F>(
        &mut self,
        control: IffCodeDrumDigit4,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffCodeDrumDigit4 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_enable_sw<F>(
        &mut self,
        control: IffEnableSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IffEnableSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffEnableSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_id_btn<F>(
        &mut self,
        control: IffIdBtn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IffIdBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffIdBtn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_m1_sel1<F>(
        &mut self,
        control: IffM1Sel1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IffM1Sel1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffM1Sel1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_m1_sel2<F>(
        &mut self,
        control: IffM1Sel2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IffM1Sel2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffM1Sel2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_m3_sel1<F>(
        &mut self,
        control: IffM3Sel1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IffM3Sel1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffM3Sel1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_m3_sel2<F>(
        &mut self,
        control: IffM3Sel2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IffM3Sel2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffM3Sel2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_m4_code_sw<F>(
        &mut self,
        control: IffM4CodeSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IffM4CodeSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffM4CodeSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_m4_monitor_sw<F>(
        &mut self,
        control: IffM4MonitorSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IffM4MonitorSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffM4MonitorSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_m4_reply_sw<F>(
        &mut self,
        control: IffM4ReplySw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IffM4ReplySwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffM4ReplySw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_master_knb<F>(
        &mut self,
        control: IffMasterKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IffMasterKnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffMasterKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ils_pwr_knb<F>(
        &mut self,
        control: IlsPwrKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IlsPwrKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ins_knb<F>(
        &mut self,
        control: InsKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(InsKnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::InsKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_intercom_knb<F>(
        &mut self,
        control: IntercomKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IntercomKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_jfs_sw<F>(
        &mut self,
        control: JfsSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::JfsSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ky58_fill_knb<F>(
        &mut self,
        control: Ky58FillKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Ky58FillKnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ky58FillKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ky58_mode_knb<F>(
        &mut self,
        control: Ky58ModeKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Ky58ModeKnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ky58ModeKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ky58_pwr_knb<F>(
        &mut self,
        control: Ky58PwrKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Ky58PwrKnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ky58PwrKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ky58_vol_knb<F>(
        &mut self,
        control: Ky58VolKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Ky58VolKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_land_taxi_light_sw<F>(
        &mut self,
        control: LandTaxiLightSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(LandTaxiLightSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LandTaxiLightSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_laser_arm_sw<F>(
        &mut self,
        control: LaserArmSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(LaserArmSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LaserArmSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_le_flaps_sw<F>(
        &mut self,
        control: LeFlapsSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(LeFlapsSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LeFlapsSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_acft_batt_fail<F>(
        &mut self,
        control: LightAcftBattFail,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightAcftBattFail { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_active<F>(
        &mut self,
        control: LightActive,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightActive { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_aft_fuel_low<F>(
        &mut self,
        control: LightAftFuelLow,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightAftFuelLow { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_air<F>(
        &mut self,
        control: LightAir,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightAir { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_anti_skid<F>(
        &mut self,
        control: LightAntiSkid,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightAntiSkid { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_aoa_dn<F>(
        &mut self,
        control: LightAoaDn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightAoaDn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_aoa_mid<F>(
        &mut self,
        control: LightAoaMid,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightAoaMid { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_aoa_up<F>(
        &mut self,
        control: LightAoaUp,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightAoaUp { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ar_nws<F>(
        &mut self,
        control: LightArNws,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightArNws { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_atf_not<F>(
        &mut self,
        control: LightAtfNot,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightAtfNot { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_avionics_fault<F>(
        &mut self,
        control: LightAvionicsFault,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightAvionicsFault { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_buc<F>(
        &mut self,
        control: LightBuc,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightBuc { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_cabin_press<F>(
        &mut self,
        control: LightCabinPress,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightCabinPress { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_cadc<F>(
        &mut self,
        control: LightCadc,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightCadc { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_canopy<F>(
        &mut self,
        control: LightCanopy,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightCanopy { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_caution1<F>(
        &mut self,
        control: LightCaution1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightCaution1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_caution2<F>(
        &mut self,
        control: LightCaution2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightCaution2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_caution3<F>(
        &mut self,
        control: LightCaution3,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightCaution3 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_caution4<F>(
        &mut self,
        control: LightCaution4,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightCaution4 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_caution5<F>(
        &mut self,
        control: LightCaution5,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightCaution5 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_caution6<F>(
        &mut self,
        control: LightCaution6,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightCaution6 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_cmds_disp<F>(
        &mut self,
        control: LightCmdsDisp,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightCmdsDisp { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_cmds_go<F>(
        &mut self,
        control: LightCmdsGo,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightCmdsGo { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_cmds_no_go<F>(
        &mut self,
        control: LightCmdsNoGo,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightCmdsNoGo { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_cmds_rdy<F>(
        &mut self,
        control: LightCmdsRdy,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightCmdsRdy { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_consles<F>(
        &mut self,
        control: LightConsles,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightConsles { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_consles_flood<F>(
        &mut self,
        control: LightConslesFlood,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightConslesFlood { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_dbu_on<F>(
        &mut self,
        control: LightDbuOn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightDbuOn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_disc<F>(
        &mut self,
        control: LightDisc,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightDisc { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm1_a<F>(
        &mut self,
        control: LightEcm1A,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm1A { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm1_f<F>(
        &mut self,
        control: LightEcm1F,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm1F { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm1_s<F>(
        &mut self,
        control: LightEcm1S,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm1S { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm1_t<F>(
        &mut self,
        control: LightEcm1T,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm1T { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm2_a<F>(
        &mut self,
        control: LightEcm2A,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm2A { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm2_f<F>(
        &mut self,
        control: LightEcm2F,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm2F { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm2_s<F>(
        &mut self,
        control: LightEcm2S,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm2S { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm2_t<F>(
        &mut self,
        control: LightEcm2T,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm2T { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm3_a<F>(
        &mut self,
        control: LightEcm3A,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm3A { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm3_f<F>(
        &mut self,
        control: LightEcm3F,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm3F { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm3_s<F>(
        &mut self,
        control: LightEcm3S,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm3S { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm3_t<F>(
        &mut self,
        control: LightEcm3T,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm3T { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm4_a<F>(
        &mut self,
        control: LightEcm4A,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm4A { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm4_f<F>(
        &mut self,
        control: LightEcm4F,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm4F { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm4_s<F>(
        &mut self,
        control: LightEcm4S,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm4S { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm4_t<F>(
        &mut self,
        control: LightEcm4T,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm4T { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm5_a<F>(
        &mut self,
        control: LightEcm5A,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm5A { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm5_f<F>(
        &mut self,
        control: LightEcm5F,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm5F { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm5_s<F>(
        &mut self,
        control: LightEcm5S,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm5S { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm5_t<F>(
        &mut self,
        control: LightEcm5T,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcm5T { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm_a<F>(
        &mut self,
        control: LightEcmA,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcmA { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm_f<F>(
        &mut self,
        control: LightEcmF,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcmF { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm_frm_a<F>(
        &mut self,
        control: LightEcmFrmA,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcmFrmA { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm_frm_f<F>(
        &mut self,
        control: LightEcmFrmF,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcmFrmF { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm_frm_s<F>(
        &mut self,
        control: LightEcmFrmS,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcmFrmS { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm_frm_t<F>(
        &mut self,
        control: LightEcmFrmT,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcmFrmT { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm_s<F>(
        &mut self,
        control: LightEcmS,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcmS { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm_spl_a<F>(
        &mut self,
        control: LightEcmSplA,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcmSplA { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm_spl_f<F>(
        &mut self,
        control: LightEcmSplF,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcmSplF { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm_spl_s<F>(
        &mut self,
        control: LightEcmSplS,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcmSplS { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm_spl_t<F>(
        &mut self,
        control: LightEcmSplT,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcmSplT { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_ecm_t<F>(
        &mut self,
        control: LightEcmT,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEcmT { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_edge<F>(
        &mut self,
        control: LightEdge,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEdge { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_eec<F>(
        &mut self,
        control: LightEec,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEec { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_elec<F>(
        &mut self,
        control: LightElec,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightElec { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_elec_sys<F>(
        &mut self,
        control: LightElecSys,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightElecSys { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_eng_fire<F>(
        &mut self,
        control: LightEngFire,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEngFire { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_engine<F>(
        &mut self,
        control: LightEngine,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEngine { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_engine_fault<F>(
        &mut self,
        control: LightEngineFault,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEngineFault { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_epu<F>(
        &mut self,
        control: LightEpu,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEpu { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_epu_gen<F>(
        &mut self,
        control: LightEpuGen,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEpuGen { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_epu_pmg<F>(
        &mut self,
        control: LightEpuPmg,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEpuPmg { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_equip_hot<F>(
        &mut self,
        control: LightEquipHot,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightEquipHot { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_fl_fail<F>(
        &mut self,
        control: LightFlFail,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightFlFail { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_fl_run<F>(
        &mut self,
        control: LightFlRun,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightFlRun { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_flcs<F>(
        &mut self,
        control: LightFlcs,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightFlcs { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_flcs_fault<F>(
        &mut self,
        control: LightFlcsFault,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightFlcsFault { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_flcs_pmg<F>(
        &mut self,
        control: LightFlcsPmg,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightFlcsPmg { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_flcs_pwr_a<F>(
        &mut self,
        control: LightFlcsPwrA,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightFlcsPwrA { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_flcs_pwr_b<F>(
        &mut self,
        control: LightFlcsPwrB,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightFlcsPwrB { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_flcs_pwr_c<F>(
        &mut self,
        control: LightFlcsPwrC,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightFlcsPwrC { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_flcs_pwr_d<F>(
        &mut self,
        control: LightFlcsPwrD,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightFlcsPwrD { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_flcs_rly<F>(
        &mut self,
        control: LightFlcsRly,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightFlcsRly { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_fuel_oil_hot<F>(
        &mut self,
        control: LightFuelOilHot,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightFuelOilHot { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_fwd_fuel_low<F>(
        &mut self,
        control: LightFwdFuelLow,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightFwdFuelLow { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_gear_l<F>(
        &mut self,
        control: LightGearL,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightGearL { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_gear_n<F>(
        &mut self,
        control: LightGearN,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightGearN { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_gear_r<F>(
        &mut self,
        control: LightGearR,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightGearR { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_gear_warn<F>(
        &mut self,
        control: LightGearWarn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightGearWarn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_hook<F>(
        &mut self,
        control: LightHook,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightHook { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_hyd_oil_press<F>(
        &mut self,
        control: LightHydOilPress,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightHydOilPress { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_hydrazn<F>(
        &mut self,
        control: LightHydrazn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightHydrazn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_iff<F>(
        &mut self,
        control: LightIff,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightIff { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_inlet_icing<F>(
        &mut self,
        control: LightInletIcing,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightInletIcing { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_inst_pnl<F>(
        &mut self,
        control: LightInstPnl,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightInstPnl { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_inst_pnl_flood<F>(
        &mut self,
        control: LightInstPnlFlood,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightInstPnlFlood { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_jfs_run<F>(
        &mut self,
        control: LightJfsRun,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightJfsRun { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_main_gen<F>(
        &mut self,
        control: LightMainGen,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightMainGen { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_marker_beacon<F>(
        &mut self,
        control: LightMarkerBeacon,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightMarkerBeacon { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_master_caution<F>(
        &mut self,
        control: LightMasterCaution,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightMasterCaution { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_nuclear<F>(
        &mut self,
        control: LightNuclear,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightNuclear { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_nws_fail<F>(
        &mut self,
        control: LightNwsFail,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightNwsFail { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_obogs<F>(
        &mut self,
        control: LightObogs,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightObogs { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_overheat<F>(
        &mut self,
        control: LightOverheat,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightOverheat { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_oxy_low<F>(
        &mut self,
        control: LightOxyLow,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightOxyLow { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_probe_heat<F>(
        &mut self,
        control: LightProbeHeat,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightProbeHeat { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_radar_alt<F>(
        &mut self,
        control: LightRadarAlt,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightRadarAlt { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_rdy<F>(
        &mut self,
        control: LightRdy,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightRdy { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_rwr_act_power<F>(
        &mut self,
        control: LightRwrActPower,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightRwrActPower { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_rwr_activity<F>(
        &mut self,
        control: LightRwrActivity,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightRwrActivity { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_rwr_alt<F>(
        &mut self,
        control: LightRwrAlt,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightRwrAlt { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_rwr_alt_low<F>(
        &mut self,
        control: LightRwrAltLow,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightRwrAltLow { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_rwr_handoff_h<F>(
        &mut self,
        control: LightRwrHandoffH,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightRwrHandoffH { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_rwr_handoff_up<F>(
        &mut self,
        control: LightRwrHandoffUp,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightRwrHandoffUp { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_rwr_mode_open<F>(
        &mut self,
        control: LightRwrModeOpen,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightRwrModeOpen { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_rwr_mode_pri<F>(
        &mut self,
        control: LightRwrModePri,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightRwrModePri { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_rwr_msl_launch<F>(
        &mut self,
        control: LightRwrMslLaunch,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightRwrMslLaunch { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_rwr_power<F>(
        &mut self,
        control: LightRwrPower,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightRwrPower { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_rwr_search<F>(
        &mut self,
        control: LightRwrSearch,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightRwrSearch { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_rwr_ship_unk<F>(
        &mut self,
        control: LightRwrShipUnk,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightRwrShipUnk { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_rwr_systest<F>(
        &mut self,
        control: LightRwrSystest,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightRwrSystest { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_rwr_tgtsep_dn<F>(
        &mut self,
        control: LightRwrTgtsepDn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightRwrTgtsepDn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_rwr_tgtsep_up<F>(
        &mut self,
        control: LightRwrTgtsepUp,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightRwrTgtsepUp { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_seat_not<F>(
        &mut self,
        control: LightSeatNot,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightSeatNot { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_sec<F>(
        &mut self,
        control: LightSec,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightSec { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_stby<F>(
        &mut self,
        control: LightStby,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightStby { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_stby_gen<F>(
        &mut self,
        control: LightStbyGen,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightStbyGen { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_stores_config<F>(
        &mut self,
        control: LightStoresConfig,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightStoresConfig { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_tf_fail<F>(
        &mut self,
        control: LightTfFail,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightTfFail { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_to_flcs<F>(
        &mut self,
        control: LightToFlcs,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightToFlcs { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_light_to_ldg_config<F>(
        &mut self,
        control: LightToLdgConfig,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LightToLdgConfig { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mach_indicator<F>(
        &mut self,
        control: MachIndicator,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MachIndicator { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_main_pwr_sw<F>(
        &mut self,
        control: MainPwrSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MainPwrSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MainPwrSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mal_ind_lts_brt_sw<F>(
        &mut self,
        control: MalIndLtsBrtSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MalIndLtsBrtSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mal_ind_lts_test<F>(
        &mut self,
        control: MalIndLtsTest,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MalIndLtsTestValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MalIndLtsTest { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_man_tf_flyup_sw<F>(
        &mut self,
        control: ManTfFlyupSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ManTfFlyupSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ManTfFlyupSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_manual_pitch_sw<F>(
        &mut self,
        control: ManualPitchSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ManualPitchSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ManualPitchSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_map_sw<F>(
        &mut self,
        control: MapSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MapSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MapSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_master_arm_sw<F>(
        &mut self,
        control: MasterArmSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MasterArmSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MasterArmSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_master_caution<F>(
        &mut self,
        control: MasterCaution,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MasterCautionValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MasterCaution { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_master_light_sw<F>(
        &mut self,
        control: MasterLightSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MasterLightSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MasterLightSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_max_airspeed<F>(
        &mut self,
        control: MaxAirspeed,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MaxAirspeed { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_max_pwr_sw<F>(
        &mut self,
        control: MaxPwrSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MaxPwrSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MaxPwrSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l1<F>(
        &mut self,
        control: MfdL1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdL1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l10<F>(
        &mut self,
        control: MfdL10,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdL10Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL10 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l11<F>(
        &mut self,
        control: MfdL11,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdL11Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL11 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l12<F>(
        &mut self,
        control: MfdL12,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdL12Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL12 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l13<F>(
        &mut self,
        control: MfdL13,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdL13Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL13 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l14<F>(
        &mut self,
        control: MfdL14,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdL14Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL14 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l15<F>(
        &mut self,
        control: MfdL15,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdL15Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL15 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l16<F>(
        &mut self,
        control: MfdL16,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdL16Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL16 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l17<F>(
        &mut self,
        control: MfdL17,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdL17Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL17 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l18<F>(
        &mut self,
        control: MfdL18,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdL18Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL18 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l19<F>(
        &mut self,
        control: MfdL19,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdL19Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL19 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l2<F>(
        &mut self,
        control: MfdL2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdL2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l20<F>(
        &mut self,
        control: MfdL20,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdL20Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL20 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l3<F>(
        &mut self,
        control: MfdL3,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdL3Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL3 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l4<F>(
        &mut self,
        control: MfdL4,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdL4Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL4 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l5<F>(
        &mut self,
        control: MfdL5,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdL5Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL5 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l6<F>(
        &mut self,
        control: MfdL6,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdL6Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL6 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l7<F>(
        &mut self,
        control: MfdL7,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdL7Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL7 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l8<F>(
        &mut self,
        control: MfdL8,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdL8Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL8 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l9<F>(
        &mut self,
        control: MfdL9,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdL9Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdL9 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l_brt_sw<F>(
        &mut self,
        control: MfdLBrtSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdLBrtSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdLBrtSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l_con_sw<F>(
        &mut self,
        control: MfdLConSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdLConSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdLConSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l_gain_sw<F>(
        &mut self,
        control: MfdLGainSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdLGainSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdLGainSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_l_sym_sw<F>(
        &mut self,
        control: MfdLSymSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdLSymSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdLSymSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r1<F>(
        &mut self,
        control: MfdR1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdR1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r10<F>(
        &mut self,
        control: MfdR10,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdR10Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR10 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r11<F>(
        &mut self,
        control: MfdR11,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdR11Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR11 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r12<F>(
        &mut self,
        control: MfdR12,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdR12Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR12 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r13<F>(
        &mut self,
        control: MfdR13,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdR13Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR13 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r14<F>(
        &mut self,
        control: MfdR14,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdR14Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR14 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r15<F>(
        &mut self,
        control: MfdR15,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdR15Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR15 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r16<F>(
        &mut self,
        control: MfdR16,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdR16Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR16 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r17<F>(
        &mut self,
        control: MfdR17,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdR17Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR17 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r18<F>(
        &mut self,
        control: MfdR18,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdR18Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR18 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r19<F>(
        &mut self,
        control: MfdR19,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdR19Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR19 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r2<F>(
        &mut self,
        control: MfdR2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdR2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r20<F>(
        &mut self,
        control: MfdR20,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdR20Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR20 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r3<F>(
        &mut self,
        control: MfdR3,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdR3Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR3 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r4<F>(
        &mut self,
        control: MfdR4,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdR4Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR4 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r5<F>(
        &mut self,
        control: MfdR5,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdR5Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR5 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r6<F>(
        &mut self,
        control: MfdR6,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdR6Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR6 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r7<F>(
        &mut self,
        control: MfdR7,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdR7Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR7 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r8<F>(
        &mut self,
        control: MfdR8,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdR8Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR8 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r9<F>(
        &mut self,
        control: MfdR9,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdR9Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdR9 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r_brt_sw<F>(
        &mut self,
        control: MfdRBrtSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdRBrtSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdRBrtSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r_con_sw<F>(
        &mut self,
        control: MfdRConSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdRConSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdRConSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r_gain_sw<F>(
        &mut self,
        control: MfdRGainSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdRGainSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdRGainSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_r_sym_sw<F>(
        &mut self,
        control: MfdRSymSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdRSymSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdRSymSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mfd_sw<F>(
        &mut self,
        control: MfdSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MfdSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MfdSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mids_lvt_knb<F>(
        &mut self,
        control: MidsLvtKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MidsLvtKnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MidsLvtKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_mmc_pwr_sw<F>(
        &mut self,
        control: MmcPwrSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MmcPwrSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MmcPwrSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_msl_knb<F>(
        &mut self,
        control: MslKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MslKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_obogs_sw<F>(
        &mut self,
        control: ObogsSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ObogsSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ObogsSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oxy_diluter_lvr<F>(
        &mut self,
        control: OxyDiluterLvr,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(OxyDiluterLvrValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::OxyDiluterLvr { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oxy_emerg_lvr<F>(
        &mut self,
        control: OxyEmergLvr,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OxyEmergLvr { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oxy_supply_lvr<F>(
        &mut self,
        control: OxySupplyLvr,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(OxySupplyLvrValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::OxySupplyLvr { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oxygen_pressure<F>(
        &mut self,
        control: OxygenPressure,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OxygenPressure { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pitch_trim<F>(
        &mut self,
        control: PitchTrim,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::PitchTrim { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pitchtrimind<F>(
        &mut self,
        control: Pitchtrimind,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Pitchtrimind { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_plain_cipher_sw<F>(
        &mut self,
        control: PlainCipherSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(PlainCipherSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::PlainCipherSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pos_flash_light_sw<F>(
        &mut self,
        control: PosFlashLightSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(PosFlashLightSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::PosFlashLightSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pos_fuselage_light_sw<F>(
        &mut self,
        control: PosFuselageLightSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(PosFuselageLightSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::PosFuselageLightSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pos_wing_tail_light_sw<F>(
        &mut self,
        control: PosWingTailLightSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(PosWingTailLightSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::PosWingTailLightSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pri_consoles_brt_knb<F>(
        &mut self,
        control: PriConsolesBrtKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::PriConsolesBrtKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pri_data_display_brt_knb<F>(
        &mut self,
        control: PriDataDisplayBrtKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::PriDataDisplayBrtKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pri_inst_pnl_brt_knb<F>(
        &mut self,
        control: PriInstPnlBrtKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::PriInstPnlBrtKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_probe_heat_sw<F>(
        &mut self,
        control: ProbeHeatSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ProbeHeatSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rdr_alt_pwr_sw<F>(
        &mut self,
        control: RdrAltPwrSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RdrAltPwrSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RdrAltPwrSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rf_sw<F>(
        &mut self,
        control: RfSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RfSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RfSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_roll_trim<F>(
        &mut self,
        control: RollTrim,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RollTrim { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rolltrimind<F>(
        &mut self,
        control: Rolltrimind,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Rolltrimind { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_act_pwr_btn<F>(
        &mut self,
        control: RwrActPwrBtn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RwrActPwrBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrActPwrBtn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_alt_btn<F>(
        &mut self,
        control: RwrAltBtn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RwrAltBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrAltBtn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_handoff_btn<F>(
        &mut self,
        control: RwrHandoffBtn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RwrHandoffBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrHandoffBtn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_ind_dim_knb<F>(
        &mut self,
        control: RwrIndDimKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrIndDimKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_intens_knb<F>(
        &mut self,
        control: RwrIntensKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrIntensKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_launch_btn<F>(
        &mut self,
        control: RwrLaunchBtn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RwrLaunchBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrLaunchBtn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_mode_btn<F>(
        &mut self,
        control: RwrModeBtn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RwrModeBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrModeBtn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_pwr_btn<F>(
        &mut self,
        control: RwrPwrBtn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RwrPwrBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrPwrBtn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_search_btn<F>(
        &mut self,
        control: RwrSearchBtn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RwrSearchBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrSearchBtn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_sys_test_btn<F>(
        &mut self,
        control: RwrSysTestBtn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RwrSysTestBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrSysTestBtn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_t_btn<F>(
        &mut self,
        control: RwrTBtn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RwrTBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrTBtn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rwr_unknown_ship_btn<F>(
        &mut self,
        control: RwrUnknownShipBtn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RwrUnknownShipBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RwrUnknownShipBtn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sai_aircraftreferencesymbol<F>(
        &mut self,
        control: SaiAircraftreferencesymbol,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SaiAircraftreferencesymbol { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sai_bank<F>(
        &mut self,
        control: SaiBank,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SaiBank { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sai_bank_arrow<F>(
        &mut self,
        control: SaiBankArrow,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SaiBankArrow { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sai_cage<F>(
        &mut self,
        control: SaiCage,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(SaiCageValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SaiCage { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sai_knb_arrow<F>(
        &mut self,
        control: SaiKnbArrow,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SaiKnbArrow { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sai_off_flag<F>(
        &mut self,
        control: SaiOffFlag,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SaiOffFlag { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sai_pitch<F>(
        &mut self,
        control: SaiPitch,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SaiPitch { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sai_pitch_trim<F>(
        &mut self,
        control: SaiPitchTrim,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SaiPitchTrim { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_seat_adj<F>(
        &mut self,
        control: SeatAdj,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SeatAdj { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_seat_eject_handle<F>(
        &mut self,
        control: SeatEjectHandle,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(SeatEjectHandleValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SeatEjectHandle { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_seat_eject_safe<F>(
        &mut self,
        control: SeatEjectSafe,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(SeatEjectSafeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SeatEjectSafe { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_seat_height<F>(
        &mut self,
        control: SeatHeight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SeatHeight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sec_voice_knb<F>(
        &mut self,
        control: SecVoiceKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SecVoiceKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_set_airspeed<F>(
        &mut self,
        control: SetAirspeed,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SetAirspeed { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_speedbrake_indicator<F>(
        &mut self,
        control: SpeedbrakeIndicator,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SpeedbrakeIndicator { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_st_sta_sw<F>(
        &mut self,
        control: StStaSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(StStaSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::StStaSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_stores_config_sw<F>(
        &mut self,
        control: StoresConfigSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(StoresConfigSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::StoresConfigSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sysa_pressure<F>(
        &mut self,
        control: SysaPressure,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SysaPressure { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sysb_pressure<F>(
        &mut self,
        control: SysbPressure,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SysbPressure { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tacan_knb<F>(
        &mut self,
        control: TacanKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TacanKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tank_interting_sw<F>(
        &mut self,
        control: TankIntertingSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(TankIntertingSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::TankIntertingSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_temp_knb<F>(
        &mut self,
        control: TempKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(TempKnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::TempKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_tf_knb<F>(
        &mut self,
        control: TfKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TfKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_threat_knb<F>(
        &mut self,
        control: ThreatKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ThreatKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_trim_ap_disc_sw<F>(
        &mut self,
        control: TrimApDiscSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(TrimApDiscSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::TrimApDiscSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ufc_sw<F>(
        &mut self,
        control: UfcSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(UfcSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UfcSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_ant_sel_sw<F>(
        &mut self,
        control: UhfAntSelSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(UhfAntSelSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfAntSelSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_chan_disp<F>(
        &mut self,
        control: UhfChanDisp,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfChanDisp { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_chan_knb<F>(
        &mut self,
        control: UhfChanKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfChanKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_door<F>(
        &mut self,
        control: UhfDoor,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(UhfDoorValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfDoor { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_freq0025_knb<F>(
        &mut self,
        control: UhfFreq0025Knb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(UhfFreq0025KnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfFreq0025Knb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_freq01_knb<F>(
        &mut self,
        control: UhfFreq01Knb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(UhfFreq01KnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfFreq01Knb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_freq100_knb<F>(
        &mut self,
        control: UhfFreq100Knb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(UhfFreq100KnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfFreq100Knb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_freq10_knb<F>(
        &mut self,
        control: UhfFreq10Knb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(UhfFreq10KnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfFreq10Knb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_freq1_knb<F>(
        &mut self,
        control: UhfFreq1Knb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(UhfFreq1KnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfFreq1Knb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_freq_disp<F>(
        &mut self,
        control: UhfFreqDisp,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfFreqDisp { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_func_knb<F>(
        &mut self,
        control: UhfFuncKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(UhfFuncKnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfFuncKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_mode_knb<F>(
        &mut self,
        control: UhfModeKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(UhfModeKnbValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfModeKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_squelch_sw<F>(
        &mut self,
        control: UhfSquelchSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(UhfSquelchSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfSquelchSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_status_btn<F>(
        &mut self,
        control: UhfStatusBtn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(UhfStatusBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfStatusBtn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_test_btn<F>(
        &mut self,
        control: UhfTestBtn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(UhfTestBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfTestBtn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_tone_btn<F>(
        &mut self,
        control: UhfToneBtn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(UhfToneBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfToneBtn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_vol_knb<F>(
        &mut self,
        control: UhfVolKnb,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfVolKnb { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vms_inhibit_sw<F>(
        &mut self,
        control: VmsInhibitSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(VmsInhibitSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::VmsInhibitSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vvi<F>(
        &mut self,
        control: Vvi,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Vvi { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_yaw_trim<F>(
        &mut self,
        control: YawTrim,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::YawTrim { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_zeroize_sw<F>(
        &mut self,
        control: ZeroizeSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ZeroizeSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ZeroizeSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_zeroize_sw_cover<F>(
        &mut self,
        control: ZeroizeSwCover,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ZeroizeSwCoverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ZeroizeSwCover { control, callback })
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
