//! Generated from model-definitions/UH-1H.json.
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

/// AC GEN 1 Current
/// Output: gauge position
pub struct AcGen1Current(AnalogGauge);
impl AcGen1Current {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5134, 65535, 0))
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
pub struct AcGen2Current(AnalogGauge);
impl AcGen2Current {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5136, 65535, 0))
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
pub struct AcV(AnalogGauge);
impl AcV {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5230, 65535, 0))
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
pub enum AcVmSrcValue {
    Select1,
    Select2,
    Select3,
}
pub type AcVmSrcValueCallback<'a> = &'a mut dyn FnMut(AcVmSrcValue);
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
pub struct AcVmSrc(Selector);
impl AcVmSrc {
    pub const fn new() -> Self {
        Self(Selector::new(5294, 24, 3))
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

/// AC Voltage
/// Output: gauge position
pub struct AcVoltage(AnalogGauge);
impl AcVoltage {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5122, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdfBandValue {
    Select1,
    Select2,
    Select3,
}
pub type AdfBandValueCallback<'a> = &'a mut dyn FnMut(AdfBandValue);
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
pub struct AdfBand(Selector);
impl AdfBand {
    pub const fn new() -> Self {
        Self(Selector::new(5372, 192, 6))
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
pub type AdfBfoSwValueCallback<'a> = &'a mut dyn FnMut(AdfBfoSwValue);
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
pub struct AdfBfoSw(Selector);
impl AdfBfoSw {
    pub const fn new() -> Self {
        Self(Selector::new(5372, 8, 3))
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
pub struct AdfFreq(AnalogGauge);
impl AdfFreq {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5158, 65535, 0))
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
pub struct AdfGain(LimitedDial);
impl AdfGain {
    pub const fn new() -> Self {
        Self(LimitedDial::new(5374, 65535, 0))
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
pub type AdfLoopLrValueCallback<'a> = &'a mut dyn FnMut(AdfLoopLrValue);
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
pub struct AdfLoopLr(Selector);
impl AdfLoopLr {
    pub const fn new() -> Self {
        Self(Selector::new(5372, 7, 0))
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
pub type AdfModeValueCallback<'a> = &'a mut dyn FnMut(AdfModeValue);
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
pub struct AdfMode(Selector);
impl AdfMode {
    pub const fn new() -> Self {
        Self(Selector::new(5372, 48, 4))
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
pub struct AdfSignal(AnalogGauge);
impl AdfSignal {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5160, 65535, 0))
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
pub struct AdfTune(AnalogDial);
impl AdfTune {
    pub const fn new() -> Self {
        Self(AnalogDial::new(5376, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AdfVorSwValue {
    Select1,
    Select2,
}
pub type AdfVorSwValueCallback<'a> = &'a mut dyn FnMut(AdfVorSwValue);
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
pub struct AdfVorSw(Selector);
impl AdfVorSw {
    pub const fn new() -> Self {
        Self(Selector::new(5378, 512, 9))
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

/// Attitude Pitch Shift
/// Output: gauge position
pub struct AdiPitchShift(AnalogGauge);
impl AdiPitchShift {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5266, 65535, 0))
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

/// ADI Pitch Trim Knob
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct AdiPitchTrim(LimitedDial);
impl AdiPitchTrim {
    pub const fn new() -> Self {
        Self(LimitedDial::new(5324, 65535, 0))
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
pub struct AdiPitchTrimCplt(LimitedDial);
impl AdiPitchTrimCplt {
    pub const fn new() -> Self {
        Self(LimitedDial::new(5320, 65535, 0))
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
pub struct AdiRollTrim(LimitedDial);
impl AdiRollTrim {
    pub const fn new() -> Self {
        Self(LimitedDial::new(5322, 65535, 0))
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
pub struct AltAdjCplt(AnalogDial);
impl AltAdjCplt {
    pub const fn new() -> Self {
        Self(AnalogDial::new(5326, 65535, 0))
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
pub struct AltAdjPlt(AnalogDial);
impl AltAdjPlt {
    pub const fn new() -> Self {
        Self(AnalogDial::new(5328, 65535, 0))
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

/// Anti-Ice Amperemeter
/// Output: gauge position
pub struct AntiIceAmp(AnalogGauge);
impl AntiIceAmp {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5138, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnticollLtsSwValue {
    Select1,
    Select2,
}
pub type AnticollLtsSwValueCallback<'a> = &'a mut dyn FnMut(AnticollLtsSwValue);
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
pub struct AnticollLtsSw(Selector);
impl AnticollLtsSw {
    pub const fn new() -> Self {
        Self(Selector::new(5372, 16384, 14))
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

/// ARMED Indicator (red)
/// Output: 0 if light is off, 1 if light is on
pub struct ArmedInd(Led);
impl ArmedInd {
    pub const fn new() -> Self {
        Self(Led::new(5142, 16, 4))
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
pub enum BatSwValue {
    Select1,
    Select2,
}
pub type BatSwValueCallback<'a> = &'a mut dyn FnMut(BatSwValue);
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
pub struct BatSw(Selector);
impl BatSw {
    pub const fn new() -> Self {
        Self(Selector::new(5142, 8192, 13))
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
pub enum BeaconSensingSwValue {
    Select1,
    Select2,
}
pub type BeaconSensingSwValueCallback<'a> = &'a mut dyn FnMut(BeaconSensingSwValue);
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
pub struct BeaconSensingSw(Selector);
impl BeaconSensingSw {
    pub const fn new() -> Self {
        Self(Selector::new(5356, 32768, 15))
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
pub struct BeaconVol(LimitedDial);
impl BeaconVol {
    pub const fn new() -> Self {
        Self(LimitedDial::new(5370, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BleedAirSwValue {
    Select1,
    Select2,
    Select3,
    Select4,
    Select5,
}
pub type BleedAirSwValueCallback<'a> = &'a mut dyn FnMut(BleedAirSwValue);
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
pub struct BleedAirSw(Selector);
impl BleedAirSw {
    pub const fn new() -> Self {
        Self(Selector::new(5378, 448, 6))
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
pub struct BrtConsole(LimitedDial);
impl BrtConsole {
    pub const fn new() -> Self {
        Self(LimitedDial::new(5380, 65535, 0))
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
pub struct BrtCopilot(LimitedDial);
impl BrtCopilot {
    pub const fn new() -> Self {
        Self(LimitedDial::new(5388, 65535, 0))
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
pub struct BrtEngine(LimitedDial);
impl BrtEngine {
    pub const fn new() -> Self {
        Self(LimitedDial::new(5386, 65535, 0))
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
pub struct BrtPed(LimitedDial);
impl BrtPed {
    pub const fn new() -> Self {
        Self(LimitedDial::new(5382, 65535, 0))
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
pub struct BrtPilot(LimitedDial);
impl BrtPilot {
    pub const fn new() -> Self {
        Self(LimitedDial::new(5390, 65535, 0))
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
pub struct BrtSec(LimitedDial);
impl BrtSec {
    pub const fn new() -> Self {
        Self(LimitedDial::new(5384, 65535, 0))
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
pub enum CargoRelCpValue {
    Select1,
    Select2,
}
pub type CargoRelCpValueCallback<'a> = &'a mut dyn FnMut(CargoRelCpValue);
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
pub struct CargoRelCp(Selector);
impl CargoRelCp {
    pub const fn new() -> Self {
        Self(Selector::new(5432, 256, 8))
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
pub type CargoRelPValueCallback<'a> = &'a mut dyn FnMut(CargoRelPValue);
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
pub struct CargoRelP(Selector);
impl CargoRelP {
    pub const fn new() -> Self {
        Self(Selector::new(5432, 128, 7))
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
pub enum CargoRelSafeValue {
    Select1,
    Select2,
}
pub type CargoRelSafeValueCallback<'a> = &'a mut dyn FnMut(CargoRelSafeValue);
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
pub struct CargoRelSafe(Selector);
impl CargoRelSafe {
    pub const fn new() -> Self {
        Self(Selector::new(5432, 512, 9))
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
pub enum Cb28vValue {
    Select1,
    Select2,
}
pub type Cb28vValueCallback<'a> = &'a mut dyn FnMut(Cb28vValue);
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
pub struct Cb28v(Selector);
impl Cb28v {
    pub const fn new() -> Self {
        Self(Selector::new(5302, 1024, 10))
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
pub type CbAirValveValueCallback<'a> = &'a mut dyn FnMut(CbAirValveValue);
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
pub struct CbAirValve(Selector);
impl CbAirValve {
    pub const fn new() -> Self {
        Self(Selector::new(5298, 1024, 10))
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
pub type CbAmRadioValueCallback<'a> = &'a mut dyn FnMut(CbAmRadioValue);
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
pub struct CbAmRadio(Selector);
impl CbAmRadio {
    pub const fn new() -> Self {
        Self(Selector::new(5296, 32, 5))
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
pub type CbAntiIceValueCallback<'a> = &'a mut dyn FnMut(CbAntiIceValue);
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
pub struct CbAntiIce(Selector);
impl CbAntiIce {
    pub const fn new() -> Self {
        Self(Selector::new(5298, 8192, 13))
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
pub type CbAnticollValueCallback<'a> = &'a mut dyn FnMut(CbAnticollValue);
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
pub struct CbAnticoll(Selector);
impl CbAnticoll {
    pub const fn new() -> Self {
        Self(Selector::new(5298, 1, 0))
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
pub type CbArc102ValueCallback<'a> = &'a mut dyn FnMut(CbArc102Value);
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
pub struct CbArc102(Selector);
impl CbArc102 {
    pub const fn new() -> Self {
        Self(Selector::new(5294, 32768, 15))
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
pub type CbCableValueCallback<'a> = &'a mut dyn FnMut(CbCableValue);
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
pub struct CbCable(Selector);
impl CbCable {
    pub const fn new() -> Self {
        Self(Selector::new(5296, 256, 8))
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
pub type CbCautionLtsValueCallback<'a> = &'a mut dyn FnMut(CbCautionLtsValue);
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
pub struct CbCautionLts(Selector);
impl CbCautionLts {
    pub const fn new() -> Self {
        Self(Selector::new(5298, 32, 5))
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
pub type CbCockpitLtsValueCallback<'a> = &'a mut dyn FnMut(CbCockpitLtsValue);
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
pub struct CbCockpitLts(Selector);
impl CbCockpitLts {
    pub const fn new() -> Self {
        Self(Selector::new(5298, 16, 4))
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
pub type CbConsoleLtsValueCallback<'a> = &'a mut dyn FnMut(CbConsoleLtsValue);
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
pub struct CbConsoleLts(Selector);
impl CbConsoleLts {
    pub const fn new() -> Self {
        Self(Selector::new(5298, 64, 6))
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
pub type CbCpltAttd1ValueCallback<'a> = &'a mut dyn FnMut(CbCpltAttd1Value);
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
pub struct CbCpltAttd1(Selector);
impl CbCpltAttd1 {
    pub const fn new() -> Self {
        Self(Selector::new(5302, 64, 6))
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
pub type CbCpltAttd2ValueCallback<'a> = &'a mut dyn FnMut(CbCpltAttd2Value);
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
pub struct CbCpltAttd2(Selector);
impl CbCpltAttd2 {
    pub const fn new() -> Self {
        Self(Selector::new(5302, 128, 7))
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
pub type CbCrsindValueCallback<'a> = &'a mut dyn FnMut(CbCrsindValue);
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
pub struct CbCrsind(Selector);
impl CbCrsind {
    pub const fn new() -> Self {
        Self(Selector::new(5304, 1, 0))
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
pub type CbDomeLtsValueCallback<'a> = &'a mut dyn FnMut(CbDomeLtsValue);
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
pub struct CbDomeLts(Selector);
impl CbDomeLts {
    pub const fn new() -> Self {
        Self(Selector::new(5298, 8, 3))
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
pub type CbEngValueCallback<'a> = &'a mut dyn FnMut(CbEngValue);
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
pub struct CbEng(Selector);
impl CbEng {
    pub const fn new() -> Self {
        Self(Selector::new(5302, 32768, 15))
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
pub type CbFailRelayValueCallback<'a> = &'a mut dyn FnMut(CbFailRelayValue);
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
pub struct CbFailRelay(Selector);
impl CbFailRelay {
    pub const fn new() -> Self {
        Self(Selector::new(5302, 2048, 11))
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
pub type CbFireValueCallback<'a> = &'a mut dyn FnMut(CbFireValue);
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
pub struct CbFire(Selector);
impl CbFire {
    pub const fn new() -> Self {
        Self(Selector::new(5298, 16384, 14))
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
pub type CbFm2RadioValueCallback<'a> = &'a mut dyn FnMut(CbFm2RadioValue);
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
pub struct CbFm2Radio(Selector);
impl CbFm2Radio {
    pub const fn new() -> Self {
        Self(Selector::new(5296, 16, 4))
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
pub type CbFmRadioValueCallback<'a> = &'a mut dyn FnMut(CbFmRadioValue);
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
pub struct CbFmRadio(Selector);
impl CbFmRadio {
    pub const fn new() -> Self {
        Self(Selector::new(5296, 4, 2))
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
pub type CbFuelValueCallback<'a> = &'a mut dyn FnMut(CbFuelValue);
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
pub struct CbFuel(Selector);
impl CbFuel {
    pub const fn new() -> Self {
        Self(Selector::new(5302, 4096, 12))
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
pub type CbFuelQtyValueCallback<'a> = &'a mut dyn FnMut(CbFuelQtyValue);
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
pub struct CbFuelQty(Selector);
impl CbFuelQty {
    pub const fn new() -> Self {
        Self(Selector::new(5302, 512, 9))
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
pub type CbFuelTransValueCallback<'a> = &'a mut dyn FnMut(CbFuelTransValue);
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
pub struct CbFuelTrans(Selector);
impl CbFuelTrans {
    pub const fn new() -> Self {
        Self(Selector::new(5300, 16384, 14))
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
pub type CbFuelValvesValueCallback<'a> = &'a mut dyn FnMut(CbFuelValvesValue);
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
pub struct CbFuelValves(Selector);
impl CbFuelValves {
    pub const fn new() -> Self {
        Self(Selector::new(5300, 32768, 15))
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
pub type CbFusLtsValueCallback<'a> = &'a mut dyn FnMut(CbFusLtsValue);
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
pub struct CbFusLts(Selector);
impl CbFusLts {
    pub const fn new() -> Self {
        Self(Selector::new(5298, 2, 1))
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
pub type CbGenBusValueCallback<'a> = &'a mut dyn FnMut(CbGenBusValue);
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
pub struct CbGenBus(Selector);
impl CbGenBus {
    pub const fn new() -> Self {
        Self(Selector::new(5300, 512, 9))
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
pub type CbGovValueCallback<'a> = &'a mut dyn FnMut(CbGovValue);
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
pub struct CbGov(Selector);
impl CbGov {
    pub const fn new() -> Self {
        Self(Selector::new(5300, 2048, 11))
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
pub type CbGyroValueCallback<'a> = &'a mut dyn FnMut(CbGyroValue);
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
pub struct CbGyro(Selector);
impl CbGyro {
    pub const fn new() -> Self {
        Self(Selector::new(5302, 256, 8))
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
pub type CbHeatedBlanket1ValueCallback<'a> = &'a mut dyn FnMut(CbHeatedBlanket1Value);
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
pub struct CbHeatedBlanket1(Selector);
impl CbHeatedBlanket1 {
    pub const fn new() -> Self {
        Self(Selector::new(5302, 1, 0))
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
pub type CbHeatedBlanket2ValueCallback<'a> = &'a mut dyn FnMut(CbHeatedBlanket2Value);
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
pub struct CbHeatedBlanket2(Selector);
impl CbHeatedBlanket2 {
    pub const fn new() -> Self {
        Self(Selector::new(5302, 2, 1))
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
pub type CbHfAntValueCallback<'a> = &'a mut dyn FnMut(CbHfAntValue);
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
pub struct CbHfAnt(Selector);
impl CbHfAnt {
    pub const fn new() -> Self {
        Self(Selector::new(5296, 1, 0))
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
pub type CbHfArc102ValueCallback<'a> = &'a mut dyn FnMut(CbHfArc102Value);
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
pub struct CbHfArc102(Selector);
impl CbHfArc102 {
    pub const fn new() -> Self {
        Self(Selector::new(5296, 2, 1))
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
pub type CbHookValueCallback<'a> = &'a mut dyn FnMut(CbHookValue);
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
pub struct CbHook(Selector);
impl CbHook {
    pub const fn new() -> Self {
        Self(Selector::new(5300, 16, 4))
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
pub type CbHydctlValueCallback<'a> = &'a mut dyn FnMut(CbHydctlValue);
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
pub struct CbHydctl(Selector);
impl CbHydctl {
    pub const fn new() -> Self {
        Self(Selector::new(5300, 4, 2))
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
pub type CbIdleStopValueCallback<'a> = &'a mut dyn FnMut(CbIdleStopValue);
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
pub struct CbIdleStop(Selector);
impl CbIdleStop {
    pub const fn new() -> Self {
        Self(Selector::new(5300, 4096, 12))
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
pub type CbIffApx1ValueCallback<'a> = &'a mut dyn FnMut(CbIffApx1Value);
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
pub struct CbIffApx1(Selector);
impl CbIffApx1 {
    pub const fn new() -> Self {
        Self(Selector::new(5294, 128, 7))
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
pub type CbIffApx2ValueCallback<'a> = &'a mut dyn FnMut(CbIffApx2Value);
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
pub struct CbIffApx2(Selector);
impl CbIffApx2 {
    pub const fn new() -> Self {
        Self(Selector::new(5294, 256, 8))
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
pub type CbIgnValueCallback<'a> = &'a mut dyn FnMut(CbIgnValue);
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
pub struct CbIgn(Selector);
impl CbIgn {
    pub const fn new() -> Self {
        Self(Selector::new(5302, 8, 3))
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
pub type CbInstLtsValueCallback<'a> = &'a mut dyn FnMut(CbInstLtsValue);
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
pub struct CbInstLts(Selector);
impl CbInstLts {
    pub const fn new() -> Self {
        Self(Selector::new(5298, 128, 7))
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
pub type CbInstSecLtsValueCallback<'a> = &'a mut dyn FnMut(CbInstSecLtsValue);
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
pub struct CbInstSecLts(Selector);
impl CbInstSecLts {
    pub const fn new() -> Self {
        Self(Selector::new(5298, 256, 8))
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
pub type CbIntCpltValueCallback<'a> = &'a mut dyn FnMut(CbIntCpltValue);
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
pub struct CbIntCplt(Selector);
impl CbIntCplt {
    pub const fn new() -> Self {
        Self(Selector::new(5294, 8192, 13))
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
pub type CbIntPltValueCallback<'a> = &'a mut dyn FnMut(CbIntPltValue);
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
pub struct CbIntPlt(Selector);
impl CbIntPlt {
    pub const fn new() -> Self {
        Self(Selector::new(5294, 16384, 14))
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
pub type CbInvtrValueCallback<'a> = &'a mut dyn FnMut(CbInvtrValue);
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
pub struct CbInvtr(Selector);
impl CbInvtr {
    pub const fn new() -> Self {
        Self(Selector::new(5300, 128, 7))
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
pub type CbJtsnValueCallback<'a> = &'a mut dyn FnMut(CbJtsnValue);
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
pub struct CbJtsn(Selector);
impl CbJtsn {
    pub const fn new() -> Self {
        Self(Selector::new(5300, 32, 5))
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
pub type CbKy20ValueCallback<'a> = &'a mut dyn FnMut(CbKy20Value);
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
pub struct CbKy20(Selector);
impl CbKy20 {
    pub const fn new() -> Self {
        Self(Selector::new(5296, 2048, 11))
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
pub type CbLdgltsValueCallback<'a> = &'a mut dyn FnMut(CbLdgltsValue);
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
pub struct CbLdglts(Selector);
impl CbLdglts {
    pub const fn new() -> Self {
        Self(Selector::new(5296, 16384, 14))
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
pub type CbLfNavValueCallback<'a> = &'a mut dyn FnMut(CbLfNavValue);
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
pub struct CbLfNav(Selector);
impl CbLfNav {
    pub const fn new() -> Self {
        Self(Selector::new(5294, 4096, 12))
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
pub type CbLhFuelValueCallback<'a> = &'a mut dyn FnMut(CbLhFuelValue);
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
pub struct CbLhFuel(Selector);
impl CbLhFuel {
    pub const fn new() -> Self {
        Self(Selector::new(5298, 32768, 15))
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
pub type CbMainInvtrValueCallback<'a> = &'a mut dyn FnMut(CbMainInvtrValue);
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
pub struct CbMainInvtr(Selector);
impl CbMainInvtr {
    pub const fn new() -> Self {
        Self(Selector::new(5300, 256, 8))
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
pub type CbMarkerBeaconValueCallback<'a> = &'a mut dyn FnMut(CbMarkerBeaconValue);
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
pub struct CbMarkerBeacon(Selector);
impl CbMarkerBeacon {
    pub const fn new() -> Self {
        Self(Selector::new(5294, 1024, 10))
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
pub type CbNavLtsValueCallback<'a> = &'a mut dyn FnMut(CbNavLtsValue);
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
pub struct CbNavLts(Selector);
impl CbNavLts {
    pub const fn new() -> Self {
        Self(Selector::new(5298, 4, 2))
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
pub type CbOutletValveValueCallback<'a> = &'a mut dyn FnMut(CbOutletValveValue);
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
pub struct CbOutletValve(Selector);
impl CbOutletValve {
    pub const fn new() -> Self {
        Self(Selector::new(5298, 512, 9))
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
pub type CbPitotTubeValueCallback<'a> = &'a mut dyn FnMut(CbPitotTubeValue);
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
pub struct CbPitotTube(Selector);
impl CbPitotTube {
    pub const fn new() -> Self {
        Self(Selector::new(5296, 64, 6))
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
pub type CbPltAttd1ValueCallback<'a> = &'a mut dyn FnMut(CbPltAttd1Value);
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
pub struct CbPltAttd1(Selector);
impl CbPltAttd1 {
    pub const fn new() -> Self {
        Self(Selector::new(5302, 16, 4))
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
pub type CbPltAttd2ValueCallback<'a> = &'a mut dyn FnMut(CbPltAttd2Value);
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
pub struct CbPltAttd2(Selector);
impl CbPltAttd2 {
    pub const fn new() -> Self {
        Self(Selector::new(5302, 32, 5))
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
pub type CbPorxWarnValueCallback<'a> = &'a mut dyn FnMut(CbPorxWarnValue);
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
pub struct CbPorxWarn(Selector);
impl CbPorxWarn {
    pub const fn new() -> Self {
        Self(Selector::new(5294, 512, 9))
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
pub type CbRescueContValueCallback<'a> = &'a mut dyn FnMut(CbRescueContValue);
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
pub struct CbRescueCont(Selector);
impl CbRescueCont {
    pub const fn new() -> Self {
        Self(Selector::new(5296, 128, 7))
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
pub type CbRescuePwrValueCallback<'a> = &'a mut dyn FnMut(CbRescuePwrValue);
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
pub struct CbRescuePwr(Selector);
impl CbRescuePwr {
    pub const fn new() -> Self {
        Self(Selector::new(5298, 2048, 11))
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
pub type CbRhFuelValueCallback<'a> = &'a mut dyn FnMut(CbRhFuelValue);
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
pub struct CbRhFuel(Selector);
impl CbRhFuel {
    pub const fn new() -> Self {
        Self(Selector::new(5300, 8192, 13))
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
pub type CbRpmWarnValueCallback<'a> = &'a mut dyn FnMut(CbRpmWarnValue);
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
pub struct CbRpmWarn(Selector);
impl CbRpmWarn {
    pub const fn new() -> Self {
        Self(Selector::new(5298, 4096, 12))
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
pub type CbSlCtlValueCallback<'a> = &'a mut dyn FnMut(CbSlCtlValue);
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
pub struct CbSlCtl(Selector);
impl CbSlCtl {
    pub const fn new() -> Self {
        Self(Selector::new(5296, 32768, 15))
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
pub type CbSlPwrValueCallback<'a> = &'a mut dyn FnMut(CbSlPwrValue);
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
pub struct CbSlPwr(Selector);
impl CbSlPwr {
    pub const fn new() -> Self {
        Self(Selector::new(5296, 8192, 13))
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
pub type CbSpareInvtrValueCallback<'a> = &'a mut dyn FnMut(CbSpareInvtrValue);
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
pub struct CbSpareInvtr(Selector);
impl CbSpareInvtr {
    pub const fn new() -> Self {
        Self(Selector::new(5300, 64, 6))
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
pub type CbStarterValueCallback<'a> = &'a mut dyn FnMut(CbStarterValue);
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
pub struct CbStarter(Selector);
impl CbStarter {
    pub const fn new() -> Self {
        Self(Selector::new(5296, 4096, 12))
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
pub type CbStbyGenValueCallback<'a> = &'a mut dyn FnMut(CbStbyGenValue);
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
pub struct CbStbyGen(Selector);
impl CbStbyGen {
    pub const fn new() -> Self {
        Self(Selector::new(5300, 1024, 10))
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
pub type CbTempValueCallback<'a> = &'a mut dyn FnMut(CbTempValue);
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
pub struct CbTemp(Selector);
impl CbTemp {
    pub const fn new() -> Self {
        Self(Selector::new(5300, 2, 1))
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
pub type CbTorqValueCallback<'a> = &'a mut dyn FnMut(CbTorqValue);
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
pub struct CbTorq(Selector);
impl CbTorq {
    pub const fn new() -> Self {
        Self(Selector::new(5302, 8192, 13))
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
pub type CbTrimValueCallback<'a> = &'a mut dyn FnMut(CbTrimValue);
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
pub struct CbTrim(Selector);
impl CbTrim {
    pub const fn new() -> Self {
        Self(Selector::new(5300, 8, 3))
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
pub type CbTurnSlipValueCallback<'a> = &'a mut dyn FnMut(CbTurnSlipValue);
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
pub struct CbTurnSlip(Selector);
impl CbTurnSlip {
    pub const fn new() -> Self {
        Self(Selector::new(5300, 1, 0))
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
pub type CbUhfRadioValueCallback<'a> = &'a mut dyn FnMut(CbUhfRadioValue);
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
pub struct CbUhfRadio(Selector);
impl CbUhfRadio {
    pub const fn new() -> Self {
        Self(Selector::new(5296, 8, 3))
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
pub type CbVhfNavValueCallback<'a> = &'a mut dyn FnMut(CbVhfNavValue);
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
pub struct CbVhfNav(Selector);
impl CbVhfNav {
    pub const fn new() -> Self {
        Self(Selector::new(5294, 2048, 11))
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
pub type CbVmValueCallback<'a> = &'a mut dyn FnMut(CbVmValue);
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
pub struct CbVm(Selector);
impl CbVm {
    pub const fn new() -> Self {
        Self(Selector::new(5302, 4, 2))
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
pub type CbWiperCpltValueCallback<'a> = &'a mut dyn FnMut(CbWiperCpltValue);
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
pub struct CbWiperCplt(Selector);
impl CbWiperCplt {
    pub const fn new() -> Self {
        Self(Selector::new(5296, 512, 9))
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
pub type CbWiperPltValueCallback<'a> = &'a mut dyn FnMut(CbWiperPltValue);
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
pub struct CbWiperPlt(Selector);
impl CbWiperPlt {
    pub const fn new() -> Self {
        Self(Selector::new(5296, 1024, 10))
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
pub type CbXmsnValueCallback<'a> = &'a mut dyn FnMut(CbXmsnValue);
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
pub struct CbXmsn(Selector);
impl CbXmsn {
    pub const fn new() -> Self {
        Self(Selector::new(5302, 16384, 14))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChipDetSwValue {
    Select1,
    Select2,
    Select3,
}
pub type ChipDetSwValueCallback<'a> = &'a mut dyn FnMut(ChipDetSwValue);
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
pub struct ChipDetSw(Selector);
impl ChipDetSw {
    pub const fn new() -> Self {
        Self(Selector::new(5310, 192, 6))
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

/// Rotating Course Card
/// Output: gauge position
pub struct CiCourseCard(AnalogGauge);
impl CiCourseCard {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5152, 65535, 0))
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
pub struct CiFrom(AnalogGauge);
impl CiFrom {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5150, 65535, 0))
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
pub struct CiHbar(AnalogGauge);
impl CiHbar {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5146, 65535, 0))
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
pub struct CiHoff(AnalogGauge);
impl CiHoff {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5156, 65535, 0))
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
pub struct CiTo(AnalogGauge);
impl CiTo {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5148, 65535, 0))
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
pub struct CiVbar(AnalogGauge);
impl CiVbar {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5144, 65535, 0))
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
pub struct CiVoff(AnalogGauge);
impl CiVoff {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5154, 65535, 0))
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

/// ENGINE OIL PRESS (yellow)
/// Output: 0 if light is off, 1 if light is on
pub struct ClA0(Led);
impl ClA0 {
    pub const fn new() -> Self {
        Self(Led::new(5140, 1, 0))
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
pub struct ClA1(Led);
impl ClA1 {
    pub const fn new() -> Self {
        Self(Led::new(5140, 1024, 10))
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
pub struct ClB0(Led);
impl ClB0 {
    pub const fn new() -> Self {
        Self(Led::new(5140, 2, 1))
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
pub struct ClB1(Led);
impl ClB1 {
    pub const fn new() -> Self {
        Self(Led::new(5140, 2048, 11))
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
pub struct ClC0(Led);
impl ClC0 {
    pub const fn new() -> Self {
        Self(Led::new(5140, 4, 2))
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
pub struct ClC1(Led);
impl ClC1 {
    pub const fn new() -> Self {
        Self(Led::new(5140, 4096, 12))
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
pub struct ClD0(Led);
impl ClD0 {
    pub const fn new() -> Self {
        Self(Led::new(5140, 8, 3))
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
pub struct ClD1(Led);
impl ClD1 {
    pub const fn new() -> Self {
        Self(Led::new(5140, 8192, 13))
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
pub struct ClE0(Led);
impl ClE0 {
    pub const fn new() -> Self {
        Self(Led::new(5140, 16, 4))
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
pub struct ClE1(Led);
impl ClE1 {
    pub const fn new() -> Self {
        Self(Led::new(5140, 16384, 14))
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
pub struct ClF0(Led);
impl ClF0 {
    pub const fn new() -> Self {
        Self(Led::new(5140, 32, 5))
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
pub struct ClF1(Led);
impl ClF1 {
    pub const fn new() -> Self {
        Self(Led::new(5140, 32768, 15))
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
pub struct ClG0(Led);
impl ClG0 {
    pub const fn new() -> Self {
        Self(Led::new(5140, 64, 6))
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
pub struct ClG1(Led);
impl ClG1 {
    pub const fn new() -> Self {
        Self(Led::new(5142, 1, 0))
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
pub struct ClH0(Led);
impl ClH0 {
    pub const fn new() -> Self {
        Self(Led::new(5140, 128, 7))
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
pub struct ClH1(Led);
impl ClH1 {
    pub const fn new() -> Self {
        Self(Led::new(5142, 2, 1))
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
pub struct ClI0(Led);
impl ClI0 {
    pub const fn new() -> Self {
        Self(Led::new(5140, 256, 8))
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
pub struct ClI1(Led);
impl ClI1 {
    pub const fn new() -> Self {
        Self(Led::new(5142, 4, 2))
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
pub struct ClJ0(Led);
impl ClJ0 {
    pub const fn new() -> Self {
        Self(Led::new(5140, 512, 9))
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
pub struct ClJ1(Led);
impl ClJ1 {
    pub const fn new() -> Self {
        Self(Led::new(5142, 8, 3))
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

/// Adjustment Dial
/// Input: turn the dial left or right
/// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
pub struct ClockAdjust(AnalogDial);
impl ClockAdjust {
    pub const fn new() -> Self {
        Self(AnalogDial::new(5316, 65535, 0))
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
pub struct ClockAdjustPull(Selector);
impl ClockAdjustPull {
    pub const fn new() -> Self {
        Self(Selector::new(5308, 32768, 15))
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

/// Clock Hours
/// Output: gauge position
pub struct ClockH(AnalogGauge);
impl ClockH {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5240, 65535, 0))
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
pub struct ClockM(AnalogGauge);
impl ClockM {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5242, 65535, 0))
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
pub struct ClockS(AnalogGauge);
impl ClockS {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5244, 65535, 0))
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
pub enum ClpBrightDimSwValue {
    Select1,
    Select2,
    Select3,
}
pub type ClpBrightDimSwValueCallback<'a> = &'a mut dyn FnMut(ClpBrightDimSwValue);
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
pub struct ClpBrightDimSw(Selector);
impl ClpBrightDimSw {
    pub const fn new() -> Self {
        Self(Selector::new(5304, 384, 7))
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
pub type ClpResetTestSwValueCallback<'a> = &'a mut dyn FnMut(ClpResetTestSwValue);
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
pub struct ClpResetTestSw(Selector);
impl ClpResetTestSw {
    pub const fn new() -> Self {
        Self(Selector::new(5304, 96, 5))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CmArmSwValue {
    Select1,
    Select2,
}
pub type CmArmSwValueCallback<'a> = &'a mut dyn FnMut(CmArmSwValue);
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
pub struct CmArmSw(Selector);
impl CmArmSw {
    pub const fn new() -> Self {
        Self(Selector::new(5406, 16, 4))
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
pub type CmArmedTestValueCallback<'a> = &'a mut dyn FnMut(CmArmedTestValue);
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
pub struct CmArmedTest(Selector);
impl CmArmedTest {
    pub const fn new() -> Self {
        Self(Selector::new(5406, 128, 7))
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
pub struct CmChaffcntDisplay(Metadata);
impl CmChaffcntDisplay {
    pub const fn new() -> Self {
        Self(Metadata::new(5418, 32256, 9))
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
pub struct CmChaffcntDisplayStr(Display<2>);
impl CmChaffcntDisplayStr {
    pub const fn new() -> Self {
        Self(Display::<2>::new(5410, 2))
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
pub type CmChaffcntResetValueCallback<'a> = &'a mut dyn FnMut(CmChaffcntResetValue);
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
pub struct CmChaffcntReset(Selector);
impl CmChaffcntReset {
    pub const fn new() -> Self {
        Self(Selector::new(5406, 512, 9))
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
pub type CmFlareBtnValueCallback<'a> = &'a mut dyn FnMut(CmFlareBtnValue);
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
pub struct CmFlareBtn(Selector);
impl CmFlareBtn {
    pub const fn new() -> Self {
        Self(Selector::new(5406, 64, 6))
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
pub struct CmFlarecntDisplay(Metadata);
impl CmFlarecntDisplay {
    pub const fn new() -> Self {
        Self(Metadata::new(5418, 504, 3))
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
pub struct CmFlarecntDisplayStr(Display<2>);
impl CmFlarecntDisplayStr {
    pub const fn new() -> Self {
        Self(Display::<2>::new(5408, 2))
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
pub type CmFlarecntResetValueCallback<'a> = &'a mut dyn FnMut(CmFlarecntResetValue);
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
pub struct CmFlarecntReset(Selector);
impl CmFlarecntReset {
    pub const fn new() -> Self {
        Self(Selector::new(5406, 256, 8))
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
pub type CmManPgrmSwValueCallback<'a> = &'a mut dyn FnMut(CmManPgrmSwValue);
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
pub struct CmManPgrmSw(Selector);
impl CmManPgrmSw {
    pub const fn new() -> Self {
        Self(Selector::new(5406, 32, 5))
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
pub type CmRippleCoverValueCallback<'a> = &'a mut dyn FnMut(CmRippleCoverValue);
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
pub struct CmRippleCover(Selector);
impl CmRippleCover {
    pub const fn new() -> Self {
        Self(Selector::new(5406, 4, 2))
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
pub type CmRippleSwValueCallback<'a> = &'a mut dyn FnMut(CmRippleSwValue);
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
pub struct CmRippleSw(Selector);
impl CmRippleSw {
    pub const fn new() -> Self {
        Self(Selector::new(5406, 8, 3))
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

/// Console Light
/// Output: gauge position
pub struct ConsoleLight(AnalogGauge);
impl ConsoleLight {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5274, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CpltAdiCageValue {
    Select1,
    Select2,
}
pub type CpltAdiCageValueCallback<'a> = &'a mut dyn FnMut(CpltAdiCageValue);
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
pub struct CpltAdiCage(Selector);
impl CpltAdiCage {
    pub const fn new() -> Self {
        Self(Selector::new(5310, 16384, 14))
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
pub struct CrsKnob(AnalogDial);
impl CrsKnob {
    pub const fn new() -> Self {
        Self(AnalogDial::new(5396, 65535, 0))
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

/// DC Battery 1 Current
/// Output: gauge position
pub struct DcBat1Current(AnalogGauge);
impl DcBat1Current {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5124, 65535, 0))
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
pub struct DcBat2Current(AnalogGauge);
impl DcBat2Current {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5126, 65535, 0))
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
pub struct DcV(AnalogGauge);
impl DcV {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5228, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DcVmSrcValue {
    Select1,
    Select2,
    Select3,
    Select4,
    Select5,
}
pub type DcVmSrcValueCallback<'a> = &'a mut dyn FnMut(DcVmSrcValue);
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
pub struct DcVmSrc(Selector);
impl DcVmSrc {
    pub const fn new() -> Self {
        Self(Selector::new(5294, 7, 0))
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

/// DC Voltage
/// Output: gauge position
pub struct DcVoltage(AnalogGauge);
impl DcVoltage {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5120, 65535, 0))
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
pub struct DcVu1Current(AnalogGauge);
impl DcVu1Current {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5128, 65535, 0))
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
pub struct DcVu2Current(AnalogGauge);
impl DcVu2Current {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5130, 65535, 0))
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
pub struct DcVu3Current(AnalogGauge);
impl DcVu3Current {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5132, 65535, 0))
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

/// Dome Light
/// Output: gauge position
pub struct DomeLight(AnalogGauge);
impl DomeLight {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5276, 65535, 0))
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
pub struct DomeLightGreen(AnalogGauge);
impl DomeLightGreen {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5278, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DomeLightSwValue {
    Select1,
    Select2,
    Select3,
}
pub type DomeLightSwValueCallback<'a> = &'a mut dyn FnMut(DomeLightSwValue);
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
pub struct DomeLightSw(Selector);
impl DomeLightSw {
    pub const fn new() -> Self {
        Self(Selector::new(5378, 48, 4))
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
pub enum DoorLPtrValue {
    Select1,
    Select2,
}
pub type DoorLPtrValueCallback<'a> = &'a mut dyn FnMut(DoorLPtrValue);
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
pub struct DoorLPtr(Selector);
impl DoorLPtr {
    pub const fn new() -> Self {
        Self(Selector::new(5406, 8192, 13))
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
pub type DoorRPtrValueCallback<'a> = &'a mut dyn FnMut(DoorRPtrValue);
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
pub struct DoorRPtr(Selector);
impl DoorRPtr {
    pub const fn new() -> Self {
        Self(Selector::new(5406, 16384, 14))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EmerGovSwValue {
    Select1,
    Select2,
}
pub type EmerGovSwValueCallback<'a> = &'a mut dyn FnMut(EmerGovSwValue);
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
pub struct EmerGovSw(Selector);
impl EmerGovSw {
    pub const fn new() -> Self {
        Self(Selector::new(5310, 256, 8))
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
pub type EngDeiceValueCallback<'a> = &'a mut dyn FnMut(EngDeiceValue);
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
pub struct EngDeice(Selector);
impl EngDeice {
    pub const fn new() -> Self {
        Self(Selector::new(5310, 16, 4))
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

/// Engine Oil Pressure
/// Output: gauge position
pub struct EngOilPress(AnalogGauge);
impl EngOilPress {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5204, 65535, 0))
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
pub struct EngOilTemp(AnalogGauge);
impl EngOilTemp {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5206, 65535, 0))
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
pub struct EngRpm(AnalogGauge);
impl EngRpm {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5218, 65535, 0))
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
pub struct ExhaustTemp(AnalogGauge);
impl ExhaustTemp {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5216, 65535, 0))
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

/// Left Position Light (red)
/// Output: Left Position Light (red)
pub struct ExtPositionLightLeft(Metadata);
impl ExtPositionLightLeft {
    pub const fn new() -> Self {
        Self(Metadata::new(5406, 32768, 15))
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
pub struct ExtPositionLightRight(Metadata);
impl ExtPositionLightRight {
    pub const fn new() -> Self {
        Self(Metadata::new(5418, 1, 0))
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
pub struct ExtStrobe(Metadata);
impl ExtStrobe {
    pub const fn new() -> Self {
        Self(Metadata::new(5418, 2, 1))
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
pub struct ExtWowSkid(Metadata);
impl ExtWowSkid {
    pub const fn new() -> Self {
        Self(Metadata::new(5418, 4, 2))
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

/// FIRE Warning Light (yellow)
/// Output: 0 if light is off, 1 if light is on
pub struct FireInd(Led);
impl FireInd {
    pub const fn new() -> Self {
        Self(Led::new(5142, 64, 6))
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
pub type FireTestBtnValueCallback<'a> = &'a mut dyn FnMut(FireTestBtnValue);
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
pub struct FireTestBtn(Selector);
impl FireTestBtn {
    pub const fn new() -> Self {
        Self(Selector::new(5310, 8192, 13))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ForceTrimSwValue {
    Select1,
    Select2,
}
pub type ForceTrimSwValueCallback<'a> = &'a mut dyn FnMut(ForceTrimSwValue);
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
pub struct ForceTrimSw(Selector);
impl ForceTrimSw {
    pub const fn new() -> Self {
        Self(Selector::new(5310, 1024, 10))
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

/// Fuel Pressure
/// Output: gauge position
pub struct FuelPress(AnalogGauge);
impl FuelPress {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5236, 65535, 0))
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
pub struct FuelQty(AnalogGauge);
impl FuelQty {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5238, 65535, 0))
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
pub struct GasRpm(AnalogGauge);
impl GasRpm {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5224, 65535, 0))
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
pub struct GasRpmT(AnalogGauge);
impl GasRpmT {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5222, 65535, 0))
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

/// Output: gauge position
pub struct GmcAnnunciator(AnalogGauge);
impl GmcAnnunciator {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5170, 65535, 0))
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

/// GMC Course Pointer 1
/// Output: gauge position
pub struct GmcCrs1(AnalogGauge);
impl GmcCrs1 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5162, 65535, 0))
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
pub struct GmcCrs2(AnalogGauge);
impl GmcCrs2 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5164, 65535, 0))
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
pub struct GmcHdg(AnalogGauge);
impl GmcHdg {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5168, 65535, 0))
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
pub struct GmcHdgMarker(AnalogGauge);
impl GmcHdgMarker {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5166, 65535, 0))
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

/// Output: gauge position
pub struct GmcPwrfail(AnalogGauge);
impl GmcPwrfail {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5172, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GovRpmSwValue {
    Select1,
    Select2,
    Select3,
}
pub type GovRpmSwValueCallback<'a> = &'a mut dyn FnMut(GovRpmSwValue);
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
pub struct GovRpmSw(Selector);
impl GovRpmSw {
    pub const fn new() -> Self {
        Self(Selector::new(5310, 6144, 11))
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
pub type GovRpmSw2ValueCallback<'a> = &'a mut dyn FnMut(GovRpmSw2Value);
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
pub struct GovRpmSw2(Selector);
impl GovRpmSw2 {
    pub const fn new() -> Self {
        Self(Selector::new(5432, 3, 0))
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
pub enum GunSelValue {
    Select1,
    Select2,
    Select3,
}
pub type GunSelValueCallback<'a> = &'a mut dyn FnMut(GunSelValue);
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
pub struct GunSel(Selector);
impl GunSel {
    pub const fn new() -> Self {
        Self(Selector::new(5378, 24576, 13))
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
pub enum GyroModeSwValue {
    Select1,
    Select2,
}
pub type GyroModeSwValueCallback<'a> = &'a mut dyn FnMut(GyroModeSwValue);
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
pub struct GyroModeSw(Selector);
impl GyroModeSw {
    pub const fn new() -> Self {
        Self(Selector::new(5378, 1024, 10))
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
pub struct HdgSet(AnalogDial);
impl HdgSet {
    pub const fn new() -> Self {
        Self(AnalogDial::new(5392, 65535, 0))
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
pub struct HdgSync(AnalogDial);
impl HdgSync {
    pub const fn new() -> Self {
        Self(AnalogDial::new(5394, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HydContSwValue {
    Select1,
    Select2,
}
pub type HydContSwValueCallback<'a> = &'a mut dyn FnMut(HydContSwValue);
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
pub struct HydContSw(Selector);
impl HydContSw {
    pub const fn new() -> Self {
        Self(Selector::new(5310, 512, 9))
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

/// IAS Front
/// Output: gauge position
pub struct IasNose(AnalogGauge);
impl IasNose {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5212, 65535, 0))
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
pub struct IasRoof(AnalogGauge);
impl IasRoof {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5214, 65535, 0))
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

/// IFF Code: ZERO - B - A - (HOLD)
/// Input: switch to previous or next state
/// Output: selector position
pub struct IffCode(Selector);
impl IffCode {
    pub const fn new() -> Self {
        Self(Selector::new(5304, 6144, 11))
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
pub type IffMasterValueCallback<'a> = &'a mut dyn FnMut(IffMasterValue);
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
pub struct IffMaster(Selector);
impl IffMaster {
    pub const fn new() -> Self {
        Self(Selector::new(5304, 57344, 13))
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
pub type IffMicIdentValueCallback<'a> = &'a mut dyn FnMut(IffMicIdentValue);
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
pub struct IffMicIdent(Selector);
impl IffMicIdent {
    pub const fn new() -> Self {
        Self(Selector::new(5306, 12288, 12))
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
pub type IffMode1Wheel1ValueCallback<'a> = &'a mut dyn FnMut(IffMode1Wheel1Value);
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
pub struct IffMode1Wheel1(Selector);
impl IffMode1Wheel1 {
    pub const fn new() -> Self {
        Self(Selector::new(5308, 7, 0))
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
pub type IffMode1Wheel2ValueCallback<'a> = &'a mut dyn FnMut(IffMode1Wheel2Value);
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
pub struct IffMode1Wheel2(Selector);
impl IffMode1Wheel2 {
    pub const fn new() -> Self {
        Self(Selector::new(5308, 24, 3))
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
pub type IffMode3aWheel1ValueCallback<'a> = &'a mut dyn FnMut(IffMode3aWheel1Value);
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
pub struct IffMode3aWheel1(Selector);
impl IffMode3aWheel1 {
    pub const fn new() -> Self {
        Self(Selector::new(5308, 224, 5))
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
pub type IffMode3aWheel2ValueCallback<'a> = &'a mut dyn FnMut(IffMode3aWheel2Value);
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
pub struct IffMode3aWheel2(Selector);
impl IffMode3aWheel2 {
    pub const fn new() -> Self {
        Self(Selector::new(5308, 1792, 8))
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
pub type IffMode3aWheel3ValueCallback<'a> = &'a mut dyn FnMut(IffMode3aWheel3Value);
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
pub struct IffMode3aWheel3(Selector);
impl IffMode3aWheel3 {
    pub const fn new() -> Self {
        Self(Selector::new(5308, 14336, 11))
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
pub type IffMode3aWheel4ValueCallback<'a> = &'a mut dyn FnMut(IffMode3aWheel4Value);
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
pub struct IffMode3aWheel4(Selector);
impl IffMode3aWheel4 {
    pub const fn new() -> Self {
        Self(Selector::new(5310, 7, 0))
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
pub type IffOnOutValueCallback<'a> = &'a mut dyn FnMut(IffOnOutValue);
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
pub struct IffOnOut(Selector);
impl IffOnOut {
    pub const fn new() -> Self {
        Self(Selector::new(5306, 16384, 14))
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
pub type IffOutAudioLightValueCallback<'a> = &'a mut dyn FnMut(IffOutAudioLightValue);
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
pub struct IffOutAudioLight(Selector);
impl IffOutAudioLight {
    pub const fn new() -> Self {
        Self(Selector::new(5306, 3, 0))
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
pub type IffRadtestValueCallback<'a> = &'a mut dyn FnMut(IffRadtestValue);
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
pub struct IffRadtest(Selector);
impl IffRadtest {
    pub const fn new() -> Self {
        Self(Selector::new(5306, 3072, 10))
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
pub struct IffReplyDim(LimitedDial);
impl IffReplyDim {
    pub const fn new() -> Self {
        Self(LimitedDial::new(5312, 65535, 0))
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
pub struct IffReplyInd(Led);
impl IffReplyInd {
    pub const fn new() -> Self {
        Self(Led::new(5142, 512, 9))
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
pub type IffReplyTestValueCallback<'a> = &'a mut dyn FnMut(IffReplyTestValue);
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
pub struct IffReplyTest(Selector);
impl IffReplyTest {
    pub const fn new() -> Self {
        Self(Selector::new(5306, 32768, 15))
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
pub struct IffTestDim(LimitedDial);
impl IffTestDim {
    pub const fn new() -> Self {
        Self(LimitedDial::new(5314, 65535, 0))
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
pub struct IffTestInd(Led);
impl IffTestInd {
    pub const fn new() -> Self {
        Self(Led::new(5142, 1024, 10))
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
pub type IffTestM1ValueCallback<'a> = &'a mut dyn FnMut(IffTestM1Value);
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
pub struct IffTestM1(Selector);
impl IffTestM1 {
    pub const fn new() -> Self {
        Self(Selector::new(5306, 12, 2))
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
pub type IffTestM2ValueCallback<'a> = &'a mut dyn FnMut(IffTestM2Value);
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
pub struct IffTestM2(Selector);
impl IffTestM2 {
    pub const fn new() -> Self {
        Self(Selector::new(5306, 48, 4))
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
pub type IffTestM3ValueCallback<'a> = &'a mut dyn FnMut(IffTestM3Value);
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
pub struct IffTestM3(Selector);
impl IffTestM3 {
    pub const fn new() -> Self {
        Self(Selector::new(5306, 192, 6))
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
pub type IffTestM4ValueCallback<'a> = &'a mut dyn FnMut(IffTestM4Value);
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
pub struct IffTestM4(Selector);
impl IffTestM4 {
    pub const fn new() -> Self {
        Self(Selector::new(5306, 768, 8))
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
pub type IffTestTestValueCallback<'a> = &'a mut dyn FnMut(IffTestTestValue);
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
pub struct IffTestTest(Selector);
impl IffTestTest {
    pub const fn new() -> Self {
        Self(Selector::new(5308, 16384, 14))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntIntSwValue {
    Select1,
    Select2,
}
pub type IntIntSwValueCallback<'a> = &'a mut dyn FnMut(IntIntSwValue);
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
pub struct IntIntSw(Selector);
impl IntIntSw {
    pub const fn new() -> Self {
        Self(Selector::new(5330, 8192, 13))
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
pub type IntModeValueCallback<'a> = &'a mut dyn FnMut(IntModeValue);
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
pub struct IntMode(Selector);
impl IntMode {
    pub const fn new() -> Self {
        Self(Selector::new(5340, 1792, 8))
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
pub type IntNavSwValueCallback<'a> = &'a mut dyn FnMut(IntNavSwValue);
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
pub struct IntNavSw(Selector);
impl IntNavSw {
    pub const fn new() -> Self {
        Self(Selector::new(5330, 16384, 14))
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
pub type IntRcvr1SwValueCallback<'a> = &'a mut dyn FnMut(IntRcvr1SwValue);
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
pub struct IntRcvr1Sw(Selector);
impl IntRcvr1Sw {
    pub const fn new() -> Self {
        Self(Selector::new(5330, 512, 9))
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
pub type IntRcvr2SwValueCallback<'a> = &'a mut dyn FnMut(IntRcvr2SwValue);
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
pub struct IntRcvr2Sw(Selector);
impl IntRcvr2Sw {
    pub const fn new() -> Self {
        Self(Selector::new(5330, 1024, 10))
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
pub type IntRcvr3SwValueCallback<'a> = &'a mut dyn FnMut(IntRcvr3SwValue);
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
pub struct IntRcvr3Sw(Selector);
impl IntRcvr3Sw {
    pub const fn new() -> Self {
        Self(Selector::new(5330, 2048, 11))
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
pub type IntRecvr4SwValueCallback<'a> = &'a mut dyn FnMut(IntRecvr4SwValue);
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
pub struct IntRecvr4Sw(Selector);
impl IntRecvr4Sw {
    pub const fn new() -> Self {
        Self(Selector::new(5330, 4096, 12))
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
pub struct IntVol(LimitedDial);
impl IntVol {
    pub const fn new() -> Self {
        Self(LimitedDial::new(5342, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InverterSwValue {
    Select1,
    Select2,
    Select3,
}
pub type InverterSwValueCallback<'a> = &'a mut dyn FnMut(InverterSwValue);
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
pub struct InverterSw(Selector);
impl InverterSw {
    pub const fn new() -> Self {
        Self(Selector::new(5294, 96, 5))
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
pub enum JtsnBtnValue {
    Select1,
    Select2,
}
pub type JtsnBtnValueCallback<'a> = &'a mut dyn FnMut(JtsnBtnValue);
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
pub struct JtsnBtn(Selector);
impl JtsnBtn {
    pub const fn new() -> Self {
        Self(Selector::new(5398, 64, 6))
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
pub type JtsnCoverValueCallback<'a> = &'a mut dyn FnMut(JtsnCoverValue);
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
pub struct JtsnCover(Selector);
impl JtsnCover {
    pub const fn new() -> Self {
        Self(Selector::new(5398, 32, 5))
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
pub enum LdgLightSwValue {
    Select1,
    Select2,
}
pub type LdgLightSwValueCallback<'a> = &'a mut dyn FnMut(LdgLightSwValue);
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
pub struct LdgLightSw(Selector);
impl LdgLightSw {
    pub const fn new() -> Self {
        Self(Selector::new(5372, 32768, 15))
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
pub type LdgLightSw2ValueCallback<'a> = &'a mut dyn FnMut(LdgLightSw2Value);
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
pub struct LdgLightSw2(Selector);
impl LdgLightSw2 {
    pub const fn new() -> Self {
        Self(Selector::new(5432, 4, 2))
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
pub type LdgLtCtrlValueCallback<'a> = &'a mut dyn FnMut(LdgLtCtrlValue);
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
pub struct LdgLtCtrl(Selector);
impl LdgLtCtrl {
    pub const fn new() -> Self {
        Self(Selector::new(5378, 12, 2))
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
pub type LdgLtCtrl2ValueCallback<'a> = &'a mut dyn FnMut(LdgLtCtrl2Value);
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
pub struct LdgLtCtrl2(Selector);
impl LdgLtCtrl2 {
    pub const fn new() -> Self {
        Self(Selector::new(5432, 96, 5))
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
pub enum LowRpmAudioValue {
    Select1,
    Select2,
}
pub type LowRpmAudioValueCallback<'a> = &'a mut dyn FnMut(LowRpmAudioValue);
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
pub struct LowRpmAudio(Selector);
impl LowRpmAudio {
    pub const fn new() -> Self {
        Self(Selector::new(5310, 32, 5))
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

/// Low RPM Warning Light (yellow)
/// Output: 0 if light is off, 1 if light is on
pub struct LowRpmInd(Led);
impl LowRpmInd {
    pub const fn new() -> Self {
        Self(Led::new(5142, 128, 7))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MainFuelSwValue {
    Select1,
    Select2,
}
pub type MainFuelSwValueCallback<'a> = &'a mut dyn FnMut(MainFuelSwValue);
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
pub struct MainFuelSw(Selector);
impl MainFuelSw {
    pub const fn new() -> Self {
        Self(Selector::new(5304, 512, 9))
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
pub enum MainGenCoverValue {
    Select1,
    Select2,
}
pub type MainGenCoverValueCallback<'a> = &'a mut dyn FnMut(MainGenCoverValue);
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
pub struct MainGenCover(Selector);
impl MainGenCover {
    pub const fn new() -> Self {
        Self(Selector::new(5304, 16, 4))
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

/// Main Generator Load
/// Output: gauge position
pub struct MainGenLoad(AnalogGauge);
impl MainGenLoad {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5232, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MainGenSwValue {
    Select1,
    Select2,
    Select3,
}
pub type MainGenSwValueCallback<'a> = &'a mut dyn FnMut(MainGenSwValue);
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
pub struct MainGenSw(Selector);
impl MainGenSw {
    pub const fn new() -> Self {
        Self(Selector::new(5304, 12, 2))
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

/// Marker Beacon
/// Output: gauge position
pub struct MarkerBeacon(AnalogGauge);
impl MarkerBeacon {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5268, 65535, 0))
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

/// Master Arm OFF - SAFE - ARMED
/// Input: switch to previous or next state
/// Input: set position
/// Output: selector position
pub struct MasterArmSw(Selector);
impl MasterArmSw {
    pub const fn new() -> Self {
        Self(Selector::new(5378, 6144, 11))
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

/// MASTER CAUTION Light (yellow)
/// Output: 0 if light is off, 1 if light is on
pub struct MasterCautionInd(Led);
impl MasterCautionInd {
    pub const fn new() -> Self {
        Self(Led::new(5142, 256, 8))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NavLtsSwValue {
    Select1,
    Select2,
    Select3,
}
pub type NavLtsSwValueCallback<'a> = &'a mut dyn FnMut(NavLtsSwValue);
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
pub struct NavLtsSw(Selector);
impl NavLtsSw {
    pub const fn new() -> Self {
        Self(Selector::new(5372, 6144, 11))
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
pub enum NonEssBusSwValue {
    Select1,
    Select2,
}
pub type NonEssBusSwValueCallback<'a> = &'a mut dyn FnMut(NonEssBusSwValue);
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
pub struct NonEssBusSw(Selector);
impl NonEssBusSw {
    pub const fn new() -> Self {
        Self(Selector::new(5142, 32768, 15))
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
pub enum NvgPosLtsValue {
    Select1,
    Select2,
    Select3,
    Select4,
    Select5,
    Select6,
}
pub type NvgPosLtsValueCallback<'a> = &'a mut dyn FnMut(NvgPosLtsValue);
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
pub struct NvgPosLts(Selector);
impl NvgPosLts {
    pub const fn new() -> Self {
        Self(Selector::new(5372, 1792, 8))
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

/// Off Warning Flag
/// Output: gauge position
pub struct OadiOff(AnalogGauge);
impl OadiOff {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5264, 65535, 0))
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
pub struct OadiPitch(AnalogGauge);
impl OadiPitch {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5262, 65535, 0))
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
pub struct OadiRoll(AnalogGauge);
impl OadiRoll {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5260, 65535, 0))
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

/// 100 Foot Counter
/// Output: gauge position
pub struct Oalt100(AnalogGauge);
impl Oalt100 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5186, 65535, 0))
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
pub struct Oalt1000(AnalogGauge);
impl Oalt1000 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5184, 65535, 0))
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
pub struct Oalt10000(AnalogGauge);
impl Oalt10000 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5182, 65535, 0))
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
pub struct OaltCodeOffFlag(AnalogGauge);
impl OaltCodeOffFlag {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5194, 65535, 0))
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
pub struct OaltPress0(AnalogGauge);
impl OaltPress0 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5192, 65535, 0))
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
pub struct OaltPress1(AnalogGauge);
impl OaltPress1 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5190, 65535, 0))
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
pub struct OaltPress2(AnalogGauge);
impl OaltPress2 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5188, 65535, 0))
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
pub struct OaltPtr(AnalogGauge);
impl OaltPtr {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5180, 65535, 0))
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

/// Off Warning Flag
/// Output: gauge position
pub struct PadiOff(AnalogGauge);
impl PadiOff {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5258, 65535, 0))
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
pub struct PadiPitch(AnalogGauge);
impl PadiPitch {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5256, 65535, 0))
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
pub struct PadiRoll(AnalogGauge);
impl PadiRoll {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5254, 65535, 0))
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

/// 100 Foot Counter
/// Output: gauge position
pub struct Palt100(AnalogGauge);
impl Palt100 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5200, 65535, 0))
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
pub struct Palt1000(AnalogGauge);
impl Palt1000 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5198, 65535, 0))
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
pub struct Palt10000(AnalogGauge);
impl Palt10000 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5196, 65535, 0))
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
pub struct PaltPress(AnalogGauge);
impl PaltPress {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5202, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PitotHeatValue {
    Select1,
    Select2,
}
pub type PitotHeatValueCallback<'a> = &'a mut dyn FnMut(PitotHeatValue);
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
pub struct PitotHeat(Selector);
impl PitotHeat {
    pub const fn new() -> Self {
        Self(Selector::new(5304, 2, 1))
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
pub type PosLtsSwValueCallback<'a> = &'a mut dyn FnMut(PosLtsSwValue);
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
pub struct PosLtsSw(Selector);
impl PosLtsSw {
    pub const fn new() -> Self {
        Self(Selector::new(5372, 8192, 13))
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

/// Radar Altimeter High Setting
/// Input: turn the dial left or right
/// Output: the rotation of the knob in the cockpit (not the value that is controlled by this knob!)
pub struct RadarAltHi(AnalogDial);
impl RadarAltHi {
    pub const fn new() -> Self {
        Self(AnalogDial::new(5414, 65535, 0))
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
pub struct RadarAltLo(AnalogDial);
impl RadarAltLo {
    pub const fn new() -> Self {
        Self(AnalogDial::new(5412, 65535, 0))
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
pub enum RadarAltPwrValue {
    Select1,
    Select2,
}
pub type RadarAltPwrValueCallback<'a> = &'a mut dyn FnMut(RadarAltPwrValue);
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
pub struct RadarAltPwr(Selector);
impl RadarAltPwr {
    pub const fn new() -> Self {
        Self(Selector::new(5406, 1024, 10))
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
pub enum RadarAltTestValue {
    Select1,
    Select2,
}
pub type RadarAltTestValueCallback<'a> = &'a mut dyn FnMut(RadarAltTestValue);
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
pub struct RadarAltTest(Selector);
impl RadarAltTest {
    pub const fn new() -> Self {
        Self(Selector::new(5406, 2048, 11))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RadioIcsSwValue {
    Select1,
    Select2,
}
pub type RadioIcsSwValueCallback<'a> = &'a mut dyn FnMut(RadioIcsSwValue);
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
pub struct RadioIcsSw(Selector);
impl RadioIcsSw {
    pub const fn new() -> Self {
        Self(Selector::new(5330, 32768, 15))
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

/// Radar Altimeter 1.Digit
/// Output: gauge position
pub struct RaltDigit1(AnalogGauge);
impl RaltDigit1 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5422, 65535, 0))
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
pub struct RaltDigit2(AnalogGauge);
impl RaltDigit2 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5424, 65535, 0))
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
pub struct RaltDigit3(AnalogGauge);
impl RaltDigit3 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5426, 65535, 0))
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
pub struct RaltDigit4(AnalogGauge);
impl RaltDigit4 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5428, 65535, 0))
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
pub struct RaltDisplay(Metadata);
impl RaltDisplay {
    pub const fn new() -> Self {
        Self(Metadata::new(5420, 65535, 0))
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
pub struct RaltDisplayStr(Display<4>);
impl RaltDisplayStr {
    pub const fn new() -> Self {
        Self(Display::<4>::new(5290, 4))
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
pub struct RaltHiIdx(AnalogGauge);
impl RaltHiIdx {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5288, 65535, 0))
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
pub struct RaltHiLamp(Led);
impl RaltHiLamp {
    pub const fn new() -> Self {
        Self(Led::new(5142, 4096, 12))
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
pub struct RaltLoIdx(AnalogGauge);
impl RaltLoIdx {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5286, 65535, 0))
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
pub struct RaltLoLamp(Led);
impl RaltLoLamp {
    pub const fn new() -> Self {
        Self(Led::new(5142, 2048, 11))
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
pub struct RaltNeedle(AnalogGauge);
impl RaltNeedle {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5282, 65535, 0))
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
pub struct RaltOff(AnalogGauge);
impl RaltOff {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5284, 65535, 0))
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

/// RamTemp
/// Output: gauge position
pub struct RamTemp(AnalogGauge);
impl RamTemp {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5280, 65535, 0))
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

/// RMI Course Pointer 1
/// Output: gauge position
pub struct RmiCrs1(AnalogGauge);
impl RmiCrs1 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5174, 65535, 0))
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
pub struct RmiCrs2(AnalogGauge);
impl RmiCrs2 {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5176, 65535, 0))
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
pub struct RmiHdg(AnalogGauge);
impl RmiHdg {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5178, 65535, 0))
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
pub type RocketPairValueCallback<'a> = &'a mut dyn FnMut(RocketPairValue);
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
pub struct RocketPair(Selector);
impl RocketPair {
    pub const fn new() -> Self {
        Self(Selector::new(5398, 28, 2))
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
pub type RocketResetValueCallback<'a> = &'a mut dyn FnMut(RocketResetValue);
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
pub struct RocketReset(Selector);
impl RocketReset {
    pub const fn new() -> Self {
        Self(Selector::new(5378, 32768, 15))
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
pub type RocketSelValueCallback<'a> = &'a mut dyn FnMut(RocketSelValue);
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
pub struct RocketSel(Selector);
impl RocketSel {
    pub const fn new() -> Self {
        Self(Selector::new(5398, 3, 0))
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

/// Rotor RPM
/// Output: gauge position
pub struct RotorRpm(AnalogGauge);
impl RotorRpm {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5220, 65535, 0))
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

/// SAFE Indicator (green)
/// Output: 0 if light is off, 1 if light is on
pub struct SafeInd(Led);
impl SafeInd {
    pub const fn new() -> Self {
        Self(Led::new(5142, 32, 5))
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

/// Standby Compass Bank
/// Output: gauge position
pub struct SbyCompassBank(AnalogGauge);
impl SbyCompassBank {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5438, 65535, 0))
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
pub struct SbyCompassHdg(AnalogGauge);
impl SbyCompassHdg {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5434, 65535, 0))
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
pub struct SbyCompassPitch(AnalogGauge);
impl SbyCompassPitch {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5436, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SearchLightSwValue {
    Select1,
    Select2,
    Select3,
}
pub type SearchLightSwValueCallback<'a> = &'a mut dyn FnMut(SearchLightSwValue);
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
pub struct SearchLightSw(Selector);
impl SearchLightSw {
    pub const fn new() -> Self {
        Self(Selector::new(5378, 3, 0))
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
pub type SearchLightSw2ValueCallback<'a> = &'a mut dyn FnMut(SearchLightSw2Value);
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
pub struct SearchLightSw2(Selector);
impl SearchLightSw2 {
    pub const fn new() -> Self {
        Self(Selector::new(5432, 24, 3))
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

/// Side Slip
/// Output: gauge position
pub struct Sideslip(AnalogGauge);
impl Sideslip {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5248, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SightArmPltValue {
    Select1,
    Select2,
}
pub type SightArmPltValueCallback<'a> = &'a mut dyn FnMut(SightArmPltValue);
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
pub struct SightArmPlt(Selector);
impl SightArmPlt {
    pub const fn new() -> Self {
        Self(Selector::new(5398, 512, 9))
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
pub struct SightElevPlt(LimitedDial);
impl SightElevPlt {
    pub const fn new() -> Self {
        Self(LimitedDial::new(5404, 65535, 0))
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
pub struct SightIntenCplt(LimitedDial);
impl SightIntenCplt {
    pub const fn new() -> Self {
        Self(LimitedDial::new(5400, 65535, 0))
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
pub struct SightIntenPlt(LimitedDial);
impl SightIntenPlt {
    pub const fn new() -> Self {
        Self(LimitedDial::new(5402, 65535, 0))
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
pub type SightLampSwValueCallback<'a> = &'a mut dyn FnMut(SightLampSwValue);
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
pub struct SightLampSw(Selector);
impl SightLampSw {
    pub const fn new() -> Self {
        Self(Selector::new(5398, 384, 7))
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
pub type SightPwrPltValueCallback<'a> = &'a mut dyn FnMut(SightPwrPltValue);
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
pub struct SightPwrPlt(Selector);
impl SightPwrPlt {
    pub const fn new() -> Self {
        Self(Selector::new(5398, 1024, 10))
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

/// SIGHTS_FOR_CIVIL / controllers.Civil_Heli
/// Output: gauge position
pub struct SightsForCivil(AnalogGauge);
impl SightsForCivil {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5416, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StarterGenSwValue {
    Select1,
    Select2,
}
pub type StarterGenSwValueCallback<'a> = &'a mut dyn FnMut(StarterGenSwValue);
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
pub struct StarterGenSw(Selector);
impl StarterGenSw {
    pub const fn new() -> Self {
        Self(Selector::new(5142, 16384, 14))
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

/// STBY Generator Load
/// Output: gauge position
pub struct StbyGenLoad(AnalogGauge);
impl StbyGenLoad {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5234, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TestFuelGaugeBtnValue {
    Select1,
    Select2,
}
pub type TestFuelGaugeBtnValueCallback<'a> = &'a mut dyn FnMut(TestFuelGaugeBtnValue);
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
pub struct TestFuelGaugeBtn(Selector);
impl TestFuelGaugeBtn {
    pub const fn new() -> Self {
        Self(Selector::new(5304, 1024, 10))
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

/// Throttle Axis
/// Input: set the position of the dial
/// Input: turn the dial left or right
/// Output: position of the potentiometer
pub struct Throttle(LimitedDial);
impl Throttle {
    pub const fn new() -> Self {
        Self(LimitedDial::new(5318, 65535, 0))
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
pub struct Throttle2(LimitedDial);
impl Throttle2 {
    pub const fn new() -> Self {
        Self(LimitedDial::new(5430, 65535, 0))
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
pub type ThrottleStopValueCallback<'a> = &'a mut dyn FnMut(ThrottleStopValue);
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
pub struct ThrottleStop(Selector);
impl ThrottleStop {
    pub const fn new() -> Self {
        Self(Selector::new(5310, 8, 3))
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
pub type ThrottleStop2ValueCallback<'a> = &'a mut dyn FnMut(ThrottleStop2Value);
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
pub struct ThrottleStop2(Selector);
impl ThrottleStop2 {
    pub const fn new() -> Self {
        Self(Selector::new(5418, 32768, 15))
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

/// Torque Pressure
/// Output: gauge position
pub struct TorquePress(AnalogGauge);
impl TorquePress {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5226, 65535, 0))
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
pub struct TransOilPress(AnalogGauge);
impl TransOilPress {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5208, 65535, 0))
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
pub struct TransOilTemp(AnalogGauge);
impl TransOilTemp {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5210, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrimCpltValue {
    Select1,
    Select2,
}
pub type TrimCpltValueCallback<'a> = &'a mut dyn FnMut(TrimCpltValue);
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
pub struct TrimCplt(Selector);
impl TrimCplt {
    pub const fn new() -> Self {
        Self(Selector::new(5406, 2, 1))
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
pub type TrimPltValueCallback<'a> = &'a mut dyn FnMut(TrimPltValue);
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
pub struct TrimPlt(Selector);
impl TrimPlt {
    pub const fn new() -> Self {
        Self(Selector::new(5406, 1, 0))
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

/// Turn Indicator
/// Output: gauge position
pub struct TurnPtr(AnalogGauge);
impl TurnPtr {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5246, 65535, 0))
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

/// UHF Frequency
/// Output: UHF Frequency
pub struct UhfFreq(Display<6>);
impl UhfFreq {
    pub const fn new() -> Self {
        Self(Display::<6>::new(5346, 6))
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
pub type UhfFunctionValueCallback<'a> = &'a mut dyn FnMut(UhfFunctionValue);
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
pub struct UhfFunction(Selector);
impl UhfFunction {
    pub const fn new() -> Self {
        Self(Selector::new(5352, 12, 2))
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
pub type UhfModeValueCallback<'a> = &'a mut dyn FnMut(UhfModeValue);
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
pub struct UhfMode(Selector);
impl UhfMode {
    pub const fn new() -> Self {
        Self(Selector::new(5352, 3, 0))
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
pub struct UhfPresetInt(Selector);
impl UhfPresetInt {
    pub const fn new() -> Self {
        Self(Selector::new(5340, 63488, 11))
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
pub struct UhfPresetStr(Display<2>);
impl UhfPresetStr {
    pub const fn new() -> Self {
        Self(Display::<2>::new(5344, 2))
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

/// Squelch Switch
/// Input: switch to previous or next state
/// Input: set position
/// Input: Toggle switch state
/// Output: selector position
pub struct UhfSquelchSw(Selector);
impl UhfSquelchSw {
    pub const fn new() -> Self {
        Self(Selector::new(5352, 16, 4))
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
pub struct UhfVol(LimitedDial);
impl UhfVol {
    pub const fn new() -> Self {
        Self(LimitedDial::new(5354, 65535, 0))
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

/// VHF Frequency
/// Output: VHF Frequency
pub struct VhfcommFreq(Display<7>);
impl VhfcommFreq {
    pub const fn new() -> Self {
        Self(Display::<7>::new(5334, 7))
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
pub struct VhfcommKhz(Selector);
impl VhfcommKhz {
    pub const fn new() -> Self {
        Self(Selector::new(5330, 480, 5))
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
pub struct VhfcommMhz(Selector);
impl VhfcommMhz {
    pub const fn new() -> Self {
        Self(Selector::new(5330, 30, 1))
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
pub type VhfcommPwrValueCallback<'a> = &'a mut dyn FnMut(VhfcommPwrValue);
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
pub struct VhfcommPwr(Selector);
impl VhfcommPwr {
    pub const fn new() -> Self {
        Self(Selector::new(5330, 1, 0))
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
pub type VhfcommTestSwValueCallback<'a> = &'a mut dyn FnMut(VhfcommTestSwValue);
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
pub struct VhfcommTestSw(Selector);
impl VhfcommTestSw {
    pub const fn new() -> Self {
        Self(Selector::new(5310, 32768, 15))
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
pub struct VhfcommVol(LimitedDial);
impl VhfcommVol {
    pub const fn new() -> Self {
        Self(LimitedDial::new(5332, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VhffmFreq1IntValue {
    Select1,
    Select2,
    Select3,
    Select4,
    Select5,
}
pub type VhffmFreq1IntValueCallback<'a> = &'a mut dyn FnMut(VhffmFreq1IntValue);
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
pub struct VhffmFreq1Int(Selector);
impl VhffmFreq1Int {
    pub const fn new() -> Self {
        Self(Selector::new(5352, 224, 5))
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
pub struct VhffmFreq1Str(Display<1>);
impl VhffmFreq1Str {
    pub const fn new() -> Self {
        Self(Display::<1>::new(5356, 1))
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
pub type VhffmFreq2ValueCallback<'a> = &'a mut dyn FnMut(VhffmFreq2Value);
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
pub struct VhffmFreq2(Selector);
impl VhffmFreq2 {
    pub const fn new() -> Self {
        Self(Selector::new(5352, 3840, 8))
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
pub type VhffmFreq3ValueCallback<'a> = &'a mut dyn FnMut(VhffmFreq3Value);
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
pub struct VhffmFreq3(Selector);
impl VhffmFreq3 {
    pub const fn new() -> Self {
        Self(Selector::new(5352, 61440, 12))
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
pub type VhffmFreq4IntValueCallback<'a> = &'a mut dyn FnMut(VhffmFreq4IntValue);
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
pub struct VhffmFreq4Int(Selector);
impl VhffmFreq4Int {
    pub const fn new() -> Self {
        Self(Selector::new(5356, 256, 8))
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
pub struct VhffmFreq4Str(Display<1>);
impl VhffmFreq4Str {
    pub const fn new() -> Self {
        Self(Display::<1>::new(5358, 1))
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
pub type VhffmModeValueCallback<'a> = &'a mut dyn FnMut(VhffmModeValue);
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
pub struct VhffmMode(Selector);
impl VhffmMode {
    pub const fn new() -> Self {
        Self(Selector::new(5356, 1536, 9))
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
pub type VhffmSquelchValueCallback<'a> = &'a mut dyn FnMut(VhffmSquelchValue);
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
pub struct VhffmSquelch(Selector);
impl VhffmSquelch {
    pub const fn new() -> Self {
        Self(Selector::new(5356, 6144, 11))
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
pub struct VhffmVol(LimitedDial);
impl VhffmVol {
    pub const fn new() -> Self {
        Self(LimitedDial::new(5360, 65535, 0))
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

/// VHF NAV Frequency
/// Output: VHF NAV Frequency
pub struct VhfnavFreq(Display<6>);
impl VhfnavFreq {
    pub const fn new() -> Self {
        Self(Display::<6>::new(5362, 6))
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
pub struct VhfnavKhz(Selector);
impl VhfnavKhz {
    pub const fn new() -> Self {
        Self(Selector::new(5358, 61440, 12))
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
pub struct VhfnavMhz(Selector);
impl VhfnavMhz {
    pub const fn new() -> Self {
        Self(Selector::new(5358, 3840, 8))
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
pub type VhfnavPwrValueCallback<'a> = &'a mut dyn FnMut(VhfnavPwrValue);
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
pub struct VhfnavPwr(Selector);
impl VhfnavPwr {
    pub const fn new() -> Self {
        Self(Selector::new(5356, 24576, 13))
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
pub struct VhfnavVol(LimitedDial);
impl VhfnavVol {
    pub const fn new() -> Self {
        Self(LimitedDial::new(5368, 65535, 0))
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

/// Vertical Velocity Indicator (Operator)
/// Output: gauge position
pub struct VviO(AnalogGauge);
impl VviO {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5252, 65535, 0))
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
pub struct VviP(AnalogGauge);
impl VviP {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5250, 65535, 0))
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

/// Windshield Wiper (Operator)
/// Output: gauge position
pub struct WiperO(AnalogGauge);
impl WiperO {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5272, 65535, 0))
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
pub struct WiperP(AnalogGauge);
impl WiperP {
    pub const fn new() -> Self {
        Self(AnalogGauge::new(5270, 65535, 0))
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WiperSelValue {
    Select1,
    Select2,
    Select3,
}
pub type WiperSelValueCallback<'a> = &'a mut dyn FnMut(WiperSelValue);
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
pub struct WiperSel(Selector);
impl WiperSel {
    pub const fn new() -> Self {
        Self(Selector::new(5398, 6144, 11))
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
pub struct WiperSpd(Selector);
impl WiperSpd {
    pub const fn new() -> Self {
        Self(Selector::new(5398, 57344, 13))
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

/// X130 Armed Lamp (blue)
/// Output: 0 if light is off, 1 if light is on
pub struct X130Armed(Led);
impl X130Armed {
    pub const fn new() -> Self {
        Self(Led::new(5406, 4096, 12))
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

pub enum RegisteredControl<'a> {
    AcGen1Current {
        control: AcGen1Current,
        callback: IntegerCallback<'a>,
    },
    AcGen2Current {
        control: AcGen2Current,
        callback: IntegerCallback<'a>,
    },
    AcV {
        control: AcV,
        callback: IntegerCallback<'a>,
    },
    AcVmSrc {
        control: AcVmSrc,
        callback: AcVmSrcValueCallback<'a>,
    },
    AcVoltage {
        control: AcVoltage,
        callback: IntegerCallback<'a>,
    },
    AdfBand {
        control: AdfBand,
        callback: AdfBandValueCallback<'a>,
    },
    AdfBfoSw {
        control: AdfBfoSw,
        callback: AdfBfoSwValueCallback<'a>,
    },
    AdfFreq {
        control: AdfFreq,
        callback: IntegerCallback<'a>,
    },
    AdfGain {
        control: AdfGain,
        callback: IntegerCallback<'a>,
    },
    AdfLoopLr {
        control: AdfLoopLr,
        callback: AdfLoopLrValueCallback<'a>,
    },
    AdfMode {
        control: AdfMode,
        callback: AdfModeValueCallback<'a>,
    },
    AdfSignal {
        control: AdfSignal,
        callback: IntegerCallback<'a>,
    },
    AdfTune {
        control: AdfTune,
        callback: IntegerCallback<'a>,
    },
    AdfVorSw {
        control: AdfVorSw,
        callback: AdfVorSwValueCallback<'a>,
    },
    AdiPitchShift {
        control: AdiPitchShift,
        callback: IntegerCallback<'a>,
    },
    AdiPitchTrim {
        control: AdiPitchTrim,
        callback: IntegerCallback<'a>,
    },
    AdiPitchTrimCplt {
        control: AdiPitchTrimCplt,
        callback: IntegerCallback<'a>,
    },
    AdiRollTrim {
        control: AdiRollTrim,
        callback: IntegerCallback<'a>,
    },
    AltAdjCplt {
        control: AltAdjCplt,
        callback: IntegerCallback<'a>,
    },
    AltAdjPlt {
        control: AltAdjPlt,
        callback: IntegerCallback<'a>,
    },
    AntiIceAmp {
        control: AntiIceAmp,
        callback: IntegerCallback<'a>,
    },
    AnticollLtsSw {
        control: AnticollLtsSw,
        callback: AnticollLtsSwValueCallback<'a>,
    },
    ArmedInd {
        control: ArmedInd,
        callback: LedCallback<'a>,
    },
    BatSw {
        control: BatSw,
        callback: BatSwValueCallback<'a>,
    },
    BeaconSensingSw {
        control: BeaconSensingSw,
        callback: BeaconSensingSwValueCallback<'a>,
    },
    BeaconVol {
        control: BeaconVol,
        callback: IntegerCallback<'a>,
    },
    BleedAirSw {
        control: BleedAirSw,
        callback: BleedAirSwValueCallback<'a>,
    },
    BrtConsole {
        control: BrtConsole,
        callback: IntegerCallback<'a>,
    },
    BrtCopilot {
        control: BrtCopilot,
        callback: IntegerCallback<'a>,
    },
    BrtEngine {
        control: BrtEngine,
        callback: IntegerCallback<'a>,
    },
    BrtPed {
        control: BrtPed,
        callback: IntegerCallback<'a>,
    },
    BrtPilot {
        control: BrtPilot,
        callback: IntegerCallback<'a>,
    },
    BrtSec {
        control: BrtSec,
        callback: IntegerCallback<'a>,
    },
    CargoRelCp {
        control: CargoRelCp,
        callback: CargoRelCpValueCallback<'a>,
    },
    CargoRelP {
        control: CargoRelP,
        callback: CargoRelPValueCallback<'a>,
    },
    CargoRelSafe {
        control: CargoRelSafe,
        callback: CargoRelSafeValueCallback<'a>,
    },
    Cb28v {
        control: Cb28v,
        callback: Cb28vValueCallback<'a>,
    },
    CbAirValve {
        control: CbAirValve,
        callback: CbAirValveValueCallback<'a>,
    },
    CbAmRadio {
        control: CbAmRadio,
        callback: CbAmRadioValueCallback<'a>,
    },
    CbAntiIce {
        control: CbAntiIce,
        callback: CbAntiIceValueCallback<'a>,
    },
    CbAnticoll {
        control: CbAnticoll,
        callback: CbAnticollValueCallback<'a>,
    },
    CbArc102 {
        control: CbArc102,
        callback: CbArc102ValueCallback<'a>,
    },
    CbCable {
        control: CbCable,
        callback: CbCableValueCallback<'a>,
    },
    CbCautionLts {
        control: CbCautionLts,
        callback: CbCautionLtsValueCallback<'a>,
    },
    CbCockpitLts {
        control: CbCockpitLts,
        callback: CbCockpitLtsValueCallback<'a>,
    },
    CbConsoleLts {
        control: CbConsoleLts,
        callback: CbConsoleLtsValueCallback<'a>,
    },
    CbCpltAttd1 {
        control: CbCpltAttd1,
        callback: CbCpltAttd1ValueCallback<'a>,
    },
    CbCpltAttd2 {
        control: CbCpltAttd2,
        callback: CbCpltAttd2ValueCallback<'a>,
    },
    CbCrsind {
        control: CbCrsind,
        callback: CbCrsindValueCallback<'a>,
    },
    CbDomeLts {
        control: CbDomeLts,
        callback: CbDomeLtsValueCallback<'a>,
    },
    CbEng {
        control: CbEng,
        callback: CbEngValueCallback<'a>,
    },
    CbFailRelay {
        control: CbFailRelay,
        callback: CbFailRelayValueCallback<'a>,
    },
    CbFire {
        control: CbFire,
        callback: CbFireValueCallback<'a>,
    },
    CbFm2Radio {
        control: CbFm2Radio,
        callback: CbFm2RadioValueCallback<'a>,
    },
    CbFmRadio {
        control: CbFmRadio,
        callback: CbFmRadioValueCallback<'a>,
    },
    CbFuel {
        control: CbFuel,
        callback: CbFuelValueCallback<'a>,
    },
    CbFuelQty {
        control: CbFuelQty,
        callback: CbFuelQtyValueCallback<'a>,
    },
    CbFuelTrans {
        control: CbFuelTrans,
        callback: CbFuelTransValueCallback<'a>,
    },
    CbFuelValves {
        control: CbFuelValves,
        callback: CbFuelValvesValueCallback<'a>,
    },
    CbFusLts {
        control: CbFusLts,
        callback: CbFusLtsValueCallback<'a>,
    },
    CbGenBus {
        control: CbGenBus,
        callback: CbGenBusValueCallback<'a>,
    },
    CbGov {
        control: CbGov,
        callback: CbGovValueCallback<'a>,
    },
    CbGyro {
        control: CbGyro,
        callback: CbGyroValueCallback<'a>,
    },
    CbHeatedBlanket1 {
        control: CbHeatedBlanket1,
        callback: CbHeatedBlanket1ValueCallback<'a>,
    },
    CbHeatedBlanket2 {
        control: CbHeatedBlanket2,
        callback: CbHeatedBlanket2ValueCallback<'a>,
    },
    CbHfAnt {
        control: CbHfAnt,
        callback: CbHfAntValueCallback<'a>,
    },
    CbHfArc102 {
        control: CbHfArc102,
        callback: CbHfArc102ValueCallback<'a>,
    },
    CbHook {
        control: CbHook,
        callback: CbHookValueCallback<'a>,
    },
    CbHydctl {
        control: CbHydctl,
        callback: CbHydctlValueCallback<'a>,
    },
    CbIdleStop {
        control: CbIdleStop,
        callback: CbIdleStopValueCallback<'a>,
    },
    CbIffApx1 {
        control: CbIffApx1,
        callback: CbIffApx1ValueCallback<'a>,
    },
    CbIffApx2 {
        control: CbIffApx2,
        callback: CbIffApx2ValueCallback<'a>,
    },
    CbIgn {
        control: CbIgn,
        callback: CbIgnValueCallback<'a>,
    },
    CbInstLts {
        control: CbInstLts,
        callback: CbInstLtsValueCallback<'a>,
    },
    CbInstSecLts {
        control: CbInstSecLts,
        callback: CbInstSecLtsValueCallback<'a>,
    },
    CbIntCplt {
        control: CbIntCplt,
        callback: CbIntCpltValueCallback<'a>,
    },
    CbIntPlt {
        control: CbIntPlt,
        callback: CbIntPltValueCallback<'a>,
    },
    CbInvtr {
        control: CbInvtr,
        callback: CbInvtrValueCallback<'a>,
    },
    CbJtsn {
        control: CbJtsn,
        callback: CbJtsnValueCallback<'a>,
    },
    CbKy20 {
        control: CbKy20,
        callback: CbKy20ValueCallback<'a>,
    },
    CbLdglts {
        control: CbLdglts,
        callback: CbLdgltsValueCallback<'a>,
    },
    CbLfNav {
        control: CbLfNav,
        callback: CbLfNavValueCallback<'a>,
    },
    CbLhFuel {
        control: CbLhFuel,
        callback: CbLhFuelValueCallback<'a>,
    },
    CbMainInvtr {
        control: CbMainInvtr,
        callback: CbMainInvtrValueCallback<'a>,
    },
    CbMarkerBeacon {
        control: CbMarkerBeacon,
        callback: CbMarkerBeaconValueCallback<'a>,
    },
    CbNavLts {
        control: CbNavLts,
        callback: CbNavLtsValueCallback<'a>,
    },
    CbOutletValve {
        control: CbOutletValve,
        callback: CbOutletValveValueCallback<'a>,
    },
    CbPitotTube {
        control: CbPitotTube,
        callback: CbPitotTubeValueCallback<'a>,
    },
    CbPltAttd1 {
        control: CbPltAttd1,
        callback: CbPltAttd1ValueCallback<'a>,
    },
    CbPltAttd2 {
        control: CbPltAttd2,
        callback: CbPltAttd2ValueCallback<'a>,
    },
    CbPorxWarn {
        control: CbPorxWarn,
        callback: CbPorxWarnValueCallback<'a>,
    },
    CbRescueCont {
        control: CbRescueCont,
        callback: CbRescueContValueCallback<'a>,
    },
    CbRescuePwr {
        control: CbRescuePwr,
        callback: CbRescuePwrValueCallback<'a>,
    },
    CbRhFuel {
        control: CbRhFuel,
        callback: CbRhFuelValueCallback<'a>,
    },
    CbRpmWarn {
        control: CbRpmWarn,
        callback: CbRpmWarnValueCallback<'a>,
    },
    CbSlCtl {
        control: CbSlCtl,
        callback: CbSlCtlValueCallback<'a>,
    },
    CbSlPwr {
        control: CbSlPwr,
        callback: CbSlPwrValueCallback<'a>,
    },
    CbSpareInvtr {
        control: CbSpareInvtr,
        callback: CbSpareInvtrValueCallback<'a>,
    },
    CbStarter {
        control: CbStarter,
        callback: CbStarterValueCallback<'a>,
    },
    CbStbyGen {
        control: CbStbyGen,
        callback: CbStbyGenValueCallback<'a>,
    },
    CbTemp {
        control: CbTemp,
        callback: CbTempValueCallback<'a>,
    },
    CbTorq {
        control: CbTorq,
        callback: CbTorqValueCallback<'a>,
    },
    CbTrim {
        control: CbTrim,
        callback: CbTrimValueCallback<'a>,
    },
    CbTurnSlip {
        control: CbTurnSlip,
        callback: CbTurnSlipValueCallback<'a>,
    },
    CbUhfRadio {
        control: CbUhfRadio,
        callback: CbUhfRadioValueCallback<'a>,
    },
    CbVhfNav {
        control: CbVhfNav,
        callback: CbVhfNavValueCallback<'a>,
    },
    CbVm {
        control: CbVm,
        callback: CbVmValueCallback<'a>,
    },
    CbWiperCplt {
        control: CbWiperCplt,
        callback: CbWiperCpltValueCallback<'a>,
    },
    CbWiperPlt {
        control: CbWiperPlt,
        callback: CbWiperPltValueCallback<'a>,
    },
    CbXmsn {
        control: CbXmsn,
        callback: CbXmsnValueCallback<'a>,
    },
    ChipDetSw {
        control: ChipDetSw,
        callback: ChipDetSwValueCallback<'a>,
    },
    CiCourseCard {
        control: CiCourseCard,
        callback: IntegerCallback<'a>,
    },
    CiFrom {
        control: CiFrom,
        callback: IntegerCallback<'a>,
    },
    CiHbar {
        control: CiHbar,
        callback: IntegerCallback<'a>,
    },
    CiHoff {
        control: CiHoff,
        callback: IntegerCallback<'a>,
    },
    CiTo {
        control: CiTo,
        callback: IntegerCallback<'a>,
    },
    CiVbar {
        control: CiVbar,
        callback: IntegerCallback<'a>,
    },
    CiVoff {
        control: CiVoff,
        callback: IntegerCallback<'a>,
    },
    ClA0 {
        control: ClA0,
        callback: LedCallback<'a>,
    },
    ClA1 {
        control: ClA1,
        callback: LedCallback<'a>,
    },
    ClB0 {
        control: ClB0,
        callback: LedCallback<'a>,
    },
    ClB1 {
        control: ClB1,
        callback: LedCallback<'a>,
    },
    ClC0 {
        control: ClC0,
        callback: LedCallback<'a>,
    },
    ClC1 {
        control: ClC1,
        callback: LedCallback<'a>,
    },
    ClD0 {
        control: ClD0,
        callback: LedCallback<'a>,
    },
    ClD1 {
        control: ClD1,
        callback: LedCallback<'a>,
    },
    ClE0 {
        control: ClE0,
        callback: LedCallback<'a>,
    },
    ClE1 {
        control: ClE1,
        callback: LedCallback<'a>,
    },
    ClF0 {
        control: ClF0,
        callback: LedCallback<'a>,
    },
    ClF1 {
        control: ClF1,
        callback: LedCallback<'a>,
    },
    ClG0 {
        control: ClG0,
        callback: LedCallback<'a>,
    },
    ClG1 {
        control: ClG1,
        callback: LedCallback<'a>,
    },
    ClH0 {
        control: ClH0,
        callback: LedCallback<'a>,
    },
    ClH1 {
        control: ClH1,
        callback: LedCallback<'a>,
    },
    ClI0 {
        control: ClI0,
        callback: LedCallback<'a>,
    },
    ClI1 {
        control: ClI1,
        callback: LedCallback<'a>,
    },
    ClJ0 {
        control: ClJ0,
        callback: LedCallback<'a>,
    },
    ClJ1 {
        control: ClJ1,
        callback: LedCallback<'a>,
    },
    ClockAdjust {
        control: ClockAdjust,
        callback: IntegerCallback<'a>,
    },
    ClockAdjustPull {
        control: ClockAdjustPull,
        callback: IntegerCallback<'a>,
    },
    ClockH {
        control: ClockH,
        callback: IntegerCallback<'a>,
    },
    ClockM {
        control: ClockM,
        callback: IntegerCallback<'a>,
    },
    ClockS {
        control: ClockS,
        callback: IntegerCallback<'a>,
    },
    ClpBrightDimSw {
        control: ClpBrightDimSw,
        callback: ClpBrightDimSwValueCallback<'a>,
    },
    ClpResetTestSw {
        control: ClpResetTestSw,
        callback: ClpResetTestSwValueCallback<'a>,
    },
    CmArmSw {
        control: CmArmSw,
        callback: CmArmSwValueCallback<'a>,
    },
    CmArmedTest {
        control: CmArmedTest,
        callback: CmArmedTestValueCallback<'a>,
    },
    CmChaffcntDisplay {
        control: CmChaffcntDisplay,
        callback: IntegerCallback<'a>,
    },
    CmChaffcntDisplayStr {
        control: CmChaffcntDisplayStr,
        callback: DisplayCallback<'a>,
    },
    CmChaffcntReset {
        control: CmChaffcntReset,
        callback: CmChaffcntResetValueCallback<'a>,
    },
    CmFlareBtn {
        control: CmFlareBtn,
        callback: CmFlareBtnValueCallback<'a>,
    },
    CmFlarecntDisplay {
        control: CmFlarecntDisplay,
        callback: IntegerCallback<'a>,
    },
    CmFlarecntDisplayStr {
        control: CmFlarecntDisplayStr,
        callback: DisplayCallback<'a>,
    },
    CmFlarecntReset {
        control: CmFlarecntReset,
        callback: CmFlarecntResetValueCallback<'a>,
    },
    CmManPgrmSw {
        control: CmManPgrmSw,
        callback: CmManPgrmSwValueCallback<'a>,
    },
    CmRippleCover {
        control: CmRippleCover,
        callback: CmRippleCoverValueCallback<'a>,
    },
    CmRippleSw {
        control: CmRippleSw,
        callback: CmRippleSwValueCallback<'a>,
    },
    ConsoleLight {
        control: ConsoleLight,
        callback: IntegerCallback<'a>,
    },
    CpltAdiCage {
        control: CpltAdiCage,
        callback: CpltAdiCageValueCallback<'a>,
    },
    CrsKnob {
        control: CrsKnob,
        callback: IntegerCallback<'a>,
    },
    DcBat1Current {
        control: DcBat1Current,
        callback: IntegerCallback<'a>,
    },
    DcBat2Current {
        control: DcBat2Current,
        callback: IntegerCallback<'a>,
    },
    DcV {
        control: DcV,
        callback: IntegerCallback<'a>,
    },
    DcVmSrc {
        control: DcVmSrc,
        callback: DcVmSrcValueCallback<'a>,
    },
    DcVoltage {
        control: DcVoltage,
        callback: IntegerCallback<'a>,
    },
    DcVu1Current {
        control: DcVu1Current,
        callback: IntegerCallback<'a>,
    },
    DcVu2Current {
        control: DcVu2Current,
        callback: IntegerCallback<'a>,
    },
    DcVu3Current {
        control: DcVu3Current,
        callback: IntegerCallback<'a>,
    },
    DomeLight {
        control: DomeLight,
        callback: IntegerCallback<'a>,
    },
    DomeLightGreen {
        control: DomeLightGreen,
        callback: IntegerCallback<'a>,
    },
    DomeLightSw {
        control: DomeLightSw,
        callback: DomeLightSwValueCallback<'a>,
    },
    DoorLPtr {
        control: DoorLPtr,
        callback: DoorLPtrValueCallback<'a>,
    },
    DoorRPtr {
        control: DoorRPtr,
        callback: DoorRPtrValueCallback<'a>,
    },
    EmerGovSw {
        control: EmerGovSw,
        callback: EmerGovSwValueCallback<'a>,
    },
    EngDeice {
        control: EngDeice,
        callback: EngDeiceValueCallback<'a>,
    },
    EngOilPress {
        control: EngOilPress,
        callback: IntegerCallback<'a>,
    },
    EngOilTemp {
        control: EngOilTemp,
        callback: IntegerCallback<'a>,
    },
    EngRpm {
        control: EngRpm,
        callback: IntegerCallback<'a>,
    },
    ExhaustTemp {
        control: ExhaustTemp,
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
    ExtStrobe {
        control: ExtStrobe,
        callback: IntegerCallback<'a>,
    },
    ExtWowSkid {
        control: ExtWowSkid,
        callback: IntegerCallback<'a>,
    },
    FireInd {
        control: FireInd,
        callback: LedCallback<'a>,
    },
    FireTestBtn {
        control: FireTestBtn,
        callback: FireTestBtnValueCallback<'a>,
    },
    ForceTrimSw {
        control: ForceTrimSw,
        callback: ForceTrimSwValueCallback<'a>,
    },
    FuelPress {
        control: FuelPress,
        callback: IntegerCallback<'a>,
    },
    FuelQty {
        control: FuelQty,
        callback: IntegerCallback<'a>,
    },
    GasRpm {
        control: GasRpm,
        callback: IntegerCallback<'a>,
    },
    GasRpmT {
        control: GasRpmT,
        callback: IntegerCallback<'a>,
    },
    GmcAnnunciator {
        control: GmcAnnunciator,
        callback: IntegerCallback<'a>,
    },
    GmcCrs1 {
        control: GmcCrs1,
        callback: IntegerCallback<'a>,
    },
    GmcCrs2 {
        control: GmcCrs2,
        callback: IntegerCallback<'a>,
    },
    GmcHdg {
        control: GmcHdg,
        callback: IntegerCallback<'a>,
    },
    GmcHdgMarker {
        control: GmcHdgMarker,
        callback: IntegerCallback<'a>,
    },
    GmcPwrfail {
        control: GmcPwrfail,
        callback: IntegerCallback<'a>,
    },
    GovRpmSw {
        control: GovRpmSw,
        callback: GovRpmSwValueCallback<'a>,
    },
    GovRpmSw2 {
        control: GovRpmSw2,
        callback: GovRpmSw2ValueCallback<'a>,
    },
    GunSel {
        control: GunSel,
        callback: GunSelValueCallback<'a>,
    },
    GyroModeSw {
        control: GyroModeSw,
        callback: GyroModeSwValueCallback<'a>,
    },
    HdgSet {
        control: HdgSet,
        callback: IntegerCallback<'a>,
    },
    HdgSync {
        control: HdgSync,
        callback: IntegerCallback<'a>,
    },
    HydContSw {
        control: HydContSw,
        callback: HydContSwValueCallback<'a>,
    },
    IasNose {
        control: IasNose,
        callback: IntegerCallback<'a>,
    },
    IasRoof {
        control: IasRoof,
        callback: IntegerCallback<'a>,
    },
    IffCode {
        control: IffCode,
        callback: IntegerCallback<'a>,
    },
    IffMaster {
        control: IffMaster,
        callback: IffMasterValueCallback<'a>,
    },
    IffMicIdent {
        control: IffMicIdent,
        callback: IffMicIdentValueCallback<'a>,
    },
    IffMode1Wheel1 {
        control: IffMode1Wheel1,
        callback: IffMode1Wheel1ValueCallback<'a>,
    },
    IffMode1Wheel2 {
        control: IffMode1Wheel2,
        callback: IffMode1Wheel2ValueCallback<'a>,
    },
    IffMode3aWheel1 {
        control: IffMode3aWheel1,
        callback: IffMode3aWheel1ValueCallback<'a>,
    },
    IffMode3aWheel2 {
        control: IffMode3aWheel2,
        callback: IffMode3aWheel2ValueCallback<'a>,
    },
    IffMode3aWheel3 {
        control: IffMode3aWheel3,
        callback: IffMode3aWheel3ValueCallback<'a>,
    },
    IffMode3aWheel4 {
        control: IffMode3aWheel4,
        callback: IffMode3aWheel4ValueCallback<'a>,
    },
    IffOnOut {
        control: IffOnOut,
        callback: IffOnOutValueCallback<'a>,
    },
    IffOutAudioLight {
        control: IffOutAudioLight,
        callback: IffOutAudioLightValueCallback<'a>,
    },
    IffRadtest {
        control: IffRadtest,
        callback: IffRadtestValueCallback<'a>,
    },
    IffReplyDim {
        control: IffReplyDim,
        callback: IntegerCallback<'a>,
    },
    IffReplyInd {
        control: IffReplyInd,
        callback: LedCallback<'a>,
    },
    IffReplyTest {
        control: IffReplyTest,
        callback: IffReplyTestValueCallback<'a>,
    },
    IffTestDim {
        control: IffTestDim,
        callback: IntegerCallback<'a>,
    },
    IffTestInd {
        control: IffTestInd,
        callback: LedCallback<'a>,
    },
    IffTestM1 {
        control: IffTestM1,
        callback: IffTestM1ValueCallback<'a>,
    },
    IffTestM2 {
        control: IffTestM2,
        callback: IffTestM2ValueCallback<'a>,
    },
    IffTestM3 {
        control: IffTestM3,
        callback: IffTestM3ValueCallback<'a>,
    },
    IffTestM4 {
        control: IffTestM4,
        callback: IffTestM4ValueCallback<'a>,
    },
    IffTestTest {
        control: IffTestTest,
        callback: IffTestTestValueCallback<'a>,
    },
    IntIntSw {
        control: IntIntSw,
        callback: IntIntSwValueCallback<'a>,
    },
    IntMode {
        control: IntMode,
        callback: IntModeValueCallback<'a>,
    },
    IntNavSw {
        control: IntNavSw,
        callback: IntNavSwValueCallback<'a>,
    },
    IntRcvr1Sw {
        control: IntRcvr1Sw,
        callback: IntRcvr1SwValueCallback<'a>,
    },
    IntRcvr2Sw {
        control: IntRcvr2Sw,
        callback: IntRcvr2SwValueCallback<'a>,
    },
    IntRcvr3Sw {
        control: IntRcvr3Sw,
        callback: IntRcvr3SwValueCallback<'a>,
    },
    IntRecvr4Sw {
        control: IntRecvr4Sw,
        callback: IntRecvr4SwValueCallback<'a>,
    },
    IntVol {
        control: IntVol,
        callback: IntegerCallback<'a>,
    },
    InverterSw {
        control: InverterSw,
        callback: InverterSwValueCallback<'a>,
    },
    JtsnBtn {
        control: JtsnBtn,
        callback: JtsnBtnValueCallback<'a>,
    },
    JtsnCover {
        control: JtsnCover,
        callback: JtsnCoverValueCallback<'a>,
    },
    LdgLightSw {
        control: LdgLightSw,
        callback: LdgLightSwValueCallback<'a>,
    },
    LdgLightSw2 {
        control: LdgLightSw2,
        callback: LdgLightSw2ValueCallback<'a>,
    },
    LdgLtCtrl {
        control: LdgLtCtrl,
        callback: LdgLtCtrlValueCallback<'a>,
    },
    LdgLtCtrl2 {
        control: LdgLtCtrl2,
        callback: LdgLtCtrl2ValueCallback<'a>,
    },
    LowRpmAudio {
        control: LowRpmAudio,
        callback: LowRpmAudioValueCallback<'a>,
    },
    LowRpmInd {
        control: LowRpmInd,
        callback: LedCallback<'a>,
    },
    MainFuelSw {
        control: MainFuelSw,
        callback: MainFuelSwValueCallback<'a>,
    },
    MainGenCover {
        control: MainGenCover,
        callback: MainGenCoverValueCallback<'a>,
    },
    MainGenLoad {
        control: MainGenLoad,
        callback: IntegerCallback<'a>,
    },
    MainGenSw {
        control: MainGenSw,
        callback: MainGenSwValueCallback<'a>,
    },
    MarkerBeacon {
        control: MarkerBeacon,
        callback: IntegerCallback<'a>,
    },
    MasterArmSw {
        control: MasterArmSw,
        callback: MasterArmSwValueCallback<'a>,
    },
    MasterCautionInd {
        control: MasterCautionInd,
        callback: LedCallback<'a>,
    },
    NavLtsSw {
        control: NavLtsSw,
        callback: NavLtsSwValueCallback<'a>,
    },
    NonEssBusSw {
        control: NonEssBusSw,
        callback: NonEssBusSwValueCallback<'a>,
    },
    NvgPosLts {
        control: NvgPosLts,
        callback: NvgPosLtsValueCallback<'a>,
    },
    OadiOff {
        control: OadiOff,
        callback: IntegerCallback<'a>,
    },
    OadiPitch {
        control: OadiPitch,
        callback: IntegerCallback<'a>,
    },
    OadiRoll {
        control: OadiRoll,
        callback: IntegerCallback<'a>,
    },
    Oalt100 {
        control: Oalt100,
        callback: IntegerCallback<'a>,
    },
    Oalt1000 {
        control: Oalt1000,
        callback: IntegerCallback<'a>,
    },
    Oalt10000 {
        control: Oalt10000,
        callback: IntegerCallback<'a>,
    },
    OaltCodeOffFlag {
        control: OaltCodeOffFlag,
        callback: IntegerCallback<'a>,
    },
    OaltPress0 {
        control: OaltPress0,
        callback: IntegerCallback<'a>,
    },
    OaltPress1 {
        control: OaltPress1,
        callback: IntegerCallback<'a>,
    },
    OaltPress2 {
        control: OaltPress2,
        callback: IntegerCallback<'a>,
    },
    OaltPtr {
        control: OaltPtr,
        callback: IntegerCallback<'a>,
    },
    PadiOff {
        control: PadiOff,
        callback: IntegerCallback<'a>,
    },
    PadiPitch {
        control: PadiPitch,
        callback: IntegerCallback<'a>,
    },
    PadiRoll {
        control: PadiRoll,
        callback: IntegerCallback<'a>,
    },
    Palt100 {
        control: Palt100,
        callback: IntegerCallback<'a>,
    },
    Palt1000 {
        control: Palt1000,
        callback: IntegerCallback<'a>,
    },
    Palt10000 {
        control: Palt10000,
        callback: IntegerCallback<'a>,
    },
    PaltPress {
        control: PaltPress,
        callback: IntegerCallback<'a>,
    },
    PitotHeat {
        control: PitotHeat,
        callback: PitotHeatValueCallback<'a>,
    },
    PosLtsSw {
        control: PosLtsSw,
        callback: PosLtsSwValueCallback<'a>,
    },
    RadarAltHi {
        control: RadarAltHi,
        callback: IntegerCallback<'a>,
    },
    RadarAltLo {
        control: RadarAltLo,
        callback: IntegerCallback<'a>,
    },
    RadarAltPwr {
        control: RadarAltPwr,
        callback: RadarAltPwrValueCallback<'a>,
    },
    RadarAltTest {
        control: RadarAltTest,
        callback: RadarAltTestValueCallback<'a>,
    },
    RadioIcsSw {
        control: RadioIcsSw,
        callback: RadioIcsSwValueCallback<'a>,
    },
    RaltDigit1 {
        control: RaltDigit1,
        callback: IntegerCallback<'a>,
    },
    RaltDigit2 {
        control: RaltDigit2,
        callback: IntegerCallback<'a>,
    },
    RaltDigit3 {
        control: RaltDigit3,
        callback: IntegerCallback<'a>,
    },
    RaltDigit4 {
        control: RaltDigit4,
        callback: IntegerCallback<'a>,
    },
    RaltDisplay {
        control: RaltDisplay,
        callback: IntegerCallback<'a>,
    },
    RaltDisplayStr {
        control: RaltDisplayStr,
        callback: DisplayCallback<'a>,
    },
    RaltHiIdx {
        control: RaltHiIdx,
        callback: IntegerCallback<'a>,
    },
    RaltHiLamp {
        control: RaltHiLamp,
        callback: LedCallback<'a>,
    },
    RaltLoIdx {
        control: RaltLoIdx,
        callback: IntegerCallback<'a>,
    },
    RaltLoLamp {
        control: RaltLoLamp,
        callback: LedCallback<'a>,
    },
    RaltNeedle {
        control: RaltNeedle,
        callback: IntegerCallback<'a>,
    },
    RaltOff {
        control: RaltOff,
        callback: IntegerCallback<'a>,
    },
    RamTemp {
        control: RamTemp,
        callback: IntegerCallback<'a>,
    },
    RmiCrs1 {
        control: RmiCrs1,
        callback: IntegerCallback<'a>,
    },
    RmiCrs2 {
        control: RmiCrs2,
        callback: IntegerCallback<'a>,
    },
    RmiHdg {
        control: RmiHdg,
        callback: IntegerCallback<'a>,
    },
    RocketPair {
        control: RocketPair,
        callback: RocketPairValueCallback<'a>,
    },
    RocketReset {
        control: RocketReset,
        callback: RocketResetValueCallback<'a>,
    },
    RocketSel {
        control: RocketSel,
        callback: RocketSelValueCallback<'a>,
    },
    RotorRpm {
        control: RotorRpm,
        callback: IntegerCallback<'a>,
    },
    SafeInd {
        control: SafeInd,
        callback: LedCallback<'a>,
    },
    SbyCompassBank {
        control: SbyCompassBank,
        callback: IntegerCallback<'a>,
    },
    SbyCompassHdg {
        control: SbyCompassHdg,
        callback: IntegerCallback<'a>,
    },
    SbyCompassPitch {
        control: SbyCompassPitch,
        callback: IntegerCallback<'a>,
    },
    SearchLightSw {
        control: SearchLightSw,
        callback: SearchLightSwValueCallback<'a>,
    },
    SearchLightSw2 {
        control: SearchLightSw2,
        callback: SearchLightSw2ValueCallback<'a>,
    },
    Sideslip {
        control: Sideslip,
        callback: IntegerCallback<'a>,
    },
    SightArmPlt {
        control: SightArmPlt,
        callback: SightArmPltValueCallback<'a>,
    },
    SightElevPlt {
        control: SightElevPlt,
        callback: IntegerCallback<'a>,
    },
    SightIntenCplt {
        control: SightIntenCplt,
        callback: IntegerCallback<'a>,
    },
    SightIntenPlt {
        control: SightIntenPlt,
        callback: IntegerCallback<'a>,
    },
    SightLampSw {
        control: SightLampSw,
        callback: SightLampSwValueCallback<'a>,
    },
    SightPwrPlt {
        control: SightPwrPlt,
        callback: SightPwrPltValueCallback<'a>,
    },
    SightsForCivil {
        control: SightsForCivil,
        callback: IntegerCallback<'a>,
    },
    StarterGenSw {
        control: StarterGenSw,
        callback: StarterGenSwValueCallback<'a>,
    },
    StbyGenLoad {
        control: StbyGenLoad,
        callback: IntegerCallback<'a>,
    },
    TestFuelGaugeBtn {
        control: TestFuelGaugeBtn,
        callback: TestFuelGaugeBtnValueCallback<'a>,
    },
    Throttle {
        control: Throttle,
        callback: IntegerCallback<'a>,
    },
    Throttle2 {
        control: Throttle2,
        callback: IntegerCallback<'a>,
    },
    ThrottleStop {
        control: ThrottleStop,
        callback: ThrottleStopValueCallback<'a>,
    },
    ThrottleStop2 {
        control: ThrottleStop2,
        callback: ThrottleStop2ValueCallback<'a>,
    },
    TorquePress {
        control: TorquePress,
        callback: IntegerCallback<'a>,
    },
    TransOilPress {
        control: TransOilPress,
        callback: IntegerCallback<'a>,
    },
    TransOilTemp {
        control: TransOilTemp,
        callback: IntegerCallback<'a>,
    },
    TrimCplt {
        control: TrimCplt,
        callback: TrimCpltValueCallback<'a>,
    },
    TrimPlt {
        control: TrimPlt,
        callback: TrimPltValueCallback<'a>,
    },
    TurnPtr {
        control: TurnPtr,
        callback: IntegerCallback<'a>,
    },
    UhfFreq {
        control: UhfFreq,
        callback: DisplayCallback<'a>,
    },
    UhfFunction {
        control: UhfFunction,
        callback: UhfFunctionValueCallback<'a>,
    },
    UhfMode {
        control: UhfMode,
        callback: UhfModeValueCallback<'a>,
    },
    UhfPresetInt {
        control: UhfPresetInt,
        callback: IntegerCallback<'a>,
    },
    UhfPresetStr {
        control: UhfPresetStr,
        callback: DisplayCallback<'a>,
    },
    UhfSquelchSw {
        control: UhfSquelchSw,
        callback: UhfSquelchSwValueCallback<'a>,
    },
    UhfVol {
        control: UhfVol,
        callback: IntegerCallback<'a>,
    },
    VhfcommFreq {
        control: VhfcommFreq,
        callback: DisplayCallback<'a>,
    },
    VhfcommKhz {
        control: VhfcommKhz,
        callback: IntegerCallback<'a>,
    },
    VhfcommMhz {
        control: VhfcommMhz,
        callback: IntegerCallback<'a>,
    },
    VhfcommPwr {
        control: VhfcommPwr,
        callback: VhfcommPwrValueCallback<'a>,
    },
    VhfcommTestSw {
        control: VhfcommTestSw,
        callback: VhfcommTestSwValueCallback<'a>,
    },
    VhfcommVol {
        control: VhfcommVol,
        callback: IntegerCallback<'a>,
    },
    VhffmFreq1Int {
        control: VhffmFreq1Int,
        callback: VhffmFreq1IntValueCallback<'a>,
    },
    VhffmFreq1Str {
        control: VhffmFreq1Str,
        callback: DisplayCallback<'a>,
    },
    VhffmFreq2 {
        control: VhffmFreq2,
        callback: VhffmFreq2ValueCallback<'a>,
    },
    VhffmFreq3 {
        control: VhffmFreq3,
        callback: VhffmFreq3ValueCallback<'a>,
    },
    VhffmFreq4Int {
        control: VhffmFreq4Int,
        callback: VhffmFreq4IntValueCallback<'a>,
    },
    VhffmFreq4Str {
        control: VhffmFreq4Str,
        callback: DisplayCallback<'a>,
    },
    VhffmMode {
        control: VhffmMode,
        callback: VhffmModeValueCallback<'a>,
    },
    VhffmSquelch {
        control: VhffmSquelch,
        callback: VhffmSquelchValueCallback<'a>,
    },
    VhffmVol {
        control: VhffmVol,
        callback: IntegerCallback<'a>,
    },
    VhfnavFreq {
        control: VhfnavFreq,
        callback: DisplayCallback<'a>,
    },
    VhfnavKhz {
        control: VhfnavKhz,
        callback: IntegerCallback<'a>,
    },
    VhfnavMhz {
        control: VhfnavMhz,
        callback: IntegerCallback<'a>,
    },
    VhfnavPwr {
        control: VhfnavPwr,
        callback: VhfnavPwrValueCallback<'a>,
    },
    VhfnavVol {
        control: VhfnavVol,
        callback: IntegerCallback<'a>,
    },
    VviO {
        control: VviO,
        callback: IntegerCallback<'a>,
    },
    VviP {
        control: VviP,
        callback: IntegerCallback<'a>,
    },
    WiperO {
        control: WiperO,
        callback: IntegerCallback<'a>,
    },
    WiperP {
        control: WiperP,
        callback: IntegerCallback<'a>,
    },
    WiperSel {
        control: WiperSel,
        callback: WiperSelValueCallback<'a>,
    },
    WiperSpd {
        control: WiperSpd,
        callback: IntegerCallback<'a>,
    },
    X130Armed {
        control: X130Armed,
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
                    if let Ok(value) =
                        <AcVmSrcValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                    if let Ok(value) =
                        <AdfBandValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::AdfBfoSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <AdfBfoSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
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
                        <AdfLoopLrValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::AdfMode { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <AdfModeValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
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
                        <AdfVorSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                    if let Ok(value) = <AnticollLtsSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
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
                    if let Ok(value) =
                        <BatSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::BeaconSensingSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <BeaconSensingSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
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
                    if let Ok(value) =
                        <BleedAirSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                        <CargoRelCpValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CargoRelP { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CargoRelPValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CargoRelSafe { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <CargoRelSafeValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::Cb28v { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <Cb28vValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbAirValve { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbAirValveValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbAmRadio { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbAmRadioValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbAntiIce { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbAntiIceValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbAnticoll { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbAnticollValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbArc102 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbArc102Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbCable { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbCableValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbCautionLts { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <CbCautionLtsValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::CbCockpitLts { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <CbCockpitLtsValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::CbConsoleLts { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <CbConsoleLtsValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::CbCpltAttd1 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbCpltAttd1Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbCpltAttd2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbCpltAttd2Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbCrsind { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbCrsindValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbDomeLts { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbDomeLtsValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbEng { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbEngValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbFailRelay { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbFailRelayValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbFire { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbFireValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbFm2Radio { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbFm2RadioValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbFmRadio { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbFmRadioValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbFuel { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbFuelValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbFuelQty { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbFuelQtyValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbFuelTrans { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbFuelTransValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbFuelValves { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <CbFuelValvesValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::CbFusLts { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbFusLtsValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbGenBus { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbGenBusValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbGov { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbGovValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbGyro { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbGyroValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbHeatedBlanket1 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbHeatedBlanket1Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CbHeatedBlanket2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbHeatedBlanket2Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CbHfAnt { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbHfAntValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbHfArc102 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbHfArc102Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbHook { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbHookValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbHydctl { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbHydctlValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbIdleStop { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbIdleStopValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbIffApx1 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbIffApx1Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbIffApx2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbIffApx2Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbIgn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbIgnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbInstLts { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbInstLtsValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbInstSecLts { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <CbInstSecLtsValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::CbIntCplt { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbIntCpltValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbIntPlt { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbIntPltValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbInvtr { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbInvtrValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbJtsn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbJtsnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbKy20 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbKy20Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbLdglts { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbLdgltsValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbLfNav { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbLfNavValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbLhFuel { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbLhFuelValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbMainInvtr { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbMainInvtrValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbMarkerBeacon { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbMarkerBeaconValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CbNavLts { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbNavLtsValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbOutletValve { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <CbOutletValveValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::CbPitotTube { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbPitotTubeValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbPltAttd1 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbPltAttd1Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbPltAttd2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbPltAttd2Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbPorxWarn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbPorxWarnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbRescueCont { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <CbRescueContValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::CbRescuePwr { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbRescuePwrValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbRhFuel { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbRhFuelValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbRpmWarn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbRpmWarnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbSlCtl { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbSlCtlValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbSlPwr { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbSlPwrValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbSpareInvtr { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <CbSpareInvtrValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::CbStarter { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbStarterValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbStbyGen { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbStbyGenValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbTemp { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbTempValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbTorq { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbTorqValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbTrim { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbTrimValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbTurnSlip { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbTurnSlipValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbUhfRadio { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbUhfRadioValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbVhfNav { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbVhfNavValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbVm { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbVmValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbWiperCplt { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbWiperCpltValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbWiperPlt { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbWiperPltValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CbXmsn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CbXmsnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::ChipDetSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ChipDetSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                    if let Ok(value) =
                        <ClpBrightDimSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::ClpResetTestSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ClpResetTestSwValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CmArmSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CmArmSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CmArmedTest { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CmArmedTestValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                    if let Ok(value) =
                        <CmChaffcntResetValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CmFlareBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CmFlareBtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                    if let Ok(value) =
                        <CmFlarecntResetValue as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::CmManPgrmSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CmManPgrmSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::CmRippleCover { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <CmRippleCoverValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::CmRippleSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <CmRippleSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                    if let Ok(value) =
                        <CpltAdiCageValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                    if let Ok(value) =
                        <DcVmSrcValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                    if let Ok(value) =
                        <DomeLightSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::DoorLPtr { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <DoorLPtrValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::DoorRPtr { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <DoorRPtrValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::EmerGovSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <EmerGovSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::EngDeice { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <EngDeiceValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                    if let Ok(value) =
                        <FireTestBtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::ForceTrimSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <ForceTrimSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                        <GovRpmSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::GovRpmSw2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <GovRpmSw2Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::GunSel { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <GunSelValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::GyroModeSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <GyroModeSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                    if let Ok(value) =
                        <HydContSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                        <IffMasterValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IffMicIdent { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IffMicIdentValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IffMode1Wheel1 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IffMode1Wheel1Value as core::convert::TryFrom<u16>>::try_from(
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
                        <IffMode1Wheel2Value as core::convert::TryFrom<u16>>::try_from(
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
                        <IffMode3aWheel1Value as core::convert::TryFrom<u16>>::try_from(
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
                        <IffMode3aWheel2Value as core::convert::TryFrom<u16>>::try_from(
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
                        <IffMode3aWheel3Value as core::convert::TryFrom<u16>>::try_from(
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
                        <IffMode3aWheel4Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
                    {
                        callback(value);
                    }
                }
            }
            Self::IffOnOut { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IffOnOutValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IffOutAudioLight { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IffOutAudioLightValue as core::convert::TryFrom<u16>>::try_from(
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
                        <IffRadtestValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                    if let Ok(value) = <IffReplyTestValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
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
                        <IffTestM1Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IffTestM2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IffTestM2Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IffTestM3 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IffTestM3Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IffTestM4 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IffTestM4Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IffTestTest { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IffTestTestValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IntIntSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IntIntSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IntMode { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IntModeValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IntNavSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IntNavSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IntRcvr1Sw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IntRcvr1SwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IntRcvr2Sw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IntRcvr2SwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IntRcvr3Sw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IntRcvr3SwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::IntRecvr4Sw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <IntRecvr4SwValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                        <InverterSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::JtsnBtn { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <JtsnBtnValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::JtsnCover { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <JtsnCoverValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::LdgLightSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <LdgLightSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::LdgLightSw2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <LdgLightSw2Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::LdgLtCtrl { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <LdgLtCtrlValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::LdgLtCtrl2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <LdgLtCtrl2Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::LowRpmAudio { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <LowRpmAudioValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                    if let Ok(value) =
                        <MainFuelSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::MainGenCover { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <MainGenCoverValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
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
                    if let Ok(value) =
                        <MainGenSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                    if let Ok(value) =
                        <MasterArmSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                    if let Ok(value) =
                        <NavLtsSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::NonEssBusSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <NonEssBusSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::NvgPosLts { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <NvgPosLtsValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                    if let Ok(value) =
                        <PitotHeatValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::PosLtsSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <PosLtsSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                    if let Ok(value) =
                        <RadarAltPwrValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RadarAltTest { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <RadarAltTestValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::RadioIcsSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <RadioIcsSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                    if let Ok(value) =
                        <RocketPairValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RocketReset { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <RocketResetValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::RocketSel { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <RocketSelValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                    if let Ok(value) = <SearchLightSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::SearchLightSw2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <SearchLightSw2Value as core::convert::TryFrom<u16>>::try_from(
                            control.value(),
                        )
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
                    if let Ok(value) =
                        <SightArmPltValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                    if let Ok(value) =
                        <SightLampSwValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::SightPwrPlt { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <SightPwrPltValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                    if let Ok(value) = <StarterGenSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
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
                        <TestFuelGaugeBtnValue as core::convert::TryFrom<u16>>::try_from(
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
                    if let Ok(value) = <ThrottleStopValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
                        callback(value);
                    }
                }
            }
            Self::ThrottleStop2 { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <ThrottleStop2Value as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
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
                        <TrimCpltValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::TrimPlt { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <TrimPltValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                        <UhfFunctionValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::UhfMode { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <UhfModeValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                    if let Ok(value) = <UhfSquelchSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
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
                    if let Ok(value) =
                        <VhfcommPwrValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::VhfcommTestSw { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <VhfcommTestSwValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
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
                    if let Ok(value) = <VhffmFreq1IntValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
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
                    if let Ok(value) =
                        <VhffmFreq2Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::VhffmFreq3 { control, callback } => {
                if control.commit() {
                    if let Ok(value) =
                        <VhffmFreq3Value as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::VhffmFreq4Int { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <VhffmFreq4IntValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
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
                        <VhffmModeValue as core::convert::TryFrom<u16>>::try_from(control.value())
                    {
                        callback(value);
                    }
                }
            }
            Self::VhffmSquelch { control, callback } => {
                if control.commit() {
                    if let Ok(value) = <VhffmSquelchValue as core::convert::TryFrom<u16>>::try_from(
                        control.value(),
                    ) {
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
                    if let Ok(value) =
                        <VhfnavPwrValue as core::convert::TryFrom<u16>>::try_from(control.value())
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
                    if let Ok(value) =
                        <WiperSelValue as core::convert::TryFrom<u16>>::try_from(control.value())
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

    pub fn register_ac_gen1_current<F>(
        &mut self,
        control: AcGen1Current,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AcGen1Current { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ac_gen2_current<F>(
        &mut self,
        control: AcGen2Current,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AcGen2Current { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ac_v<F>(
        &mut self,
        control: AcV,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AcV { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ac_vm_src<F>(
        &mut self,
        control: AcVmSrc,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(AcVmSrcValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AcVmSrc { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ac_voltage<F>(
        &mut self,
        control: AcVoltage,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AcVoltage { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_band<F>(
        &mut self,
        control: AdfBand,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(AdfBandValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfBand { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_bfo_sw<F>(
        &mut self,
        control: AdfBfoSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(AdfBfoSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfBfoSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_freq<F>(
        &mut self,
        control: AdfFreq,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfFreq { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_gain<F>(
        &mut self,
        control: AdfGain,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfGain { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_loop_lr<F>(
        &mut self,
        control: AdfLoopLr,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(AdfLoopLrValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfLoopLr { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_mode<F>(
        &mut self,
        control: AdfMode,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(AdfModeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfMode { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_signal<F>(
        &mut self,
        control: AdfSignal,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfSignal { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_tune<F>(
        &mut self,
        control: AdfTune,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfTune { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adf_vor_sw<F>(
        &mut self,
        control: AdfVorSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(AdfVorSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdfVorSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_pitch_shift<F>(
        &mut self,
        control: AdiPitchShift,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiPitchShift { control, callback })
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

    pub fn register_adi_pitch_trim_cplt<F>(
        &mut self,
        control: AdiPitchTrimCplt,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiPitchTrimCplt { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_adi_roll_trim<F>(
        &mut self,
        control: AdiRollTrim,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AdiRollTrim { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_adj_cplt<F>(
        &mut self,
        control: AltAdjCplt,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltAdjCplt { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_alt_adj_plt<F>(
        &mut self,
        control: AltAdjPlt,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AltAdjPlt { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_anti_ice_amp<F>(
        &mut self,
        control: AntiIceAmp,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::AntiIceAmp { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_anticoll_lts_sw<F>(
        &mut self,
        control: AnticollLtsSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(AnticollLtsSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::AnticollLtsSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_armed_ind<F>(
        &mut self,
        control: ArmedInd,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ArmedInd { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_bat_sw<F>(
        &mut self,
        control: BatSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(BatSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::BatSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_beacon_sensing_sw<F>(
        &mut self,
        control: BeaconSensingSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(BeaconSensingSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::BeaconSensingSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_beacon_vol<F>(
        &mut self,
        control: BeaconVol,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BeaconVol { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_bleed_air_sw<F>(
        &mut self,
        control: BleedAirSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(BleedAirSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::BleedAirSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_brt_console<F>(
        &mut self,
        control: BrtConsole,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BrtConsole { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_brt_copilot<F>(
        &mut self,
        control: BrtCopilot,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BrtCopilot { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_brt_engine<F>(
        &mut self,
        control: BrtEngine,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BrtEngine { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_brt_ped<F>(
        &mut self,
        control: BrtPed,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BrtPed { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_brt_pilot<F>(
        &mut self,
        control: BrtPilot,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BrtPilot { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_brt_sec<F>(
        &mut self,
        control: BrtSec,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::BrtSec { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cargo_rel_cp<F>(
        &mut self,
        control: CargoRelCp,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CargoRelCpValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CargoRelCp { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cargo_rel_p<F>(
        &mut self,
        control: CargoRelP,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CargoRelPValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CargoRelP { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cargo_rel_safe<F>(
        &mut self,
        control: CargoRelSafe,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CargoRelSafeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CargoRelSafe { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb28v<F>(
        &mut self,
        control: Cb28v,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(Cb28vValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::Cb28v { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_air_valve<F>(
        &mut self,
        control: CbAirValve,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbAirValveValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbAirValve { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_am_radio<F>(
        &mut self,
        control: CbAmRadio,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbAmRadioValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbAmRadio { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_anti_ice<F>(
        &mut self,
        control: CbAntiIce,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbAntiIceValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbAntiIce { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_anticoll<F>(
        &mut self,
        control: CbAnticoll,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbAnticollValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbAnticoll { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_arc102<F>(
        &mut self,
        control: CbArc102,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbArc102Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbArc102 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_cable<F>(
        &mut self,
        control: CbCable,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbCableValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbCable { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_caution_lts<F>(
        &mut self,
        control: CbCautionLts,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbCautionLtsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbCautionLts { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_cockpit_lts<F>(
        &mut self,
        control: CbCockpitLts,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbCockpitLtsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbCockpitLts { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_console_lts<F>(
        &mut self,
        control: CbConsoleLts,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbConsoleLtsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbConsoleLts { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_cplt_attd1<F>(
        &mut self,
        control: CbCpltAttd1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbCpltAttd1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbCpltAttd1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_cplt_attd2<F>(
        &mut self,
        control: CbCpltAttd2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbCpltAttd2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbCpltAttd2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_crsind<F>(
        &mut self,
        control: CbCrsind,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbCrsindValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbCrsind { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_dome_lts<F>(
        &mut self,
        control: CbDomeLts,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbDomeLtsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbDomeLts { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_eng<F>(
        &mut self,
        control: CbEng,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbEngValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbEng { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_fail_relay<F>(
        &mut self,
        control: CbFailRelay,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbFailRelayValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbFailRelay { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_fire<F>(
        &mut self,
        control: CbFire,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbFireValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbFire { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_fm2_radio<F>(
        &mut self,
        control: CbFm2Radio,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbFm2RadioValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbFm2Radio { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_fm_radio<F>(
        &mut self,
        control: CbFmRadio,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbFmRadioValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbFmRadio { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_fuel<F>(
        &mut self,
        control: CbFuel,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbFuelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbFuel { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_fuel_qty<F>(
        &mut self,
        control: CbFuelQty,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbFuelQtyValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbFuelQty { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_fuel_trans<F>(
        &mut self,
        control: CbFuelTrans,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbFuelTransValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbFuelTrans { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_fuel_valves<F>(
        &mut self,
        control: CbFuelValves,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbFuelValvesValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbFuelValves { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_fus_lts<F>(
        &mut self,
        control: CbFusLts,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbFusLtsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbFusLts { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_gen_bus<F>(
        &mut self,
        control: CbGenBus,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbGenBusValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbGenBus { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_gov<F>(
        &mut self,
        control: CbGov,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbGovValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbGov { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_gyro<F>(
        &mut self,
        control: CbGyro,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbGyroValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbGyro { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_heated_blanket1<F>(
        &mut self,
        control: CbHeatedBlanket1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbHeatedBlanket1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbHeatedBlanket1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_heated_blanket2<F>(
        &mut self,
        control: CbHeatedBlanket2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbHeatedBlanket2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbHeatedBlanket2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_hf_ant<F>(
        &mut self,
        control: CbHfAnt,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbHfAntValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbHfAnt { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_hf_arc102<F>(
        &mut self,
        control: CbHfArc102,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbHfArc102Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbHfArc102 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_hook<F>(
        &mut self,
        control: CbHook,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbHookValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbHook { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_hydctl<F>(
        &mut self,
        control: CbHydctl,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbHydctlValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbHydctl { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_idle_stop<F>(
        &mut self,
        control: CbIdleStop,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbIdleStopValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbIdleStop { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_iff_apx1<F>(
        &mut self,
        control: CbIffApx1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbIffApx1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbIffApx1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_iff_apx2<F>(
        &mut self,
        control: CbIffApx2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbIffApx2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbIffApx2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_ign<F>(
        &mut self,
        control: CbIgn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbIgnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbIgn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_inst_lts<F>(
        &mut self,
        control: CbInstLts,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbInstLtsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbInstLts { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_inst_sec_lts<F>(
        &mut self,
        control: CbInstSecLts,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbInstSecLtsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbInstSecLts { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_int_cplt<F>(
        &mut self,
        control: CbIntCplt,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbIntCpltValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbIntCplt { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_int_plt<F>(
        &mut self,
        control: CbIntPlt,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbIntPltValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbIntPlt { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_invtr<F>(
        &mut self,
        control: CbInvtr,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbInvtrValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbInvtr { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_jtsn<F>(
        &mut self,
        control: CbJtsn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbJtsnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbJtsn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_ky20<F>(
        &mut self,
        control: CbKy20,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbKy20Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbKy20 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_ldglts<F>(
        &mut self,
        control: CbLdglts,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbLdgltsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbLdglts { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_lf_nav<F>(
        &mut self,
        control: CbLfNav,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbLfNavValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbLfNav { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_lh_fuel<F>(
        &mut self,
        control: CbLhFuel,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbLhFuelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbLhFuel { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_main_invtr<F>(
        &mut self,
        control: CbMainInvtr,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbMainInvtrValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbMainInvtr { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_marker_beacon<F>(
        &mut self,
        control: CbMarkerBeacon,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbMarkerBeaconValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbMarkerBeacon { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_nav_lts<F>(
        &mut self,
        control: CbNavLts,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbNavLtsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbNavLts { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_outlet_valve<F>(
        &mut self,
        control: CbOutletValve,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbOutletValveValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbOutletValve { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_pitot_tube<F>(
        &mut self,
        control: CbPitotTube,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbPitotTubeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbPitotTube { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_plt_attd1<F>(
        &mut self,
        control: CbPltAttd1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbPltAttd1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbPltAttd1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_plt_attd2<F>(
        &mut self,
        control: CbPltAttd2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbPltAttd2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbPltAttd2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_porx_warn<F>(
        &mut self,
        control: CbPorxWarn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbPorxWarnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbPorxWarn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_rescue_cont<F>(
        &mut self,
        control: CbRescueCont,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbRescueContValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbRescueCont { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_rescue_pwr<F>(
        &mut self,
        control: CbRescuePwr,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbRescuePwrValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbRescuePwr { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_rh_fuel<F>(
        &mut self,
        control: CbRhFuel,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbRhFuelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbRhFuel { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_rpm_warn<F>(
        &mut self,
        control: CbRpmWarn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbRpmWarnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbRpmWarn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_sl_ctl<F>(
        &mut self,
        control: CbSlCtl,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbSlCtlValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbSlCtl { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_sl_pwr<F>(
        &mut self,
        control: CbSlPwr,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbSlPwrValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbSlPwr { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_spare_invtr<F>(
        &mut self,
        control: CbSpareInvtr,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbSpareInvtrValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbSpareInvtr { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_starter<F>(
        &mut self,
        control: CbStarter,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbStarterValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbStarter { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_stby_gen<F>(
        &mut self,
        control: CbStbyGen,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbStbyGenValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbStbyGen { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_temp<F>(
        &mut self,
        control: CbTemp,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbTempValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbTemp { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_torq<F>(
        &mut self,
        control: CbTorq,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbTorqValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbTorq { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_trim<F>(
        &mut self,
        control: CbTrim,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbTrimValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbTrim { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_turn_slip<F>(
        &mut self,
        control: CbTurnSlip,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbTurnSlipValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbTurnSlip { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_uhf_radio<F>(
        &mut self,
        control: CbUhfRadio,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbUhfRadioValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbUhfRadio { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_vhf_nav<F>(
        &mut self,
        control: CbVhfNav,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbVhfNavValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbVhfNav { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_vm<F>(
        &mut self,
        control: CbVm,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbVmValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbVm { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_wiper_cplt<F>(
        &mut self,
        control: CbWiperCplt,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbWiperCpltValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbWiperCplt { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_wiper_plt<F>(
        &mut self,
        control: CbWiperPlt,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbWiperPltValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbWiperPlt { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cb_xmsn<F>(
        &mut self,
        control: CbXmsn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CbXmsnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CbXmsn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_chip_det_sw<F>(
        &mut self,
        control: ChipDetSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ChipDetSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ChipDetSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ci_course_card<F>(
        &mut self,
        control: CiCourseCard,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CiCourseCard { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ci_from<F>(
        &mut self,
        control: CiFrom,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CiFrom { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ci_hbar<F>(
        &mut self,
        control: CiHbar,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CiHbar { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ci_hoff<F>(
        &mut self,
        control: CiHoff,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CiHoff { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ci_to<F>(
        &mut self,
        control: CiTo,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CiTo { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ci_vbar<F>(
        &mut self,
        control: CiVbar,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CiVbar { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ci_voff<F>(
        &mut self,
        control: CiVoff,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CiVoff { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_a0<F>(
        &mut self,
        control: ClA0,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClA0 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_a1<F>(
        &mut self,
        control: ClA1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClA1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_b0<F>(
        &mut self,
        control: ClB0,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClB0 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_b1<F>(
        &mut self,
        control: ClB1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClB1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_c0<F>(
        &mut self,
        control: ClC0,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClC0 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_c1<F>(
        &mut self,
        control: ClC1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClC1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_d0<F>(
        &mut self,
        control: ClD0,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClD0 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_d1<F>(
        &mut self,
        control: ClD1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClD1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_e0<F>(
        &mut self,
        control: ClE0,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClE0 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_e1<F>(
        &mut self,
        control: ClE1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClE1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_f0<F>(
        &mut self,
        control: ClF0,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClF0 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_f1<F>(
        &mut self,
        control: ClF1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClF1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_g0<F>(
        &mut self,
        control: ClG0,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClG0 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_g1<F>(
        &mut self,
        control: ClG1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClG1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_h0<F>(
        &mut self,
        control: ClH0,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClH0 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_h1<F>(
        &mut self,
        control: ClH1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClH1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_i0<F>(
        &mut self,
        control: ClI0,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClI0 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_i1<F>(
        &mut self,
        control: ClI1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClI1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_j0<F>(
        &mut self,
        control: ClJ0,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClJ0 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cl_j1<F>(
        &mut self,
        control: ClJ1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClJ1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_adjust<F>(
        &mut self,
        control: ClockAdjust,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockAdjust { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_adjust_pull<F>(
        &mut self,
        control: ClockAdjustPull,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockAdjustPull { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_h<F>(
        &mut self,
        control: ClockH,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockH { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_m<F>(
        &mut self,
        control: ClockM,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockM { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clock_s<F>(
        &mut self,
        control: ClockS,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClockS { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_bright_dim_sw<F>(
        &mut self,
        control: ClpBrightDimSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ClpBrightDimSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpBrightDimSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_clp_reset_test_sw<F>(
        &mut self,
        control: ClpResetTestSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ClpResetTestSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ClpResetTestSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cm_arm_sw<F>(
        &mut self,
        control: CmArmSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CmArmSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmArmSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cm_armed_test<F>(
        &mut self,
        control: CmArmedTest,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CmArmedTestValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmArmedTest { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cm_chaffcnt_display<F>(
        &mut self,
        control: CmChaffcntDisplay,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmChaffcntDisplay { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cm_chaffcnt_display_str<F>(
        &mut self,
        control: CmChaffcntDisplayStr,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmChaffcntDisplayStr { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cm_chaffcnt_reset<F>(
        &mut self,
        control: CmChaffcntReset,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CmChaffcntResetValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmChaffcntReset { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cm_flare_btn<F>(
        &mut self,
        control: CmFlareBtn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CmFlareBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmFlareBtn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cm_flarecnt_display<F>(
        &mut self,
        control: CmFlarecntDisplay,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmFlarecntDisplay { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cm_flarecnt_display_str<F>(
        &mut self,
        control: CmFlarecntDisplayStr,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmFlarecntDisplayStr { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cm_flarecnt_reset<F>(
        &mut self,
        control: CmFlarecntReset,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CmFlarecntResetValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmFlarecntReset { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cm_man_pgrm_sw<F>(
        &mut self,
        control: CmManPgrmSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CmManPgrmSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmManPgrmSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cm_ripple_cover<F>(
        &mut self,
        control: CmRippleCover,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CmRippleCoverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmRippleCover { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cm_ripple_sw<F>(
        &mut self,
        control: CmRippleSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CmRippleSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CmRippleSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_console_light<F>(
        &mut self,
        control: ConsoleLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ConsoleLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_cplt_adi_cage<F>(
        &mut self,
        control: CpltAdiCage,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(CpltAdiCageValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::CpltAdiCage { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_crs_knob<F>(
        &mut self,
        control: CrsKnob,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::CrsKnob { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dc_bat1_current<F>(
        &mut self,
        control: DcBat1Current,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DcBat1Current { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dc_bat2_current<F>(
        &mut self,
        control: DcBat2Current,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DcBat2Current { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dc_v<F>(
        &mut self,
        control: DcV,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DcV { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dc_vm_src<F>(
        &mut self,
        control: DcVmSrc,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(DcVmSrcValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::DcVmSrc { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dc_voltage<F>(
        &mut self,
        control: DcVoltage,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DcVoltage { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dc_vu1_current<F>(
        &mut self,
        control: DcVu1Current,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DcVu1Current { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dc_vu2_current<F>(
        &mut self,
        control: DcVu2Current,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DcVu2Current { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dc_vu3_current<F>(
        &mut self,
        control: DcVu3Current,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DcVu3Current { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dome_light<F>(
        &mut self,
        control: DomeLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DomeLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dome_light_green<F>(
        &mut self,
        control: DomeLightGreen,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::DomeLightGreen { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_dome_light_sw<F>(
        &mut self,
        control: DomeLightSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(DomeLightSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::DomeLightSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_door_l_ptr<F>(
        &mut self,
        control: DoorLPtr,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(DoorLPtrValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::DoorLPtr { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_door_r_ptr<F>(
        &mut self,
        control: DoorRPtr,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(DoorRPtrValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::DoorRPtr { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_emer_gov_sw<F>(
        &mut self,
        control: EmerGovSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EmerGovSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EmerGovSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_eng_deice<F>(
        &mut self,
        control: EngDeice,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(EngDeiceValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngDeice { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_eng_oil_press<F>(
        &mut self,
        control: EngOilPress,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngOilPress { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_eng_oil_temp<F>(
        &mut self,
        control: EngOilTemp,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngOilTemp { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_eng_rpm<F>(
        &mut self,
        control: EngRpm,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::EngRpm { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_exhaust_temp<F>(
        &mut self,
        control: ExhaustTemp,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExhaustTemp { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_position_light_left<F>(
        &mut self,
        control: ExtPositionLightLeft,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtPositionLightLeft { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_position_light_right<F>(
        &mut self,
        control: ExtPositionLightRight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtPositionLightRight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_strobe<F>(
        &mut self,
        control: ExtStrobe,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtStrobe { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ext_wow_skid<F>(
        &mut self,
        control: ExtWowSkid,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::ExtWowSkid { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fire_ind<F>(
        &mut self,
        control: FireInd,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::FireInd { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fire_test_btn<F>(
        &mut self,
        control: FireTestBtn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(FireTestBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::FireTestBtn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_force_trim_sw<F>(
        &mut self,
        control: ForceTrimSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ForceTrimSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ForceTrimSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_press<F>(
        &mut self,
        control: FuelPress,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelPress { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_fuel_qty<F>(
        &mut self,
        control: FuelQty,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::FuelQty { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gas_rpm<F>(
        &mut self,
        control: GasRpm,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::GasRpm { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gas_rpm_t<F>(
        &mut self,
        control: GasRpmT,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::GasRpmT { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gmc_annunciator<F>(
        &mut self,
        control: GmcAnnunciator,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::GmcAnnunciator { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gmc_crs1<F>(
        &mut self,
        control: GmcCrs1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::GmcCrs1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gmc_crs2<F>(
        &mut self,
        control: GmcCrs2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::GmcCrs2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gmc_hdg<F>(
        &mut self,
        control: GmcHdg,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::GmcHdg { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gmc_hdg_marker<F>(
        &mut self,
        control: GmcHdgMarker,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::GmcHdgMarker { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gmc_pwrfail<F>(
        &mut self,
        control: GmcPwrfail,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::GmcPwrfail { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gov_rpm_sw<F>(
        &mut self,
        control: GovRpmSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(GovRpmSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::GovRpmSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gov_rpm_sw2<F>(
        &mut self,
        control: GovRpmSw2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(GovRpmSw2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::GovRpmSw2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gun_sel<F>(
        &mut self,
        control: GunSel,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(GunSelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::GunSel { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_gyro_mode_sw<F>(
        &mut self,
        control: GyroModeSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(GyroModeSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::GyroModeSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hdg_set<F>(
        &mut self,
        control: HdgSet,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HdgSet { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hdg_sync<F>(
        &mut self,
        control: HdgSync,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::HdgSync { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_hyd_cont_sw<F>(
        &mut self,
        control: HydContSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(HydContSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::HydContSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ias_nose<F>(
        &mut self,
        control: IasNose,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IasNose { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ias_roof<F>(
        &mut self,
        control: IasRoof,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IasRoof { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_code<F>(
        &mut self,
        control: IffCode,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffCode { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_master<F>(
        &mut self,
        control: IffMaster,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IffMasterValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffMaster { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_mic_ident<F>(
        &mut self,
        control: IffMicIdent,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IffMicIdentValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffMicIdent { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_mode1_wheel1<F>(
        &mut self,
        control: IffMode1Wheel1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IffMode1Wheel1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffMode1Wheel1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_mode1_wheel2<F>(
        &mut self,
        control: IffMode1Wheel2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IffMode1Wheel2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffMode1Wheel2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_mode3a_wheel1<F>(
        &mut self,
        control: IffMode3aWheel1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IffMode3aWheel1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffMode3aWheel1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_mode3a_wheel2<F>(
        &mut self,
        control: IffMode3aWheel2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IffMode3aWheel2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffMode3aWheel2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_mode3a_wheel3<F>(
        &mut self,
        control: IffMode3aWheel3,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IffMode3aWheel3Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffMode3aWheel3 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_mode3a_wheel4<F>(
        &mut self,
        control: IffMode3aWheel4,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IffMode3aWheel4Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffMode3aWheel4 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_on_out<F>(
        &mut self,
        control: IffOnOut,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IffOnOutValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffOnOut { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_out_audio_light<F>(
        &mut self,
        control: IffOutAudioLight,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IffOutAudioLightValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffOutAudioLight { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_radtest<F>(
        &mut self,
        control: IffRadtest,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IffRadtestValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffRadtest { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_reply_dim<F>(
        &mut self,
        control: IffReplyDim,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffReplyDim { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_reply_ind<F>(
        &mut self,
        control: IffReplyInd,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffReplyInd { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_reply_test<F>(
        &mut self,
        control: IffReplyTest,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IffReplyTestValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffReplyTest { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_test_dim<F>(
        &mut self,
        control: IffTestDim,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffTestDim { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_test_ind<F>(
        &mut self,
        control: IffTestInd,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffTestInd { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_test_m1<F>(
        &mut self,
        control: IffTestM1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IffTestM1Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffTestM1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_test_m2<F>(
        &mut self,
        control: IffTestM2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IffTestM2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffTestM2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_test_m3<F>(
        &mut self,
        control: IffTestM3,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IffTestM3Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffTestM3 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_test_m4<F>(
        &mut self,
        control: IffTestM4,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IffTestM4Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffTestM4 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_iff_test_test<F>(
        &mut self,
        control: IffTestTest,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IffTestTestValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IffTestTest { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_int_int_sw<F>(
        &mut self,
        control: IntIntSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IntIntSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IntIntSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_int_mode<F>(
        &mut self,
        control: IntMode,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IntModeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IntMode { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_int_nav_sw<F>(
        &mut self,
        control: IntNavSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IntNavSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IntNavSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_int_rcvr1_sw<F>(
        &mut self,
        control: IntRcvr1Sw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IntRcvr1SwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IntRcvr1Sw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_int_rcvr2_sw<F>(
        &mut self,
        control: IntRcvr2Sw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IntRcvr2SwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IntRcvr2Sw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_int_rcvr3_sw<F>(
        &mut self,
        control: IntRcvr3Sw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IntRcvr3SwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IntRcvr3Sw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_int_recvr4_sw<F>(
        &mut self,
        control: IntRecvr4Sw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(IntRecvr4SwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::IntRecvr4Sw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_int_vol<F>(
        &mut self,
        control: IntVol,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::IntVol { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_inverter_sw<F>(
        &mut self,
        control: InverterSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(InverterSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::InverterSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_jtsn_btn<F>(
        &mut self,
        control: JtsnBtn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(JtsnBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::JtsnBtn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_jtsn_cover<F>(
        &mut self,
        control: JtsnCover,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(JtsnCoverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::JtsnCover { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ldg_light_sw<F>(
        &mut self,
        control: LdgLightSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(LdgLightSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LdgLightSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ldg_light_sw2<F>(
        &mut self,
        control: LdgLightSw2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(LdgLightSw2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::LdgLightSw2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ldg_lt_ctrl<F>(
        &mut self,
        control: LdgLtCtrl,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(LdgLtCtrlValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LdgLtCtrl { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ldg_lt_ctrl2<F>(
        &mut self,
        control: LdgLtCtrl2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(LdgLtCtrl2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::LdgLtCtrl2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_low_rpm_audio<F>(
        &mut self,
        control: LowRpmAudio,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(LowRpmAudioValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::LowRpmAudio { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_low_rpm_ind<F>(
        &mut self,
        control: LowRpmInd,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::LowRpmInd { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_main_fuel_sw<F>(
        &mut self,
        control: MainFuelSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MainFuelSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MainFuelSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_main_gen_cover<F>(
        &mut self,
        control: MainGenCover,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MainGenCoverValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MainGenCover { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_main_gen_load<F>(
        &mut self,
        control: MainGenLoad,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MainGenLoad { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_main_gen_sw<F>(
        &mut self,
        control: MainGenSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(MainGenSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::MainGenSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_marker_beacon<F>(
        &mut self,
        control: MarkerBeacon,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::MarkerBeacon { control, callback })
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

    pub fn register_master_caution_ind<F>(
        &mut self,
        control: MasterCautionInd,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::MasterCautionInd { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nav_lts_sw<F>(
        &mut self,
        control: NavLtsSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(NavLtsSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::NavLtsSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_non_ess_bus_sw<F>(
        &mut self,
        control: NonEssBusSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(NonEssBusSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::NonEssBusSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_nvg_pos_lts<F>(
        &mut self,
        control: NvgPosLts,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(NvgPosLtsValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::NvgPosLts { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oadi_off<F>(
        &mut self,
        control: OadiOff,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OadiOff { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oadi_pitch<F>(
        &mut self,
        control: OadiPitch,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OadiPitch { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oadi_roll<F>(
        &mut self,
        control: OadiRoll,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OadiRoll { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oalt100<F>(
        &mut self,
        control: Oalt100,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Oalt100 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oalt1000<F>(
        &mut self,
        control: Oalt1000,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Oalt1000 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oalt10000<F>(
        &mut self,
        control: Oalt10000,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Oalt10000 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oalt_code_off_flag<F>(
        &mut self,
        control: OaltCodeOffFlag,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OaltCodeOffFlag { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oalt_press0<F>(
        &mut self,
        control: OaltPress0,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OaltPress0 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oalt_press1<F>(
        &mut self,
        control: OaltPress1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OaltPress1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oalt_press2<F>(
        &mut self,
        control: OaltPress2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OaltPress2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_oalt_ptr<F>(
        &mut self,
        control: OaltPtr,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::OaltPtr { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_padi_off<F>(
        &mut self,
        control: PadiOff,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::PadiOff { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_padi_pitch<F>(
        &mut self,
        control: PadiPitch,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::PadiPitch { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_padi_roll<F>(
        &mut self,
        control: PadiRoll,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::PadiRoll { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_palt100<F>(
        &mut self,
        control: Palt100,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Palt100 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_palt1000<F>(
        &mut self,
        control: Palt1000,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Palt1000 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_palt10000<F>(
        &mut self,
        control: Palt10000,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Palt10000 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_palt_press<F>(
        &mut self,
        control: PaltPress,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::PaltPress { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pitot_heat<F>(
        &mut self,
        control: PitotHeat,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(PitotHeatValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::PitotHeat { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_pos_lts_sw<F>(
        &mut self,
        control: PosLtsSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(PosLtsSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::PosLtsSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_alt_hi<F>(
        &mut self,
        control: RadarAltHi,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarAltHi { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_alt_lo<F>(
        &mut self,
        control: RadarAltLo,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarAltLo { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_alt_pwr<F>(
        &mut self,
        control: RadarAltPwr,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RadarAltPwrValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarAltPwr { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radar_alt_test<F>(
        &mut self,
        control: RadarAltTest,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RadarAltTestValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadarAltTest { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_radio_ics_sw<F>(
        &mut self,
        control: RadioIcsSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RadioIcsSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RadioIcsSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ralt_digit1<F>(
        &mut self,
        control: RaltDigit1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RaltDigit1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ralt_digit2<F>(
        &mut self,
        control: RaltDigit2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RaltDigit2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ralt_digit3<F>(
        &mut self,
        control: RaltDigit3,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RaltDigit3 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ralt_digit4<F>(
        &mut self,
        control: RaltDigit4,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RaltDigit4 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ralt_display<F>(
        &mut self,
        control: RaltDisplay,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RaltDisplay { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ralt_display_str<F>(
        &mut self,
        control: RaltDisplayStr,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::RaltDisplayStr { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ralt_hi_idx<F>(
        &mut self,
        control: RaltHiIdx,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RaltHiIdx { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ralt_hi_lamp<F>(
        &mut self,
        control: RaltHiLamp,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RaltHiLamp { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ralt_lo_idx<F>(
        &mut self,
        control: RaltLoIdx,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RaltLoIdx { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ralt_lo_lamp<F>(
        &mut self,
        control: RaltLoLamp,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::RaltLoLamp { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ralt_needle<F>(
        &mut self,
        control: RaltNeedle,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RaltNeedle { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ralt_off<F>(
        &mut self,
        control: RaltOff,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RaltOff { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_ram_temp<F>(
        &mut self,
        control: RamTemp,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RamTemp { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rmi_crs1<F>(
        &mut self,
        control: RmiCrs1,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RmiCrs1 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rmi_crs2<F>(
        &mut self,
        control: RmiCrs2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RmiCrs2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rmi_hdg<F>(
        &mut self,
        control: RmiHdg,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RmiHdg { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rocket_pair<F>(
        &mut self,
        control: RocketPair,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RocketPairValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RocketPair { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rocket_reset<F>(
        &mut self,
        control: RocketReset,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RocketResetValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RocketReset { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rocket_sel<F>(
        &mut self,
        control: RocketSel,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(RocketSelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::RocketSel { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_rotor_rpm<F>(
        &mut self,
        control: RotorRpm,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::RotorRpm { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_safe_ind<F>(
        &mut self,
        control: SafeInd,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::SafeInd { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sby_compass_bank<F>(
        &mut self,
        control: SbyCompassBank,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SbyCompassBank { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sby_compass_hdg<F>(
        &mut self,
        control: SbyCompassHdg,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SbyCompassHdg { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sby_compass_pitch<F>(
        &mut self,
        control: SbyCompassPitch,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SbyCompassPitch { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_search_light_sw<F>(
        &mut self,
        control: SearchLightSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(SearchLightSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SearchLightSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_search_light_sw2<F>(
        &mut self,
        control: SearchLightSw2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(SearchLightSw2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::SearchLightSw2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sideslip<F>(
        &mut self,
        control: Sideslip,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Sideslip { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sight_arm_plt<F>(
        &mut self,
        control: SightArmPlt,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(SightArmPltValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SightArmPlt { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sight_elev_plt<F>(
        &mut self,
        control: SightElevPlt,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SightElevPlt { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sight_inten_cplt<F>(
        &mut self,
        control: SightIntenCplt,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SightIntenCplt { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sight_inten_plt<F>(
        &mut self,
        control: SightIntenPlt,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SightIntenPlt { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sight_lamp_sw<F>(
        &mut self,
        control: SightLampSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(SightLampSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SightLampSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sight_pwr_plt<F>(
        &mut self,
        control: SightPwrPlt,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(SightPwrPltValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::SightPwrPlt { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_sights_for_civil<F>(
        &mut self,
        control: SightsForCivil,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::SightsForCivil { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_starter_gen_sw<F>(
        &mut self,
        control: StarterGenSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(StarterGenSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::StarterGenSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_stby_gen_load<F>(
        &mut self,
        control: StbyGenLoad,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::StbyGenLoad { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_test_fuel_gauge_btn<F>(
        &mut self,
        control: TestFuelGaugeBtn,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(TestFuelGaugeBtnValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::TestFuelGaugeBtn { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_throttle<F>(
        &mut self,
        control: Throttle,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Throttle { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_throttle2<F>(
        &mut self,
        control: Throttle2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::Throttle2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_throttle_stop<F>(
        &mut self,
        control: ThrottleStop,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ThrottleStopValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::ThrottleStop { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_throttle_stop2<F>(
        &mut self,
        control: ThrottleStop2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(ThrottleStop2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::ThrottleStop2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_torque_press<F>(
        &mut self,
        control: TorquePress,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TorquePress { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_trans_oil_press<F>(
        &mut self,
        control: TransOilPress,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TransOilPress { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_trans_oil_temp<F>(
        &mut self,
        control: TransOilTemp,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TransOilTemp { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_trim_cplt<F>(
        &mut self,
        control: TrimCplt,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(TrimCpltValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::TrimCplt { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_trim_plt<F>(
        &mut self,
        control: TrimPlt,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(TrimPltValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::TrimPlt { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_turn_ptr<F>(
        &mut self,
        control: TurnPtr,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::TurnPtr { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_freq<F>(
        &mut self,
        control: UhfFreq,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfFreq { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_function<F>(
        &mut self,
        control: UhfFunction,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(UhfFunctionValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfFunction { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_mode<F>(
        &mut self,
        control: UhfMode,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(UhfModeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfMode { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_preset_int<F>(
        &mut self,
        control: UhfPresetInt,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfPresetInt { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_uhf_preset_str<F>(
        &mut self,
        control: UhfPresetStr,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfPresetStr { control, callback })
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

    pub fn register_uhf_vol<F>(
        &mut self,
        control: UhfVol,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::UhfVol { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhfcomm_freq<F>(
        &mut self,
        control: VhfcommFreq,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfcommFreq { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhfcomm_khz<F>(
        &mut self,
        control: VhfcommKhz,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfcommKhz { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhfcomm_mhz<F>(
        &mut self,
        control: VhfcommMhz,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfcommMhz { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhfcomm_pwr<F>(
        &mut self,
        control: VhfcommPwr,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(VhfcommPwrValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfcommPwr { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhfcomm_test_sw<F>(
        &mut self,
        control: VhfcommTestSw,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(VhfcommTestSwValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfcommTestSw { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhfcomm_vol<F>(
        &mut self,
        control: VhfcommVol,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfcommVol { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhffm_freq1_int<F>(
        &mut self,
        control: VhffmFreq1Int,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(VhffmFreq1IntValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhffmFreq1Int { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhffm_freq1_str<F>(
        &mut self,
        control: VhffmFreq1Str,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhffmFreq1Str { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhffm_freq2<F>(
        &mut self,
        control: VhffmFreq2,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(VhffmFreq2Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhffmFreq2 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhffm_freq3<F>(
        &mut self,
        control: VhffmFreq3,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(VhffmFreq3Value) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhffmFreq3 { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhffm_freq4_int<F>(
        &mut self,
        control: VhffmFreq4Int,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(VhffmFreq4IntValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhffmFreq4Int { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhffm_freq4_str<F>(
        &mut self,
        control: VhffmFreq4Str,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhffmFreq4Str { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhffm_mode<F>(
        &mut self,
        control: VhffmMode,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(VhffmModeValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhffmMode { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhffm_squelch<F>(
        &mut self,
        control: VhffmSquelch,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(VhffmSquelchValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhffmSquelch { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhffm_vol<F>(
        &mut self,
        control: VhffmVol,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhffmVol { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhfnav_freq<F>(
        &mut self,
        control: VhfnavFreq,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(&[u8]) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfnavFreq { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhfnav_khz<F>(
        &mut self,
        control: VhfnavKhz,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfnavKhz { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhfnav_mhz<F>(
        &mut self,
        control: VhfnavMhz,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfnavMhz { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhfnav_pwr<F>(
        &mut self,
        control: VhfnavPwr,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(VhfnavPwrValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfnavPwr { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vhfnav_vol<F>(
        &mut self,
        control: VhfnavVol,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VhfnavVol { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vvi_o<F>(
        &mut self,
        control: VviO,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VviO { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_vvi_p<F>(
        &mut self,
        control: VviP,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::VviP { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_wiper_o<F>(
        &mut self,
        control: WiperO,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::WiperO { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_wiper_p<F>(
        &mut self,
        control: WiperP,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::WiperP { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_wiper_sel<F>(
        &mut self,
        control: WiperSel,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(WiperSelValue) + 'a,
    {
        self.controls
            .push(RegisteredControl::WiperSel { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_wiper_spd<F>(
        &mut self,
        control: WiperSpd,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(u16) + 'a,
    {
        self.controls
            .push(RegisteredControl::WiperSpd { control, callback })
            .map_err(|_| "aircraft control capacity exceeded")?;
        Ok(self)
    }

    pub fn register_x130_armed<F>(
        &mut self,
        control: X130Armed,
        callback: &'a mut F,
    ) -> Result<&mut Self, &'static str>
    where
        F: FnMut(bool) + 'a,
    {
        self.controls
            .push(RegisteredControl::X130Armed { control, callback })
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
